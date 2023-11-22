// player struct
// static module: UnityPlayer.dll
pub const PLAYER_STRUCT_BASE: (&'static str, usize) = ("UnityPlayer.dll", 0x01A06C70);
pub const PLAYER_STRUCT_OFFSETS: [usize; 3] = [0x8, 0xD0, 0xE8];
pub const CUR_WEP: usize = 0xA0; // ptr
pub const HP: usize = 0x118; // float
pub const BALANCE: usize = 0x120; // float
pub const POS_X: usize = 0x370; //
pub const POS_Y: usize = 0x374; //
pub const POS_Z: usize = 0x378; // float
pub const IS_WALKING: usize = 0x274;
pub const IS_PLAYING: usize = 0x38C;

// current weapon struct
// player->cur_wep-><offset>
pub const MAG: usize = 0x130;
pub const RESERVE: usize = 0x1D0;
pub const IS_AIMING: usize = 0x168;
pub const SUSTAINED_FIRE: usize = 0x170; // bool
pub const WEP_TYPE: usize = 0x114;
