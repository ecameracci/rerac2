//! The hand item (slot 0 of the item slots `0x18b040 + 0x50·k`): its creation `0x2afc68` (the item's moby class from
//! the item record table `0x262500` row `0x139568[item]`, [`ItemRow`]), its placement at Ratchet's attachment matrix
//! `0x2b09e0` (`0x18aa70 + 0x40·k`, k = the record's word 0), the item change `0x2b13c8` (□ takes the wrench out, ○
//! the last weapon back; the old item is deleted on the next pass and the new one created two frames on, `0x2b12a8`),
//! the use stance (`0x2af5e0` / `0x2af848` / `0x2af8d8`: Ratchet's sequence while a weapon fires) and the requests
//! the states make of it (its animation, the throw). The world side ([`item_pass`]) runs after the hero update, as
//! `0x2b21f0` / `0x2b09e0` do inside it.
//!
//! A game starts with the Lancer (crate::game::GameState::start_items), so Ratchet arrives holding it.
//!
//! The back slot (slot 3, `0x18b130`: Clank and the packs) is crate::hero::back's.
//!
//! Not ported: the quick select (`0x3227d8`), the
//! gadgets a request uses instead of holding (0x1f, 0x3d, 9, 0x2d), the weapon before the last (`0x1a7a0c`), the item a forced wrench gives back (`0x18c120`), the class streaming (`0x305cc0` /
//! `0x305ab0`: the engine loads every hand class with the level), the upper-body layers of a use (`0x311e10`, rows
//! `+0x1c` / `+0x20`: the Lancer has none), the L2 + R2 weapon pull (`0x1a7a88` bit 0x20, `0x2d67b8`), the strafe /
//! look stance marker.

use super::physics::V3;
use crate::moby::{mode, Ctx, MobyWorld};

/// The wrench's moby class (`0x262504` of item 10's row).
pub const WRENCH_CLASS: i16 = 0x47;
/// The wrench's item id.
pub const WRENCH: i32 = 10;
/// The Lancer: item 0x1e, class 0x5d2 (`0x262504` of its row; a gadget, `G/GADGET.WAD`).
pub const LANCER: i32 = 0x1e;
pub const LANCER_CLASS: i16 = 0x5d2;
/// The Dynamo: item 0x24, class 0x825 (L01 item table `0x2637b0`; got on Oozla, crate::moby::classes::dynamo).
pub const DYNAMO: i32 = 0x24;
pub const DYNAMO_CLASS: i16 = 0x825;
/// The Gravity Bomb (item 0x2a) and its hand moby's class (crate::moby::classes::gravity_bomb).
pub const GRAVITY_BOMB: i32 = 0x2a;
pub const GRAVITY_BOMB_CLASS: i16 = 0xce3;
/// The Tractor Beam: item 0x2e, class 0xbc (L01 item table; sold on Oozla, crate::moby::classes::tractor).
pub const TRACTOR: i32 = 0x2e;
pub const TRACTOR_CLASS: i16 = 0xbc;
/// The Electrolyzer: item 0x26, class 0x870 (L02 item table `0x264a80`, row 0x26; its puzzle:
/// crate::moby::classes::electrolyzer).
pub const ELECTROLYZER: i32 = 0x26;
pub const ELECTROLYZER_CLASS: i16 = 0x870;
/// The Spiderbot Glove: item 0x20, class 0x907 (L02 item table, row `0x266680`; crate::moby::classes::spiderbot).
pub const SPIDERBOT: i32 = 0x20;
pub const SPIDERBOT_CLASS: i16 = 0x907;
/// The Swingshot: item 0xd, class 0xd0 (L03 item table, row `0x267910`; crate::moby::classes::swingshot).
pub const SWINGSHOT: i32 = 0xd;
pub const SWINGSHOT_CLASS: i16 = crate::moby::classes::swingshot::HAND_CLASS;
/// The Chopper: item 0x16, class 0x879 (L00 row `0x263840`; crate::moby::classes::chopper).
pub const CHOPPER: i32 = 0x16;
pub const CHOPPER_CLASS: i16 = crate::moby::classes::chopper::HAND_CLASS;
/// The Blitz Gun: item 0x1a, class 0x5d1 (L00 row `0x263bc0`; crate::moby::classes::blitz).
pub const BLITZ: i32 = 0x1a;
pub const BLITZ_CLASS: i16 = crate::moby::classes::blitz::HAND_CLASS;
/// The Pulse Rifle: item 0x17, class 0x5cb (L00 row `0x263920`; crate::moby::classes::pulse_rifle).
pub const PULSE: i32 = 0x17;
pub const PULSE_CLASS: i16 = crate::moby::classes::pulse_rifle::HAND_CLASS;
/// The Mini-Turret Glove: item 0x29, class 0xc5a (L00 row `0x2648e0`; crate::moby::classes::turret).
pub const MINI_TURRET: i32 = 0x29;
pub const MINI_TURRET_CLASS: i16 = crate::moby::classes::turret::HAND_CLASS;
/// The Synthenoids: item 0x1f, class 0xcb2 (L00 row `0x264020`; not held: asked for, they are sent out,
/// crate::moby::classes::synthenoid).
pub const SYNTHENOID: i32 = 0x1f;
/// The Lava Gun: item 0x1d, class 0x5c9 (L00 row `0x263e60`; crate::moby::classes::lava_gun).
pub const LAVA: i32 = 0x1d;
/// The Seeker Gun: item 0x18, class 0x5ce (L00 row `0x263a00`; crate::moby::classes::seeker).
pub const SEEKER: i32 = 0x18;
pub const SEEKER_CLASS: i16 = crate::moby::classes::seeker::HAND_CLASS;
pub const LAVA_CLASS: i16 = crate::moby::classes::lava_gun::HAND_CLASS;
/// The Bouncer: item 0x25, class 0xe14 (L00 row `0x264560`; crate::moby::classes::bouncer).
pub const BOUNCER: i32 = 0x25;
pub const BOUNCER_CLASS: i16 = crate::moby::classes::bouncer::HAND_CLASS;
/// The Minirocket Tube: item 0x1b, class 0x5cf (L00 row `0x263ca0`; crate::moby::classes::minirocket).
pub const MINIROCKET: i32 = 0x1b;
pub const MINIROCKET_CLASS: i16 = crate::moby::classes::minirocket::HAND_CLASS;
/// The Plasma Coil: item 0x1c, class 0xc4a (L00 row `0x263d80`; crate::moby::classes::plasma_coil).
pub const PLASMA_COIL: i32 = 0x1c;
pub const PLASMA_COIL_CLASS: i16 = crate::moby::classes::plasma_coil::HAND_CLASS;
/// The Hoverbomb Gun: item 0x19, class 0xa5b (L00 row `0x263ae0`; crate::moby::classes::hoverbomb).
pub const HOVERBOMB: i32 = 0x19;
pub const HOVERBOMB_CLASS: i16 = crate::moby::classes::hoverbomb::HAND_CLASS;
/// The Shield Charger: item 0x2d, its shield's class 0xa5c (L00 row `0x264c60`; not held: asked for, the shield is
/// raised, crate::moby::classes::shield).
pub const SHIELD: i32 = 0x2d;
/// The Zodiac: item 0x2b, class 0xec8 (L00 row `0x264aa0`; crate::moby::classes::zodiac).
pub const ZODIAC: i32 = crate::moby::classes::zodiac::ITEM;
pub const ZODIAC_CLASS: i16 = crate::moby::classes::zodiac::HAND_CLASS;
/// The RYNO II: item 0x2c, class 0xa67 (L00 row `0x264b80`; crate::moby::classes::ryno).
pub const RYNO: i32 = crate::moby::classes::ryno::ITEM;
pub const RYNO_CLASS: i16 = crate::moby::classes::ryno::HAND_CLASS;
/// The Sheepinator: item 0x10, class 0xa5d (L00 row `0x263300`; crate::moby::classes::sheepinator).
pub const SHEEPINATOR: i32 = crate::moby::classes::sheepinator::ITEM;
pub const SHEEPINATOR_CLASS: i16 = crate::moby::classes::sheepinator::HAND_CLASS;
/// The Clank Zapper: item 9 (L00 row `0x262ce0`; not held: asked for, it turns on, crate::moby::classes::zapper). Its
/// row's class is the zap 0x0ed8.
pub const ZAPPER: i32 = crate::moby::classes::zapper::ITEM;

/// An item record (`0x262500 + 0xe0·row`, row `0x139568[item]`: crate::upgrades): the fields the port reads.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ItemRow {
    /// The record's row (a base row's is its item's id; the upgrades' rows follow 0x37, [`UPGRADE_ROWS`]).
    pub item: i32,
    /// `+0x00`: Ratchet's attachment it sits at; `+0x04` its moby class.
    pub attach: usize,
    pub class: i16,
    /// `+0x0c`: its stance type (`0x18c0db`; 0 the wrench's).
    pub stance: u8,
    /// `+0x2c`: its HUD icon (the weapon panel, crate::hud::WeaponPanel).
    pub icon: u16,
    /// `+0x18`: Ratchet's sequence while it is used standing; `+0x24`: the use keeps his stance (`0x18c0da`).
    pub use_seq: i32,
    pub keep: bool,
    /// `+0x1c` / `+0x20`: the firing arm's sequence while it is used moving / crouched (group 0xc); −1 none.
    pub fire_seq: [i32; 2],
    /// `+0x40`: its shots' speed (per second); `+0x44` the shots a second; `+0x60` the damage.
    pub speed: f32,
    pub rate: f32,
    pub damage: f32,
    /// `+0x78` ≠ 0: it has ammo; `+0x7e` the most; `+0x82` what it comes with.
    pub ammo: bool,
    pub max_ammo: i32,
    pub start_ammo: i32,
}

/// The rows the port has (L00 `0x262dc0`, `0x263f40`, `0x2649c0`; the Dynamo's from Oozla's live table; the
/// Electrolyzer's and the Spiderbot Glove's from L02's; the Swingshot's from L03's; the Chopper's L00 `0x263840`; the
/// Blitz Gun's L00 `0x263bc0`; the Pulse Rifle's L00 `0x263920`; the Mini-Turret Glove's L00 `0x2648e0`; the
/// Synthenoids' L00 `0x264020`; the Lava Gun's L00 `0x263e60`; the Seeker Gun's L00 `0x263a00`; the Bouncer's L00 `0x264560`;
/// the Minirocket Tube's L00 `0x263ca0`; the Plasma Coil's L00 `0x263d80`; the Hoverbomb Gun's L00 `0x263ae0`; the Shield Charger's L00 `0x264c60`; the Zodiac's L00 `0x264aa0`; the RYNO II's L00 `0x264b80`; the Sheepinator's L00 `0x263300`).
pub const ROWS: [ItemRow; 24] = [
    ItemRow { item: WRENCH, attach: 0, class: WRENCH_CLASS, stance: 0, icon: 0, use_seq: 0, keep: false, fire_seq: [0, 0], speed: 0.0, rate: 0.0, damage: 1.0, ammo: false, max_ammo: 0, start_ammo: 0 },
    ItemRow { item: LANCER, attach: 0, class: LANCER_CLASS, stance: 1, icon: 0xea64, use_seq: 0x38, keep: true, fire_seq: [-1, 0x3e], speed: 60.0, rate: 7.0, damage: 0.5, ammo: true, max_ammo: 200, start_ammo: 200 },
    ItemRow { item: DYNAMO, attach: 1, class: DYNAMO_CLASS, stance: 1, icon: 0xea7f, use_seq: 0x8d, keep: false, fire_seq: [0x8e, 0x8d], speed: 0.0, rate: 0.0, damage: 0.0, ammo: false, max_ammo: 0, start_ammo: 0 },
    ItemRow { item: GRAVITY_BOMB, attach: 0, class: GRAVITY_BOMB_CLASS, stance: 2, icon: 0xea65, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 0.0, rate: 0.5, damage: 0.0, ammo: true, max_ammo: 8, start_ammo: 8 },
    ItemRow { item: TRACTOR, attach: 0, class: TRACTOR_CLASS, stance: 1, icon: 0xea80, use_seq: 0, keep: true, fire_seq: [-1, -1], speed: 0.0, rate: 0.0, damage: 0.0, ammo: false, max_ammo: 0, start_ammo: 0 },
    ItemRow { item: ELECTROLYZER, attach: 0, class: ELECTROLYZER_CLASS, stance: 1, icon: 0xea81, use_seq: 0, keep: false, fire_seq: [0, 0], speed: 0.0, rate: 0.0, damage: 0.0, ammo: false, max_ammo: 0, start_ammo: 0 },
    ItemRow { item: SPIDERBOT, attach: 6, class: SPIDERBOT_CLASS, stance: 0, icon: 0xea6e, use_seq: 0x2c, keep: false, fire_seq: [0x42, 0x2c], speed: 0.0, rate: 0.0, damage: 12.0, ammo: true, max_ammo: 8, start_ammo: 8 },
    ItemRow { item: SWINGSHOT, attach: 1, class: SWINGSHOT_CLASS, stance: 1, icon: 0xea8b, use_seq: 0, keep: false, fire_seq: [0, 0], speed: 0.0, rate: 0.0, damage: 0.0, ammo: false, max_ammo: 0, start_ammo: 0 },
    ItemRow { item: CHOPPER, attach: 6, class: CHOPPER_CLASS, stance: 0, icon: 0xea66, use_seq: 0x8f, keep: false, fire_seq: [0x8f, 0x8f], speed: 30.0, rate: 1.8, damage: 1.0, ammo: true, max_ammo: 35, start_ammo: 35 },
    ItemRow { item: BLITZ, attach: 0, class: BLITZ_CLASS, stance: 2, icon: 0xea67, use_seq: 0x89, keep: false, fire_seq: [0x8a, -1], speed: 0.0, rate: 2.0, damage: 3.0, ammo: true, max_ammo: 40, start_ammo: 40 },
    ItemRow { item: PULSE, attach: 0, class: PULSE_CLASS, stance: 2, icon: 0xea68, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 400.0, rate: 1.0, damage: 10.0, ammo: true, max_ammo: 8, start_ammo: 8 },
    ItemRow { item: MINI_TURRET, attach: 6, class: MINI_TURRET_CLASS, stance: 0, icon: 0xea6a, use_seq: 0x2c, keep: false, fire_seq: [0x42, 0x2c], speed: 30.0, rate: 6.0, damage: 0.7, ammo: true, max_ammo: 20, start_ammo: 20 },
    ItemRow { item: SYNTHENOID, attach: 6, class: crate::moby::classes::synthenoid::DRONE, stance: 0, icon: 0xea6d, use_seq: 0x2c, keep: false, fire_seq: [0x42, 0x2c], speed: 60.0, rate: 2.0, damage: 0.75, ammo: true, max_ammo: 12, start_ammo: 12 },
    ItemRow { item: LAVA, attach: 0, class: LAVA_CLASS, stance: 2, icon: 0xea69, use_seq: 0x36, keep: true, fire_seq: [-1, 0x46], speed: 0.0, rate: 0.0, damage: 6.0, ammo: true, max_ammo: 200, start_ammo: 200 },
    ItemRow { item: SEEKER, attach: 0, class: SEEKER_CLASS, stance: 1, icon: 0xea6b, use_seq: 0x33, keep: false, fire_seq: [-1, -1], speed: 12.0, rate: 2.0, damage: 6.0, ammo: true, max_ammo: 25, start_ammo: 25 },
    ItemRow { item: BOUNCER, attach: 0, class: BOUNCER_CLASS, stance: 2, icon: 0xea73, use_seq: 0x89, keep: false, fire_seq: [-1, -1], speed: 0.0, rate: 0.3, damage: 6.0, ammo: true, max_ammo: 25, start_ammo: 25 },
    ItemRow { item: MINIROCKET, attach: 0, class: MINIROCKET_CLASS, stance: 2, icon: 0xea70, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 60.0, rate: 1.7, damage: 12.0, ammo: true, max_ammo: 25, start_ammo: 25 },
    ItemRow { item: PLASMA_COIL, attach: 0, class: PLASMA_COIL_CLASS, stance: 1, icon: 0xea71, use_seq: 0x38, keep: true, fire_seq: [-1, 0x3e], speed: 15.0, rate: 1.0, damage: 18.0, ammo: true, max_ammo: 15, start_ammo: 15 },
    ItemRow { item: HOVERBOMB, attach: 0, class: HOVERBOMB_CLASS, stance: 2, icon: 0xea72, use_seq: 0x89, keep: false, fire_seq: [-1, -1], speed: 7.0, rate: 3.0, damage: 30.0, ammo: true, max_ammo: 10, start_ammo: 10 },
    ItemRow { item: SHIELD, attach: 0, class: crate::moby::classes::shield::CLASS, stance: 1, icon: 0xea74, use_seq: 0x38, keep: true, fire_seq: [0x39, 0x3e], speed: 0.0, rate: 0.0, damage: 1.0, ammo: true, max_ammo: 5, start_ammo: 5 },
    ItemRow { item: ZODIAC, attach: 0, class: ZODIAC_CLASS, stance: 1, icon: 0xea76, use_seq: 0x94, keep: false, fire_seq: [-1, 0x3e], speed: 0.0, rate: 0.25, damage: 100.0, ammo: true, max_ammo: 4, start_ammo: 4 },
    ItemRow { item: RYNO, attach: 0, class: RYNO_CLASS, stance: 2, icon: 0xea6c, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 0.0, rate: 6.5, damage: 100.0, ammo: true, max_ammo: 100, start_ammo: 100 },
    ItemRow { item: SHEEPINATOR, attach: 0, class: SHEEPINATOR_CLASS, stance: 1, icon: 0xea77, use_seq: 0x38, keep: true, fire_seq: [-1, 0x3e], speed: 0.0, rate: 0.0, damage: 0.0, ammo: false, max_ammo: 0, start_ammo: 0 },
    ItemRow { item: ZAPPER, attach: 5, class: crate::moby::classes::zapper::ZAP, stance: 0, icon: 0xea99, use_seq: -1, keep: false, fire_seq: [-1, -1], speed: 30.0, rate: 2.4, damage: 16.0, ammo: true, max_ammo: 30, start_ammo: 10 },
];

/// The upgraded rows the port has (L00 `0x262500 + 0xe0·row`; their item in brackets): the Heavy Lancer 0x3c and the
/// Mega rows 0x4f / 0x50 (0x1e), the Mini-Nuke 0x47, 0x51, 0x52 (0x2a), the Spiderbot Glove's 0x4e, 0x69, 0x6a (0x20),
/// the Mega Chopper 0x41, 0x53, 0x54 (0x16), the upgraded Blitz Gun 0x44, 0x5b, 0x5c (0x1a), the upgraded Mini-Turret
/// Glove 0x40, 0x59, 0x5a (0x29: L00 `0x265d00`, `0x2672e0`, `0x2673c0`), the Vaporizer 0x42, 0x57, 0x58 (0x17: L00
/// `0x265ec0`, `0x267120`, `0x267200`), the upgraded Seeker Gun 0x43, 0x55, 0x56 (0x18: L00 `0x265fa0`, `0x266f60`,
/// `0x267040`), the Meteor Gun 0x3f, 0x5f, 0x60 (0x1d: L00 `0x265c20`, `0x267820`, `0x267900`), the upgraded Bouncer 0x4c,
/// 0x61, 0x62 (0x25: L00 `0x266780`, `0x2679e0`, `0x267ac0`), the Megarocket Cannon 0x45, 0x63, 0x64 (0x1b: L00
/// `0x266160`, `0x267ba0`, `0x267c80`), the Tetrabomb Gun 0x46, 0x67, 0x68 (0x19: L00 `0x266240`, `0x267f20`, `0x268000`),
/// the Sheepinator's challenge-mode Mega row 0x6d (0x10: L00 `0x268460`, the base hand 0xa5d; its 0x48 "Black
/// Sheepinator" and 0x6e, the hand 0xead `0x406cf8`, are not ported), the Plasma Storm 0x3e, 0x65, 0x66 (0x1c: L00
/// `0x265b40`, `0x267d60`, `0x267e40`), the upgraded Synthenoids 0x3d, 0x5d, 0x5e (0x1f: L00 `0x265a60`, `0x267660`,
/// `0x267740`; their class the drone's, crate::moby::classes::synthenoid), the upgraded Shield Charger 0x4d, 0x6b, 0x6c
/// (0x2d: L00 `0x266860`, `0x2682a0`, `0x268380`; their class the shield's, crate::moby::classes::shield).
/// Each names its own hand class (`+0x04`).
pub const UPGRADE_ROWS: [ItemRow; 49] = [
    ItemRow { item: 0x3c, attach: 0, class: 0xc05, stance: 2, icon: 0xea64, use_seq: 0x33, keep: true, fire_seq: [-1, -1], speed: 80.0, rate: 10.0, damage: 0.67, ammo: true, max_ammo: 300, start_ammo: 300 },
    ItemRow { item: 0x4f, attach: 0, class: 0xc05, stance: 2, icon: 0xea64, use_seq: 0x33, keep: true, fire_seq: [-1, -1], speed: 80.0, rate: 10.0, damage: 4.0, ammo: true, max_ammo: 400, start_ammo: 300 },
    ItemRow { item: 0x50, attach: 0, class: 0xc05, stance: 2, icon: 0xea64, use_seq: 0x33, keep: true, fire_seq: [-1, -1], speed: 80.0, rate: 10.0, damage: 8.0, ammo: true, max_ammo: 500, start_ammo: 300 },
    ItemRow { item: 0x47, attach: 0, class: 0xa5a, stance: 2, icon: 0xea65, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 7.0, rate: 0.5, damage: 0.0, ammo: true, max_ammo: 8, start_ammo: 8 },
    ItemRow { item: 0x51, attach: 0, class: 0xa5a, stance: 2, icon: 0xea65, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 7.0, rate: 0.5, damage: 0.0, ammo: true, max_ammo: 20, start_ammo: 8 },
    ItemRow { item: 0x52, attach: 0, class: 0xa5a, stance: 2, icon: 0xea65, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 7.0, rate: 0.5, damage: 0.0, ammo: true, max_ammo: 30, start_ammo: 8 },
    ItemRow { item: 0x4e, attach: 6, class: 0xe7a, stance: 0, icon: 0xea6e, use_seq: 0x2c, keep: false, fire_seq: [0x42, 0x2c], speed: 0.0, rate: 0.0, damage: 18.0, ammo: true, max_ammo: 8, start_ammo: 8 },
    ItemRow { item: 0x69, attach: 6, class: 0xe7a, stance: 0, icon: 0xea6e, use_seq: 0x2c, keep: false, fire_seq: [0x42, 0x2c], speed: 0.0, rate: 0.0, damage: 30.0, ammo: true, max_ammo: 16, start_ammo: 8 },
    ItemRow { item: 0x6a, attach: 6, class: 0xe7a, stance: 0, icon: 0xea6e, use_seq: 0x2c, keep: false, fire_seq: [0x42, 0x2c], speed: 0.0, rate: 0.0, damage: 100.0, ammo: true, max_ammo: 24, start_ammo: 8 },
    ItemRow { item: 0x41, attach: 6, class: 0xaf6, stance: 0, icon: 0xea66, use_seq: 0x8f, keep: false, fire_seq: [0x8f, 0x8f], speed: 30.0, rate: 1.8, damage: 2.0, ammo: true, max_ammo: 35, start_ammo: 35 },
    ItemRow { item: 0x53, attach: 6, class: 0xaf6, stance: 0, icon: 0xea66, use_seq: 0x8f, keep: false, fire_seq: [0x8f, 0x8f], speed: 30.0, rate: 1.8, damage: 5.0, ammo: true, max_ammo: 50, start_ammo: 35 },
    ItemRow { item: 0x54, attach: 6, class: 0xaf6, stance: 0, icon: 0xea66, use_seq: 0x8f, keep: false, fire_seq: [0x8f, 0x8f], speed: 30.0, rate: 1.8, damage: 8.0, ammo: true, max_ammo: 70, start_ammo: 35 },
    ItemRow { item: 0x44, attach: 0, class: 0xaf1, stance: 2, icon: 0xea67, use_seq: 0x89, keep: false, fire_seq: [0x8a, -1], speed: 0.0, rate: 2.6, damage: 6.0, ammo: true, max_ammo: 40, start_ammo: 40 },
    ItemRow { item: 0x5b, attach: 0, class: 0xaf1, stance: 2, icon: 0xea67, use_seq: 0x89, keep: false, fire_seq: [0x8a, -1], speed: 0.0, rate: 2.6, damage: 20.0, ammo: true, max_ammo: 50, start_ammo: 40 },
    ItemRow { item: 0x5c, attach: 0, class: 0xaf1, stance: 2, icon: 0xea67, use_seq: 0x89, keep: false, fire_seq: [0x8a, -1], speed: 0.0, rate: 2.6, damage: 40.0, ammo: true, max_ammo: 60, start_ammo: 40 },
    ItemRow { item: 0x40, attach: 6, class: crate::moby::classes::turret::UP_HAND, stance: 0, icon: 0xea6a, use_seq: 0x2c, keep: false, fire_seq: [0x42, 0x2c], speed: 30.0, rate: 8.0, damage: 4.1, ammo: true, max_ammo: 20, start_ammo: 20 },
    ItemRow { item: 0x59, attach: 6, class: crate::moby::classes::turret::UP_HAND, stance: 0, icon: 0xea6a, use_seq: 0x2c, keep: false, fire_seq: [0x42, 0x2c], speed: 30.0, rate: 1.5, damage: 8.2, ammo: true, max_ammo: 30, start_ammo: 20 },
    ItemRow { item: 0x5a, attach: 6, class: crate::moby::classes::turret::UP_HAND, stance: 0, icon: 0xea6a, use_seq: 0x2c, keep: false, fire_seq: [0x42, 0x2c], speed: 30.0, rate: 1.5, damage: 18.0, ammo: true, max_ammo: 50, start_ammo: 20 },
    ItemRow { item: 0x42, attach: 0, class: crate::moby::classes::vaporizer::HAND_CLASS, stance: 2, icon: 0xea68, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 400.0, rate: 1.0, damage: 20.0, ammo: true, max_ammo: 8, start_ammo: 8 },
    ItemRow { item: 0x57, attach: 0, class: crate::moby::classes::vaporizer::HAND_CLASS, stance: 2, icon: 0xea68, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 400.0, rate: 1.0, damage: 40.0, ammo: true, max_ammo: 12, start_ammo: 8 },
    ItemRow { item: 0x58, attach: 0, class: crate::moby::classes::vaporizer::HAND_CLASS, stance: 2, icon: 0xea68, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 400.0, rate: 1.0, damage: 80.0, ammo: true, max_ammo: 20, start_ammo: 8 },
    ItemRow { item: 0x43, attach: 0, class: crate::moby::classes::seeker::PLUS_CLASS, stance: 1, icon: 0xea6b, use_seq: 0x33, keep: false, fire_seq: [-1, -1], speed: 14.0, rate: 2.0, damage: 6.0, ammo: true, max_ammo: 20, start_ammo: 20 },
    ItemRow { item: 0x55, attach: 0, class: crate::moby::classes::seeker::PLUS_CLASS, stance: 1, icon: 0xea6b, use_seq: 0x33, keep: false, fire_seq: [-1, -1], speed: 14.0, rate: 2.0, damage: 16.0, ammo: true, max_ammo: 40, start_ammo: 20 },
    ItemRow { item: 0x56, attach: 0, class: crate::moby::classes::seeker::PLUS_CLASS, stance: 1, icon: 0xea6b, use_seq: 0x33, keep: false, fire_seq: [-1, -1], speed: 14.0, rate: 2.0, damage: 24.0, ammo: true, max_ammo: 50, start_ammo: 20 },
    ItemRow { item: 0x3f, attach: 0, class: crate::moby::classes::meteor::HAND_CLASS, stance: 2, icon: 0xea69, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 0.0, rate: 4.5, damage: 8.0, ammo: true, max_ammo: 200, start_ammo: 200 },
    ItemRow { item: 0x5f, attach: 0, class: crate::moby::classes::meteor::HAND_CLASS, stance: 2, icon: 0xea69, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 0.0, rate: 7.5, damage: 12.0, ammo: true, max_ammo: 300, start_ammo: 200 },
    ItemRow { item: 0x60, attach: 0, class: crate::moby::classes::meteor::HAND_CLASS, stance: 2, icon: 0xea69, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 0.0, rate: 7.5, damage: 24.0, ammo: true, max_ammo: 400, start_ammo: 200 },
    ItemRow { item: 0x4c, attach: 0, class: crate::moby::classes::bouncer::UPGRADED_HAND, stance: 2, icon: 0xea73, use_seq: 0x89, keep: false, fire_seq: [-1, -1], speed: 0.0, rate: 0.3, damage: 12.0, ammo: true, max_ammo: 25, start_ammo: 25 },
    ItemRow { item: 0x61, attach: 0, class: crate::moby::classes::bouncer::UPGRADED_HAND, stance: 2, icon: 0xea73, use_seq: 0x89, keep: false, fire_seq: [0x8a, 0x8a], speed: 0.0, rate: 0.3, damage: 24.0, ammo: true, max_ammo: 40, start_ammo: 25 },
    ItemRow { item: 0x62, attach: 0, class: crate::moby::classes::bouncer::UPGRADED_HAND, stance: 2, icon: 0xea73, use_seq: 0x89, keep: false, fire_seq: [0x8a, 0x8a], speed: 0.0, rate: 0.3, damage: 50.0, ammo: true, max_ammo: 60, start_ammo: 25 },
    ItemRow { item: 0x45, attach: 0, class: crate::moby::classes::megarocket::HAND_CLASS, stance: 2, icon: 0xea70, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 60.0, rate: 4.0, damage: 18.0, ammo: true, max_ammo: 25, start_ammo: 25 },
    ItemRow { item: 0x63, attach: 0, class: crate::moby::classes::megarocket::HAND_CLASS, stance: 2, icon: 0xea70, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 60.0, rate: 4.0, damage: 30.0, ammo: true, max_ammo: 40, start_ammo: 25 },
    ItemRow { item: 0x64, attach: 0, class: crate::moby::classes::megarocket::HAND_CLASS, stance: 2, icon: 0xea70, use_seq: 0x36, keep: false, fire_seq: [-1, 0x46], speed: 60.0, rate: 4.0, damage: 60.0, ammo: true, max_ammo: 60, start_ammo: 25 },
    ItemRow { item: 0x46, attach: 0, class: crate::moby::classes::hoverbomb::TETRA_HAND, stance: 2, icon: 0xea72, use_seq: 0x89, keep: false, fire_seq: [-1, -1], speed: 7.0, rate: 3.0, damage: 30.0, ammo: true, max_ammo: 10, start_ammo: 10 },
    ItemRow { item: 0x67, attach: 0, class: crate::moby::classes::hoverbomb::TETRA_HAND, stance: 2, icon: 0xea72, use_seq: 0x89, keep: false, fire_seq: [0x8a, 0x8a], speed: 7.0, rate: 3.0, damage: 30.0, ammo: true, max_ammo: 15, start_ammo: 10 },
    ItemRow { item: 0x68, attach: 0, class: crate::moby::classes::hoverbomb::TETRA_HAND, stance: 2, icon: 0xea72, use_seq: 0x89, keep: false, fire_seq: [0x8a, 0x8a], speed: 7.0, rate: 3.0, damage: 100.0, ammo: true, max_ammo: 25, start_ammo: 10 },
    ItemRow { item: 0x6d, attach: 0, class: SHEEPINATOR_CLASS, stance: 1, icon: 0xea77, use_seq: 0x38, keep: true, fire_seq: [-1, 0x3e], speed: 0.0, rate: 0.0, damage: 0.0, ammo: false, max_ammo: 0, start_ammo: 0 },
    // The Black Sheepinator (L00 `0x266400` / `0x268540`; crate::moby::classes::sheepinator::BLACK_CLASS).
    ItemRow { item: 0x48, attach: 0, class: crate::moby::classes::sheepinator::BLACK_CLASS, stance: 1, icon: 0xea77, use_seq: 0x38, keep: true, fire_seq: [0x38, 0x3e], speed: 0.0, rate: 0.0, damage: 0.0, ammo: false, max_ammo: 0, start_ammo: 0 },
    ItemRow { item: 0x6e, attach: 0, class: crate::moby::classes::sheepinator::BLACK_CLASS, stance: 1, icon: 0xea77, use_seq: 0x38, keep: true, fire_seq: [-1, 0x3e], speed: 0.0, rate: 0.0, damage: 0.0, ammo: false, max_ammo: 0, start_ammo: 0 },
    ItemRow { item: 0x3e, attach: 0, class: crate::moby::classes::plasma_storm::HAND_CLASS, stance: 2, icon: 0xea71, use_seq: 0, keep: true, fire_seq: [-1, -1], speed: 15.0, rate: 1.0, damage: 24.0, ammo: true, max_ammo: 15, start_ammo: 15 },
    ItemRow { item: 0x65, attach: 0, class: crate::moby::classes::plasma_storm::HAND_CLASS, stance: 2, icon: 0xea71, use_seq: 0, keep: true, fire_seq: [-1, -1], speed: 15.0, rate: 1.0, damage: 30.0, ammo: true, max_ammo: 25, start_ammo: 15 },
    ItemRow { item: 0x66, attach: 0, class: crate::moby::classes::plasma_storm::HAND_CLASS, stance: 2, icon: 0xea71, use_seq: 0, keep: true, fire_seq: [-1, -1], speed: 15.0, rate: 1.0, damage: 50.0, ammo: true, max_ammo: 50, start_ammo: 15 },
    ItemRow { item: 0x3d, attach: 6, class: crate::moby::classes::synthenoid::PLUS_DRONE, stance: 0, icon: 0xea6d, use_seq: 0x2c, keep: false, fire_seq: [0x42, 0x2c], speed: 80.0, rate: 3.0, damage: 0.5, ammo: true, max_ammo: 12, start_ammo: 12 },
    ItemRow { item: 0x5d, attach: 6, class: crate::moby::classes::synthenoid::PLUS_DRONE, stance: 0, icon: 0xea6d, use_seq: 0x2c, keep: false, fire_seq: [0x42, 0x2c], speed: 80.0, rate: 4.0, damage: 4.0, ammo: true, max_ammo: 20, start_ammo: 12 },
    ItemRow { item: 0x5e, attach: 6, class: crate::moby::classes::synthenoid::PLUS_DRONE, stance: 0, icon: 0xea6d, use_seq: 0x2c, keep: false, fire_seq: [0x42, 0x2c], speed: 80.0, rate: 4.0, damage: 8.0, ammo: true, max_ammo: 40, start_ammo: 12 },
    ItemRow { item: 0x4d, attach: 0, class: crate::moby::classes::shield::PLUS_CLASS, stance: 1, icon: 0xea74, use_seq: 0x38, keep: true, fire_seq: [0x39, 0x3e], speed: 0.0, rate: 1.0, damage: 5.0, ammo: true, max_ammo: 8, start_ammo: 5 },
    ItemRow { item: 0x6b, attach: 0, class: crate::moby::classes::shield::PLUS_CLASS, stance: 1, icon: 0xea74, use_seq: 0x38, keep: true, fire_seq: [0x39, 0x3e], speed: 0.0, rate: 1.0, damage: 24.0, ammo: true, max_ammo: 10, start_ammo: 5 },
    ItemRow { item: 0x6c, attach: 0, class: crate::moby::classes::shield::PLUS_CLASS, stance: 1, icon: 0xea74, use_seq: 0x38, keep: true, fire_seq: [0x39, 0x3e], speed: 0.0, rate: 1.0, damage: 64.0, ammo: true, max_ammo: 15, start_ammo: 5 },
    // The upgraded Clank Zapper (L00 `0x2664e0`): its class is Clank's back moby 0x259 (crate::hero::back).
    ItemRow { item: 0x49, attach: 5, class: crate::hero::back::CLANK_CLASS, stance: 0, icon: 0xea99, use_seq: -1, keep: false, fire_seq: [-1, -1], speed: 30.0, rate: 2.0, damage: 20.0, ammo: true, max_ammo: 30, start_ammo: 10 },
];

/// Row `row` of the records ([`ROWS`], [`UPGRADE_ROWS`]).
pub fn row_at(row: usize) -> Option<&'static ItemRow> { ROWS.iter().chain(UPGRADE_ROWS.iter()).find(|r| r.item as usize == row) }

/// Item `item`'s record: its current row (`0x139568[item]`, crate::upgrades).
pub fn row(game: &crate::game::GameState, item: i32) -> Option<&'static ItemRow> {
    if item < 0 { return None; }
    row_at(game.item_row(item))
}

/// The head item's moby class (slot 2, `0x18b0e0`: item 0x2f, the pilot's helmet; `0x2d5138` names it).
pub const HEAD_CLASS: i16 = 0x50a;
/// `0x1a7d08` + 4: the joints of Ratchet whose pose the head item's joints 1 and 2 take (`0x2aa278`).
pub const HEAD_JOINTS: [u8; 2] = [8, 10];
/// The head item's attachment (its row's word 0; `HERO_LISTS[4]`: joints 0..7, the head).
pub const HEAD_ATTACH: usize = 4;
/// The worn feet item (slot 1, `0x18b090` / `0x18b094`): `0x1a739c`, 0 for the default 0x34 (its row: attachment 2,
/// class 0xa73 for both feet).
pub const FEET_ITEM: i32 = 0x34;
/// `0x2b09e0`: the second foot sits at attachment 3 whatever the row says.
pub const FEET_SECOND_ATTACH: usize = 3;
/// `0x1a7d18` + 4 / `0x1a7d30` + 4: the joints of Ratchet whose pose each foot's joints 1..4 take (`0x2aa278`): his
/// left foot 100..103, his right 94..97.
pub const FEET_JOINTS: [[u8; 4]; 2] = [[100, 101, 102, 103], [94, 95, 96, 97]];

/// A throw request (`0x2ba610`): `look` thrown from the look stance (state 1 / 0x1d past 0x15 ticks), along the
/// view ([`throw_dir`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Throw {
    pub dir: V3,
    pub push: f32,
    pub look: bool,
}

/// The first-person weapon (`0x18b2c0..0x18b338`; `0x2d54f8`'s look-stance part and `0x2d5ce0`).
#[derive(Clone, Copy, Debug, Default)]
pub struct FpWeapon {
    /// `0x18b320`: drawn in first person.
    pub on: bool,
    /// `0x18b2c0`: the hand item's place when it started, in Ratchet's frame; `0x18b2d0` its rotation.
    pub start_pos: V3,
    pub start_q: crate::quat::Q,
    /// `0x18b318` / `0x18b31c`: the blend from there (0 → 1) and its speed.
    pub t: f32,
    pub t_v: f32,
    /// `0x18b328` / `0x18b324` / `0x18b32c`: the pull back toward the eye (≤ 0) where the weapon would meet the world,
    /// its speed, its target.
    pub pull: f32,
    pub pull_v: f32,
    pub pull_to: f32,
    /// `0x18b330` / `0x18b334`: the bob's phase and its speed.
    pub bob: f32,
    pub bob_v: f32,
    /// `0x18b2e0`: the rotation it was drawn at last (the blend back to the hand's).
    pub q: crate::quat::Q,
}

/// The item slot block (slot 0, `0x18b040..0x18b090`) and the flags the states set.
#[derive(Clone, Copy, Debug, Default)]
pub struct ItemBlock {
    /// `0x18b040`: the hand item's moby; its class.
    pub hand: Option<usize>,
    pub hand_class: i16,
    /// `0x18b068`: the item (10 = the wrench); `0x139568[item]`: its row as the last item pass saw it (the hero's
    /// tick has no game state, [`ItemBlock::held_row`]).
    pub item: i32,
    pub row: usize,
    /// `0x18b0e0`: the head item's moby ([`head_pass`]).
    pub head: Option<usize>,
    /// `0x18b090` / `0x18b094`: the worn feet's mobys ([`feet_pass`]).
    pub feet: [Option<usize>; 2],
    /// `0x18c250`: the thrown wrench's loop sound (its class sound 0xb).
    pub throw_sound: Option<i32>,
    /// `0x18b064`: 2 = in the hand, ready.
    pub ready: i32,
    /// `0x18b050`: the buttons that use it (the wrench: □ 0x80, the others ○ 0x20).
    pub trigger: u32,
    /// `0x18b059`: 2 while a swing can hit (the states set it; the wrench's update reads it).
    pub use_state: u8,
    /// `0x18b05a`: the wrench is thrown (out of the hand).
    pub thrown: u8,
    /// `0x18b05c`: the wrench's update says it is flying (2).
    pub flying: u8,
    /// `0x18c0dc`: states that keep the item away.
    pub no_use: u8,
    /// `0x18b060`: ticks the item has been in the hand.
    pub item_ticks: i32,
    /// `0x18b06c`: the hand item's glow phase ([`hand_glow`]).
    pub glow_phase: f32,
    /// Requests for the item pass: the hand item's sequence `(seq, frame, blend)` (`0x31b5f0`), the throw.
    pub hand_anim: Option<(u8, u8, u8)>,
    pub throw: Option<Throw>,
    /// `0x18c104`: the item to hold (0: the default, [`item_pass`]); `0x18c0d6`: the wrench asked for (the items
    /// locked away); `0x18b054`: the frame the next item may be created after.
    pub selected: i32,
    pub wrench_req: bool,
    /// `0x18c0e8`: an item asked for (the pause's Weapons page, `0x344dd0`; 0x77: none), taken by the next change.
    pub request: i32,
    pub next_create: i32,
    /// `0x18b058`: 0xff when the item has ammo and none is left.
    pub empty: u8,
    /// `0x2d43e0(item)`: the held item's ammo not out, as the last item pass left it (the hero's triggers read it:
    /// the port's hero tick has no game state).
    pub ammo_left: bool,
    /// The Chopper's trigger with no blades left: its class sound 0 at Ratchet (`0x360698(0, 0, Ratchet, class)`),
    /// played by the item pass.
    pub empty_click: bool,
    /// `0x18a01c` (s16) / `0x18c314`: the swap face's ticks and its kind (`0x2b1380`: the Blitz Gun and the items 0xe /
    /// 0x1b / 0x2b / 0x2c drawn give 0x46 ticks of kind 3; crate::hero::layers plays it; counted down by `0x2c3330`).
    pub swap_face: i16,
    pub swap_kind: u8,
    /// `0x18c0db`: the held item's stance type (its row's `+0x0c`: 1 one hand, 2 both; crate::hero::layers).
    pub stance: u8,
    /// `0x18c0df`: the hand item tucked away (some states' entries; every change clears it).
    pub tucked: bool,
    /// The use stance: `0x18c0d8` in use, `0x18c0da` it keeps Ratchet's stance, `0x18c2d0` his sequence, `0x18c2d4` the
    /// item, `0x189ff8` the ticks in use.
    pub using: bool,
    pub keep: bool,
    pub use_seq: i32,
    pub use_item: i32,
    pub use_ticks: i32,
    /// `0x18c354`: in the scripted look stance 0x1d, the ticks before a weapon may fire (0x28).
    pub look_wait: i32,
    /// The pad as the hero tick read it (`0x138320` held, `0x138324` pressed; □ pressed within 3 ticks,
    /// `0x330240(0x80, 3)`).
    pub held: u32,
    pub pressed: u32,
    pub square_recent: bool,
    /// `0x18aa18`: the moby a weapon aims at (its lock) and its aim height, set by the weapon's update each tick it
    /// holds one and read once by the posture (crate::hero::posture, `0x2abc18`).
    pub aim_target: Option<(usize, f32)>,
    /// A sequence for Ratchet `(blend, seq)` the use stance asks for (`0x2d2508`; the engine plays it after the pass).
    pub hero_anim: Option<(f32, u8)>,
    /// The first-person weapon ([`first_person_weapon`]).
    pub fp: FpWeapon,
    /// `0x18c30c`: the weapon arm raised at a wall (crate::hero::posture, `0x2ab250`).
    pub raised: bool,
    /// `0x18acb8` / `0x18acbc`: the firing arm's layers (lists 12 / 13, his own sequences), `0x18acc4` leaving them
    /// (1; 2 faster); the requests the item pass and SetState leave for crate::hero::layers: a firing layer to make
    /// `(sequence, blend, weight)`, the idle look started over (`0x2ab458`, its timer drawn 90..120: `0x18afc0`).
    pub fire: [Option<crate::hero::anim::AnimLayer>; 2],
    pub fire_leave: u8,
    pub fire_start: Option<(u8, i32, f32)>,
    pub use_reset: bool,
    /// `0x18b350` (16 × 3 rows), `0x18c1ec` / `0x18c1f0`: the ring of the hand attachment's last rows, its next slot
    /// and fill; `0x18c1f4`: how many the average takes ([`ItemBlock::smoothed`]).
    pub ring: [[V3; 3]; 16],
    pub ring_at: usize,
    pub ring_fill: usize,
    pub ring_n: i32,
}

impl ItemBlock {
    /// The held item's record (its row `0x139568[0x18b068]`, [`ItemBlock::row`]).
    pub fn held_row(&self) -> Option<&'static ItemRow> { if self.item <= 0 { None } else { row_at(self.row) } }

    /// `0x2b05a0`'s record: the attachment's rows (their columns made unit, [`unit_columns`]) into the ring.
    pub fn ring_push(&mut self, rows: [V3; 3]) {
        self.ring[self.ring_at] = rows;
        self.ring_at = (self.ring_at + 1) % 16;
        self.ring_fill = (self.ring_fill + 1).min(16);
    }

    /// `0x2b05a0(item, rows, 1)`: the rows an item of a stance takes — the average of the ring's last n (n stepping 1 a
    /// tick toward 11, 5 while running `run`; at least 2; at most 3 in the first 10 ticks of a use), the ones off from
    /// the others by more than a fifth of the most left out when one or two are (`0x2b04f8`: the summed differences of
    /// their entries); orthonormal through their Euler angles (`0x320000`). `None` while the ring holds fewer.
    pub fn smoothed(&mut self, run: bool) -> Option<[V3; 3]> {
        let target = if run { 5 } else { 0xb };
        if self.ring_n < target { self.ring_n += 1 } else if target < self.ring_n { self.ring_n -= 1 }
        let mut n = self.ring_n.max(2) as usize;
        if self.using && self.use_ticks < 10 && 3 < n { n = 3; }
        if self.ring_fill < n { return None; }
        let at = |i: usize| self.ring[(self.ring_at + 15 - i) % 16];
        let diff = |a: [V3; 3], b: [V3; 3]| (0..3).flat_map(|r| (0..3).map(move |c| (r, c))).map(|(r, c)| (a[r][c] - b[r][c]).abs()).sum::<f32>();
        let d: Vec<f32> = (0..n).map(|i| (0..n).filter(|&j| j != i).map(|j| diff(at(i), at(j))).sum()).collect();
        let most = d.iter().copied().fold(0.0f32, f32::max);
        let off = d.iter().filter(|&&x| most * 0.2 < x).count();
        let mut acc = [[0.0f32; 3]; 3];
        let mut used = 0;
        for (i, &x) in d.iter().enumerate() {
            if x <= most * 0.2 || off < 1 || 2 < off {
                let r = at(i);
                for k in 0..3 { for c in 0..3 { acc[k][c] += r[k][c]; } }
                used += 1;
            }
        }
        let acc = acc.map(|r| r.map(|x| x / used as f32));
        Some(crate::moby::attach::rows_of(crate::moby::attach::angles_of(&acc)))
    }

    /// `0x3227d8(item)` for a hand item (its record's slot kind `+0x0c` is 0): the hand item asked for (`0x18c0e8`),
    /// taken by the next change; 0 asks for none (0x77), the item already held asks for nothing. Only the items the
    /// hero can hold ([`ROWS`]); the head, feet and back slots (kinds 1..3) are not ported. True when asked.
    pub fn ask(&mut self, held: i32, item: i32) -> bool {
        let item = if item == 0 { 0x77 } else { item };
        if item == held {
            self.request = 0;
            true
        } else if item == 0x77 || row_at(item.max(0) as usize).is_some() {
            self.request = item;
            true
        } else {
            false
        }
    }

    /// `0x2d43e0(-1)`: the held item's ammo is not out (an item without ammo always can).
    pub fn has_ammo(&self, game: &crate::game::GameState) -> bool {
        match row(game, self.item) { Some(r) if r.ammo => game.ammo.get(self.item.max(0) as usize).is_some_and(|&a| a != 0), _ => true }
    }

    /// `0x2d42d0(-1, n)`: take `n` of the held item's ammo; false when there is too little.
    pub fn take_ammo(&self, game: &mut crate::game::GameState, n: i32) -> bool {
        if !row(game, self.item).is_some_and(|r| r.ammo) { return true; }
        let Some(a) = game.ammo.get_mut(self.item.max(0) as usize) else { return false };
        if n <= *a { *a -= n; true } else { false }
    }

    /// `0x2af5e0`: the held item in use, keeping the stance by its row (`+0x24`). Standing (group 0, the arm not raised
    /// at a wall, `0x18c30c`), Ratchet takes its sequence (`+0x18`) blended over 0xb ticks (stance type 0), 2 (sequence
    /// 0x36, items 0x24 / 0x1a), 5 (items 0x1e / 0x19 / 0x46) or SetAnim's −1; else the firing arm's layers play its
    /// moving sequence (`+0x1c`; crouched `+0x20`) over the same blend (−1: 0xb), on both arms with stance 2
    /// (crate::hero::layers). The idle look starts over (`0x2ab458`).
    pub fn use_start(&mut self, group: i32) {
        if self.ready != 2 { return; }
        let Some(r) = self.held_row() else { return };
        self.using = true;
        self.keep = r.keep;
        let blend = if r.stance == 0 {
            0xb
        } else if r.use_seq == 0x36 || matches!(self.item, 0x24 | 0x1a) {
            2
        } else if matches!(self.item, 0x1e | 0x19 | 0x46) {
            5
        } else {
            -1
        };
        self.use_item = self.item;
        self.use_reset = true;
        let layer = r.fire_seq[(group == 0xc) as usize];
        if group == 0 && !self.raised {
            self.use_seq = r.use_seq;
            self.hero_anim = Some((blend as f32, r.use_seq as u8));
        } else if layer != -1 {
            self.use_seq = layer;
            self.fire_start = Some((layer as u8, if blend < 0 { 0xb } else { blend }, 1.0));
        }
    }

    /// `0x2cb650`'s standing case: while a use keeps the stance and the hand item is still the one used, Ratchet's
    /// sequence is its use sequence (set again, blend −4, when it is not and no blend runs); another item ends it.
    pub fn use_stance(&mut self, state: i32, seq_b: u8, blending: bool, idle: u8) {
        if !self.using || blending || !self.keep || self.raised { return; }
        if self.ready == 2 && self.item == self.use_item {
            let s = self.held_row().map_or(0, |r| r.use_seq);
            if seq_b as i32 != s {
                if self.fire[0].is_some() { self.fire_leave = 1; }
                self.use_seq = s;
                self.hero_anim = Some((-4.0, s as u8));
            }
        } else {
            self.use_end(state, seq_b, idle);
        }
    }

    /// `0x2af848`: the use over; with the firing arm's layers out they leave, else standing (state 0) in the use's
    /// sequence Ratchet's idle comes back (blend −2).
    pub fn use_end(&mut self, state: i32, seq_b: u8, idle: u8) {
        if !self.using { return; }
        if self.fire[0].is_some() {
            self.fire_leave = 1;
        } else if state == 0 && seq_b as i32 == self.use_seq {
            self.hero_anim = Some((-2.0, idle));
        }
        self.keep = false;
        self.using = false;
    }
}

/// The items' part of the hero update `0x2a6dd0`: the item pass (`0x2b21f0`, [`hand_pass`]), then — under the view
/// lock `0x18c0d5` — his items hidden (`0x2d3208`: the hand, head and feet; the wrench in flight stays), and the
/// first-person weapon (`0x2d54f8`, [`first_person_weapon`]).
pub fn item_pass(world: &mut MobyWorld, ctx: &mut Ctx) {
    // `0x2a6dd0`'s head: the hand item's glow queue emptied (crate::fx::HeroGlow).
    world.hero_glows.clear();
    hand_pass(world, ctx);
    if ctx.hero.view_lock != 0 {
        let it = &ctx.hero.items;
        for id in [it.hand, it.head, it.feet[0], it.feet[1]].into_iter().flatten().chain(ctx.hero.back.mobys()) {
            if let Some(m) = world.mobys.get_mut(id) { m.mode |= 1; }
        }
        if let Some(m) = it.hand.filter(|_| it.item == WRENCH).and_then(|id| world.mobys.get_mut(id)) {
            if m.state != 0 { m.mode &= !0x41; }
        }
    }
    first_person_weapon(world, ctx);
}

/// `0x31f858(m)`: each column of the 3×3 set to length 1 (`0x2ef138(1, ..)`). `0x2b09e0` does it to the hand
/// attachment (Ratchet's arm joints carry scale keys, the holding classes' too, so its rows are not unit) before the
/// ring records it (`0x2b05a0`) and before an item of no stance takes it as its matrix (moby `+0xc0`).
fn unit_columns(r: [V3; 3]) -> [V3; 3] {
    let mut out = r;
    for c in 0..3 {
        let l = (r[0][c] * r[0][c] + r[1][c] * r[1][c] + r[2][c] * r[2][c]).sqrt();
        if 0.0 < l {
            for row in out.iter_mut() { row[c] /= l; }
        }
    }
    out
}

/// After the hero's transitions (`0x2b21f0`): create the hand item when there is none ([`create_hand`]), place it at
/// its row's attachment of Ratchet while it is in the hand (`0x2b09e0`: the rebuild, then the attachment matrix for its
/// rows, mode 6: out of the moby loop), carry out the states' requests, test for a change ([`switch`]) and run its
/// update (`0x2b2060`: the item slots' updates are called here, not by the moby loop; the item tick `0x18b060`
/// counts).
fn hand_pass(world: &mut MobyWorld, ctx: &mut Ctx) {
    // The body's hits of this tick (the Thruster-Pack's ram).
    for h in std::mem::take(&mut ctx.hero.hits_out) {
        let t = crate::moby::hits::HitTemplate::new(h.damage, ctx.hero_moby, h.flags, [h.dir[0], h.dir[1], h.dir[2], 0.0]);
        let mut c = h.centre;
        if let (Some([a, b]), Some(hm)) = (h.joints, ctx.hero_moby) {
            if let (Some(pa), Some(pb)) = (ctx.host.joint(&world.mobys[hm], a), ctx.host.joint(&world.mobys[hm], b)) {
                c = [(pa[0] + pb[0]) * 0.5, (pa[1] + pb[1]) * 0.5, (pa[2] + pb[2]) * 0.5];
            }
        }
        world.sphere_hit(ctx.scene, c, h.r, 0, ctx.hero_moby, Some(&t));
    }
    // `0x2b21f0`'s head: in water (`0x2adff8`: the swim groups 0x11 / 0x12, the states 0x67 / 0x7f / 0x72 / 0x73),
    // the groups 0x15 / 0x16 / 0x1f keep the items away and the △ menu shut (`0x18c0dc`, `0x18c0d7`); the wrench
    // is put away (`0x18c0de`) in water, on a ledge (group 3) and in the states 0x31 / 0x12 / 0x82.
    let h = &mut *ctx.hero;
    let in_water = (0x11..=0x12).contains(&h.group) || matches!(h.state, 0x67 | 0x7f | 0x72 | 0x73);
    if matches!(h.group, 0x15 | 0x16 | 0x1f) || in_water {
        h.items.no_use = 1;
        h.no_menu = 1;
    }
    let away = in_water || h.group == 3 || matches!(h.state, 0x31 | 0x12 | 0x82);
    head_pass(world, ctx);
    feet_pass(world, ctx);
    // Slot 3: Clank on the back and the pack (crate::hero::back).
    crate::hero::back::back_pass(world, ctx);
    // `0x2b2408` (from the hero update `0x2a6dd0`): the Clank Zapper's zaps, from the point on Clank's head.
    crate::moby::classes::zapper::fire(world, ctx);
    look_kind(world, ctx);
    create_hand(world, ctx);
    {
        // The held item's row (`0x139568[0x18b068]`) for the hero's tick, and what `0x3228f8` reads of the hand
        // (crate::upgrades::hitter_item).
        let it = &mut ctx.hero.items;
        it.row = ctx.game.item_row(it.item);
        world.hitters.refresh(ctx.game, it.hand, it.item);
    }
    let Some(id) = ctx.hero.items.hand else {
        switch(world, ctx);
        return;
    };
    if let Some((seq, frame, blend)) = ctx.hero.items.hand_anim.take() { ctx.host.set_anim(&world.mobys[id], seq, frame, blend); }
    if let Some(mut t) = ctx.hero.items.throw.take() {
        let rows = throw_dir(world, ctx, &mut t);
        let m = &mut world.mobys[id];
        m.pvars.resize(crate::moby::PVAR_BYTES, 0);
        crate::moby::classes::wrench::start_throw(m, t);
        m.rows = rows;
        let f = rows[0];
        m.rot = [0.0, -f[2].atan2((f[0] * f[0] + f[1] * f[1]).sqrt()), f[1].atan2(f[0])];
        ctx.host.set_anim(&world.mobys[id], 6, 0, 5);
    }
    let it = &mut ctx.hero.items;
    // `0x313420`: the hand item is out of the moby loop; its placement `0x2b09e0` advances its animation.
    ctx.host.advance(&world.mobys[id]);
    if it.thrown == 0 {
        let attach = it.held_row().map_or(0, |r| r.attach);
        if let Some((rows, mut pos)) = ctx.host.hero_attach(attach) {
            let stance = it.held_row().is_some_and(|r| r.stance != 0);
            // `0x2b05a0`: the attachment's rows into the ring; an item of a stance turns as their recent average.
            let rows = unit_columns(rows);
            it.ring_push(rows);
            let mut out = if stance { it.smoothed(ctx.hero.state == 2).unwrap_or(world.mobys[id].rows) } else { rows };
            // `0x2b09e0`: out of first person the weapon blends back from where it was drawn there (`0x18b318` eased to
            // 0: 1/240 a tick, at most 1/6; the place in Ratchet's frame `0x18b2c0`, the rotation `0x18b2e0`).
            let fp = &mut ctx.hero.items.fp;
            if stance && 0.0 < fp.t && !fp.on {
                let (mut t, mut v) = (fp.t, fp.t_v);
                crate::hero::physics::ease(0.0, 15.0 / 3600.0, 15.0 / 3600.0, 10.0 / 60.0, &mut t, &mut v);
                (fp.t, fp.t_v) = (t, v);
                let (hp, hr, sp) = (ctx.hero.pos, ctx.hero.rows, fp.start_pos);
                let start = [0, 1, 2].map(|c| hp[c] + hr[0][c] * sp[0] + hr[1][c] * sp[1] + hr[2][c] * sp[2]);
                pos = [0, 1, 2].map(|c| pos[c] + (start[c] - pos[c]) * t);
                out = crate::quat::rows(crate::quat::nlerp(crate::quat::from_rows(&out), fp.q, t));
            }
            {
                let m = &mut world.mobys[id];
                m.pos = pos;
                m.mode |= mode::NO_UPDATE | mode::NO_MATRIX | mode::KEEP_ROWS;
                m.rows = out;
                // `+0xf4` / `+0xf8`: the pitch and yaw of the first row.
                let f = out[0];
                m.rot = [0.0, -f[2].atan2((f[0] * f[0] + f[1] * f[1]).sqrt()), f[1].atan2(f[0])];
            }
            world.rebuild(id, &*ctx.host);
        }
    }
    // `0x2d3408` (from Ratchet's update): the hand item shown, the wrench hidden while put away.
    {
        let m = &mut world.mobys[id];
        m.mode &= !1;
        if away && ctx.hero.items.item == WRENCH { m.mode |= 0x41; }
    }
    // `0x2af8d8`: a kept stance ends when the ammo runs out.
    {
        let h = &mut *ctx.hero;
        if h.items.using && h.items.keep && !h.items.has_ammo(ctx.game) {
            let idle = h.idle_seq();
            h.items.use_end(h.state, ctx.hero_view.seq_b, idle);
        }
        // `0x2c3330`: the ticks in use.
        h.items.use_ticks = if h.items.using { h.items.use_ticks + 1 } else { 0 };
    }
    hand_glow(world, ctx, id);
    // `0x2b2060`: ready (2), the change test then the update; changing (3), the old item goes (`0x2b12a8`).
    match ctx.hero.items.ready {
        2 => {
            ctx.hero.items.item_ticks += 1;
            switch(world, ctx);
            // Port: the row changed under the hand outside the upgrade screen (the vendor's Mega purchase, a dev
            // switch): the hand goes out as a change sends it (its sequence 2, ready 3: its update sees the put-away
            // this tick, the next pass deletes it) and the new row's class comes with the next creation.
            let it = &mut ctx.hero.items;
            if let (2, Some(id), Some(r)) = (it.ready, it.hand, it.held_row()) {
                if r.class != it.hand_class {
                    let idle = ctx.hero.idle_seq();
                    let it = &mut ctx.hero.items;
                    it.use_end(ctx.hero.state, ctx.hero_view.seq_b, idle);
                    ctx.host.set_anim(&world.mobys[id], 2, 0, 2);
                    ctx.hero.items.ready = 3;
                }
            }
            // Its sequence 0 (the draw-out: the Spiderbot Gloves' unfolding) run out, a class of more sequences goes
            // to its sequence 1 (`0x31b5f0(item, 1, 0, 2)`), so the draw-out plays once.
            if let Some(&id) = ctx.hero.items.hand.as_ref() {
                let m = &world.mobys[id];
                if ctx.host.anim(m).is_some_and(|a| a.flags & 2 != 0 && a.seq_b == 0) && ctx.host.seq_frames(m, 1).is_some() {
                    ctx.host.set_anim(&world.mobys[id], 1, 0, 2);
                }
            }
        }
        3 => {
            let it = &mut ctx.hero.items;
            it.next_create = ctx.game.frame + 2;
            it.ready = 0;
            it.item = 0;
            it.hand = None;
            if let Some(h) = it.throw_sound.take() { ctx.host.stop_sound(h); }
            world.delete(id);
            return;
        }
        _ => {}
    }
    if world.mobys[id].state < 0xfe {
        if let Some(f) = world.mobys[id].update { f(world, id, ctx); }
    }
    let it = &mut ctx.hero.items;
    it.ammo_left = it.has_ammo(ctx.game);
    if std::mem::take(&mut it.empty_click) {
        if let (Some(h), Some(r)) = (ctx.hero_moby, it.held_row()) { ctx.host.play_sound_of(&world.mobys[h], r.class, 0, 0); }
    }
}

/// `0x2a7ed8`: the hand item's glow, once it has been in the hand 0x17 ticks (not while hidden or in state 0x61): its
/// phase (`0x18b06c`) steps, its glow word (`+0x78`, its glow vertices' colour) pulses between two colours by `sin`,
/// and some items shed glow sprites of it (`0x33f550`: type 0x59, at an offset in their rows, pulled toward the
/// camera, 2 ticks), the alpha capped by the ticks in the hand − 15 ([`GLOWS`]). By item and its row's level
/// (`0x2624f4 + 0xe0·0x139568[item]`, crate::upgrades): the Lancer (0x1e) eases its word 0.15 a tick toward the
/// pulse — in use 0.22 and faster — and on the Heavy Lancer's rows it is the pulse of `0xff0020ff` .. `0xff104080`
/// after the ease; the Chopper (0x16) and the Spiderbot Glove (0x20) only pulse; the Gravity Bomb (0x2a) pulses and
/// sheds one sprite, its upgrade two.
///
/// The Blitz Gun (0x1a) only pulses. The Pulse Rifle (0x17) queues two glows at its joints 2 and 3 (crate::fx::HeroGlow).
///
/// The Lava Gun (0x1d) pulses `0xff00ffff` .. `0xff104080` and sheds a sprite at (0.06, −0.509, 0): 73500, capped
/// 0x30, in use faster (0.1076) at 57750 capped 0x28.
///
/// The Vaporizer (0x17 upgraded) queues a red glow at its joint 1 and sheds a red sprite; the Minirocket Tube (0x1b) glows
/// only on its upgraded rows (the Megarocket Cannon: `0xff20205f` .. `0xffdfdfdf`). The Meteor Gun (0x1d upgraded) pulses
/// slower and sheds its own sprite.
///
/// The Plasma Coil (0x1c) pulses `0xff202020` .. `0xffefefef`. The Hoverbomb Gun (0x19) pulses `0xff5f5f5f` ..
/// `0xffdfdfdf` and sheds a sprite at (0.05, −0.45, 0). The Zodiac (0x2b) pulses `0xff9f4f7f` .. `0xffdfdfdf`,
/// sheds a sprite and queues a red glow at its joint 0.
///
/// The RYNO II (0x2c) only pulses (`0xffdf6060` .. `0xffdfbfbf`).
///
/// The Sheepinator (0x10) pulses `0xff00bfef` .. `0xff104080` and sheds a sprite at (0.16, −0.41, 0).
///
/// Not ported: the other items' cases (0xe, 0x12: their weapons are not ported).
fn hand_glow(world: &mut MobyWorld, ctx: &mut Ctx, id: usize) {
    let it = &mut ctx.hero.items;
    if ctx.hero.state == 0x61 || world.mobys[id].mode & 1 != 0 || it.item_ticks < 0x17 { return; }
    let ramp = ((it.item_ticks - 0xf).clamp(0, 0x40) as u32) << 24 | 0xff_ffff;
    let cap = |c: u32| if (ramp as i32) < c as i32 { ramp } else { c };
    let lerp = crate::hud::lerp_rgba;
    let up = ctx.game.upgrade_level(it.item) != 0;
    let using = it.using;
    let pulse = |phase: &mut f32, step: f32, keys: [u32; 2]| {
        *phase = crate::hero::physics::add_rot(*phase, step);
        lerp(phase.sin() * 0.5 + 0.5, keys[0], keys[1])
    };
    // The sprites: (size, pull, offset, colour).
    let mut sprites: Vec<(f32, f32, V3, u32)> = Vec::new();
    match it.item {
        LANCER if !up => {
            let (step, keys, ease, c) = if using { (0.116_355_3, [0xff00_40ff, 0xff10_bebe], 0.22, 0x32ff_ffffu32) } else { (0.040_724_356, [0xff00_28c8, 0xff10_b4b4], 0.15, 0x38ff_ffff) };
            let target = pulse(&mut it.glow_phase, step, keys);
            let m = &mut world.mobys[id];
            m.glow = lerp(ease, m.glow, target);
            sprites.push((50_400.0, 0.15, [0.195, -0.317, 0.0], m.glow & cap(c)));
        }
        LANCER => {
            let (step, keys) = if using { (0.203_621_77, [0xff00_40ff, 0xff10_4080]) } else { (0.040_724_356, [0xff00_40aa, 0xff10_4046]) };
            let target = pulse(&mut it.glow_phase, step, keys);
            let m = &mut world.mobys[id];
            m.glow = lerp(0.15, m.glow, target);
            m.glow = lerp(it.glow_phase.sin() * 0.5 + 0.5, 0xff00_20ff, 0xff10_4080);
            sprites.push((77_700.0, 0.2, [-0.059, -0.516, 0.0], m.glow & cap(0x40ff_ffff)));
        }
        CHOPPER => {
            let (step, keys) = if up { (0.040_724_356, [0xffff_20bf, 0xffff_2070]) } else { (0.058_177_65, [0xffff_20ff, 0xffff_2090]) };
            world.mobys[id].glow = pulse(&mut it.glow_phase, step, keys);
        }
        // The Mini-Turret Glove's case 0x29 is the Spiderbot Glove's.
        SPIDERBOT | MINI_TURRET => {
            let step = if up { 0.040_724_356 } else { 0.049_451_005 };
            world.mobys[id].glow = pulse(&mut it.glow_phase, step, [0xff50_5050, 0xffef_efef]);
        }
        GRAVITY_BOMB if !up => {
            let g = pulse(&mut it.glow_phase, 0.052_359_887, [0xff00_ffff, 0xff10_4080]);
            world.mobys[id].glow = g;
            sprites.push((73_500.0, 0.15, [0.333, -0.05, 0.0], g & 0x35ff_ffff));
        }
        GRAVITY_BOMB => {
            let g = pulse(&mut it.glow_phase, 0.040_724_356, [0xff20_4090, 0xff80_4040]);
            world.mobys[id].glow = g;
            for z in [0.1263, -0.1263] { sprites.push((57_750.0, 0.15, [0.423, -0.0555, z], g & 0x40ff_ffff)); }
        }
        // The Hoverbomb Gun (case 0x19): a sprite at (0.05, −0.45, 0), 94500, pull 0.1, capped 0x30 (in use faster,
        // capped 0x20); its upgrade slower, at (0.05, −0.35, 0), 102900, pull 0.15, its alpha 0x15 uncapped.
        HOVERBOMB if !up => {
            let (step, c) = if using { (0.107_628_65, 0x20ff_ffffu32) } else { (0.063_995_41, 0x30ff_ffff) };
            let g = pulse(&mut it.glow_phase, step, [0xff5f_5f5f, 0xffdf_dfdf]);
            world.mobys[id].glow = g;
            sprites.push((94_500.0, 0.1, [0.05, -0.45, 0.0], g & cap(c)));
        }
        HOVERBOMB => {
            let g = pulse(&mut it.glow_phase, 0.046_542_123, [0xff5f_5f5f, 0xffdf_dfdf]);
            world.mobys[id].glow = g;
            sprites.push((102_900.0, f32::from_bits(0x3e19_999a), [0.05, -0.35, 0.0], g & 0x15ff_ffff));
        }
        // The Plasma Coil (case 0x1c): only pulses (its upgrade slower).
        PLASMA_COIL => {
            let step = if up { 0.040_724_356 } else { 0.058_177_65 };
            world.mobys[id].glow = pulse(&mut it.glow_phase, step, [0xff20_2020, 0xffef_efef]);
        }
        // The Zodiac (case 0x2b): a sprite at (−0.1245, −0.8, 0), 76650, pull 0.1, capped 0x20, and a red glow at its joint
        // 0 (`0x2a7e60(0.031, 0.03, joint, 0x421010ff)`).
        ZODIAC => {
            let g = pulse(&mut it.glow_phase, f32::from_bits(0x3d83_1007), [0xff9f_4f7f, 0xffdf_dfdf]);
            world.mobys[id].glow = g;
            sprites.push((76_650.0, 0.1, [f32::from_bits(0xbdfe_f9db), -0.8, 0.0], g & cap(0x20ff_ffff)));
            if let Some(point) = ctx.host.joint(&world.mobys[id], 0) { world.hero_glows.push(crate::fx::HeroGlow { size: 0.031, push: 0.03, point, rgba: 0x4210_10ff }); }
        }
        // The Blitz Gun (case 0x1a): only pulses.
        BLITZ => {
            let step = if up { 0.043_633_237 } else { 0.058_177_65 };
            world.mobys[id].glow = pulse(&mut it.glow_phase, step, [0xffdf_dfdf, 0xff20_205f]);
        }
        // The Pulse Rifle (case 0x17): two green glows at its joints 2 and 3 (`0x2a7e60(0.1, 0.03, joint, 0x3500ff10)`),
        // no word.
        // The Lava Gun (case 0x1d).
        LAVA if !up => {
            let (step, size, c) = if using { (0.107_628_65, 57_750.0, 0x28ff_ffffu32) } else { (0.058_177_65, 73_500.0, 0x30ff_ffff) };
            let g = pulse(&mut it.glow_phase, step, [0xff00_ffff, 0xff10_4080]);
            world.mobys[id].glow = g;
            sprites.push((size, 0.1, [0.06, -0.509, 0.0], g & cap(c)));
        }
        // The Meteor Gun (case 0x1d upgraded): slower, a sprite at (−0.07, −0.67, 0), 99750, pull 0.15, its alpha 0x30
        // uncapped.
        LAVA => {
            let g = pulse(&mut it.glow_phase, 0.040_724_356, [0xff00_ffff, 0xff10_4080]);
            world.mobys[id].glow = g;
            sprites.push((99_750.0, f32::from_bits(0x3e19_999a), [f32::from_bits(0xbd8f_5c29), f32::from_bits(0xbf2b_851f), 0.0], g & 0x30ff_ffff));
        }
        // The Seeker Gun (case 0x18): a sprite at (−0.158, −0.194, 0), 98700, pull 0.1, capped 0x30 (in use faster,
        // capped 0x28); its upgrade pulses greys and sheds 109200 at 0.15, its alpha 0x30 uncapped.
        SEEKER if !up => {
            let (step, c) = if using { (0.107_628_65, 0x28ff_ffffu32) } else { (0.058_177_65, 0x30ff_ffff) };
            let g = pulse(&mut it.glow_phase, step, [0xffff_2050, 0xffff_2080]);
            world.mobys[id].glow = g;
            sprites.push((98_700.0, 0.1, [-0.158, -0.194, 0.0], g & cap(c)));
        }
        SEEKER => {
            let g = pulse(&mut it.glow_phase, 0.046_542_123, [0xff5f_5f5f, 0xffef_efef]);
            world.mobys[id].glow = g;
            sprites.push((109_199.99, 0.15, [-0.158, -0.194, 0.0], g & 0x30ff_ffff));
        }
        PULSE if !up => {
            for j in [2, 3] {
                if let Some(point) = ctx.host.joint(&world.mobys[id], j) { world.hero_glows.push(crate::fx::HeroGlow { size: 0.1, push: 0.03, point, rgba: 0x3500_ff10 }); }
            }
            return;
        }
        // The Vaporizer (case 0x17 upgraded): a red glow at its joint 1 (`0x2a7e60(0.14, 0.03, joint, 0x7f1010ff)`) and a
        // sprite at (0.399, 0.035, 0), 42000, no pull, `0x7f1010ff` (no word, no cap).
        PULSE => {
            if let Some(point) = ctx.host.joint(&world.mobys[id], 1) { world.hero_glows.push(crate::fx::HeroGlow { size: 0.14, push: 0.03, point, rgba: 0x7f10_10ff }); }
            sprites.push((42_000.0, 0.0, [f32::from_bits(0x3ecc_49ba), f32::from_bits(0x3d0f_5c29), 0.0], 0x7f10_10ff));
        }
        // The Sheepinator (case 0x10): a sprite at (0.16, −0.41, 0), 94500, pull 0.1, capped 0x30 (in use faster, capped
        // 0x28); its upgraded rows slower, `0xff00bfff` .. `0xff104080`, 105000 at 0.15, its alpha 0x30 uncapped.
        SHEEPINATOR if !up => {
            let (step, c) = if using { (f32::from_bits(0x3ddc_6c69), 0x28ff_ffffu32) } else { (f32::from_bits(0x3d6e_4bb0), 0x30ff_ffff) };
            let g = pulse(&mut it.glow_phase, step, [0xff00_bfef, 0xff10_4080]);
            world.mobys[id].glow = g;
            sprites.push((94_500.0, 0.1, [f32::from_bits(0x3e23_d70a), f32::from_bits(0xbed1_eb85), 0.0], g & cap(c)));
        }
        SHEEPINATOR => {
            let g = pulse(&mut it.glow_phase, f32::from_bits(0x3d26_ce95), [0xff00_bfff, 0xff10_4080]);
            world.mobys[id].glow = g;
            sprites.push((105_000.0, f32::from_bits(0x3e19_999a), [f32::from_bits(0x3e23_d70a), f32::from_bits(0xbed1_eb85), 0.0], g & 0x30ff_ffff));
        }
        // The RYNO II (case 0x2c): only pulses.
        RYNO => {
            world.mobys[id].glow = pulse(&mut it.glow_phase, f32::from_bits(0x3d03_1007), [0xffdf_6060, 0xffdf_bfbf]);
        }
        // The Megarocket Cannon (case 0x1b): only the Minirocket Tube's upgraded rows glow; only pulses.
        MINIROCKET if up => {
            world.mobys[id].glow = pulse(&mut it.glow_phase, 0.040_724_356, [0xff20_205f, 0xffdf_dfdf]);
        }
        _ => return,
    }
    let m = &world.mobys[id];
    let (pos, draw_dist) = (m.pos, m.draw_dist);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    for (size, pull, at, colour) in sprites { crate::particles::glow(sys, &mut ctx.game.rng, size, pull, at, id, pos, draw_dist, false, colour, 2); }
}

/// `0x2aeb90`'s read of the enemy the head looks at (`0x18c2b0`): its health block's look kind (`+0x2c`).
fn look_kind(world: &MobyWorld, ctx: &mut Ctx) {
    let h = &mut *ctx.hero;
    h.layers.look = h.posture.look_at.and_then(|t| world.mobys.get(t)).filter(|m| m.mode & crate::moby::ai::AI != 0).map(|m| m.pv_b(crate::moby::ai::health(m) + 0x2c));
}

/// `0x2afc68`'s slot 0: with no hand item, once the frame passed `0x18b054` — the item to hold (`0x18c104`; else the
/// wrench when it was out, else the last weapon `0x1a7398`, else the wrench), its row's class: draw distance 0x20,
/// drawn, ready; the wrench takes □ (0x80) and its sequence 1, the others ○ (0x20); an item with ammo and none left
/// is marked empty.
fn create_hand(world: &mut MobyWorld, ctx: &mut Ctx) {
    let it = &mut ctx.hero.items;
    if it.hand.is_some() || ctx.game.frame <= it.next_create { return; }
    let mut item = it.selected;
    if item == 0 { item = if ctx.game.wrench_out || ctx.game.held == 0 { WRENCH } else { ctx.game.held }; }
    let Some(r) = row(ctx.game, item) else { return };
    if world.class(r.class).is_none() {
        if it.next_create != i32::MAX { eprintln!("rc_game::hero::items: item {item:#x}'s class {:#x} is not loaded: no hand item", r.class); }
        it.next_create = i32::MAX;
        return;
    }
    let id = world.create(r.class);
    let m = &mut world.mobys[id];
    m.draw_dist = 0x20;
    m.drawn = 1;
    if m.shine != 0 { m.shine = 0x10; }
    m.parent = ctx.hero_moby;
    it.hand = Some(id);
    it.hand_class = r.class;
    it.item = item;
    it.row = ctx.game.item_row(item);
    it.ready = 2;
    it.empty = if r.ammo && ctx.game.ammo.get(item as usize).is_some_and(|&a| a == 0) { 0xff } else { 0 };
    it.ammo_left = it.empty == 0;
    if item == WRENCH {
        it.trigger = 0x80;
        ctx.host.set_anim(&world.mobys[id], 1, 0, 1);
    } else {
        it.trigger = 0x20;
    }
    it.stance = r.stance;
}

/// `0x2b13c8(0)`: the hand item's change. Locked away (`0x18c0d7`), a weapon asks for the wrench; with the items
/// kept away (`0x18c0dc`) nothing changes unless that. □ within 3 ticks (or the wrench asked for) takes the wrench
/// out; ○ pressed brings the last weapon (`0x1a7398`) back. Not while the thrown wrench flies or a swing's window
/// (`0x18b059`, counted down here) is open. A change: the use ends, the wrench-out flag follows, the item plays its
/// sequence 2 and goes out on the next pass (ready 3).
fn switch(world: &mut MobyWorld, ctx: &mut Ctx) {
    let h = &mut *ctx.hero;
    let idle = h.idle_seq();
    let it = &mut h.items;
    if h.no_menu != 0 && it.item != WRENCH { it.wrench_req = true; }
    // The Synthenoids asked for (0x1f; 0x3d the upgraded) are sent out, not held (`0x3ee110`): locked
    // away with the wrench in the hand here, else past the items' lock below.
    if h.no_menu != 0 && it.item == WRENCH && matches!(it.request, SYNTHENOID | 0x3d) {
        it.request = 0;
        crate::moby::classes::synthenoid::deploy(world, ctx);
    }
    // The Shield Charger asked for (`0x3c4eb8`): its shield raised, not held (crate::moby::classes::shield).
    let it = &mut ctx.hero.items;
    if ctx.hero.no_menu != 0 && it.item == WRENCH && it.request == SHIELD {
        it.request = 0;
        crate::moby::classes::shield::deploy(world, ctx);
    }
    // The Clank Zapper asked for: on (crate::moby::classes::zapper, `0x2b13c8`'s item 9).
    let it = &mut ctx.hero.items;
    if ctx.hero.no_menu != 0 && it.item == WRENCH && it.request == ZAPPER {
        it.request = 0;
        crate::moby::classes::zapper::request(world, ctx);
    }
    let h = &mut *ctx.hero;
    let it = &mut h.items;
    if it.no_use != 0 && (!it.wrench_req || it.item == WRENCH) {
        it.wrench_req = false;
        return;
    }
    if matches!(it.request, SYNTHENOID | 0x3d) {
        it.request = 0;
        crate::moby::classes::synthenoid::deploy(world, ctx);
    }
    if ctx.hero.items.request == SHIELD {
        ctx.hero.items.request = 0;
        crate::moby::classes::shield::deploy(world, ctx);
    }
    if ctx.hero.items.request == ZAPPER {
        ctx.hero.items.request = 0;
        crate::moby::classes::zapper::request(world, ctx);
    }
    let h = &mut *ctx.hero;
    let it = &mut h.items;
    let mut changed = false;
    if (it.square_recent || it.wrench_req) && it.item != WRENCH {
        it.selected = WRENCH;
        changed = true;
    }
    if it.pressed & crate::pad::button::CIRCLE != 0 && h.no_menu == 0 && ctx.game.held != it.item && ctx.game.held != 0 {
        it.selected = ctx.game.held;
        changed = true;
    }
    if it.request != 0 {
        if it.request == it.selected {
            it.request = 0;
        } else {
            it.selected = if it.request == 0x77 { 0 } else { it.request };
            ctx.game.held = it.selected;
            changed = true;
        }
    }
    if it.flying == 2 { changed = false; }
    if it.use_state != 0 {
        it.use_state -= 1;
        if it.use_state != 0 { changed = false; }
    }
    if !changed { return; }
    it.request = 0;
    it.item_ticks = 3;
    // `0x2b1380(item)`: the swap face for the items that have one.
    if matches!(it.selected, 0xe | BLITZ | 0x1b | 0x2b | 0x2c) {
        it.swap_face = 0x46;
        it.swap_kind = 3;
    }
    it.use_end(h.state, ctx.hero_view.seq_b, idle);
    ctx.game.wrench_out = it.selected == WRENCH;
    if it.selected != WRENCH { ctx.game.held = it.selected; }
    it.thrown = 0;
    it.wrench_req = false;
    if let Some(id) = it.hand { ctx.host.set_anim(&world.mobys[id], 2, 0, 2); }
    it.ready = 3;
    // A new stance type (`0x18c0db` against the new item's row `+0x0c`): the holding layers go for good (`+0x34`).
    if row(ctx.game, it.selected).map_or(0, |r| r.stance) != it.stance {
        for l in h.layers.hold.iter_mut().flatten() { l.kill = true; }
    }
}

/// The worn feet (slot 1): `0x2afc68` creates both when there are none — the item [`FEET_ITEM`]'s row's classes
/// `+0x04` / `+0x08` (draw distance 0x20, drawn, lit as Ratchet, ready) — not in armour 4 or with a body cheat, which
/// delete them (crate::cheats::worn_hidden). `0x2b09e0` then puts the first at the row's attachment and the second at
/// [`FEET_SECOND_ATTACH`], each posed from his feet ([`FEET_JOINTS`], `0x2aa278`: the engine draws them so). While they
/// are worn his own feet are scaled to 0.01 (`0x2ac218`, crate::hero::posture).
///
/// Not ported: the Charge Boots' feet (the item `0x1a739c` = 0x36).
pub fn feet_pass(world: &mut MobyWorld, ctx: &mut Ctx) {
    let Some(row) = ctx.level.items.get(FEET_ITEM as usize).copied() else { return };
    let it = &mut ctx.hero.items;
    for (k, slot) in it.feet.iter_mut().enumerate() {
        let class = row.classes[k] as i16;
        if slot.is_some_and(|id| world.mobys.get(id).is_none_or(|m| m.state >= 0xfe || m.o_class != class)) { *slot = None; }
    }
    // `0x2afc68`: armour 4 or a body cheat deletes both (`0x311688`) and makes none.
    if ctx.game.worn_hidden() {
        for slot in it.feet.iter_mut() {
            if let Some(id) = slot.take() { world.delete(id); }
        }
        return;
    }
    if it.feet[0].is_none() {
        for k in 0..2 {
            let class = row.classes[k] as i16;
            if class == 0 || world.class(class).is_none() { continue; }
            let id = world.create(class);
            let m = &mut world.mobys[id];
            m.draw_dist = 0x20;
            m.drawn = 1;
            if m.shine != 0 { m.shine = 0x10; }
            it.feet[k] = Some(id);
        }
    }
    let light = ctx.hero_moby.and_then(|h| world.mobys.get(h)).map(|h| (h.light, h.ambient));
    for k in 0..2 {
        let Some(id) = ctx.hero.items.feet[k] else { continue };
        let attach = if k == 1 { FEET_SECOND_ATTACH } else { row.attach.max(0) as usize };
        let Some((rows, pos)) = ctx.host.hero_attach(attach) else { continue };
        {
            let m = &mut world.mobys[id];
            m.parent = ctx.hero_moby;
            (m.pos, m.rows) = (pos, rows);
            m.mode = (m.mode | mode::NO_UPDATE | mode::NO_MATRIX | mode::KEEP_ROWS) & !1;
            let f = rows[0];
            m.rot = [0.0, -f[2].atan2((f[0] * f[0] + f[1] * f[1]).sqrt()), f[1].atan2(f[0])];
            if let Some((l, a)) = light { (m.light, m.ambient) = (l, a); }
        }
        world.rebuild(id, &*ctx.host);
    }
}

/// The head item (slot 2): `0x2afc68` creates it when there is none (item `0x1a73a0`, else 0x2f: class
/// [`HEAD_CLASS`]; draw distance 0x20, drawn, ready), in play and, through `0x2d4fc8`, in a scene (on the scene's
/// Ratchet: the host's hero pose). Not in armour 4 or with a body cheat (`0x1a7a51` / `0x4a` / `0x53` / `0x52`), which
/// delete it (crate::cheats::worn_hidden). `0x2b09e0` then puts it at Ratchet's attachment [`HEAD_ATTACH`] (his head), out of the
/// moby loop; its pose is built from his (`0x2aa278`: joint 0 at rest, joints 1 and 2 his [`HEAD_JOINTS`]; the
/// engine draws it so). Shown (`0x2d4fc8` clears bit 0 in a scene). Its visor (`0x2b13c8`, slot 2: `+0x7c` = 1, its
/// piece 0 drawn on top, crate::moby::Moby::bangles) is down while Ratchet swims (movement group 0x11), in the
/// states 0x73 / 0x67 / 0x7f, or in one of the tutorial's no-item areas (`0x18c310`: outside the ship).
pub fn head_pass(world: &mut MobyWorld, ctx: &mut Ctx) {
    let it = &mut ctx.hero.items;
    if it.head.is_some_and(|id| world.mobys.get(id).is_none_or(|m| m.state >= 0xfe || m.o_class != HEAD_CLASS)) { it.head = None; }
    // `0x2afc68`: armour 4 or a body cheat deletes it (`0x311688`) and makes none.
    if ctx.game.worn_hidden() {
        if let Some(id) = it.head.take() { world.delete(id); }
        return;
    }
    if it.head.is_none() && world.class(HEAD_CLASS).is_some() {
        let id = world.create(HEAD_CLASS);
        let m = &mut world.mobys[id];
        m.draw_dist = 0x20;
        m.drawn = 1;
        if m.shine != 0 { m.shine = 0x10; }
        it.head = Some(id);
    }
    let Some(id) = it.head else { return };
    let visor = ctx.hero.group == 0x11 || matches!(ctx.hero.state, 0x73 | 0x67 | 0x7f) || ctx.game.no_item_area;
    world.mobys[id].bangles = visor as u16;
    let Some((rows, pos)) = ctx.host.hero_attach(HEAD_ATTACH) else { return };
    {
        let m = &mut world.mobys[id];
        m.parent = ctx.hero_moby;
        (m.pos, m.rows) = (pos, rows);
        m.mode = (m.mode | mode::NO_UPDATE | mode::NO_MATRIX | mode::KEEP_ROWS) & !1;
        let f = rows[0];
        m.rot = [0.0, -f[2].atan2((f[0] * f[0] + f[1] * f[1]).sqrt()), f[1].atan2(f[0])];
    }
    world.rebuild(id, &*ctx.host);
}

/// `0x2ba610`'s aim, for the throw `t` taken by the item pass: the direction of flight and the wrench's rows. From the
/// look stance (`t.look`) along the view: `0x2ba518`'s line 10 out along the camera's yaw and pitch (flags 4,
/// Ratchet left out) — met, at the point it meets from Ratchet's point (0.5, 0, 0.5) in his frame (`0x2d3a80`); else
/// the camera's yaw and its pitch raised 7° — kept as the throw's yaw and pitch (`0x18aa24` / `0x18aa28`), his sound
/// 0x11, the wrench turned as the camera. Else along his facing (`0x2d3b38`: his yaw's direction, in the gravity
/// frames his forward row), the wrench turned as he is.
fn throw_dir(_world: &MobyWorld, ctx: &mut Ctx, t: &mut Throw) -> [V3; 3] {
    use crate::hero::physics::{add, atan2, len2, scale, sub};
    let h = &mut *ctx.hero;
    if !t.look {
        t.dir = if h.gravity.mode != 0 { h.rows[0] } else { [h.rot[2].cos(), h.rot[2].sin(), 0.0] };
        return h.rows;
    }
    let (cam, rows) = (ctx.host.camera_pos(), ctx.host.camera_rows());
    let f = rows[0];
    let (cam_yaw, cam_up) = (f[1].atan2(f[0]), f[2].atan2((f[0] * f[0] + f[1] * f[1]).sqrt()));
    let along = |yaw: f32, pitch: f32| [yaw.cos() * pitch.cos(), yaw.sin() * pitch.cos(), pitch.sin()];
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, cam, add(cam, scale(along(cam_yaw, cam_up), 10.0)), crate::collision::QueryFlags(4), ctx.hero_moby));
    match hit {
        Some(o) => {
            let from = add(h.pos, add(scale(h.rows[0], 0.5), scale(h.rows[2], 0.5)));
            let d = sub(o.point, from);
            h.melee.throw_yaw = atan2(d[0], d[1]);
            h.melee.throw_pitch = atan2(len2(d), d[2]);
        }
        None => {
            h.melee.throw_yaw = cam_yaw;
            h.melee.throw_pitch = crate::hero::physics::add_rot(cam_up, 0.122_173_05);
        }
    }
    h.sounds.push((0x11, 0));
    t.dir = along(h.melee.throw_yaw, h.melee.throw_pitch);
    rows
}

/// `0x2d54f8`'s look-stance part: the hand item becomes the first-person weapon once the look stance (1 / 0x1d) has
/// run 15 ticks (`$gp−0x7308`), starting from where it was in Ratchet's hand (its place in his frame and its
/// rotation, blended from there: `t` 0); out of the stance it is placed once more, then left where it is (`t` 1).
/// Not ported: the First Person Mode's aim (`0x1a790c`, `0x18b300..0x18b314`: the Specials page's mode is not ported).
pub fn first_person_weapon(world: &mut MobyWorld, ctx: &mut Ctx) {
    use crate::hero::physics::{dot, sub};
    let (look, hero_pos, hero_rows) = (matches!(ctx.hero.state, 1 | 0x1d), ctx.hero.pos, ctx.hero.rows);
    let local = move |p: V3| { let d = sub(p, hero_pos); [dot(d, hero_rows[0]), dot(d, hero_rows[1]), dot(d, hero_rows[2])] };
    let not_wrench = ctx.hero.items.hand.filter(|_| ctx.hero.items.hand_class != WRENCH_CLASS).and_then(|id| world.mobys.get(id)).map(|m| (local(m.pos), crate::quat::from_rows(&m.rows)));
    if !look || ctx.hero.timer < 15 {
        if !look {
            if !ctx.hero.items.fp.on { return; }
            view_model(world, ctx);
            let at = ctx.hero.items.hand.filter(|_| ctx.hero.items.hand_class != WRENCH_CLASS).and_then(|id| world.mobys.get(id)).map(|m| local(m.pos));
            let fp = &mut ctx.hero.items.fp;
            if let Some(p) = at {
                fp.start_pos = p;
                (fp.t_v, fp.t) = (0.0, 1.0);
            }
            fp.on = false;
            return;
        }
    } else if !ctx.hero.items.fp.on {
        let fp = &mut ctx.hero.items.fp;
        if let Some((p, q)) = not_wrench { (fp.start_pos, fp.start_q) = (p, q); }
        fp.on = true;
        (fp.pull_to, fp.t, fp.t_v) = (0.0, 0.0, 0.0);
    }
    if ctx.hero.items.fp.on { view_model(world, ctx); }
}

/// `0x2d5ce0`: the first-person weapon. The wrench shows only in flight. The others (but items 0xc / 0x11, hidden,
/// `0x2adf50`) sit in the view's frame at their record's place (`+0xa0`: its x pulled back by `0x18b328`; y mirrored
/// with the mirror cheat) bobbing (its phase speeding to 90′ a tick standing, 600′ moving, at 0.4° a tick; 0.02 · sin
/// up), turned by the record's angles (`+0xb0`) in the view's frame, with the platform's carry; blended in from the
/// hand (`t` eased to 1: 1/6 a tick up, 1/240 down, at most 1/6). Every other frame a line from the eye to 1.2 of the
/// weapon's sphere behind its centre (along −row 1) finds the world in the way: the pull back's target is how far it
/// would go in (at most 1.5), eased toward (1/240, 1/240, at most 1/12).
///
/// Not ported: the call of the item's `+0x64` hook after the placement.
fn view_model(world: &mut MobyWorld, ctx: &mut Ctx) {
    use crate::hero::physics::{add, add_rot, approach, dist, ease, euler_rows, scale, set_len, sub};
    let Some(id) = ctx.hero.items.hand else { return };
    if world.mobys.get(id).is_none() { return; }
    if ctx.hero.items.hand_class == WRENCH_CLASS {
        let m = &mut world.mobys[id];
        if ctx.hero.items.flying == 0 && m.state == 0 { m.mode |= 1 } else { m.mode &= !1 }
        return;
    }
    let item = ctx.hero.items.item;
    if matches!(item, 0xc | 0x11) && ctx.hero.items.ready == 2 {
        world.mobys[id].mode |= 1;
        world.rebuild(id, &*ctx.host);
        return;
    }
    // `0x2f4b88(item)`: the place of the item's current row (`0x139568`).
    let Some(rec) = ctx.game.record(&ctx.level.items, item).copied() else { return };
    let w = rec.first_person.map(f32::from_bits);
    let (cam, cam_rows) = (ctx.host.camera_pos(), ctx.host.camera_rows());
    let mirror = ctx.game.cheat(crate::cheats::cheat::MIRROR);
    let (hero_pos, hero_rows, carry, moving) = (ctx.hero.pos, ctx.hero.rows, ctx.hero.carry, 0.02 < ctx.hero.eff_len_xy);
    let fp = &mut ctx.hero.items.fp;
    let mut off = [w[0] + fp.pull, w[1], w[2]];
    if mirror { off[1] = -off[1]; }
    let rate = if moving { 600.0 } else { 90.0 } * 0.000_290_888_2;
    approach(rate, 0.006_981_317, &mut fp.bob_v);
    fp.bob = add_rot(fp.bob, fp.bob_v);
    off[2] += fp.bob.sin() * 0.02;
    let mut pos = add(cam, add(add(scale(cam_rows[0], off[0]), scale(cam_rows[1], off[1])), scale(cam_rows[2], off[2])));
    pos = add(pos, carry);
    let e = euler_rows([w[3], w[4], w[5]]);
    let mut rows: [V3; 3] = std::array::from_fn(|i| add(add(scale(cam_rows[0], e[i][0]), scale(cam_rows[1], e[i][1])), scale(cam_rows[2], e[i][2])));
    fp.q = crate::quat::from_rows(&rows);
    if fp.t < 1.0 {
        let (mut t, mut v) = (fp.t, fp.t_v);
        ease(1.0, 10.0 / 60.0, 15.0 / 3600.0, 10.0 / 60.0, &mut t, &mut v);
        (fp.t, fp.t_v) = (t, v);
        let s = fp.start_pos;
        let start = add(hero_pos, add(add(scale(hero_rows[0], s[0]), scale(hero_rows[1], s[1])), scale(hero_rows[2], s[2])));
        pos = add(start, scale(sub(pos, start), t));
        rows = crate::quat::rows(crate::quat::nlerp(fp.start_q, crate::quat::from_rows(&rows), t));
    }
    {
        let m = &mut world.mobys[id];
        m.pos = pos;
        m.rows = rows;
        m.mode = (m.mode | mode::NO_UPDATE | mode::NO_MATRIX | mode::KEEP_ROWS) & !1;
        let f = rows[0];
        m.rot = [0.0, -f[2].atan2((f[0] * f[0] + f[1] * f[1]).sqrt()), f[1].atan2(f[0])];
    }
    world.rebuild(id, &*ctx.host);
    if ctx.game.frame & 1 == 0 {
        let b = world.mobys[id].bsphere;
        let r = b[3] / 1024.0;
        let c = [b[0] / 1024.0, b[1] / 1024.0, b[2] / 1024.0];
        let far = add(c, set_len(world.mobys[id].rows[1], r * -1.2));
        let hit = ctx.coll.and_then(|k| crate::collision::coll_line_m(k, ctx.scene, cam, far, crate::collision::QueryFlags(0), ctx.hero_moby));
        let fp = &mut ctx.hero.items.fp;
        fp.pull_to = match hit {
            Some(o) => (fp.pull - r + dist(o.point, c)).clamp(-1.5, 0.0),
            None => 0.0,
        };
    }
    let fp = &mut ctx.hero.items.fp;
    let (mut p, mut v) = (fp.pull, fp.pull_v);
    ease(fp.pull_to, 1.0 / 240.0, 1.0 / 240.0, 1.0 / 12.0, &mut p, &mut v);
    (fp.pull, fp.pull_v) = (p, v);
}
