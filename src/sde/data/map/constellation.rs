use serde::{Deserialize, Serialize};

use crate::sde::{
    data::{Name, map},
    types::ids,
};

pub(crate) const CONSTELLATIONS_FILENAME: &str = "mapConstellations.jsonl";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Constellation {
    #[serde(rename = "_key")]
    pub(crate) id: ids::ConstellationID,
    pub(crate) name: Name,
    #[serde(rename = "regionID")]
    pub(crate) region_id: ids::RegionID,
    #[serde(rename = "solarSystemIDs")]
    pub(crate) solar_system_ids: Vec<ids::SolarSystemID>,
    pub(crate) position: map::Position,
}
