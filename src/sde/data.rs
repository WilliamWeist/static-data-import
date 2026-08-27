pub(crate) mod entity;
pub(crate) mod industry;
pub(crate) mod map;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Name {
    pub(crate) en: String,
    pub(crate) de: Option<String>,
    pub(crate) es: Option<String>,
    pub(crate) fr: Option<String>,
    pub(crate) ja: Option<String>,
    pub(crate) ko: Option<String>,
    pub(crate) ru: Option<String>,
    pub(crate) zh: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Description {
    pub(crate) en: String,
    pub(crate) de: Option<String>,
    pub(crate) es: Option<String>,
    pub(crate) fr: Option<String>,
    pub(crate) ja: Option<String>,
    pub(crate) ko: Option<String>,
    pub(crate) ru: Option<String>,
    pub(crate) zh: Option<String>,
}
