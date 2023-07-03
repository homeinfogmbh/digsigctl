#![allow(clippy::let_underscore_untyped, clippy::no_effect_underscore_binding)]

use digsigctl::{get_config, Config};
use rocket::serde::json::Json;
use rocket::{get, launch, post, routes, Build, Rocket};
use std::process::exit;

#[post("/configure", format = "application/json", data = "<config>")]
#[allow(clippy::needless_pass_by_value)]
fn configure(config: Json<Config>) -> String {
    config.url().to_string()
}

#[get("/sysinfo", format = "application/json")]
fn sysinfo() -> String {
    "Hello world.".into()
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
