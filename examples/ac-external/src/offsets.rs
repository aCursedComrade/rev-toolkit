#![allow(dead_code)]
// !!! Assault Cube 1.3 offsets !!!

// Local player struct offset
pub const STRUCT_SELF: u64 = 0x0017E0A8;

// Local player struct property offsets
pub const M_HEAD_X: u32 = 0x4;
pub const M_HEAD_Y: u32 = 0x8;
pub const M_HEAD_Z: u32 = 0xC;

pub const M_BASE_X: u32 = 0x28;
pub const M_BASE_Y: u32 = 0x2C;
pub const M_BASE_Z: u32 = 0x30;

pub const CAMERA_X: u32 = 0x34;
pub const CAMERA_Y: u32 = 0x38;
pub const CAMERA_Z: u32 = 0x3C;

pub const HP: u32 = 0xEC;
pub const ARMOR: u32 = 0xF0;

pub const NADES: u32 = 0x144;
pub const AR_CLIP: u32 = 0x140;
pub const AR_RESV: u32 = 0x11C;
pub const PISTOL_CLIP: u32 = 0x12C;
pub const PISTOL_RESV: u32 = 0x108;
pub const SHOTGUN_CLIP: u32 = 0x134;
pub const SHOTGUN_RESV: u32 = 0x110;
pub const SNIPER_CLIP: u32 = 0x13C;
pub const SNIPER_RESV: u32 = 0x118;
pub const SMG_CLIP: u32 = 0x138;
pub const SMG_RESV: u32 = 0x0;
pub const CARBINE_CLIP: u32 = 0x130;
pub const CARBINE_RESV: u32 = 0x10C;
