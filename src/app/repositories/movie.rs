use crate::app::schemas::{Createmovie, SubCategory, Translate};
use crate::app::{queries, schemas::movie};
use serde_json::{from_str, from_value};
use sqlx::postgres::PgRow;
use sqlx::types::Json;
use sqlx::Row;
use sqlx::{query, Error, Pool, Postgres};
use std::sync::Arc;

pub async fn get_movies(
    pool: Arc<Pool<Postgres>>,
    status: bool,
    offset: i32,
    limit: i32,
) -> Result<Vec<movie>, Error> {
    #[allow(unused)]
    let mut rows: Vec<PgRow> = vec![];

    if status {
        rows = query(queries::GET_MOVIES_FOR_ADMIN)
            .bind(offset)
            .bind(limit)
            .fetch_all(&*pool)
            .await?;
    } else {
        rows = query(queries::GET_MOVIES)
            .bind(offset)
            .bind(limit)
            .fetch_all(&*pool)
            .await?;
    }

    let movies = rows
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

            movie {
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

    Ok(movies)
}

pub async fn get_movie(
    pool: Arc<Pool<Postgres>>,
    movie_id: i32,
) -> Result<movie, Error> {
    let row = query(queries::GET_MOVIE)
        .bind(movie_id)
        .fetch_one(&*pool)
        .await?;

    let v_title = row.get("title");
    let title: Translate = from_value(v_title).unwrap();

    let v_description = row.get("description");
    let description: Translate = serde_json::from_value(v_description).unwrap();

    let v_sc: Vec<String> = row.get("sub_categories");
    let sc: Vec<SubCategory> =
        v_sc.iter().map(|str| from_str(str).unwrap()).collect();

    let movie = movie {
        id: row.get("id"),
        title,
        description,
        duration: row.get("duration"),
        image: row.get("image"),
        status: row.get("status"),
        sub_categories: sc,
    };

    Ok(movie)
}

pub async fn create_movie(
    pool: Arc<Pool<Postgres>>,
    body: Createmovie,
) -> Result<i32, Error> {
    let row = query(queries::CREATE_MOVIE)
        .bind(Json(body.title))
        .bind(Json(body.description))
        .bind(body.duration as i32)
        .fetch_one(&*pool)
        .await?;

    let movie_id = row.try_get("id")?;

    for id in body.sub_categories {
        query(queries::CREATE_MOVIE_SC)
            .bind(movie_id)
            .bind(id as i32)
            .execute(&*pool)
            .await?;
    }

    Ok(movie_id)
}

pub async fn update_movie_image(
    pool: Arc<Pool<Postgres>>,
    path: &str,
    movie_id: i32,
) -> Result<(), Error> {
    let _ = query(queries::UPDATE_MOVIE_IMAGE)
        .bind(path)
        .bind(movie_id)
        .execute(&*pool)
        .await?;

    Ok(())
}
