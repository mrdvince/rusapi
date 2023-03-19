use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};

async fn greet() -> impl Responder {
    ("Hello World!").to_string()
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();
    Ok(server)
}
