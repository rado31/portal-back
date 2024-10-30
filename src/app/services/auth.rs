use crate::{
    app::{repositories, schemas::Admin},
    config::State,
    utils::create_token,
};
use serde_json::json;
use tide::{http::mime::JSON, Request, Response, Result};

pub async fn sign_in(mut req: Request<State>) -> Result<Response> {
    let body: Admin = match req.body_json().await {
        Ok(val) => val,
        Err(error) => {
            let response = Response::builder(422)
                .body(json!({ "message": format!("{error}") }))
                .content_type(JSON)
                .build();

            return Ok(response);
        }
    };
    let pool = req.state().pool.clone();
    let key = req.state().key.clone();
    let exp = req.state().exp;

    match repositories::get_admin(pool, body.login).await {
        Ok(admin) => {
            if !bcrypt::verify(body.password, &admin.password).unwrap() {
                return Ok(Response::new(403));
            };

            let response = Response::builder(200)
                .body(json!({ "token": create_token(exp, &key) }))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Get admin: {error}");
            Ok(Response::new(500))
        }
    }
}
