use reqwest;
use serde_json::Value;
use colored::Colorize;
use std::time::Instant;
use clap::{App, Arg};
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use libc::mode_t;
use libc::chmod;
use std::ffi::CString;

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
    let url = "https://5500-kevinalavik-pot-ex225ne9166.ws-eu97.gitpod.io/pot.json";

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
                    println!("📦 {} Package {}: {}", "Found".green(), i+1, name);
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
    let url = "https://5500-kevinalavik-pot-ex225ne9166.ws-eu97.gitpod.io/pot.json";

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
                let response = reqwest::get(binary).await?;
                let package_name = package.split('@').next().unwrap();
                let filename = format!("/usr/local/bin/{}", package_name);
            
                let content = response.bytes().await?;
                fs::write(&filename, &content)?;
            
                let path = Path::new(&filename);
                fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;
                let permissions = (libc::S_IRUSR | libc::S_IWUSR | libc::S_IXUSR) as mode_t;
                let path_str = path.to_str().ok_or("Invalid path")?;
                let c_path = CString::new(path_str)?;
                let path_bytes = c_path.as_bytes_with_nul();
                let path_ptr = path_bytes.as_ptr() as *const i8;
                let result = unsafe { chmod(path_ptr, permissions) };
                if result != 0 {
                    return Err(std::io::Error::last_os_error().into());
                }
                println!("Package {} has been installed at {}", package_name, path.display());
            } else {
                println!("Binary URL not found for the specified package.");
            }
            
        } else {
            let package_name = package.split('@').next().unwrap();
            let package_version = package.split('@').nth(1).unwrap();
            let available_versions = json["packages"]
                .as_array()
                .and_then(|packages| {
                    packages.iter().find(|pkg| {
                        pkg["name"].as_str() == Some(package_name)
                    })
                })
                .and_then(|pkg| {
                    pkg["versions"]
                        .as_array()
                        .map(|versions| {
                            versions
                                .iter()
                                .map(|ver| ver["versionNumber"].as_str().unwrap_or(""))
                                .collect::<Vec<_>>()
                        })
                });

            println!("{} {} {} not found", "Package".red(), package_name, format!("version {}", package_version).red());
            
            if let Some(versions) = available_versions {
                let selected_version = format!("{} {}", package_name, package_version);
                for version in versions {
                    let formatted_version = format!("{} {}", package_name, version);
                    let arrow = if formatted_version == selected_version {
                        "->".green().bold()
                    } else {
                        "  ".dimmed()
                    };
                    println!("{} {} {}", arrow, formatted_version, "✅".green());
                }
            } else {
                println!("No available versions found for the package.");
            }
        }
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    Ok(())
}
