use crate::app::schemas::Translate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateCategory {
    pub title: Translate,
}
