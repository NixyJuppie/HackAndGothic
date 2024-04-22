mod generators;

use clap::Parser;
use generators::DaedalusGenerator;
use lazy_static::lazy_static;
use log::{debug, info, warn};
use regex::Regex;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(short, help = "Set source directory")]
    source: PathBuf,

    #[arg(short, help = "Set target directory")]
    target: PathBuf,

    #[arg(long, help = "Purge target directory before generation")]
    purge_target_dir: bool,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = CliArgs::parse();
    debug!("Parsed args: {:?}", args);

    assert!(args.source.is_dir());
    assert!(!args.source.exists() || args.source.is_dir());

    if args.purge_target_dir {
        info!("Purging directory {}", args.target.display());
        fs::remove_dir_all(args.target.clone()).unwrap();
    }
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

        match deserialize_generator(&content) {
            None => {
                warn!("Skipping file without header '{}'", file.path().display());
                let _ = fs::remove_file(target);
            }
            Some(generator) => {
                info!("Writing generated code for {}", file.path().display());
                for (index, content) in generator.generate().iter_mut().enumerate() {
                    let target = target.with_extension(format!("{index}.d"));

                    content.insert_str(
                        0,
                        &format!(
                            "\
// THIS FILE IS AUTO-GENERATED
// DATE: {}
// TEMPLATE: {:?}\n\n",
                            chrono::Local::now(),
                            generator
                        ),
                    );

                    debug!("Writing generated code to {}", target.display());
                    fs::write(target.clone(), content).expect("Cannot write content to file")
                }
            }
        }
    }
}

lazy_static! {
    static ref HEADER_REGEX: Regex =
        Regex::new(r#"(?s)/\*(?:\s|)+@RANDOMIZE(?:\s|)+\n(?<content>.*?)\*/"#).unwrap();
}

fn deserialize_generator(content: &str) -> Option<DaedalusGenerator> {
    let Some(capture) = Regex::captures(&HEADER_REGEX, content).and_then(|c| c.name("content"))
    else {
        return None;
    };

    let generator: DaedalusGenerator = ron::Options::default()
        .with_default_extension(ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES)
        .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
        .from_str(capture.as_str())
        .expect("Cannot deserialize file header");

    Some(generator)
}
