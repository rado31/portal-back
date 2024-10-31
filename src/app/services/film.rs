use crate::{
    app::{
        repositories,
        schemas::{CreateFilm, FilmQuery},
    },
    config::State,
};
use async_std::{
    fs::OpenOptions,
    io::{self},
};
use serde_json::json;
use tide::{http::mime::JSON, log, Request, Response, Result};

pub async fn get_films(req: Request<State>) -> Result<Response> {
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
    let pool = req.state().pool.clone();

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

pub async fn get_film(req: Request<State>) -> Result<Response> {
    let film_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::get_film(pool, film_id as i32).await {
        Ok(films) => {
            let response = Response::builder(200)
                .body(json!(films))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(sqlx::Error::RowNotFound) => Ok(Response::new(404)),
        Err(error) => {
            log::error!("Get film: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn create_film(mut req: Request<State>) -> Result<Response> {
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
    let pool = req.state().pool.clone();

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

pub async fn upload_image(req: Request<State>) -> Result<Response> {
    let film_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::get_film(pool.clone(), film_id as i32).await {
        Ok(_) => (),
        Err(sqlx::Error::RowNotFound) => return Ok(Response::new(404)),
        Err(error) => {
            log::error!("Check film exists for image upload: {error}");
            return Ok(Response::new(500));
        }
    };

    let upload_path = req.state().upload_path.clone();
    let path = format!("/uploads/images/films/{film_id}.jpg");
    let transaction = pool.begin().await.unwrap();

    match repositories::update_film_image(pool, &path, film_id as i32).await {
        Ok(_) => (),
        Err(error) => {
            log::error!("Update film image: {error}");
            transaction.rollback().await.unwrap();
            return Ok(Response::new(500));
        }
    };

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{upload_path}/images/films/{film_id}.jpg"))
        .await
        .unwrap();

    match io::copy(req, file).await {
        Ok(_) => {
            transaction.commit().await.unwrap();
            Ok(Response::new(200))
        }
        Err(error) => {
            transaction.rollback().await.unwrap();
            log::error!("Save image for film: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn upload_film(req: Request<State>) -> Result<Response> {
    let _film_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };

    Ok(Response::new(200))
}

pub async fn serve_film(req: Request<State>) -> Result<Response> {
    let video_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => {
            let segment = req.param("id").unwrap();
            let segment_path = format!("./video/result/{segment}");

            match async_std::fs::read(segment_path).await {
                Ok(segment_content) => {
                    let res = Response::builder(200)
                        .body(segment_content)
                        .content_type("video/mp4")
                        .build();

                    return Ok(res);
                }
                Err(error) => {
                    log::error!("Server segment: {error}");
                    return Ok(Response::new(404));
                }
            }
        }
    };

    let manifest_path = format!("./video/result/{video_id}.mpd");

    match async_std::fs::read(manifest_path).await {
        Ok(manifest) => {
            let res = Response::builder(200)
                .body(manifest)
                .content_type("application/dash+xml")
                .build();

            Ok(res)
        }
        Err(_) => Ok(Response::new(404)),
    }
}
