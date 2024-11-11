use crate::app::{
    queries,
    schemas::{req, Category, Translate},
};
use serde_json::from_value;
use sqlx::Row;
use sqlx::{query, types::Json, Error, Pool, Postgres, Result};
use std::sync::Arc;

pub async fn get_category(
    pool: Arc<Pool<Postgres>>,
    category_id: i32,
) -> Result<Category, Error> {
    let row = query(queries::category::GET_CATEGORY)
        .bind(category_id)
        .fetch_one(&*pool)
        .await?;

    let id = row.try_get("id")?;
    let value = row.get("title");
    let title: Translate = from_value(value).unwrap();

    let category = Category { id, title };

    Ok(category)
}

pub async fn one(
    pool: Arc<Pool<Postgres>>,
    sub_category_id: i32,
) -> Result<Category, Error> {
    let row = query(queries::category::ONE)
        .bind(sub_category_id)
        .fetch_one(&*pool)
        .await?;

    let id = row.try_get("id")?;
    let value = row.get("title");
    let title: Translate = from_value(value).unwrap();

    let category = Category { id, title };

    Ok(category)
}

pub async fn all(
    pool: Arc<Pool<Postgres>>,
    category_id: i32,
) -> Result<Vec<Category>, Error> {
    let rows = query(queries::category::ALL)
        .bind(category_id)
        .fetch_all(&*pool)
        .await?;

    let sub_categories = rows
        .iter()
        .map(|row| {
            let id = row.get("id");
            let value = row.get("title");
            let title: Translate = from_value(value).unwrap();

            Category { id, title }
        })
        .collect();

    Ok(sub_categories)
}

pub async fn create(
    pool: Arc<Pool<Postgres>>,
    body: req::category::Create,
) -> Result<i32, Error> {
    let row = sqlx::query(queries::category::CREATE)
        .bind(Json(body.title))
        .bind(body.category_id as i32)
        .fetch_one(&*pool)
        .await?;

    Ok(row.try_get("id")?)
}

pub async fn update(
    pool: Arc<Pool<Postgres>>,
    body: req::category::Update,
) -> Result<u64, Error> {
    let row = sqlx::query(queries::category::UPDATE)
        .bind(Json(body.title))
        .bind(body.id as i32)
        .execute(&*pool)
        .await?;

    Ok(row.rows_affected())
}

pub async fn delete(
    pool: Arc<Pool<Postgres>>,
    sub_category_id: i32,
) -> Result<u64, Error> {
    let row = sqlx::query(queries::category::DELETE)
        .bind(sub_category_id)
        .execute(&*pool)
        .await?;

    Ok(row.rows_affected())
}
