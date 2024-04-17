mod weapon;

use crate::weapon::WeaponGenerator;
use clap::Parser;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Parser)]
struct CliArgs {
    #[arg(short)]
    source: PathBuf,

    #[arg(short)]
    target: PathBuf,
}

fn main() {
    let args = CliArgs::parse();

    assert!(args.source.is_dir());
    assert!(!args.source.exists() || args.source.is_dir());
    fs::create_dir_all(args.target.clone()).unwrap();

    for file in fs::read_dir(args.source)
        .unwrap()
        .filter_map(|f| f.ok())
        .filter(|f| f.path().is_file() && f.path().extension().is_some_and(|e| e == "d"))
    {
        if let Some(target_content) = process_file(file.path()) {
            let target_path = args.target.join(file.file_name());

            println!("Writing {}", target_path.display());
            fs::write(target_path, target_content).unwrap()
        }
    }
}

fn process_file(file: PathBuf) -> Option<String> {
    println!("Processing {}", file.display());

    let Some(header) = read_header(file.clone()) else {
        return None;
    };

    match header.get("Type") {
        None => panic!("Template header is missing a 'Type' property"),
        Some(generator) => match generator.as_str() {
            "Weapon" => Some(Into::<WeaponGenerator>::into(header).generate()),
            _ => panic!("Not supported generator type '{}'", generator),
        },
    }
}

fn read_header(file: PathBuf) -> Option<HashMap<String, String>> {
    let mut map: HashMap<String, String> = HashMap::new();

    for (i, line) in fs::read_to_string(file).unwrap().lines().enumerate() {
        if i == 0 {
            if !line
                .trim()
                .trim_start_matches("/*")
                .trim()
                .starts_with("@RANDOMIZE")
            {
                return None;
            } else {
                continue;
            }
        }

        if line.trim().ends_with("*/") {
            break;
        }

        let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
        assert!(
            parts.len() == 2,
            "Header line must be in format '<NAME>:<VALUE>'"
        );

        map.insert(parts[0].to_string(), parts[1].to_string());
    }

    Some(map)
}
