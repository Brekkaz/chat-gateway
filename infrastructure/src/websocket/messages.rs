use actix::prelude::*;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageFromServer(pub String);

#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub user_id: Uuid,
    pub addr: Recipient<MessageFromServer>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub session_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct MessageFromClient {
    pub session_id: Uuid,
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageFromBroker {
    pub session_id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub msg: String,
}
