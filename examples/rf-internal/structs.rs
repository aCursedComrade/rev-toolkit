#![allow(dead_code)]
// NOTE: the structs and pointers are left as it is when a player
// quits to menu while playing

// player struct
// static module: UnityPlayer.dll
pub const PLAYER_STRUCT_BASE: (&str, usize) = ("UnityPlayer.dll", 0x01A06C70);
pub const PLAYER_STRUCT_OFFSETS: [usize; 3] = [0x8, 0xD0, 0xE8];
pub const PLAYER_STRUCT_PATTERN: [u8; 12] = [
    0x57, 0x48, 0x83, 0xEC, 0x70, 0x48, 0x8B, 0x3D, 0xCF, 0x6A, 0x7B, 0x01,
];

pub const CUR_WEP: usize = 0xA0; // pointer
pub const HP: usize = 0x118; // float
pub const BALANCE: usize = 0x120; // float
pub const POS_X: usize = 0x370; // float
pub const POS_Y: usize = 0x374; // float
pub const POS_Z: usize = 0x378; // float
pub const IS_WALKING: usize = 0x274; // bool

// FIXME below defines if the player is playing or is spectator when in-game
// should try looking for something else
pub const IS_PLAYING: usize = 0x38C; // bool

// current weapon struct
// player->cur_wep-><offset>
pub const MAG: usize = 0x130; // u32
pub const RESERVE: usize = 0x1D0; // u32
pub const IS_AIMING: usize = 0x168; // bool
pub const SUSTAINED_FIRE: usize = 0x170; // bool
pub const WEP_TYPE: usize = 0x114; // u32
