mod model;
mod utils;

use model::Model;
use mongodb::bson::Document;
use utils::read_once;

fn main() {
    // Connect to gosumemory
    let mut socket = utils::connect_gosu("ws://localhost:24050/ws");

    // Connect to db
    let db = utils::connect_db("mongodb://localhost:27017", "osu");
    let coll = db.collection::<Document>("rust_structs");
    println!("Ready.");

    let first_frame: Model =
        serde_json::from_str(&read_once(&mut socket)).expect("Can't parse to Model");

    let mut prev_state = first_frame.menu.state;
    // let mut prev_frame = remove_useless(first_frame);
    let mut prev_frame = first_frame;
    let mut prev_hits = prev_frame.gameplay.hits.hit_sum();
    let mut last_dumped_hits = u32::MAX;
    loop {
        // get ws message
        let msg = read_once(&mut socket);

        // parse to Model
        let curr_frame: Model = serde_json::from_str(&msg).expect("Can't parse to Model");

        // get new state
        let curr_state = curr_frame.menu.state;
        let curr_hits = curr_frame.gameplay.hits.hit_sum();

        // a play is valid if:
        // the play has at least 1 hit
        // AND either
        // state changed from 2 (gameplay ended)
        // OR
        // hitcount reset (map restarted, state remained 2)
        if (prev_hits > 0 && (last_dumped_hits != prev_hits))
            && (((curr_state != prev_state) && (prev_state == 2)) || (curr_hits == 0))
        {
            utils::print_score(&prev_frame);
            utils::dump_to_db(&prev_frame, &coll);
            last_dumped_hits = prev_hits;
        }

        prev_state = curr_state;
        prev_frame = curr_frame;
        prev_hits = curr_hits;
    }
}
