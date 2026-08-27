use serde::{Deserialize, Serialize};

use crate::sde::{
    data::{Description, Name},
    types::{ids, values},
};

pub(crate) const TYPES_FILENAME: &str = "types.jsonl";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Type {
    #[serde(rename = "_key")]
    pub(crate) id: ids::TypeID,
    pub(crate) description: Option<Description>,
    #[serde(rename = "groupID")]
    pub(crate) group_id: ids::GroupID,
    pub(crate) name: Name,
    pub(crate) published: bool,
    #[serde(rename = "isRepackable")]
    pub(crate) is_repackable: Option<bool>,
    #[serde(rename = "packagedVolume")]
    pub(crate) packaged_volume: Option<values::Volume>,
    pub(crate) volume: Option<values::Volume>,
}
