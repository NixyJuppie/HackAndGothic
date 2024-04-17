class C_NPC
{
    var int Id;
    var string Name[5];
    var string Slot;
    var string Effect;
    var int NpcType;
    var int Flags;
    var int Attribute[ATR_LENGTH];
    var int HitChance[5];
    var int Protection[PROT_LENGTH];
    var int Damage[DAM_LENGTH];
    var int DamageType;
    var int Guild;
    var int Level;

    var func Mission[5];
    var int FightTactic;
    var int Weapon;
    var int Voice;
    var int VoicePitch;
    var int BodyMass;

    var func DailyRoutine;
    var func StartAiState;
    var string SpawnPoint;
    var int SpawnDelay;
    var int Senses;
    var int SensesRange;
    var int AiVar[100];
    var string Waypoint;

    var int Experience;
    var int ExperienceNext;
    var int LearningPoints;

    var int BodyStateInterruptableOverride;
    var int NoFocus;
};

instance hero(C_NPC);
instance self(C_NPC);
instance other(C_NPC);
instance victim(C_NPC);