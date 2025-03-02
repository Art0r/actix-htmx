mod views;
mod api;

use views::index::index;
use api::views::index::{ping, pong};

use actix_files::Files;
use tera::{Tera};
use actix_web::{middleware, web::{self}, App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Error initing Tera: {}", e);
            ::std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .app_data(web::Data::new(tera.clone()))
            .service(Files::new("/static", "static")
                .prefer_utf8(true)
                .show_files_listing()
                .use_last_modified(true)
            )
            .service(
                web::scope("/api")
                    .service(ping)
                    .service(pong)
            )
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


