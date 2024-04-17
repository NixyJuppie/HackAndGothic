instance PC_Hero(C_NPC)
{
    attribute[ATR_HITPOINTS] = 100;
    attribute[ATR_HITPOINTS_MAX] = 100;
    attribute[ATR_MANA] = 25;
    attribute[ATR_MANA_MAX] = 25;
    attribute[ATR_STRENGTH] = 10;
    attribute[ATR_DEXTERITY] = 10;

    Mdl_SetVisual(self, "HUMANS.mds");
    Mdl_SetVisualBody(self, "Hum_Body_Naked0", 9, 0, "Hum_Head_Pony", 18, 0, -1);
};