use actix_web::middleware::from_fn;
use actix_web::{web, Scope};
use crate::middlewares::middlewares::auth_middleware;
use crate::views::api::pets::{create_pet, delete_pet, get_all_pets, get_one_pet};
use crate::views::api::user::{delete_user, edit_user, get_all_users, get_one_user, sign_up_user};
use crate::views::user::{render_users_table, index};

pub fn build_user_routes() -> Scope {
    web::scope("")
        .service(index)
        .service(render_users_table)
        .service(
            web::scope("api")
            .wrap(from_fn(auth_middleware))
            .service(get_all_users)
            .service(get_one_user)
            .service(sign_up_user)
            .service(edit_user)
            .service(delete_user)
            .service(get_all_pets)
            .service(get_one_pet)
            .service(delete_pet)
            .service(create_pet)
        )
}