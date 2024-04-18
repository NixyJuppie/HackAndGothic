mod generators;

use clap::Parser;
use generators::{DaedalusGenerator, Generator};
use log::{debug, info, warn};
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(short)]
    source: PathBuf,

    #[arg(short)]
    target: PathBuf,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let args = CliArgs::parse();
    debug!("Parsed args: {:?}", args);

    assert!(args.source.is_dir());
    assert!(!args.source.exists() || args.source.is_dir());
    fs::create_dir_all(args.target.clone()).unwrap();

    debug!("Reading files from directory {}", args.source.display());
    for file in fs::read_dir(args.source)
        .unwrap()
        .filter_map(|f| f.ok())
        .filter(|f| f.path().is_file() && f.path().extension().is_some_and(|e| e == "d"))
    {
        info!("Reading file '{}'", file.path().display());
        let content = fs::read_to_string(file.path()).expect("Cannot read file as string");
        let target = args.target.join(file.file_name());

        match DaedalusGenerator::from_file_content(&content) {
            None => {
                warn!("Skipping file without header '{}'", file.path().display());
                let _ = fs::remove_file(target);
            }
            Some(generator) => {
                info!("Writing generated code to {}", target.display());
                fs::write(target, generator.generate()).expect("Cannot write content to file")
            }
        }
    }
}
