use crate::utils::verify_token;
use tide::{
    http::headers::HeaderValue,
    security::{CorsMiddleware, Origin},
    Middleware, Next, Request, Response,
};

pub fn cors() -> CorsMiddleware {
    CorsMiddleware::new()
        .allow_origin(Origin::from("*"))
        .allow_methods("*".parse::<HeaderValue>().unwrap())
        .allow_headers("*".parse::<HeaderValue>().unwrap())
        .allow_credentials(false)
}

pub struct JwtMiddleware {
    pub secret: String,
}

impl JwtMiddleware {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for JwtMiddleware {
    async fn handle(
        &self,
        mut req: Request<State>,
        next: Next<'_, State>,
    ) -> tide::Result {
        let auth_header = req.header("Authorization").map(|h| h.as_str());

        if let Some(auth_header) = auth_header {
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                match verify_token(token, &self.secret) {
                    Ok(decoded_value) => {
                        req.set_ext(decoded_value.claims.is_admin);
                        return Ok(next.run(req).await);
                    }
                    Err(_) => (),
                };
            }
        }

        Ok(Response::new(401))
    }
}
