use actix_web::{get, web, HttpServer, App};
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}

#[actix_web::main]
async fn main()-> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().app_data(web::Data::new(AppState{
            app_name: String::from("Learning some actix")
        })).service(index)
    }).bind(("127.0.0.1", 8080))?.run().await
}