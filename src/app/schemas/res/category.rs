use crate::app::schemas::translate::Translate;
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct Category {
    pub id: i32,
    pub title: Translate,
}
