func string CreateWeaponSummary(var int damage, var int range, var int flags)
{
    const int twoHandedFlags = ITEM_FLAG_2HD_SWD | ITEM_FLAG_2HD_AXE;

    var string typeText; if ((flags & twoHandedFlags) == 0) { typeText = TEXT_OneHanded; } else { typeText = TEXT_TwoHanded; };
    var string damageText; damageText = ConcatStrings(TEXT_Damage, ConcatStrings(" = ", IntToString(damage)));
    var string rangeText; rangeText = ConcatStrings(TEXT_Range, ConcatStrings(" = ", IntToString(range)));
    var string damageRangeText; damageRangeText = ConcatStrings(damageText, ConcatStrings(", ", rangeText));
    return ConcatStrings(ConcatStrings(typeText, ", "), damageRangeText);
};