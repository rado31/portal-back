use tide::{
    http::headers::HeaderValue,
    security::{CorsMiddleware, Origin},
};

pub fn cors() -> CorsMiddleware {
    CorsMiddleware::new()
        .allow_origin(Origin::from("*"))
        .allow_methods("*".parse::<HeaderValue>().unwrap())
        .allow_headers("*".parse::<HeaderValue>().unwrap())
        .allow_credentials(false)
}
