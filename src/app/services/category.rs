use crate::{
    app::{repositories, schemas::CreateCategory},
    config::State,
};
use log::error;
use serde_json::json;
use tide::{http::mime::JSON, Request, Response, Result};

pub async fn get_categories(req: Request<State>) -> Result<Response> {
    let pool = req.state().pool.clone();

    match repositories::get_categories(pool).await {
        Ok(categories) => {
            let response = Response::builder(200)
                .body(json!(categories))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            error!("Get categories: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn create_category(mut req: Request<State>) -> Result<Response> {
    let pool = req.state().pool.clone();
    let body: CreateCategory = req.body_json().await?;

    match repositories::create_category(pool, body).await {
        Ok(id) => {
            let response = Response::builder(200)
                .body(json!({ "id": id }))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            error!("Create category: {error}");
            Ok(Response::new(500))
        }
    }
}
