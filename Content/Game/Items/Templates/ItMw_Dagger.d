/* @RANDOMIZE
(
    prototype: "ItMw_Dagger",
    category: Weapon,
    unique: "ItMw_FireDagger"
)
*/

prototype ItMw_Dagger (C_ITEM)
{
    Name = "Dagger";
    DescriptionTitle = Name;
    Category = ITEM_CATEGORY_MELEE;
    Flags = ITEM_FLAG_SWD;
    Material = MATERIAL_METAL;
    DamageTotal = 5;
    DamageType = DAMAGE_EDGE;
    Visual = "ItMw_005_1h_Dagger_01.3DS";
    Value = 15;
};

instance ItMw_FireDagger (ItMw_Dagger)
{
    Name = "Fire Dagger";
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

    DescriptionLabel[0] = TEXT_Damage;
    DescriptionValue[0] = DamageTotal;

    Value = 500;
    DescriptionLabel[5] = TEXT_Value;
    DescriptionValue[5] = Value;
};