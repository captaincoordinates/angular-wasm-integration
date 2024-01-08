use crate::browser::console_log;
use crate::image::Image as ImageData;
use crate::utils::set_panic_hook;
use image::{GenericImageView, DynamicImage};
use image::imageops::{resize, FilterType};
use reqwest;
use wasm_bindgen::prelude::*;
use serde_json::json;
use image::io::Reader as ImageReader;
use std::collections::HashMap;
use std::io::Cursor;
use std::convert::TryInto;

#[wasm_bindgen]
pub struct Processor {
    jwt: Option<String>,
    http_client: reqwest::Client,
    fetch_cache: HashMap<String, DynamicImage>,
    processed_cache: HashMap<String, ImageData>,
}

#[wasm_bindgen]
impl Processor {

    pub fn new() -> Self {
        set_panic_hook();
        Self {
            jwt: Option::None,
            http_client: reqwest::Client::new(),
            fetch_cache: HashMap::new(),
            processed_cache: HashMap::new(),
        }
    }

    pub async fn authenticate(&mut self, username: &str, password: &str) {
        if let Ok(response) = self.http_client
            .post("http://localhost:3000/api/token")
            .body(json!({
                "user": username,
                "pass": password,
            }).to_string())
            .send()
            .await {
            if let Some(access_token) = response.headers().get("Access_token") {
                let access_token_str = access_token.to_str().unwrap();
                self.jwt = Some(String::from(access_token_str));
            } else {
                wasm_bindgen::throw_str("access_token not provided by backend");
            }
        } else {
            wasm_bindgen::throw_str("Problem calling backend API")
        }
    }

    pub async fn fetch_image(&mut self, band: u8, histogram_stretch: bool, width_option: Option<u32>, height_option: Option<u32>) -> ImageData {
        let width = width_option.unwrap_or(0);
        let height = height_option.unwrap_or(0);
        let cache_key = Processor::get_processed_cache_key(&band, &histogram_stretch, &width, &height);
        if self.processed_cache.contains_key(&cache_key) {
            console_log("processed cache hit, returning cached image");
            return self.processed_cache.get(&cache_key).unwrap().clone();
        } else {
            console_log("processed cache miss, processing image");
            let fetch_cache_key = Processor::get_fetch_cache_key(&band);
            if !self.fetch_cache.contains_key(&fetch_cache_key) {
                console_log("fetch cache miss, fetching image");
                if let Ok(response) = self.http_client
                    .get(&format!("http://localhost:3000/api/T09UXA_20231210T194821?band={:}", band))
                    .header("Authorization", &format!("Bearer {:}", self.jwt.clone().unwrap()))
                    .send()
                    .await {
                    if response.status().is_success() {
                        let bytes = response.bytes().await.unwrap();
                        let img = ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap();
                        self.fetch_cache.insert(fetch_cache_key.clone(), img);
                    } else {
                        wasm_bindgen::throw_str(&format!("error fetching image band {:}", band));
                    }
                } else {
                    wasm_bindgen::throw_str(&format!("error fetching image band {:}", band));
                }
            } else {
                console_log("fetch cache hit, using cached image");
            }
            let mut img = self.fetch_cache.get(&fetch_cache_key).unwrap().clone();
            if width > 0 && height > 0 {
                img = DynamicImage::ImageRgba8(resize(&img, width, height, FilterType::Nearest));
            }
            let mut grey_scale_pixels: Vec<u8> = vec![];
            for i in 0..img.width() {
                for j in 0..img.height() {
                    grey_scale_pixels.push(img.get_pixel(i, j)[0]);
                }
            }
            if histogram_stretch {
                Processor::apply_histogram_stretch(&mut grey_scale_pixels);
            }
            let mut rgba_pixels: Vec<u8> = vec![];
            for i in 0..grey_scale_pixels.len() {
                for _ in 0..3 {
                    rgba_pixels.push(grey_scale_pixels[i])
                }
                rgba_pixels.push(255);
            }
            self.processed_cache.insert(cache_key.clone(), ImageData::new(img.width().try_into().unwrap(), img.height().try_into().unwrap(), rgba_pixels));
            return self.processed_cache.get(&cache_key).unwrap().clone();
        }
    }

    fn apply_histogram_stretch(pixels: &mut Vec<u8>) {
        let non_zero_values: Vec<u8> = pixels.iter().cloned().filter(|&x| x != 0).collect();
        let min_value_option = non_zero_values.iter().min();
        let max_value_option = non_zero_values.iter().max();
        if let Some(min_value) = min_value_option {
            if let Some(max_value) = max_value_option {
                let value_range = max_value - min_value;
                if value_range > 0 {
                    console_log(&format!("stretching with min: {:}, max: {:}", min_value, max_value));
                    for pixel in pixels.iter_mut() {
                        let calculated = ((*pixel as f32) - (*min_value as f32)) / (value_range as f32) * (255 as f32);
                        *pixel = calculated.round() as u8;
                    }
                } else {
                    console_log("value range is zero");
                }
            } else {
                console_log("unable to determine max non-zero pixel value");
            }
        } else {
            console_log("unable to determine min non-zero pixel value");
        }
    }

    pub fn clear_processed_cache(&mut self) {
        console_log(&format!("WASM clearing processed cache"));
        self.processed_cache.clear();
    }

    fn get_fetch_cache_key(band: &u8) -> String {
        format!("b{:}", band)
    }

    fn get_processed_cache_key(band: &u8, histogram_stretch: &bool, width: &u32, height: &u32) -> String {
        format!("b{:}_stretch{:}_w{:}_h{:}", band, histogram_stretch, width, height).to_owned()
    }
}
