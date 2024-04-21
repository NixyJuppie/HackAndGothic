use super::AffixDefinition;

pub const DEFINITIONS: [AffixDefinition; 5] = [
    AffixDefinition {
        identifier: "EnhancedDamage",
        code_template: "DamageTotal += DamageTotal * {} / 100;",
        value_range: 5..50,
        value_step: 5,
        value_weight: 1.25,
    },
    AffixDefinition {
        identifier: "EnhancedRange",
        code_template: "Range += Range * {} / 100;",
        value_range: 5..30,
        value_step: 5,
        value_weight: 0.9,
    },
    AffixDefinition {
        identifier: "BonusHitpoints",
        code_template: "ChangeAttribute[0] = ATTRIBUTE_HITPOINTS_MAX; ChangeValue[0] = {};",
        value_range: 5..25,
        value_step: 5,
        value_weight: 1.1,
    },
    AffixDefinition {
        identifier: "BonusMana",
        code_template: "ChangeAttribute[1] = ATTRIBUTE_MANA_MAX; ChangeValue[1] = {};",
        value_range: 3..15,
        value_step: 3,
        value_weight: 1.1,
    },
    AffixDefinition {
        identifier: "BonusStrength",
        code_template: "ChangeAttribute[2] = ATTRIBUTE_STRENGTH; ChangeValue[2] = {};",
        value_range: 2..10,
        value_step: 2,
        value_weight: 1.5,
    },
];
