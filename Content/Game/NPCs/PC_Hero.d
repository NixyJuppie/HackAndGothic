instance PC_Hero(C_NPC)
{
    Name = TEXT_PLAYER;

    Attribute[ATTRIBUTE_HITPOINTS] = 100;
    Attribute[ATTRIBUTE_HITPOINTS_MAX] = 100;
    Attribute[ATTRIBUTE_MANA] = 25;
    Attribute[ATTRIBUTE_MANA_MAX] = 25;
    Attribute[ATTRIBUTE_STRENGTH] = 10;
    Attribute[ATTRIBUTE_DEXTERITY] = 10;

    Mdl_SetVisual(self, "HUMANS.mds");
    Mdl_SetVisualBody(self, "Hum_Body_Naked0", 9, 0, "Hum_Head_Pony", 18, 0, -1);

    CreateInvItem(self, Dev_WeaponSpawner);
    CreateInvItem(self, Dev_EnemySpawner);
};