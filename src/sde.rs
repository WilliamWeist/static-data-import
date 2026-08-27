pub(crate) mod data;
pub(crate) mod quirks;
pub(crate) mod types;

use std::{
    fs::{File, write},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    thread,
    time::Duration,
};

use anyhow::{Context, Result, bail};
use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::from_str;
use tempfile::TempDir;
use zip_extensions::zip_extract;

use crate::{config, db::reader, sde::types::values};

pub(crate) const SDE_LATEST_URL: &str =
    "https://developers.eveonline.com/static-data/tranquility/latest.jsonl";
pub(crate) const ARCHIVE_URL: &str =
    "https://developers.eveonline.com/static-data/tranquility/eve-online-static-data-";
pub(crate) const ARCHIVE_FILE: &str = "sde.jsonl.zip";
pub(crate) const ROOT_DIR: &str = "sde/";
const MAX_RETRY: usize = 10;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Build {
    #[serde(rename = "_key")]
    pub(crate) id: String,
    #[serde(rename = "buildNumber")]
    pub(crate) number: values::BuildNumber,
    pub(crate) release_date: String,
}

#[derive(Default, Debug)]
pub(crate) struct SDE {
    pub(crate) build: Build,
    pub(crate) map: data::map::Map,
    pub(crate) entity: data::entity::Entity,
    pub(crate) industry: data::industry::Industry,
}

pub(crate) fn is_out_of_date(ccp_build: &Build) -> Result<bool> {
    let local_build: Option<Build> = reader::get_sde_build()?;
    println!(
        "CCP Build version '{}' released '{}'",
        ccp_build.number, ccp_build.release_date
    );
    if let Some(local_build) = local_build {
        println!(
            "Local Build version '{}' released '{}'",
            local_build.number, local_build.release_date
        );
        if local_build.number != ccp_build.number {
            return Ok(true);
        }
    } else {
        println!("No local Build found");
        return Ok(true);
    }

    Ok(false)
}

pub(crate) fn fetch_build() -> Result<Build> {
    print!("Fetching SDE Build information...");
    std::io::stdout().flush().context("flushing stdout")?;

    let response: Response = fetch_with_retry(SDE_LATEST_URL)?;
    let jsonl: String = response.text()?;
    match jsonl.lines().next() {
        Some(json) => {
            let sde: Build = from_str(json)
                .with_context(|| format!("parsing json '{}' into sde::Build", &json))?;
            println!(" {}", config::SUCCESS_SYMBOL);
            return Ok(sde);
        }
        None => {
            println!(" {}", config::ERROR_SYMBOL);
            bail!("empty jsonl '{}' received from '{}'", jsonl, SDE_LATEST_URL);
        }
    }
}

pub(crate) fn download(ccp_sde: &Build, tmp_dir: &TempDir) -> Result<()> {
    print!(
        "Downloading SDE version {} released {}...",
        ccp_sde.number, ccp_sde.release_date
    );
    std::io::stdout().flush().context("flushing stdout")?;

    let mut archive_url: String = String::from(ARCHIVE_URL);
    archive_url.push_str(&ccp_sde.number.to_string());
    archive_url.push_str("-jsonl.zip");
    let archive_path: PathBuf = tmp_dir.path().join(ARCHIVE_FILE);
    let response: Response = fetch_with_retry(&archive_url)?;
    let data = response.bytes()?;

    write(archive_path, data)?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(())
}

fn fetch_with_retry(url: &str) -> Result<Response> {
    let http: Client = Client::builder().user_agent(config::USER_AGENT).build()?;
    let mut response: Response = http.get(url).send()?;
    let mut retry: usize = 0;
    while !response.status().is_success() {
        if retry >= MAX_RETRY {
            bail!("Tried '{}' times without success... CRASHING", retry);
        }
        retry += 1;
        if response.status().is_client_error() {
            println!(" {}", config::ERROR_SYMBOL);
            bail!("received '{}' when fetching '{}'", response.status(), url);
        } else {
            eprintln!(
                "received '{}' when fetching '{}'\nRetry in 5 seconds.",
                response.status(),
                url
            );
            thread::sleep(Duration::from_secs(5));
            response = http.get(url).send()?;
        }
    }

    Ok(response)
}

pub(crate) fn extract(tmp_dir: &TempDir) -> Result<PathBuf> {
    print!("Extracting SDE...");
    std::io::stdout().flush().context("flushing stdout")?;
    let archive: PathBuf = tmp_dir.path().join(ARCHIVE_FILE);
    let sde: PathBuf = tmp_dir.path().join(ROOT_DIR);
    zip_extract::zip_extract(&archive, &sde)?;
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(sde)
}

pub(crate) fn load_sde_data<T: DeserializeOwned>(
    tmp_dir: &TempDir,
    filename: &str,
) -> Result<Vec<T>> {
    load_jsonl(tmp_dir, filename)
        .with_context(|| format!("parsing SDE file '{}'", filename))
        .inspect_err(|_| println!(" {}", config::ERROR_SYMBOL))
}

fn load_jsonl<T: DeserializeOwned>(tmp_dir: &TempDir, filename: &str) -> Result<Vec<T>> {
    let jsonl_path: PathBuf = tmp_dir.path().join(format!("{}{}", ROOT_DIR, filename));
    let jsonl_file: File =
        File::open(&jsonl_path).with_context(|| format!("opening '{}'", jsonl_path.display()))?;
    let file_reader: BufReader<File> = BufReader::new(jsonl_file);

    let mut vec: Vec<T> = Vec::new();
    for (index, line) in file_reader.lines().enumerate() {
        let line_index = index + 1; // human-readable
        print!("\rParsing line {} from {}", line_index, filename);
        std::io::stdout().flush().context("flushing stdout")?;

        let line: String = line.with_context(|| format!("reading line '{}'", line_index))?;
        let entry: T = from_str(&line).with_context(|| format!("parsing line '{}'", line_index))?;
        vec.push(entry);
    }
    println!(" {}", config::SUCCESS_SYMBOL);

    Ok(vec)
}
