extern crate chrono;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate roman;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate strum;
#[macro_use]
extern crate strum_macros;

use std::collections::HashMap;

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use minecraft::{parse_color_codes, strip_color_codes};

pub mod minecraft;
pub mod report;

/// process_report(raw_json_report, /)
/// --
///
/// This method takes a raw JSON report as string, and returns a dict containing the following keys:
/// - "processed_report": the processed JSON report (with everything pre-calculated and ready to be
///   displayed) as a string, so it is ready to be stored without parsing and loading the big JSON;
/// - "match_uuid": the report UUID extracted from the received report;
/// - "minecraft_version": the Minecraft version extracted from the received report (may be absent);
/// - "generator_name": the generator name extracted from the received report (may be absent);
/// - "generator_link": the generator link extracted from the received report (may be absent).
///
/// If the JSON is invalid or does not complies to the report format, a ValueError will be raised.
/// If for some reason the processed report cannot be converted back to a JSON string, a
/// RuntimeError will be raised, but this should never happens (except if there is a bug in the
/// Rust implementation).
#[pyfunction]
fn process_report(raw_json_report: String) -> PyResult<HashMap<String, String>> {
    match serde_json::from_str(raw_json_report.as_str()) {
        Ok(raw_report) => match report::report::Report::from_raw(raw_report) {
            Ok(report) => match serde_json::to_string(&report) {
                Ok(json_report) => {
                    let mut report_return = HashMap::new();

                    report_return.insert("processed_report".to_string(), json_report);
                    report_return.insert(
                        "match_uuid".to_string(),
                        report.match_uuid.to_hyphenated().to_string(),
                    );
                    report_return.insert("title".to_string(), strip_color_codes(report.title));

                    match report.minecraft {
                        Some(minecraft) => {
                            report_return.insert("minecraft_version".to_string(), minecraft);
                        }
                        None => {}
                    };

                    match report.settings.generator {
                        Some(generator) => {
                            report_return.insert("generator_name".to_string(), generator.name);

                            match generator.link {
                                Some(link) => {
                                    report_return.insert("generator_link".to_string(), link);
                                }
                                None => {}
                            }
                        }
                        None => {}
                    };

                    Ok(report_return)
                }
                Err(error) => Err(PyRuntimeError::new_err(format!(
                    "Unable to convert report to JSON string: {}",
                    error
                ))),
            },
            Err(error) => Err(PyValueError::new_err(format!("Invalid report: {}", error))),
        },
        Err(error) => Err(PyValueError::new_err(format!("Invalid JSON: {}", error))),
    }
}

/// parse_minecraft_color_codes(raw_string, /)
/// --
///
/// This method takes a raw string containing Minecraft formatting codes (e.g. "§2§lMy §6word!")
/// and returns a HTML version of this string with all codes converted.
///
/// >>> hawk_processing.parse_minecraft_color_codes("§2§lMy §6word!")
/// '<span style="color: #00AA00;"><span style="font-weight: bold;">My </span></span><span style="color: #FFAA00;">word!</span>'
#[pyfunction]
fn parse_minecraft_color_codes(raw_string: String) -> PyResult<String> {
    Ok(parse_color_codes(raw_string))
}

/// strip_minecraft_color_codes(raw_string, /)
/// --
///
/// This method takes a raw string containing Minecraft formatting codes (e.g. "§2§lMy §6word!")
/// and returns the same string without the color codes.
///
/// It will keep invalid formatting codes (e.g. “§W” will not be removed).
///
/// >>> hawk_processing.parse_minecraft_color_codes("§2§lMy §6word!")
/// 'My word!'
#[pyfunction]
fn strip_minecraft_color_codes(raw_string: String) -> PyResult<String> {
    Ok(strip_color_codes(raw_string))
}

/// to_roman(number, /)
/// --
///
/// This method converts a number (e.g. 421) into a roman numeral (e.g. "CDXXI").
/// If the number cannot be converted (i.e. if it is out of [1; 3999]), it returns the
/// number as a string (e.g. to_roman(4269) == "4269").
#[pyfunction]
fn to_roman(number: i32) -> PyResult<String> {
    Ok(match roman::to(number) {
        Some(roman_number) => roman_number,
        None => format!("{}", number),
    })
}

#[pymodule]
fn hawk_processing(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(process_report))?;
    m.add_wrapped(wrap_pyfunction!(parse_minecraft_color_codes))?;
    m.add_wrapped(wrap_pyfunction!(strip_minecraft_color_codes))?;
    m.add_wrapped(wrap_pyfunction!(to_roman))?;

    Ok(())
}
