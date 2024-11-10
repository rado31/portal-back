use crate::app::schemas::Translate;
use serde::Serialize;

#[derive(Serialize)]
pub struct Musics {
    pub total: i32,
    pub musics: Vec<Music>,
}

#[derive(Serialize)]
pub struct Music {
    pub id: i32,
    pub title: Translate,
    pub path: Option<String>,
}
