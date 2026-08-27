pub(crate) mod constellation;
pub(crate) mod galaxy;
pub(crate) mod region;
pub(crate) mod solar_system;
pub(crate) mod stargate;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tempfile::TempDir;

use crate::{
    config,
    sde::{self, types::values},
};

#[derive(Default, Debug)]
pub(crate) struct Map {
    pub(crate) galaxies: Vec<galaxy::Galaxy>,
    pub(crate) regions: Vec<region::Region>,
    pub(crate) constellations: Vec<constellation::Constellation>,
    pub(crate) solar_systems: Vec<solar_system::SolarSystem>,
    pub(crate) stargates: Vec<stargate::Stargate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Position {
    pub(crate) x: values::Coordinate,
    pub(crate) y: values::Coordinate,
    pub(crate) z: values::Coordinate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Position2d {
    pub(crate) x: values::Coordinate,
    pub(crate) y: values::Coordinate,
}

pub(crate) fn update(tmp_dir: &TempDir, map: &mut Map) -> Result<()> {
    map.galaxies = galaxy::load()
        .context("parsing hardcoded GALAXY input, should not happen")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?;
    map.regions = sde::load_sde_data(tmp_dir, region::REGIONS_FILENAME)?;
    map.constellations = sde::load_sde_data(tmp_dir, constellation::CONSTELLATIONS_FILENAME)?;
    map.solar_systems = sde::load_sde_data(tmp_dir, solar_system::SOLAR_SYSTEMS_FILENAME)?;
    map.stargates = sde::load_sde_data(tmp_dir, stargate::STARGATES_FILENAME)?;

    Ok(())
}
