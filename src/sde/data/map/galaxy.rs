use std::io::Write;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::{
    config,
    sde::{
        data::{Description, Name},
        types::ids,
    },
};

const KSPACE_ID: ids::GalaxyID = 1;
const JSPACE_ID: ids::GalaxyID = 2;
const ABYSS_ID: ids::GalaxyID = 3;
const VOID_ID: ids::GalaxyID = 4;
const HIDDEN_ID: ids::GalaxyID = 5;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Galaxy {
    #[serde(rename = "_key")]
    pub(crate) id: ids::GalaxyID,
    pub(crate) name: Name,
    pub(crate) description: Description,
}

pub(crate) fn load() -> Result<Vec<Galaxy>> {
    // HARDCODED VALUE FROM CCP DOCUMENTATION
    // see: https://developers.eveonline.com/docs/guides/id-ranges/#regions
    print!("\rCreating Galaxies Structures");
    std::io::stdout().flush().context("flushing stdout")?;
    let mut galaxies: Vec<Galaxy> = Vec::new();
    galaxies.push(Galaxy {
        id: KSPACE_ID,
        name: Name {
            en: "NEW EDEN".to_string(),
            ..Default::default()
        },
        description: Description {
            en: "New Eden (known space) regions".to_string(),
            ..Default::default()
        },
    });
    galaxies.push(Galaxy {
        id: JSPACE_ID,
        name: Name {
            en: "ANOIKIS".to_string(),
            ..Default::default()
        },
        description: Description {
            en: "Wormhole regions".to_string(),
            ..Default::default()
        },
    });
    galaxies.push(Galaxy {
        id: ABYSS_ID,
        name: Name {
            en: "ABYSSAL DEADSPACE".to_string(),
            ..Default::default()
        },
        description: Description {
            en: "Abyssal regions".to_string(),
            ..Default::default()
        },
    });
    galaxies.push(Galaxy {
        id: VOID_ID,
        name: Name {
            en: "JOVE".to_string(),
            ..Default::default()
        },
        description: Description {
            en: "Jove regions".to_string(),
            ..Default::default()
        },
    });
    galaxies.push(Galaxy {
        id: HIDDEN_ID,
        name: Name {
            en: "GLOBAL MARKET".to_string(),
            ..Default::default()
        },
        description: Description {
            en: "Global PLEX market region".to_string(),
            ..Default::default()
        },
    });
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(galaxies)
}

pub(crate) fn get_galaxy_id(region_id: ids::RegionID) -> Result<ids::GalaxyID> {
    match region_id {
        10_000_000..=10_999_999 => Ok(KSPACE_ID),
        11_000_000..=11_999_999 => Ok(JSPACE_ID),
        12_000_000..=12_999_999 => Ok(ABYSS_ID),
        14_000_000..=14_999_999 => Ok(VOID_ID),
        19_000_000..=19_999_999 => Ok(HIDDEN_ID),
        _ => bail!(
            "region_id value '{}' is invalid per CCP documentation",
            region_id
        ),
    }
}
