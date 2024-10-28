use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Translate {
    pub tk: String,
    pub ru: String,
}
