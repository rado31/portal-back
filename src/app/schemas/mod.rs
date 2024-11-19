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
    pub deleted: DeletedFilesJSON,
}

#[derive(Serialize, Deserialize)]
pub struct DeletedFilesJSON {
    pub movies: Vec<i32>,
    pub musics: Vec<i32>,
    pub books: Vec<i32>,
}
