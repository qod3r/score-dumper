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
        // self.count_300 + self.count_100 + self.count_50 + self.count_miss
        self.count_300 + self.count_100 + self.count_50
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

impl Gameplay {
    pub fn sum(&self) -> u32 {
        self.score + self.hits.hit_sum()
    }
}

impl PartialEq for Gameplay {
    fn eq(&self, other: &Self) -> bool {
        // consider self == other
        // when sum of score and hit_sum() are equal
        self.sum() == other.sum()
        // self.score == other.score && self.hits.hit_sum() == other.hits.hit_sum()
    }
}

impl Eq for Gameplay {}

impl PartialOrd for Gameplay {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // consider self > other
        // when both score and hit_sum() are bigger
        // Some(self.sum().cmp(&other.sum()))
        Some(self.cmp(other))
        // Some(
        //     self.score.cmp(&other.score)
        //         .then(
        //             self.hits.hit_sum().cmp(&other.hits.hit_sum())),
        // )
    }
}

impl Ord for Gameplay {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sum().cmp(&other.sum())
    }
}