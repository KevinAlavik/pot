use reqwest;
use serde_json::Value;
#[allow(unused_imports)]
use colored::Colorize;
use std::time::Instant;
use clap::{App, Arg};

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
    let url = "http://keso.local/pot.json";

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        // Store JSON data in the 'packages' variable
        let packages = json.get("packages");

        // Start the timer
        let start_time = Instant::now();

        // Accessing and printing the value of each package's name
        if let Some(packages_array) = packages {
            for (i, package) in packages_array.as_array().unwrap().iter().enumerate() {
                if let Some(name) = package.get("name") {
                    println!("{} Package {}: {}", "Found".green(), i+1, name);
                }
            }
        }

        // Stop the timer and calculate the elapsed time
        let elapsed_time = start_time.elapsed();

        // Print the elapsed time
        println!("Execution time: {:.2?}", elapsed_time);
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}

async fn install_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://keso.local/pot.json";

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;

        // Find the requested package
        let package_info = json["packages"]
            .as_array()
            .and_then(|packages| {
                packages.iter().find(|pkg| {
                    pkg["name"].as_str() == Some(package.split('@').next().unwrap())
                })
            })
            .and_then(|pkg| {
                pkg["versions"]
                    .as_array()
                    .and_then(|versions| {
                        versions.iter().find(|ver| {
                            ver["versionNumber"].as_str() == Some(package.split('@').nth(1).unwrap())
                        })
                    })
            });

        if let Some(package_info) = package_info {
            if let Some(binary) = package_info["binary"].as_str() {
                println!("Binary URL: {}", binary);
            } else {
                println!("Binary URL not found for the specified package.");
            }
        } else {
            println!("Package not found");
        }
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
