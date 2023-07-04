use serde_json::{Map, Value};
use std::{io, io::Write, net::TcpStream};
use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};
use url::Url;

fn read_once(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) -> String {
    let msg = socket.read_message().expect("Error reading message");
    let msg = match msg {
        tungstenite::Message::Text(s) => s,
        _ => {
            panic!()
        }
    };
    msg
}

fn hit_sum(h: &Map<String, Value>) -> u64 {
    let _300 = h["300"].as_u64().unwrap();
    let _100 = h["100"].as_u64().unwrap();
    let _50 = h["50"].as_u64().unwrap();
    let _0 = h["0"].as_u64().unwrap();

    _300 + _100 + _50 + _0
}

fn main() {
    // Connect to the WS server locally
    print!("Connecting to gosumemory ... ");
    io::stdout().flush().unwrap();
    let (mut socket, _) =
        connect(Url::parse("ws://localhost:24050/ws").unwrap()).expect("Can't connect");
    println!("OK\nReady.");

    loop {
        // get ws message
        let msg = read_once(&mut socket);

        // parse to json
        let curr_frame: Map<String, Value> =
            serde_json::from_str(&msg).expect("Can't parse to JSON");

        let hits = curr_frame["gameplay"]["hits"].as_object().unwrap();
        let l = match hits["hitErrorArray"].as_array() {
            Some(v) => v.len(),
            None => 0,
        };
        let s = curr_frame["gameplay"]["score"].as_u64().unwrap();

        println!("{0} {1} {2}", hit_sum(hits), l, s);
        // after finishing a map, 
        // restarting it from the results screen
        // hitting some notes and then restarting again
        // hitErrorArray is not reset
        // 3 3 960
        // 0 3 0
        // 0 3 0
        // 0 3 0
        // 0 3 0
        // ???
        // bruh
    }
}
