use serde::{Deserialize, Serialize};

use crate::sde::{data::Name, types::ids};

pub(crate) const CATEGORIES_FILENAME: &str = "categories.jsonl";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Category {
    #[serde(rename = "_key")]
    pub(crate) id: ids::CategoryID,
    pub(crate) name: Name,
    pub(crate) published: bool,
}
