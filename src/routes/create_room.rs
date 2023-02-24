use actix_web::web::{Data, Json};

use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::database::create_new_room;
use crate::models::{Db, NewRoom};
use crate::utils::validate_token;

pub async fn create_room(
    new_room: Json<NewRoom>,
    req: HttpRequest,
    db: Data<Db>,
) -> impl Responder {
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => match create_new_room(new_room.name.clone(), user_id, &db).await {
            Ok(_) => HttpResponse::Ok().body("ROOM CREATED"),
            Err(_) => HttpResponse::InternalServerError().body("Error creating room"),
        },
        Err(error) => error.message(),
    }
}
