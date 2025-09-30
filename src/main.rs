#![allow(unused_imports)]

use std::fmt::Display;
use std::fs;
use std::{path::Path, str::FromStr};
use log::{debug, info, warn, error};

use env_logger;
use clap::{Parser as ClapParser, ValueEnum};
use eyre::Result;

#[derive(ClapParser, Debug)]
#[clap(author, version = env!("GIT_DESCRIBE"), about, long_about = None)]
struct Args {
    #[clap(value_parser, help = "path to file or directory")]
    path: String,

    #[clap(short, long, help = "process directories recursively")]
    recursive: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    println!("Starting with arguments: {:?}", args);

    let path = Path::new(&args.path);
    process_path(path, args.recursive)?;
    Ok(())
}

fn process_path(path: &Path, recursive: bool) -> Result<()> {
    debug!("Processing path: {:?}", path);
    if path.is_dir() && recursive {
        debug!("Directory found, processing recursively");
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            process_path(&entry.path(), recursive)?;
        }
    } else if path.is_file() {
        debug!("File found, processing: {:?}", path);
        process_file(path)?;
    }
    Ok(())
}

fn process_file(path: &Path) -> Result<()> {
    debug!("Processing file: {:?}", path);
    let content = fs::read_to_string(path)?;
    Ok(())
}
