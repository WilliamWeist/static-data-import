use serde::{Deserialize, Serialize};

use crate::sde::{data::Name, types::ids};

pub(crate) const GROUPS_FILENAME: &str = "groups.jsonl";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Group {
    #[serde(rename = "_key")]
    pub(crate) id: ids::GroupID,
    #[serde(rename = "categoryID")]
    pub(crate) category_id: ids::CategoryID,
    pub(crate) name: Name,
    pub(crate) published: bool,
}
