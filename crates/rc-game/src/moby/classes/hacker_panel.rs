//! The hacking panel **0x0b29** (L03 `0x3e5c90`; the same unit on levels 3, 11, 12, 14, 17, 19 and 20, 10 placed,
//! ported from Endako's copy): a plate Ratchet stands on to work the Hacker (item 0x33) in the game's mode 7, the
//! puzzle crate::hacker. Its pvars: two mobys `+0` / `+4` it hooks when solved (−1 none), a level camera `+8` (−1
//! none), the list `+0xc..` (14 at most, −1 ends it: each step's kind, crate::hacker), the seed `+0x44`, the clock's
//! start `+0x48` and a new depth's bonus `+0x4c` (ticks), two music words `+0x50` / `+0x54`. Its own: the board
//! `+0x58` (index + 1 here), solved `+0x60` (the board writes it), the held item kept `+0x64`, the fade `+0x6c`.
//!
//! * **0** (the level start): solved in the save (`0x21eec4[uid]` or the uid's count): hooked with its two mobys
//!   (`0x32bf18(m, 1)`), 8; else 1.
//! * **1** with Ratchet standing on it (`0x18a15c`) and the Hacker had (`0x1a7b33`): the prompt 0x1342 (owner 0xc,
//!   `0x328808`); △ keeps the held item, asks for 0x33 (`0x18c0e8`), 2.
//! * **2** Ratchet walks (`0x2d4fc0`) to its frame's point (1.3, 0, 0) facing across it (its yaw + π); there (`0x2d5010`
//!   = 2), 3; the walk given up (or 301 ticks), the way out ([`clean_up`]; idle after the 301), 1.
//! * **3** (first tick: its sound 0, Ratchet held 0x6f with sequence 0x54 at 20, the script camera swinging from the
//!   view over 120 ticks, mode 3) each tick: Ratchet pushed (0.05 a tick at most) to the point and turned 1° a tick to
//!   face it, sequence 0x55 once 0x54 is on; the camera aimed from his frame's (−0.5, −1, 1.5) at his (0.7, 0, 0.82);
//!   not held any more: the way out, 1; after 60 ticks, 4.
//! * **4** (first tick: its sound 0, the field of view eased to 1° over 30 ticks: `0x2d8630(0x3c8efa36, …, 30, 1)`):
//!   from tick 21 the screen whitens (+0.1167 a tick, `$gp+0x3474`), at 31 the field of view back (0x3f8fea69) and the
//!   game's mode 7 asked for (`0x32dc90(7, 1, moby, 0, 0)`; refused: asked again next tick), 5. Not held: the way
//!   out, 1.
//! * **5** in mode 7: the camera at the origin facing +x, the board 0x0b28 made (`0x3e6680`; its Euler angles
//!   (0, −π/2, 0)) with the list, the seed and the clock (crate::hacker::Board::new), 6. Out of the mode for 6 ticks:
//!   the way out, 1.
//! * **6** in mode 7: the board 1.2 in front of the camera and its tick (`0x3e66f0`, crate::hacker::tick). Out of it:
//!   the way out; solved: its uid counted (with a mission), on level 19 its two mobys hooked, 7; else Ratchet idle
//!   (`0x2caec8(0)`), 1.
//! * **7** hooked with its two mobys, Ratchet idle, 8. **8** done.
//!
//! The way out (`0x3e6780`): the camera released (kind 3), the field of view back, the held item asked back, the fade
//! and the white 0, the board deleted.
//!
//! `RC_HACKER=1`: the first panel in state 1 opens its puzzle at once (mode 7, as from state 4's end), its path and
//! ways on printed; `RC_HACKER=2` also walks it to the end; `RC_HACKER=3` only prints (crate::hacker::dev).
//!
//! Not ported: the free moby slots it waits for (`0x1b2940` above 15, the board's), the class streaming of the item
//! (`0x307078` / `0x306e68`: the port loads it with the level), the hidden board made at the level start (`0x3e6680`
//! in state 0: never shown, nothing reads it), the music switch (`0x330570(+0x50, +0x54)` at 3 → 4, back to the kept
//! track `+0x70` on the way out), Ratchet's word `0x18a042` = 3, the level camera `+8`'s release (`0x43cf38`: a
//! camera class the port has not got), the board as the panel's child (`0x32be48`).

use crate::hero::physics::*;
use crate::moby::{mode, Ctx, MobyWorld};
use crate::pad::button;

pub const CLASS: i16 = 0x0b29;
/// The Hacker (its item; `0x1a7b00 + 0x33`).
pub const ITEM: i32 = 0x33;
/// `0x1342`: the prompt.
pub const PROMPT: u32 = 0x1342;

pub mod pv {
    pub const HOOKS: usize = 0;
    pub const LIST: usize = 0xc;
    pub const SEED: usize = 0x44;
    pub const START: usize = 0x48;
    pub const BONUS: usize = 0x4c;
    pub const BOARD: usize = 0x58;
    pub const SOLVED: usize = 0x60;
    pub const HELD: usize = 0x64;
    pub const FADE: usize = 0x6c;
}

/// `$gp+0x1c40`: where Ratchet stands in its frame; `$gp+0x1c50` / `+0x1c60`: the camera's eye and aim in his.
const STAND: V3 = [1.3, 0.0, 0.0];
const EYE: V3 = [-0.5, -1.0, 1.5];
const AIM: V3 = [0.7, 0.0, f32::from_bits(0x3f51_eb85)];
/// The field of view: the play's (`0x3f8fea69`) and the zoom's (`0x3c8efa36`).
const FOV: u32 = 0x3f8f_ea69;
const FOV_ZOOM: u32 = 0x3c8e_fa36;

fn idx(w: &MobyWorld, v: i32) -> Option<usize> { usize::try_from(v).ok().filter(|&i| i < w.mobys.len()) }

/// `0x2f02d8` + the position: a point of moby `m`'s frame in the world.
fn frame_point(rows: &[V3; 3], pos: V3, v: V3) -> V3 { add(std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]), pos) }

/// `RC_HACKER=1` (module docs), once.
fn dev_open() -> bool {
    static USED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    matches!(crate::hacker::dev(), 1 | 2) && !USED.swap(true, std::sync::atomic::Ordering::Relaxed)
}

/// `0x32bf18(m, 1)` on it and its two mobys.
fn hook(w: &mut MobyWorld, id: usize, me: bool) {
    if me { w.mobys[id].once |= 4; }
    for k in 0..2 {
        if let Some(i) = idx(w, w.mobys[id].pv_i(pv::HOOKS + 4 * k)) { w.mobys[i].once |= 4; }
    }
}

/// `0x2d5010`: Ratchet's scripted walk: 1 walking (0x62, 0x63; or asked to: the port starts the state on his next
/// tick), 2 there (0x64), else 0.
fn walk(ctx: &Ctx) -> i32 {
    match ctx.hero.state {
        0x62 | 0x63 => 1,
        100 => 2,
        _ if ctx.hero.state_req.is_some_and(|r| r.id == 0x62) => 1,
        _ => 0,
    }
}

/// Ratchet held (`0x18c0b4` = 0x6f; or asked to be: the port starts the state on his next tick).
fn held(ctx: &Ctx) -> bool { ctx.hero.state == 0x6f || ctx.hero.state_req.is_some_and(|r| r.id == 0x6f) }

/// `0x2caec8(0)`: Ratchet idle.
fn idle(ctx: &mut Ctx) { ctx.hero.request_state(0, true); }

/// `0x3e6780`: the way out (module docs).
pub fn clean_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.game.cameras.camera_script2(3, ctx.hero);
    let h = &mut ctx.game.hacker;
    h.zoom = None;
    h.white = 0.0;
    h.view = None;
    h.hud = None;
    h.quads.clear();
    h.board = None;
    ctx.game.puzzle.tan = None;
    ctx.hero.items.request = w.mobys[id].pv_i(pv::HELD);
    w.mobys[id].set_pv_f(pv::FADE, 0.0);
    if let Some(b) = idx(w, w.mobys[id].pv_i(pv::BOARD) - 1) {
        if w.mobys[b].state < 0x80 && w.mobys[b].o_class == crate::hacker::BOARD { w.delete(b); }
        w.changed = true;
    }
    w.mobys[id].set_pv_i(pv::BOARD, 0);
}

/// The panel's list `+0xc..` (until −1, 14 at most).
fn list(w: &MobyWorld, id: usize) -> Vec<i32> {
    (0..crate::hacker::STEPS).map(|k| w.mobys[id].pv_i(pv::LIST + 4 * k)).take_while(|&v| 0 <= v).collect()
}

/// `0x32dc90(7, 1, moby, 0, 0)`: the mode asked for; whether it was taken.
fn ask_mode(id: usize, ctx: &mut Ctx) -> bool { ctx.game.scenes.request(crate::hacker::MODE, true, id as i32, 0, ctx.hero.hp) == 0 }

/// `0x3e5c90` (0x0b29): see the module.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let first = w.mobys[id].first();
    let in_mode = ctx.game.scenes.mode == crate::hacker::MODE;
    match w.mobys[id].state {
        0 => {
            let uid = w.mobys[id].uid;
            let g = &*ctx.game;
            let done = g.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || g.uid_count(uid) != 0;
            if crate::hacker::dev() != 0 {
                let m = &w.mobys[id];
                let at = frame_point(&m.rows, m.pos, STAND);
                println!("hacker: panel #{id} uid {uid} at ({:.2}, {:.2}, {:.2}), stand at ({:.2}, {:.2}, {:.2}) facing {:.3}, solved {done}, list {:?}", m.pos[0], m.pos[1], m.pos[2], at[0], at[1], at[2], add_rot(m.rot[2], std::f32::consts::PI), list(w, id));
            }
            if done {
                w.mobys[id].set_state(8, -1);
                hook(w, id, true);
            } else {
                w.mobys[id].set_state(1, -1);
            }
        }
        1 => {
            let had = ctx.game.items.get(ITEM as usize).is_some_and(|&b| b != 0);
            if ctx.hero.ground_moby == Some(id) && had {
                ctx.game.prompt.force(0xc, PROMPT, 1);
                if ctx.hero.items.pressed & button::TRIANGLE != 0 {
                    w.mobys[id].set_pv_i(pv::HELD, ctx.hero.items.item);
                    ctx.hero.items.request = ITEM;
                    w.mobys[id].set_state(2, -1);
                }
            } else if dev_open() {
                w.mobys[id].set_pv_i(pv::HELD, ctx.hero.items.item);
                ctx.hero.request_state(0x6f, false);
                if ask_mode(id, ctx) {
                    println!("hacker: RC_HACKER: panel #{id} uid {} opens its puzzle", w.mobys[id].uid);
                    w.mobys[id].set_state(5, -1);
                }
            }
        }
        2 => {
            let m = &w.mobys[id];
            if first {
                let at = frame_point(&m.rows, m.pos, STAND);
                ctx.hero.script_walk_to(add_rot(m.rot[2], f32::from_bits(0x4049_0fdb)), at, false);
                if crate::hacker::dev() != 0 { println!("hacker: panel #{id}: Ratchet walks to ({:.2}, {:.2}, {:.2}) facing {:.3}", at[0], at[1], at[2], add_rot(m.rot[2], f32::from_bits(0x4049_0fdb))); }
            }
            let s = walk(ctx);
            if s == 2 {
                w.mobys[id].set_state(3, -1);
                return;
            }
            let t = w.mobys[id].state_ticks as u16;
            w.mobys[id].state_ticks = t.wrapping_add(1) as i16;
            if t < 0x12d {
                if s != 0 { return; }
                clean_up(w, id, ctx);
            } else {
                clean_up(w, id, ctx);
                idle(ctx);
            }
            w.mobys[id].set_state(1, -1);
        }
        3 => {
            if first {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                ctx.hero.request_state(0x6f, false);
                ctx.hero.anim_req = Some((20.0, 0x54, 0));
                let (view, euler) = ctx.game.cameras.view_euler();
                ctx.game.cameras.camera_script(view, euler, crate::script_camera::MODE_SWING, 0x78, false, ctx.hero);
                w.mobys[id].set_pv_f(pv::FADE, 0.0);
            }
            let m = &w.mobys[id];
            let to = frame_point(&m.rows, m.pos, STAND);
            let mut push = sub(to, ctx.hero.pos);
            push[2] = 0.0;
            let most = f32::from_bits(0x3d4c_ccce);
            if most < len(push) { push = set_len(push, most); }
            ctx.hero.push = push;
            // `0x3204d8(yaw + π, 1°, &yaw)`.
            let want = add_rot(m.rot[2], f32::from_bits(0x4049_0fdb));
            let step = f32::from_bits(0x3c8e_fa37);
            ctx.hero.rot[2] = add_rot(ctx.hero.rot[2], sub_rot(want, ctx.hero.rot[2]).clamp(-step, step));
            if ctx.hero_view.flags & 2 != 0 && ctx.hero_view.seq_b == 0x54 { ctx.hero.anim_req = Some((7.0, 0x55, 0)); }
            if !held(ctx) {
                clean_up(w, id, ctx);
                w.mobys[id].set_state(1, -1);
            }
            let (rows, pos) = (ctx.hero.rows, ctx.hero.pos);
            let (eye, aim) = (frame_point(&rows, pos, EYE), frame_point(&rows, pos, AIM));
            let d = sub(aim, eye);
            let yaw = atan2(d[0], d[1]);
            let pitch = -atan2(dist2(eye, aim), d[2]);
            ctx.game.cameras.camera_script_targets(Some(eye), Some([0.0, pitch, yaw]));
            let t = w.mobys[id].state_ticks as u16;
            w.mobys[id].state_ticks = t.wrapping_add(1) as i16;
            if 0x3c < t { w.mobys[id].set_state(4, -1); }
        }
        4 => {
            if first {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                ctx.game.hacker.zoom = Some((0x1e, 0x1e, f32::from_bits(FOV), f32::from_bits(FOV_ZOOM)));
            }
            if !held(ctx) {
                clean_up(w, id, ctx);
                w.mobys[id].set_state(1, -1);
                return;
            }
            let t = w.mobys[id].state_ticks as u16;
            w.mobys[id].state_ticks = t.wrapping_add(1) as i16;
            if 0x15 <= t {
                let mut f = w.mobys[id].pv_f(pv::FADE);
                approach(1.0, f32::from_bits(0x3dee_eef0), &mut f);
                w.mobys[id].set_pv_f(pv::FADE, f);
                ctx.game.hacker.white = f;
                if 0x1f <= t + 1 {
                    ctx.game.hacker.zoom = None;
                    ctx.game.puzzle.tan = Some(crate::moby::classes::electrolyzer::tan());
                    if ask_mode(id, ctx) { w.mobys[id].set_state(5, -1); }
                }
            }
        }
        5 => {
            if !in_mode {
                let t = w.mobys[id].state_ticks as u16;
                w.mobys[id].state_ticks = t.wrapping_add(1) as i16;
                if t < 6 { return; }
                clean_up(w, id, ctx);
                w.mobys[id].set_state(1, -1);
                return;
            }
            ctx.game.hacker.view = Some(([0.0; 3], crate::hacker::VIEW_ROWS));
            // `0x3e6680`: the board, its turn kept (`+0x34` |= 0x100).
            let b = w.create(crate::hacker::BOARD);
            {
                let bm = &mut w.mobys[b];
                bm.mode = (bm.mode | mode::KEEP_ROWS) & !0x81;
                bm.rot = [0.0, f32::from_bits(0xbfc9_0fdb), 0.0];
                bm.rows = rc_formats::moby_light::rotation_rows(bm.rot).map(|r| [f32::from_bits(r[0]), f32::from_bits(r[1]), f32::from_bits(r[2])]);
                bm.pos = set_len(crate::hacker::VIEW_ROWS[0], crate::hacker::BOARD_DIST);
            }
            w.rebuild(b, &*ctx.host);
            w.changed = true;
            let m = &w.mobys[id];
            let kinds = list(w, id);
            let board = crate::hacker::Board::new(b, id, &kinds, m.pv_i(pv::SEED) as i16, m.pv_i(pv::START) as i16, m.pv_i(pv::BONUS) as i16);
            w.mobys[id].set_pv_i(pv::BOARD, b as i32 + 1);
            ctx.game.hacker.board = Some(board);
            w.mobys[id].set_state(6, -1);
        }
        6 => {
            if in_mode {
                let (eye, rows) = ([0.0; 3], crate::hacker::VIEW_ROWS);
                ctx.game.hacker.view = Some((eye, rows));
                if let Some(b) = ctx.game.hacker.board.as_ref().map(|b| b.moby).filter(|&b| w.mobys.get(b).is_some_and(|m| m.state < 0x80)) {
                    w.mobys[b].pos = add(set_len(rows[0], crate::hacker::BOARD_DIST), eye);
                    crate::hacker::tick(w, ctx);
                }
                return;
            }
            clean_up(w, id, ctx);
            if w.mobys[id].pv_i(pv::SOLVED) != 0 {
                w.mobys[id].set_state(7, -1);
                let m = &w.mobys[id];
                let (uid, mission) = (m.uid, m.mission);
                if mission != 0xff { ctx.game.count_uid(uid); }
                if ctx.game.level == 0x13 { hook(w, id, false); }
                return;
            }
            idle(ctx);
            w.mobys[id].set_state(1, -1);
        }
        7 => {
            hook(w, id, true);
            idle(ctx);
            w.mobys[id].set_state(8, -1);
        }
        _ => {}
    }
    // The field of view's ease (`0x2d8740`, the camera's step after the mobys).
    if let Some(tan) = ctx.game.hacker.zoom_step() { ctx.game.puzzle.tan = Some(tan); }
}
