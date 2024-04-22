/* @RANDOMIZE
(
    category: Enemy,
    prototype: "Goblin",
    uniques: []
)
*/

prototype Goblin(C_NPC)
{
    Name = TEXT_Goblin;

    Attribute[ATTRIBUTE_HITPOINTS] = 50;
    Attribute[ATTRIBUTE_HITPOINTS_MAX] = 50;
    Attribute[ATTRIBUTE_STRENGTH] = 10;
    Attribute[ATTRIBUTE_DEXTERITY] = 10;

    Mdl_SetVisual(self, "Gobbo.mds");
    Mdl_SetVisualBody(self, "Gob_Body", 0, 0, "", 0, 0, -1);

    Npc_SetToFightMode(self, ItMw_HeavyBranch_Random());
};