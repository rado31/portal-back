use crate::app::{
    queries,
    schemas::{req, Book, Books, Translate},
};
use serde_json::from_value;
use sqlx::{query, Error, Pool, Postgres};
use sqlx::{types::Json, Row};
use std::sync::Arc;

pub async fn all(
    pool: Arc<Pool<Postgres>>,
    offset: i32,
    limit: i32,
) -> Result<Books, Error> {
    let rows = query(queries::book::ALL)
        .bind(offset)
        .bind(limit)
        .fetch_all(&*pool)
        .await?;

    let books = rows
        .iter()
        .map(|row| {
            let v_title = row.get("title");
            let title: Translate = from_value(v_title).unwrap();

            Book {
                id: row.get("id"),
                title,
                path: row.get("path"),
            }
        })
        .collect();

    let row = query(queries::book::TOTAL).fetch_one(&*pool).await?;

    let res = Books {
        total: row.get("total"),
        books,
    };

    Ok(res)
}

pub async fn one(
    pool: Arc<Pool<Postgres>>,
    book_id: i32,
) -> Result<Book, Error> {
    let row = query(queries::book::ONE)
        .bind(book_id)
        .fetch_one(&*pool)
        .await?;

    let v_title = row.get("title");
    let title: Translate = from_value(v_title).unwrap();

    let book = Book {
        id: row.try_get("id")?,
        title,
        path: row.get("path"),
    };

    Ok(book)
}

pub async fn create(
    pool: Arc<Pool<Postgres>>,
    body: req::book::Create,
) -> Result<i32, Error> {
    let row = query(queries::book::CREATE)
        .bind(Json(body.title))
        .fetch_one(&*pool)
        .await?;

    Ok(row.try_get("id")?)
}

pub async fn update_book_path(
    pool: Arc<Pool<Postgres>>,
    book_path: &str,
    book_id: i32,
) -> Result<u64, Error> {
    let row = query(queries::book::UPDATE_BOOK_PATH)
        .bind(book_path)
        .bind(book_id)
        .execute(&*pool)
        .await?;

    Ok(row.rows_affected())
}

pub async fn update(
    pool: Arc<Pool<Postgres>>,
    body: req::book::Update,
) -> Result<u64, Error> {
    let row = query(queries::book::UPDATE)
        .bind(Json(body.title))
        .bind(body.id as i32)
        .execute(&*pool)
        .await?;

    Ok(row.rows_affected())
}

pub async fn delete(
    pool: Arc<Pool<Postgres>>,
    book_id: i32,
) -> Result<u64, Error> {
    let row = query(queries::book::DELETE)
        .bind(book_id)
        .execute(&*pool)
        .await?;

    Ok(row.rows_affected())
}
