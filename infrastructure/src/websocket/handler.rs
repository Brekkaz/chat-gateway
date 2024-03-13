use super::{server::ChatServer, session::WsChatSession};
use actix::*;
use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::Instant;
use uuid::Uuid;

pub async fn handler_ws(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    let user_id = req
        .headers()
        .get("user_id")
        .ok_or_else(|| error::ErrorBadRequest("Falta el encabezado 'user_id'"))?;

    let user_id = Uuid::parse_str(
        user_id
            .to_str()
            .map_err(|_| error::ErrorBadRequest("Valor de encabezado 'user_id' inválido"))?,
    )
    .map_err(|_| error::ErrorBadRequest("Valor de encabezado 'user_id' inválido"))?;

    ws::start(
        WsChatSession {
            id: Uuid::nil(),
            user_id,
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
