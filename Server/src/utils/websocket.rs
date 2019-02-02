use actix::*;
use actix_web::{ server ,ws, Error, HttpRequest, HttpResponse };
use std::time::{ Instant, Duration };
use rand::{ self, rngs::ThreadRng };
use std::collections::{ HashMap, HashSet };

use crate::{ State, Wsstate };
use crate::chat::model::*;
use crate::chat::model::{ Message as ChatMessage };

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct ChatServer {
    pub sessions: HashMap<i32, Recipient<ChatMessage>>,
    pub rooms: HashMap<String, HashSet<i32>>,
    pub rng: ThreadRng,
}

pub fn chat_route(req: &HttpRequest<State>) -> Result<HttpResponse, Error> {
    ws::start(
        &req,
        Ws {
            id: 0,
            hb: Instant::now(),
            room: "Main".to_owned(),
            name: None,
        },
    )
}

pub struct Ws {
    pub id: i32,
    pub hb: Instant,
    pub room: String,
    pub name: Option<String>
}

impl Actor for Ws {
    type Context = ws::WebsocketContext<Self, State>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        let Wsstate(ws) = ctx.state().ws;
            ws
            .send(Connect { addr: addr.recipient() })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ok()
            }).wait(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        let Wsstate(ws) = ctx.state().ws;
        ws.do_send(Disconnect{ id: self.id });
        Running::Stop
    }

}

impl Handler<ChatMessage> for Ws {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Ws {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        println!("WEBSOCKET MESSAGE: {:?}", msg);
        let Wsstate(ctx) = ctx.state().ws;
        // let State(ctx) = ctx;
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    match v[0] {
                        "/list" => {
                            println!("List Rooms");
                            ctx.state()
                                .addr
                                .send(ListRooms)
                                .into_actor(self)
                                .then(|res, _, ctx| {
                                    match res {
                                        Ok(rooms) => {
                                            for room in rooms {
                                                ctx.text(room);
                                            }
                                        }
                                        _ => println!("Something is wrong"),
                                    }
                                    fut::ok(())
                                }).wait(ctx)
                        }
                        "/name" => {
                            if v.len() == 2 {
                                self.name = Some(v[1].to_owned());
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }
                        _ => ctx.text(format!("!!! unknown command: {:?}", m))
                    }
                } else {
                    let msg = if let Some(ref name) = self.name {
                            format!("{}: {}", name, m)
                        } else {
                            m.to_owned()
                        };
                        ctx.state().addr.do_send(ClientMessage {
                            id: self.id,
                            content: msg,
                            room_id: self.room.clone(),
                        })
                    }
                }
                ws::Message::Binary(bin) => println!("Unexpected binary"),
                ws::Message::Close(_) => {
                    ctx.stop();
            },
        }
    }
}

impl Ws {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self, State>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting !");
                let Wsstate(ws) = ctx.state().ws;
                    ws.do_send(Disconnect { id: act.id});
                ctx.stop();
                return;
            }
            ctx.ping("");
        });
    }
}