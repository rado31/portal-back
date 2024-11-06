use crate::app::schemas::Translate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateSubCategory {
    pub category_id: u32,
    pub title: Translate,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateSubCategory {
    pub id: u32,
    pub title: Translate,
}
