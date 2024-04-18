mod weapon;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use weapon::WeaponGenerator;

pub trait Generator {
    fn generate(&self) -> String;
}

#[derive(Deserialize, Debug)]
pub enum DaedalusGenerator {
    Weapon(WeaponGenerator),
}

impl Generator for DaedalusGenerator {
    fn generate(&self) -> String {
        let mut content = match self {
            DaedalusGenerator::Weapon(g) => g.generate(),
        };
        content.insert_str(
            0,
            &format!(
                "\
// THIS FILE IS AUTO-GENERATED
// DATE: {}
// TEMPLATE: {:?}\n",
                chrono::Local::now(),
                self
            ),
        );
        content
    }
}

impl DaedalusGenerator {
    pub fn from_file_content(content: &str) -> Option<Self> {
        let Some(capture) = Regex::captures(&HEADER_REGEX, content).and_then(|c| c.name("content"))
        else {
            return None;
        };

        let generator: DaedalusGenerator = ron::Options::default()
            .with_default_extension(ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES)
            .from_str(capture.as_str())
            .expect("Cannot deserialize file header");

        Some(generator)
    }
}

lazy_static! {
    static ref HEADER_REGEX: Regex =
        Regex::new(r#"(?s)/\*(?:\s|)+@RANDOMIZE(?:\s|)+\n(?<content>.*?)\*/"#).unwrap();
}
