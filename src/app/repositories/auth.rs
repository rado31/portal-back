use crate::app::{queries, schemas::Admin};
use sqlx::{query_as, Error, Pool, Postgres, Result};
use std::sync::Arc;

pub async fn get_me(pool: Arc<Pool<Postgres>>) -> Result<Admin, Error> {
    let admin: Admin = query_as(queries::GET_ME)
        .bind("admin")
        .fetch_one(&*pool)
        .await?;

    Ok(admin)
}
