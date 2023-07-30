use std::io::{self, Write};
use std::net::TcpStream;

use bson::Document;
use mongodb::options::ClientOptions;
use mongodb::sync::{Client, Collection, Database};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, WebSocket};
use url::Url;

use crate::model::Model;

pub fn connect_db(uri: &str, db: &str) -> Database {
    print!("Connecting to mongodb ... "); // TODO: catch errors
    io::stdout().flush().unwrap();
    let client_options = ClientOptions::parse(uri).unwrap();
    let client = Client::with_options(client_options).unwrap();
    println!("OK");
    client.database(db)
}

pub fn dump_to_db(model: &Model, coll: &Collection<Document>) {
    let doc = bson::to_document(&model).unwrap();
    let result = coll.insert_one(doc, None).unwrap().inserted_id;
    println!("Mongo _id: {}", result);
}

pub fn connect_gosu(uri: &str) -> WebSocket<MaybeTlsStream<TcpStream>> {
    print!("Connecting to gosumemory ... "); // TODO: wait till its launched
    io::stdout().flush().unwrap();
    let (socket, _) = connect(Url::parse(uri).unwrap()).expect("Can't connect");
    println!("OK");
    return socket;
}

pub fn read_once(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) -> String {
    let msg = socket.read_message().expect("Error reading message");

    let msg = match msg {
        tungstenite::Message::Text(s) => s,
        _ => panic!(),
    };
    msg
}

pub fn print_score(model: &Model) {
    let meta = &model.menu.bm.metadata;
    let song_artist = &meta.artist;
    let song_title = &meta.title;
    let diff = &meta.difficulty;

    let acc = &model.gameplay.accuracy;
    let combo = &model.gameplay.combo.max;
    let max_combo = &model.menu.bm.stats.max_combo;

    let mods = &model.menu.mods.str;

    println!(
        "\nSaved score on {0} - {1} [{2}] +{6} | {3}% {4}/{5}x ",
        song_artist, song_title, diff, acc, combo, max_combo, mods
    );
}
