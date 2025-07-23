use actix_web::App;
use crate::errors::AppError;

use crate::models::user::User;

pub async fn get_user() -> Result<User, String> {


        Ok(
            User{
                id: 1,
                name: "vignesh".to_string()
            }
        )
    

}

pub async fn user_repository()-> User{
     User{
        id:1,
        name:"sam altman".to_string(),
    
}
}



pub async fn update_user(user: User) -> Result<User, String> {

    let userdb = user_repository().await;

    if user.id == userdb.id {
        Ok(
            User { 
                id: user.id, 
                name: user.name 
            }
        )
    }
    else {
        Err(AppError::UserNotFound.to_string())
    }
    
}