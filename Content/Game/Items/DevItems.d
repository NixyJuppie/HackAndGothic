instance Dev_WeaponSpawner (C_ITEM)
{
    Name = "Weapon Spawner";
    Category = ITEM_CATEGORY_DOC;
    Visual = "ItRu_Zap.3DS";
    Effect = "SPELLFX_HEALTHPOTION";
    ScemeName = "MAP";
    DescriptionTitle = Name;
    OnState[0] = UseWeaponSpawner;
};
func void UseWeaponSpawner()
{
    CreateInvItem(self, ItMw_Dagger_Random());
    CreateInvItem(self, ItMw_HeavyBranch_Random());
    CreateInvItem(self, ItMw_ShortSword_Random());
    CreateInvItem(self, ItMw_OrcSlayer_Random());

    CreateInvItem(self, ItMw_Pickaxe_Random());
    CreateInvItem(self, ItMw_RustyAxe_Random());
    CreateInvItem(self, ItMw_RustyTwoHander_Random());
    CreateInvItem(self, ItMw_DragonSlayer_Random());
};

instance Dev_EnemySpawner (C_ITEM)
{
    Name = "Enemy Spawner";
    Category = ITEM_CATEGORY_DOC;
    Visual = "ItRu_Beliar05.3DS";
    Effect = "SPELLFX_ITEMGLIMMER";
    ScemeName = "MAP";
    DescriptionTitle = Name;
    OnState[0] = UseEnemySpawner;
};
func void UseEnemySpawner()
{
    var string wp; wp = Npc_GetNearestWP(self);
    Wld_InsertNpc(Enemy, wp);
    Wld_InsertNpc(Enemy, wp);
    Wld_InsertNpc(Enemy, wp);
    Wld_InsertNpc(Enemy, wp);
    Wld_InsertNpc(Enemy, wp);
};