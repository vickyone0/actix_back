use actix_web::{FromRequest, HttpRequest, Error};
use actix_web::dev::Payload;
use futures::future::{ok, Ready, ready};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserProfile {
    pub user_id: u32,
    pub role: String,
}

impl FromRequest for UserProfile {
    type Error = Error;
    type Future = Ready<Result<UserProfile, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // Extract user_id from the "user-id" header
        let user_id = match req.headers().get("user-id") {
            Some(header) => match header.to_str() {
                Ok(id_str) => match id_str.parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => return ready(Err(actix_web::error::ErrorBadRequest("Invalid user-id format"))),
                },
                Err(_) => return ready(Err(actix_web::error::ErrorBadRequest("Invalid user-id header"))),
            },
            None => return ready(Err(actix_web::error::ErrorBadRequest("User-id header missing"))),
        };

        // Extract role from the "user-role" header
        let role = match req.headers().get("user-role") {
            Some(header) => match header.to_str() {
                Ok(role_str) => role_str.to_string(),
                Err(_) => return ready(Err(actix_web::error::ErrorBadRequest("Invalid user-role header"))),
            },
            None => return ready(Err(actix_web::error::ErrorBadRequest("User-role header missing"))),
        };

        ready(Ok(UserProfile { user_id, role }))
    }
}