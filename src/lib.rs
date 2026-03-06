//! A [pastebin service](https://wikipedia.org/wiki/Pastebin) using [Turso](https://turso.tech/) as a database and experimenting with different web backends

/// The different web backends you can use for pastebin
pub mod backends;

/// For paste data and managing the database
pub mod pastes;

/// Main utils for pastebin
pub mod utils;
