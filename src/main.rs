use anyhow::Result;
use tempfile::{TempDir, tempdir};

fn main() -> Result<()> {
    let tmp_dir: TempDir = tempdir()?;
    sdi::update(&tmp_dir)?;

    Ok(())
}
