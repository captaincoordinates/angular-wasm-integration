use lazy_static::lazy_static;
use std::time::{ SystemTime, Duration };
use hmac::{ Hmac, Mac };
use jwt::{ SignWithKey, VerifyWithKey };
use sha2::Sha256;
use std::collections::BTreeMap;

type HmacSha256 = Hmac<Sha256>;

lazy_static! {
    static ref JWT_KEY: HmacSha256 = {
        let jwt_private_key = std::env::var("JWT_SECRET").unwrap();
        let jwt_slice = &jwt_private_key[0..44]; // 32-bit base64-encoded will always be 44 characters that are single byte in UTF8
        HmacSha256::new_from_slice(jwt_slice.as_bytes()).unwrap()
    };
}

pub enum AuthFailure {
    InvalidToken,
    ExpiredToken,
}

fn get_jwt_key() -> &'static HmacSha256 {
    &JWT_KEY
}

pub fn create_token() -> http::Response<Vec<u8>> {
    let mut claims = BTreeMap::new();
    let in_24_hours = SystemTime::now() + Duration::from_secs(24 * 60 * 60);
    let exp = in_24_hours.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    claims.insert("exp", exp.to_string());
    let key = get_jwt_key();
    let token = claims.sign_with_key(key).unwrap();
    http::Response
        ::builder()
        .status(204)
        .header("access_token", token)
        .body("".as_bytes().to_vec())
        .unwrap()
}

pub fn validate_token(token_header_str: &str) -> Result<(), AuthFailure> {
    let value_prefix = "Bearer ";
    let header_value_str = String::from(token_header_str);
    if header_value_str.starts_with(value_prefix) && header_value_str.len() > value_prefix.len() {
        println!("valid token format");
        let token = header_value_str[value_prefix.len()..].to_owned();
        println!("token: '{:}'", token);
        let key = get_jwt_key();
        let claims_result: Result<BTreeMap<String, String>, jwt::Error> = token.verify_with_key(
            key
        );
        match claims_result {
            Ok(claims) => {
                if let Some(expiry_str) = claims.get("exp") {
                    if let Ok(expiry_secs) = expiry_str.parse::<u64>() {
                        if
                            expiry_secs >=
                            SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                        {
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
            }
            Err(jwt_error) => {
                println!("invalid token: {:?}", jwt_error);
                Err(AuthFailure::InvalidToken)
            }
        }
    } else {
        println!("invalid token format");
        Err(AuthFailure::InvalidToken)
    }
}
