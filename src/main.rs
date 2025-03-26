mod views;
mod api;
mod middlewares;

use crate::views::index::index;

use actix_files::Files;
use tera::Tera;
use actix_web::{middleware::{self, from_fn, Logger}, web::{self}, App, HttpServer};
use env_logger::Env;
use crate::api::connection::create_db_pool;
use crate::api::users::users_views::{get_all_users, get_one_user};
use crate::middlewares::middlewares::auth_middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_db_pool();

    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Error initing Tera: {}", e);
            ::std::process::exit(1);
        }
    };
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::NormalizePath::trim())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))            .app_data(web::Data::new(tera.clone()))
            .service(Files::new("/static", "static")
                .prefer_utf8(true)
                .show_files_listing()
                .use_last_modified(true)
            )
            .service(
                web::scope("/api")
                    .wrap(from_fn(auth_middleware))
                    .service(get_all_users)
                    .service(get_one_user)
            )
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


