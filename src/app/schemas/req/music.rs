use crate::app::schemas::Translate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Create {
    pub title: Translate,
}

#[derive(Deserialize)]
pub struct Update {
    pub id: u32,
    pub title: Translate,
}
