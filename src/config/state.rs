use sqlx::{PgPool, Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    pub pool: Arc<Pool<Postgres>>,
}

impl State {
    pub async fn new(url: &str) -> Self {
        Self {
            pool: Arc::new(PgPool::connect(url).await.unwrap()),
        }
    }
}
