use std::net::TcpListener;

#[actix_web::test]
async fn connection() {
  fake_player_api();
  let addr = spawn_app();
  let (resp, _connection) = awc::Client::new()
    .ws(format!("{addr}/player"))
    .connect()
    .await
    .unwrap();
  assert!(resp.status().is_success())
}

fn fake_player_api() {
  let listener = match TcpListener::bind("127.0.0.1:2324") {
    Ok(list) => list,
    _ => return,
  };
  let server = broadsign_extended_api::run(listener).expect("Failed to start fake player server");
  _ = actix::spawn(server);
}

fn spawn_app() -> String {
  let listener = TcpListener::bind("[::1]:0").expect("Failed to bind to a random port");
  let port = listener.local_addr().unwrap().port();
  let server = broadsign_extended_api::run(listener).expect("Failed to start the server");
  _ = actix::spawn(server);
  format!("ws://[::1]:{port}")
}
