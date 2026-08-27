pub(crate) mod activity;
pub(crate) mod blueprint;

use anyhow::Result;
use tempfile::TempDir;

use crate::sde;

#[derive(Default, Debug)]
pub(crate) struct Industry {
    pub(crate) activities: Vec<activity::Activity>,
    pub(crate) blueprints: Vec<blueprint::Blueprint>,
}

pub(crate) fn update(tmp_dir: &TempDir, industry: &mut Industry) -> Result<()> {
    industry.activities = sde::load_sde_data(tmp_dir, activity::ACTIVITIES_FILENAME)?;
    industry.blueprints = sde::load_sde_data(tmp_dir, blueprint::BLUEPRINTS_FILENAME)?;

    Ok(())
}
