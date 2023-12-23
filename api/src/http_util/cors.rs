use http::response::Builder;

pub fn permit_cors(builder: Builder) -> Builder {
    builder
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Expose-Headers", "Access_token")
}
