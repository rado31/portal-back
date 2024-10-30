use crate::{
    app::{
        repositories,
        schemas::{CreateCategory, CreateSubCategory},
    },
    config::State,
};
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
            log::error!("Get categories: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn get_category(req: Request<State>) -> Result<Response> {
    let pool = req.state().pool.clone();
    let category_id = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };

    match repositories::get_category(pool, category_id).await {
        Ok(category) => {
            let response = Response::builder(200)
                .body(json!(category))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(sqlx::Error::RowNotFound) => Ok(Response::new(404)),
        Err(error) => {
            log::error!("Get category: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn get_sub_category(req: Request<State>) -> Result<Response> {
    let pool = req.state().pool.clone();
    let sub_category_id = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };

    match repositories::get_sub_category(pool, sub_category_id).await {
        Ok(category) => {
            let response = Response::builder(200)
                .body(json!(category))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(sqlx::Error::RowNotFound) => Ok(Response::new(404)),
        Err(error) => {
            log::error!("Get sub category: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn get_sub_categories(req: Request<State>) -> Result<Response> {
    let pool = req.state().pool.clone();
    let category_id = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };

    // check that category exists
    match repositories::get_category(pool.clone(), category_id as i32).await {
        Ok(_) => (),
        Err(sqlx::Error::RowNotFound) => return Ok(Response::new(404)),
        Err(error) => {
            log::error!("Get category for sub: {error}");
            return Ok(Response::new(500));
        }
    };

    match repositories::get_sub_categories(pool, category_id).await {
        Ok(sub_categories) => {
            let response = Response::builder(200)
                .body(json!(sub_categories))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(error) => {
            log::error!("Get sub categories: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn create_category(mut req: Request<State>) -> Result<Response> {
    let pool = req.state().pool.clone();
    let body: CreateCategory = match req.body_json().await {
        Ok(val) => val,
        Err(error) => {
            let response = Response::builder(422)
                .body(json!({ "message": format!("{error}") }))
                .content_type(JSON)
                .build();

            return Ok(response);
        }
    };

    match repositories::create_category(pool, body).await {
        Ok(id) => {
            let response = Response::builder(200)
                .body(json!({ "id": id }))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(sqlx::Error::RowNotFound) => Ok(Response::new(404)),
        Err(error) => {
            log::error!("Create category: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn create_sub_category(mut req: Request<State>) -> Result<Response> {
    let pool = req.state().pool.clone();
    let body: CreateSubCategory = match req.body_json().await {
        Ok(val) => val,
        Err(error) => {
            let response = Response::builder(422)
                .body(json!({ "message": format!("{error}") }))
                .content_type(JSON)
                .build();

            return Ok(response);
        }
    };

    // check that category exists
    match repositories::get_category(pool.clone(), body.category_id as i32)
        .await
    {
        Ok(_) => (),
        Err(sqlx::Error::RowNotFound) => return Ok(Response::new(404)),
        Err(error) => {
            log::error!("Get category for sub creation: {error}");
            return Ok(Response::new(500));
        }
    };

    match repositories::create_sub_category(pool, body).await {
        Ok(id) => {
            let response = Response::builder(200)
                .body(json!({ "id": id }))
                .content_type(JSON)
                .build();

            Ok(response)
        }
        Err(sqlx::Error::RowNotFound) => Ok(Response::new(404)),
        Err(error) => {
            log::error!("Create sub category: {error}");
            Ok(Response::new(500))
        }
    }
}
