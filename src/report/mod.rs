use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::iter;
use std::path::Path;

use failure::{Error, ResultExt};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use serde_json;

pub mod raw;
pub mod settings;
pub mod player;
pub mod team;
pub mod damage;
pub mod heal;
pub mod event;
pub mod aggregates;
pub mod report;
pub mod errors;

static PROCESSED_REPORT_FILE_NAME: &'static str = "report.json";
static RAW_REPORT_FILE_NAME: &'static str = "raw-report.json";

///
/// Reads a raw JSON report to the raw Rust structure.
///
pub fn read_raw_report<R: Read>(reader: R) -> Result<raw::Report, Error> {
    Ok(serde_json::from_reader(reader).context("Unable to parse raw report JSON")?)
}

///
/// Reads a processed JSON report to the processed Rust structure.
///
pub fn read_processed_report<R: Read>(reader: R) -> Result<report::Report, Error> {
    Ok(serde_json::from_reader(reader).context("Unable to parse processed report JSON")?)
}

///
/// Reads a processed report from its slug into the processed Rust structure.
///
pub fn read_processed_report_from_slug(slug: String) -> Result<report::Report, Error> {
    let raw_path = &path_for_slug(slug);
    let path = Path::new(raw_path);
    let file = File::open(path.join(PROCESSED_REPORT_FILE_NAME)).with_context(|_| format!("Unable to open report at {}", raw_path))?;
    Ok(read_processed_report(file)?)
}

///
/// From a raw report structure, process and save both raw & processed JSON
/// into the file system. Returns the slug to use to access the report, or
/// the error.
///
pub fn save_report(raw_report: raw::Report) -> Result<String, Error> {
    let mut rng = thread_rng();

    let report_slug = loop {
        let report_slug: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(8).collect();

        if Path::new(&path_for_slug(report_slug.clone())).exists() {
            continue;
        }

        break report_slug
    };

    let root = path_for_slug(report_slug.clone());
    let root_path = Path::new(&root);
    let raw_path = root_path.join(RAW_REPORT_FILE_NAME);
    let processed_path = root_path.join(PROCESSED_REPORT_FILE_NAME);

    create_dir_all(root).expect("Unable to create directory to store the report");

    let mut raw_file = File::create(raw_path)?;
    raw_file.write_all(serde_json::to_string(&raw_report)?.as_bytes())?;

    let report = report::Report::from_raw(raw_report)?;

    let mut processed_file = File::create(processed_path)?;
    processed_file.write_all(serde_json::to_string(&report)?.as_bytes())?;

    Ok(report_slug)
}

fn path_for_slug<'a>(slug: String) -> String {
    format!(
        "{f}/reports/{prefix}/{slug}",
        f = crate::USERS_CONTENT_FOLDER,
        prefix = &slug[..2],
        slug = &slug
    )
}
