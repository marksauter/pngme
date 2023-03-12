use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use chunk::Chunk;
use chunk_type::ChunkType;

use crate::png::Png;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let matches = args::cli().get_matches();

    match matches.subcommand() {
        Some(("encode", sub_matches)) => {
            let path = PathBuf::from_str(sub_matches.get_one::<String>("PATH").expect("required"))?;

            let contents = fs::read(&path)?;
            let mut png = Png::try_from(contents.as_slice())?;

            let chunk_type = ChunkType::from_str(
                sub_matches
                    .get_one::<String>("CHUNK_TYPE")
                    .expect("required"),
            )?;
            let message = sub_matches.get_one::<String>("MESSAGE").expect("required");
            let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

            png.append_chunk(chunk);

            fs::write(&path, png.as_bytes())?;

            Ok(())
        }
        Some(("decode", sub_matches)) => {
            let path = PathBuf::from_str(sub_matches.get_one::<String>("PATH").expect("required"))?;
            let contents = fs::read(&path)?;
            let png = Png::try_from(contents.as_slice())?;

            let chunk_type = ChunkType::from_str(
                sub_matches
                    .get_one::<String>("CHUNK_TYPE")
                    .expect("required"),
            )?;

            let e: Box<dyn std::error::Error> = String::from("no message found").into();
            let message = png
                .chunks()
                .iter()
                .find(|c| c.chunk_type() == &chunk_type)
                .ok_or(e)?
                .data_as_string()?;

            println!("Message: {}", message);
            Ok(())
        }
        Some(("remove", sub_matches)) => {
            let path = PathBuf::from_str(sub_matches.get_one::<String>("PATH").expect("required"))?;
            let contents = fs::read(&path)?;
            let mut png = Png::try_from(contents.as_slice())?;

            let chunk_type = sub_matches
                .get_one::<String>("CHUNK_TYPE")
                .expect("required");

            png.remove_chunk(chunk_type)?;

            fs::write(&path, png.as_bytes())?;

            Ok(())
        }
        Some(("print", sub_matches)) => {
            let path = PathBuf::from_str(sub_matches.get_one::<String>("PATH").expect("required"))?;
            let contents = fs::read(&path)?;
            let png = Png::try_from(contents.as_slice())?;

            let chunks: Vec<&Chunk> = png
                .chunks()
                .iter()
                .filter(|c| !c.chunk_type().is_public())
                .collect();

            println!("Chunks: {:#?}", chunks);

            Ok(())
        }
        _ => Ok(()),
    }
}
