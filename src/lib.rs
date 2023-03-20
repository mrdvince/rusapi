use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};

async fn greet() -> impl Responder {
    ("Hello World!").to_string()
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    println!("{}, {}", _form.name, _form.email);
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
