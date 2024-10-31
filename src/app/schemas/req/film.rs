use crate::app::schemas::Translate;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct FilmQuery {
    pub page: u32,
    pub count: u32,
}

impl FilmQuery {
    pub fn page_to_offset(&mut self) {
        self.page = self.page * self.count - self.count;
    }
}

impl Default for FilmQuery {
    fn default() -> Self {
        Self { page: 1, count: 10 }
    }
}

#[derive(Deserialize)]
pub struct CreateFilm {
    pub title: Translate,
    pub description: Translate,
    pub duration: u32,
    pub sub_categories: Vec<u32>,
}

#[derive(Deserialize)]
pub struct UpdateFilm {
    pub id: u32,
    pub title: Translate,
    pub description: Translate,
    pub duration: u32,
    pub status: bool,
    pub sub_categories: Vec<u32>,
}
