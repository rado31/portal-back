use crate::{
    app::{
        repositories,
        schemas::{CreateMovie, MovieQuery},
    },
    config::State,
    utils::{count_total_frames, save_file},
};
use async_std::{
    fs::{self, OpenOptions},
    io::{self},
};
use serde_json::json;
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};
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
    let upload_path = req.state().upload_path.clone();
    let movie_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => {
            // movie id
            let segment = req.param("id").unwrap();
            let splitted_seg: Vec<&str> = segment.split('_').collect();
            let segment_path =
                format!("{upload_path}/movies/{}/{segment}", splitted_seg[0]);

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

    let manifest_path = format!("{upload_path}/movies/{movie_id}/manifest.mpd");

    match fs::read(manifest_path).await {
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

pub async fn fraction_movie(
    req: Request<State>,
    sender: tide::sse::Sender,
) -> Result<()> {
    let movie_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(error) => return Err(error.into()),
    };
    let upload_path = req.state().upload_path.clone();
    let movie_path = format!("{upload_path}/movies/{movie_id}/movie.mp4");
    let output_path = format!("{upload_path}/movies/{movie_id}/manifest.mpd");
    let total_frames = count_total_frames(&movie_path);
    let mut process = Command::new("ffmpeg")
        .args([
            "-y",
            "-loglevel",
            "error",
            "-i",
            &movie_path,
            "-map",
            "0",
            "-f",
            "dash",
            "-init_seg_name",
            &format!("{movie_id}_init-stream$RepresentationID$.$ext$"),
            "-media_seg_name",
            &format!(
                "{movie_id}_chunk-stream$RepresentationID$-$Number%05d$.$ext$"
            ),
            "-progress",
            "pipe:1",
            &output_path,
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(stdout) = process.stdout.take() {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            match line {
                Ok(text) => {
                    if text.starts_with("frame=") {
                        let splitted: Vec<&str> = text.split('=').collect();
                        let frame: i32 = splitted[1].parse().unwrap();
                        let percent: i32 = frame * 100 / total_frames;
                        match sender
                            .send("message", percent.to_string(), None)
                            .await
                        {
                            Ok(_) => (),
                            Err(_) => (),
                        };
                    }
                }
                Err(error) => {
                    log::error!("Read line of ffmpeg process: {error}");
                }
            }
        }
    }

    match fs::remove_file(format!("{upload_path}/movies/{movie_id}/movie.mp4"))
        .await
    {
        Ok(_) => Ok(()),
        Err(error) => {
            log::error!("Delete movie: {error}");
            Ok(())
        }
    }
}
