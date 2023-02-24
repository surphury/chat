use actix_web::web::{Data, Path};

use actix_web::{HttpResponse, Responder};

use crate::database::get_user_by_username;

use crate::models::Db;

pub async fn get_user(username: Path<String>, db: Data<Db>) -> impl Responder {
    match get_user_by_username(&*username, &db).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => {
            println!("{:#?}", error);
            HttpResponse::InternalServerError().body("Error adding user")
        }
    }
}
