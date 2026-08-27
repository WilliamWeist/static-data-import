use serde::{Deserialize, Serialize};

use crate::sde::{
    data::{Description, Name, map},
    types::ids,
};

pub(crate) const REGIONS_FILENAME: &str = "mapRegions.jsonl";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Region {
    #[serde(rename = "_key")]
    pub(crate) id: ids::RegionID,
    pub(crate) name: Name,
    pub(crate) description: Option<Description>,
    #[serde(rename = "constellationIDs")]
    pub(crate) constellation_ids: Vec<ids::ConstellationID>,
    pub(crate) position: map::Position,
}
