#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading: {}", err)))?;
    println!("Content: {}", content);
    Ok(())
}