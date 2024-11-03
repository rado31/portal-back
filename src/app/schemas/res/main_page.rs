use crate::app::schemas::Translate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MainPageData {
    pub id: i32,
    pub title: Translate,
    pub movies: Vec<MainPageMovie>,
}

#[derive(Serialize, Deserialize)]
pub struct MainPageMovie {
    pub id: i32,
    pub title: Translate,
    pub description: Translate,
    pub duration: i32,
    pub image: Option<String>,
}
