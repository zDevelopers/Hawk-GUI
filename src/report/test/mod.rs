extern crate insta;

use std::fs;
use std::path::Path;

use insta::*;

use crate::*;
use crate::report::errors::ReportResult;
use crate::report::report::Report;

fn process_from_path(path: &Path) -> ReportResult<Report> {
    match fs::read_to_string(path) {
        Ok(input) => match serde_json::from_str(input.as_str()) {
            Ok(raw_report) => report::report::Report::from_raw(raw_report),
            Err(e) => panic!("Unable to parse raw report: {}", e)
        },
        Err(e) => panic!("Unable to read input report test file: {}", e)
    }
}

fn process(input: &'static str) -> ReportResult<Report> {
    process_from_path(Path::new(format!("./src/report/test/inputs/{}.json", input).as_str()))
}

fn process_to_string(input: &'static str) -> String {
    match process(input) {
        Ok(report) => match serde_json::to_string_pretty(&report) {
            Ok(json_report) => json_report,
            Err(e) => panic!("Unable to convert processed report to json: {}", e)
        }
        Err(e) => panic!("Unable to process report: {}", e)
    }
}

fn assert_input_with_snapshot(input: &'static str) {
    assert_snapshot!(input, process_to_string(input));
}

fn assert_input_fails(input: &'static str, message: &'static str) {
    assert!(process(input).is_err(), message);
}

#[test]
fn test_empty_report() {
    assert_input_with_snapshot("empty");
}

#[test]
fn test_minecraft_version() {
    assert_input_with_snapshot("minecraft_version");
}

#[test]
fn test_generator() {
    assert_input_with_snapshot("generator");
    assert_input_with_snapshot("generator_without_link");
}

#[test]
#[should_panic(expected = "missing field `name`")]
#[allow(unused_must_use)]
fn test_generator_without_name() {
    process("generator_without_name");
}

#[test]
fn test_broken_player_reference_should_fail() {
    assert_input_fails("broken_player_link", "Processing must fail if there is a broken player reference");
}
