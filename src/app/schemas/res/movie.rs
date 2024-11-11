use crate::app::schemas::Translate;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Movies {
    pub total: i32,
    pub movies: Vec<Movie>,
}

#[derive(Serialize)]
pub struct Movie {
    pub id: i32,
    pub title: Translate,
    pub description: Translate,
    pub duration: i32,
    pub image: Option<String>,
    pub status: bool,
    pub is_uploaded: bool,
    pub sub_categories: Vec<SubCategory>,
}

#[derive(Serialize, Deserialize)]
pub struct SubCategory {
    pub id: Option<i32>,
    pub title: Option<Translate>,
}
