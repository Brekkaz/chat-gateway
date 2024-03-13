use crate::kafka::KafkaProducer;
use crate::websocket::messages::{
    Connect, Disconnect, MessageFromBroker, MessageFromClient, MessageFromServer,
};
use actix::prelude::*;
use application::chat::ChatUseCase;
use domain::usecases::Chat;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

pub struct ChatServer {
    sessions: HashMap<Uuid, HashMap<Uuid, Recipient<MessageFromServer>>>,
}

impl ChatServer {
    pub fn new() -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
        }
    }
}

impl ChatServer {
    fn send_message(
        &self,
        message: &str,
        sender_id: &Uuid,
        sender_socket_id: &Uuid,
        receiver_id: &Uuid,
    ) {
        if let Some(sender_sessions) = self.sessions.get(sender_id) {
            for (socket_id, addr) in sender_sessions {
                if socket_id != sender_socket_id {
                    addr.do_send(MessageFromServer(message.to_owned()));
                }
            }
        }
        if sender_id != receiver_id {
            if let Some(receiver_sessions) = self.sessions.get(receiver_id) {
                for (_, addr) in receiver_sessions {
                    addr.do_send(MessageFromServer(message.to_owned()));
                }
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = String;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");
        let id = Uuid::new_v4();
        self.sessions
            .entry(msg.user_id)
            .or_insert_with(HashMap::new)
            .insert(id, msg.addr);
        id.to_string()
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");
        if let Some(user_sessions) = self.sessions.get_mut(&msg.user_id) {
            user_sessions.remove(&msg.session_id);
            if user_sessions.len() == 0 {
                self.sessions.remove(&msg.user_id);
            }
        }
    }
}

impl Handler<MessageFromClient> for ChatServer {
    type Result = ResponseFuture<Result<(), ()>>;

    fn handle(&mut self, _msg: MessageFromClient, _: &mut Context<Self>) -> Self::Result {
        Box::pin(async move {
            let producer = Arc::new(KafkaProducer::new());
            let usecase = ChatUseCase::new(producer);
            if let Err(err) = usecase.new_message().await {
                log::error!("{:?}", err);
            }
            Ok(())
        })
    }
}

impl Handler<MessageFromBroker> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: MessageFromBroker, _: &mut Context<Self>) -> Self::Result {
        self.send_message(&msg.msg, &msg.sender_id, &msg.session_id, &msg.receiver_id);
    }
}
