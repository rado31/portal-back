use crate::app::schemas::{
    req, MainPageData, MainPageMovie, Movies, SubCategory, Translate,
};
use crate::app::{queries, schemas::Movie};
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
) -> Result<Vec<Movie>, Error> {
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

            Movie {
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
) -> Result<Movie, Error> {
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

    let movie = Movie {
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

pub async fn get_movies_by_sc(
    pool: Arc<Pool<Postgres>>,
    sub_category_id: i32,
    offset: i32,
    limit: i32,
) -> Result<Movies, Error> {
    let rows = query(queries::GET_MOVIES_BY_SC)
        .bind(sub_category_id)
        .bind(offset)
        .bind(limit)
        .fetch_all(&*pool)
        .await?;

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

            Movie {
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

    let row = query(queries::GET_MOVIES_BY_SC_TOTAL)
        .bind(sub_category_id)
        .fetch_one(&*pool)
        .await?;

    let res = Movies {
        total: row.get("total"),
        movies,
    };

    Ok(res)
}

pub async fn get_main_page_data(
    pool: Arc<Pool<Postgres>>,
) -> Result<Vec<MainPageData>, Error> {
    let rows = query(queries::GET_MAIN_PAGE_DATA).fetch_all(&*pool).await?;

    let result = rows
        .iter()
        .map(|row| {
            let v_title = row.get("title");
            let title: Translate = from_value(v_title).unwrap();

            let v_movies: Vec<String> = row.get("movies");
            let movies: Vec<MainPageMovie> =
                v_movies.iter().map(|str| from_str(str).unwrap()).collect();

            MainPageData {
                id: row.get("id"),
                title,
                movies,
            }
        })
        .collect();

    Ok(result)
}

pub async fn search_movie(
    pool: Arc<Pool<Postgres>>,
    text: &str,
) -> Result<Vec<Movie>, Error> {
    let rows = query(queries::SEARCH_MOVIE)
        .bind(text)
        .fetch_all(&*pool)
        .await?;

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

            Movie {
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

pub async fn create_movie(
    pool: Arc<Pool<Postgres>>,
    body: req::movie::CreateMovie,
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
    query(queries::UPDATE_MOVIE_IMAGE)
        .bind(path)
        .bind(movie_id)
        .execute(&*pool)
        .await?;

    Ok(())
}

pub async fn update_movie(
    pool: Arc<Pool<Postgres>>,
    body: req::movie::UpdateMovie,
) -> Result<u64, Error> {
    let row = query(queries::UPDATE_MOVIE)
        .bind(Json(body.title))
        .bind(Json(body.description))
        .bind(body.duration as i32)
        .bind(body.status)
        .bind(body.id as i32)
        .execute(&*pool)
        .await?;

    let affected = row.rows_affected();

    if affected == 0 {
        return Ok(0);
    }

    query(queries::DELETE_MOVIE_SC)
        .bind(body.id as i32)
        .execute(&*pool)
        .await?;

    for sub_id in body.sub_categories {
        query(queries::CREATE_MOVIE_SC)
            .bind(body.id as i32)
            .bind(sub_id as i32)
            .execute(&*pool)
            .await?;
    }

    Ok(affected)
}

pub async fn delete_movie(
    pool: Arc<Pool<Postgres>>,
    movie_id: i32,
) -> Result<u64, Error> {
    let row = query(queries::DELETE_MOVIE)
        .bind(movie_id)
        .execute(&*pool)
        .await?;

    Ok(row.rows_affected())
}
