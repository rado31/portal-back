use crate::{
    app::{
        repositories,
        schemas::{CreateFilm, FilmQuery},
    },
    config::State,
};
use serde_json::json;
use tide::{http::mime::JSON, Request, Response, Result};

pub async fn get_films(req: Request<State>) -> Result<Response> {
    let pool = req.state().pool.clone();
    let mut query: FilmQuery = match req.query() {
        Ok(val) => val,
        Err(error) => {
            let response = Response::builder(422)
                .body(json!({ "message": format!("{error}") }))
                .content_type(JSON)
                .build();

            return Ok(response);
        }
    };

    query.page_to_offset();

    match repositories::get_films(pool, query.page, query.count).await {
        Ok(films) => {
            let response = Response::builder(200)
                .body(json!(films))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Get films: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn get_film(_req: Request<State>) -> Result<Response> {
    Ok(Response::new(200))
}

pub async fn create_film(mut req: Request<State>) -> Result<Response> {
    let pool = req.state().pool.clone();
    let body: CreateFilm = match req.body_json().await {
        Ok(val) => val,
        Err(error) => {
            let response = Response::builder(422)
                .body(json!({ "message": format!("{error}") }))
                .content_type(JSON)
                .build();

            return Ok(response);
        }
    };

    match repositories::create_film(pool, body).await {
        Ok(id) => {
            let response = Response::builder(200)
                .body(json!({ "id": id }))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Create film: {error}");
            Ok(Response::new(500))
        }
    }
}
