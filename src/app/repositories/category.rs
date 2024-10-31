use crate::app::{
    queries,
    schemas::{
        Category, CreateCategory, CreateSubCategory, Translate,
        UpdateSubCategory,
    },
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

pub async fn get_category(
    pool: Arc<Pool<Postgres>>,
    category_id: i32,
) -> Result<Category, Error> {
    let row = query(queries::GET_CATEGORY)
        .bind(category_id)
        .fetch_one(&*pool)
        .await?;

    let id = row.try_get("id")?;
    let value = row.get("title");
    let title: Translate = from_value(value).unwrap();

    let category = Category { id, title };

    Ok(category)
}

pub async fn get_sub_category(
    pool: Arc<Pool<Postgres>>,
    sub_category_id: i32,
) -> Result<Category, Error> {
    let row = query(queries::GET_SUB_CATEGORY)
        .bind(sub_category_id)
        .fetch_one(&*pool)
        .await?;

    let id = row.try_get("id")?;
    let value = row.get("title");
    let title: Translate = from_value(value).unwrap();

    let category = Category { id, title };

    Ok(category)
}

pub async fn get_sub_categories(
    pool: Arc<Pool<Postgres>>,
    category_id: i32,
) -> Result<Vec<Category>, Error> {
    let rows = query(queries::GET_SUB_CATEGORIES)
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

pub async fn create_category(
    pool: Arc<Pool<Postgres>>,
    body: CreateCategory,
) -> Result<i32, Error> {
    let row = sqlx::query(queries::CREATE_CATEGORY)
        .bind(Json(body.title))
        .fetch_one(&*pool)
        .await?;

    Ok(row.try_get("id")?)
}

pub async fn create_sub_category(
    pool: Arc<Pool<Postgres>>,
    body: CreateSubCategory,
) -> Result<i32, Error> {
    let row = sqlx::query(queries::CREATE_SUB_CATEGORY)
        .bind(Json(body.title))
        .bind(body.category_id as i32)
        .fetch_one(&*pool)
        .await?;

    Ok(row.try_get("id")?)
}

pub async fn update_sub_category(
    pool: Arc<Pool<Postgres>>,
    body: UpdateSubCategory,
) -> Result<(), Error> {
    sqlx::query(queries::UPDATE_SUB_CATEGORY)
        .bind(Json(body.title))
        .bind(body.category_id as i32)
        .bind(body.id as i32)
        .execute(&*pool)
        .await?;

    Ok(())
}

pub async fn delete_sub_category(
    pool: Arc<Pool<Postgres>>,
    sub_category_id: i32,
) -> Result<u64, Error> {
    let row = sqlx::query(queries::DELETE_SUB_CATEGORY)
        .bind(sub_category_id)
        .execute(&*pool)
        .await?;

    Ok(row.rows_affected())
}
