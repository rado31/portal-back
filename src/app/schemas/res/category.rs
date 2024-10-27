use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize)]
pub struct Category {
    pub id: i32,
    pub title: Title,
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    pub tk: String,
    pub ru: String,
}
