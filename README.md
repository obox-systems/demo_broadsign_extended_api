# API Middleman demo

This project was meant to be a middleman between two APIs.

It connects to the websocket and its endpoints, parses data they give
and transforms that data to the correct form for another API websocket.

The communication is done via JSON messages.

# Try it out!

1. Install [Rust](https://rustup.rs/)
2. Setup two websocket servers
- For example, you can use [websocat](https://github.com/vi/websocat):
```bash
$ cargo install websockat
$ websockat -s 8080 &
$ websockat -s 6666 &
```
3. Run the app to begin the communication:
```bash
$ cargo run --release
```
