use crate::model;
use rosu_pp::{osu::OsuScoreState, Beatmap, OsuPP};

pub struct PPCalc {
    map: Beatmap,
    mods: u32,
}

impl PPCalc {
    pub fn from_path(path: &String, mods: u32) -> Result<Self, ()> {
        match Beatmap::from_path(path) {
            Ok(bm) => Ok(Self { map: bm, mods }),
            Err(e) => Err(println!("{}", e)),
        }
    }

    fn get_state(frame: &model::Model) -> OsuScoreState {
        let g = &frame.gameplay;
        OsuScoreState {
            max_combo: g.combo.max as usize,
            n300: g.hits.count_300 as usize,
            n100: g.hits.count_100 as usize,
            n50: g.hits.count_50 as usize,
            n_misses: g.hits.count_miss as usize,
        }
    }

    fn remove_misses(state: &mut OsuScoreState, frame: &model::Model) {
        // assume misses are instead 300's
        state.n300 += state.n_misses;
        state.n_misses = 0;
        state.max_combo = frame.menu.bm.stats.max_combo as usize
    }

    pub fn current(&self, frame: &model::Model) -> f64 {
        let state = Self::get_state(frame);
        
        OsuPP::new(&self.map)
            .mods(self.mods)
            .state(state)
            .calculate()
            .pp()
    }

    pub fn fc(&self, frame: &model::Model) -> f64 {
        let mut state = Self::get_state(frame);
        Self::remove_misses(&mut state, frame);

        OsuPP::new(&self.map)
            .mods(self.mods)
            .state(state)
            .calculate()
            .pp()
    }

    // not very efficient i know
    pub fn ss(&self) -> f64 {
        OsuPP::new(&self.map)
            .mods(self.mods)
            .accuracy(100.0)
            .calculate()
            .pp()
    }

    pub fn pp(&self, frame: &model::Model) -> model::gameplay::PP {
        model::gameplay::PP {
            current: self.current(frame),
            fc: self.fc(frame),
            ss: self.ss()
        }
    }
}
