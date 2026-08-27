mod config;
mod db;
mod sde;

use std::{fs, path::PathBuf};

use anyhow::{Context, Result, bail};
use rusqlite::Connection;
use tempfile::TempDir;

pub fn update(tmp_dir: &TempDir) -> Result<()> {
    // Compare CCP build number with local build number
    let mut sde: sde::SDE = sde::SDE {
        build: sde::fetch_build().context("fetching CCP SDE Build info")?,
        ..Default::default()
    };
    if sde::is_out_of_date(&sde.build)
        .context("fetching local SDE Build info")
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))?
    {
        // Download SDE
        sde::download(&sde.build, &tmp_dir).context("downloading SDE from CCP")?;

        // Extract SDE
        sde::extract(&tmp_dir).context("extracting SDE")?;

        // Parse SDE
        sde::data::map::update(tmp_dir, &mut sde.map).context("updating map data")?;
        sde::data::entity::update(tmp_dir, &mut sde.entity).context("updating entity data")?;
        sde::data::industry::update(tmp_dir, &mut sde.industry)
            .context("updating industry data")?;

        // Convert to SQL
        let tmp_db_file: &PathBuf = &tmp_dir.path().join(config::LOCAL_DATABASE);
        let mut db: Connection =
            Connection::open(tmp_db_file).context("Opening/Creating DB file")?;
        db::schema::initialize_database(&db).context("Initializing the database")?;
        db::writer::write_records(&mut db, &sde).context("Writing records to database")?;

        if let Err(error) = db.close() {
            bail!("closing EVE.db connection: {:#?}", error);
        }
        fs::copy(&tmp_db_file, format!("{}.tmp", config::LOCAL_DATABASE))
            .context("Moving tmp/EVE.db to ./EVE.db.tmp")?;
        fs::rename(
            format!("{}.tmp", config::LOCAL_DATABASE),
            config::LOCAL_DATABASE,
        )
        .context("Renaming ./EVE.db.tmp to ./EVE.db")?;
    }

    Ok(())
}
