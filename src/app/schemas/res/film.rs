use crate::app::schemas::translate::Translate;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Film {
    pub id: i32,
    pub title: Translate,
    pub description: Translate,
    pub duration: i32,
    pub image: Option<String>,
    pub sub_categories: Vec<SubCategory>,
}

#[derive(Serialize, Deserialize)]
pub struct SubCategory {
    pub id: i32,
    pub title: Translate,
}
