use serde::{Deserialize, Serialize};

use crate::sde::types::{ids, values};

pub(crate) const BLUEPRINTS_FILENAME: &str = "blueprints.jsonl";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Blueprint {
    #[serde(rename = "_key")]
    pub(crate) id: ids::TypeID,
    pub(crate) activities: Activities,
    #[serde(rename = "blueprintTypeID")]
    pub(crate) blueprint_type_id: ids::TypeID,
    #[serde(rename = "maxProductionLimit")]
    pub(crate) max_production_limit: values::MaxProductionLimit,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Activities {
    pub(crate) copying: Option<Copying>,
    pub(crate) invention: Option<Invention>,
    pub(crate) manufacturing: Option<Manufacturing>,
    pub(crate) reaction: Option<Reaction>,
    #[serde(rename = "research_material")]
    pub(crate) research_material: Option<ResearchMaterial>,
    #[serde(rename = "research_time")]
    pub(crate) research_time: Option<ResearchTime>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Copying {
    pub(crate) materials: Option<Vec<Material>>,
    pub(crate) skills: Option<Vec<Skill>>,
    pub(crate) time: values::Time,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Invention {
    pub(crate) materials: Option<Vec<Material>>,
    pub(crate) products: Option<Vec<Product>>,
    pub(crate) skills: Option<Vec<Skill>>,
    pub(crate) time: values::Time,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Manufacturing {
    pub(crate) materials: Option<Vec<Material>>,
    pub(crate) products: Option<Vec<Product>>,
    pub(crate) skills: Option<Vec<Skill>>,
    pub(crate) time: values::Time,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Reaction {
    pub(crate) materials: Option<Vec<Material>>,
    pub(crate) products: Option<Vec<Product>>,
    pub(crate) skills: Option<Vec<Skill>>,
    pub(crate) time: values::Time,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ResearchMaterial {
    pub(crate) materials: Option<Vec<Material>>,
    pub(crate) skills: Option<Vec<Skill>>,
    pub(crate) time: values::Time,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ResearchTime {
    pub(crate) materials: Option<Vec<Material>>,
    pub(crate) skills: Option<Vec<Skill>>,
    pub(crate) time: values::Time,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Material {
    pub(crate) quantity: values::Quantity,
    #[serde(rename = "typeID")]
    pub(crate) type_id: ids::TypeID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Product {
    pub(crate) probability: Option<values::Probability>,
    pub(crate) quantity: values::Quantity,
    #[serde(rename = "typeID")]
    pub(crate) type_id: ids::TypeID,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Skill {
    pub(crate) level: values::Level,
    #[serde(rename = "typeID")]
    pub(crate) type_id: ids::TypeID,
}
