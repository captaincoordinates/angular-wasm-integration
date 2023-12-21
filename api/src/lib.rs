use spin_sdk::http::{IntoResponse, Request, Method};
use spin_sdk::http_component;
use regex::Regex;
use http_util::{QueryString, Value};
use sentinel2::{Band, BandIdentifier};

mod http_util;
mod sentinel2;

const API_BASE: &str = "/api";

enum ApiBehaviour<'req> {
    Get(BandIdentifier<'req>),
    BadRequest,
    NotFound,
    MethodNotAllowed,
}

// A simple Spin HTTP component.
#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    api_from_request(&req);
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")?)
}


fn api_from_request<'req>(req: &Request) -> ApiBehaviour<'req> {
    match &req.method() {
        Method::Get => {
            let rel_path = &req.path()[API_BASE.len() + 1..];   // safe as long as API_BASE is always ASCII characters
            let query_string = QueryString::from(req.query());
            let image_id_re = Regex::new("^[A-Z0-9]+_[0-9]{8}T[0-9]{6}$").unwrap();
            if image_id_re.is_match(rel_path) {
                if let Some(band_value) = query_string.get("band") {
                    match band_value {
                        Value::Single(val) => {
                            let band = Band::try_from(*val).unwrap_or(Band::B1);
                            let identifier = BandIdentifier::new(rel_path, &band);

                            println!("Fetching {:} band {:?}", rel_path, band);
                        },
                        _ => {},
                    }
                }
            }
            ApiBehaviour::NotFound
        },
        _ => {
            ApiBehaviour::NotFound
        }
    }
}