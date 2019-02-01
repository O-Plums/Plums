use actix::prelude::*;
use rand::{ self, rngs::ThreadRng, Rng};
use std::collections::{ HashMap, HashSet };
use crate::utils::websocket::ChatServer;
// use actix;
// use crate::messages::model::{ Message };

#[derive(Message)]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(i32)]
pub struct Connect {
    pub addr: Recipient<Message>
}

#[derive(Message)]
pub struct Disconnect {
    pub id: i32,
}

#[derive(Message)]
pub struct ClientMessage {
    pub id: i32,
    pub content: String,
    pub room_id: i32,
}

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Message)]
pub struct Join {
    pub id: i32,
    pub name: String,
}

impl Default for ChatServer {
    fn default() -> ChatServer {
        let mut rooms = HashMap::new();
        rooms.insert("Main".to_owned(), HashSet::new());
        ChatServer {
            sessions: HashMap::new(),
            rooms: rooms,
            rng: rand::thread_rng(),
        }
    }
}

impl ChatServer {
    fn send_message(&self, room: &str, message: &str, skip_id: i32) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = i32;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("SomeOne joined");
        self.send_message(&"Main".to_owned(), "Someone joined", 0);

        let id = self.rng.gen::<i32>();
        self.sessions.insert(id, msg.addr);

        self.rooms.get_mut(&"Main".to_owned()).unwrap().insert(id);

        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        let mut rooms: Vec<String> = Vec::new();

        if self.sessions.remove(&msg.id).is_some() {
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }
        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&msg.room_id, msg.content.as_str(), msg.id)
    }
}

impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.key() {
            rooms.push(key.to_owned())
        }

        MessageResult(rooms)
    }
}

impl Handler<Join> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;
        let mut rooms = Vec::new();

        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }

        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }

        if self.rooms.get_mut(&name).is_none() {
            self.rooms.insert(name.clone(), HashSet::new());
        }
        self.send_message(&name, "Someone connected", id);
        self.rooms.get_mut(&name).unwrap().insert(id);
    }
}