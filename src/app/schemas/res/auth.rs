use serde::Deserialize;

#[derive(sqlx::FromRow, Deserialize)]
pub struct Admin {
    pub login: String,
    pub password: String,
}
