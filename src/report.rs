extern crate serde;
extern crate serde_json;
extern crate failure;

#[macro_use] extern crate serde_derive;

pub mod report_raw;
pub mod serde_instant;

use std::io::Read;
use failure::Error;


pub fn read_report_raw<R: Read>(reader: R) -> Result<report_raw::Report, Error> {
    Ok(report_raw::read_report(reader)?)
}
