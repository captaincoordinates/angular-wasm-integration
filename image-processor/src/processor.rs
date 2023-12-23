use crate::browser::console_log;
use crate::utils::set_panic_hook;
use reqwest;
use wasm_bindgen::prelude::*;
use serde_json::json;
use image::io::Reader as ImageReader;
use std::io::Cursor;

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

    pub async fn fetch_image(&self, band: u8) {
        if let Ok(response) = self.http_client
            .get(&format!("http://localhost:3000/api/T09UXA_20231210T194821?band={:}", band))
            .header("Authorization", &format!("Bearer {:}", self.jwt.clone().unwrap()))
            .send()
            .await {
            if response.status().is_success() {
                let bytes = response.bytes().await.unwrap();
                let img = ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap();
                console_log(&format!("dimensions: {:}x{:}", img.width(), img.height()));
            } else {
                wasm_bindgen::throw_str(&format!("error fetching image band {:}", band));
            }
        } else {
            wasm_bindgen::throw_str(&format!("error fetching image band {:}", band));
        }
    }

    pub fn greet(&self) {
        if let Some(jwt_str) = &self.jwt {
            console_log(&format!("Hello JWT: '{:}'", jwt_str));
        } else {
            console_log(&format!("Hello darkness my old friend"));
        }
    }
}
