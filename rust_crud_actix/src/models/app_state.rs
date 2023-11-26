use sqlx::{Pool, Postgres};

pub struct AppState {
    pub postgres_client: Pool<Postgres>,
}
