instance Focus_Normal (C_FOCUS)
{
    NpcLongRange = 500.0;
    NpcRange1 = 0.0;
    NpcRange2 = 300.0;
    NpcAzimuth = 30.0;
    NpcElevationUp = 30.0;
    NpcElevationDown = -45.0;
    NpcPriority = 1;

    ItemRange1 = 0.0;
    ItemRange2 = 300.0;
    ItemAzimuth = 30.0;
    ItemElevationUp = 60.0;
    ItemElevationDown = -89.0;
    ItemPriority = 1;

    MobRange1 = 0.0;
    MobRange2 = 350.0;
    MobAzimuth = 30.0;
    MobElevationUp = 89.0;
    MobElevationDown = -89.0;
    MobPriority = 0;
};

instance Focus_Melee (C_FOCUS)
{
    NpcLongRange = 600.0;
    NpcRange1 = 0.0;
    NpcRange2 = 500.0;
    NpcAzimuth = 50.0;
    NpcElevationUp = 90.0;
    NpcElevationDown = -60.0;
    NpcPriority = 1;

    ItemPriority = -1;
    MobPriority = -1;
};


instance Focus_Throw_Item (C_FOCUS)
{
    NpcRange1 = 300.0;
    NpcRange2 = 1500.0;
    NpcAzimuth = 20.0;
    NpcElevationUp = 45.0;
    NpcElevationDown = -45.0;

    ItemRange1 = 300.0;
    ItemRange2 = 1500.0;
    ItemAzimuth = 20.0;
    ItemElevationUp = 45.0;
    ItemElevationDown = -45.0;

    MobRange1 = 300.0;
    MobRange2 = 1500.0;
    MobAzimuth = 20.0;
    MobElevationUp = 45.0;
    MobElevationDown = -45.0;
};
