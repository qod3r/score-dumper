use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Folders {
    pub game: String,
    pub skin: String,
    pub songs: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub show_interface: bool,
    pub folders: Folders,
}
