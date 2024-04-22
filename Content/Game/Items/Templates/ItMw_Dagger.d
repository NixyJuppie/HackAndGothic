/* @RANDOMIZE
(
    category: Weapon,
    prototype: "ItMw_Dagger",
    uniques: [ "ItMw_FireDagger" ]
)
*/

prototype ItMw_Dagger (C_ITEM)
{
    Name = TEXT_Dagger;
    DescriptionTitle = Name;
    Category = ITEM_CATEGORY_MELEE;
    Flags = ITEM_FLAG_SWD;
    Material = MATERIAL_METAL;
    DamageTotal = 5;
    DamageType = DAMAGE_EDGE;
    Range = 50;
    Visual = "ItMw_005_1h_Dagger_01.3DS";
    Value = 10;
};

instance ItMw_FireDagger (ItMw_Dagger)
{
    Name = TEXT_FireDagger;
    DamageType = DAMAGE_FIRE;
    Visual = "ItMw_005_1h_Dagger_01.3DS";
    Effect = "SPELLFX_FIREARROW";
    DescriptionTitle = Name;

    DamageTotal += DamageTotal * 400 / 100;
    DescriptionLabel[1] = TEXT_EnhancedDamageAffix;
    DescriptionValue[1] = 400;

    Range += Range * 25 / 100;
    DescriptionLabel[2] = TEXT_EnhancedRangeAffix;
    DescriptionValue[2] = 25;

    ChangeAttribute[0] = ATTRIBUTE_HITPOINTS_MAX; ChangeValue[0] = 25;
    DescriptionLabel[3] = TEXT_BonusHitpointsAffix;
    DescriptionValue[3] = 25;

    ChangeAttribute[1] = ATTRIBUTE_STRENGTH; ChangeValue[1] = 10;
    DescriptionLabel[4] = TEXT_BonusStrengthAffix;
    DescriptionValue[4] = 10;

    var string damageText; damageText = ConcatStrings(TEXT_Damage, ConcatStrings(" = ", IntToString(DamageTotal)));
    var string rangeText; rangeText = ConcatStrings(TEXT_Range, ConcatStrings(" = ", IntToString(Range)));
    DescriptionLabel[0] = ConcatStrings(damageText, ConcatStrings(", ", rangeText));
    DescriptionValue[0] = 0;

    Value = 250;
    DescriptionLabel[5] = TEXT_Value;
    DescriptionValue[5] = Value;
};