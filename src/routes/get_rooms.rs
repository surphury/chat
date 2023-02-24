use actix_web::{web::Data, HttpRequest, HttpResponse};

use crate::database::fetch_available_rooms;
use crate::models::Db;
use crate::utils::validate_token;

pub async fn get_rooms(req: HttpRequest, db: Data<Db>) -> HttpResponse {
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => {
            let rooms = fetch_available_rooms(user_id.try_into().unwrap(), &db).await;
            match rooms {
                Ok(rooms) => HttpResponse::Ok().json(rooms),
                Err(_) => HttpResponse::InternalServerError().body("Error getting tasks"),
            }
        }
        Err(error) => error.message(),
    }
}
