pub(crate) mod category;
pub(crate) mod group;
pub(crate) mod r#type;

use anyhow::Result;
use tempfile::TempDir;

use crate::sde;

#[derive(Default, Debug)]
pub(crate) struct Entity {
    pub(crate) categories: Vec<category::Category>,
    pub(crate) groups: Vec<group::Group>,
    pub(crate) types: Vec<r#type::Type>,
}

pub(crate) fn update(tmp_dir: &TempDir, entity: &mut Entity) -> Result<()> {
    entity.categories = sde::load_sde_data(tmp_dir, category::CATEGORIES_FILENAME)?;
    entity.groups = sde::load_sde_data(tmp_dir, group::GROUPS_FILENAME)?;
    entity.types = sde::load_sde_data(tmp_dir, r#type::TYPES_FILENAME)?;

    Ok(())
}
