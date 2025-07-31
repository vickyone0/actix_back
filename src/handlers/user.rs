use actix_web::{get,post, put, web,HttpResponse, Responder, Error};
use crate::{asynawait, echo};
use crate::extractors::user::UserProfile;
use crate::handlers::user;
use crate::services::user as UserService;
use crate::models::user::User;
use serde::Deserialize;
use actix_multipart::Multipart;
use std::io::Write;
use std::fs;
use futures_util::stream::StreamExt;
use futures_util::TryStreamExt;

#[get("/user/info")]
pub async fn user_info() -> impl Responder {
    match UserService::get_user().await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("User not found")
    }
    // Serialize the user struct to JSON
}

#[put("/user")]
pub async fn update_info(user: web::Json<User> ) -> impl Responder {
    match UserService::update_user(user.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}


#[get("/user/{user_id}")]
pub async fn get_user_by_id(user_id : web::Path<u32>) -> impl Responder {

    let user_by_id = UserService::get_user_id(user_id.into_inner()).await;

    match user_by_id {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("User not found")
    }
}

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
    page: Option<u32>,
}

#[get("/search")]
pub async fn search_user(query: web::Query<SearchQuery>) -> impl Responder {
    let (query,page) = (query.query.clone(), query.page.unwrap_or(1));
     

    format!("Query: {}, Page: {}", query, page)
}

#[derive(Deserialize)]
struct ProductQuery {
    min_price: Option<f32>,
    max_price: Option<f32>,
}
#[get("/products/{category}")]
pub async fn list_products(catagory: web::Path<String> ,price_range:web::Query<ProductQuery> ) -> impl Responder {
    let min_price = price_range.min_price.unwrap_or(0.0);
    let max_price = price_range.max_price.unwrap_or(f32::MAX);

   format!("Category: {}, Min Price: {}, Max Price: {}", catagory, min_price, max_price)
}


#[derive(Deserialize)]
struct FormData {
    #[serde(default = "default_name")]
    name: String,
    #[serde(default = "default_age")]
    age: u32,
}

fn default_name() -> String {
    "Default Name".to_string()
}
fn default_age() -> u32 {
    18
}

#[post("/items")]
pub async fn create_item(form: web::Form<FormData>) -> impl Responder {

    let item = format!("Item created with name: {}, age: {}", form.name, form.age);
    HttpResponse::Created().body(item)

} 


#[post("/file")]
pub async fn upload_file(mut multipart: Multipart) -> Result<HttpResponse,Error> {
    
    while let Some(mut filed) = multipart.next().await{
        let mut filed = filed?; // This is a Result, so use ? here
        let content_type = filed.content_disposition().unwrap();
        let file_name = content_type.get_filename().unwrap_or("default.txt");
        let dir_path = "./tem";
        let filepath = format!("{}/{}", dir_path, file_name);

        // Ensure the directory exists
        fs::create_dir_all(dir_path).unwrap();

        let mut f = fs::File::create(filepath).unwrap();
        while let Ok(Some(chunk)) = filed.try_next().await {
            f.write_all(&chunk)?;
    }
}

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}


#[get("/user/extrator")]
pub async fn user_extractor(user: UserProfile) -> impl Responder {
    HttpResponse::Ok().body(format!("User ID: {}, Role: {}", user.user_id, user.role))


}

#[get("/asynccall")]
pub async fn async_call() -> impl Responder {
    asynawait::multi_file().await;
    HttpResponse::Ok().body("async operation completed")


}

#[get("/echo")]
pub async fn echos() -> impl Responder {
    echo::async_network_io().await;
    HttpResponse::Ok().body("async operation completed")


}