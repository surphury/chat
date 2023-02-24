use actix_web::test::{
    call_and_read_body, call_and_read_body_json, call_service, init_service, read_body,
    try_read_body_json, TestRequest,
};

use crate::models::{DBUser, User};

use super::*;

#[actix_web::test]
async fn can_add_new_users() {
    dotenv().ok();

    let database_url: String =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set as a environment variable");

    let pool = connect(&database_url).await.unwrap();

    let app = init_service(
        App::new()
            .app_data(Data::new(Db { pool }))
            /* .route("login", web::post().to(login)) */
            .route("register_user", web::post().to(register_user))
           /*  .route("room/{room_id}", web::post().to(room))
            .route("get_rooms", web::post().to(get_rooms))
            .route("room/new", web::post().to(create_room)) */
            .route("user/{username}", web::get().to(get_user))
            /* test
            .service(web::resource("/ws").route(web::get().to(websocket)))
            /* test */
            .wrap(middleware::Logger::default()), */
    )
    .await;

    let user = User {
        username: "janedoe".into(),
        email: "example@example.com".into(),
        password: "password".into(),
    };

    let register_user_response = TestRequest::post()
        .uri("/register_user")
        .set_json(&user)
        .to_request();

    let _body = call_and_read_body(&app, register_user_response).await;

    let req = TestRequest::get()
        .uri(&format!("/user/{}", &user.username))
        .to_request();

    let fetched_user: DBUser = call_and_read_body_json(&app, req).await;

    assert_eq!(fetched_user.username, user.username)
}

/* #[actix_web::test]
async fn can_delete_users() {
    let client = connect().await;
    client
        .database(DB_NAME)
        .collection::<NewUser>(COLL_NAME)
        .drop(None)
        .await
        .expect("drop collection should succeed");

    let app = init_service(
        App::new()
            .app_data(Data::new(client))
            .service(register_user)
            .service(get_user)
            .service(delete_user),
    )
    .await;

    let user = NewUser {
        username: "janedoe".into(),
        email: "example@example.com".into(),
    };

    let req = TestRequest::post()
        .uri("/user")
        .set_form(&user)
        .to_request();

    let response = call_and_read_body(&app, req).await;
    assert_eq!(response, Bytes::from_static(b"user added"));

    let req = TestRequest::delete()
        .uri("/user")
        .set_form(&user)
        .to_request();

    let response = call_and_read_body(&app, req).await;
    assert_eq!(response, Bytes::from_static(b"user deleted"));
}
 */
