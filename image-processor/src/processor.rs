use crate::browser::console_log;
use crate::image::{Image as ImageData};
use crate::utils::set_panic_hook;
use image::GenericImageView;
use reqwest;
use wasm_bindgen::prelude::*;
use serde_json::json;
use image::io::Reader as ImageReader;
use std::io::Cursor;
use std::convert::TryInto;

#[wasm_bindgen]
pub struct Processor {
    jwt: Option<String>,
    http_client: reqwest::Client,
}

#[wasm_bindgen]
impl Processor {

    pub fn new() -> Self {
        set_panic_hook();
        Self {
            jwt: Option::None,
            http_client: reqwest::Client::new(),
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

    pub async fn fetch_image(&self, band: u8, histogram_stretch: bool) -> ImageData {
        if let Ok(response) = self.http_client
            .get(&format!("http://localhost:3000/api/T09UXA_20231210T194821?band={:}", band))
            .header("Authorization", &format!("Bearer {:}", self.jwt.clone().unwrap()))
            .send()
            .await {
            if response.status().is_success() {
                let bytes = response.bytes().await.unwrap();
                let img = ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap();
                let mut pixels: Vec<u8> = vec![];
                for i in 0..img.width() {
                    for j in 0..img.height() {
                        pixels.push(img.get_pixel(i, j)[0]);
                    }
                }
                if histogram_stretch {
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
                                let mut idx = 0;
                                let mut first: Vec<u8> = vec![];
                                let mut last: Vec<u8> = vec![];
                                for pixel in pixels.iter() {
                                    if idx < 10 {
                                        first.push(*pixel);
                                    }
                                    if idx >= (pixels.len() - 10) {
                                        last.push(*pixel);
                                    }
                                    idx += 1;
                                }
                                console_log(&format!("rs first: {:?}", first));
                                console_log(&format!("rs last: {:?}", last));
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
                ImageData::new(img.width().try_into().unwrap(), img.height().try_into().unwrap(), pixels)
            } else {
                wasm_bindgen::throw_str(&format!("error fetching image band {:}", band));
            }
        } else {
            wasm_bindgen::throw_str(&format!("error fetching image band {:}", band));
        }
    }
}
