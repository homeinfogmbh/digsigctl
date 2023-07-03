#![allow(clippy::let_underscore_untyped, clippy::no_effect_underscore_binding)]

use digsigctl::{get_config, Config, SystemInformation};
use rocket::serde::json::Json;
use rocket::{get, launch, post, routes, Build, Rocket};
use std::process::exit;

#[allow(clippy::needless_pass_by_value)]
#[post("/configure", format = "application/json", data = "<config>")]
fn configure(config: Json<Config>) -> String {
    match config.apply() {
        Ok(_) => "Configuration applied.".to_string(),
        Err(error) => error.to_string(),
    }
}

#[get("/sysinfo", format = "application/json")]
fn sysinfo() -> Json<SystemInformation> {
    Json(SystemInformation::gather())
}

#[launch]
fn rocket() -> Rocket<Build> {
    match get_config() {
        Ok(config) => rocket::custom(config).mount("/", routes![configure, sysinfo]),
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    }
}
