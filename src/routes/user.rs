use actix_web::middleware::from_fn;
use actix_web::{web, Scope};
use crate::middlewares::middlewares::auth_middleware;
use crate::views::api::user::{edit_user, get_all_users, get_one_user, sign_up_user};
use crate::views::user::{index, users_table};

pub fn build_user_routes() -> Scope {
    web::scope("")
        .service(users_table)
        .service(index)
        .service(
            web::scope("api")
            .wrap(from_fn(auth_middleware))
            .service(get_all_users)
            .service(get_one_user)
            .service(sign_up_user)
            .service(edit_user)
        )
}