use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use rust_crud_actix::{controllers, database, models::app_state::AppState};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Olá mundo")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let _pool = database::connection::start_connection().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                postgres_client: _pool.clone(),
            }))
            .service(index)
            .configure(controllers::users_controller::config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
