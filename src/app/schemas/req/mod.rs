use serde::Deserialize;

pub mod category;
pub mod movie;
pub mod music;

#[derive(Deserialize)]
#[serde(default)]
pub struct PaginationQuery {
    pub page: u32,
    pub count: u32,
}

impl PaginationQuery {
    pub fn page_to_offset(&mut self) {
        self.page = self.page * self.count - self.count;
    }
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self { page: 1, count: 10 }
    }
}
