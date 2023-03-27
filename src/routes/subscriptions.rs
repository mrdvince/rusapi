use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
pub async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    println!("{}, {}", _form.name, _form.email);
    HttpResponse::Ok()
}