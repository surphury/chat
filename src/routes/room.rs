use actix_web::web::{Data, Path, Payload};

use actix_web::{Error, HttpRequest, HttpResponse};

use tokio::task::spawn_local;

use crate::models::Db;
use crate::{handler, ChatServerHandle};

pub async fn room(
    room_id: Path<i16>,
    req: HttpRequest,
    stream: Payload,
    db: Data<Db>,
    chat_server: Data<ChatServerHandle>,
) -> Result<HttpResponse, Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;

    // spawn websocket handler (and don't await it) so that the response is returned immediately
    spawn_local(handler::chat_ws(
        (**chat_server).clone(),
        session,
        msg_stream,
    ));

    Ok(res)

    /*  match validate_token(authorization) {
    Ok(user_id) => { */
    /*  let user_id = 1; */
    /* should verify user */
    /*   let resp = ws::start(
        Session {
            user_id,
            room_id: *room_id,
            db: db.clone(),
        },
        &req,
        stream,
    ); */

    /*  resp */
    /* println!("{:?}", resp); */

    /*   let messages = fetch_messages_by_room(*room_id, user_id.try_into().unwrap(), &db).await;
    match messages {
        Ok(messages) => {
            println!("{:#?}", messages);
            HttpResponse::Ok().json(messages)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error getting tasks"),
    } */
}
