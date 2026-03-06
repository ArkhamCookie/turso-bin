//! Main utils for pastebin

use rand::RngExt;

/// Create a random link for a pastebin entry
pub(crate) fn random_link() -> String {
	const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	let mut rng = rand::rng();

	let link: String = (0..5)
		.map(|_| {
			let index = rng.random_range(0..6);
			CHARSET[index] as char
		})
		.collect();

	return link;
}
