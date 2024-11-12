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

    query.page_to_offset();

    match repositories::book::all(pool, query.page as i32, query.count as i32)
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
            log::error!("Get books: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn one(req: Request<State>) -> Result<Response> {
    let book_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::book::one(pool, book_id as i32).await {
        Ok(music) => {
            let response = Response::builder(200)
                .body(json!(music))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Get book: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn create(mut req: Request<State>) -> Result<Response> {
    let body: req::book::Create = match req.body_json().await {
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

    match repositories::book::create(pool, body).await {
        Ok(id) => {
            let response = Response::builder(200)
                .body(json!({ "id": id }))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Create book: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn upload(req: Request<State>) -> Result<Response> {
    let book_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::book::one(pool.clone(), book_id as i32).await {
        Ok(_) => (),
        Err(sqlx::Error::RowNotFound) => return Ok(Response::new(404)),
        Err(error) => {
            log::error!("Check book exists for upload: {error}");
            return Ok(Response::new(500));
        }
    };

    let upload_path = req.state().upload_path.clone();
    let path = format!("/uploads/books/{book_id}.pdf");
    let transaction = pool.begin().await.unwrap();

    if let Err(error) =
        repositories::book::update_book_path(pool, &path, book_id as i32).await
    {
        transaction.rollback().await.unwrap();
        log::error!("Update book path: {error}");
        return Ok(Response::new(500));
    };

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{upload_path}/books/{book_id}.pdf"))
        .await
        .unwrap();

    if let Err(error) = io::copy(req, file).await {
        transaction.rollback().await.unwrap();
        log::error!("Save book path: {error}");
        return Ok(Response::new(500));
    }

    transaction.commit().await.unwrap();
    Ok(Response::new(200))
}

pub async fn update(mut req: Request<State>) -> Result<Response> {
    let body: req::book::Update = match req.body_json().await {
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

    match repositories::book::update(pool, body).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                return Ok(Response::new(404));
            }

            Ok(Response::new(200))
        }
        Err(error) => {
            log::error!("Update book: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn delete(req: Request<State>) -> Result<Response> {
    let book_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::book::delete(pool, book_id as i32).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                return Ok(Response::new(404));
            }

            let upload_path = req.state().upload_path.clone();

            if let Err(error) =
                fs::remove_file(format!("{upload_path}/books/{book_id}.pdf"))
                    .await
            {
                log::error!("Remove book: {error}");
            }

            Ok(Response::new(200))
        }
        Err(error) => {
            log::error!("Delete book: {error}");
            Ok(Response::new(500))
        }
    }
}
