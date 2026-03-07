use crate::cli::{Args, PastebinCommand};
use crate::config::get_config;

use turso_bin::pastes::{Paste, Pastes};

use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::exit;

use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};

use tokio::net::TcpListener;

use turso::Builder;

#[cfg(feature = "axum")]
use axum::{Router, routing::get};
#[cfg(feature = "axum")]
use std::sync::Arc;
#[cfg(feature = "axum")]
use tokio::sync::RwLock;
#[cfg(feature = "axum")]
use turso_bin::backends::axum::{
	AppState, get_paste_by_id, get_paste_by_link, get_pastes, version,
};

/// Cli args and commands for clap
pub(crate) mod cli;
/// Config for pastebin
pub(crate) mod config;

/// If fetching by id or link
enum GetBy {
	Id,
	Link,
}

#[tokio::main]
async fn main() {
	let args = Args::parse();

	if args.version {
		println!("{}: v{}", crate_name!(), crate_version!());
		println!("{}", crate_authors!());
		println!("\n{}", crate_description!());

		exit(0);
	}

	let config = get_config(&args);

	const DEFAULT_DATABASE_FILE: &str = "sqlite.db";
	let mut path = PathBuf::from(DEFAULT_DATABASE_FILE);

	if args.database_file.is_some() {
		path = args
			.database_file
			.expect("failed to get given database file");
	} else if config.database_file.is_some() {
		path = config.database_file.unwrap();
	}

	let database = Builder::new_local(path.to_str().expect("failed to convert path to str"))
		.build()
		.await
		.unwrap();
	let connection = database.connect().unwrap();

	if args.init {
		connection
			.execute(
				"CREATE TABLE IF NOT EXISTS pastebin (
				id INTEGER PRIMARY KEY AUTOINCREMENT,
				link TEXT,
				content TEXT,
				created_at DATETIME DEFAULT CURRENT_TIMESTAMP
				)",
				(),
			)
			.await
			.expect("unable to create database");
	}

	if args.force_init {
		connection
			.execute("DROP TABLE pastebin", ())
			.await
			.expect("unable to drop database");

		connection
			.execute(
				"CREATE TABLE pastebin (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			link TEXT,
			content TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
			)",
				(),
			)
			.await
			.expect("unable to create forced database");
	}

	match args.command {
		PastebinCommand::Add { content } => {
			let paste = Paste::create_new(&connection, content).await;

			if let Err(error) = paste {
				eprintln!("error creating paste: {:?}", error);
				exit(1)
			}

			let id = paste.unwrap().id;
			println!("paste {} added", id);

			exit(0)
		}
		PastebinCommand::Get { id, link } => {
			let paste: Result<Option<Paste>, turso::Error>;
			let get_by: GetBy;

			if let Some(id) = id {
				paste = Paste::get_by_id(&connection, id).await;
				get_by = GetBy::Id;
			} else if let Some(link) = link {
				paste = Paste::get_by_link(&connection, link).await;
				get_by = GetBy::Link;
			} else {
				eprintln!("ERROR: provide either link or id of paste");
				exit(2)
			}

			if let Err(error) = paste {
				eprintln!("error getting paste: {:?}", error);
				exit(1)
			}

			let paste = paste.unwrap();

			match get_by {
				GetBy::Id => {
					if paste.is_none() {
						println!("paste not found, no paste by that id");
						exit(0)
					}
				}
				GetBy::Link => {
					if paste.is_none() {
						println!("paste not found, no paste by that link");
						exit(0)
					}
				}
			}

			let paste = paste.unwrap();

			println!("{:?}", paste);

			exit(0)
		}
		PastebinCommand::List {} => {
			let pastes = Pastes::fetch(&connection).await;

			if let Err(error) = pastes {
				eprintln!("error fetching pastes: {:?}", error);
				exit(1)
			}

			let pastes = pastes.unwrap();

			for paste in pastes.pastes {
				println!("{:?}", paste);
			}

			exit(0)
		}
		PastebinCommand::Rm { id } => {
			let removed = Paste::remove(&connection, id).await;

			if let Err(error) = removed {
				eprintln!("error removing paste: {:?}", error);
				exit(1)
			}

			exit(0)
		}
		PastebinCommand::Serve { port } => {
			// TODO: Get default port from TOML config?
			let mut serve_port: u16 = 8080;
			if port.is_some() {
				serve_port = port.unwrap();
			} else if config.port.is_some() {
				serve_port = config.port.unwrap();
			}

			let address = SocketAddr::from(([127, 0, 0, 1], serve_port));
			#[allow(unused_variables)] // Used when databases are enabled
			let listener = TcpListener::bind(address).await.unwrap();

			println!("running on http://{}", address);

			#[cfg(feature = "axum")]
			let state = Arc::new(RwLock::new(AppState { connection }));

			#[cfg(feature = "axum")]
			let app: Router<()> = Router::new()
				.route("/", get(version))
				.route("/version", get(version))
				.route("/paste/by_id/{id}", get(get_paste_by_id))
				.route("/paste/by_link/{link}", get(get_paste_by_link))
				.route("/pastes", get(get_pastes))
				.with_state(state);

			#[cfg(feature = "axum")]
			axum::serve(listener, app).await.unwrap();
		}
	}

	unimplemented!("please select a database using cargo features")
}
