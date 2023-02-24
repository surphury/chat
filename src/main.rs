use actix_ws::Session;
use tokio::{
    task::{spawn, spawn_local},
    try_join,
};
/* use actix::{Actor, Addr, StreamHandler}; */
use actix_web::web::Data;
use actix_web::{
    middleware, web, App,
    /*  Error, HttpRequest, HttpResponse, */ HttpServer, /* , Responder */
};
/* use actix_web_actors::ws::{self, CloseReason}; */

use actix_cors::Cors;

mod database;
mod handler;
mod hashing;
mod jwt;
mod models;
mod routes;
mod server;
mod utils;

#[cfg(test)]
mod test;

use routes::{create_room, get_rooms, get_user, login, register_user, room};

pub use server::{ChatServer, ChatServerHandle};

/* use utils::validate_token;
use uuid::Uuid; */

use std::{collections::HashMap, env::var, rc::Rc, sync::Arc};

use dotenv::dotenv;

use database::connect;

use models::Db;

/// Connection ID.
pub type ConnId = usize;

/// Room ID.
pub type RoomId = i16;

/// Message sent to a room/client.
pub type Msg = String;

/* // Import the WebSocket logic we wrote earlier.
mod server;
use self::server::MyWebSocket; */

/* async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
} */

// WebSocket handshake and start `MyWebSocket` actor.
/* async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    db: Data<Db>,
    /*  group_id: Path<Uuid>, */
    server: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => {
            let web_socket = WsConnection::new(Uuid::new_v4(), server.get_ref().clone());

            let resp = ws::start(web_socket, &req, stream)?;
            Ok(resp)
        }
        Err(error) => Ok(error.message()),
    }
    /*  Ok(get_rooms(req, db).await) */
} */

/// Define HTTP actor

/// Handler for ws::Message message

/* async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
} */

/* #[actix_web::main] */
#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    /* env_logger::init_from_env(env_logger::Env::new().default_filter_or("info")); */

    /*   log::info!("starting HTTP server at http://localhost:8080"); */

    dotenv().ok();

    let database_url: String = var("DATABASE_URL").unwrap();

    let pool = connect(&database_url)
        .await
        .expect("Could not connect to database");

    /*  let chat_server = Lobby::default().start(); */

    let (chat_server, server_tx) = ChatServer::new().await;

    let chat_server = spawn(chat_server.run());

    println!("CONNECTED TO DATABASE");

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_tx.clone()))
            .app_data(Data::new(Db { pool: pool.clone() }))
            /*  .app_data(Data::new(chat_server.clone())) */
            .route("login", web::post().to(login))
            .route("register_user", web::post().to(register_user))
            /* .route("room/{room_id}", web::post().to(room)) */
            .route("get_rooms", web::post().to(get_rooms))
            .route("room/new", web::post().to(create_room))
            .route("user/{username}", web::get().to(get_user))
            /* test */
            .service(web::resource("/room/{room_id}").route(web::get().to(room)))
            /* test */
            .wrap(middleware::Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run();

    try_join!(http_server, async move { chat_server.await.unwrap() })?;

    Ok(())
}

/* /// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url: String = var("DATABASE_URL").unwrap();

    let pool: MySqlPool = connect(&database_url)
        .await
        .expect("Could not connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Db { pool: pool.clone() }))
            .route("/ws/", web::get().to(index))
            .route("/register_user", web::post().to(register_user))
            .route("/login", web::post().to(login))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
 */
