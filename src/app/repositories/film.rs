use crate::app::schemas::{CreateFilm, SubCategory, Translate};
use crate::app::{queries, schemas::Film};
use serde_json::{from_str, from_value};
use sqlx::types::Json;
use sqlx::Row;
use sqlx::{query, Error, Pool, Postgres};
use std::sync::Arc;

pub async fn get_films(
    pool: Arc<Pool<Postgres>>,
    offset: i32,
    limit: i32,
) -> Result<Vec<Film>, Error> {
    let rows = query(queries::GET_FILMS)
        .bind(offset)
        .bind(limit)
        .fetch_all(&*pool)
        .await?;

    let films = rows
        .iter()
        .map(|row| {
            let v_title = row.get("title");
            let title: Translate = from_value(v_title).unwrap();

            let v_description = row.get("description");
            let description: Translate =
                serde_json::from_value(v_description).unwrap();

            let v_sc: Vec<String> = row.get("sub_categories");
            let sc: Vec<SubCategory> =
                v_sc.iter().map(|str| from_str(str).unwrap()).collect();

            Film {
                id: row.get("id"),
                title,
                description,
                duration: row.get("duration"),
                image: row.get("image"),
                sub_categories: sc,
            }
        })
        .collect();

    Ok(films)
}

pub async fn get_film() {}

pub async fn create_film(
    pool: Arc<Pool<Postgres>>,
    body: CreateFilm,
) -> Result<i32, Error> {
    let row = sqlx::query(queries::CREATE_FILM)
        .bind(Json(body.title))
        .bind(Json(body.description))
        .bind(body.duration)
        .bind(body.category_id)
        .fetch_one(&*pool)
        .await?;

    Ok(row.try_get("id").unwrap())
}
