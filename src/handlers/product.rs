use actix_web::{get, web, Responder, HttpResponse};
use serde::Deserialize;
use crate::services::product as ProductService;


#[derive(Deserialize)]
struct Parameters {
    min_price: i32,
    max_price: i32,
}

#[get("/products")]
async fn list_products(params: web::Query<Parameters>) -> impl Responder {
    let product_service = ProductService::get_products_in_price_range(params.min_price, params.max_price).await;
    match product_service{
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching products :{}", e)),

}

}