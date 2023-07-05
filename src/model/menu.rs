use serde::{Deserialize, Serialize};
// use serde_json::{Map, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mods {
    pub num: u32,
    pub str: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeatmapPath {
    pub folder: String,
    pub file: String,
    pub bg: String,
    pub audio: String,
    // #[serde(flatten)]
    // other: Map<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BeatmapMetadata {
    pub artist: String,
    pub title: String,
    pub mapper: String,
    pub difficulty: String,
    // #[serde(flatten)]
    // other: Map<String, Value>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct BeatmapDifficulty {
//     #[serde(rename = "AR")]
//     pub ar: f64,
//     #[serde(rename = "CS")]
//     pub cs: f64,
//     #[serde(rename = "OD")]
//     pub od: f64,
//     #[serde(rename = "HP")]
//     pub hp: f64,
//     #[serde(rename = "fullSR")]
//     pub sr: f64,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct BPM {
    pub min: f64,
    pub max: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BeatmapStats {
    // #[serde(flatten)]
    // pub diff: BeatmapDifficulty,
    #[serde(rename = "AR")]
    pub ar: f64,
    #[serde(rename = "CS")]
    pub cs: f64,
    #[serde(rename = "OD")]
    pub od: f64,
    #[serde(rename = "HP")]
    pub hp: f64,
    #[serde(rename = "fullSR")]
    pub sr: f64,
    pub max_combo: u32,
    #[serde(rename = "BPM")]
    pub bpm: BPM,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Beatmap {
    // idk if these three are returned on unsubmitted maps
    pub id: i32,
    pub set: i32,
    pub md5: String,
    pub ranked_status: u32,
    pub metadata: BeatmapMetadata,
    pub stats: BeatmapStats,
    pub path: BeatmapPath,
    // imma pull the rest myself
    // #[serde(flatten)]
    // other: Map<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Menu {
    pub state: i32,
    pub game_mode: u32,
    pub bm: Beatmap,
    pub mods: Mods,
    // pp, chatEnabled and mainMenu
    // #[serde(flatten)]
    // other: Map<String, Value>,
}
