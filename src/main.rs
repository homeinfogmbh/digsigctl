#![allow(clippy::let_underscore_untyped, clippy::no_effect_underscore_binding)]

use digsigctl::get_config;
use rocket::{get, launch, routes, Build, Rocket};
use std::process::exit;

#[get("/")]
fn index() -> String {
    "Hello world.".into()
}

#[launch]
fn rocket() -> Rocket<Build> {
    match get_config() {
        Ok(config) => rocket::custom(config).mount("/", routes![index]),
        Err(error) => {
            eprintln!("{error}");
            exit(1);
        }
    }
}
