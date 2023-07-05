use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Combo {
    pub current: u32,
    pub max: u32,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Grade {
//     SS,
//     SSH,
//     S,
//     SH,
//     A,
//     B,
//     C,
//     D,
//     F,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Grade {
    pub current: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScoreStats {
    #[serde(rename = "0")]
    pub count_miss: u32,
    #[serde(rename = "50")]
    pub count_50: u32,
    #[serde(rename = "100")]
    pub count_100: u32,
    #[serde(rename = "300")]
    pub count_300: u32,
    pub slider_breaks: u32,
    pub grade: Grade,
    pub unstable_rate: f64,
}

impl ScoreStats {
    pub fn hit_sum(&self) -> u32 {
        self.count_300 + self.count_100 + self.count_50 + self.count_miss
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PP {
    pub current: f64,
    pub fc: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyCounts {
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyOverlay {
    pub k1: KeyCounts,
    pub k2: KeyCounts,
    pub m1: KeyCounts,
    pub m2: KeyCounts,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Gameplay {
    pub game_mode: u32,
    pub name: String,
    pub score: u32,
    pub accuracy: f64,
    pub combo: Combo,
    // hp
    pub hits: ScoreStats,
    pub pp: PP,
    pub key_overlay: KeyOverlay,
    // leaderboard
}
