mod model;
mod pp;
mod utils;

use model::Model;
use mongodb::bson::Document;
use std::collections::VecDeque;
use utils::read_once;

use crate::pp::PPCalc;

// TODO: async
fn main() {
    // Connect to gosumemory
    let mut socket = utils::connect_gosu("ws://localhost:24050/ws");

    // Connect to db
    let db = utils::connect_db("mongodb://localhost:27017", "osu");
    let coll_name = format!("rust_release_{}", option_env!("CARGO_PKG_VERSION").unwrap());
    let coll = db.collection::<Document>(coll_name.as_str());
    println!("Using collection {}", coll_name);

    let buf_max = 50; // buf_max / 10 = rough buffer length in seconds
    println!("Using buffer with size {}", buf_max);
    println!("Ready.");

    // use a buffer of frames
    // because gosu sometimes reads transitive memory states or something
    let mut buf: VecDeque<Model> = VecDeque::with_capacity(buf_max + 1);
    // have at least two frames before logic, a bit janky but whatever
    buf.push_back(
        serde_json::from_str::<Model>(&read_once(&mut socket)).expect("Can't parse to Model"),
    );
    let mut prev_state = buf.get(0).expect("Can't buf.get(0)").menu.state;
    let mut last_submitted_score = u32::MAX;
    // prevent logging spam when gosu freaks out at aspire maps
    let mut skipped_recently = false;
    let mut curr_path = String::from("");
    let mut calculator: Option<PPCalc> = None;
    loop {
        let msg = read_once(&mut socket);
        let frame: Model = match serde_json::from_str(&msg) {
            Ok(f) => f,
            Err(e) => {
                if skipped_recently {
                    continue;
                }
                println!("Can't parse to Model: {}", e.to_string());
                println!("Skipping frame...");
                skipped_recently = true;
                continue;
            }
        };
        skipped_recently = false;

        let curr_state = frame.menu.state;
        // skip non-std scores (for now) or if the game is closed
        if curr_state == -1 || frame.gameplay.game_mode != 0 {
            continue;
        }
        // bruh
        else if curr_state == 2 || curr_state == 7 || curr_state == 14 {
            // per-map PPCalc
            if curr_path.ne(&frame.menu.bm.path.file) {
                let calc_path = format!(
                    "{osu}\\Songs\\{folder}\\{file}",
                    osu = frame.settings.folders.game,
                    folder = frame.menu.bm.path.folder,
                    file = frame.menu.bm.path.file
                );
                calculator = match PPCalc::from_path(&calc_path, frame.menu.mods.num) {
                    Ok(c) => Some(c),
                    _ => None,
                };
                curr_path = frame.menu.bm.path.file.clone();
            }
        }

        // keep the last `buf_max` frames
        if buf.len() >= buf_max {
            buf.pop_front();
        }
        buf.push_back(frame);

        let curr_frame = buf.back().expect("Can't get back of buf");
        // search for a submittable score when
        // state changed from 2 (quit or finish)
        // state didn't change but gameplay values did (restart)
        if (prev_state == 2 && curr_state != 2) || (curr_frame.gameplay.is_empty()) {
            // get the frame with the highest score
            let mut max = buf
                .iter_mut()
                .max_by_key(|f| f.gameplay.score)
                .expect("Couldn't get max by key");

            // make sure it has stuff in it and that it wasn't submitted already
            if (max.gameplay.is_valid()) && (max.gameplay.score != last_submitted_score) {
                match calculator {
                    Some(ref calc) => {
                        max.gameplay.pp = calc.pp(&mut max);
                    }
                    None => { /* keep gosu values */ }
                }
                utils::print_score(max);
                utils::dump_to_db(max, &coll);
                // TODO: figure out how to not duplicate a submit after retrying from the results screen instead of checking the score
                // BUG: valid subsequent scores with the same `score` are not submitted
                //      probably not that important since it's very rare
                last_submitted_score = max.gameplay.score;
                buf.clear();
            }
        }
        prev_state = curr_state;
    }
}
