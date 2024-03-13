use crate::websocket::{handler::handler_ws, server::ChatServer};
use actix::prelude::*;
use actix_web::web;
use actix_web::{middleware::Logger, App, HttpServer};

pub async fn init_server(server: Addr<ChatServer>) -> std::io::Result<()> {
    let host: String = std::env::var("HTTP_HOST").expect("Can't get HTTP_HOST env var");
    let port: u16 = std::env::var("HTTP_PORT")
        .expect("Can't get HTTP_PORT env var")
        .parse()
        .expect("HTTP_PORT isn't a number");
    log::info!("Websocket server running at: ws://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").to(handler_ws))
            .app_data(web::Data::new(server.clone()))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind((host, port))?
    .run()
    .await
}
