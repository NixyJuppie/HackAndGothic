class C_ITEM
{
    var int id;
    var string name;
    var string nameID;
    var int Hitpoints;
    var int MaxHitpoints;
    var int MainFlag;
    var int Flags;
    var int Weight;
    var int Value;

    var int DamageType;
    var int DamageTotal;
    var int Damage[DAM_LENGTH];
    var int Wear;
    var int Protection[PROT_LENGTH];
    var int Nutrition;

    var int ConditionAttribute[3];
    var int ConditionValue[3];
    var int ChangeAttribute[3];
    var int ChangeValue[3];

    var func Magic;
    var func OnEquip;
    var func OnUnequip;
    var func OnState[4];

    var func Owner;
    var int OwnerGuild;
    var int DisguiseGuild;

    var string Visual;
    var string VisualChange;
    var string Effect;
    var int VisualSkin;
    var string ScemeName;
    var int Material;

    var int Munition;
    var int Spell;
    var int Range;
    var int MagCircle;

    var string DescriptionTitle;
    var string DescriptionLabel[6];
    var int DescriptonValue[6];

    var int Inventory_BiasZ;
    var int Inventory_RotationX;
    var int Inventory_RotationY;
    var int Inventory_RotationZ;
    var int Inventory_Animate;
};

instance item(C_ITEM);