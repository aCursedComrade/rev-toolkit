#![allow(dead_code)]
// !!! Assault Cube 1.3.0.2 offsets !!!

// Local player struct offset
pub const STRUCT_SELF: usize = 0x17E0A8;

// Local player struct property offsets
pub const M_HEAD_X: usize = 0x4;
pub const M_HEAD_Y: usize = 0x8;
pub const M_HEAD_Z: usize = 0xC;

pub const M_BASE_X: usize = 0x28;
pub const M_BASE_Y: usize = 0x2C;
pub const M_BASE_Z: usize = 0x30;

pub const CAMERA_X: usize = 0x34;
pub const CAMERA_Y: usize = 0x38;
pub const CAMERA_Z: usize = 0x3C;

pub const HP: usize = 0xEC;
pub const ARMOR: usize = 0xF0;

pub const NADES: usize = 0x144;
pub const AR_CLIP: usize = 0x140;
pub const AR_RESV: usize = 0x11C;
pub const PISTOL_CLIP: usize = 0x12C;
pub const PISTOL_RESV: usize = 0x108;
pub const SHOTGUN_CLIP: usize = 0x134;
pub const SHOTGUN_RESV: usize = 0x110;
pub const SNIPER_CLIP: usize = 0x13C;
pub const SNIPER_RESV: usize = 0x118;
pub const SMG_CLIP: usize = 0x138;
pub const SMG_RESV: usize = 0x114;
pub const CARBINE_CLIP: usize = 0x130;
pub const CARBINE_RESV: usize = 0x10C;
