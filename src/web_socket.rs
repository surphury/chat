use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use std::thread;
use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web::web::Bytes;
use actix_web::web::Data;
use actix_web_actors::ws;

use crate::database::send_message;
use crate::lobby::Lobby;
use crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use crate::models::Db;

use actix::ActorFutureExt;
use actix::{fut, ActorContext, ActorFuture, ContextFutureSpawner, WrapFuture};
use actix::{Actor, Addr, Running, StreamHandler};
use actix::{AsyncContext, Handler};

use actix_web_actors::ws::Message::Text;

use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/*pub struct WsConnection {
    room: Uuid,
    lobby_address: Addr<Lobby>,
    heartbeat: Instant,
    id: Uuid,
}

impl WsConnection {
    pub fn new(room: Uuid, lobby: Addr<Lobby>) -> WsConnection {
        WsConnection {
            id: Uuid::new_v4(),
            room,
            heartbeat: Instant::now(),
            lobby_address: lobby,
        }
    }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        self.heartbeat(context);

        let address = context.address();

        self.lobby_address
            .send(Connect {
                addr: address.recipient(),
                lobby_id: self.room,
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, context| {
                match res {
                    Ok(_res) => {}
                    _ => context.stop(),
                };
                fut::ready(())
            })
            .wait(context)
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.lobby_address.do_send(Disconnect {
            id: self.id,
            room_id: self.room,
        });

        Running::Stop
    }
}

impl WsConnection {
    fn heartbeat(&self, context: &mut ws::WebsocketContext<Self>) {
        context.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
            if Instant::now().duration_since(actor.heartbeat) > CLIENT_TIMEOUT {
                println!("{:#?}", "OOF");

                actor.lobby_address.do_send(Disconnect {
                    id: actor.id,
                    room_id: actor.room,
                });

                context.stop();

                return;
            }

            context.ping(b"PING");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, context: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                context.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => context.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                context.close(reason);
                context.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                context.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => self.lobby_address.do_send(ClientActorMessage {
                id: self.id,
                msg: s.to_string(),
                room_id: self.room,
            }),

            Err(e) => panic!("{:#?}", e),
        }
    }
}

impl Handler<WsMessage> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/* pub struct MyWebSocket {
    hb: Instant,
}

impl MyWebSocket {
    pub fn new() -> Self {
        Self { hb: Instant::now() }
    }

    // This function will run on an interval, every 5 seconds to check
    // that the connection is still alive. If it's been more than
    // 10 seconds since the last ping, we'll close the connection.
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    // Start the heartbeat process for this connection
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

// The `StreamHandler` trait is used to handle the messages that are sent over the socket.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    // The `handle()` function is where we'll determine the response
    // to the client's messages. So, for example, if we ping the client,
    // it should respond with a pong. These two messages are necessary
    // for the `hb()` function to maintain the connection status.
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // Ping/Pong will be used to make sure the connection is still alive
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            // Text will echo any text received back to the client (for now)
            Ok(ws::Message::Text(text)) => {
                ctx.text("xd");
                ctx.set_mailbox_capacity(20);
                /* 		ctx.add_message_stream(fut) */
            }
            // Close will close the socket
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
} */
 */

/* 768325+6 */

pub struct Session {
    pub user_id: i16,
    pub room_id: i16,
    pub db: Data<Db>,
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                ctx.text(text);
                /* println!("{:#?}", ctx.add_stream(fut)); */
                /* actix_web::rt::System::new().block_on(async { */
                /*   let Err(error) = send_message(text.to_string(), self.room_id, self.user_id, &self.db).await else { */
                /*      return ()
                }; */

                /*  println!("{:#?}", error); */
                /* /// It's a syntax error.
                }) */
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
