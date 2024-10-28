use crate::app::{queries, schemas::Admin};
use sqlx::{query_as, Error, Pool, Postgres, Result};
use std::sync::Arc;

pub async fn get_admin(
    pool: Arc<Pool<Postgres>>,
    login: String,
) -> Result<Admin, Error> {
    let admin: Admin = query_as(queries::GET_ADMIN)
        .bind(login)
        .fetch_one(&*pool)
        .await?;

    Ok(admin)
}
