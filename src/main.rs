#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate error_chain;
extern crate lib;
extern crate reqwest;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::copy;
use std::path::Path;

use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera;
use rocket_contrib::uuid::{Uuid, uuid_crate};
use serde_json::value::{from_value, to_value, Value};

use lib::minecraft::parse_color_codes;
use lib::report::raw;
use lib::report::save_report;

use lib::USERS_CONTENT_FOLDER;

///
/// Stores the base URI for the current server according to configuration
/// (e.g. “https://domain.co/”).
///
struct BaseURI {
    ///
    /// The base URL, including protocol and ending with a slash.
    ///
    uri: String
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/publish",  data = "<match_report>")]
fn publish(match_report: Result<Json<raw::Report>, JsonError>, base_uri: State<BaseURI>) -> Result<status::Custom<JsonValue>, status::Custom<JsonValue>> {
    match match_report {
        Ok(match_report) => match save_report(match_report.into_inner()) {
            Ok(slug) => Ok(status::Custom(Status::Created, json!({
                "uri": format!("{base_uri}{slug}", base_uri = &base_uri.uri, slug = &slug)
            }))),
            Err(error) => Err(status::Custom(Status::UnprocessableEntity, json!({
                "error": "Unprocessable Entity",
                "error_code": format!("{}", error.as_ref()),
                "description": format!("{}", error)
            })))
        },

        Err(error) => {
            Err(status::Custom(Status::UnprocessableEntity, json!({
                "error": "Unprocessable Entity",
                "error_code": "JsonSchemaParseError",
                "description": match error {
                    JsonError::Io(error) => format!("IO Error: {}", error),
                    JsonError::Parse(_input, error) => format!("Parse Error: {}", error)
                }
            })))
        }
    }
}

#[get("/publish")]
fn publish_get() -> status::Custom<JsonValue> {
    status::Custom(Status::MethodNotAllowed, json!({
        "error": "This endpoint can only be used with HTTP POST requests.",
        "description": "POST to this URL a JSON file representing a match to get an online report page."
    }))
}

#[get("/head/<uuid>/<size>")]
fn get_head(uuid: Uuid, size: u8) -> Option<NamedFile> {
    let uuid_str = String::from(uuid.to_string());
    let path: String = format!(
        "{f}/heads/{prefix}/{uuid_str}-{size}.png",
        f = USERS_CONTENT_FOLDER, prefix = &uuid_str[..2], uuid_str = uuid_str, size = size
    );
    let path = Path::new(&path);

    match NamedFile::open(&path) {
        Ok(f) => Some(f),
        Err(_) => match reqwest::get(format!("https://crafatar.com/avatars/{}?overlay&size={}", uuid_str, size).as_str()) {
            Ok(mut response) => match create_dir_all(&path.parent().unwrap_or(Path::new(&"."))) {
                Ok(_) => match File::create(&path) {
                    Ok(mut file) => {
                        match copy(&mut response, &mut file) {
                            Ok(_) => match NamedFile::open(&path) {
                                Ok(f) => Some(f),
                                Err(e) => {
                                    println!("Error open after download: {}", e);
                                    None
                                }
                            }
                            Err(e) => {
                                println!("Error copy: {}", e);
                                None
                            }
                        }
                    },
                    Err(e) => {
                        println!("Error create file: {}", e);
                        None
                    }
                },
                Err(e) => {
                    println!("Error create dir: {}", e);
                    None
                }
            }
            Err(e) => {
                println!("Error download: {}", e);
                None
            }
        }
    }
}

#[get("/<match_id>")]
fn display_match(match_id: String) -> Template {
    let mut context = HashMap::new();
    context.insert("match_id", match_id);
    context.insert("match_title", String::from("§5§lKTZ §d§lVII"));

    Template::render("report", context)
}

#[get("/<match_id>/as_json")]
fn display_match_json(match_id: String) -> Option<JsonValue> {
    Some(json!({
        "match_uuid": "9be8ef14-a14e-4f96-b61e-b865c27ada8f",
        "match_url": uri!(display_match: match_id).to_string(),
        "date": "2018-12-16T11:08:26",
        "teams": []
    }))
}


#[catch(422)]
fn error_unprocessable_entity() -> JsonValue {
    json!({
        "error": "Unprocessable Entity",
        "error_code": "JsonSchemaError",
        "description": "The request was well-formed, but we were unable to process it due to \
        semantic errors in the data provided."
    })
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, publish, publish_get, get_head, display_match, display_match_json])
        .mount("/static", StaticFiles::from("static/dist"))
        .attach(Template::custom(|engines| {
            engines.tera.register_filter("minecraft", |input, _args| Ok(to_value(parse_color_codes(input.as_str().unwrap_or("").to_string())).unwrap()));
            engines.tera.register_function("head", Box::new(move |args| -> tera::Result<Value> {
                let uuid = match args.get("uuid") {
                    Some(uuid_str) => match from_value::<uuid_crate::Uuid>(uuid_str.clone()) {
                        Ok(uuid) => uuid,
                        Err(_) => bail!(
                            "Function `head` received uuid={} but `uuid` can only be a valid UUID",
                            uuid_str
                        )
                    },
                    None => bail!("Function `head` was called without a `uuid` argument")
                };

                let size = match args.get("size") {
                    Some(size) => match from_value::<u8>(size.clone()) {
                        Ok(size) => size,
                        Err(_) => bail!(
                            "Function `head` received size={} but `size` can only be an integer",
                            size
                        )
                    },
                    None => 16
                };

                // TODO Ok(to_value(uri!(get_head: uuid, size).to_string()).unwrap())
                Ok(to_value(format!("/head/{uuid}/{size}", uuid = uuid, size = size)).unwrap())
            }));
        }))
        .attach(AdHoc::on_attach("Base URI", |rocket| {
            let protocol = match rocket.config().get_bool("https").unwrap_or(false) {
                true => "https",
                false => "http"
            };

            let host = rocket.config().address.clone();
            let port = rocket.config().port;

            println!("Port: {:?}, port in config:: {:?}", &port, rocket.config().get_int("port"));

            Ok(rocket.manage(BaseURI {
                uri: format!("{protocol}://{host}{port}/", protocol = protocol, host = host, port = match port {
                    80 => String::new(),
                    _ => format!(":{}", port)
                })
            }))
        }))
        .register(catchers!(error_unprocessable_entity))
        .launch();
}
