use actix_web::{body, get, post, web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};

use crate::models::{app_state::AppState, user::User};

#[get("")]
async fn get_all_users(app_state: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query!("select * from users")
        .fetch_all(&app_state.postgres_client)
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(
            users
                .iter()
                .map(|u| User::new(u.id, u.name.clone(), u.email.clone(), u.password.clone()))
                .collect::<Vec<User>>(),
        ),
        Err(_) => {
            HttpResponse::InternalServerError().body("Error trying to get all users from database")
        }
    }
}

#[post("")]
async fn create_user(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let password_hash = hash(&user.password, DEFAULT_COST).expect("Failed on hash password");

    if !(password_hash != user.password) {
        return HttpResponse::InternalServerError().body("Error trying to hash password");
    }

    let result = sqlx::query!(
        "insert into users (name, email, password) values($1, $2, $3)",
        user.name,
        user.email,
        password_hash
    )
    .fetch_one(&app_state.postgres_client)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(User {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            password: "".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().body("Error trying to create user"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_all_users)
            .service(create_user),
    );
}
