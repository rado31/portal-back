use crate::app::schemas::Translate;
use serde::Serialize;

#[derive(Serialize)]
pub struct Category {
    pub id: i32,
    pub title: Translate,
}
