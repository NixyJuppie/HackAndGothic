use std::collections::HashMap;

pub struct WeaponGenerator {
    prototype: String,
    damage: (u32, u32),
    range: (u32, u32),
    value: u32,
}

impl Into<WeaponGenerator> for HashMap<String, String> {
    fn into(self) -> WeaponGenerator {
        WeaponGenerator {
            prototype: read_string(&self, "Prototype"),
            damage: read_u32_pair(&self, "Damage"),
            range: read_u32_pair(&self, "Range"),
            value: read_u32(&self, "Value"),
        }
    }
}

fn read_string(map: &HashMap<String, String>, name: &str) -> String {
    map.get(name)
        .expect(&format!("Missing property '{}'", name))
        .clone()
}

fn read_u32(map: &HashMap<String, String>, name: &str) -> u32 {
    map.get(name)
        .expect(&format!("Missing property '{}'", name))
        .parse()
        .expect(&format!("Cannot parse '{}'", name))
}

fn read_u32_pair(map: &HashMap<String, String>, name: &str) -> (u32, u32) {
    let splits: Vec<&str> = map
        .get(name)
        .expect(&format!("Missing property '{}'", name))
        .split(' ')
        .collect();

    assert!(splits.len() == 2, "Cannot parse property '{}'", name);

    (
        splits[0].parse().expect("Cannot parse property '{}'[0]"),
        splits[1].parse().expect("Cannot parse property '{}'[1]"),
    )
}

impl WeaponGenerator {
    pub fn generate(&self) -> String {
        let mut content = String::new();
        let mut names = vec![];

        for damage in (self.damage.0 - self.damage.1)..=(self.damage.0 + self.damage.1) {
            for range in (self.range.0 - self.range.1)..=(self.range.0 + self.range.1) {
                let value = (self.value as f32
                    * (damage as f32 / self.damage.0 as f32)
                    * (range as f32 / self.range.0 as f32)) as u32;

                let (code, name) = Self::write_instance(&self.prototype, damage, range, value);
                names.push(name);
                content += &code;
            }
        }

        content += &format!(
            "\nfunc int Random{}() {{\n    var int random; random = Hlp_Random({});\n",
            self.prototype,
            names.len()
        );
        for (index, name) in names.iter().enumerate() {
            content += &format!("    if (random == {}) {{ return {}; }};\n", index, name);
        }

        content += &format!(
            "\n    print(\"FALLBACK: {}\");\n    return {};\n}};",
            self.prototype, names[0]
        );

        content
    }

    fn write_instance(prototype: &str, damage: u32, range: u32, value: u32) -> (String, String) {
        let name = format!("{prototype}_{damage}_{range}_{value}");
        (
            format!(
                "
instance {name} ({prototype})
{{
    DamageTotal = {damage};
    Range = {range};
    Value = {value};
    DescriptionTitle = Name;
    DescriptionLabel[0] = TEXT_DAMAGE;
    DescriptionValue[0] = DamageTotal;
    DescriptionLabel[1] = TEXT_Range;
    DescriptionValue[1] = Range;
    DescriptionLabel[2] = TEXT_VALUE;
    DescriptionValue[2] = Value;
}};
"
            ),
            name,
        )
    }
}
