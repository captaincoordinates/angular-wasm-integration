use spin_sdk::http::{ IntoResponse, Request, Method, send, Response };
use spin_sdk::http_component;
use regex::Regex;
use http_util::{ QueryString, Value, permit_cors };
use sentinel2::{ Band, BandIdentifier };
use auth::{ create_token, validate_token, AuthFailure };

mod auth;
mod http_util;
mod sentinel2;

const API_BASE: &str = "/api";
const IMG_BASE_HREF: &str = "https://tchristian-wasm-data.s3.us-west-2.amazonaws.com/";

enum ApiBehaviour<'req> {
    Get(BandIdentifier<'req>),
    CreateToken,
    PermitCors,
    NotFound,
}

#[http_component]
async fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    match api_from_request(&req).await {
        ApiBehaviour::CreateToken => { Ok(create_token()) },
        ApiBehaviour::PermitCors => {
            Ok(permit_cors(http::Response::builder().status(204)).body("".as_bytes().to_vec())?)
        },
        not_create_token => {
            if let Some(header_value) = req.header("Authorization") {
                match validate_token(header_value.as_str().unwrap()) {
                    Ok(()) => {
                        match not_create_token {
                            ApiBehaviour::Get(identifier) => { Ok(get_image(&identifier).await) }
                            _ =>
                                Ok(
                                    http::Response
                                        ::builder()
                                        .status(404)
                                        .header("content-type", "text/plain")
                                        .body("Not Found".as_bytes().to_vec())?
                                ),
                        }
                    }
                    Err(auth_failure) => {
                        match auth_failure {
                            AuthFailure::InvalidToken => {
                                Ok(
                                    http::Response
                                        ::builder()
                                        .status(403)
                                        .body("Invalid Token".as_bytes().to_vec())?
                                )
                            }
                            AuthFailure::ExpiredToken => {
                                Ok(
                                    http::Response
                                        ::builder()
                                        .status(401)
                                        .body("Expired Token".as_bytes().to_vec())?
                                )
                            }
                        }
                    }
                }
            } else {
                Ok(http::Response::builder().status(401).body("Missing Token".as_bytes().to_vec())?)
            }
        },
    }
}

async fn api_from_request<'req>(req: &'req Request) -> ApiBehaviour<'req> {
    let rel_path = &req.path()[API_BASE.len() + 1..]; // safe as long as API_BASE is always ASCII characters
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
                        }
                        _ => {}
                    }
                }
            }
            ApiBehaviour::NotFound
        }
        Method::Post => {
            if rel_path == "token" {
                return ApiBehaviour::CreateToken;
            }
            ApiBehaviour::NotFound
        },
        Method::Options => {
            ApiBehaviour::PermitCors
        },
        _ => { ApiBehaviour::NotFound }
    }
}

async fn get_image<'req>(identifier: &'req BandIdentifier<'req>) -> http::Response<Vec<u8>> {
    let img_href = format!("{:}{:}_{:?}_clip.tif", IMG_BASE_HREF, identifier.image_id, identifier.band);
    let response_result: Result<Response, _> = send(Request::get(img_href)).await;
    match response_result {
        Ok(response) => {
            let mut builder = http::Response::builder().status(*response.status());
            for header_entry in response.headers() {
                if let Some(header_value_str) = header_entry.1.as_str() {
                    builder = builder.header(header_entry.0, header_value_str);
                }
            }
            return permit_cors(builder).body(response.body().to_vec()).unwrap();
        },
        _ => {
            http::Response::builder()
                .status(500)
                .body("Error fetching data".as_bytes().to_vec())
                .unwrap()
        },
    }
}
