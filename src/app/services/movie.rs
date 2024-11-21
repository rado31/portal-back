use crate::{
    app::{repositories, schemas::req},
    config::State,
    utils::{
        count_total_frames, get_changes_json, save_file, set_changes_json,
    },
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

pub async fn all_for_admin(req: Request<State>) -> Result<Response> {
    let pool = req.state().pool.clone();

    match repositories::movie::all_for_admin(pool).await {
        Ok(movies) => {
            let response = Response::builder(200)
                .body(json!(movies))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Get movies for admin: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn all(req: Request<State>) -> Result<Response> {
    let mut query: req::PaginationQuery = match req.query() {
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

    match repositories::movie::all(pool, query.page as i32, query.count as i32)
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

pub async fn one(req: Request<State>) -> Result<Response> {
    let movie_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::movie::one(pool, movie_id as i32).await {
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

pub async fn all_by_sc(req: Request<State>) -> Result<Response> {
    let sub_category_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let mut query: req::PaginationQuery = match req.query() {
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

    match repositories::movie::all_by_sc(
        pool,
        sub_category_id as i32,
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
        Err(sqlx::Error::RowNotFound) => Ok(Response::new(404)),
        Err(error) => {
            log::error!("Get movies by sub category: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn main_page(req: Request<State>) -> Result<Response> {
    let pool = req.state().pool.clone();

    match repositories::movie::main_page(pool).await {
        Ok(data) => {
            let response = Response::builder(200)
                .body(json!(data))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Get main page data: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn search(req: Request<State>) -> Result<Response> {
    let text = req.param("text").unwrap();
    let pool = req.state().pool.clone();

    match repositories::movie::search(pool, text).await {
        Ok(movies) => {
            let response = Response::builder(200)
                .body(json!(movies))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Search movie: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn create(mut req: Request<State>) -> Result<Response> {
    let body: req::movie::Create = match req.body_json().await {
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

    match repositories::movie::create(pool, body).await {
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

    match repositories::movie::one(pool.clone(), movie_id as i32).await {
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

    if let Err(error) =
        repositories::movie::update_image_path(pool, &path, movie_id as i32)
            .await
    {
        transaction.rollback().await.unwrap();
        log::error!("Update movie image: {error}");
        return Ok(Response::new(500));
    };

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{upload_path}/images/movies/{movie_id}.jpg"))
        .await
        .unwrap();

    if let Err(error) = io::copy(req, file).await {
        transaction.rollback().await.unwrap();
        log::error!("Save image for movie: {error}");
        return Ok(Response::new(500));
    }

    transaction.commit().await.unwrap();
    Ok(Response::new(200))
}

pub async fn upload(req: Request<State>) -> Result<Response> {
    let movie_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::movie::one(pool.clone(), movie_id as i32).await {
        Ok(_) => (),
        Err(sqlx::Error::RowNotFound) => return Ok(Response::new(404)),
        Err(error) => {
            log::error!("Check movie exists for movie upload: {error}");
            return Ok(Response::new(500));
        }
    };

    let upload_path = req.state().upload_path.clone();
    let folder = format!("{upload_path}/movies/{movie_id}");

    if let Ok(_) = fs::metadata(format!("{folder}/movie.mp4")).await {
        return Ok(Response::new(418));
    };

    if let Err(error) = fs::create_dir(&folder).await {
        if error.kind() != io::ErrorKind::AlreadyExists {
            log::error!("Folder creation for movie: {error}");
            return Ok(Response::new(500));
        }

        fs::remove_dir_all(&folder).await.unwrap();
        fs::create_dir(&folder).await.unwrap();
    };

    if let Err(error) = save_file(format!("{folder}/movie.mp4"), req).await {
        log::error!("Save movie: {error}");
        return Ok(Response::new(500));
    }

    Ok(Response::new(200))
}

pub async fn serve(req: Request<State>) -> Result<Response> {
    let upload_path = req.state().upload_path.clone();
    let movie_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => {
            let segment = req.param("id").unwrap();
            // first char of segment's name is ID
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

    if let Ok(manifest) = fs::read(manifest_path).await {
        let res = Response::builder(200)
            .body(manifest)
            .content_type("application/dash+xml")
            .build();

        return Ok(res);
    }

    Ok(Response::new(404))
}

pub async fn fraction(
    req: Request<State>,
    sender: tide::sse::Sender,
) -> Result<()> {
    let movie_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(error) => return Err(error.into()),
    };
    let upload_path = req.state().upload_path.clone();
    let folder = format!("{upload_path}/movies/{movie_id}");
    let total_frames = count_total_frames(&format!("{folder}/movie.mp4"));
    let mut process = Command::new("ffmpeg")
        .args([
            "-y",
            "-loglevel",
            "error",
            "-i",
            &format!("{folder}/movie.mp4"),
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
            &format!("{folder}/manifest.mpd"),
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(stdout) = process.stdout.take() {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let text = line.unwrap();

            if text.starts_with("frame=") {
                let splitted: Vec<&str> = text.split('=').collect();
                let frame: i32 = splitted[1].parse().unwrap();
                let percent: i32 = frame * 100 / total_frames;
                sender
                    .send("message", percent.to_string(), None)
                    .await
                    .map_or_else(|_| (), |_| ());
            }
        }
    }

    if let Err(error) = fs::remove_file(format!("{folder}/movie.mp4")).await {
        log::error!("Remove movie: {error}");
    }

    let pool = req.state().pool.clone();
    if let Err(error) =
        repositories::movie::update_upload_status(pool, movie_id as i32).await
    {
        log::error!("Update upload status: {error}");
    }

    // write to changes.json
    let mut json_file = get_changes_json();

    if !json_file.movie_id_exists(movie_id as i32) {
        json_file.movies.push(movie_id as i32);
    };

    set_changes_json(&json_file);

    Ok(())
}

pub async fn update(mut req: Request<State>) -> Result<Response> {
    let body: req::movie::Update = match req.body_json().await {
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

    match repositories::movie::update(pool, body).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                transaction.rollback().await.unwrap();
                return Ok(Response::new(404));
            }

            transaction.commit().await.unwrap();
            Ok(Response::new(200))
        }
        Err(error) => {
            transaction.rollback().await.unwrap();
            log::error!("Update movie: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn delete(req: Request<State>) -> Result<Response> {
    let movie_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::movie::delete(pool, movie_id as i32).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                return Ok(Response::new(404));
            }

            let upload_path = req.state().upload_path.clone();

            if let Err(error) =
                fs::remove_dir_all(format!("{upload_path}/movies/{movie_id}"))
                    .await
            {
                log::error!("Remove movie's folder: {error}");
            }

            if let Err(error) = fs::remove_file(format!(
                "{upload_path}/images/movies/{movie_id}.jpg"
            ))
            .await
            {
                log::error!("Remove movie's image: {error}");
            }

            // write to changes.json
            let mut json_file = get_changes_json();

            json_file.deleted.movies.push(movie_id as i32);
            json_file.remove_movie(movie_id as i32);

            set_changes_json(&json_file);

            Ok(Response::new(200))
        }
        Err(error) => {
            log::error!("Delete movie: {error}");
            Ok(Response::new(500))
        }
    }
}
