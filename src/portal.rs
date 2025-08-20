//! Portal API integration module for digital signage systems.
//!
//! This module provides functionality to interact with the portal API at
//! https://portal.homeinfo.de/test.html to fetch configuration URLs based on hostname
//! and verify if they match the current Chromium startup page.

use crate::config::Config;
use crate::rpc::default_preferences_file;
use anyhow::Result;
use rocket::serde::json::serde_json;
use serde::Deserialize;
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct PortalResponse {
    url: String,
}

/// Get the hostname of the current system
pub fn get_hostname() -> Result<String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("hostname").output()?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    #[cfg(target_os = "linux")]
    {
        Ok(fs::read_to_string("/etc/hostname")?.trim().to_string())
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        Err(anyhow::anyhow!("Unsupported operating system"))
    }
}

/// Fetch URL from portal API using hostname as parameter
pub async fn fetch_portal_url(hostname: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://termgr.homeinfo.de/administer/get-url/")
        .query(&[("hostname", hostname)])
        .send()
        .await?;

    let portal_response: PortalResponse = response.json().await?;
    Ok(portal_response.url)
}

/// Check if the portal URL matches the current Chromium startup page
pub async fn verify_startup_page() -> Result<bool> {
    let hostname = get_hostname()?;
    let portal_url = fetch_portal_url(&hostname).await?;

    // Get the current Chromium startup URL from preferences
    let startup_url = get_current_startup_url()?;

    Ok(portal_url == startup_url)
}

/// Get the current Chromium startup URL from preferences
fn get_current_startup_url() -> Result<String> {
    let preferences_file = default_preferences_file()
        .ok_or_else(|| anyhow::anyhow!("Default preferences file not found"))?;

    if !preferences_file.exists() {
        return Ok(String::new());
    }

    let content = fs::read_to_string(preferences_file)?;
    let preferences: serde_json::Value = serde_json::from_str(&content)?;

    // Navigate to the startup URLs in the preferences
    if let Some(session) = preferences.get("session") {
        if let Some(startup_urls) = session.get("startup_urls") {
            if let Some(urls) = startup_urls.as_array() {
                if let Some(first_url) = urls.first() {
                    if let Some(url) = first_url.as_str() {
                        return Ok(url.to_string());
                    }
                }
            }
        }
    }

    Ok(String::new())
}

/// Apply portal configuration on startup only if needed
pub async fn apply_portal_config_if_needed() -> Result<bool> {
    let hostname = get_hostname()?;
    let portal_url = fetch_portal_url(&hostname).await?;

    // Get the current Chromium startup URL from preferences
    let startup_url = get_current_startup_url()?;

    // Only apply configuration if there's a mismatch
    if portal_url != startup_url {
        let config = Config::new(portal_url);
        config.apply()?;
        Ok(true) // Configuration was applied
    } else {
        Ok(false) // No configuration needed
    }
}

/// Apply portal configuration on startup
pub async fn apply_portal_config_on_startup() -> Result<()> {
    let hostname = get_hostname()?;
    let portal_url = fetch_portal_url(&hostname).await?;

    // Create a config with the portal URL and apply it
    let config = Config::new(portal_url);
    config.apply()?;

    Ok(())
}
