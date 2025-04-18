use actix_web::{get, http::header::ContentType, Error, HttpResponse};
use actix_web::web::Data;
use tera::{Context};
use tokio::time::{sleep, Duration};
use crate::config::app::AppState;
use crate::models::user::User;
use crate::services::user::UserService;

#[get("/users-table")]
async fn users_table(
    app_data: Data<AppState>,
) -> Result<HttpResponse, Error> {

    sleep(Duration::from_secs(2)).await;

    let user_service = UserService::new(app_data.db.clone());
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

    let rendered = app_data.tera.render("index/partials/users_table.html", &context)
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
    app_data: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let rendered = app_data.tera.render("index/index.html", &Context::new())
        .map_err(|err| {
            log::error!("Template rendering error: {}", err);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}
