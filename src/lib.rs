extern crate failure;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::io::Read;

use failure::Error;

pub mod report_raw;
pub mod serde_instant;
pub mod minecraft;

pub fn read_report_raw<R: Read>(reader: R) -> Result<report_raw::Report, Error> {
    Ok(report_raw::read_report(reader)?)
}
