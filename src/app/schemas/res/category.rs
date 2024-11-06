use crate::app::schemas::translate::Translate;
use serde::Serialize;

#[derive(Serialize)]
pub struct Category {
    pub id: i32,
    pub title: Translate,
}
