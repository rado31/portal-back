use crate::app::schemas::Translate;
use serde::Serialize;

#[derive(Serialize)]
pub struct Books {
    pub total: i32,
    pub books: Vec<Book>,
}

#[derive(Serialize)]
pub struct Book {
    pub id: i32,
    pub title: Translate,
    pub path: Option<String>,
}
