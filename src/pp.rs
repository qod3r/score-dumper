use crate::model;
use rosu_pp::{
    osu::OsuGradualPerformanceAttributes, osu::OsuScoreState, Beatmap, BeatmapExt,
    PerformanceAttributes,
};

// beware: actual schizo comments below

pub struct PPCalc<'a> {
    map: Beatmap,
    result: Option<PerformanceAttributes>,
    gradual: Option<OsuGradualPerformanceAttributes<'a>>,
}

// [~1am] HOLY SHIT
impl<'a> PPCalc<'a> {
    pub fn for_path(path: &String) -> Result<Self, ()> {
        // [~10pm]               let's avoid lifetimes
        match Beatmap::from_path(path.clone()) {
            // not sure if this is jank af
            Ok(bm) => Ok(Self {
                map: bm,
                result: None,
                gradual: None,
            }),
            Err(e) => Err(println!("{}", e)),
        }
    }

    // i cannot figure out how to do this in the `for_path` 💀
    pub fn create_gradual(&'a mut self, frame: model::Model) {
        self.gradual = Some(OsuGradualPerformanceAttributes::new(
            &self.map,
            frame.menu.mods.num,
        ));
    }

    pub fn calc(&'a mut self, frame: model::Model) -> model::gameplay::PP {
        // TODO: calc for curr, fc and (separately) ss and return PP
        self.gradual
            .as_mut()
            .unwrap()
            .process_next_object(OsuScoreState {
                max_combo: 0,
                n300: 0,
                n100: 0,
                n50: 0,
                n_misses: 0,
            });
        todo!()
    }
}
