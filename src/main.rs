mod model;
mod utils;

use model::Model;
use mongodb::bson::Document;
use std::collections::VecDeque;
use utils::read_once;

// TODO: async
fn main() {
    // Connect to gosumemory
    let mut socket = utils::connect_gosu("ws://localhost:24050/ws");

    // Connect to db
    let db = utils::connect_db("mongodb://localhost:27017", "osu");
    let coll_name = format!("rust_release_{}", option_env!("CARGO_PKG_VERSION").unwrap());
    let coll = db.collection::<Document>(coll_name.as_str());
    println!("Using collection {}", coll_name);

    let buf_max = 30;
    println!("Using buffer with size {}", buf_max);
    println!("Ready.");

    // use a buffer of frames to compare against
    // because using just two frames sometimes results in some weird stuff happening
    let mut buf: VecDeque<Model> = VecDeque::with_capacity(buf_max + 1);
    // have at least two frames before logic, a bit janky but whatever
    buf.push_back(
        serde_json::from_str::<Model>(&read_once(&mut socket)).expect("Can't parse to Model"),
    );
    let mut prev_state = buf.get(0).expect("Can't buf.get(0)").menu.state;
    let mut last_submitted_sum = u32::MAX;
    // prevent logging spam when gosu freaks out at aspire maps
    let mut skipped_recently = false;
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
        // skip if the game is closed
        if curr_state == -1 {
            continue;
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
        if (prev_state == 2 && curr_state != 2) || (curr_frame.gameplay.sum() == 0) {
            let max = buf
                .iter()
                .max_by_key(|f| &f.gameplay) // if several frames are equal, returns the most recent one
                .expect("Couldn't get max by key");
            if (max.gameplay.sum() != 0) && (max.gameplay.sum() != last_submitted_sum) {
                utils::print_score(max);
                utils::dump_to_db(max, &coll);

                // TODO: figure out how to not duplicate a submit after retrying from the results screen instead of checking the sum
                // BUG: valid subsequent scores with the same score and hit_sum are not submitted
                // probably not that important since it's very rare
                last_submitted_sum = max.gameplay.sum();
                buf.clear();
            }
        }
        prev_state = curr_state;
    }
}
