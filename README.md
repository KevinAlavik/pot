# Pot - Package Manager

Pot is a simple package manager written in Rust. It allows you to fetch and install packages from a remote JSON file.

## Installation

To install the Pot package manager, execute the following command in your terminal:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/KevinAlavik/pot/main/install.sh)"
```
Please ensure that you have git and cargo installed on your system before running the installation command.

## Usage
Pot provides two main commands: fetch and install.

## Fetch
The fetch command retrieves all available packages from a remote JSON file and prints their details.

To fetch and print the packages, use the following command:

```bash
pot fetch
```

## Install
The install command allows you to install a specific package by providing its name and version.

To install a package, use the following command:

```bash
pot install <package-name>@<version>
```
Replace <package-name> with the name of the package you want to install, and <version> with the desired version number.

For example, to install a package named example-package with version 1.0.0, run the following command:

```bash
pot install example-package@1.0.0
```
## Contributing
Contributions to Pot are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License
This project is licensed under the MIT License.
 
Please note that the installation command assumes it will be executed in a UNIX-like environment with `/bin/bash` available.
