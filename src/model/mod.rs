pub mod gameplay;
pub mod menu;
pub mod settings;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub settings: settings::Settings,
    pub menu: menu::Menu,
    pub gameplay: gameplay::Gameplay,
    #[serde(default = "default_dt")]
    pub timestamp: bson::DateTime,
}

fn default_dt() -> bson::DateTime {
    bson::DateTime::from_chrono(Utc::now())
}
