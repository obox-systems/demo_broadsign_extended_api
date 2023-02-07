use std::{net::TcpListener, error::Error};

use broadsign_extended_api::run;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let listener = TcpListener::bind("127.0.0.1:8080")
    .expect("Failed to bind on port 8080");
  
  _ = run(listener)?.await;
  
  Ok(())
}
