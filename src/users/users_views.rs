use actix_web::{get, http::header::ContentType, post, put, web::{self}, Error, HttpResponse};
use actix_web::web::Data;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use serde_json::json;
use crate::users::users_service::UserService;
use crate::users::user_forms::{CreateUserForm, EditUserForm};
use tera::{Tera, Context};
use crate::models::models::User;
use tokio::time::{sleep, Duration};


#[put("/users/{uid}")]
async fn edit_user(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
                   uid: web::Path<i32>,
                   web::Json(form): web::Json<EditUserForm>) -> HttpResponse {
    let user_service = UserService::new(pool.get_ref().clone());

    match user_service.edit_user(uid, form) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {e}"))
    }
}

#[post("/users")]
async fn sign_up_user(pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
                      web::Json(form): web::Json<CreateUserForm>) -> HttpResponse {
    let user_service = UserService::new(pool.get_ref().clone());

    match user_service.sign_up(form) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {e}")),
    }
}

#[get("/users-table")]
async fn users_table(
    tera: Data<Tera>,
    pool: web::Data<Pool<ConnectionManager<SqliteConnection>>>
) -> Result<HttpResponse, Error> {

    sleep(Duration::from_secs(2)).await;

    let user_service = UserService::new(pool.get_ref().clone());
    let mut context = Context::new();

    match user_service.find_all() {
        Ok(users) => {
            context.insert("users", &users);
            context.insert("error", "");
        },
        Err(e) => {
            context.insert("users", &Vec::<User>::new());
            context.insert("error", &e.to_string())
        }
    }

    let rendered = tera.render("index/partials/users_table.html", &context)
        .map_err(|err| {
            log::error!("Template rendering error: {}", err);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}

#[get("/")]
async fn index(
    tera: Data<Tera>,
) -> Result<HttpResponse, Error> {
    let rendered = tera.render("index/index.html", &Context::new())
        .map_err(|err| {
            log::error!("Template rendering error: {}", err);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
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
