#![doc = include_str!("../README.md")]

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

/// Just a basic hello world for now
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
