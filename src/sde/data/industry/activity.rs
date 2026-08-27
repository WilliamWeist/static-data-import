use serde::{Deserialize, Serialize};

use crate::sde::types::ids;

pub(crate) const ACTIVITIES_FILENAME: &str = "industryActivities.jsonl";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Activity {
    #[serde(rename = "_key")]
    pub(crate) id: ids::IndustryActivityID,
    pub(crate) description: String,
    pub(crate) name: String,
}
