#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use reqwest;
use serde_json::Value;
use colored::Colorize;
use std::time::Instant;
use clap::{App, Arg};
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

#[tokio::test]
async fn test_fetch_json() {
    let result = fetch_json().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_install_package_valid() {
    let result = install_package("chrome-pie@0.0.1").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_install_package_invalid() {
    let result = install_package("invalid-package").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_install_package_missing_url() {
    let result = install_package("package2@1.0").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_fetch_json_failed_request() {
    let result = fetch_json().await;
    assert!(result.is_err());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("pot")
        .version("0.1.0")
        .author("Kevin Alavik")
        .about("Package manager")
        .subcommand(App::new("fetch").about("Fetches all available packages"))
        .subcommand(App::new("install").about("Install a package").arg(Arg::with_name("package").required(true).takes_value(true)))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("fetch") {
        fetch_json().await?;
    } else if let Some(matches) = matches.subcommand_matches("install") {
        if let Some(package) = matches.value_of("package") {
            install_package(package).await?;
        } else {
            println!("No package specified. Use 'pot install <package-name>@<version>' to install a package.");
        }
    } else {
        println!("No valid command specified. Use 'pot fetch' to fetch and print JSON data, or 'pot install <package-name>@<version>' to install a package.");
    }

    Ok(())
}

async fn fetch_json() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://puffer.is-a.dev/pot/pot.json";

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        // Store JSON data in the 'packages' variable
        let packages = json.get("packages");

        // Start the timer
        let start_time = Instant::now();

        // Accessing and printing the value of each package's name, versions, and exec-name
        if let Some(packages_array) = packages {
            for (i, package) in packages_array.as_array().unwrap().iter().enumerate() {
                if let Some(name) = package.get("name") {
                    if let Some(exec_name) = package.get("exec-name") {
                        let version_info = package.get("versions").and_then(|versions| {
                            versions.as_array().map(|ver| {
                                if ver.len() > 0 {
                                    let version_numbers = ver
                                        .iter()
                                        .map(|ver| format!("v{}", ver["versionNumber"].as_str().unwrap_or("N/A")))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    format!("{}", version_numbers)
                                } else {
                                    "N/A".to_string()
                                }
                            })
                        }).unwrap_or_else(|| "N/A".to_string());
                        let package_name = format!("📦 {} Package {}: {}", "Found".green(), i + 1, name);
                        
                        if !version_info.is_empty() {
                            let version_text = format!("{} {}", "└──".cyan(), version_info);
                            println!("{}\n{}", package_name, version_text);
                        } else {
                            println!("{}", package_name);
                        }
                    } else {
                        println!("📦 {} Package {}: {}", "Found".green(), i + 1, name);
                    }
                }
            }
        }

        let elapsed_time = start_time.elapsed();
        println!("Fetching packages took: {:?}", elapsed_time);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}

async fn install_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://puffer.is-a.dev/pot/pot.json";
    println!("{} package list...", "Fetching".green());
    let response = reqwest::get(url).await?;
    
    if response.status().is_success() {
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        let package_info = json["packages"]
            .as_array()
            .and_then(|packages| {
                packages.iter().find(|pkg| {
                    pkg["name"].as_str() == Some(package.split('@').next().unwrap())
                })
            })
            .and_then(|pkg| {
                let version = package.split('@').nth(1);
                if let Some(version) = version {
                    pkg["versions"]
                        .as_array()
                        .and_then(|versions| {
                            versions.iter().find(|ver| {
                                ver["versionNumber"].as_str() == Some(version)
                            })
                        })
                } else {
                    pkg["versions"]
                        .as_array()
                        .and_then(|versions| {
                            versions.iter().find(|ver| {
                                ver["latest"].as_bool() == Some(true)
                            })
                        })
                }
            });

        if let Some(package_info) = package_info {
            if let Some(binary) = package_info["binary"].as_str() {
                let response = reqwest::get(binary).await?;
                let package_name = package.split('@').next().unwrap();
                let exec_name = package_info["exec-name"].as_str().unwrap_or(package_name);
                let filename = format!("/usr/bin/{}", exec_name);

                let content = response.bytes().await?;
                fs::write(&filename, &content)?;
                println!("{} {}...", "Downloading".green(), package);
                // Set executable permissions
                let path = Path::new(&filename);
                let mut permissions = fs::metadata(path)?.permissions();
                let mode = permissions.mode();
                permissions.set_mode(mode | 0o111); // Add executable permissions
                fs::set_permissions(path, permissions)?;
                let output = Command::new("sudo")
                    .arg("chmod")
                    .arg("a+x")
                    .arg(&filename)
                    .output()?;
                if !output.status.success() {
                    return Err(format!("Failed to set executable permissions for {}: {:?}", filename, output).into());
                }

                println!("📦 {} has been installed (Path: {})", package_name, path.display());
            } else {
                return Err("Binary URL not found for the specified package.".into());
            }
        } else {
            return Err(format!("Package {} not found", package).into());
        }
    } else {
        return Err(format!("Request failed with status code: {}", response.status()).into());
    }

    Ok(())
}
