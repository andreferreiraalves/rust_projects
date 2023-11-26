use actix_web::{middleware, web, App, HttpServer};
use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use handlers::*;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod handlers;
mod models;
mod schema;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let log_level = std::env::var("LOG_LEVEL").expect(".env, LOG_LEVEL not found");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or(log_level));

    let database_url = std::env::var("DATABASE_URL").expect(".env, DATABASE_URL not found");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(index)
            .service(create)
            .service(show)
            .service(update)
            .service(delete)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
