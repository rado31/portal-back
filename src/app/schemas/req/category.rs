use crate::app::schemas::Translate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Create {
    pub category_id: u32,
    pub title: Translate,
}

#[derive(Deserialize, Serialize)]
pub struct Update {
    pub id: u32,
    pub title: Translate,
}
