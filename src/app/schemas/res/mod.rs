use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct Admin {
    pub username: String,
    pub password: String,
}
