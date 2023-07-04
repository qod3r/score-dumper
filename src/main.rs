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
    println!("Mongo _id: {}", result);
}

fn remove_useless(json: &mut Map<String, Value>) {
    // remove useless info
    json.get_mut("gameplay")
        .and_then(|v| v.as_object_mut())
        .and_then(|v| {
            v.remove("hp");
            v.remove("leaderboard");
            Some(v)
        });

    json.get_mut("menu")
        .and_then(|v| v.as_object_mut())
        .and_then(|v| {
            v.remove("mainMenu");
            Some(v)
        })
        .and_then(|v| v.get_mut("pp"))
        .and_then(|v| v.as_object_mut())
        .and_then(|v| v.remove("strains"));

    json.remove("tourney");
    json.remove("resultsScreen");
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

fn hit_sum(h: &Map<String, Value>) -> u64 {
    let _300 = h["300"].as_u64().unwrap();
    let _100 = h["100"].as_u64().unwrap();
    let _50 = h["50"].as_u64().unwrap();
    let _0 = h["0"].as_u64().unwrap();

    _300 + _100 + _50 + _0
}

fn print_score(json: &Map<String, Value>) {
    let song_artist = &json["menu"]["bm"]["metadata"]["artist"].as_str().unwrap();
    let song_title = &json["menu"]["bm"]["metadata"]["title"].as_str().unwrap();
    let diff = &json["menu"]["bm"]["metadata"]["difficulty"]
        .as_str()
        .unwrap();

    let acc = &json["gameplay"]["accuracy"].as_f64().unwrap();
    let combo = &json["gameplay"]["combo"]["max"].as_i64().unwrap();
    let max_combo = &json["menu"]["bm"]["stats"]["maxCombo"].as_i64().unwrap();

    let mods = &json["menu"]["mods"]["str"].as_str().unwrap();

    println!(
        "\nSaved score on {0} - {1} [{2}] +{6} | {3}% {4}/{5}x ",
        song_artist, song_title, diff, acc, combo, max_combo, mods
    );
    // println!("\t{0} {1}/{2}", acc, combo, max_combo);
}

fn main() {
    // Connect to the WS server locally
    print!("Connecting to gosumemory ... ");
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
    let coll = db.collection::<Document>("rust_release_0.1.0");
    println!("OK");
    println!("Ready.");

    let mut first_frame: Map<String, Value> =
        serde_json::from_str(&read_once(&mut socket)).expect("Can't parse to JSON");
    first_frame.insert("timestamp".to_owned(), Utc::now().timestamp().into());

    let mut prev_state = first_frame["menu"]["state"].to_owned();
    // let mut prev_frame = remove_useless(first_frame);
    let mut prev_frame = first_frame;
    let mut prev_hits = hit_sum(prev_frame["gameplay"]["hits"].as_object().unwrap());
    let mut last_dumped_hits = u64::MAX;
    loop {
        // get ws message
        let msg = read_once(&mut socket);

        // parse to json
        let mut curr_frame: Map<String, Value> =
            serde_json::from_str(&msg).expect("Can't parse to JSON");
        curr_frame.insert("timestamp".to_owned(), Utc::now().to_string().into());

        // get new state
        let curr_state: Value = curr_frame["menu"]["state"].to_owned();
        let curr_hits = hit_sum(curr_frame["gameplay"]["hits"].as_object().unwrap());

        // a play is valid if:
        // the play has at least 1 hit
        // AND either
        // state changed from 2 (gameplay ended)
        // OR
        // hitcount reset (map restarted, state remained 2)
        if (prev_hits > 0 && (last_dumped_hits != prev_hits))
            && (((curr_state != prev_state) && (prev_state == 2)) || (curr_hits == 0))
        {
            remove_useless(&mut prev_frame);
            // println!("Saving data!\n{:#?}", prev_frame);
            print_score(&prev_frame);
            // println!("{}", !prev_hits.is_null());
            // println!("{}", curr_state != prev_state);
            // println!("{}", prev_state == 2);
            // println!("{}", curr_hits.is_null());
            // println!("{}", prev_hits != last_dumped_hits);
            dump_to_db(&prev_frame, &coll);
            last_dumped_hits = prev_hits;
        }

        prev_state = curr_state;
        prev_frame = curr_frame;
        prev_hits = curr_hits;
    }
}
