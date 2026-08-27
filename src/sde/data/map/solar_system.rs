use serde::{Deserialize, Serialize};

use crate::sde::{
    data::{Name, map},
    types::{ids, values},
};

pub(crate) const SOLAR_SYSTEMS_FILENAME: &str = "mapSolarSystems.jsonl";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SolarSystem {
    #[serde(rename = "_key")]
    pub(crate) id: ids::SolarSystemID,
    pub(crate) name: Name,
    #[serde(rename = "constellationID")]
    pub(crate) constellation_id: ids::ConstellationID,
    #[serde(rename = "securityStatus")]
    pub(crate) security_status: values::SecurityStatus,
    pub(crate) position: map::Position,
    #[serde(rename = "position2D")]
    pub(crate) position_2d: Option<map::Position2d>,
    #[serde(rename = "stargateIDs")]
    pub(crate) stargate_ids: Option<Vec<ids::StargateID>>,
}
