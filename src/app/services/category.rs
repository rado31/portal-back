use crate::{
    app::{repositories, schemas::req},
    config::State,
};
use serde_json::json;
use tide::{http::mime::JSON, Request, Response, Result};

pub async fn get_sub_category(req: Request<State>) -> Result<Response> {
    let sub_category_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::get_sub_category(pool, sub_category_id as i32).await {
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
    let category_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    // check that category exists
    match repositories::get_category(pool.clone(), category_id as i32).await {
        Ok(_) => (),
        Err(sqlx::Error::RowNotFound) => return Ok(Response::new(404)),
        Err(error) => {
            log::error!("Get category for sub: {error}");
            return Ok(Response::new(500));
        }
    };

    match repositories::get_sub_categories(pool, category_id as i32).await {
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

pub async fn create_sub_category(mut req: Request<State>) -> Result<Response> {
    let body: req::category::CreateSubCategory = match req.body_json().await {
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

pub async fn update_sub_category(mut req: Request<State>) -> Result<Response> {
    let body: req::category::UpdateSubCategory = match req.body_json().await {
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

    match repositories::update_sub_category(pool, body).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                return Ok(Response::new(404));
            }

            Ok(Response::new(200))
        }
        Err(error) => {
            log::error!("Update sub category: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn delete_sub_category(req: Request<State>) -> Result<Response> {
    let sub_category_id: u32 = match req.param("id").unwrap().parse() {
        Ok(id) => id,
        Err(_) => return Ok(Response::new(422)),
    };
    let pool = req.state().pool.clone();

    match repositories::delete_sub_category(pool, sub_category_id as i32).await
    {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                return Ok(Response::new(404));
            }

            Ok(Response::new(200))
        }
        Err(error) => {
            log::error!("Delete sub category: {error}");
            Ok(Response::new(500))
        }
    }
}
