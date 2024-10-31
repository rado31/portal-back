use crate::{
    app::{
        repositories,
        schemas::{CreateMovie, MovieQuery},
    },
    config::State,
    utils::save_file,
};
use async_std::{
    fs::{self, OpenOptions},
    io::{self},
};
use serde_json::json;
use tide::{http::mime::JSON, log, Request, Response, Result};

pub async fn get_movies(req: Request<State>) -> Result<Response> {
    let mut query: MovieQuery = match req.query() {
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
    //let _is_admin: bool = *req.ext().unwrap();

    query.page_to_offset();

    match repositories::get_movies(
        pool,
        true, // TO-DO for release set to variable above "is_admin"
        query.page as i32,
        query.count as i32,
    )
    .await
    {
        Ok(movies) => {
            let response = Response::builder(200)
                .body(json!(movies))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Get movies: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn get_movie(req: Request<State>) -> Result<Response> {
    let movie_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::get_movie(pool, movie_id as i32).await {
        Ok(movies) => {
            let response = Response::builder(200)
                .body(json!(movies))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(sqlx::Error::RowNotFound) => Ok(Response::new(404)),
        Err(error) => {
            log::error!("Get movie: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn create_movie(mut req: Request<State>) -> Result<Response> {
    let body: CreateMovie = match req.body_json().await {
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
    let transaction = pool.begin().await.unwrap();

    match repositories::create_movie(pool, body).await {
        Ok(id) => {
            transaction.commit().await.unwrap();
            let response = Response::builder(200)
                .body(json!({ "id": id }))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            transaction.rollback().await.unwrap();
            log::error!("Create movie: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn upload_image(req: Request<State>) -> Result<Response> {
    let movie_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::get_movie(pool.clone(), movie_id as i32).await {
        Ok(_) => (),
        Err(sqlx::Error::RowNotFound) => return Ok(Response::new(404)),
        Err(error) => {
            log::error!("Check movie exists for image upload: {error}");
            return Ok(Response::new(500));
        }
    };

    let upload_path = req.state().upload_path.clone();
    let path = format!("/uploads/images/movies/{movie_id}.jpg");
    let transaction = pool.begin().await.unwrap();

    match repositories::update_movie_image(pool, &path, movie_id as i32).await {
        Ok(_) => (),
        Err(error) => {
            transaction.rollback().await.unwrap();
            log::error!("Update movie image: {error}");
            return Ok(Response::new(500));
        }
    };

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{upload_path}/images/movies/{movie_id}.jpg"))
        .await
        .unwrap();

    match io::copy(req, file).await {
        Ok(_) => {
            transaction.commit().await.unwrap();
            Ok(Response::new(200))
        }
        Err(error) => {
            transaction.rollback().await.unwrap();
            log::error!("Save image for movie: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn upload_movie(req: Request<State>) -> Result<Response> {
    let movie_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::get_movie(pool.clone(), movie_id as i32).await {
        Ok(_) => (),
        Err(sqlx::Error::RowNotFound) => return Ok(Response::new(404)),
        Err(error) => {
            log::error!("Check movie exists for movie upload: {error}");
            return Ok(Response::new(500));
        }
    };

    let upload_path = req.state().upload_path.clone();
    let movie_path = format!("{upload_path}/movies/{movie_id}");
    match fs::create_dir(&movie_path).await {
        Ok(_) => (),
        Err(error) => {
            log::error!("Folder creation for movie: {error}");
            return Ok(Response::new(500));
        }
    };

    match save_file(format!("{movie_path}/movie.mp4"), req).await {
        Ok(_) => Ok(Response::new(200)),
        Err(error) => {
            log::error!("Save movie: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn serve_movie(req: Request<State>) -> Result<Response> {
    let video_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => {
            // video id
            let url_split: Vec<&str> = req.url().path().split('/').collect();
            let segment = req.param("id").unwrap();
            let segment_path = format!("./video/{}/{segment}", url_split[2]);

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

    let manifest_path = format!("./video/{video_id}/manifest.mpd");

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
