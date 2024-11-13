use crate::app::{
    queries,
    schemas::{req, Music, Musics, Translate},
};
use serde_json::from_value;
use sqlx::{postgres::PgRow, query, Error, Pool, Postgres};
use sqlx::{types::Json, Row};
use std::sync::Arc;

pub async fn all(
    pool: Arc<Pool<Postgres>>,
    status: bool,
    offset: i32,
    limit: i32,
) -> Result<Musics, Error> {
    #[allow(unused)]
    let mut rows: Vec<PgRow> = vec![];

    if status {
        rows = query(queries::music::ALL_FOR_ADMIN)
            .bind(offset)
            .bind(limit)
            .fetch_all(&*pool)
            .await?;
    } else {
        rows = query(queries::music::ALL)
            .bind(offset)
            .bind(limit)
            .fetch_all(&*pool)
            .await?;
    }

    let musics = rows
        .iter()
        .map(|row| {
            let v_title = row.get("title");
            let title: Translate = from_value(v_title).unwrap();

            Music {
                id: row.get("id"),
                title,
                path: row.get("path"),
            }
        })
        .collect();

    let row = query(queries::music::TOTAL).fetch_one(&*pool).await?;

    let res = Musics {
        total: row.get("total"),
        musics,
    };

    Ok(res)
}

pub async fn one(
    pool: Arc<Pool<Postgres>>,
    music_id: i32,
) -> Result<Music, Error> {
    let row = query(queries::music::ONE)
        .bind(music_id)
        .fetch_one(&*pool)
        .await?;

    let v_title = row.get("title");
    let title: Translate = from_value(v_title).unwrap();

    let music = Music {
        id: row.try_get("id")?,
        title,
        path: row.get("path"),
    };

    Ok(music)
}

pub async fn create(
    pool: Arc<Pool<Postgres>>,
    body: req::music::Create,
) -> Result<i32, Error> {
    let row = query(queries::music::CREATE)
        .bind(Json(body.title))
        .fetch_one(&*pool)
        .await?;

    Ok(row.try_get("id")?)
}

pub async fn update_music_path(
    pool: Arc<Pool<Postgres>>,
    music_path: &str,
    music_id: i32,
) -> Result<u64, Error> {
    let row = query(queries::music::UPDATE_MUSIC_PATH)
        .bind(music_path)
        .bind(music_id)
        .execute(&*pool)
        .await?;

    Ok(row.rows_affected())
}

pub async fn update(
    pool: Arc<Pool<Postgres>>,
    body: req::music::Update,
) -> Result<u64, Error> {
    let row = query(queries::music::UPDATE)
        .bind(Json(body.title))
        .bind(body.id as i32)
        .execute(&*pool)
        .await?;

    Ok(row.rows_affected())
}

pub async fn delete(
    pool: Arc<Pool<Postgres>>,
    music_id: i32,
) -> Result<u64, Error> {
    let row = query(queries::music::DELETE)
        .bind(music_id)
        .execute(&*pool)
        .await?;

    Ok(row.rows_affected())
}
