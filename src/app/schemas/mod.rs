use serde::{Deserialize, Serialize};

pub mod req;
mod res;

pub use res::*;

#[derive(Serialize, Deserialize)]
pub struct Translate {
    pub tk: String,
    pub ru: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangesJSON {
    pub movies: Vec<i32>,
    pub musics: Vec<i32>,
    pub books: Vec<i32>,
    pub images: Vec<i32>,
    pub deleted: DeletedFilesJSON,
}

#[derive(Serialize, Deserialize)]
pub struct DeletedFilesJSON {
    pub movies: Vec<i32>,
    pub musics: Vec<i32>,
    pub books: Vec<i32>,
    pub images: Vec<i32>,
}

impl ChangesJSON {
    pub fn movie_id_exists(&self, id: i32) -> bool {
        match &self.movies.iter().find(|number| **number == id) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn remove_movie(&mut self, id: i32) {
        self.movies.retain(|movie| *movie != id);
    }

    pub fn music_id_exists(&self, id: i32) -> bool {
        match &self.musics.iter().find(|number| **number == id) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn remove_music(&mut self, id: i32) {
        self.musics.retain(|music| *music != id);
    }

    pub fn book_id_exists(&self, id: i32) -> bool {
        match &self.books.iter().find(|number| **number == id) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn remove_book(&mut self, id: i32) {
        self.books.retain(|book| *book != id);
    }

    pub fn image_id_exists(&self, id: i32) -> bool {
        match &self.images.iter().find(|number| **number == id) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn remove_image(&mut self, id: i32) {
        self.images.retain(|image| *image != id);
    }
}
