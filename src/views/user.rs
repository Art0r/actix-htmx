use actix_web::{get, http::header::ContentType, Error, HttpResponse};
use actix_web::web::Data;
use tera::{Context};
use tokio::time::{sleep, Duration};
use crate::config::app::AppState;
use crate::models::user::User;
use crate::services::user::UserService;

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

#[get("/users-table")]
async fn render_users_table(
    app_state: Data<AppState>
) ->  Result<HttpResponse, Error> {

    let mut context = Context::new();

    let user_service = UserService::new(app_state.db.clone());

    let users= match user_service.find_all() {
        users => users,
    };

    context.insert("users", users.unwrap().as_slice());

    let rendered = app_state.tera.render("index/partials/users_table.html", &context)
        .map_err(|err| {
            log::error!("Template rendering error: {}", err);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}
