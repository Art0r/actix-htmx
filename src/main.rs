mod database;
mod middlewares;
pub mod users;
pub mod errors;
mod models;
mod schema;

use actix_files::Files;
use users::users_views::sign_up_user;
use tera::Tera;
use actix_web::{middleware::{self, from_fn, Logger}, web::{self}, App, HttpServer};
use env_logger::Env;
use users::users_views::{edit_user, index, users_table, get_one_user};
use crate::database::connection::create_db_pool;
use crate::middlewares::middlewares::auth_middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_db_pool();

    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            log::error!("Error initing Tera: {}", e);
            ::std::process::exit(1);
        }
    };
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
            .wrap(middleware::NormalizePath::trim())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(Files::new("static/", "static")
                .prefer_utf8(true)
                .show_files_listing()
                .use_last_modified(true)
            )
            .service(
                web::scope("")
                    .wrap(from_fn(auth_middleware))
                    .service(get_one_user)
                    .service(sign_up_user)
                    .service(edit_user)
                    .service(users_table)
                    .service(index)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


