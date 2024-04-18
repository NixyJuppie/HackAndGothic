// Inventory can't be opened without this
const int BS_STAND = 0 | 32768 /* Interruptable */ | 65536 /* FreeHands */;

func void Startup_Global()
{
    Game_InitGerman();
};

func void Init_Global()
{
    Game_InitGerman();
};