use actix_web::{web, Scope};
use actix_web::middleware::from_fn;
use crate::middlewares::middlewares::auth_middleware;
use crate::views::api::pets::{create_pet, get_all_pets, get_one_pet};
use crate::views::api::user::{edit_user, get_all_users, get_one_user, sign_up_user};
use crate::views::user::{index, render_users_table};

pub fn build_pets_routes() -> Scope {
    web::scope("")
        .service(
            web::scope("api")
                .service(get_all_pets)
                .service(get_one_pet)
                .service(create_pet)
        )
}