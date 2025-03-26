use std::{u8};

use actix_web::{get, http::header::ContentType, web::{self},HttpResponse};
use serde::{Deserialize, Serialize};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use serde_json::json;
use crate::api::users::users_service::UserService;

#[derive(Serialize, Deserialize)]
struct FormBody {
    name: String,
}

#[get("/users")]
async fn get_all_users(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>) -> HttpResponse {

    let user_service = UserService::new(pool.get_ref().clone());

    match user_service.find_all() {
        Ok(users) => HttpResponse::Ok().content_type(ContentType::json()).json(users),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()}))
    }
}

#[get("/users/{uid}")]
async fn get_one_user(
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
    uid: web::Path<i32>,
) -> HttpResponse {
    let user_service = UserService::new(pool.get_ref().clone());

    match user_service.find_one(uid.into_inner()) {
        Ok(Some(user)) => HttpResponse::Ok().content_type(ContentType::json()).json(user),
        Ok(None) => HttpResponse::NotFound().content_type(ContentType::json()).body(json!({"error": "User not found"}).to_string()),
        Err(e) => HttpResponse::InternalServerError().content_type(ContentType::json())
            .body(json!({"error": e.to_string()}).to_string()),
    }
}