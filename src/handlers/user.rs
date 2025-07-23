use actix_web::{get, put, web,HttpResponse, Responder};
use crate::services::user as UserService;
use crate::models::user::User;

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


