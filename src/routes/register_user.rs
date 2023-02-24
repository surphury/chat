use crate::models::{Db, User};
use actix_web::web::{Data, Json};

use actix_web::{HttpResponse, Responder};

use crate::database::insert_new_user;

pub async fn register_user(new_user: Json<User>, db: Data<Db>) -> impl Responder {
    let new_user = User {
        password: new_user.password.clone(),
        username: new_user.username.clone(),
        email: new_user.email.clone(),
    };

    match insert_new_user(new_user, &db).await {
        Ok(_) => HttpResponse::Ok().body("User added"),
        Err(error) => {
            println!("{:#?}", error);
            HttpResponse::InternalServerError().body("Error adding user")
        }
    }
}
