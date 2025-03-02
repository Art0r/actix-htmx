use actix_web::{get, http::header::ContentType, web::{Data}, HttpResponse};
use tera::{Tera, Context};


#[get("/")]
pub async fn index(tera: Data<Tera>) -> Result<HttpResponse, actix_web::Error> {
    let mut context = Context::new();
    context.insert("name", "Hello, World"); 

    let rendered = tera.render("index.html", &context)
        .map_err(|err| {
            eprintln!("Template rendering error: {}", err);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;
    
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(rendered))
}
