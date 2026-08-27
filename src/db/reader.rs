use std::io::Write;

use anyhow::{Context, Result};
use rusqlite::{Connection, Error as RusqliteError};

use crate::{config, sde};

pub(crate) fn get_sde_build() -> Result<Option<sde::Build>> {
    print!("Fetching local SDE Build information...");
    std::io::stdout().flush().context("flushing stdout")?;

    let db: Connection =
        Connection::open(config::LOCAL_DATABASE).context("Opening/Creating local DB file")?;
    let build: Option<sde::Build>;
    let prepare = db.prepare("SELECT id, build_number, release_date FROM build");
    match prepare {
        Ok(prepare) => {
            let mut stmt: rusqlite::Statement<'_> = prepare;
            let mut rows: rusqlite::Rows<'_> = stmt.query(())?;
            match rows.next()? {
                Some(row) => {
                    build = Some(sde::Build {
                        id: row.get("id")?,
                        number: row.get("build_number")?,
                        release_date: row.get("release_date")?,
                    });
                }
                None => build = None,
            };
            println!(" {}", config::SUCCESS_SYMBOL);
        }
        Err(RusqliteError::SqliteFailure(_, Some(msg))) if msg.contains("no such table") => {
            build = None
        }
        Err(err) => return Err(err.into()),
    }

    Ok(build)
}
