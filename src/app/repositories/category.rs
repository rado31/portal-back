use crate::app::{
    queries,
    schemas::{Category, CreateCategory, Translate},
};
use serde_json::from_value;
use sqlx::Row;
use sqlx::{query, types::Json, Error, Pool, Postgres, Result};
use std::sync::Arc;

pub async fn get_categories(
    pool: Arc<Pool<Postgres>>,
) -> Result<Vec<Category>, Error> {
    let rows = query(queries::GET_CATEGORIES).fetch_all(&*pool).await?;

    let categories = rows
        .iter()
        .map(|row| {
            let value = row.get("title");
            let title: Translate = from_value(value).unwrap();

            Category {
                id: row.get("id"),
                title,
            }
        })
        .collect();

    Ok(categories)
}

pub async fn create_category(
    pool: Arc<Pool<Postgres>>,
    body: CreateCategory,
) -> Result<i32, Error> {
    let row = sqlx::query(queries::CREATE_CATEGORY)
        .bind(Json(body.title))
        .fetch_one(&*pool)
        .await?;

    Ok(row.try_get("id").unwrap())
}
