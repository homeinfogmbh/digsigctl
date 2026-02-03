    //! Digital signage system controller.
//!
//! This program is intended to run on digital signage systems and act as an RPC server.

use clap::Parser;
use digsigctl::{
    discover_address_or_exit, take_screenshot, Command, Config, CONFIGURATION_MODE, is_active,
    Result, ScreenshotResponse, SystemInformation, apply_portal_config_if_needed, verify_startup_page,
};
use rocket::serde::json::Json;
use rocket::{get, launch, post, routes, Build, Rocket};
use std::thread;
use subprocess::ExitStatus;
use tokio::runtime::Runtime;

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

    // Run portal verification on startup in a separate thread
    // Only apply configuration if the portal URL doesn't match the current startup page
    // Skip this if operation mode is CONFIGURATION_MODE
    thread::spawn(|| {
        // Check if CONFIGURATION_MODE is active
        let is_config_mode = match is_active(CONFIGURATION_MODE) {
            Ok(ExitStatus::Exited(0)) => true,
            _ => false,
        };

        if is_config_mode {
            eprintln!("Skipping portal verification - system is in configuration mode");
            return;
        }

        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            match verify_startup_page().await {
                Ok(matches) => {
                    if !matches {
                        // Only apply configuration if there's a mismatch
                        match apply_portal_config_if_needed().await {
                            Ok(applied) => {
                                if applied {
                                    eprintln!("Portal configuration applied on startup - URL mismatch detected");
                                } else {
                                    eprintln!("Portal configuration not needed - URL already matches");
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to apply portal config on startup: {}", e);
                            }
                        }
                    } else {
                        eprintln!("Portal configuration not needed - URL already matches");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to verify startup page on startup: {}", e);
                }
            }
        });
    });

    #[allow(clippy::redundant_type_annotations)]
    rocket::custom(
        rocket::Config::figment()
            .merge(("port", args.port))
            .merge(("address", discover_address_or_exit(args.network.as_str()))),
    )
    .mount("/", routes![configure, screenshot, sysinfo, rpc, verify_portal, get_portal_url])
}

#[allow(clippy::needless_pass_by_value)]
#[post("/configure", format = "application/json", data = "<config>")]
fn configure(config: Json<Config>) -> String {
    match config.apply() {
        Ok(()) => "Configuration applied.".to_string(),
        Err(error) => error.to_string(),
    }
}

#[get("/screenshot")]
fn screenshot() -> ScreenshotResponse {
    take_screenshot().into()
}

#[get("/sysinfo", format = "application/json")]
fn sysinfo() -> Json<SystemInformation> {
    Json(SystemInformation::default())
}

#[allow(clippy::needless_pass_by_value)]
#[post("/rpc", format = "application/json", data = "<command>")]
fn rpc(command: Json<Command>) -> Result {
    command.run()
}

/// Verify if the portal URL matches the current Chromium startup page
#[get("/verify-portal")]
async fn verify_portal() -> String {
    match verify_startup_page().await {
        Ok(matches) => {
            if matches {
                "Portal URL matches Chromium startup page".to_string()
            } else {
                "Portal URL does not match Chromium startup page".to_string()
            }
        }
        Err(e) => format!("Error verifying portal: {}", e),
    }
}

/// Get the current portal URL for the hostname
#[get("/portal-url")]
async fn get_portal_url() -> String {
    match digsigctl::portal::get_hostname() {
        Ok(hostname) => {
            match digsigctl::portal::fetch_portal_url(&hostname).await {
                Ok(url) => url,
                Err(e) => format!("Error fetching portal URL: {}", e),
            }
        }
        Err(e) => format!("Error getting hostname: {}", e),
    }
}
