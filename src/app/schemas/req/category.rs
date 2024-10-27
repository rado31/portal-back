use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateCategory {
    pub title: Translate,
}

#[derive(Deserialize, Serialize)]
pub struct Translate {
    pub tk: String,
    pub ru: String,
}
