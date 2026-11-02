//! The Mini-Turret Glove (item 0x29, crate::hero::items::ROWS; L00's record `0x2648e0`: attachment 6, stance 0, use
//! sequence 0x2c, the firing arm's 0x42 moving and 0x2c crouched, icon 0xea6a, 20 turrets carried; its turrets' shots
//! 30 a second, 6 a second, damage 0.7, 20 shots each, `+0x48`): the hand moby **0x0c5a** (L00 `0x3ebed8`, called by
//! the item pass while it is the hand item) and the turret it throws, **0x1366** (L00's update `0x387ee8`, made by
//! `0x389078`), whose shots are records of its own (`0x237240`), not mobys.
//!
//! L00's `0x387ee8` is no function in the Ghidra project; it is byte for byte L03's `0x38d190` but for the relocated
//! addresses (its helpers L00 `0x388740`..`0x38a5d0` are, in L03, `0x38d9e8`..`0x38f878`), which is where it was read.
//!
//! **The hand** (pvars: `+0x00` the landing point, `+0x10` its normal, `+0x20` a timer, `+0x24` a landing found): each
//! tick from its joint 0 the throw (`0x37e328(1.5, reach, 0.25 a tick, 11 a second², ..)`, no target: the reach 4.5
//! ahead of Ratchet, 11 in the look stance 0x1d, or with L1 the camera's look;
//! crate::moby::classes::gravity_bomb::launch_with; Ratchet's look stance 1 goes to 0x1d) and the arc followed to where
//! it lands (`0x37e988`, crate::moby::classes::gravity_bomb::landing_with), its marker drawn ([`draw_marker`],
//! `0x3ec108` → `0x37dfc0(1, ..)`). In the look stance 0x1d it waits out `0x18c354` (else 0x28 again). **0** → 1 (no
//! landing this tick); **1** ○ pressed (`0x138324 & 0x18b050`, not kept away `0x18c0dc`, the look wait out): one
//! turret (`0x2d42d0`), the use stance (`0x2af5e0`), **2** for 20 ticks — none left: its class sound 2. **2** out:
//! class sound 0, the turret made at the joint with the throw's velocity, the hand its owner (`+0xb8`), the use over
//! (`0x2af848`), **3** for 10 ticks, then 1.
//!
//! **The turret** (pvars: `+0x00` its velocity, `+0x10` / `+0x20` / `+0x30` its look from, where it wants to look and
//! its look (in its own frame), `+0x40` its life (ticks), `+0x44` the muzzle flash's ticks, `+0x48` its joint node
//! (−1 none), `+0x4c` (s16) a new look wanted, `+0x4e` (s8) its slot in the table, `+0x50` aimed, `+0x54` the turn's
//! share, `+0x58` its step a tick (1.5°), `+0x5c` the cooldown (seconds), `+0x60` the shots left, `+0x64` its shots in
//! flight, `+0x68` the target, `+0x6c` its index in the frames' table, `+0x70` the moby it stands on and `+0x74` that
//! moby's place). Made (`0x389078`): update and draw distance 0xff, its rows kept (`+0x34 | 0x100`) from Ratchet's
//! angles turned to the up ([`align_up`]), ¾ of its size, its life 1200 ticks, the shots the row's `+0x48`, its look
//! ahead, its joint node on list 0 (`0x37f060`). By state:
//! * **0**: its index (`gp−0xf4`, counted mod 100), no collision; **1** (flying): its rows to the up, it moves, falls 11
//!   a second² and sweeps ([`sweep`], `0x37edb8`: a 0.1 sphere 0.074 up its up row, else the line from its last
//!   place; not Ratchet nor the glove): off a wall (less than 0.5556 up) or a moby of class type 5 it bounces (the
//!   normal speed ×0.45); stopped on another turret (0x00cb, 0x1366, 0x1367) it flies on 1.1 times as fast. Landed:
//!   still, the moby under it kept, by the surface's kind (`0x3813b0`): water (0, 0xd, 0xe) — it sinks (**7**, 0.025 a
//!   tick); 1 / 3 / 0xb / 0xc — **8**; else a slot in the table (`0x381048`), the ready set (`0x3810f8`), ready: class
//!   sound 1, sequence 1, **2**; else **6**. Its collision on.
//! * **2** (unfolding): its frame at 6 → sequence 2, **3** (searching, [`search`] `0x3889a8`): the aim search all round
//!   within 15 of its joint 0 (crate::moby::aim::search, no crates) — a target: **4**, the cooldown 0; with none and a
//!   new look wanted, one 45..90° either way from its look and −0.1..0.45 up. **4** (tracking, [`track`] `0x388b98`):
//!   its want the way to the target's aim point (a target gone, dead or out of the target list: **3**); it fires
//!   ([`fire`] `0x388cc8`) with the cooldown out, aimed and a target: from its joint 1 at the row's speed toward the aim
//!   point, class sound 0, its shot ([`Shot`], `0x389388`), the muzzle flash 6 ticks, its life 1200 again, a shot less
//!   (none left: **8**), the cooldown 1 / the row's rate. The moby under it gone (state −2 / −3): no collision, **1**.
//! * In 3 and 4 the look turns ([`turn`], `0x388e98`: from its look-from toward its want by `1 − (1 − s)²` of the share
//!   `s`, which grows 1.5° a tick; aimed past 0.9; at 1 a new look is wanted), its joint node the look's frame
//!   (`0x388f80`); the 0.2 sphere at its joint 0 (flags 2) meeting the world blows it up (`0x388888`, **8**); out of the
//!   ready set it folds (sequence 1 from frame 9 played backward, **5**, its node let go, the look back to its
//!   look-from). **5**: its frame at 1 → sequence 0, **6** (folded): back in the ready set → class sound 1, sequence 1,
//!   a new node, **2**.
//! * **7** (sinking, `0x388740`): it moves, falls 11 a second² / 10, the line from its last place (flags 0x10) meeting
//!   anything: **8**; one tick in five a bubble (`0x3887d8`, crate::particles::water::bubble). **8**: class sound 3, the
//!   smoke burst (`0x3210a8(0.5, 13)`, crate::moby::blast::smoke_burst), hidden (`+0x34 | 0x41`), **9**: once its shots
//!   are spent, its node and slot let go, gone.
//!
//! Every tick: the cooldown runs down; its life out or Ratchet dying (0x6f) → **8**; its shots step ([`step_shots`],
//! `0x389438`); below 8 standing on a moby it rides its move (`0x3888e8`; on a class of type 0x12 whose pvar byte
//! `+0x14` is 0 it sinks, **7**); its draw (`0x389818`).
//!
//! **The ready set** (the table `0x238500`: 64 turrets, their ready flags `+0x100`, the count `+0x220`, a flag `+0x224`,
//! the most `+0x228` = 5, `0x3892c0`): once a frame before the moby loop (`0x37fa28` → `0x389368`) and at each landing,
//! the five turrets nearest Ratchet are ready ([`Table::refresh`]); the others fold.
//!
//! **The shots** ([`Shot`], 100 records of 0x30 bytes at `0x237240`: `+0x00` the place, `+0x10` the velocity, `+0x20`
//! the life in seconds (120), `+0x24` the turret): each tick of its turret ([`step_shots`]) a shot out of the world's
//! 2..1021 box or its life out goes; else it moves and the line from its last place (flags 0, its turret left out)
//! meeting the world, a moby neither in the target list nor of class type 5, Ratchet or another turret ends it with 3
//! sparks; another moby takes the row's damage (flags 0x210000 along its way, kind `0x1366 << 16 | 0x0305`) and 6
//! sparks ([`sparks`], `0x38a5d0`: twinkles, red and white, flying out and toward the camera).
//!
//! **The draw** ([`draw`], `0x389818`): while the flash's ticks run down (the draw counts them), the muzzle flash
//! (`0x389b60`: a glow at joint 1 (`0x389860`) and two orange streaks 2..4 out along its look, FX 0x17); then each of its
//! shots (`0x3896e8` → `0x38a268`): a red glow (`0x389f90`, FX 0xb) and two streaks (FX 0xe) back 2.1 steps.
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x29, the Spiderbot Glove's): crate::hero::items.
//!
//! **The upgraded glove** (rows 0x40, 0x59, 0x5a, crate::hero::items::UPGRADE_ROWS): its hand **0x0a65** (`0x3c9c70`,
//! the hand above but for the turret it throws, its marker `0x3c9ea0`) throws **0x1367** (made by `0x38bb30`, updated by
//! `0x38a9a8`): the turret above with its own ready set (`0x238d70`, `0x38bdc8`), its full size, 10 shots (not the
//! row's `+0x48`), the sweep's sphere 0.13 lifted 0.0962, unfolded from frame 6.5, its head's sphere 0.25 and no shot
//! records: a shot ([`fire`], `0x38b770`) is its class sound 0 and a **plasma shot 0x0a82** (`0x3cb8f0(2, 15, 0.5, 3,
//! muzzle, way, 0x50, 200, 1, 0, 0x10e, target)`, the glove its owner), the cooldown 2/3 s (not the row's rate). Its
//! draw (`0x38bde8`) is the muzzle flash. The original keeps its target at `+0x64` and the glove at `+0x68`; the port
//! keeps the base turret's pvars.
//!
//! **The plasma shot** ([`update_plasma`], `0x3cadc0`; pvars: `+0x00` its velocity, `+0x10..+0x40` its trail's four
//! points, `+0x50` its target, `+0x54` its turn's spring, `+0x58` / `+0x5c` its spiral's phases, `+0x60` its speed (a
//! second; 2), `+0x64` its trail's scroll, `+0x68` its top speed (15), `+0x6c` (s16) its smoke's count, `+0x70` its
//! speed's growth a tick (0.5), `+0x74` its damage (3), `+0x78` / `+0x79` its smoke's life (0x50..200), `+0x7a` its
//! smoke's interval (1), `+0x7b` the wide homing (0), `+0x7c` no search (made without a target), `+0x7d` the trail's
//! shift timer, `+0x7e` (u16) its life (270 ticks)); size 0.015, its rows kept. **0**: its trail laid behind it (0.3
//! back along its way, three more points 0.01 on), **1**. Each tick its trail's scroll −0.1 (wrapping at −8); its
//! target gone (state −x) or without collision is dropped and, free to search, the first moby within 15 (flags 0) of
//! the target list, not 0x0a7e nor Ratchet, of class type 5 is taken; its rows along its way (row 1 the world's up ×
//! it); it moves; its life out, or out of the world's 2..1021 box (put back), **2** hidden. The 0.25 sphere at it
//! (flags 0x10, itself left out), else the step's line, meeting anything but a turret: every moby within 1 of mode
//! 0x1000 / 0x4000 takes its damage (flags 0x210000 along its velocity, kind `0x0a82 << 16 | 0x0305`), a smoke burst
//! (`0x3210a8(1, 13, shot, point, 0)`), **2** hidden. Its speed the least of its speed and its top (then +0.5 a tick).
//! With a target it homes ([`home`], `0x321de8(0.03, 0.2, 0, 0.25, …)`, the spring `0x2d7f80`) on the target's aim
//! point spiralled by its phases (`sin b · min(0.749 d, 0.749)` across, `cos b · min(0.751 d, 0.751)` back, `(sin a +
//! 1) · min(d / 2, 0.5)` up; d its distance, 0 under 0.25). Every tick a smoke puff (crate::particles::plasma,
//! `0x342740`) 0.3 behind it, half the step large, its ALPHA 0x48 / 0x44 in turn; the phases step (b +0.01..0.1, a
//! −0.025..0.25); every second tick its trail shifts on, the newest its tail. **2**: gone. Its row 1's y and row 2's z
//! are halved after the tail is taken (the model's squash). The draw ([`draw_plasma`], `0x3cbaa0` → `0x3cbad0`, state
//! 1): the glow `0x2ed370` (crate::fx::halo) at its tail and the streak `0x2ed6d8(scroll, 0.25, tail, trail, 4)`
//! (crate::fx::streak), both `0x103080`.
//!
//! Not ported (the upgraded glove): the free mobys' check of the shot's make (`0x1b1d40` below 0x11: none), the
//! search's class list (`$gp−0x90`: {0, 0x1367}, of no class of type 5), the weapon mods on the plasma's hits
//! (`0x4126b8`). The gravity frames and moving frames (`0x37f538` /
//! `0x37f618` / `0x37f5f8`, `0x380e08` / `0x380e58`, the down `0x37ecc0` of the frames: the port's crate::gravity
//! stands in for the up); the level's moving frame (`0x1adb0c`, the tables `0x238730` / `0x2388c0` by the turret's
//! index: what it lands on carries it with its turn; null on the ported levels), the water's ripple (`0x37d5a0`, with
//! the ripple module: G-REN-009) and splash moby (`0x39f938`: class 0x307, in no level), the water's patch and plane
//! heights for the bubbles (`0x31d7a0`: the bubble's own height here, as the port's water does); the weapon mods on the
//! shots' hits (`0x4126b8`); the shots' line through the gravity frames (`0x380e58`). The original keeps the table's
//! moby pointers and the shots' owners as pointers; the port keeps moby indices.

use crate::collision::QueryFlags;
use crate::hero::items::{row, MINI_TURRET};
use crate::hero::physics::{add, add_rot, cross, dist, dot, len, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{aim, countdown, mode, Ctx, MobyWorld};
use rc_formats::moby_anim::JointModifier;

/// The hand's and the turret's classes; the upgraded glove's hand, turret and plasma shot; the class a turret flies
/// past.
pub const HAND_CLASS: i16 = 0x0c5a;
pub const TURRET: i16 = 0x1366;
pub const UP_HAND: i16 = 0x0a65;
pub const UP_TURRET: i16 = 0x1367;
pub const PLASMA: i16 = 0x0a82;
const PASS_CLASS: i16 = 0x00cb;

/// `0x37e328`'s arguments: the least reach (with L1), the reach ahead (in the look stance 0x1d), the speed across (a
/// tick) and the gravity (a tick²: 11 a second², `0x3b483fb8`).
const LEAST: f32 = 1.5;
const REACH: f32 = 4.5;
const REACH_LOOK: f32 = 11.0;
const SPEED: f32 = 0.25;
const GRAVITY: f32 = f32::from_bits(0x3b48_3fb8);
/// The turret's fall (`gp−0x110`: 11 a second², ·1/3600 a tick²; sinking ·1/36000).
const FALL: f32 = 11.0;
/// `0x37edb8`'s record from the turret: the sphere's radius and its lift along the up row, the bounce's share of the
/// normal speed (`gp−0x108`), the least up of a floor, the classes it passes (Ratchet, the glove).
const RADIUS: f32 = 0.1;
const LIFT: f32 = f32::from_bits(0x3d97_8d50);
/// The upgraded turret's (`0x38a9a8`): 0.13 and 0.0962.
const UP_RADIUS: f32 = f32::from_bits(0x3e05_1eb8);
const UP_LIFT: f32 = f32::from_bits(0x3dc5_0481);
const RESTITUTION: f32 = 0.45;
const FLOOR: f32 = f32::from_bits(0x3f0e_38e4);
const PASS: [i16; 2] = [0, HAND_CLASS];
/// `gp−0xf8`: the life (ticks), from its making and again at each shot.
const LIFE: i32 = 1200;
/// `0x3889a8`'s search (`0x3257a0(2π, 2π, gp−0xfc = 15, 0, 0, ..)`; its class list `gp−0xf0` {0, 0x1366} has no class
/// of type 5 and is not passed).
const CONE: aim::Cone = aim::Cone { yaw: std::f32::consts::TAU, pitch: std::f32::consts::TAU, range: 15.0, near_pitch: 0.0, near_yaw: 0.0 };
/// The turn's step a tick (`0x3cd67752`, 1.5°).
const TURN_STEP: f32 = f32::from_bits(0x3cd6_7752);
/// The sphere at joint 0 that blows the turret up in the world (`0x388888`; the upgraded turret's `0x38b348` 0.25).
const HEAD: f32 = 0.2;
const UP_HEAD: f32 = 0.25;
/// The frame of its unfolding sequence from which it searches (the upgraded turret's 6.5).
const UNFOLD: f32 = 6.0;
const UP_UNFOLD: f32 = 6.5;
/// The upgraded turret's shots (`0x38bb30`: 10, not the row's) and their cooldown (`0x38b770`: 2/3 s, not the row's
/// rate).
const UP_SHOTS: i32 = 10;
const UP_COOL: f32 = f32::from_bits(0x3f2a_aaab);
/// The table's turrets and how many are ready at most (`0x3892c0`).
const SLOTS: usize = 64;
const MOST_READY: usize = 5;
/// The shots' records, their life (seconds) and the frame's time step (`gp−0x76c4`).
const SHOTS: usize = 100;
const SHOT_LIFE: f32 = 120.0;
const DT: f32 = 1.0 / 60.0;
/// The world's box.
const WORLD: std::ops::RangeInclusive<f32> = 2.0..=1021.0;
/// The marker's colours (`gp−0x14e0`, crate::moby::classes::gravity_bomb::MARKER_RGBA).
const MARKER_RGBA: [u32; 2] = super::gravity_bomb::MARKER_RGBA;

/// The hand's pvars.
mod hand {
    pub const LAND: usize = 0x00;
    pub const NORMAL: usize = 0x10;
    pub const TIMER: usize = 0x20;
    pub const FOUND: usize = 0x24;
}

/// The turret's pvars (module docs).
mod pv {
    pub const VEL: usize = 0x00;
    pub const FROM: usize = 0x10;
    pub const WANT: usize = 0x20;
    pub const LOOK: usize = 0x30;
    pub const LIFE: usize = 0x40;
    pub const FLASH: usize = 0x44;
    pub const NODE: usize = 0x48;
    pub const NEW_LOOK: usize = 0x4c;
    pub const SLOT: usize = 0x4e;
    pub const AIMED: usize = 0x50;
    pub const SHARE: usize = 0x54;
    pub const STEP: usize = 0x58;
    pub const COOL: usize = 0x5c;
    pub const LEFT: usize = 0x60;
    pub const FLYING: usize = 0x64;
    pub const TARGET: usize = 0x68;
    pub const INDEX: usize = 0x6c;
    pub const UNDER: usize = 0x70;
    pub const UNDER_AT: usize = 0x74;
}

/// The row's `+0x48`: the shots a turret has (the base row's 20; the upgraded rows' turrets are not ported).
const SHOTS_EACH: i32 = 20;

/// One of the shots (`0x237240 + 0x30·i`).
#[derive(Clone, Copy, Debug, Default)]
pub struct Shot {
    pos: V3,
    vel: V3,
    life: f32,
    /// `+0x24`: the turret (none: free).
    owner: Option<usize>,
}

/// The turrets' table (`0x238500`, module docs).
#[derive(Clone, Debug, Default)]
pub struct Table {
    turrets: Vec<Option<usize>>,
    ready: Vec<bool>,
    count: usize,
    on: bool,
}

impl Table {
    fn grow(&mut self) {
        if self.turrets.len() < SLOTS {
            self.turrets.resize(SLOTS, None);
            self.ready.resize(SLOTS, false);
        }
    }

    /// `0x381048(table, turret)`: the first free slot (−1 none), not ready.
    fn take(&mut self, id: usize) -> i32 {
        self.grow();
        let Some(k) = self.turrets.iter().position(Option::is_none) else { return -1 };
        self.turrets[k] = Some(id);
        self.on = true;
        self.count += 1;
        self.ready[k] = false;
        k as i32
    }

    /// `0x3810c8(table, slot)`.
    fn free(&mut self, k: usize) {
        self.grow();
        let Some(t) = self.turrets.get_mut(k) else { return };
        *t = None;
        self.ready[k] = false;
        self.count = self.count.saturating_sub(1);
        if self.count == 0 { self.on = false; }
    }

    fn is_ready(&self, k: i32) -> bool { usize::try_from(k).ok().and_then(|k| self.ready.get(k)).copied().unwrap_or(false) }

    /// `0x3810f8(table)`: with turrets in it, every one not ready, then the nearest to Ratchet (`0x189ea0`) made ready,
    /// at most five (the first of equals).
    pub fn refresh(&mut self, w_pos: impl Fn(usize) -> Option<V3>, hero: V3) {
        if !self.on { return; }
        self.grow();
        let mut dist = [0.0f32; SLOTS];
        let mut seen = 0;
        for k in 0..SLOTS {
            if let Some(t) = self.turrets[k] {
                self.ready[k] = false;
                seen += 1;
                dist[k] = w_pos(t).map_or(f32::MAX, |p| len(sub(hero, p)));
            }
            if seen == self.count { break; }
        }
        for _ in 0..MOST_READY.min(self.count) {
            let mut best = (9999.0f32, None);
            for k in 0..SLOTS {
                if self.turrets[k].is_some() && !self.ready[k] && dist[k] < best.0 { best = (dist[k], Some(k)); }
            }
            let Some(k) = best.1 else { break };
            self.ready[k] = true;
        }
    }
}

/// The turrets' globals: the tables (the upgraded turrets' own, `0x238d70`), the shots and the frame the ready sets were
/// last made in (the port's: the original makes them in the frame's start, before the moby loop, `0x37fa28` →
/// `0x389368` / `0x38bdc8`, where the first turret's update stands in).
#[derive(Clone, Debug, Default)]
pub struct Mem {
    pub table: Table,
    pub table_up: Table,
    shots: Vec<Shot>,
    stamp: Option<i32>,
    /// `gp−0xf4`: the next turret's index.
    next_index: i32,
}

impl Mem {
    fn shots(&mut self) -> &mut Vec<Shot> {
        if self.shots.len() < SHOTS { self.shots.resize(SHOTS, Shot::default()); }
        &mut self.shots
    }
}

/// `0x3ebed8` (module docs).
pub fn update_hand(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let from = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let hero_state = ctx.hero.state;
    let reach = if hero_state == 0x1d { REACH_LOOK } else { REACH };
    // `0x37e328` with no target: the look stance 1 goes to its scripted twin (at once in the original: the look wait
    // below already counts).
    if hero_state == 1 { ctx.hero.request_state(0x1d, true); }
    let vel = super::gravity_bomb::launch_with(ctx, LEAST, reach, SPEED, GRAVITY, from, None);
    let found = super::gravity_bomb::landing_with(w, ctx, id, from, vel, GRAVITY);
    {
        let m = &mut w.mobys[id];
        if let Some((p, n)) = found {
            m.set_pv_v(hand::LAND, p);
            m.set_pv_v(hand::NORMAL, n);
        }
        m.set_pv_i(hand::FOUND, found.is_some() as i32);
    }
    let look_ok = if matches!(hero_state, 0x1d | 1) {
        countdown(&mut ctx.hero.items.look_wait)
    } else {
        ctx.hero.items.look_wait = 0x28;
        true
    };
    let mut test = false;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state = 1;
            m.set_pv_i(hand::FOUND, 0);
            test = true;
        }
        1 => test = true,
        2 => {
            let mut t = w.mobys[id].pv_i(hand::TIMER);
            if countdown(&mut t) {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                if let Some(tur) = make(w, ctx, from, vel, w.mobys[id].o_class == UP_HAND) { w.mobys[tur].parent = Some(id); }
                let idle = ctx.hero.idle_seq();
                ctx.hero.items.use_end(hero_state, ctx.hero_view.seq_b, idle);
                w.mobys[id].state = 3;
                t = 10;
            }
            w.mobys[id].set_pv_i(hand::TIMER, t);
        }
        3 => {
            let mut t = w.mobys[id].pv_i(hand::TIMER);
            if countdown(&mut t) { w.mobys[id].state = 1; }
            w.mobys[id].set_pv_i(hand::TIMER, t);
        }
        _ => {}
    }
    let it = ctx.hero.items;
    if test && it.pressed & it.trigger != 0 && it.no_use == 0 && look_ok {
        if ctx.hero.items.take_ammo(ctx.game, 1) {
            ctx.hero.items.use_start(ctx.hero.group);
            let m = &mut w.mobys[id];
            m.state = 2;
            m.set_pv_i(hand::TIMER, 0x14);
        } else {
            ctx.host.play_sound(&w.mobys[id], 2, 0);
        }
    }
    w.register_draw(id);
}

/// `0x3ec108` → `0x37dfc0(1, landing, normal, 0)`: the marker where the throw lands, when one was found; not in
/// Ratchet's states 0x17..0x1b, 0x6f, 0x62 or in a scene.
pub fn draw_marker(w: &MobyWorld, id: usize, hero_state: i32, scene_mode: i32, frame: i32, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if m.pv_i(hand::FOUND) == 0 || matches!(hero_state, 0x17..=0x1b | 0x6f | 0x62) || scene_mode == crate::scene::SCENE { return; }
    super::gravity_bomb::marker_quads(m.pv_v(hand::LAND), m.pv_v(hand::NORMAL), MARKER_RGBA, frame, out);
}

/// `0x389078`'s and the update's turn of the rows so that the third is the up: the quaternion about `up × row 2`
/// whose half angle is *half their cosine* (the game's arithmetic, as `0x37e328`'s L1 branch:
/// crate::moby::classes::gravity_bomb::look_aim_with), the rows turned by it (`0x2efc20`, `0x2efa00`).
fn align_up(rows: [V3; 3], up: V3) -> [V3; 3] {
    let b = unit(rows[2]);
    let axis = cross(up, b);
    let half = dot(b, up) * 0.5;
    let (s, c) = rc_formats::moby_light::vu0_sin_cos(half.to_bits());
    let a = scale(axis, f32::from_bits(s));
    crate::quat::turn_rows(&rows, [a[0], a[1], a[2], f32::from_bits(c)])
}

/// `0x389078(pos, vel)`: the turret (module docs), None when its class is not loaded; `up`: the upgraded turret
/// (`0x38bb30`: class 0x1367, its full size, 10 shots).
fn make(w: &mut MobyWorld, ctx: &mut Ctx, pos: V3, vel: V3, up: bool) -> Option<usize> {
    let class = if up { UP_TURRET } else { TURRET };
    if w.class(class).is_none() {
        eprintln!("rc_game::turret: class {class:#x} is not loaded: no turret");
        return None;
    }
    let id = w.create(class);
    let rows = align_up(crate::hero::physics::euler_rows(ctx.hero.rot), ctx.hero.gravity.up_at(pos));
    let m = &mut w.mobys[id];
    m.update = Some(update);
    m.mode = (m.mode & !mode::NO_UPDATE) | mode::KEEP_ROWS;
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    if !up { m.scale *= 0.75; }
    m.state = 0;
    m.pos = pos;
    m.rows = rows;
    m.rot = ctx.hero.rot;
    m.set_pv_v(pv::VEL, vel);
    m.set_pv_i(pv::TARGET, 0);
    m.set_pv_i(pv::FLYING, 0);
    m.set_pv_i(pv::AIMED, 0);
    m.set_pv_i(pv::FLASH, 0);
    m.set_pv_i(pv::LEFT, if up { UP_SHOTS } else { SHOTS_EACH });
    m.set_pv_s(pv::NEW_LOOK, 1);
    m.set_pv_i(pv::LIFE, LIFE);
    for o in [pv::FROM, pv::LOOK, pv::WANT] { m.set_pv_v(o, [1.0, 0.0, 0.0]); }
    m.set_pv_f(pv::STEP, TURN_STEP);
    // `0x37f060(turret, 0)`: its node on joint list 0 (`0x312480` leaves it composing, unturned).
    m.joint_nodes = vec![(0, JointModifier::compose(0))];
    m.set_pv_i(pv::NODE, 0);
    m.set_pv_b(pv::SLOT, 0xff);
    m.set_pv_i(pv::UNDER, 0);
    w.rebuild(id, &*ctx.host);
    w.changed = true;
    Some(id)
}

/// What [`sweep`] found.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Swept {
    None,
    /// Off a wall or a moby of class type 5 (the velocity bounced).
    Bounce,
    /// Stopped.
    Stop,
}

/// `0x37edb8` with the turret's record: the 0.1 sphere at `to` lifted 0.074 along its up row (its pushed-out centre),
/// else the line from `from` (its point), flags 0, the turret left out; a hit on Ratchet or the glove is none. The
/// normal (unit) less than 0.5556 along the up at the turret, or a moby of class type 5, bounces `vel` (reflected,
/// 0.45 of the normal speed kept); else it stops at the query's point. Returns the place, the moby met and the surface.
fn sweep(w: &MobyWorld, ctx: &Ctx, id: usize, from: V3, to: V3, vel: &mut V3) -> (Swept, V3, Option<usize>, i32) {
    let none = (Swept::None, to, None, -1);
    let Some(c) = ctx.coll else { return none };
    let up = ctx.hero.gravity.up_at(w.mobys[id].pos);
    let (radius, lift) = if is_up(w, id) { (UP_RADIUS, UP_LIFT) } else { (RADIUS, LIFT) };
    let centre = add(to, set_len(w.mobys[id].rows[2], lift));
    let hit = crate::collision::coll_sphere_m(c, ctx.scene, centre, radius, QueryFlags(0), Some(id)).map(|h| (h.pushed_centre.unwrap_or(h.point), h))
        .or_else(|| w.coll_line(ctx, from, to, 0, Some(id)).map(|h| (h.point, h)));
    let Some((at, h)) = hit else { return none };
    let surface = h.surface_id();
    if h.moby.is_some_and(|t| w.mobys.get(t).is_some_and(|x| PASS.contains(&x.o_class))) { return none; }
    let n = unit(h.normal);
    let kind5 = h.moby.and_then(|t| w.mobys.get(t)).and_then(|x| w.class(x.o_class)).is_some_and(|c| c.kind == 5);
    if dot(up, n) < FLOOR || kind5 {
        // `0x2ef2a8`: reflected; then the normal part 0.45.
        let r = sub(*vel, scale(n, 2.0 * dot(*vel, n)));
        let along = scale(n, dot(r, n));
        *vel = add(sub(r, along), scale(along, RESTITUTION));
        return (Swept::Bounce, at, h.moby, surface);
    }
    (Swept::Stop, h.point, h.moby, surface)
}

/// `0x3813b0(surface)`: 2 for 0 / 0xd / 0xe (the waters), 1 for 1 / 3 / 0xb / 0xc, else 0.
fn surface_kind(s: i32) -> i32 {
    match s {
        0 | 0xd | 0xe => 2,
        1 | 3 | 0xb | 0xc => 1,
        _ => 0,
    }
}

fn gone(w: &MobyWorld, t: usize) -> bool { w.mobys.get(t).is_none_or(|m| matches!(m.state, 0xfe | 0xfd)) }

/// The ready set made once a frame (module docs).
fn frame_refresh(w: &mut MobyWorld, ctx: &Ctx) {
    if w.turret.stamp == Some(ctx.game.frame) { return; }
    w.turret.stamp = Some(ctx.game.frame);
    refresh(w, ctx, false);
    refresh(w, ctx, true);
}

/// `0x3810f8` on the turrets' table (`0x389368`) or the upgraded turrets' (`0x38bdc8`).
fn refresh(w: &mut MobyWorld, ctx: &Ctx, up: bool) {
    let mobys = &w.mobys;
    let t = if up { &mut w.turret.table_up } else { &mut w.turret.table };
    t.refresh(|t| mobys.get(t).map(|m| m.pos), ctx.hero.pos);
}

/// The upgraded turret (0x1367, `0x38a9a8`).
fn is_up(w: &MobyWorld, id: usize) -> bool { w.mobys[id].o_class == UP_TURRET }

/// The table of turret `id`'s kind.
fn table(w: &mut MobyWorld, id: usize) -> &mut Table { if is_up(w, id) { &mut w.turret.table_up } else { &mut w.turret.table } }

/// `0x387ee8` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    frame_refresh(w, ctx);
    let slot = |w: &MobyWorld| w.mobys[id].pv_b(pv::SLOT) as i8 as i32;
    match w.mobys[id].state {
        0 | 1 => {
            if w.mobys[id].state == 0 {
                let i = w.turret.next_index;
                w.turret.next_index = (i + 1) % 100;
                let m = &mut w.mobys[id];
                m.state = 1;
                m.set_pv_i(pv::INDEX, i);
                m.has_collision = false;
            }
            fly(w, id, ctx);
        }
        2 => {
            if (if is_up(w, id) { UP_UNFOLD } else { UNFOLD }) <= ctx.host.frame_time(&w.mobys[id]) {
                set_seq(w, id, ctx, 2, 0);
                w.mobys[id].state = 3;
            }
        }
        3 => {
            search(w, id, ctx);
            under_gone(w, id, ctx);
        }
        4 => {
            track(w, id, ctx);
            fire(w, id, ctx);
            under_gone(w, id, ctx);
        }
        5 => {
            if ctx.host.frame_time(&w.mobys[id]) <= 1.0 {
                set_seq(w, id, ctx, 0, 0);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                w.mobys[id].state = 6;
            }
        }
        6 => {
            if { let k = slot(w); table(w, id).is_ready(k) } {
                w.mobys[id].state = 2;
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                set_seq(w, id, ctx, 1, 0);
                let m = &mut w.mobys[id];
                m.joint_nodes = vec![(0, JointModifier::compose(0))];
                m.set_pv_i(pv::NODE, 0);
            }
        }
        7 => {
            sink(w, id, ctx);
            bubbles(w, id, ctx);
        }
        8 => {
            ctx.host.play_sound(&w.mobys[id], 3, 0);
            let at = w.mobys[id].pos;
            crate::moby::blast::smoke_burst(w, ctx, 0.5, 13.0, Some(id), at, -1);
            let m = &mut w.mobys[id];
            m.state = 9;
            m.mode |= 0x41;
        }
        9 => {
            if w.mobys[id].pv_i(pv::FLYING) == 0 {
                if w.mobys[id].pv_i(pv::NODE) != -1 { w.mobys[id].joint_nodes.clear(); }
                if let Ok(k) = usize::try_from(slot(w)) { table(w, id).free(k); }
                w.delete(id);
                return;
            }
        }
        _ => {}
    }
    let state = w.mobys[id].state;
    if matches!(state, 3 | 4) { turn(w, id); }
    {
        let m = &mut w.mobys[id];
        let c = m.pv_f(pv::COOL);
        m.set_pv_f(pv::COOL, if DT < c { c - DT } else { 0.0 });
        let mut life = m.pv_i(pv::LIFE);
        let out = countdown(&mut life);
        m.set_pv_i(pv::LIFE, life);
        if (out || ctx.hero.state == 0x6f) && m.state != 9 { m.state = 8; }
    }
    step_shots(w, id, ctx);
    if matches!(w.mobys[id].state, 3 | 4) {
        head_test(w, id, ctx);
        if !{ let k = slot(w); table(w, id).is_ready(k) } {
            set_seq(w, id, ctx, 1, 9);
            let m = &mut w.mobys[id];
            m.state = 5;
            m.joint_nodes.clear();
            m.set_pv_i(pv::NODE, -1);
            m.set_pv_s(pv::NEW_LOOK, 1);
            m.set_pv_i(pv::AIMED, 0);
            m.set_pv_i(pv::FLASH, 0);
            m.set_pv_i(pv::TARGET, 0);
            let from = m.pv_v(pv::FROM);
            m.set_pv_v(pv::LOOK, from);
            m.set_pv_v(pv::WANT, from);
            ctx.host.set_anim_speed(&w.mobys[id], -1.0);
        }
    }
    if w.mobys[id].state < 8 && w.mobys[id].pv_link(pv::UNDER).is_some() { ride(w, id); }
    w.register_draw(id);
}

/// `0x31b760(turret, seq, frame, 0, 0)`: the sequence cut in at `frame`, its speed 1.
fn set_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx, seq: u8, frame: u8) {
    ctx.host.set_anim(&w.mobys[id], seq, frame, 0);
    ctx.host.set_anim_speed(&w.mobys[id], 1.0);
}

/// State 1 (module docs): the flight and the landing.
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let from = w.mobys[id].pos;
    let up = ctx.hero.gravity.up_at(from);
    {
        let m = &mut w.mobys[id];
        m.rows = align_up(m.rows, up);
        m.pos = add(m.pos, m.pv_v(pv::VEL));
    }
    let to = w.mobys[id].pos;
    // `0x37ec68(11/3600, pos, vel)`: the fall along the up at the new place.
    let mut vel = sub(w.mobys[id].pv_v(pv::VEL), scale(ctx.hero.gravity.up_at(to), FALL / 3600.0));
    let (s, at, moby, surface) = sweep(w, ctx, id, from, to, &mut vel);
    {
        let m = &mut w.mobys[id];
        m.pos = at;
        m.set_pv_v(pv::VEL, vel);
    }
    if s != Swept::Stop { return; }
    if moby.is_some_and(|t| w.mobys.get(t).is_some_and(|x| matches!(x.o_class, PASS_CLASS | TURRET | UP_TURRET))) {
        w.mobys[id].set_pv_v(pv::VEL, scale(vel, f32::from_bits(0x3f8c_cccd)));
        return;
    }
    {
        let under_at = moby.and_then(|t| w.mobys.get(t)).map(|x| x.pos);
        let m = &mut w.mobys[id];
        m.set_pv_v(pv::VEL, [0.0; 3]);
        m.set_pv_i(pv::UNDER, moby.map_or(0, |t| t as i32));
        if let Some(p) = under_at { m.set_pv_v(pv::UNDER_AT, p); }
    }
    match surface_kind(surface) {
        2 => {
            let m = &mut w.mobys[id];
            let mut v = m.pv_v(pv::VEL);
            v[2] = f32::from_bits(0xbccc_ccce);
            m.set_pv_v(pv::VEL, v);
            m.state = 7;
        }
        1 => w.mobys[id].state = 8,
        _ => {
            let k = table(w, id).take(id);
            w.mobys[id].set_pv_b(pv::SLOT, k as i8 as u8);
            let up = is_up(w, id);
            refresh(w, ctx, up);
            if table(w, id).is_ready(k) {
                w.mobys[id].state = 2;
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                set_seq(w, id, ctx, 1, 0);
            } else {
                w.mobys[id].state = 6;
            }
        }
    }
    // Its collision on (`+0x98` the class's).
    let solid = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    w.mobys[id].has_collision = solid;
    w.changed = true;
}

/// States 3 and 4: the moby under it gone (−2 / −3), no collision and **1** (it falls again).
fn under_gone(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let Some(u) = w.mobys[id].pv_link(pv::UNDER) else { return };
    if !gone(w, u) { return; }
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.state = 1;
    m.set_pv_i(pv::UNDER, 0);
    w.changed = true;
}

/// `0x3889a8` (module docs).
fn search(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let head = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    if w.mobys[id].pv_link(pv::TARGET).is_none() {
        let t = aim::search(w, ctx, CONE, head, w.mobys[id].rows[0], false);
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::TARGET, t.map_or(0, |t| t as i32));
        if t.is_some() {
            m.state = 4;
            m.set_pv_f(pv::COOL, 0.0);
        }
    }
    let m = &w.mobys[id];
    let target = m.pv_link(pv::TARGET);
    if m.pv_s(pv::NEW_LOOK) == 0 && target.is_none() { return; }
    if target.is_none() {
        // A new look: 45..90° either way about the up from its look's yaw, −0.1..0.45 up (`0x320000`, `0x325100`).
        let f = m.pv_v(pv::LOOK);
        let left = unit(cross([0.0, 0.0, 1.0], f));
        let up = cross(f, left);
        let yaw = crate::moby::attach::angles_of(&[f, left, up])[2];
        let g = &mut ctx.game.rng;
        let yaw = add_rot(yaw, g.randf_signed(std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_2));
        let pitch = g.randf(-0.1, 0.45);
        w.mobys[id].set_pv_v(pv::WANT, super::shot::polar(1.0, yaw, pitch));
    }
    let m = &mut w.mobys[id];
    let look = m.pv_v(pv::LOOK);
    m.set_pv_v(pv::FROM, look);
    m.set_pv_s(pv::NEW_LOOK, 0);
    m.set_pv_f(pv::SHARE, 0.0);
}

/// The target's aim point, while it is a target (not gone, in the target list).
fn target_point(w: &MobyWorld, id: usize) -> Option<V3> {
    let t = w.mobys[id].pv_link(pv::TARGET)?;
    if gone(w, t) || w.mobys[t].mode & mode::TARGET == 0 { return None; }
    Some(aim::aim_point(w, t))
}

/// `0x388b98` (module docs): the want, in its frame (`0x2ef9b0` the transpose, `0x2ef318`), the way from its joint 0
/// to the target's aim point; the target gone (−2 / −3) or out of the target list: **3**, no target.
fn track(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let Some(p) = target_point(w, id) else {
        let m = &mut w.mobys[id];
        m.state = 3;
        m.set_pv_i(pv::TARGET, 0);
        return;
    };
    let head = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let m = &mut w.mobys[id];
    let d = unit(sub(p, head));
    let rows = m.rows;
    m.set_pv_v(pv::WANT, [dot(d, rows[0]), dot(d, rows[1]), dot(d, rows[2])]);
}

/// `0x388cc8` (module docs).
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let muzzle = ctx.host.joint(m, 1).unwrap_or(m.pos);
    if m.pv_f(pv::COOL) != 0.0 || m.pv_i(pv::AIMED) == 0 { return; }
    let Some(t) = m.pv_link(pv::TARGET).filter(|&t| t < w.mobys.len()) else { return };
    let p = aim::aim_point(w, t);
    let Some(r) = row(ctx.game, MINI_TURRET) else { return };
    let vel = set_len(sub(p, muzzle), r.speed * 0.016_666_668);
    let up = is_up(w, id);
    if up {
        // `0x38b770`: its class sound 0, then the plasma shot (`0x3cb8f0(2, 15, 0.5, 3, muzzle, vel, 0x50, 200, 1, 0,
        // 0x10e, target)`), the glove its owner.
        ctx.host.play_sound(&w.mobys[id], 0, 0);
        let owner = w.mobys[id].parent;
        if let Some(s) = spawn_plasma(w, ctx, muzzle, vel, Some(t)) { w.mobys[s].parent = owner; }
    } else {
        spawn_shot(w, id, muzzle, vel);
        ctx.host.play_sound(&w.mobys[id], 0, 0);
    }
    let m = &mut w.mobys[id];
    let left = m.pv_i(pv::LEFT) - 1;
    m.set_pv_i(pv::FLASH, 6);
    m.set_pv_i(pv::LIFE, LIFE);
    m.set_pv_i(pv::LEFT, left);
    if left < 1 { m.state = 8; }
    m.set_pv_f(pv::COOL, if up { UP_COOL } else { 1.0 / r.rate });
}

/// `0x389388(turret, vel)`: a shot in the first free record from its joint 1, 120 s of life; none when all 100 fly.
fn spawn_shot(w: &mut MobyWorld, id: usize, at: V3, vel: V3) {
    let Some(s) = w.turret.shots().iter_mut().find(|s| s.owner.is_none()) else { return };
    *s = Shot { pos: at, vel, life: SHOT_LIFE, owner: Some(id) };
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::FLYING, m.pv_i(pv::FLYING) + 1);
}

/// `0x388e98` (module docs).
fn turn(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let s = m.pv_f(pv::SHARE);
    let t = 1.0 - (1.0 - s) * (1.0 - s);
    let look = crate::quat::turn_toward(t, m.pv_v(pv::FROM), m.pv_v(pv::WANT));
    m.set_pv_v(pv::LOOK, look);
    let s = s + m.pv_f(pv::STEP);
    m.set_pv_f(pv::SHARE, s);
    // `0x388f80`: its node the look's frame across the up (`0x31fe30`).
    let left = unit(cross([0.0, 0.0, 1.0], look));
    let up = cross(look, left);
    let q = crate::quat::from_rows(&[look, left, up]);
    if let Some((_, n)) = m.joint_nodes.get_mut(0) { n.quat = q; }
    m.set_pv_i(pv::AIMED, (0.9 < s) as i32);
    if 1.0 <= s {
        m.set_pv_f(pv::SHARE, 1.0);
        m.set_pv_s(pv::NEW_LOOK, 1);
    }
}

/// `0x388888`: the 0.2 sphere at its joint 0 (flags 2, itself left out) meeting a face of the world or of a moby's mesh
/// (`0x1bf19c` > 0) blows it up (**8**).
fn head_test(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let Some(c) = ctx.coll else { return };
    let head = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    if crate::collision::coll_sphere_m(c, ctx.scene, head, if is_up(w, id) { UP_HEAD } else { HEAD }, QueryFlags(2), Some(id)).is_some_and(|h| 0 < h.kind) { w.mobys[id].state = 8; }
}

/// `0x3888e8`: carried by the move of the moby under it since the last tick; on a class of type 0x12 whose pvar byte
/// `+0x14` is 0 it sinks (**7**).
fn ride(w: &mut MobyWorld, id: usize) {
    let Some(u) = w.mobys[id].pv_link(pv::UNDER) else { return };
    let Some(up) = w.mobys.get(u).map(|x| x.pos) else { return };
    let sinks = w.class(w.mobys[u].o_class).is_some_and(|c| c.kind == 0x12) && w.mobys[u].pv_b(0x14) == 0;
    let m = &mut w.mobys[id];
    m.pos = add(m.pos, sub(up, m.pv_v(pv::UNDER_AT)));
    m.set_pv_v(pv::UNDER_AT, up);
    if sinks { m.state = 7; }
}

/// `0x388740`: it moves and falls 11 a second² / 10; the line from its last place (flags 0x10) meeting anything: **8**.
fn sink(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let from = w.mobys[id].pos;
    let to = add(from, w.mobys[id].pv_v(pv::VEL));
    w.mobys[id].pos = to;
    let vel = sub(w.mobys[id].pv_v(pv::VEL), scale(ctx.hero.gravity.up_at(to), FALL * 2.777_777_8e-5));
    w.mobys[id].set_pv_v(pv::VEL, vel);
    if w.coll_line(ctx, from, to, 0x10, Some(id)).is_some() { w.mobys[id].state = 8; }
}

/// `0x3887d8`: one tick in five a bubble at it (0.05..0.1 · 210000), moving a quarter of its velocity, its top the
/// water's height there (`0x31d7a0`: the patch and plane not ported, its own).
fn bubbles(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.rng.randi(5) != 0 { return; }
    let m = &w.mobys[id];
    let (pos, vel) = (m.pos, scale(m.pv_v(pv::VEL), 0.25));
    let size = ctx.game.rng.randf(0.05, 0.1) * 210_000.0;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    crate::particles::water::bubble(sys, &mut ctx.game.rng, size, pos[2], pos, vel);
}

/// `0x389438(turret)`: its shots' tick (module docs).
fn step_shots(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for i in 0..w.turret.shots().len() {
        let s = w.turret.shots[i];
        if s.owner != Some(id) { continue; }
        let mut life = s.life;
        let out = if DT < life { life -= DT; false } else { life = 0.0; true };
        w.turret.shots[i].life = life;
        let lost = |w: &mut MobyWorld| {
            w.turret.shots[i].owner = None;
            let m = &mut w.mobys[id];
            m.set_pv_i(pv::FLYING, m.pv_i(pv::FLYING) - 1);
        };
        if out || s.pos.iter().any(|c| !WORLD.contains(c)) {
            lost(w);
            continue;
        }
        let to = add(s.pos, s.vel);
        w.turret.shots[i].pos = to;
        let Some(h) = w.coll_line(ctx, s.pos, to, 0, Some(id)) else { continue };
        let n = match h.moby {
            None => 3,
            Some(t) => {
                let x = &w.mobys[t];
                let kind5 = w.class(x.o_class).is_some_and(|c| c.kind == 5);
                if (x.mode & mode::TARGET == 0 && !kind5) || Some(t) == ctx.hero_moby || x.o_class == TURRET {
                    3
                } else {
                    let d = unit(s.vel);
                    let damage = row(ctx.game, MINI_TURRET).map_or(0.0, |r| r.damage);
                    let mut tm = HitTemplate::new(damage, Some(id), 0x0021_0000, [d[0], d[1], d[2], 0.0]);
                    tm.kind = (TURRET as u16 as u32) << 16 | 0x0305;
                    w.deliver_hit_any(t, &tm, [0.0; 4]);
                    6
                }
            }
        };
        lost(w);
        sparks(ctx, h.point, n);
    }
}

/// `0x38a5d0(at, n)`: `n` pairs of twinkles (crate::particles::twinkle, far 0x74) at `at` jittered 0.15: each a size
/// 0.5..2, a spin either way, a way across toward the camera turned −60..60° about the up at 0.05..3 a second, rising
/// 1..5 a second; on an even draw ×1.5 and the twinkles 0.35 and 0.245 of the size over 60 ticks, else toward the
/// camera by that speed more and 2.05 and 1.435 over 40 — a red one (`0x7f000080`) growing to the size, then a white
/// one (`0x7f7f7f7f`) to 0.7 of it turning the other way; both falling 0.002 a tick.
fn sparks(ctx: &mut Ctx, at: V3, n: i32) {
    let cam = ctx.host.camera_pos();
    let up = ctx.hero.gravity.up_at(at);
    let c = sub(cam, at);
    let across = sub(c, scale(up, dot(up, c)));
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for _ in 0..n {
        let size = g.randf(0.5, 2.0);
        let spin: i8 = if g.randi(2) != 0 { 1 } else { -1 };
        let a = g.randf(f32::from_bits(0xbf86_0a93), f32::from_bits(0x3f86_0a93));
        let mut v = super::gravity_bomb::rotate(across, a, up);
        let speed = g.randf(0.05, 3.0) * 0.016_666_668;
        v = set_len(v, speed);
        v[2] = g.randf(1.0, 5.0) * 0.016_666_668;
        let p: V3 = std::array::from_fn(|k| at[k] + g.randf_signed(0.0, 0.15));
        let (k, life) = if g.rand() & 1 == 0 {
            v = scale(v, 1.5);
            (0.245, 0x3c)
        } else {
            v = add(v, set_len(sub(cam, p), speed));
            (1.435, 0x28)
        };
        let big = if life == 0x3c { 0.35 } else { 2.05 };
        for (s, e, rgba, rot, sp) in [(size * big, size, 0x7f00_0080u32, 0u8, spin), (size * k, size * 0.7, 0x7f7f_7f7f, 1, -spin)] {
            if let Some(i) = crate::particles::twinkle(sys, g, s, e, 0.002, p, life, rgba, rot, sp, v, 1) { sys.pool.recs[i][9] = 0x74; }
        }
    }
}

/// The template quad of `0x389788` (±2 in its second and third axes) with its ST.
const ST: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];

/// `0x389860` / `0x389f90(turret, at)`: three quads of `fx` (additive) facing the camera at `at` (pulled 0.25 toward
/// it with `pull`), rolled a random way: the template × k (`randf(k0, k1)`) in `c0` at alpha `rand_range(0x30, 0x40)`,
/// then half that in `c1` and a quarter white (`0x7f7f7f`), each at alpha `rand_range(0x40, 0x7f)`.
#[allow(clippy::too_many_arguments)]
fn glow(out: &mut Vec<crate::fx::FxQuad>, rng: &mut crate::rng::Rng, at: V3, cam: V3, pull: bool, fx: usize, k: (f32, f32), c: [u32; 2]) {
    let a = rng.rand_range(0x30, 0x40) as u32;
    let d = sub(cam, at);
    let yaw = crate::hero::physics::atan2(d[0], d[1]);
    let pitch = -crate::hero::physics::atan2((d[0] * d[0] + d[1] * d[1]).sqrt(), d[2]);
    let roll = rng.rand_angle();
    let rows = crate::hero::physics::euler_rows([roll, pitch, yaw]);
    let p = if pull { add(at, set_len(d, 0.25)) } else { at };
    let mut size = rng.randf(k.0, k.1);
    let corner = |y: f32, z: f32, s: f32| add(p, add(scale(rows[1], y * s), scale(rows[2], z * s)));
    let quad = |s: f32| [corner(-2.0, -2.0, s), corner(-2.0, 2.0, s), corner(2.0, -2.0, s), corner(2.0, 2.0, s)];
    out.push(crate::fx::FxQuad { fx, additive: true, corners: quad(size), st: ST, rgba: [a << 24 | c[0]; 4] });
    for rgb in [c[1], 0x7f_7f7f] {
        size *= 0.5;
        let a = rng.rand_range(0x40, 0x7f) as u32;
        out.push(crate::fx::FxQuad { fx, additive: true, corners: quad(size), st: ST, rgba: [a << 24 | rgb; 4] });
    }
}

/// A strip of `fx` (additive) from `a` (half-width `wa`) to `b` (`wb`) facing the camera, across `(b − a) × (camera −
/// a)`, in `rgba` (its corners in the template's order) and `st`.
#[allow(clippy::too_many_arguments)]
fn strip(out: &mut Vec<crate::fx::FxQuad>, fx: usize, a: V3, wa: f32, b: V3, wb: f32, cam: V3, rgba: [u32; 4], st: [[f32; 2]; 4]) {
    let side = unit(cross(unit(sub(b, a)), sub(cam, a)));
    let corners = [add(a, scale(side, wa)), sub(a, scale(side, wa)), add(b, scale(side, wb)), sub(b, scale(side, wb))];
    out.push(crate::fx::FxQuad { fx, additive: true, corners, st, rgba });
}

/// `0x389818` (module docs).
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let mut t = w.mobys[id].pv_i(pv::FLASH);
    let running = !countdown(&mut t);
    w.mobys[id].set_pv_i(pv::FLASH, t);
    if running { flash(w, host, id, rng, cam, out); }
    // `0x3896e8`: each of its shots, back 2.1 steps (`0x38a268`).
    let shots: Vec<Shot> = w.turret.shots.iter().filter(|s| s.owner == Some(id)).copied().collect();
    for s in shots {
        let tail = sub(s.pos, scale(s.vel, 2.1));
        glow(out, rng, s.pos, cam, false, 0xb, (0.03, 0.0625), [0x80, 0x1f_1f1f]);
        let st = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
        strip(out, 0xe, s.pos, 0.015, tail, 0.015 * 0.25, cam, [0x7f7f_7f7f, 0x7f7f_7f7f, 0x7f_7f7f, 0x7f_7f7f], st);
        strip(out, 0xe, s.pos, 0.0825, tail, 0.015 * 0.25 * 5.5, cam, [0x7f00_0080, 0x7f00_0080, 0x80, 0x80], st);
    }
}

/// `0x389b60`: the muzzle flash at joint 1 — the glow (`0x389860`: FX 0x2c, pulled 0.25, 0.3..0.4, `0x08183f` then
/// `0x103080`), then two orange streaks of FX 0x17 out along its look (through its rows) 2..4, `w` = 0.1..0.3 wide at the
/// muzzle and 0.15 of that at the end, the second twice as wide and at half the alpha; their t flipped on an odd draw.
fn flash(w: &MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let muzzle = host.joint(m, 1).unwrap_or(m.pos);
    let l = rng.randf(2.0, 4.0);
    let v = set_len(m.pv_v(pv::LOOK), l);
    let far = add(muzzle, crate::gravity::to_world(v, &m.rows));
    glow(out, rng, muzzle, cam, true, 0x2c, (0.3, 0.4), [0x08_183f, 0x10_3080]);
    let wd = rng.randf(0.1, 0.3);
    let st = if rng.rand() & 1 == 0 { [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]] } else { [[0.0, 1.0], [0.0, 0.0], [1.0, 1.0], [1.0, 0.0]] };
    strip(out, 0x17, muzzle, wd, far, wd * 0.15, cam, [0x7f10_3080, 0x7f10_3080, 0x10_3080, 0x10_3080], st);
    strip(out, 0x17, muzzle, wd * 2.0, far, wd * 0.15 * 2.0, cam, [0x3f10_3080, 0x3f10_3080, 0x10_3080, 0x10_3080], st);
}

/// The plasma shot's pvars (module docs).
mod plasma {
    pub const VEL: usize = 0x00;
    pub const TRAIL: usize = 0x10;
    pub const TARGET: usize = 0x50;
    pub const TURN: usize = 0x54;
    pub const SPIN_A: usize = 0x58;
    pub const SPIN_B: usize = 0x5c;
    pub const SPEED: usize = 0x60;
    pub const SCROLL: usize = 0x64;
    pub const MAX: usize = 0x68;
    pub const PUFF: usize = 0x6c;
    pub const ACCEL: usize = 0x70;
    pub const DAMAGE: usize = 0x74;
    pub const LO: usize = 0x78;
    pub const HI: usize = 0x79;
    pub const EVERY: usize = 0x7a;
    pub const WIDE: usize = 0x7b;
    pub const FIXED: usize = 0x7c;
    pub const SHIFT: usize = 0x7d;
    pub const TICKS: usize = 0x7e;
}

/// `0x1b00d8`: the smoke's ALPHA modes, in turn.
const PLASMA_ALPHA: [u8; 2] = [0x48, 0x44];
/// The plasma's glow and trail colour.
const PLASMA_RGB: u32 = 0x10_3080;

/// `0x3cb8f0(2, 15, 0.5, 3, at, dir, 0x50, 200, 1, 0, 0x10e, target)`: the plasma shot (module docs).
fn spawn_plasma(w: &mut MobyWorld, ctx: &mut Ctx, at: V3, dir: V3, target: Option<usize>) -> Option<usize> {
    if w.class(PLASMA).is_none() { return None; }
    let id = w.create(PLASMA);
    let _ = ctx;
    let m = &mut w.mobys[id];
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.pos = at;
    m.set_pv_v(plasma::VEL, set_len(dir, 2.0 * 0.016_666_668));
    m.state = 0;
    m.drawn = 1;
    m.draw_dist = 0xff;
    m.update_dist = 0xff;
    m.set_pv_i(plasma::TARGET, target.map_or(-1, |t| t as i32));
    m.set_pv_f(plasma::TURN, 0.0);
    m.scale = f32::from_bits(0x3c75_c28f);
    m.mode |= mode::KEEP_ROWS;
    m.set_pv_f(plasma::SPEED, 2.0);
    m.set_pv_f(plasma::MAX, 15.0);
    m.set_pv_f(plasma::ACCEL, 0.5);
    m.set_pv_b(plasma::LO, 0x50);
    m.set_pv_b(plasma::HI, 200);
    m.set_pv_b(plasma::EVERY, 1);
    m.set_pv_b(plasma::WIDE, 0);
    m.set_pv_f(plasma::DAMAGE, 3.0);
    m.set_pv_f(plasma::SPIN_B, 0.0);
    m.set_pv_s(plasma::TICKS, 0x10e);
    m.set_pv_f(plasma::SPIN_A, 0.0);
    m.set_pv_b(plasma::FIXED, target.is_none() as u8);
    Some(id)
}

/// The plasma's rows along its velocity, row 1 the world's up × it.
fn plasma_rows(v: V3) -> [V3; 3] {
    let r0 = unit(v);
    let r1 = unit(cross([0.0, 0.0, 1.0], r0));
    [r0, r1, cross(r0, r1)]
}

/// `0x321de8(k, d, max, near, pos, vel, aim, &turn, side, 0)`: the velocity turned about `vel × (aim − pos)` (past
/// 165° off, about `vel × (side × vel)`) by the spring `0x2d7f80(0, angle, k, d, max, &turn)` on the angle (within
/// `near` and under 90°: k 0.5, d 1).
#[allow(clippy::too_many_arguments)]
fn home(k: f32, d: f32, max: f32, near: f32, pos: V3, vel: V3, aim: V3, turn: &mut f32, side: V3) -> V3 {
    let to = sub(aim, pos);
    let l = len(to);
    if l == 0.0 { return vel; }
    let mut axis = cross(vel, to);
    let lv = len(vel) * l;
    if lv == 0.0 { return vel; }
    let angle = f32::from_bits(0x3fc9_0fdb) - (dot(vel, to) / lv).clamp(-1.0, 1.0).asin();
    let (mut k, mut d) = (k, d);
    if f32::from_bits(0x4038_4e8a) < angle {
        axis = cross(vel, cross(side, vel));
    } else if l < near && angle < f32::from_bits(0x3fc9_0fdc) {
        (k, d) = (0.5, 1.0);
    }
    // `0x2d7f80`: the spring on the angle, held within ±max and ±the angle.
    let mut v = *turn + (k * angle - d * *turn);
    if max != 0.0 { v = v.clamp(-max, max); }
    v = v.clamp(-angle.abs(), angle.abs());
    *turn = v;
    if len(axis) == 0.0 { return vel; }
    crate::gravity::rotate(vel, unit(axis), add_rot(0.0, v))
}

/// `0x3cadc0` (module docs).
pub fn update_plasma(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    match w.mobys[id].state {
        2 => {
            w.delete(id);
            return;
        }
        0 => {
            let roll = (ctx.game.rng.rand() & 1) as i16;
            let m = &mut w.mobys[id];
            m.state = 1;
            m.set_pv_f(plasma::SCROLL, 0.0);
            m.set_pv_s(plasma::PUFF, roll);
            m.rows = plasma_rows(m.pv_v(plasma::VEL));
            let c = add(scale(m.rows[0], -0.3), m.pos);
            let step = set_len(c, 0.01);
            m.set_pv_v(plasma::TRAIL, c);
            for k in 0..3 {
                let p = add(m.pv_v(plasma::TRAIL + 0x10 * k), step);
                m.set_pv_v(plasma::TRAIL + 0x10 * (k + 1), p);
            }
            m.set_pv_b(plasma::SHIFT, 2);
        }
        1 => {}
        _ => {
            w.register_draw(id);
            return;
        }
    }
    {
        let m = &mut w.mobys[id];
        let mut s = m.pv_f(plasma::SCROLL) - 0.1;
        if s <= -8.0 { s += 8.0; }
        m.set_pv_f(plasma::SCROLL, s);
    }
    // The target: gone (state −x) or without collision, none; none and free to, the first moby within 15 of the
    // target list (not 0x0a7e, not Ratchet) of class type 5.
    if w.mobys[id].pv_b(plasma::FIXED) == 0 {
        let t = usize::try_from(w.mobys[id].pv_i(plasma::TARGET)).ok();
        if t.is_some_and(|t| w.mobys.get(t).is_none_or(|x| 0x80 <= x.state || !x.has_collision)) { w.mobys[id].set_pv_i(plasma::TARGET, -1); }
        if w.mobys[id].pv_i(plasma::TARGET) == -1 {
            let pos = w.mobys[id].pos;
            let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, pos, 15.0, QueryFlags(0), Some(id))).unwrap_or_default();
            let pick = near.into_iter().find(|&c| {
                let x = &w.mobys[c];
                x.mode & mode::TARGET != 0 && x.o_class != 0x0a7e && Some(c) != ctx.hero_moby && w.class(x.o_class).is_some_and(|k| k.kind == 5)
            });
            if let Some(t) = pick { w.mobys[id].set_pv_i(plasma::TARGET, t as i32); }
        }
    }
    let start = w.mobys[id].pos;
    {
        let m = &mut w.mobys[id];
        m.rows = plasma_rows(m.pv_v(plasma::VEL));
        m.pos = add(m.pos, m.pv_v(plasma::VEL));
        let mut n = m.pv_s(plasma::TICKS) as u16 as i32;
        if countdown(&mut n) {
            m.state = 2;
            m.mode |= 0x41;
        }
        m.set_pv_s(plasma::TICKS, n as u16 as i16);
        if !m.pos.iter().all(|c| WORLD.contains(c)) {
            m.pos = start;
            m.state = 2;
            m.mode |= 0x41;
        }
    }
    let tail = { let m = &w.mobys[id]; add(scale(m.rows[0], -0.3), m.pos) };
    {
        let m = &mut w.mobys[id];
        m.rows[1][1] *= 0.5;
        m.rows[2][2] *= 0.5;
    }
    // The hit: the 0.25 sphere (flags 0x10), else the step's line; not on a turret.
    let pos = w.mobys[id].pos;
    let hit = ctx.coll.and_then(|c| {
        crate::collision::coll_sphere_m(c, ctx.scene, pos, 0.25, QueryFlags(0x10), Some(id)).map(|o| (o.moby, o.pushed_centre.unwrap_or(o.point)))
            .or_else(|| crate::collision::coll_line_m(c, ctx.scene, start, pos, QueryFlags(0x10), Some(id)).map(|o| (o.moby, o.point)))
    });
    if let Some((met, at)) = hit {
        if !met.is_some_and(|t| w.mobys.get(t).is_some_and(|x| matches!(x.o_class, TURRET | UP_TURRET))) {
            let v = w.mobys[id].pv_v(plasma::VEL);
            let mut tm = HitTemplate::new(w.mobys[id].pv_f(plasma::DAMAGE), Some(id), 0x0021_0000, [v[0], v[1], v[2], 0.0]);
            tm.kind = (PLASMA as u16 as u32) << 16 | 0x0305;
            let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, pos, 1.0, QueryFlags(0), Some(id))).unwrap_or_default();
            for t in near {
                if w.mobys[t].mode & 0x5000 != 0 { w.deliver_hit_any(t, &tm, [0.0; 4]); }
            }
            crate::moby::blast::smoke_burst(w, ctx, 1.0, 13.0, Some(id), at, 0);
            let m = &mut w.mobys[id];
            m.state = 2;
            m.mode |= 0x41;
        }
    }
    {
        let m = &mut w.mobys[id];
        let s = m.pv_f(plasma::SPEED);
        let v = set_len(m.pv_v(plasma::VEL), s.min(m.pv_f(plasma::MAX)) * 0.016_666_668);
        m.set_pv_v(plasma::VEL, v);
        m.set_pv_f(plasma::SPEED, s + m.pv_f(plasma::ACCEL));
    }
    // The homing: on the target's aim point, spiralled about it (`+0x5c` / `+0x58`) within 0.75 / 0.5 of it.
    if let Some(t) = usize::try_from(w.mobys[id].pv_i(plasma::TARGET)).ok().filter(|&t| w.mobys.get(t).is_some()) {
        let side = ctx.hero.gravity.up_at(w.mobys[t].pos);
        let mut aim = aim::aim_point(w, t);
        let m = &w.mobys[id];
        let wide = m.pv_b(plasma::WIDE) != 0;
        let mut d = dist(m.pos, aim);
        if d < if wide { 1.5 } else { 0.25 } { d = 0.0; }
        let (a, b) = (m.pv_f(plasma::SPIN_A), m.pv_f(plasma::SPIN_B));
        aim[1] += b.sin() * (d * 0.749).min(f32::from_bits(0x3f3f_be77));
        aim[0] -= b.cos() * (d * 0.751).min(f32::from_bits(0x3f40_4189));
        aim[2] += (a.sin() + 1.0) * (d * 0.5).min(0.5);
        let mut turn = m.pv_f(plasma::TURN);
        let v = home(f32::from_bits(0x3cf5_c28f), 0.2, 0.0, if wide { 1.0 } else { 0.25 }, m.pos, m.pv_v(plasma::VEL), aim, &mut turn, side);
        let m = &mut w.mobys[id];
        m.set_pv_f(plasma::TURN, turn);
        m.set_pv_v(plasma::VEL, v);
    }
    // Its smoke every `+0x7a` ticks (1: each), half the step's length large.
    let moved = dist(w.mobys[id].pos, start);
    let mut every = w.mobys[id].pv_b(plasma::EVERY) as i32;
    let due = countdown(&mut every) || w.mobys[id].pv_b(plasma::EVERY) == 0;
    if due {
        let m = &w.mobys[id];
        let (at, up) = (add(scale(m.rows[0], -0.3), m.pos), ctx.hero.gravity.up_at(m.pos));
        let k = m.pv_s(plasma::PUFF);
        let (lo, hi) = (m.pv_b(plasma::LO) as i32, m.pv_b(plasma::HI) as i32);
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::plasma::puff(sys, &mut ctx.game.rng, moved * 0.5, at, up, PLASMA_ALPHA[(k % 2).unsigned_abs() as usize], lo, hi); }
        w.mobys[id].set_pv_s(plasma::PUFF, k.wrapping_add(1));
    }
    w.mobys[id].set_pv_b(plasma::EVERY, every as u8);
    {
        let g = &mut ctx.game.rng;
        let (sa, sb) = (g.randf(0.01, 0.1), g.randf(0.025, 0.25));
        let m = &mut w.mobys[id];
        m.set_pv_f(plasma::SPIN_B, add_rot(m.pv_f(plasma::SPIN_B), sa));
        m.set_pv_f(plasma::SPIN_A, crate::hero::physics::sub_rot(m.pv_f(plasma::SPIN_A), sb));
    }
    // Its trail's four points shift every second tick, the newest its tail.
    let m = &mut w.mobys[id];
    let mut n = m.pv_b(plasma::SHIFT) as i32;
    if countdown(&mut n) {
        for k in (1..4).rev() {
            let p = m.pv_v(plasma::TRAIL + 0x10 * (k - 1));
            m.set_pv_v(plasma::TRAIL + 0x10 * k, p);
        }
        m.set_pv_v(plasma::TRAIL, tail);
        n = 2;
    }
    m.set_pv_b(plasma::SHIFT, n as u8);
    w.register_draw(id);
}

/// `0x3cbaa0` → `0x3cbad0`: in state 1 the glow (`0x2ed370`, crate::fx::halo) 0.3 behind it and its streak through its
/// trail (`0x2ed6d8(scroll, 0.25, …, 4)`, crate::fx::streak), both `0x103080`.
pub fn draw_plasma(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if m.state != 1 { return; }
    let head = add(scale(m.rows[0], -0.3), m.pos);
    crate::fx::halo(out, rng, head, PLASMA_RGB, cam);
    let points: Vec<V3> = (0..4).map(|k| m.pv_v(plasma::TRAIL + 0x10 * k)).collect();
    crate::fx::streak(out, rng, m.pv_f(plasma::SCROLL), 0.25, head, &points, PLASMA_RGB, cam);
}
