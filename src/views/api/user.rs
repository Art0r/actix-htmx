use actix_web::{get, http::header::ContentType, post, put, web::{self}, Error, HttpResponse};
use actix_web::web::Data;
use serde_json::json;
use tera::Context;
use crate::config::app::AppState;
use crate::forms::user::{CreateUserForm, EditUserForm};
use crate::services::user::UserService;

#[get("/users")]
async fn get_all_users(
    app_state: Data<AppState>
) ->  HttpResponse {
    
    let user_service = UserService::new(app_state.db.clone());

    match user_service.find_all() {
        Ok(users) => HttpResponse::Ok().content_type(ContentType::json()).json(users),
        Err(e) => HttpResponse::InternalServerError().content_type(ContentType::json())
            .body(json!({"error": e.to_string()}).to_string()),
    }
}

#[get("/users/{uid}")]
async fn get_one_user(
    app_state: Data<AppState>,
    uid: web::Path<i32>,
) -> HttpResponse {
    let user_service = UserService::new(app_state.db.clone());

    match user_service.find_one(uid.into_inner()) {
        Ok(Some(user)) => HttpResponse::Ok().content_type(ContentType::json()).json(user),
        Ok(None) => HttpResponse::NotFound().content_type(ContentType::json()).body(json!({"error": "User not found"}).to_string()),
        Err(e) => HttpResponse::InternalServerError().content_type(ContentType::json())
            .body(json!({"error": e.to_string()}).to_string()),
    }
}

#[post("/users")]
async fn sign_up_user(app_state: Data<AppState>,
                      web::Json(form): web::Json<CreateUserForm>) -> HttpResponse {
    let user_service = UserService::new(app_state.db.clone());

    match user_service.sign_up(form) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {e}")),
    }
}

#[put("/users/{uid}")]
async fn edit_user(app_state: Data<AppState>,
                   uid: web::Path<i32>,
                   web::Json(form): web::Json<EditUserForm>) -> HttpResponse {
    let user_service = UserService::new(app_state.db.clone());

    match user_service.edit_user(uid, form) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {e}"))
    }
}