mod kafka;
mod websocket;

use actix::Actor;
use kafka::{ChatConsumer, KafkaConsumer, TOPIC_TEST};
use std::io;
use websocket::ChatServer;

pub async fn run() -> io::Result<()> {
    let server = ChatServer::new().start();
    let cp_server = server.clone();

    tokio::spawn(async move {
        KafkaConsumer::new(vec![TOPIC_TEST], ChatConsumer::new(cp_server))
            .consume()
            .await
    });

    websocket::init_server(server).await
}
