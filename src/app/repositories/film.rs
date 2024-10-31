use crate::app::schemas::{CreateFilm, SubCategory, Translate};
use crate::app::{queries, schemas::Film};
use serde_json::{from_str, from_value};
use sqlx::postgres::PgRow;
use sqlx::types::Json;
use sqlx::Row;
use sqlx::{query, Error, Pool, Postgres};
use std::sync::Arc;

pub async fn get_films(
    pool: Arc<Pool<Postgres>>,
    status: bool,
    offset: i32,
    limit: i32,
) -> Result<Vec<Film>, Error> {
    #[allow(unused)]
    let mut rows: Vec<PgRow> = vec![];

    if status {
        rows = query(queries::GET_FILMS_FOR_ADMIN)
            .bind(offset)
            .bind(limit)
            .fetch_all(&*pool)
            .await?;
    } else {
        rows = query(queries::GET_FILMS)
            .bind(offset)
            .bind(limit)
            .fetch_all(&*pool)
            .await?;
    }

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
                status: row.get("status"),
                sub_categories: sc,
            }
        })
        .collect();

    Ok(films)
}

pub async fn get_film(
    pool: Arc<Pool<Postgres>>,
    film_id: i32,
) -> Result<Film, Error> {
    let row = query(queries::GET_FILM)
        .bind(film_id)
        .fetch_one(&*pool)
        .await?;

    let v_title = row.get("title");
    let title: Translate = from_value(v_title).unwrap();

    let v_description = row.get("description");
    let description: Translate = serde_json::from_value(v_description).unwrap();

    let v_sc: Vec<String> = row.get("sub_categories");
    let sc: Vec<SubCategory> =
        v_sc.iter().map(|str| from_str(str).unwrap()).collect();

    let film = Film {
        id: row.get("id"),
        title,
        description,
        duration: row.get("duration"),
        image: row.get("image"),
        status: row.get("status"),
        sub_categories: sc,
    };

    Ok(film)
}

pub async fn create_film(
    pool: Arc<Pool<Postgres>>,
    body: CreateFilm,
) -> Result<i32, Error> {
    let row = query(queries::CREATE_FILM)
        .bind(Json(body.title))
        .bind(Json(body.description))
        .bind(body.duration as i32)
        .fetch_one(&*pool)
        .await?;

    let film_id = row.try_get("id")?;

    for id in body.sub_categories {
        query(queries::CREATE_FILMS_SC)
            .bind(film_id)
            .bind(id as i32)
            .execute(&*pool)
            .await?;
    }

    Ok(film_id)
}

pub async fn update_film_image(
    pool: Arc<Pool<Postgres>>,
    path: &str,
    film_id: i32,
) -> Result<(), Error> {
    let _ = query(queries::UPDATE_FILM_IMAGE)
        .bind(path)
        .bind(film_id)
        .execute(&*pool)
        .await?;

    Ok(())
}
