#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! This project was meant to be a middleman between two APIs.

use std::{error::Error, net::TcpListener};

use actix_web::{
  dev::Server,
  web::{self, BytesMut, Payload},
  App, HttpRequest, HttpResponse, HttpServer,
};
use futures::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const BROADSIGN_PREFIX: &str = "/player";
const OTHER_API_PREFIX: &str = "/other";

async fn player(req: HttpRequest, mut payload: Payload) -> HttpResponse {
  let url = if let Some(_) = req.path().strip_prefix(BROADSIGN_PREFIX) {
    "ws://127.0.0.1:2324" // Broadsign websocket
  } else {
    "ws://127.0.0.1:6666" // Other api websocket
  };
  let client = awc::Client::new().ws(url);
  let client = if let Some(key) = req.headers().get("sec-websocket-key") {
    client.set_header("sec-websocket-key", key)
  } else {
    client
  };
  let client = if let Some(key) = req.headers().get("sec-websocket-version") {
    client.set_header("sec-websocket-version", key)
  } else {
    client
  };
  let (res, socket) = client
    .server_mode()
    .connect()
    .await
    .expect("Failed to connect to the API");

  assert_eq!(res.status().as_u16(), 101);
  let mut io = socket.into_parts().io;

  let (mut tx, rx) = futures::channel::mpsc::unbounded();

  let mut buf = BytesMut::new();

  actix_web::rt::spawn(async move {
    loop {
      tokio::select! {
        res = payload.next() => {
          match res {
            None => return,
            Some(body) => {
              let body = body.unwrap();
              io.write_all(&body).await.unwrap();
            }
          }
        }
        res = io.read_buf(&mut buf) => {
          let size = res.unwrap();
          let bytes = buf.split_to(size).freeze();
          tx.send( Ok::<_, actix_web::Error>(bytes) ).await.unwrap();
        }
      }
    }
  });

  HttpResponse::SwitchingProtocols().streaming(rx)
}

/// Binds to the address and serves the middleware.
pub fn run(listener: TcpListener) -> Result<Server, Box<dyn Error>> {
  let server = HttpServer::new(move || {
    App::new()
      .service(web::resource(BROADSIGN_PREFIX).to(player))
      .service(web::resource(OTHER_API_PREFIX).to(player))
      .default_service(web::to(|| HttpResponse::NotFound()))
  })
  .listen(listener)?
  .run();

  Ok(server)
}
