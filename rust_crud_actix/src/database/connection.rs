use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn start_connection() -> Pool<Postgres> {
    let postgres_enviroment = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&postgres_enviroment)
        .await
        .expect("Failed to connect to Postgres.");

    let check_migrate = sqlx::migrate!("./src/database/migrations").run(&pool).await;

    match check_migrate {
        Ok(_) => println!("Migrations ran successfully"),
        Err(e) => println!("Error runing migrations {:?}", e),
    }

    pool
}
