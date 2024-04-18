use super::Generator;
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Serialize, Deserialize, Debug)]
pub struct WeaponGenerator {
    pub prototype: String,
    pub damage: Range<u32>,
    pub range: Range<u32>,
    pub value: u32,
}

impl Generator for WeaponGenerator {
    fn generate(&self) -> String {
        let mut content = String::new();
        let mut names = vec![];

        for damage in self.damage.clone() {
            for range in self.range.clone() {
                let value = calculate_value(self, damage, range);
                let (code, name) = generate_instance(&self, damage, range, value);
                names.push(name);
                content += &code;
            }
        }

        content += &generate_random_func(&self.prototype, names);

        content
    }
}

fn calculate_value(generator: &WeaponGenerator, damage: u32, range: u32) -> u32 {
    let mut value = generator.value as f32;
    value *= calculate_multiplier(damage, &generator.damage, 1.5);
    value *= calculate_multiplier(range, &generator.range, 0.75);

    value.round() as u32
}

fn calculate_multiplier(value: u32, range: &Range<u32>, multiplier: f32) -> f32 {
    let average = (range.clone().sum::<u32>() as f32 / range.len() as f32).round();
    let ratio = value as f32 / average;
    1.0 + ((ratio - 1.0) * multiplier)
}

fn generate_instance(
    generator: &WeaponGenerator,
    damage: u32,
    range: u32,
    value: u32,
) -> (String, String) {
    let prototype = &generator.prototype;
    let name = format!("{prototype}_{damage}_{range}_{value}");
    let quality = ((value as f32 / generator.value as f32) * 100.0).round() as u32;
    (
        format!(
            "
instance {name} ({prototype})
{{
    DamageTotal = {damage};
    Range = {range};
    Value = {value};
    DescriptionLabel[0] = TEXT_DAMAGE;
    DescriptionValue[0] = DamageTotal;
    DescriptionLabel[1] = TEXT_Range;
    DescriptionValue[1] = Range;
    DescriptionLabel[4] = TEXT_QUALITY;
    DescriptionValue[4] = {quality};
    DescriptionLabel[5] = TEXT_VALUE;
    DescriptionValue[5] = Value;
}};\n"
        ),
        name,
    )
}

fn generate_random_func(prototype: &str, names: Vec<String>) -> String {
    let mut content = String::new();

    content += &format!(
        "\nfunc int {}_Random() {{\n    var int random; random = Hlp_Random({});\n",
        prototype,
        names.len()
    );

    for (index, name) in names.iter().enumerate() {
        if index + 1 == names.len() {
            content += &format!("    return {};\n", name);
        } else {
            content += &format!("    if (random == {}) {{ return {}; }};\n", index, name);
        }
    }

    content += "};";

    content
}
