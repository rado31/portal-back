use crate::app::schemas::Translate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Create {
    pub title: Translate,
    pub description: Translate,
    pub duration: u32,
    pub sub_categories: Vec<u32>,
}

#[derive(Deserialize)]
pub struct Update {
    pub id: u32,
    pub title: Translate,
    pub description: Translate,
    pub duration: u32,
    pub sub_categories: Vec<u32>,
}
