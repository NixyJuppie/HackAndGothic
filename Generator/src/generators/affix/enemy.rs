use super::AffixDefinition;

pub const DEFINITIONS: [AffixDefinition; 3] = [AffixDefinition {
    identifier: "ExtraStrength",
    code_template: "Attribute[ATTRIBUTE_STRENGTH] += Attribute[ATTRIBUTE_STRENGTH] * {value} / 100;",
    value_range: 10..100,
    value_step: 10,
    value_weight: 1.25,
},
AffixDefinition {
    identifier: "ExtraHealth",
    code_template: "Attribute[ATTRIBUTE_HITPOINTS_MAX] += Attribute[ATTRIBUTE_HITPOINTS_MAX] * {value} / 100; Attribute[ATTRIBUTE_HITPOINTS] = Attribute[ATTRIBUTE_HITPOINTS_MAX];",
    value_range: 10..100,
    value_step: 10,
    value_weight: 1.25,
},
AffixDefinition {
    identifier: "ExtraSize",
    code_template: "Mdl_SetModelScale(self, {percent}, {percent}, {percent});",
    value_range: 110..200,
    value_step: 10,
    value_weight: 1.1,
}];
