use anyhow::{Context, Result};
use rusqlite::Connection;

pub(crate) fn initialize_database(db: &Connection) -> Result<()> {
    db.execute_batch(include_str!("SQL/00 - PRAGMA.sql"))
        .context("Configuring PRAGMA options")?;
    db.execute_batch(include_str!("SQL/01 - BUILD.sql"))
        .context("SDE Build Info table schema")?;
    db.execute_batch(include_str!("SQL/02 - COMMON.sql"))
        .context("Common tables schema")?;

    db.execute_batch(include_str!("SQL/MAP/00 - COMMON.sql"))
        .context("Map Common tables schema")?;
    db.execute_batch(include_str!("SQL/MAP/01 - GALAXY.sql"))
        .context("Map Galaxy schema")?;
    db.execute_batch(include_str!("SQL/MAP/02 - REGION.sql"))
        .context("Map Region schema")?;
    db.execute_batch(include_str!("SQL/MAP/03 - CONSTELLATION.sql"))
        .context("Map Constellation schema")?;
    db.execute_batch(include_str!("SQL/MAP/04 - SYSTEM.sql"))
        .context("Map Solar System schema")?;
    db.execute_batch(include_str!("SQL/MAP/05 - STARGATE.sql"))
        .context("Map Stargate schema")?;

    db.execute_batch(include_str!("SQL/MAP/06 - MAP_VIEW.sql"))
        .context("Map view schema")?;
    db.execute_batch(include_str!("SQL/MAP/07 - STARGATE_VIEW.sql"))
        .context("Stargate view schema")?;

    db.execute_batch(include_str!("SQL/ENTITY/01 - CATEGORY.sql"))
        .context("Entity Category schema")?;
    db.execute_batch(include_str!("SQL/ENTITY/02 - GROUP.sql"))
        .context("Entity Group schema")?;
    db.execute_batch(include_str!("SQL/ENTITY/03 - TYPE.sql"))
        .context("Entity Type schema")?;

    db.execute_batch(include_str!("SQL/ENTITY/04 - ENTITY_VIEW.sql"))
        .context("Entity view schema")?;

    db.execute_batch(include_str!("SQL/INDUSTRY/01 - ACTIVITY.sql"))
        .context("Industry Activity schema")?;
    db.execute_batch(include_str!("SQL/INDUSTRY/02 - BLUEPRINT.sql"))
        .context("Industry Blueprint schema")?;

    db.execute_batch(include_str!(
        "SQL/INDUSTRY/03 - BLUEPRINT_CATALOGUE_VIEW.sql"
    ))
    .context("Blueprint Catalogue view schema")?;
    db.execute_batch(include_str!(
        "SQL/INDUSTRY/04 - BLUEPRINT_MATERIALS_VIEW.sql"
    ))
    .context("Blueprint Materials view schema")?;

    Ok(())
}
