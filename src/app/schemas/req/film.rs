use crate::app::schemas::Translate;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct FilmQuery {
    pub page: i32,
    pub count: i32,
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
    pub duration: i32,
    pub category_id: i32,
}
