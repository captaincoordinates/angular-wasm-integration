use crate::browser::console_log;
use crate::utils::set_panic_hook;
use reqwest;
use wasm_bindgen::prelude::*;
use serde_json::json;

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

    pub fn greet(&self) {
        if let Some(jwt_str) = &self.jwt {
            console_log(&format!("Hello JWT: '{:}'", jwt_str));
        } else {
            console_log(&format!("Hello darkness my old friend"));
        }
    }
}
