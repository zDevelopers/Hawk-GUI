#![feature(proc_macro_hygiene, decl_macro)]

extern crate report;

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::serve::StaticFiles;

use report::report_raw::Report;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/publish",  data = "<match_report>")]
fn publish(match_report: Json<Report>) -> Json<Report> {
    match_report
}

#[get("/<match_id>")]
fn display_match(match_id: String) -> String {
    format!("Here will be the match {}", match_id)
}

#[get("/<match_id>/as_json")]
fn display_match_json(match_id: String) -> Option<JsonValue> {
    Some(json!({"match_uuid": "9be8ef14-a14e-4f96-b61e-b865c27ada8f", "date": "2018-12-16 11:08:26", "teams": []}))
}


#[catch(422)]
fn error_unprocessable_entity() -> JsonValue {
    json!({
        "error": "Unprocessable Entity",
        "error_code": 422,
        "description": "The request was well-formed, but we were unable to process it due to \
        semantic errors in the data provided."
    })
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, publish, display_match, display_match_json])
        .mount("/static", StaticFiles::from("/static"))
        .register(catchers!(error_unprocessable_entity))
        .launch();
}
