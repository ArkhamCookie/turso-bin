//! Config for pastebin

use crate::cli::Args;

use std::fs;
use std::path::PathBuf;
use std::process::exit;

use directories::ProjectDirs;

use serde::Deserialize;

/// turso-bin config struct
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Config {
	pub(crate) database_file: Option<PathBuf>,
	pub(crate) port: Option<u16>,
}

/// Get config from file
fn get_config_from_file(config_file: &PathBuf) -> Config {
	let toml_string = match fs::read_to_string(config_file) {
		Ok(toml_string) => toml_string,
		Err(error) => {
			eprintln!("ERROR: {}", error);
			exit(1)
		}
	};

	match toml::from_str(&toml_string) {
		Ok(config) => config,
		Err(error) => {
			eprintln!("ERROR: {}", error);
			exit(1)
		}
	}
}

/// Get config from config directory file (creates one if needed)
fn get_config_from_dirs() -> Config {
	let project_dirs = match ProjectDirs::from("com", "arkhamcookie", "turso_bin") {
		Some(project_dirs) => project_dirs,
		None => {
			eprintln!("ERROR: Project directories not found (ProjectDirs crate issue)");
			exit(1)
		}
	};

	let config_dirs = project_dirs.config_dir();

	let _ = match fs::create_dir_all(config_dirs) {
		Ok(result) => result,
		Err(error) => {
			eprintln!("ERROR: {}", error);
			exit(1)
		}
	};

	let config_path = config_dirs.join("config.toml");

	let default_toml = String::from(
		"database_file = \"sqlite.db\"
port = 8080",
	);

	if !config_path.exists() {
		let _ = match fs::write(&config_path, default_toml) {
			Ok(result) => result,
			Err(error) => {
				eprintln!("ERROR: {}", error);
				exit(1)
			}
		};
	}

	get_config_from_file(&config_path)
}

/// Get config for pastebin
pub(crate) fn get_config(args: &Args) -> Config {
	if let Some(config_file) = &args.config {
		return get_config_from_file(&config_file);
	}

	get_config_from_dirs()
}
