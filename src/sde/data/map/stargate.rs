use serde::{Deserialize, Serialize};

use crate::sde::{data::map, types::ids};

pub(crate) const STARGATES_FILENAME: &str = "mapStargates.jsonl";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Stargate {
    #[serde(rename = "_key")]
    pub(crate) id: ids::StargateID,
    #[serde(rename = "solarSystemID")]
    pub(crate) solar_system_id: ids::SolarSystemID,
    pub(crate) position: map::Position,
    pub(crate) destination: Destination,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Destination {
    #[serde(rename = "solarSystemID")]
    pub(crate) solar_system_id: ids::SolarSystemID,
    #[serde(rename = "stargateID")]
    pub(crate) stargate_id: ids::StargateID,
}
