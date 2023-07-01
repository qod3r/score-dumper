use bson;
use chrono::Utc;
use mongodb::{
    bson::Document,
    options::ClientOptions,
    sync::{Client, Collection},
};
use serde_json::{Map, Value};
use std::{io, io::Write, net::TcpStream};
use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};
use url::Url;

fn dump_to_db(json: &Map<String, Value>, coll: &Collection<Document>) {
    // let map = remove_useless(map);
    let mut doc = bson::to_document(&json).unwrap();
    let dt: bson::DateTime = Utc::now().into();

    doc.insert("timestamp".to_owned(), dt).unwrap();
    let result = coll.insert_one(doc, None).unwrap().inserted_id;
    println!("inserted {}", result);
}

fn remove_useless(map: &mut Map<String, Value>) {
    // remove useless info
    map.get_mut("gameplay")
        .and_then(|v| v.as_object_mut())
        .and_then(|v| {
            v.remove("hp");
            v.remove("leaderboard");
            Some(v)
        });

    map.get_mut("menu")
        .and_then(|v| v.as_object_mut())
        .and_then(|v| {
            v.remove("mainMenu");
            Some(v)
        })
        .and_then(|v| v.get_mut("pp"))
        .and_then(|v| v.as_object_mut())
        .and_then(|v| v.remove("strains"));

    map.remove("tourney");
    map.remove("resultsScreen");
}

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

fn main() {
    // Connect to the WS server locally
    print!("Connecting to websocket ... ");
    io::stdout().flush().unwrap();
    let (mut socket, _) =
        connect(Url::parse("ws://localhost:24050/ws").unwrap()).expect("Can't connect");
    println!("OK");

    // Connect to db
    let client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
    print!("Connecting to mongodb ... ");
    io::stdout().flush().unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("osu");
    let coll = db.collection::<Document>("rust_first");
    println!("OK");

    let mut first_frame: Map<String, Value> =
        serde_json::from_str(&read_once(&mut socket)).expect("Can't parse to JSON");
    first_frame.insert("timestamp".to_owned(), Utc::now().timestamp().into());

    let mut prev_state = first_frame["menu"]["state"].to_owned();
    // let mut prev_frame = remove_useless(first_frame);
    let mut prev_frame = first_frame;
    let mut prev_hits: Value = prev_frame["gameplay"]["hits"]["hitErrorArray"].to_owned();
    let mut last_dumped_hits = prev_hits.to_owned();
    loop {
        // get ws message
        let msg = read_once(&mut socket);

        // parse to json
        let mut curr_frame: Map<String, Value> =
            serde_json::from_str(&msg).expect("Can't parse to JSON");
        curr_frame.insert("timestamp".to_owned(), Utc::now().to_string().into());

        // get new state
        let curr_state: Value = curr_frame["menu"]["state"].to_owned();
        let curr_hits: Value = curr_frame["gameplay"]["hits"]["hitErrorArray"].to_owned();

        // a play is valid if:
        // the play has at least 1 hit
        // AND either
        // state changed from 2 (gameplay ended)
        // OR
        // hitcount reset (map restarted, state remained 2)
        if (!prev_hits.is_null() && (last_dumped_hits != prev_hits))
            && (((curr_state != prev_state) && (prev_state == 2)) || (curr_hits.is_null()))
        {
            remove_useless(&mut prev_frame);
            println!("Saving data!\n{:#?}", prev_frame);
            // println!("{}", !prev_hits.is_null());
            // println!("{}", curr_state != prev_state);
            // println!("{}", prev_state == 2);
            // println!("{}", curr_hits.is_null());
            // println!("{}", prev_hits != last_dumped_hits);
            last_dumped_hits = prev_hits;
            dump_to_db(&prev_frame, &coll);
        }

        // if ((curr_state != prev_state) && (prev_state == 2) && (!prev_hits.is_null()))
        //     || ((!prev_hits.is_null()) && (curr_hits.is_null()))
        // {
        //     remove_useless(&mut prev_frame);
        //     println!("Saving data!\n{:#?}", prev_frame);
        //     dump_to_db(&prev_frame, &coll);
        // }
        prev_state = curr_state;
        prev_frame = curr_frame;
        prev_hits = curr_hits;
    }
}
