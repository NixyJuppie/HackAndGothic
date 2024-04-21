mod affix;

use affix::AffixCategory;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::generators::affix::Affix;

#[derive(Deserialize, Serialize, Debug)]
pub struct DaedalusGenerator {
    pub prototype: String,
    pub category: AffixCategory,
    pub unique: Option<String>,
}

impl DaedalusGenerator {
    pub fn generate(&self) -> Vec<String> {
        let mut contents = vec![];
        let mut content = String::new();

        let base = format!("{}__Base", self.prototype);
        content += &generate_instance_code(&base, &self.prototype, self.category, &vec![]);

        let mut instances: [Vec<String>; 4] = [vec![], vec![], vec![], vec![]];
        for count in 0..=4 {
            for definitions in self
                .category
                .definitions()
                .0
                .into_iter()
                .combinations(count)
            {
                for affixes in definitions
                    .iter()
                    .map(|d| d.value_range.clone().map(|v| d.affix(v)))
                    .multi_cartesian_product()
                {
                    let instance = instance_identifier(&self.prototype, &affixes);
                    content += &generate_instance_code(
                        &instance,
                        &self.prototype,
                        self.category,
                        &affixes,
                    );

                    instances[count - 1].push(instance);

                    if content.len() >= 1_000_000 {
                        contents.push(content);
                        content = String::new();
                    }
                }
            }
        }

        contents.push(content);
        contents.push(generate_random_func(
            base,
            self.unique.clone(),
            instances,
            &self.prototype,
        ));
        contents
    }
}

fn instance_identifier(prototype: &str, affixes: &Vec<Affix>) -> String {
    assert!(affixes.len() > 0);

    let mut identifier = format!("{prototype}__");
    for (index, affix) in affixes.iter().enumerate() {
        identifier += &format!("{}{}", affix.definition.identifier, affix.value);

        if index + 1 < affixes.len() {
            identifier += "_";
        }
    }

    identifier
}

fn generate_instance_code(
    instance: &str,
    prototype: &str,
    category: AffixCategory,
    affixes: &Vec<Affix>,
) -> String {
    let mut content = format!("instance {} ({})\n{{\n", instance, prototype);
    let mut value_multiplier = 1.0;

    for (index, affix) in affixes.iter().enumerate() {
        content += &generate_line(
            &affix
                .definition
                .code_template
                .replace("{}", &affix.value.to_string()),
        );
        content += &generate_description(
            index + 1,
            &format!("TEXT_{}Affix", affix.definition.identifier),
            &affix.value.to_string(),
        );

        value_multiplier *= calculate_value_multiplier(affix);
    }

    match category {
        AffixCategory::Weapon => content += &generate_description(0, "TEXT_Damage", "DamageTotal"),
    };

    content += &generate_line(&format!(
        "Value *= {} / 100;",
        (value_multiplier * 100.0).round() as i32
    ));
    content += &generate_description(5, "TEXT_Value", "Value");

    content + "};\n\n"
}

fn calculate_value_multiplier(affix: &Affix) -> f32 {
    let range = affix.definition.value_range.clone();
    let average = (range.clone().sum::<i32>() as f32 / range.len() as f32).round();
    let ratio = affix.value as f32 / average;
    1.0 + ((ratio - 1.0) * affix.definition.value_weight)
}

fn generate_random_func(
    base: String,
    unique: Option<String>,
    instances: [Vec<String>; 4],
    prototype: &str,
) -> String {
    let mut content = String::new();

    for (index, instances) in instances.iter().enumerate() {
        let affixes = index + 1;
        content += &format!("func int {}_Random{}() {{\n", prototype, affixes);
        content += &generate_line(&format!(
            "var int random; random = Hlp_Random({});",
            instances.len()
        ));

        for (index, instance) in instances.iter().enumerate() {
            content += &if index + 1 < instances.len() {
                generate_line(&format!(
                    "if (random == {}) {{ return {}; }};",
                    index, instance
                ))
            } else {
                generate_line(&format!("return {};", instance))
            };
        }

        content += "};\n\n";
    }

    content += &format!("func int {}_Random() {{\n", prototype);
    content += &generate_line("var int affixesCount; affixesCount = Hlp_Random(100);");
    content += &generate_line(&format!("if (affixesCount < 40) {{ return {}; }};", base));
    content += &generate_line(&format!(
        "if (affixesCount < 60) {{ return {prototype}_Random1(); }};"
    ));
    content += &generate_line(&format!(
        "if (affixesCount < 75) {{ return {prototype}_Random2(); }};",
    ));
    content += &generate_line(&format!(
        "if (affixesCount < 85) {{ return {prototype}_Random3(); }};",
    ));
    match unique {
        Some(unique) => {
            content += &generate_line(&format!(
                "if (affixesCount < 99) {{ return {prototype}_Random4(); }};"
            ));
            content += &generate_line(&format!("return {unique};"));
        }
        None => {
            content += &generate_line(&format!("return {prototype}_Random4();"));
        }
    }

    content += "};\n\n";
    content
}

fn generate_description(index: usize, label: &str, value: &str) -> String {
    generate_line(&format!("DescriptionLabel[{index}] = {label};"))
        + &generate_line(&format!("DescriptionValue[{index}] = {value};\n"))
}

fn generate_line(line: &str) -> String {
    format!("    {line}\n")
}
