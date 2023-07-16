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
    let coll = db.collection::<Document>("rust_release_0.2.0");
    println!("Ready.");

    // use a buffer of frames to compare against
    // because using just two frames sometimes results in some weird stuff happening
    let buf_max = 10;
    let mut buf: VecDeque<Model> = VecDeque::with_capacity(buf_max + 1);
    // have at least two frames before logic, a bit janky but whatever
    buf.push_back(
        serde_json::from_str::<Model>(&read_once(&mut socket)).expect("Can't parse to Model"),
    );
    let mut prev_state = buf.get(0).expect("Can't buf.get(0)").menu.state;
    loop {
        let msg = read_once(&mut socket);
        let frame: Model = serde_json::from_str(&msg).expect("Can't parse to Model");
        let curr_state = frame.menu.state;

        // // skip if the game is closed
        // if curr_state == -1 {
        //     // TODO: panic for now
        //     panic!("osu! is closed");
        // }

        // keep the last `buf_max` frames
        if buf.len() >= buf_max {
            buf.pop_front();
        }
        buf.push_back(frame);
        let curr_frame = buf.back().expect("Can't get back of buf");

        // wait for gameplay
        // check if stuff is zero
        // iterate and compare
        // select the best candidate and submit

        // search for a submittable score when
        // state changed from 2 (quit or finish)
        // state didn't change but gameplay values did (restart)
        if (prev_state == 2 && curr_state != 2) || (curr_frame.gameplay.sum() == 0) {
            let max = buf
                .iter()
                .max_by_key(|f| &f.gameplay)
                .expect("Couldn't get max by key");
            if max.gameplay.sum() != 0 {
                utils::print_score(max);
                // println!("{:#?}", buf);

                // // clear the buffer except the last two frames to keep the logic going
                // let mut drained: VecDeque<Model> = buf.drain(buf.len() - 2..).collect();
                
                // clear the buffer except the last (empty) frame
                // let last = buf.pop_back().unwrap();
                buf.clear();
                // buf.append(&mut drained);
                // buf.push_back(last);
                
                // println!("after clear: {:#?}", buf);
            }
            // else
            // println!("buf does not have submittable scores, skipping...");
        }

        prev_state = curr_state;
    }

    // let first_frame: Model =
    //     serde_json::from_str(&read_once(&mut socket)).expect("Can't parse to Model");

    // let mut prev_state = first_frame.menu.state;
    // let mut prev_frame = first_frame;
    // let mut prev_hits = prev_frame.gameplay.hits.hit_sum();
    // let mut last_dumped_hits = u32::MAX;
    // loop {
    //     // get ws message
    //     let msg = read_once(&mut socket);

    //     // parse to Model
    //     let curr_frame: Model = serde_json::from_str(&msg).expect("Can't parse to Model");

    //     // get new state
    //     let curr_state = curr_frame.menu.state;
    //     let curr_hits = curr_frame.gameplay.hits.hit_sum();

    //     // a play is valid if:
    //     // the play has at least 1 hit
    //     // AND either
    //     // state changed from 2 (gameplay ended)
    //     // OR
    //     // hitcount reset (map restarted, state remained 2)

    //     // BUG: quick retry submits big combo 0 acc plays (all hits are 300's)
    //     //      probably another gosumemory quirk
    //     //      maybe i should count hits with something else
    //     //      i swear gosumemory has so many edge cases 💀
    //     if (prev_hits > 0 && (last_dumped_hits != prev_hits))
    //         && (((curr_state != prev_state) && (prev_state == 2)) || (curr_hits == 0))
    //     {
    //         utils::print_score(&prev_frame);
    //         utils::dump_to_db(&prev_frame, &coll);
    //         last_dumped_hits = prev_hits;
    //     }

    //     prev_state = curr_state;
    //     prev_frame = curr_frame;
    //     prev_hits = curr_hits;
    // }
}
