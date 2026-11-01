//! The mod vendor (Endako's, ported from L03; the same unit on levels 3, 8 and 13): **0x0ddc** (`0x40a8a8`), an unseen
//! keeper standing where the machine lands, and **0x0e0d** (`0x40b110`), the machine it drops from the sky. △ in its
//! cuboid asks for the game's mode 5 with the 0x0ddc moby (`0x32dc90(5, 1, moby, 0, 0)`), which `0x374038` (L03
//! `0x375660`) opens on the mod list (list 1, `0x373268`) by its class: crate::vendor's mod menu
//! (crate::game::GameState::vendor_open, the engine's vendor_play). Its exit puts Ratchet 3.5 in front of the 0x0ddc
//! facing it and sets its state to 1 (crate::moby::classes::vendor::on_exit).
//!
//! **0x0ddc** — pvars: `[0x00]` ≠ 0 a scene's copy, `[0x04]` the cuboid of its prompt, `[0x08]` its 0x0e0d, `[0x0c]` the
//! cuboid of its scene, `[0x10]` the camera's turn speed, `[0x14]` a moby kept 20 away.
//! * **0**: a scene's copy: 3. Else it and its 0x0e0d hidden, without collision, not targets (`+0x98` 0, mode
//!   `& 0xefff | 0x41`), 1.
//! * **1**: the planet's flag byte (`0x37da18(level, 1)`: the table `0x293240` of `{level, byte}`, bit 1 of `0x1395b8 +
//!   byte`) set. Unless Ratchet is in the crane (0x31): in the prompt's cuboid the prompt 0x12eb (`0x328750(1, …)`: the
//!   lease's request), and with the lease his (`0x32bf48(1)`: owner 1, the wrench in hand, health left, not group 0x14)
//!   △ (`0x138324 & 0x10`) → 2. Then, its scene not played (`0x139644`), in the scene's cuboid: the planet's scene (3:
//!   0x35, 8: 0x5c, 13: 0x79) queued and started (`0x2f5ee8` / `0x2f6030`), `0x139644` set, 4. Hidden every tick.
//! * **2** (first tick: Ratchet walks, `0x2d4fc0`, to its frame's point (3, 0, 0) facing it): the follow camera's stick
//!   off (script bits 3) and turned until it stands on its yaw (crate::camera::CamTweaks::turn_toward: the gap eased at
//!   `0x3a7e2e99` a tick² to at most `0x3d6e4bb0` a tick on `[0x10]`). The view's yaw (`0x1b9998`, the last view's
//!   forward row) more than π/2 from its own: the machine drops ([`drop_in`] from 10 above). Ratchet there (`0x2d5010` =
//!   2), the view turned past 3.135 and the machine landed ([`landed`]): 1, Ratchet idle (`0x2caec8(0)`), mode 5 asked
//!   for. Else Ratchet walking or there (0x62..0x64) stays; otherwise 1.
//! * **3** (a scene's copy): hidden while a scene plays (`0x1a8f00` = 2: `+0x31` 0, mode bit 0), else shown.
//! * **4**: after 20 ticks (`+0x96`), back in play (`0x1a8f00` = 0): the machine put in place ([`drop_in`] at once) and
//!   mode 5 asked for (each tick until taken).
//! * **Each tick** but Ratchet in the crane: the moby `[0x14]` closer than 20 is pushed by 1/6 of (20/dx, 20/dy, 0)
//!   (the original's own sum, kept).
//!
//! **0x0e0d** — pvars `[0x00..0x10]` its home (its place at its start), `[0x10]` its speed, `[0x14]` the ticks it stays
//! once landed. **0**: home kept, hidden (as above), 1. **1**: nothing. **2** falling: its height eased to home's
//! (`0x320220(z, 0, 20/3600, 20/60)`, crate::hero::physics::ease: `$gp+0x2624` / `+0x2628` = 20); landed, its speed 0
//! and the stay counted down (`0x2efa50`), out: 3. **3** leaving: its speed toward 20/60 by 20/3600 a tick (`0x31ff20`,
//! `$gp+0x2628` / `+0x2620`), rising by it; not drawn (`+0x31` 0): hidden, 1.
//!
//! `RC_MOD_VENDOR=1` (dev): each 0x0ddc prints its place, where Ratchet stands and its cuboids' places at its start, and
//! its state changes.
//!
//! Not ported: nothing of the two classes' own; mode 5's menu is crate::vendor's (the call `0x375ea0` it makes at a
//! mod list's open is crate::vendor's too).

use crate::hero::physics::{add_rot, atan2, ease, len, rot_dist, sub, V3};
use crate::moby::{countdown, mode, Ctx, Moby, MobyWorld};
use crate::pad::button;

pub const CLASS: i16 = 0x0ddc;
pub const MACHINE: i16 = 0x0e0d;
/// "△ Activate Vendor" (`0x12eb`) and the lease's owner (`0x328750(1, …)`).
const PROMPT: u32 = 0x12eb;
const OWNER: i32 = 1;
/// `0x1395b8 + 0x8c` (crate::game::GameState::flags): its scene played.
const F_SCENE: usize = 0x8c;
/// `0x293240`: the planets' flag bytes (`{level, byte}`, −1 ends), from `0x1395b8`.
const PLANET_FLAGS: [(i32, usize); 8] = [(1, 0x9e), (3, 0x95), (4, 0x9d), (8, 0x96), (9, 0x97), (13, 0x98), (16, 0x99), (19, 0x9a)];
/// Where Ratchet stands in its frame (`0x2f00f8(3, …, +0xc0)`: its x row × 3).
const STAND: f32 = 3.0;
/// The camera's turn (`0x3206d8`'s accel = brake, most) and the views that drop the machine and open the menu.
const TURN: (u32, u32) = (0x3a7e_2e99, 0x3d6e_4bb0);
const DROP_GAP: u32 = 0x3fc9_0fdb;
const OPEN_GAP: u32 = 0x4048_a3d7;
/// `$gp+0x2620` / `+0x2624` / `+0x2628` (L03): the machine's rates, 20 each.
const RATE: f32 = 20.0;
/// The moby `[0x14]` is kept this far, and the fall's height.
const KEEP: f32 = 20.0;
const FALL: f32 = 10.0;

mod pv {
    pub const COPY: usize = 0x00;
    pub const PROMPT_IN: usize = 0x04;
    pub const MACHINE: usize = 0x08;
    pub const SCENE_IN: usize = 0x0c;
    pub const TURN_V: usize = 0x10;
    pub const KEPT: usize = 0x14;
    /// The machine's.
    pub const HOME: usize = 0x00;
    pub const SPEED: usize = 0x10;
    pub const STAY: usize = 0x14;
}

/// `RC_MOD_VENDOR=1` (module docs).
fn dev() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("RC_MOD_VENDOR").is_ok_and(|v| v.trim() == "1"))
}

fn idx(w: &MobyWorld, v: i32) -> Option<usize> { usize::try_from(v).ok().filter(|&i| i < w.mobys.len()) }

/// Hidden, without collision, not a target (`+0x98` 0, mode `& 0xefff | 0x41`).
fn hide(m: &mut Moby) {
    m.has_collision = false;
    m.mode = m.mode & 0xefff | 0x41;
}

/// `0x2d5010`: Ratchet's scripted walk: 1 walking (0x62, 0x63; or asked to: the port starts the state on his next
/// tick), 2 there (0x64), else 0.
fn walk(ctx: &Ctx) -> i32 {
    match ctx.hero.state {
        0x62 | 0x63 => 1,
        0x64 => 2,
        _ if ctx.hero.state_req.is_some_and(|r| r.id == 0x62) => 1,
        _ => 0,
    }
}

/// `0x32bf48(1)`: the prompt's lease is its own and Ratchet can answer it.
fn lease_mine(ctx: &Ctx) -> bool {
    ctx.game.prompt.owner == OWNER && ctx.hero.items.thrown == 0 && ctx.hero.hp != 0 && ctx.hero.group != 0x14
}

/// `0x32dc90(5, 1, moby, 0, 0)`: game mode 5 asked for with it (taken by the engine after the moby tick).
fn ask_menu(ctx: &mut Ctx, id: usize) { ctx.game.vendor_open = Some(id); }

/// `0x40a8a8` (0x0ddc): see the module.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let machine = idx(w, w.mobys[id].pv_i(pv::MACHINE));
    let was = w.mobys[id].state;
    match was {
        0 => {
            if dev() {
                let m = &w.mobys[id];
                let at: V3 = std::array::from_fn(|k| m.pos[k] + m.rows[0][k] * STAND);
                let place = |z: i32| ctx.level.cuboid_place(z).map(|(p, _)| p);
                println!("mod vendor #{id}: at ({:.2}, {:.2}, {:.2}) yaw {:.3}, Ratchet stands at ({:.2}, {:.2}, {:.2}); prompt cuboid {} at {:?}, scene cuboid {} at {:?}, machine {:?}, copy {}", m.pos[0], m.pos[1], m.pos[2], m.rot[2], at[0], at[1], at[2], m.pv_i(pv::PROMPT_IN), place(m.pv_i(pv::PROMPT_IN)), m.pv_i(pv::SCENE_IN), place(m.pv_i(pv::SCENE_IN)), machine, m.pv_i(pv::COPY));
            }
            if w.mobys[id].pv_i(pv::COPY) != 0 {
                w.mobys[id].set_state(3, -1);
            } else {
                w.mobys[id].set_state(1, -1);
                hide(&mut w.mobys[id]);
                if let Some(k) = machine { hide(&mut w.mobys[k]); }
            }
        }
        1 => {
            if let Some(&(_, b)) = PLANET_FLAGS.iter().find(|&&(l, _)| l == ctx.game.level) {
                if let Some(f) = ctx.game.flags.get_mut(b) { *f |= 2; }
            }
            if ctx.hero.state != 0x31 {
                if ctx.level.in_cuboid(w.mobys[id].pv_i(pv::PROMPT_IN), ctx.hero.pos) {
                    ctx.game.prompt.request(OWNER, PROMPT, 1);
                    if lease_mine(ctx) && ctx.hero.items.pressed & button::TRIANGLE != 0 { w.mobys[id].set_state(2, -1); }
                }
                let played = ctx.game.flags.get(F_SCENE).copied().unwrap_or(0) != 0;
                if !played && ctx.level.in_cuboid(w.mobys[id].pv_i(pv::SCENE_IN), ctx.hero.pos) {
                    let scene = match ctx.game.level { 3 => 0x35, 8 => 0x5c, 13 => 0x79, _ => -1 };
                    if scene != -1 && ctx.game.scenes.queue.push(scene) {
                        ctx.game.scenes.start_queue(ctx.hero.hp);
                        if let Some(f) = ctx.game.flags.get_mut(F_SCENE) { *f = 1; }
                        w.mobys[id].set_state(4, -1);
                    }
                }
            }
            hide(&mut w.mobys[id]);
        }
        2 => {
            let m = &mut w.mobys[id];
            let yaw = m.rot[2];
            if m.first() {
                let at: V3 = std::array::from_fn(|k| m.pos[k] + m.rows[0][k] * STAND);
                ctx.hero.script_walk_to(add_rot(yaw, f32::from_bits(0x4049_0fdb)), at, false);
            }
            let mut v = m.pv_f(pv::TURN_V);
            ctx.game.cam_tweaks.turn_toward(yaw, 0.0, f32::from_bits(TURN.0), f32::from_bits(TURN.1), &mut v, 3);
            m.set_pv_f(pv::TURN_V, v);
            let r = ctx.game.cameras.last_view.1[0];
            let gap = rot_dist(atan2(r[0], r[1]), yaw);
            if f32::from_bits(DROP_GAP) < gap {
                if let Some(k) = machine { drop_in(w, k, ctx, false); }
            }
            let down = machine.is_some_and(|k| landed(&w.mobys[k]));
            if walk(ctx) == 2 && f32::from_bits(OPEN_GAP) < gap && down {
                w.mobys[id].set_state(1, -1);
                ctx.hero.request_state(0, true);
                ask_menu(ctx, id);
            } else if walk(ctx) == 0 {
                w.mobys[id].set_state(1, -1);
            }
        }
        3 => {
            let m = &mut w.mobys[id];
            if ctx.game.scenes.mode == crate::scene::SCENE {
                m.drawn = 0;
                m.mode |= 1;
            } else {
                m.drawn = 1;
                m.mode &= !1;
            }
        }
        4 => {
            let t = w.mobys[id].state_ticks as u16;
            w.mobys[id].state_ticks = t.wrapping_add(1) as i16;
            if 0x14 < t && ctx.game.scenes.mode == crate::scene::PLAY {
                if let Some(k) = machine { drop_in(w, k, ctx, true); }
                ask_menu(ctx, id);
            }
        }
        _ => {}
    }
    if dev() && w.mobys[id].state != was { println!("mod vendor #{id}: state {was} -> {}", w.mobys[id].state); }
    keep_away(w, id, ctx);
}

/// The tail of `0x40a8a8`: the moby `[0x14]` within 20 pushed off (module docs).
fn keep_away(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let Some(k) = idx(w, w.mobys[id].pv_i(pv::KEPT)) else { return };
    if ctx.hero.state == 0x31 { return; }
    let mut d = sub(w.mobys[k].pos, w.mobys[id].pos);
    if KEEP <= len(d) { return; }
    for c in 0..2 {
        if d[c] != 0.0 { d[c] = KEEP / d[c]; }
    }
    d[2] = 0.0;
    let p = &mut w.mobys[k].pos;
    for c in 0..3 { p[c] += d[c] * f32::from_bits(0x3e2a_aaab); }
}

/// `0x40b388`: the machine within 0.01 of its home's height.
fn landed(m: &Moby) -> bool { (m.pos[2] - m.pv_f(pv::HOME + 8)).abs() < 0.01 }

/// `0x40b298(machine, at_once)`: unless already falling, shown and solid (mode `& 0xffbe | 0x1000`, its class's
/// collision), 2, at home with its speed 0 — not at once: 10 above, its first step taken
/// (`0x320220(home z, 1, …)`). Then mode 0x400, its shadow's slab (`0x31ee60`, crate::shadow_volumes::probe), the stay 4.
fn drop_in(w: &mut MobyWorld, k: usize, ctx: &Ctx, at_once: bool) {
    let solid = w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[k];
    if m.state != 2 {
        m.mode = m.mode & 0xffbe | mode::TARGET;
        m.has_collision = solid;
        m.set_state(2, -1);
        let home = [m.pv_f(pv::HOME), m.pv_f(pv::HOME + 4), m.pv_f(pv::HOME + 8)];
        m.pos = home;
        m.set_pv_f(pv::SPEED, 0.0);
        if !at_once {
            m.pos[2] += FALL;
            fall(m, 1.0);
        }
    }
    m.mode |= 0x400;
    crate::shadow_volumes::probe(w, k, ctx);
    w.mobys[k].set_pv_i(pv::STAY, 4);
}

/// `0x320220(home z, accel, 20/3600, 20/60, &z, &speed)`: one step of the fall.
fn fall(m: &mut Moby, accel: f32) {
    let (mut z, mut v) = (m.pos[2], m.pv_f(pv::SPEED));
    ease(m.pv_f(pv::HOME + 8), accel, RATE * 0.000_277_777_78, RATE * 0.016_666_668, &mut z, &mut v);
    m.pos[2] = z;
    m.set_pv_f(pv::SPEED, v);
}

/// `0x40b110` (0x0e0d): see the module.
pub fn update_machine(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            for c in 0..3 { m.set_pv_f(pv::HOME + 4 * c, m.pos[c]); }
            m.set_pv_f(pv::HOME + 12, 0.0);
            m.set_state(1, -1);
            hide(m);
        }
        2 => {
            fall(m, 0.0);
            if m.pos[2] <= m.pv_f(pv::HOME + 8) {
                m.set_pv_f(pv::SPEED, 0.0);
                let mut t = m.pv_i(pv::STAY);
                let out = countdown(&mut t);
                m.set_pv_i(pv::STAY, t);
                if out { m.set_state(3, -1); }
            }
        }
        3 => {
            let mut v = m.pv_f(pv::SPEED);
            v += (RATE * 0.016_666_668 - v).clamp(-RATE * 0.000_277_777_78, RATE * 0.000_277_777_78);
            m.set_pv_f(pv::SPEED, v);
            m.pos[2] += v;
            if m.drawn == 0 {
                hide(m);
                m.set_state(1, -1);
            }
        }
        _ => {}
    }
}
