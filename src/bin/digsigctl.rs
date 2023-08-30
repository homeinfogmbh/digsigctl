#![allow(clippy::let_underscore_untyped, clippy::no_effect_underscore_binding)]

use clap::Parser;
use digsigctl::{
    discover_address_or_exit, take_screenshot, Command, Config, Result, ScreenshotResponse,
    SystemInformation,
};
use rocket::serde::json::Json;
use rocket::{get, launch, post, routes, Build, Rocket};

#[derive(Parser)]
#[clap(about, author, version)]
struct Args {
    #[clap(short, long, default_value = "fd56:1dda:8794:cb90::/64")]
    network: String,

    #[clap(short, long, default_value_t = 5000)]
    port: u16,
}

#[launch]
fn rocket() -> Rocket<Build> {
    let args = Args::parse();

    rocket::custom(
        rocket::Config::figment()
            .merge(("port", args.port))
            .merge(("address", discover_address_or_exit(args.network.as_str()))),
    )
    .mount("/", routes![configure, screenshot, sysinfo, rpc])
}

#[allow(clippy::needless_pass_by_value)]
#[post("/configure", format = "application/json", data = "<config>")]
fn configure(config: Json<Config>) -> String {
    match config.apply() {
        Ok(_) => "Configuration applied.".to_string(),
        Err(error) => error.to_string(),
    }
}

#[get("/screenshot")]
fn screenshot() -> ScreenshotResponse {
    take_screenshot().into()
}

#[get("/sysinfo", format = "application/json")]
fn sysinfo() -> Json<SystemInformation> {
    Json(SystemInformation::gather())
}

#[allow(clippy::needless_pass_by_value)]
#[post("/rpc", format = "application/json", data = "<command>")]
fn rpc(command: Json<Command>) -> Result {
    command.run()
}
