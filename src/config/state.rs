use sqlx::{PgPool, Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    pub pool: Arc<Pool<Postgres>>,
    pub key: String,
    pub exp: u64,
    pub upload_path: String,
}

impl State {
    pub async fn new(
        url: &str,
        key: String,
        exp: u64,
        upload_path: String,
    ) -> Self {
        Self {
            pool: Arc::new(PgPool::connect(url).await.unwrap()),
            key,
            exp,
            upload_path,
        }
    }
}
