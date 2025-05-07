use actix_web::{get, post, web, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use serde_json::json;
use crate::config::app::AppState;
use crate::forms::pet::CreatePetsForm;
use crate::forms::user::CreateUserForm;
use crate::services::pets::PetsService;
use crate::services::user::UserService;

#[get("/pets")]
async fn get_all_pets(
    app_state: Data<AppState>
) ->  HttpResponse {

    let pets_service = PetsService::new(app_state.db.clone());

    match pets_service.get_all_pets() {
        Ok(pets) => HttpResponse::Ok().content_type(ContentType::json()).json(pets),
        Err(e) => HttpResponse::InternalServerError().content_type(ContentType::json())
            .body(json!({"error": e.to_string()}).to_string()),
    }
}

#[post("/pets")]
async fn create_pet(app_state: Data<AppState>,
                      web::Json(form): web::Json<CreatePetsForm>) -> HttpResponse {
    let pets_service = PetsService::new(app_state.db.clone());

    match pets_service.create_pet(form) {
        Ok(pet) => HttpResponse::Ok().json(pet),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {e}")),
    }
}
