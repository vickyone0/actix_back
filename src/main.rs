use actix_web::{App, HttpServer, web, Responder, HttpResponse};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use env_logger;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    //Load TLS keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
    .bind_openssl("127.0.0.1:8443", builder)?
    .bind("127.0.0.1:8080")?
    .workers(4)
    .run()
    .await
}