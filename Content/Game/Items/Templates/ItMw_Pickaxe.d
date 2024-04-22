/* @RANDOMIZE
(
    category: Weapon,
    prototype: "ItMw_Pickaxe",
    uniques: []
)
*/

prototype ItMw_Pickaxe (C_ITEM)
{
    Name = TEXT_Pickaxe;
    DescriptionTitle = Name;
    Category = ITEM_CATEGORY_MELEE;
    Flags = ITEM_FLAG_2HD_AXE;
    Material = MATERIAL_METAL;
    DamageTotal = 20;
    DamageType = DAMAGE_EDGE;
    Range = 60;
    Visual = "ItMw_020_2h_Pickaxe_01.3DS";
    Value = 10;
};