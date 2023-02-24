/* use crate::database::{
    create_new_room, fetch_available_rooms, fetch_messages_by_room, get_user_by_username,
    insert_new_user, verify_password,
};

use crate::jwt::generate_token;

use crate::utils::validate_token;

use crate::handler;

use crate::models::{Db, Login, NewRoom, User}; */
/* use crate::web_socket::Session; */
/* use crate::web_socket::MyWs; */

/* use actix_web::error::HttpError;
use actix_web::web::{Data, Json, Path};
use actix_web::{rt, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws; */

/* pub async fn get_user(username: Path<String>, db: Data<Db>) -> impl Responder {
    match get_user_by_username(&*username, &db).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => {
            println!("{:#?}", error);
            HttpResponse::InternalServerError().body("Error adding user")
        }
    }
} */

/* pub async fn register_user(new_user: Json<User>, db: Data<Db>) -> impl Responder {
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
} */

/* pub async fn login(user: Json<Login>, db: Data<Db>) -> impl Responder {
    let user = Login {
        password: user.password.clone(),
        username: user.username.clone(),
    };

    let users = verify_password(&user, &db).await;

    match users {
        Ok(users) => {
            if users.len() == 1 {
                let token = generate_token(&users[0]);

                match token {
                    Ok(token) => HttpResponse::Ok().json(token),
                    Err(_) => HttpResponse::InternalServerError().body("Error generating token"),
                }
            } else {
                HttpResponse::Unauthorized().body("Verification failed")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error verifying user"),
    }
} */

/* pub async fn get_rooms(req: HttpRequest, db: Data<Db>) -> HttpResponse {
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
} */

/* pub async fn create_room(
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
} */

/* pub async fn room(
    room_id: Path<i16>,
    req: HttpRequest,
    stream: web::Payload,
    db: Data<Db>,
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
} */
/*  Err(error) => error.message(),
    }
} */

/* pub async fn delete_tasks(req: HttpRequest, task: Json<TaskId>, db: Data<Db>) -> impl Responder {
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => match delete_task(task.id, user_id, &db).await {
            Ok(_) => HttpResponse::Ok().body("Task deleted"),
            Err(_) => HttpResponse::InternalServerError().body("Error deleting task"),
        },
        Err(error) => error.message(),
    }
}

pub async fn post_task(req: HttpRequest, task: Json<NewTask>, db: Data<Db>) -> impl Responder {
    let authorization = req.headers().get("Authorization");

    let task = NewTask {
        name: task.name.clone(),
        description: task.description.clone(),
    };

    match validate_token(authorization) {
        Ok(user_id) => match add_task(user_id, task, &db).await {
            Ok(_) => match get_tasks_by_user(user_id, &db).await {
                Ok(tasks) => HttpResponse::Ok().json(tasks),
                Err(_) => HttpResponse::InternalServerError().body("Error getting tasks"),
            },
            Err(_) => HttpResponse::InternalServerError().body("Error posting tasks"),
        },
        Err(error) => error.message(),
    }
}

pub async fn start_task(task_id: Path<i32>, req: HttpRequest, db: Data<Db>) -> impl Responder {
    let task_id = task_id.into_inner();
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => match start_task_and_save_time(task_id, user_id, &db).await {
            Ok(task_history) => HttpResponse::Accepted().json(task_history),
            Err(error) => error.message(),
        },
        Err(error) => error.message(),
    }
}

pub async fn finish_task(task_id: Path<i32>, req: HttpRequest, db: Data<Db>) -> impl Responder {
    let task_id = task_id.into_inner();
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => match finish_task_and_save_time(task_id, user_id, &db).await {
            Ok(task_history) => HttpResponse::Accepted().json(task_history),

            Err(err) => err.message(),
        },
        Err(error) => error.message(),
    }
} */
