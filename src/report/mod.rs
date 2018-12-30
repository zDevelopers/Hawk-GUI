use std::io::Read;

use failure::{Error, ResultExt};
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

///
/// Reads a raw JSON report to the raw Rust structure.
///
pub fn read_raw_report<R: Read>(reader: R) -> Result<raw::Report, Error> {
    Ok(serde_json::from_reader(reader).context("Unable to parse raw report JSON")?)
}
