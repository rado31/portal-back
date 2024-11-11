use serde::{Deserialize, Serialize};

pub mod req;
mod res;

pub use res::*;

#[derive(Serialize, Deserialize)]
pub struct Translate {
    pub tk: String,
    pub ru: String,
}
