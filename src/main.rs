mod middlewares;
mod errors;
mod models;
mod schema;
mod services;
mod views;
mod forms;
mod config;
mod repositories;
mod routes;

use actix_files::Files;
use tera::Tera;
use actix_web::{middleware::{self, Logger}, web::{self}, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use dotenvy::dotenv;
use env_logger::Env;
use crate::config::app::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*
    sudo apt update
    sudo apt install libsqlite3-dev
    */

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Failed to create database pool");

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
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                tera: tera.clone()
            }))
            .wrap(middleware::NormalizePath::trim())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(Files::new("static/", "static")
                .prefer_utf8(true)
                .show_files_listing()
                .use_last_modified(true)
            )
            .service(routes::user::build_user_routes())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


