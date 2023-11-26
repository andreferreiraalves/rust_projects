use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse,
};

use crate::{models::todo::Todo, repository::database::Database};

#[post("/todos")]
pub async fn create_todo(db: Data<Database>, new_todo: Json<Todo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());

    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos")]
pub async fn get_todos(db: web::Data<Database>) -> HttpResponse {
    let todos = db.get_todos();
    HttpResponse::Ok().json(todos)
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    match db.get_todo_by_id(&id) {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_todo)
            .service(get_todos)
            .service(get_todo_by_id),
    );
}
