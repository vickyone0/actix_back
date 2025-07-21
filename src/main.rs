use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use config::{Config, File};

#[derive(Deserialize)]
struct Settings {
    server: ServerSettings,
}

#[derive(Deserialize)]
struct ServerSettings {
    host: String,
    port: u16,
}

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Config::builder()
        .add_source(File::with_name("config"))
        .build()
        .unwrap();

    let settings: Settings = config.try_deserialize().unwrap();

    let server_address = format!("{}:{}", settings.server.host,settings.server.port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
    .bind(&server_address)?
    .run()
    .await
}