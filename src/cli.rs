//! Cli args and commands for clap

use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};

/// Pastebin CLI commands
#[derive(Clone, Subcommand)]
pub(crate) enum PastebinCommand {
	/// Add a paste to the pastebin database
	Add {
		/// Paste content
		content: String,
	},
	/// Get a paste based ID or link
	Get {
		/// ID of paste
		#[arg(short, long)]
		id: Option<u64>,

		/// Link of paste
		#[arg(short, long)]
		link: Option<String>,
	},
	/// List pastes from pastebin database
	List {},
	/// Remove a paste
	Rm {
		/// Id of the paste to remove
		id: u64,
	},
	/// Serve pastebin server
	Serve {
		/// Port to serve pastebin server to
		#[arg(short, long)]
		port: Option<u16>,
	},
}

/// Pastebin CLI arguments
#[derive(Clone, Parser)]
pub(crate) struct Args {
	/// Command to be run
	#[command(subcommand)]
	pub(crate) command: PastebinCommand,

	/// Manually set the database file to be used
	#[arg(short, long)]
	pub(crate) database_file: Option<PathBuf>,

	/// Whether to init the pastebin database
	#[arg(short, long, action = ArgAction::SetTrue)]
	pub(crate) init: bool,

	/// Whether to force init the pastebin database (wipes old database)
	#[arg(short = 'I', long, action = ArgAction::SetTrue)]
	pub(crate) force_init: bool,

	/// Print version and exit
	#[arg(short = 'V', long, action = ArgAction::SetTrue)]
	pub(crate) version: bool,
}
