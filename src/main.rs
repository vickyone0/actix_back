
use actix_back::db::establish_connection;
use actix_web::{ web,App, HttpServer};
use serde::{Deserialize, Serialize};
use config::{Config, File};
use actix_back::handlers::{self, user::update_info};
use actix_back::middleware::execution_time::RequestIdMiddleware;
use actix_back::middleware::log::Logger;
use actix_back::middleware::keyvalidator::ApiKeyValidator;
use actix_back::middleware::jwt::JWTAuthentication;
use dotenv::dotenv;

#[derive(Deserialize)]
struct Settings {
    server: ServerSettings,
}

#[derive(Deserialize)]
struct ServerSettings {
    host: String,
    port: u16,
    worker_threads: u16,
}

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello, world!")
// }
// #[derive(Deserialize)]
// struct Name {
//     first_name: String,
//     last_name: String,
// }
// #[get("/hello/{first_name}/{last_name}")]
// async fn greet(path: web::Path<Name>) -> impl Responder {
//     let name = path.into_inner();
//     HttpResponse::Ok().body(format!("Hello, {} {}!", name.first_name, name.last_name))
// }






#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let pool = establish_connection();

    let config = Config::builder()
        .add_source(File::with_name("config"))
        .build()
        .unwrap();

    let settings: Settings = config.try_deserialize().unwrap();

    let server_address = format!("{}:{}", settings.server.host,settings.server.port);

    HttpServer::new(move || {
         App::new()
        //     .service(hello)
        //     .service(greet)
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger)
            .wrap(JWTAuthentication)
            .wrap(RequestIdMiddleware)
            .service(handlers::user::user_info)
            .service(handlers::user::update_info)
            .service(handlers::product::list_products)
            .service(handlers::user::user_extractor)
            .service(handlers::user::get_user_by_id)
            .service(handlers::user::search_user)
            .service(handlers::user::list_products)
            .service(handlers::user::create_item)
            .service(handlers::user::upload_file)
            
    })
    .bind(&server_address)?
    .workers(settings.server.worker_threads as usize)
    .run()
    .await
}