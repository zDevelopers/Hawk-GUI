#![feature(proc_macro_hygiene, decl_macro)]

extern crate lib;
extern crate reqwest;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

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
use rocket_contrib::templates::tera::{Context};
use rocket_contrib::uuid::{Uuid, uuid_crate};

use lib::report::raw;
use lib::report::report;
use lib::report::{save_report, read_processed_report_from_slug};
use lib::tera as hawk_tera;

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
                "uri": format!("{base_uri}/{slug}", base_uri = &base_uri.uri, slug = &slug)
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

#[get("/<slug>")]
fn display_match(slug: String) -> Option<Template> {
    match read_processed_report_from_slug(slug.clone()) {
        Ok(report) => {
            let mut context = Context::new();
            context.insert("report", &report);
            context.insert("slug", &slug);

            Some(Template::render("report", context))
        },
        Err(_) => None
    }
}

#[get("/<match_id>/as_json")]
fn display_match_json(match_id: String) -> Option<Json<report::Report>> {
    match read_processed_report_from_slug(match_id) {
        Ok(report) => {
            let mut report = report;
            report.match_uuid = uuid_crate::Uuid::nil();

            Some(Json(report))
        },
        Err(_) => None
    }
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
            engines.tera.add_template_file(Path::new("templates/__macros__.html.tera"), Some("__macros__")).unwrap_or_else(|err| panic!(err));

            engines.tera.register_filter("css_class", hawk_tera::make_css_class_filter());
            engines.tera.register_filter("duration", hawk_tera::make_duration_filter());
            engines.tera.register_filter("minecraft", hawk_tera::make_minecraft_filter());
            engines.tera.register_filter("icon", hawk_tera::make_icon_filter());
            engines.tera.register_filter("name", hawk_tera::make_name_filter());
            engines.tera.register_filter("enchantment", hawk_tera::make_enchantment_filter());

            engines.tera.register_tester("creature", hawk_tera::is_creature_test);
            engines.tera.register_tester("curse", hawk_tera::is_curse_test);

            engines.tera.register_function("head", hawk_tera::make_head_function());
        }))
        .attach(AdHoc::on_attach("Base URI", |rocket| {
            let uri = rocket.config().get_string("public-base-uri").unwrap_or("http://127.0.0.1:8000".to_string()).clone();
            Ok(rocket.manage(BaseURI { uri }))
        }))
        .register(catchers!(error_unprocessable_entity))
        .launch();
}
