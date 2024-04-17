class C_FOCUS
{
    var float NpcLongRange;
    var float NpcRange1;
    var float NpcRange2;
    var float NpcAzimuth;
    var float NpcElevationDown;
    var float NpcElevationUp;
    var int NpcPriority;

    var float ItemRange1;
    var float ItemRange2;
    var float ItemAzimuth;
    var float ItemElevationDown;
    var float ItemElevationUp;
    var int ItemPriority;

    var float MobRange1;
    var float MobRange2;
    var float MobAzimuth;
    var float MobElevationDown;
    var float MobElevationUp;
    var int MobPriority;
};

// Inventory can't be opened without this
const int BS_STAND = 0 | 32768 /* Interruptable */ | 65536 /* FreeHands */;
instance Focus_Throw_Item (C_FOCUS) { ItemAzimuth = 20.0; };