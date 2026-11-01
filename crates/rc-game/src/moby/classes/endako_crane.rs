//! Endako (level 3)'s remote cranes (L03 addresses; the engine functions they call are named by their L03 addresses):
//! the terminal **0x0949** ([`update_0949`], `0x3c8030`, two placed), the crane **0x094a** ([`update_094a`],
//! `0x3c88b0`, two), the boxes it carries **0x0b85 / 0x0cfc / 0x094c** ([`update_box`], `0x3c9430`: Ghidra has no
//! function there, read from its bytes; 0x094c is not placed) and the wall the bomb box breaks **0x10f6**
//! ([`update_10f6`], `0x422a78`, also without a function). The crane's load may be a laser sentry 0x0ffd too
//! (crate::moby::classes::endako_c: its carry and drop, and the terminal's arming of one).
//!
//! **The terminal 0x0949** (its pvars: `[0]` the crane (a moby index; −1: deleted), `[1..3]` the cuboids of its three
//! views, `[4..6]` the cuboids the crane is seen from them in, `[7]` a sentry it arms, `[8]` the moby it then hooks,
//! `[0xc]` the view in use (−1 none)):
//! * **0**: no collision, not drawn, hidden (mode | 1); the view none; 1.
//! * **1 / 2** (2 once hooked, by the Electrolyzer puzzle crate::moby::classes::electrolyzer): with Ratchet on the
//!   ground within 6 of it across and in front of it (0 to 4 ahead, within 2.5 to the side) the prompt (owner 8):
//!   0x1310 while it waits, 0x12fa once hooked; △ on its prompt (`0x32bf48(8)`: the lease his, health left, not
//!   dying) puts Ratchet 2 ahead of it facing it in state 0x31 (`0x2bf080(…, 0x31, 1)`), the crane his (`0x18b650`,
//!   [`Globals::steered`]), the script camera on at its view (snapped, crate::script_camera, `0x43a7b8`), the crane
//!   steered ([`steer_on`], `0x3c9048`); 3.
//! * **3**: Ratchet kept from harm (`0x189ffc` = 5); the view followed ([`view`], `0x3c8458` → `0x43a6f0` /
//!   `0x43a748`) and, with a sentry `[7]` it arms (crate::moby::classes::endako_c::arm, `0x420768`) and `[8]`, `[8]`
//!   hooked. △ again: the camera released behind Ratchet (`0x43a928(0)`), he stands where he is (state 0), the crane let
//!   go ([`steer_off`], `0x3c9090`); 2.
//! * **The view** (`0x3c8458`): the view `[0xc]` kept while the crane is in its cuboid `[4 + view]`, else the first other
//!   whose cuboid holds it; the camera at that view's cuboid `[1 + view]`, looking at the crane's place at its start
//!   height (`+0xf8`): its yaw toward it, its pitch toward it raised 20° (`$gp+0x1114` = −20).
//!
//! **The crane 0x094a** (its pvars: `[0]` the path it is kept 1.25 inside (`0x344c08`, crate::moby::ai::fence), the
//! arm's three manipulators `+0x20` / `+0x60` / `+0xa0` (joint lists 0..2, crate::moby::Moby::joint_moves), its
//! velocity `+0xe0`, its load `+0xf0`, the rail's height `+0xf4`, its start height `+0xf8`, the hook's drop `+0xfc`
//! and its speed `+0x100`, the rest after a drop `+0x104`, let go `+0x106`, its looped sound `+0x108`, the drop's sound
//! said `+0x10c`):
//! * **0**: its own matrix (mode | 4); the rail: the line from 1 up to 20 up (none: deleted), `+0xf4` its height;
//!   `+0xf8` its height now; 1.
//! * **1 / 2** (2 steered): steered, ○ ✕ □ (`0x138324 & 0xe0`): with a load it is dropped with the crane's velocity
//!   (the boxes [`box_drop`] `0x3c9c68`, the sentry `0x420720`), the rest 30; without, rested and the hook up (the
//!   drop at least −1.5), the hook goes down (3). Let go (`+0x106`), 1. Its height eases to 6.5 under the rail
//!   (`0x320958`: 1/360 a tick², at most 1/6 a tick), the hook back up (`0x320220`: 1/360, at most 1/3), [`steer`].
//! * **3**: the hook falls (40/3600 a tick², its sound 1 once), [`steer`]; a sphere of 1 at the hook (`0x2e0558`)
//!   meeting nothing, down to −8: on. A bomb box 0x0cfc (resting: state under 2) or a box 0x0b85 with the hook at most
//!   2.7 above its place is taken, a sentry with it at most 2 above (hooked, `0x32bf18`), the drop raised to that
//!   height; met higher, on; else (the world, another moby) the hook comes back up; with a load 4, else 2.
//! * **4**: the crane slides over the load (the hook's height eased to 2.2 above a box, 2 above a sentry, 1/180 a tick²,
//!   at most 1/6; its speed eased to the distance across, then its velocity toward it at most that; kept in its path);
//!   within 0.01: 2, its sound 2, the load hung on the hook (the boxes [`box_attach`] `0x3c9c00`, the sentry
//!   `0x4206e0`).
//! * **Each tick**: the load without collision for the crane's queries and its collision back after; the arm's three
//!   joints moved down by the drop, each at most its own reach (−1.4, −2.7, −8: `$gp+0x1198..`), its matrix built
//!   (`0x315188`) and its sphere four times as big.
//! * **The steer** ([`steer`], `0x3c90c8`): steered, the stick (`0x18bdd0`, its x reversed) turned to the view's yaw
//!   + 90° gives the velocity it wants (6 a second, 3 with a box 0x0b85, 5 with another load); its looped sound 0
//!   while the stick is pushed and Ratchet steers (0x31). Not steered: its loop stopped, it wants to stop. Its velocity
//!   eases toward that across (12 a second², 3 with a box 0x0b85, 10 with another load), it moves by it, kept 1.25
//!   inside its path; the rail's height again (the line from 0.5 to 20 above the hook, flags 2); its shadow
//!   (direction 2).
//!
//! **The boxes 0x0b85 / 0x0cfc / 0x094c** (their pvars: `[0x60]` the path they are kept in, `[0x64]` the cuboid a box
//! 0x0b85 belongs in, its home `+0x70`, velocity `+0x80`, the offset under the hook `+0x90`, the crane `+0xa0`, the
//! ground's height `+0xa8`, the offset's speed `+0xac`, landed `+0xb0`, its glow `+0xb4`, the fuse `+0xb8`, the wait to
//! come back `+0xba`, the shake's timer `+0xbc`):
//! * **0**: home here; a box 0x0b85 whose uid is done starts in its cuboid; 1.
//! * **1**: every eighth frame, more than 0.05 above the ground (the line from 0.5 up, flags 2): it falls (3). A bomb
//!   0x0cfc: a hit with the flags 0x10001 lights its fuse (4: 1 tick from a box 0x0b85, else 10), as does Ratchet
//!   touching it (his push, ground, wall or ledge moby). The others, landed and Ratchet not steering: in their cuboid,
//!   put at its place and the uid counted; else kept 2.5 inside their path.
//! * **2** (hung): its offset under the hook (2.3 down) shrinks by a speed eased toward its length (1/3600 a tick², at
//!   most 1/60); at the hook plus the offset.
//! * **3** (falling): 1/120 a tick² down, kept 1.25 inside its path; at the ground: a bomb on a surface of kind 1 goes
//!   off (5), else its fuse (4); the others 1; the crane's sound 3, the shake in 2 ticks. Each tick the hits under it
//!   ([`box_hits`], `0x3c9ca0`: a sphere of 1.25 at its place (100, flags 0x10000) and the four lines round its base).
//! * **4** (the fuse, 130 ticks unless set): in each second's first 10 ticks its sound 0 and its glow 0x800000c0;
//!   out, 5.
//! * **5**: it goes off: its sound 1, a sphere of 4 at 1 up (4, flags 0x10001), the big explosion toward the camera
//!   (crate::moby::classes::explosion); hidden without collision; 6.
//! * **6**: 60 ticks, then back home when nothing is within 1.2 at 1.3 and 2.3 above it (else 30 more): shown, a target
//!   with collision, teleported in (crate::moby::classes::teleport); 1.
//! * **The shake**: when its timer runs out, the camera's up-and-down shake 0.1 for 30 ticks and its roll 0.1 for 60.
//!
//! **The wall 0x10f6** (`0x422a78`): **0** its uid done: deleted; else 1 (the table `0x259a30`'s 16 records cleared:
//! the wall pieces' own, see below). **1**: a hit with the flags 0x10000 from a bomb 0x0cfc whose place is at most 0.5
//! under the wall's: every member of its group to 2. **2**: three blasts in a row across it (`0x403f10`: 0.75, 5 of
//! each sprite, 5 puffs, 5 streaks; at (−8 / 0 / 8, −6, 5) in its frame), 230 ticks, then the row at 15 up and 10 ticks
//! (3). **3**: the row at 25 up; its uid counted, the help slot 0x3c shut, its pieces thrown (`0x32d008`: −5..5 across,
//! −15..−30 out, 4..15 up, 90..360°/s, gravity 10..20, its sound 0); deleted.
//!
//! Not ported:
//! * The terminal's view words (`0x1b9c14` / `0x1b9c28` / `0x1b9c29`: the field of view asked for, `$gp+0x1118` =
//!   180.0 on the way in and 64.42 on the way out, both read by `0x2d8740` as radians — the port's field of view is
//!   fixed) and the steering screen's overlay (`0x30ee50(0x3c8618, 0x3c8620, 0)`: the view's number and three texts,
//!   0x2be5 / 0x2dea / 0x3077 / 0x3078, and the skill point's mark; off by `0x30eea0(0)`): the port has no draw hook
//!   for a class's screen text.
//! * `0x32bf48`'s byte `0x18b05a`; the crane's `+0x9c` and mode 0x400 (nothing in the port reads them); `$gp+0x3a88`
//!   (`0x1b2a78` = −1 in the crane's set-up).
//! * The hook's place is the arm's last joint (list 2) without the manipulators, moved down by the drop as list 2's
//!   (its lists 0..2 are the root's children 1..3; the engine's joint does not apply manipulators). The original reads
//!   the joint as the last frame's pose left it.
//! * The wall's pieces' own update (`0x423008`, given to `0x32d008` when its `[0]` is set: the table `0x259a30`).
//! * `0x420768`'s and the boxes' moby `+0xa8` (the hit record let go) where nothing reads it after.

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, dot, ease, ease_speed, len, scale, set_len, sub, unit, V3};
use crate::moby::{ai, mode, Ctx, MobyWorld};
use crate::pad::button;
use super::endako_a::to_world;
use super::endako_c;

pub const PANEL: i16 = 0x0949;
pub const CRANE: i16 = 0x094a;
/// The box the crane carries to its cuboid.
pub const BOX: i16 = 0x0b85;
/// The bomb box.
pub const BOMB: i16 = 0x0cfc;
pub const BOX_094C: i16 = 0x094c;
pub const WALL: i16 = 0x10f6;

/// The terminal's prompts: waiting, and steer.
const TEXT_OFF: u32 = 0x1310;
const TEXT_USE: u32 = 0x12fa;

/// `$gp+0x1114`: the view's pitch above the crane (degrees).
const VIEW_PITCH: f32 = -20.0;
/// `$gp+0x1170`: the crane's height under the rail.
const HANG: f32 = -6.5;
/// `$gp+0x1174..+0x1188`: its speed (a second) and acceleration (a second²): empty, with a box 0x0b85, with another load.
const SPEED: [f32; 3] = [6.0, 3.0, 5.0];
const ACCEL: [f32; 3] = [12.0, 3.0, 10.0];
/// `$gp+0x118c`: the stick's turn from the view's yaw (degrees).
const STICK_TURN: f32 = 90.0;
/// `$gp+0x1190`: the hook's fall (a second²).
const DROP_ACCEL: f32 = 40.0;
/// `$gp+0x1198..+0x11a0`: each arm joint's reach (the last the hook's).
const REACH: [f32; 3] = [f32::from_bits(0xbfb3_3333), f32::from_bits(0xc02c_cccd), -8.0];
/// `$gp+0x11a4`: the crane's shadow direction.
const SHADOW_DIR: i8 = 2;

mod pn {
    pub const CRANE: usize = 0;
    pub const VIEWS: usize = 4;
    pub const ZONES: usize = 0x10;
    pub const SENTRY: usize = 0x1c;
    pub const HOOK: usize = 0x20;
    pub const VIEW: usize = 0x30;
}

mod cr {
    pub const PATH: usize = 0;
    pub const VEL: usize = 0xe0;
    pub const LOAD: usize = 0xf0;
    pub const RAIL: usize = 0xf4;
    pub const BASE: usize = 0xf8;
    pub const DROP: usize = 0xfc;
    pub const DROP_V: usize = 0x100;
    pub const REST: usize = 0x104;
    pub const FREE: usize = 0x106;
    pub const LOOP: usize = 0x108;
    pub const SAID: usize = 0x10c;
}

mod bx {
    pub const FLAGS: usize = 0x5c;
    pub const PATH: usize = 0x60;
    pub const GOAL: usize = 0x64;
    pub const HOME: usize = 0x70;
    pub const VEL: usize = 0x80;
    pub const OFF: usize = 0x90;
    pub const CRANE: usize = 0xa0;
    pub const GROUND: usize = 0xa8;
    pub const SHRINK: usize = 0xac;
    pub const LANDED: usize = 0xb0;
    pub const GLOW: usize = 0xb4;
    pub const FUSE: usize = 0xb8;
    pub const WAIT: usize = 0xba;
    pub const SHAKE: usize = 0xbc;
}

/// A moby index from a pvar word (−1 none).
fn moby_at(w: &MobyWorld, i: i32) -> Option<usize> { usize::try_from(i).ok().filter(|&i| i < w.mobys.len()) }

fn hooked(w: &MobyWorld, id: usize) -> bool { w.mobys[id].once & 4 != 0 }

/// The class's collision back (`+0x98` = the class's `+0x10`).
fn collision_back(w: &mut MobyWorld, id: usize) {
    let c = w.mobys[id].o_class;
    let solid = w.class(c).is_some_and(|c| c.has_collision);
    if w.mobys[id].has_collision != solid { w.changed = true; }
    w.mobys[id].has_collision = solid;
}

fn collision_off(w: &mut MobyWorld, id: usize) {
    if w.mobys[id].has_collision { w.changed = true; }
    w.mobys[id].has_collision = false;
}

/// The uid done before (`0x21eec4[uid]`, or its save count).
fn uid_done(ctx: &Ctx, uid: i16) -> bool {
    ctx.game.uid_dead.get(uid.max(0) as usize).copied().unwrap_or(0) != 0 || ctx.game.uid_count(uid) != 0
}

/// `0x2efa80(&s)` on a 16-bit pvar: 1 when it was out already, 2 when it runs out now, else 0.
fn count_s(w: &mut MobyWorld, id: usize, o: usize) -> u8 {
    let m = &mut w.mobys[id];
    let v = m.pv_s(o);
    if v == 0 { return 1; }
    let n = v.max(1) - 1;
    m.set_pv_s(o, n);
    if n < 1 { 2 } else { 0 }
}

/// `0x31f0f0(m, mask, 0)`: its hit record when it has one of `mask`'s flags; one without them is let go.
fn hit_with(w: &mut MobyWorld, id: usize, mask: u32) -> Option<crate::moby::hits::HitRecord> {
    let h = w.hit_message(id)?;
    if h.tmpl.flags & mask == 0 {
        w.clear_hit(id);
        return None;
    }
    Some(h)
}

/// `0x344c08(r, path, p, p)`: `p` kept `r` inside the path's walls.
fn fence(ctx: &Ctx, path: i32, r: f32, p: V3) -> V3 {
    usize::try_from(path).ok().and_then(|k| ctx.level.paths.get(k)).and_then(|pts| ai::fence(pts, r, p)).unwrap_or(p)
}

/// `0x31e228(0.5, p, 0)`: the ground under `p` (the line from 0.5 above down to 0.01, flags 2): its height (0 none)
/// and its surface kind (`0x2e3060`: −1 none).
fn ground(ctx: &Ctx, p: V3) -> (f32, i32) {
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, [p[0], p[1], p[2] + 0.5], [p[0], p[1], 0.01], crate::collision::QueryFlags(2), None));
    hit.map_or((0.0, -1), |o| (o.point[2], o.surface_id()))
}

/// `0x2e0558(r, p, 0, m)`: a sphere meeting the world or a moby (not `m`): what it met (`Some(None)`: the world).
fn sphere(ctx: &Ctx, id: usize, p: V3, r: f32) -> Option<Option<usize>> {
    ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, p, r, crate::collision::QueryFlags(0), Some(id))).map(|o| o.moby)
}

/// `0x2df4f8(a, b, flags, m, 0)`: the height where the line meets something (not `m`).
fn line_z(ctx: &Ctx, id: usize, a: V3, b: V3, flags: u32) -> Option<f32> {
    ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, crate::collision::QueryFlags(flags), Some(id))).map(|o| o.point[2])
}

// ---------------------------------------------------------------------------------------------------------------
// The terminal

/// `0x32bf48(8)`: the prompt's lease is the terminals' (owner 8) and Ratchet can answer it.
fn prompt_mine(ctx: &Ctx) -> bool { ctx.game.prompt.owner == 8 && ctx.hero.hp != 0 && ctx.hero.group != 0x14 }

/// `0x2bf080(pos, rot, state, 1)`: Ratchet placed, in `state`.
fn place_hero(ctx: &mut Ctx, pos: V3, rot: V3, state: i32) {
    ctx.hero.teleport(pos, rot);
    ctx.hero.request_state(state, true);
}

/// `0x3c8458` (module docs): the camera's place and angles; `None` without a view.
fn view(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> Option<(V3, V3)> {
    let crane = moby_at(w, w.mobys[id].pv_i(pn::CRANE))?;
    let at = w.mobys[crane].pos;
    let zone = |w: &MobyWorld, k: i32| if k < 0 { -1 } else { w.mobys[id].pv_i(pn::ZONES + 4 * k as usize) };
    let k = w.mobys[id].pv_i(pn::VIEW);
    if k == -1 || !ctx.level.in_cuboid(zone(w, k), at) {
        for i in 0..3 {
            if i == k || zone(w, i) == -1 { continue; }
            if ctx.level.in_cuboid(zone(w, i), at) {
                w.mobys[id].set_pv_i(pn::VIEW, i);
                break;
            }
        }
    }
    let k = w.mobys[id].pv_i(pn::VIEW);
    if k < 0 { return None; }
    let (pos, _) = ctx.level.cuboid_place(w.mobys[id].pv_i(pn::VIEWS + 4 * k as usize))?;
    // `0x3c90a8`: the crane's place at its start height.
    let target = [at[0], at[1], w.mobys[crane].pv_f(cr::BASE)];
    let pitch = atan2(dist2(pos, target), target[2] - pos[2]);
    let euler = [0.0, add_rot(-pitch, VIEW_PITCH * 0.017_453_294), atan2(target[0] - pos[0], target[1] - pos[1])];
    Some((pos, euler))
}

/// 0x0949 (module docs).
pub fn update_0949(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if w.mobys[id].pv_i(pn::CRANE) == -1 {
                w.delete(id);
                return;
            }
            let m = &mut w.mobys[id];
            m.set_pv_i(pn::VIEW, -1);
            m.has_collision = false;
            m.drawn = 0;
            m.mode |= 1;
            m.set_state(1, -1);
            w.changed = true;
        }
        1 | 2 => {
            let (pos, rows) = (w.mobys[id].pos, w.mobys[id].rows);
            if !(dist2(pos, ctx.hero.pos) < 6.0 && ctx.hero.grounded_ticks != 0) { return; }
            let d = sub(ctx.hero.pos, pos);
            let local = [dot(d, rows[0]), dot(d, rows[1])];
            if w.mobys[id].state == 1 && hooked(w, id) { w.mobys[id].set_state(2, -1); }
            if !(0.0 < local[0] && local[0] < 4.0 && -2.5 < local[1] && local[1] < 2.5) { return; }
            if w.mobys[id].state == 1 {
                ctx.game.prompt.request(8, TEXT_OFF, 1);
                return;
            }
            ctx.game.prompt.request(8, TEXT_USE, 1);
            if !(prompt_mine(ctx) && ctx.hero.items.pressed & button::TRIANGLE != 0) { return; }
            w.mobys[id].set_state(3, -1);
            let m = &w.mobys[id];
            let at = add(to_world([2.0, 0.0, 0.0], &m.rows), m.pos);
            let rot = [0.0, 0.0, add_rot(m.rot[2], f32::from_bits(0x4049_0fd0))];
            place_hero(ctx, at, rot, 0x31);
            let crane = moby_at(w, w.mobys[id].pv_i(pn::CRANE));
            w.endako.steered = crane;
            if let Some((p, e)) = view(w, id, ctx) { ctx.game.cameras.camera_script(p, e, crate::script_camera::MODE_SNAP, 0, false, ctx.hero); }
            if let Some(c) = crane { steer_on(w, c); }
        }
        3 => {
            ctx.hero.damage.invincible = 5;
            if ctx.hero.items.pressed & button::TRIANGLE == 0 {
                if let Some((p, e)) = view(w, id, ctx) { ctx.game.cameras.camera_script_targets(Some(p), Some(e)); }
                let (s, h) = (w.mobys[id].pv_i(pn::SENTRY), w.mobys[id].pv_i(pn::HOOK));
                if let Some(s) = moby_at(w, s) {
                    if endako_c::arm(w, s, ctx) {
                        if let Some(h) = moby_at(w, h) { w.mobys[h].once |= 4; }
                    }
                }
            } else {
                ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_CUT_BEHIND, ctx.hero);
                let (p, r) = (ctx.hero.pos, ctx.hero.rot);
                place_hero(ctx, p, r, 0);
                w.endako.steered = None;
                if let Some(c) = moby_at(w, w.mobys[id].pv_i(pn::CRANE)) { steer_off(w, c); }
                w.mobys[id].set_state(2, -1);
            }
        }
        _ => {}
    }
}

/// `0x3c9048`: the crane steered (the hook's speed 0, not let go; mode | 0x400), 2.
fn steer_on(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_f(cr::DROP_V, 0.0);
    m.set_pv_s(cr::FREE, 0);
    m.mode |= 0x400;
    m.set_state(2, -1);
}

/// `0x3c9090`: the crane let go (`+0x106`).
fn steer_off(w: &mut MobyWorld, id: usize) { w.mobys[id].set_pv_s(cr::FREE, 1); }

// ---------------------------------------------------------------------------------------------------------------
// The crane

/// The crane's load (`+0xf0`).
fn load(w: &MobyWorld, id: usize) -> Option<usize> { ai::link(&w.mobys[id], cr::LOAD) }

/// Which of [`SPEED`] / [`ACCEL`]: empty, a box 0x0b85, another load.
fn kind(w: &MobyWorld, id: usize) -> usize {
    match load(w, id) {
        None => 0,
        Some(l) if w.mobys[l].o_class == BOX => 1,
        Some(_) => 2,
    }
}

/// `0x313be0(crane, 2, out)`: the hook, the arm's last joint (module docs).
pub fn hook_point(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    let m = &w.mobys[id];
    let j = ctx.host.joint(m, 2).unwrap_or(m.pos);
    add(j, scale(m.rows[2], m.pv_f(cr::DROP).max(REACH[2])))
}

/// `0x3c90c8` (module docs).
fn steer(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut want = [0.0; 3];
    let k = kind(w, id);
    if w.mobys[id].state != 2 {
        stop_loop(w, id, ctx);
    } else {
        let yaw = add_rot(ctx.game.cameras.view_euler().1[2], STICK_TURN * 0.017_453_294);
        let rows = crate::moby::attach::rows_of([0.0, 0.0, yaw]);
        let s = ctx.hero.stick;
        if 0.0 < (s[0] * s[0] + s[1] * s[1]).sqrt() && ctx.hero.state == 0x31 {
            let h = w.mobys[id].pv_i(cr::LOOP);
            if h == -1 || !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(cr::LOOP, h);
            }
        } else {
            stop_loop(w, id, ctx);
        }
        let v = to_world([-s[0], s[1], 0.0], &rows);
        want = scale(v, SPEED[k] * 0.016_666_668);
    }
    let m = &mut w.mobys[id];
    let vel = m.pv_v(cr::VEL);
    let mut dv = sub(want, vel);
    dv[2] = 0.0;
    // `0x322b30`: at most the acceleration.
    let a = ACCEL[k] * 0.000_277_777_78;
    if a < len(dv) { dv = set_len(dv, a); }
    let vel = add(vel, dv);
    m.set_pv_v(cr::VEL, vel);
    let p = add(m.pos, vel);
    let path = m.pv_i(cr::PATH);
    w.mobys[id].pos = fence(ctx, path, 1.25, p);
    let j = hook_point(w, id, ctx);
    if let Some(z) = line_z(ctx, id, [j[0], j[1], j[2] + 0.5], [j[0], j[1], j[2] + 20.0], 2) { w.mobys[id].set_pv_f(cr::RAIL, z); }
    crate::shadow_volumes::probe(w, id, ctx);
    w.mobys[id].shadow_dir = SHADOW_DIR;
}

/// Its looped sound stopped (`0x361ac8`, `+0x108` = −1).
fn stop_loop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(cr::LOOP);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(cr::LOOP, -1);
}

/// 0x094a (module docs).
pub fn update_094a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    // The load out of the crane's own queries this tick (back in its tail; no change the snapshot needs).
    if let Some(l) = load(w, id) { w.mobys[l].has_collision = false; }
    count_s(w, id, cr::REST);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.mode = (m.mode & !0x400) | mode::NO_MATRIX;
            m.set_pv_i(cr::SAID, 0);
            m.set_pv_i(cr::LOOP, -1);
            m.set_pv_v(cr::VEL, [0.0; 3]);
            m.set_pv_f(cr::BASE, m.pos[2]);
            let p = m.pos;
            let Some(z) = line_z(ctx, id, [p[0], p[1], p[2] + 1.0], [p[0], p[1], p[2] + 20.0], 0) else {
                w.delete(id);
                return;
            };
            let m = &mut w.mobys[id];
            m.set_pv_f(cr::RAIL, z);
            m.joint_moves = (0..3).map(|l| (l, [0.0; 3])).collect();
            m.set_state(1, -1);
        }
        1 | 2 => {
            if w.mobys[id].state == 2 {
                if ctx.hero.items.pressed & (button::CIRCLE | button::CROSS | button::SQUARE) == 0 {
                    if w.mobys[id].pv_s(cr::FREE) != 0 {
                        let m = &mut w.mobys[id];
                        m.set_pv_s(cr::FREE, 0);
                        m.mode &= !0x400;
                        m.set_state(1, -1);
                    }
                } else if let Some(l) = load(w, id) {
                    let vel = w.mobys[id].pv_v(cr::VEL);
                    match w.mobys[l].o_class {
                        BOMB | BOX => box_drop(w, l, vel),
                        endako_c::SENTRY => endako_c::drop(w, l, vel),
                        _ => {}
                    }
                    collision_back(w, l);
                    let m = &mut w.mobys[id];
                    m.set_pv_s(cr::REST, 0x1e);
                    ai::set_link(m, cr::LOAD, None);
                    m.set_pv_f(cr::VEL + 8, 0.0);
                } else if w.mobys[id].pv_s(cr::REST) == 0 && -1.5 <= w.mobys[id].pv_f(cr::DROP) {
                    w.mobys[id].set_state(3, -1);
                    return tail(w, id, ctx);
                }
            }
            let m = &mut w.mobys[id];
            let mut vz = m.pv_f(cr::VEL + 8);
            ease_speed(m.pv_f(cr::RAIL) + HANG - m.pos[2], f32::from_bits(0x3b36_0b61), f32::from_bits(0x3b36_0b61), f32::from_bits(0x3e2a_aaab), &mut vz);
            m.set_pv_f(cr::VEL + 8, vz);
            let (mut d, mut dv) = (m.pv_f(cr::DROP), m.pv_f(cr::DROP_V));
            ease(0.0, f32::from_bits(0x3b36_0b61), f32::from_bits(0x3b36_0b61), f32::from_bits(0x3eaa_aaab), &mut d, &mut dv);
            m.set_pv_f(cr::DROP, d);
            m.set_pv_f(cr::DROP_V, dv);
            steer(w, id, ctx);
        }
        3 => lower(w, id, ctx),
        4 => over(w, id, ctx),
        _ => {}
    }
    tail(w, id, ctx);
}

/// State 3 (module docs).
fn lower(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let v = m.pv_f(cr::DROP_V) - DROP_ACCEL * 0.000_277_777_78;
    m.set_pv_f(cr::DROP_V, v);
    m.set_pv_f(cr::DROP, m.pv_f(cr::DROP) + v);
    if m.pv_i(cr::SAID) == 0 {
        ctx.host.play_sound(&w.mobys[id], 1, 0);
        w.mobys[id].set_pv_i(cr::SAID, 1);
    }
    steer(w, id, ctx);
    let j = hook_point(w, id, ctx);
    let met = sphere(ctx, id, j, 1.0);
    if met.is_none() && REACH[2] <= w.mobys[id].pv_f(cr::DROP) { return; }
    if let Some(o) = met.flatten() {
        let (class, oz, ostate) = (w.mobys[o].o_class, w.mobys[o].pos[2], w.mobys[o].state);
        let top = oz + if class == endako_c::SENTRY { 2.0 } else { 2.7 };
        let take = |w: &mut MobyWorld| {
            let m = &mut w.mobys[id];
            m.set_pv_f(cr::DROP, m.pv_f(cr::DROP) + (top - j[2]));
        };
        match class {
            BOMB | BOX | endako_c::SENTRY => {
                if top < j[2] { return; }
                take(w);
                if class == endako_c::SENTRY { w.mobys[o].once |= 4; }
                if class != BOMB || ostate < 2 { ai::set_link(&mut w.mobys[id], cr::LOAD, Some(o)); }
            }
            _ => {}
        }
    }
    let next = if load(w, id).is_some() { 4 } else { 2 };
    let m = &mut w.mobys[id];
    m.set_state(next, -1);
    m.set_pv_f(cr::DROP_V, 0.0);
    m.set_pv_i(cr::SAID, 0);
}

/// State 4 (module docs).
fn over(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(o) = load(w, id) else { return };
    let class = w.mobys[o].o_class;
    let mut target = w.mobys[o].pos;
    target[2] += if class == BOMB || class == BOX { 2.2 } else { 2.0 };
    let j = hook_point(w, id, ctx);
    let m = &mut w.mobys[id];
    let mut speed = len(m.pv_v(cr::VEL));
    let across = dist2(j, target);
    let mut dv = m.pv_f(cr::DROP_V);
    let k = f32::from_bits(0x3bb6_0b61);
    let most = f32::from_bits(0x3e2a_aaab);
    ease_speed(target[2] - j[2], k, k, most, &mut dv);
    m.set_pv_f(cr::DROP_V, dv);
    m.set_pv_f(cr::DROP, m.pv_f(cr::DROP) + dv);
    ease_speed(across, k, k, most, &mut speed);
    let mut vel = sub(target, j);
    vel[2] = 0.0;
    if speed < len(vel) { vel = set_len(vel, speed); }
    m.set_pv_v(cr::VEL, vel);
    let path = m.pv_i(cr::PATH);
    let p = add(m.pos, vel);
    w.mobys[id].pos = fence(ctx, path, 1.25, p);
    if 0.01 <= dist(j, target) { return; }
    let m = &mut w.mobys[id];
    m.set_state(2, -1);
    m.set_pv_f(cr::VEL + 8, 0.0);
    m.set_pv_f(cr::DROP_V, 0.0);
    match class {
        BOMB | BOX => {
            ctx.host.play_sound(&w.mobys[id], 2, 0);
            box_attach(w, o, id, ctx);
        }
        endako_c::SENTRY => {
            ctx.host.play_sound(&w.mobys[id], 2, 0);
            endako_c::carry(w, o, id);
        }
        _ => {}
    }
}

/// The end of `0x3c88b0` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state >= 0x80 { return; }
    if let Some(l) = load(w, id) {
        let solid = w.class(w.mobys[l].o_class).is_some_and(|c| c.has_collision);
        w.mobys[l].has_collision = solid;
    }
    let m = &mut w.mobys[id];
    let d = m.pv_f(cr::DROP);
    let s = if m.scale == 0.0 { 1.0 } else { m.scale };
    m.joint_moves = (0..3).map(|l| (l as u8, [0.0, 0.0, d.max(REACH[l]) * 1024.0 / s])).collect();
    w.rebuild(id, &*ctx.host);
    w.mobys[id].bsphere[3] *= 4.0;
}

// ---------------------------------------------------------------------------------------------------------------
// The boxes

/// `0x3c9c68(box, vel)`: let go with `vel`, falling (3).
pub fn box_drop(w: &mut MobyWorld, id: usize, vel: V3) {
    let m = &mut w.mobys[id];
    m.set_pv_f(bx::SHRINK, 0.0);
    ai::set_link(m, bx::CRANE, None);
    m.set_pv_v(bx::VEL, vel);
    m.set_state(3, -1);
}

/// `0x3c9c00(box, crane)`: hung on the crane's hook (2), its offset from it kept.
pub fn box_attach(w: &mut MobyWorld, id: usize, crane: usize, ctx: &Ctx) {
    let j = hook_point(w, crane, ctx);
    let m = &mut w.mobys[id];
    ai::set_link(m, bx::CRANE, Some(crane));
    m.set_pv_v(bx::OFF, sub(m.pos, j));
    m.set_state(2, -1);
}

/// `$0x29a950`: the corners round a box's base, in its frame.
const BASE: [V3; 5] = [[1.0, 1.0, -0.1], [1.0, -1.0, -0.1], [-1.0, -1.0, -0.1], [-1.0, 1.0, -0.1], [1.0, 1.0, -0.1]];

/// `0x3c9ca0` (module docs): the sphere of 1.25 at its place (100, push 1, flags 0x10000) and the lines round its base,
/// each giving what it meets a hit of 2 (flags 0x10000, `0x31e530`).
fn box_hits(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let at = w.mobys[id].pos;
    ai::attack_sphere(w, id, ctx, 1.25, 100.0, 1.0, at, 0x10000, 0, 1, 0);
    let t = crate::moby::hits::HitTemplate::new(2.0, Some(id), 0x10000, [0.0; 4]);
    let (rows, pos) = (w.mobys[id].rows, w.mobys[id].pos);
    let mut a = add(to_world(BASE[0], &rows), pos);
    for c in &BASE[1..] {
        let b = add(to_world(*c, &rows), pos);
        let hit = ctx.coll.and_then(|cl| crate::collision::coll_line_m(cl, ctx.scene, a, b, crate::collision::QueryFlags(0), Some(id)));
        if let Some((o, p)) = hit.and_then(|o| o.moby.map(|m| (m, o.point))) { w.deliver_hit_any(o, &t, [p[0], p[1], p[2], 1.0]); }
        a = b;
    }
}

/// 0x0b85 / 0x0cfc / 0x094c (module docs).
pub fn update_box(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let class = w.mobys[id].o_class;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(bx::GLOW, m.glow as i32);
            m.set_pv_f(bx::GROUND, m.pos[2]);
            let pos = m.pos;
            m.set_pv_v(bx::HOME, pos);
            m.set_pv_i(bx::FLAGS, m.pv_i(bx::FLAGS) | 1);
            let uid = m.uid;
            if class == BOX && uid_done(ctx, uid) {
                if let Some((p, _)) = ctx.level.cuboid_place(w.mobys[id].pv_i(bx::GOAL)) { w.mobys[id].pos = p; }
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if ctx.game.frame & 7 == 0 {
                let (g, _) = ground(ctx, w.mobys[id].pos);
                if g + 0.05 <= w.mobys[id].pos[2] { w.mobys[id].set_state(3, -1); }
            }
            if class == BOMB {
                if let Some(h) = hit_with(w, id, 0x10001) {
                    w.mobys[id].set_state(4, -1);
                    let from_box = h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == BOX);
                    w.mobys[id].set_pv_s(bx::FUSE, if from_box { 1 } else { 10 });
                }
                let h = &*ctx.hero;
                if [h.push_moby, h.ground_moby, h.wall_moby, h.ledge.moby].contains(&Some(id)) { w.mobys[id].set_state(4, -1); }
            } else if w.mobys[id].pv_i(bx::LANDED) != 0 && ctx.hero.state != 0x31 {
                w.mobys[id].set_pv_i(bx::LANDED, 0);
                let goal = w.mobys[id].pv_i(bx::GOAL);
                if ctx.level.in_cuboid(goal, w.mobys[id].pos) {
                    if let Some((p, _)) = ctx.level.cuboid_place(goal) { w.mobys[id].pos = p; }
                    let uid = w.mobys[id].uid;
                    ctx.game.count_uid(uid);
                } else {
                    let (path, p) = (w.mobys[id].pv_i(bx::PATH), w.mobys[id].pos);
                    w.mobys[id].pos = fence(ctx, path, 2.5, p);
                }
            }
        }
        2 => {
            let Some(c) = ai::link(&w.mobys[id], bx::CRANE) else { return shake(w, id, ctx) };
            let m = &mut w.mobys[id];
            let mut off = m.pv_v(bx::OFF);
            off[2] += 2.3;
            let l = len(off);
            if l != 0.0 {
                let mut s = m.pv_f(bx::SHRINK);
                ease_speed(l, f32::from_bits(0x3991_a2b4), f32::from_bits(0x3991_a2b4), f32::from_bits(0x3c88_8889), &mut s);
                m.set_pv_f(bx::SHRINK, s);
                off = set_len(off, (l - s).max(0.0));
            }
            off[2] -= 2.3;
            m.set_pv_v(bx::OFF, off);
            let j = hook_point(w, c, ctx);
            w.mobys[id].pos = add(j, off);
        }
        3 => {
            let m = &mut w.mobys[id];
            let mut v = m.pv_v(bx::VEL);
            v[2] -= f32::from_bits(0x3c08_8889);
            m.set_pv_v(bx::VEL, v);
            let (path, p) = (m.pv_i(bx::PATH), add(m.pos, v));
            let p = fence(ctx, path, 1.25, p);
            w.mobys[id].pos = p;
            let (g, kind) = ground(ctx, p);
            w.mobys[id].set_pv_f(bx::GROUND, g);
            if p[2] <= g {
                if class == BOMB {
                    w.mobys[id].set_state(if kind == 1 { 5 } else { 4 }, -1);
                } else {
                    w.mobys[id].set_state(1, -1);
                }
                ctx.host.play_sound_of(&w.mobys[id], CRANE, 3, 0);
                let m = &mut w.mobys[id];
                m.set_pv_i(bx::SHAKE, 2);
                m.set_pv_v(bx::VEL, [0.0; 3]);
                m.pos[2] = m.pv_f(bx::GROUND);
                m.set_pv_i(bx::LANDED, 1);
            }
            box_hits(w, id, ctx);
        }
        4 => {
            if w.mobys[id].first() && w.mobys[id].pv_s(bx::FUSE) == 0 { w.mobys[id].set_pv_s(bx::FUSE, 0x82); }
            if count_s(w, id, bx::FUSE) != 0 { w.mobys[id].set_state(5, -1); }
            if w.mobys[id].pv_s(bx::FUSE) % 0x3c < 10 {
                ctx.host.play_sound_of(&w.mobys[id], BOMB, 0, 0);
                w.mobys[id].glow = 0x8000_00c0;
            } else {
                w.mobys[id].glow = w.mobys[id].pv_i(bx::GLOW) as u32;
            }
        }
        5 => {
            let pos = w.mobys[id].pos;
            let at = [pos[0], pos[1], pos[2] + 1.0];
            ctx.host.play_sound_of(&w.mobys[id], BOMB, 1, 0);
            let dir = unit(sub(ctx.host.camera_pos(), at));
            ai::attack_sphere(w, id, ctx, 4.0, 4.0, 1.0, at, 0x10001, 0, 1, 0);
            super::explosion::start(w, ctx, 0.0, at, dir);
            collision_off(w, id);
            let m = &mut w.mobys[id];
            m.mode = (m.mode | 0x41) & !mode::TARGET;
            m.set_state(6, -1);
        }
        6 => {
            if w.mobys[id].first() { w.mobys[id].set_pv_s(bx::WAIT, 0x3c); }
            if count_s(w, id, bx::WAIT) != 0 {
                let home = w.mobys[id].pv_v(bx::HOME);
                w.mobys[id].pos = home;
                let mut p = [home[0], home[1], home[2] + f32::from_bits(0x3fa6_6666)];
                let r = f32::from_bits(0x3f99_999a);
                let clear = sphere(ctx, id, p, r).is_none() && {
                    p[2] += 1.0;
                    sphere(ctx, id, p, r).is_none()
                };
                if clear {
                    let m = &mut w.mobys[id];
                    m.set_state(1, -1);
                    m.glow = m.pv_i(bx::GLOW) as u32;
                    m.mode = (m.mode & 0xffbe) | mode::TARGET;
                    collision_back(w, id);
                    let rot = w.mobys[id].rot;
                    super::teleport::start(w, id, home, rot, &[]);
                    w.clear_hit(id);
                } else {
                    w.mobys[id].set_pv_s(bx::WAIT, 0x1e);
                }
            }
        }
        _ => {}
    }
    shake(w, id, ctx);
}

/// The boxes' shake (module docs; its 32-bit timer `0x2efa50`, 2 when it runs out now): `0x1b99a0` 0.1 for 30 ticks,
/// the roll `0x1b99c0` 0.1 for 60.
fn shake(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let t = m.pv_i(bx::SHAKE);
    if t == 0 { return; }
    let n = t.max(1) - 1;
    m.set_pv_i(bx::SHAKE, n);
    if 0 < n { return; }
    let s = &mut ctx.game.shakes;
    s.ask(0, 0.1, 0x1e);
    s.0[2].amp = 0.1;
    s.0[2].timer = 0x3c;
}

// ---------------------------------------------------------------------------------------------------------------
// The wall

/// 0x10f6 (module docs).
pub fn update_10f6(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if uid_done(ctx, w.mobys[id].uid) {
                w.delete(id);
                return;
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if let Some(h) = hit_with(w, id, 0x10000) {
                let z = w.mobys[id].pos[2];
                let bomb = h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == BOMB && z <= s.pos[2] + 0.5);
                if bomb {
                    let members = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
                    for i in members { w.mobys[i].set_state(2, -1); }
                }
            }
            w.clear_hit(id);
        }
        2 => {
            if w.mobys[id].first() {
                w.mobys[id].set_pv_i(4, 0xe6);
                wall_row(w, id, ctx, 5.0, false);
            } else if count_i(w, id) {
                wall_row(w, id, ctx, 15.0, false);
                w.mobys[id].set_pv_i(4, 10);
                w.mobys[id].set_state(3, -1);
            }
        }
        3 => {
            if !count_i(w, id) { return; }
            wall_row(w, id, ctx, 25.0, true);
            let uid = w.mobys[id].uid;
            ctx.game.count_uid(uid);
            if let Some(c) = ctx.game.help_counts.get_mut(0x3c) { *c = -1; }
            crate::moby::shards::throw_all(w, id, ctx, [-5.0, -15.0, 4.0, 0.0, 5.0, -30.0, 15.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0xffff, 0, -1, 1, 0xffff);
            w.delete(id);
        }
        _ => {}
    }
}

/// `0x2efa50(+4)`: its 32-bit timer down; true when it is (or runs) out.
fn count_i(w: &mut MobyWorld, id: usize) -> bool {
    let mut t = w.mobys[id].pv_i(4);
    let out = crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_i(4, t);
    out
}

/// Three blasts across the wall at `up` (module docs); `last` the middle one's screen flash (`+0x38`: not ported).
fn wall_row(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, up: f32, last: bool) {
    let _ = last;
    for k in -1..2 {
        let m = &w.mobys[id];
        let at = add(to_world([k as f32 * 8.0, -6.0, up], &m.rows), m.pos);
        crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 0.75, bursts: [5, 5, 5], puffs: 5, streaks: 5, light: false, shake: false });
    }
}
