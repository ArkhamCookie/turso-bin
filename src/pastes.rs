//! For paste data

use crate::utils::random_link;

use serde::Serialize;
use turso::Connection;

/// Data for a paste
#[derive(Debug, Serialize)]
pub struct Paste {
	pub id: i64,
	pub link: String,
	pub paste_content: String,
	pub timestamp: String,
}

impl Paste {
	/// Create a new paste
	pub async fn create_new(connection: &Connection, paste_content: String) -> Result<Self, turso::Error> {
		let mut link = random_link();

		let mut rows = connection.query("SELECT link FROM pastebin", ()).await?;

		// TODO: Improve checking for existing link
		while let Some(row) = rows.next().await? {
			let existing_link = row.get_value(0)?;

			let existing_link = existing_link
				.as_text()
				.expect("unable to get existing link from database");

			while existing_link == &link {
				link = random_link()
			}
		}

		let added = connection.execute(
			"INSERT INTO pastebin (link, content) VALUES (?1, ?2)",
			[link.clone(), paste_content.clone()],
		).await;

		if let Err(error) = added {
			eprintln!("Failed to create paste: {:?}", error);
			todo!("handle errors better")
		}

		let mut statement = connection
			.prepare("SELECT * FROM pastebin WHERE link = ?1")
			.await?;
		let paste_query = statement.query_row([link.clone()]).await?;

		let id = paste_query.get_value(0)?;
		let id = id.as_integer().expect("unable to get id from value");
		let timestamp = paste_query
			.get_value(3)?
			.as_text()
			.expect("unable to get timestamp from value")
			.to_string();

		Ok(Self {
			id: *id,
			link,
			paste_content,
			timestamp,
		})
	}

	pub async fn remove(connection: &Connection, id: i64) -> Result<(), turso::Error> {
		let sql_statement = format!("DELETE FROM pastebin WHERE id = {}", id);

		connection.execute(
			sql_statement,
		()).await?;

		Ok(())
	}
}

/// A collection of pastes
#[derive(Debug, Serialize)]
pub struct Pastes {
	/// Pastes
	pub pastes: Vec<Paste>,
}

impl Pastes {
	/// Fetch all pastes from the database
	pub async fn fetch(connection: &Connection) -> Result<Self, turso::Error> {
		let mut pastes: Vec<Paste> = vec![];

		let mut rows = connection.query("SELECT * FROM pastebin", ()).await?;

		while let Ok(row) = rows.next().await {
			if row.is_none() {
				break;
			}

			let paste = row.unwrap();

			let id = paste.get_value(0)?;
			let id = id.as_integer().expect("unable to get id from value");
			let link = paste.get_value(1)?.as_text().expect("unable to get link from value").to_string();
			let paste_content = paste.get_value(2)?.as_text().expect("unable to get paste_content from value").to_string();
			let timestamp = paste.get_value(3)?.as_text().expect("unable to get timestamp from value").to_string();

			let parsed_paste = Paste {
				id: *id,
				link,
				paste_content,
				timestamp,
			};

			pastes.push(parsed_paste);
		};

		Ok(Self {
			pastes,
		})
	}
}
