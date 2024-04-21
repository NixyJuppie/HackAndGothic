mod weapon;

use serde::{Deserialize, Serialize};
use std::{fmt::Debug, ops::Range};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AffixCategory {
    Weapon,
}

impl AffixCategory {
    pub fn definitions(&self) -> AffixDefinitions {
        match self {
            AffixCategory::Weapon => AffixDefinitions(weapon::DEFINITIONS.to_vec()),
        }
    }
}

pub struct AffixDefinitions(pub Vec<AffixDefinition>);

#[derive(Clone, Debug)]
pub struct AffixDefinition {
    pub identifier: &'static str,
    pub code_template: &'static str,
    pub value_range: Range<i32>,
    pub value_step: i32,
    pub value_weight: f32,
}

impl AffixDefinition {
    pub fn affix(&self, value: i32) -> Affix {
        Affix {
            value,
            definition: self,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Affix<'a> {
    pub definition: &'a AffixDefinition,
    pub value: i32,
}
