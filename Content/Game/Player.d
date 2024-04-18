instance PC_Hero(C_NPC)
{
    attribute[ATTRIBUTE_HITPOINTS] = 100;
    attribute[ATTRIBUTE_HITPOINTS_MAX] = 100;
    attribute[ATTRIBUTE_MANA] = 25;
    attribute[ATTRIBUTE_MANA_MAX] = 25;
    attribute[ATTRIBUTE_STRENGTH] = 10;
    attribute[ATTRIBUTE_DEXTERITY] = 10;

    Mdl_SetVisual(self, "HUMANS.mds");
    Mdl_SetVisualBody(self, "Hum_Body_Naked0", 9, 0, "Hum_Head_Pony", 18, 0, -1);

    CreateInvItem(self, ItMw_Dagger_Random());
    CreateInvItem(self, ItMw_Dagger_Random());
    CreateInvItem(self, ItMw_Dagger_Random());

    CreateInvItem(self, ItMw_HeavyBranch_Random());
    CreateInvItem(self, ItMw_HeavyBranch_Random());
    CreateInvItem(self, ItMw_HeavyBranch_Random());

    CreateInvItem(self, ItMw_Pickaxe_Random());
    CreateInvItem(self, ItMw_Pickaxe_Random());
    CreateInvItem(self, ItMw_Pickaxe_Random());
};