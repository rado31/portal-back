use crate::{
    app::{repositories, schemas::req},
    config::State,
};
use async_std::{
    fs::{self, OpenOptions},
    io,
};
use serde_json::json;
use tide::{http::mime::JSON, Request, Response, Result};

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
    //let _is_admin: bool = *req.ext().unwrap();

    query.page_to_offset();

    match repositories::music::all(
        pool,
        true,
        query.page as i32,
        query.count as i32,
    )
    .await
    {
        Ok(musics) => {
            let response = Response::builder(200)
                .body(json!(musics))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Get musics: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn one(req: Request<State>) -> Result<Response> {
    let music_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::music::one(pool, music_id as i32).await {
        Ok(music) => {
            let response = Response::builder(200)
                .body(json!(music))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Get music: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn create(mut req: Request<State>) -> Result<Response> {
    let body: req::music::Create = match req.body_json().await {
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

    match repositories::music::create(pool, body).await {
        Ok(id) => {
            let response = Response::builder(200)
                .body(json!({ "id": id }))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Create music: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn upload(req: Request<State>) -> Result<Response> {
    let music_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::music::one(pool.clone(), music_id as i32).await {
        Ok(_) => (),
        Err(sqlx::Error::RowNotFound) => return Ok(Response::new(404)),
        Err(error) => {
            log::error!("Check music exists for upload: {error}");
            return Ok(Response::new(500));
        }
    };

    let upload_path = req.state().upload_path.clone();
    let path = format!("/uploads/musics/{music_id}.mp3");
    let transaction = pool.begin().await.unwrap();

    if let Err(error) =
        repositories::music::update_music_path(pool, &path, music_id as i32)
            .await
    {
        transaction.rollback().await.unwrap();
        log::error!("Update music path: {error}");
        return Ok(Response::new(500));
    };

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{upload_path}/musics/{music_id}.mp3"))
        .await
        .unwrap();

    if let Err(error) = io::copy(req, file).await {
        transaction.rollback().await.unwrap();
        log::error!("Save music path: {error}");
        return Ok(Response::new(500));
    }

    transaction.commit().await.unwrap();
    Ok(Response::new(200))
}

pub async fn update(mut req: Request<State>) -> Result<Response> {
    let body: req::music::Update = match req.body_json().await {
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

    match repositories::music::update(pool, body).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                return Ok(Response::new(404));
            }

            Ok(Response::new(200))
        }
        Err(error) => {
            log::error!("Update music: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn delete(req: Request<State>) -> Result<Response> {
    let music_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::music::delete(pool, music_id as i32).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                return Ok(Response::new(404));
            }

            let upload_path = req.state().upload_path.clone();

            if let Err(error) =
                fs::remove_file(format!("{upload_path}/musics/{music_id}.mp3"))
                    .await
            {
                log::error!("Remove music: {error}");
            }

            Ok(Response::new(200))
        }
        Err(error) => {
            log::error!("Delete music: {error}");
            Ok(Response::new(500))
        }
    }
}
