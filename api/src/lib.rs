use spin_sdk::http::{IntoResponse, Request, Method, send, Response};
use spin_sdk::http_component;
use regex::Regex;
use http_util::{QueryString, Value};
use sentinel2::{Band, BandIdentifier};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use lazy_static::lazy_static;
use std::time::{SystemTime, Duration};

mod http_util;
mod sentinel2;

type HmacSha256 = Hmac<Sha256>;

const API_BASE: &str = "/api";
const IMG_BASE_HREF: &str = "https://tchristian-wasm-data.s3.us-west-2.amazonaws.com/";

lazy_static! {
    static ref JWT_KEY: HmacSha256 = {
        let jwt_private_key = std::env::var("JWT_SECRET").unwrap();
        let jwt_slice = &jwt_private_key[0..44];    // 32-bit base64-encoded will always be 44 characters that are single byte in UTF8
        HmacSha256::new_from_slice(jwt_slice.as_bytes()).unwrap()
    };
}

enum ApiBehaviour<'req> {
    Get(BandIdentifier<'req>),
    CreateToken,
    BadRequest,
    NotFound,
    MethodNotAllowed,
}

enum AuthFailure {
    MissingToken,
    InvalidToken,
    ExpiredToken,
}

#[http_component]
async fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    match api_from_request(&req).await {
        ApiBehaviour::CreateToken => {
            Ok(create_token())
        },
        not_create_token => {
            match validate_token(&req) {
                Ok(()) => {
                    match not_create_token {
                        ApiBehaviour::Get(identifier) => {
                            Ok(get_image(&identifier).await)
                        },
                        _ => Ok(http::Response::builder()
                            .status(404)
                            .header("content-type", "text/plain")
                            .body("Not Found".as_bytes().to_vec())?)
                    }
                },
                Err(auth_failure) => {
                    match auth_failure {
                        AuthFailure::InvalidToken => {
                            Ok(http::Response::builder()
                                .status(403)
                                .body("Invalid Token".as_bytes().to_vec())?)
                        },
                        AuthFailure::MissingToken => {
                            Ok(http::Response::builder()
                                .status(401)
                                .body("Missing Token".as_bytes().to_vec())?)
                        },
                        AuthFailure::ExpiredToken => {
                            Ok(http::Response::builder()
                                .status(401)
                                .body("Expired Token".as_bytes().to_vec())?)
                        },
                    }
                }
            }
        }
    }
}

async fn api_from_request<'req>(req: &'req Request) -> ApiBehaviour<'req> {
    let rel_path = &req.path()[API_BASE.len() + 1..];   // safe as long as API_BASE is always ASCII characters
    match &req.method() {
        Method::Get => {
            let query_string = QueryString::from(req.query());
            let image_id_re = Regex::new("^[A-Z0-9]+_[0-9]{8}T[0-9]{6}$").unwrap();
            if image_id_re.is_match(rel_path) {
                if let Some(band_value) = query_string.get("band") {
                    match *band_value {
                        Value::Single(val) => {
                            let band = Band::try_from(val).unwrap_or(Band::B01);
                            return ApiBehaviour::Get(BandIdentifier::new(rel_path, band));
                        },
                        _ => {},
                    }
                }
            }
            ApiBehaviour::NotFound
        },
        Method::Post => {
            if rel_path == "token" {
                return ApiBehaviour::CreateToken;
            }
            ApiBehaviour::NotFound
        },
        _ => {
            ApiBehaviour::NotFound
        }
    }
}

fn get_jwt_key() -> &'static HmacSha256 {
    &JWT_KEY
}

fn create_token() -> http::Response<Vec<u8>> {
    let mut claims = BTreeMap::new();
    let in_24_hours = SystemTime::now() + Duration::from_secs(24 * 60 * 60);
    let exp = in_24_hours.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    claims.insert("exp", exp.to_string());
    let key = get_jwt_key();
    let token = claims.sign_with_key(key).unwrap();
    http::Response::builder()
        .status(204)
        .header("access_token", token)
        .body("".as_bytes().to_vec()).unwrap()
}

fn validate_token(req: &Request) -> Result<(), AuthFailure> {
    if let Some(header_value) = req.header("Authorization") {
        let value_prefix = "Bearer ";
        let header_value_str = String::from(header_value.as_str().unwrap());
        if header_value_str.starts_with(value_prefix) && header_value_str.len() > value_prefix.len() {
            println!("valid token format");
            let token = header_value_str[value_prefix.len()..].to_owned();
            println!("token: '{:}'", token);
            let key = get_jwt_key();
            let claims_result: Result<BTreeMap<String, String>, jwt::Error> = token.verify_with_key(key);
            match claims_result {
                Ok(claims) => {
                    if let Some(expiry_str) = claims.get("exp") {
                        println!("expires: {:}", expiry_str);
                        if let Ok(expiry_secs) = expiry_str.parse::<u64>() {
                            if expiry_secs >= SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() {
                                Ok(())
                            } else {
                                Err(AuthFailure::ExpiredToken)
                            }
                        } else {
                            Err(AuthFailure::InvalidToken)
                        }
                    } else {
                        Err(AuthFailure::InvalidToken)
                    }
                },
                Err(jwt_error) => {
                    println!("invalid token: {:?}", jwt_error);
                    Err(AuthFailure::InvalidToken)
                }
            }
        } else {
            println!("invalid token format");
            Err(AuthFailure::InvalidToken)
        }
    } else {
        Err(AuthFailure::MissingToken)
    }
}

async fn get_image<'req>(identifier: &'req BandIdentifier<'req>) -> http::Response<Vec<u8>> {
    let img_href = format!("{:}{:}_{:?}.jp2", IMG_BASE_HREF, identifier.image_id, identifier.band);
    let response: Response = send(Request::get(img_href)).await.unwrap();            
    let mut builder = http::Response::builder().status(*response.status());
    for header_entry in response.headers() {
        if let Some(header_value_str) = header_entry.1.as_str() {
            builder = builder.header(header_entry.0, header_value_str);
        }
    }
    builder.body(response.body().to_vec()).unwrap()
}
