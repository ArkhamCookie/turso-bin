use crate::cli::{Args, PastebinCommand};

use turso_bin::pastes::{Paste, Pastes};

use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::exit;

use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};

use tokio::net::TcpListener;

use turso::Builder;

#[cfg(feature = "axum")]
use turso_bin::backends::axum::version;

#[cfg(feature = "axum")]
use axum::{
	routing::get,
	Router,
};

#[cfg(feature = "hyper")]
use turso_bin::backends::hyper::hello;
#[cfg(feature = "hyper")]
use hyper::{server::conn::http1, service::service_fn};
#[cfg(feature = "hyper")]
use hyper_util::rt::TokioIo;

/// Cli args and commands for clap
pub(crate) mod cli;

#[cfg(feature = "hyper")]
/// Send a shutdown signal using tokio
async fn shutdown_signal() {
	tokio::signal::ctrl_c()
		.await
		.expect("failed to install CTRL+C signal handler");
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

	// TODO: Get default database file from TOML config?
	const DEFAULT_DATABASE_FILE: &str = "sqlite.db";
	let mut path = PathBuf::from(DEFAULT_DATABASE_FILE);

	if args.database_file.is_some() {
		path = args
			.database_file
			.expect("failed to get given database file");
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
			.execute(
				"DROP TABLE pastebin",
			()
		)
		.await
		.expect("unable to drop database");

		connection
			.execute("CREATE TABLE pastebin (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			link TEXT,
			content TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
			)",
			()
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
		PastebinCommand::Rm {
			id
		} => {
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
			}

			let address = SocketAddr::from(([127, 0, 0, 1], serve_port));
			let listener = TcpListener::bind(address).await.unwrap();

			println!("running on http://{}", address);

			#[cfg(feature = "axum")]
			let app: Router<()> = Router::new()
				.route("/", get(version))
				.route("/version", get(version));
				// .route("/pastes", get());

			#[cfg(feature = "axum")]
			axum::serve(listener, app).await.unwrap();

			#[cfg(feature = "hyper")]
			let http = http1::Builder::new();
			#[cfg(feature = "hyper")]
			let graceful = hyper_util::server::graceful::GracefulShutdown::new();
			#[cfg(feature = "hyper")]
			let mut signal = std::pin::pin!(shutdown_signal());

			#[cfg(feature = "hyper")]
			loop {
				tokio::select! {
					Ok((stream, _address)) = listener.accept() => {
						let io = TokioIo::new(stream);
						let connection = http.serve_connection(io, service_fn(hello));

						let future = graceful.watch(connection);
						tokio::spawn(async move {
							if let Err(error) = future.await {
								eprintln!("Error serving connectio: {:?}", error);
							}
						});
					},
					_ = &mut signal => {
						drop(listener);
						println!("\ngraceful shutdown signal received");
						break;
					}
				}
			}

			#[cfg(feature = "hyper")]
			tokio::select! {
				_ = graceful.shutdown() => {
					println!("all connections gracefully closed");
				},
				_ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
					println!("timed out wait for all connections to close")
				}
			}
		}
	}

	unimplemented!("please select a database using cargo features")
}
