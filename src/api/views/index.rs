use std::{u8};

use actix_web::{get, http::header::ContentType, post, web::{self},HttpResponse};
use serde::{Deserialize, Serialize};
use std::{thread, time};
use rand::Rng;


#[derive(Serialize, Deserialize)]
struct FormBody {
    name: String,
}


#[get("/ping")]
async fn ping() -> HttpResponse {
    let n1: u8 = rand::thread_rng().gen();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(n1.to_string())
}

#[post("/pong")]
async fn pong(form: web::Form<FormBody>) -> HttpResponse {
    let ten_millis = time::Duration::from_millis(10000);

    thread::sleep(ten_millis);
    
    println!("Received name: {}", form.name);

    HttpResponse::Ok()
        .content_type(ContentType::form_url_encoded())
        .body(format!("Hello, {}!", form.name))
}
