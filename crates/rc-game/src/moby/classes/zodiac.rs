//! The Zodiac (item 0x2b, crate::hero::items::ROWS; L00's record `0x264aa0`: attachment 0, stance 1, icon 0xea76, use
//! sequence 0x94 (crouched 0x3e), a shot every 4 s (rate 0.25), damage 100, 4 charges (4 with it), name 0x32fb,
//! description 0x2f2a, no upgrade row; sold from level 14, crate::vendor::STOCK_LEVELS): the hand moby **0x0ec8**
//! (`0x40a358`, called by the item pass while it is the hand item) and its lightning **0x139e** (`0x4185a8`, made by
//! `0x418468`). A charge clears the screen: every enemy the search finds round the view dies at once.
//!
//! **The hand** (pvars: `+0x00` the cooldown (seconds), `+0x04` the hits' ticks, `+0x08` the white-out's ticks, `+0x0c`
//! −1, `+0x10` the ticks of a step, `+0x14` the big lightning made, `+0x18` the pad rumble's handle, `+0x1c` the kills).
//! Its state 0 (`0x40a920`): the ticks 0x40 / 0x32, the rest cleared, its size the class's × 0.8, state 1. Each tick
//! the look stance 1 goes to its twin 0x1d; its sequence run out, back to 1 (blend 2); the cooldown runs down. By state:
//! * **1** (ready): in play (`0x1a8f00` 0), the items not kept away (`0x18c0dc`), ○ pressed (`0x138324 & 0x18b050`) and
//!   in the hand past 0x16 ticks — with a charge left the use stance (`0x2af5e0`), the hits' ticks 5 (`$gp+0x1f40`) and
//!   **2**; none left its class sound 4. Either way the cooldown is 1 / the row's rate (read by nothing here).
//! * **2** (raising it, the 0x32 ticks): with 10 left its sequence 2 (blend 3) and its class sound 1; out, **4** for 30.
//! * **4** (the summons, 30 ticks): on its first tick the big lightning ([`make_effect`], kind 1) one ahead of the gun
//!   along Ratchet's facing; out, **5** for the 0x40 ticks, its sequence 3 (blend 3).
//! * **5** (the white-out, 0x40 ticks): its draw ([`draw_flash`], `0x40a980`): four additive full-screen fills of
//!   `0x80 − 2·ticks` in every byte (ALPHA 0x48), the screen going white; out, **3**, its sequence 4 (blend 3).
//! * **3** (the hits, 5 ticks): the hand's claims age (crate::moby::claims, `0x37f440`, the hand the owner) and the aim
//!   search from the gun along the camera's forward row (`0x3257a0(π, 60°, 40, …, list of 0x20)`, no crates,
//!   crate::moby::aim::search_list) lists the targets; each one's point is its place raised 1.5 × its aim height. One
//!   above 100 health loses 20 of it (no hit, not below 0); else (not dead, state −2 / −3) 0x0a8d takes sub-state 9,
//!   0x0a8e sub-state 7, 0x1002 state 7, and by its health block's `+0xe`: 1 or 2 (it dies its own way) it is marked 2,
//!   3 nothing, else it is deleted — either way, but 3, a small lightning at its point ([`make_effect`], kind 0), its
//!   bolts (`0x31adc0(m, 0, 3)`, crate::moby::bolts) and a kill counted. Out: its class sound 2 (3 with no kill), state
//!   0 and a charge spent (`0x2d42d0(−1, 1)`).
//!
//! **The lightning** 0x139e ([`make_effect`], `0x418468(at, kind, hand)`; pvars `+0x00` its ticks (60), `+0x04` its kind
//! (1 the big one at the gun, 0 a kill's), `+0x08` the hand): made far 0xff, at `at`, its rows facing the camera.
//! Each tick ([`update_effect`], `0x4185a8`) until its ticks run out: a kill's sheds its sparks and puffs
//! ([`sparks`], `0x4188b0`); a random way (`0x31bb00` twice: yaw, pitch) 7 out (the big one's 5), cut where the
//! line from it meets the world (flags 0, itself left out), is the end of a new arc ([`Arcs::start`], `0x418f40`, 20
//! ticks); its arcs move ([`Arcs::step`], `0x419270`); its draw. Out: every arc's record freed (`0x25a93c`, all 24 of
//! them, whoever made them), a kill's ash ([`ash`], `0x418718`), deleted.
//!
//! **The arcs** (`0x25a8a0`, 24 records of 0xb0, [`Arc`]): from a start (the moby; the big one's: the gun's (−0.15,
//! −0.8, 0) in its rows) to an end that grows over 20 ticks toward the target (a kill's: by the share of its ticks
//! spent; the big one's 0.75 of the way at once), the two tangents of its curve swinging (three angles each at
//! 0.001..0.005 · 50 a tick (100 once grown), cos / sin / −sin of them × 3.25..4.75), the start shaking ±0.1 a tick
//! (the target kept off it as it was), then 20 ticks more and freed.
//!
//! **The draw** ([`draw_effect`], `0x418c00`): each of its arcs ([`draw_arc`], `0x418d68`): 24 points along the curve
//! (`0x31bca0`) each shaken 0.075 a random way, a 25th past the last; a strip through them (`0x419730`, FX 0x2b, ALPHA
//! 0x68: the colour × the fixed alpha 0..0x40 (0x40..0x80 in its last 10 ticks) added) 0.04 to each side across the
//! view, white at one edge, the arc's colour at the other (a kill's red `0x800000ff`; the big one's from red to blue by
//! the first record's ticks), grey ahead; and over it one 0.16 to a side at a third of the alpha, its texture's s
//! stepping 0.33 a point. A grown arc's end glows (`0x2edf78`: FX 0x31 blue `0x3f7f0000` and white `0x5f7f7f7f`, FX 0xb
//! dim blue `0x103f0000`, 0.625..1.25, 0.3125..0.625 and 0.75..1.75 across, pushed 1.25 / 1.24 / 1.23 toward the camera).
//!
//! **The particles** ([`sparks`], `0x4188b0`): three cinders (type 0x5b, `0x33fbb8`: 0.65..0.85 × 210000, falling 0.5,
//! within ±0.3, 0..0.5 a second down at 55°..80°, `0x4f808080` → `0x08101010`, 35..50 ticks, a trail, frame 1) and four
//! exhaust puffs riding it (type 0x4a, `0x33d400`: its up × 0.2 a tick, at 0.1..0.4, 0.9..1.2, 0.8..1.5 and 0 above
//! it, the descriptors `$gp+0x23c8`, `+0x23a8`, `+0x23b8`, `+0x23d8`, additive or not at random). The ash ([`ash`],
//! `0x418718`): 24 flakes (type 0x66, crate::particles::ash) within ±0.25 of it, 300 ticks.
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x2b): crate::hero::items.
//!
//! Not ported: the pad rumble (`0x2d6d60(0x29fba0)`), the word `0x25c5cc` set to 0.1 while drawn (no reader found);
//! the flakes' velocity the ash computes (4/60 a tick across the view) is left unused by the original (it passes the
//! zero vector `0x1a8f20`), as here.

use crate::collision::QueryFlags;
use crate::hero::items::{row, ItemRow};
use crate::hero::physics::{add, cross, scale, set_len, sub, V3};
use crate::moby::{aim, countdown, Ctx, MobyWorld};

/// The hand's class (item 0x2b's row `+0x04`) and the lightning's (`0x418468`).
pub const HAND_CLASS: i16 = 0xec8;
pub const EFFECT: i16 = 0x139e;
/// The item.
pub const ITEM: i32 = 0x2b;
/// The arc records (`0x25a8a0`).
pub const ARCS: usize = 0x18;
/// `0x40490fdc`: the search's yaw limit (all round) and `0x3f860a93` its pitch limit (60°).
const ALL_ROUND: f32 = f32::from_bits(0x4049_0fdc);
const PITCH: f32 = f32::from_bits(0x3f86_0a93);
/// The search's range.
const RANGE: f32 = 40.0;
/// `$gp+0x1f40`: the hits' ticks.
const HIT_TICKS: i32 = 5;
/// The exhaust descriptors (crate::particles::EXHAUST_DESCS): `$gp+0x23c8`, `+0x23a8`, `+0x23b8`, `+0x23d8`.
const PUFFS: [usize; 4] = [crate::particles::ZODIAC_DESC, crate::particles::ZODIAC_DESC + 1, crate::particles::ZODIAC_DESC + 2, crate::particles::ZODIAC_DESC + 3];

mod pv {
    pub const COOL: usize = 0x00;
    pub const HITS: usize = 0x04;
    pub const WHITE: usize = 0x08;
    pub const STEP: usize = 0x10;
    pub const MADE: usize = 0x14;
    pub const KILLS: usize = 0x1c;
    /// The lightning's.
    pub const TICKS: usize = 0x00;
    pub const KIND: usize = 0x04;
    pub const HAND: usize = 0x08;
}

/// One arc (`0x25a8a0 + 0xb0·k`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Arc {
    /// `+0x00` / `+0x10`: the curve's tangents at its start and end.
    pub t0: V3,
    pub t1: V3,
    /// `+0x20` its start, `+0x30` its end now, `+0x40` its target.
    pub start: V3,
    pub end: V3,
    pub target: V3,
    /// `+0x50..+0x58` / `+0x74..+0x7c`: the tangents' angles; `+0x5c..` / `+0x80..` their speeds; `+0x68..` / `+0x8c..`
    /// their lengths.
    pub a: [f32; 3],
    pub a_speed: [f32; 3],
    pub a_len: [f32; 3],
    pub b: [f32; 3],
    pub b_speed: [f32; 3],
    pub b_len: [f32; 3],
    /// `+0x98`: the ticks after it has grown; `+0x9c` its lightning (0: free); `+0xa0` / `+0xa4` the growth's ticks left
    /// and in all; `+0xa8` grown.
    pub fade: i32,
    pub owner: Option<usize>,
    pub grow: i32,
    pub grow_of: i32,
    pub grown: bool,
}

/// The records.
#[derive(Clone, Debug, Default)]
pub struct Arcs(pub [Arc; ARCS]);

/// The big lightning's start: the gun's (−0.15, −0.8, 0) in its rows (`0x2ef318`).
fn gun_point(w: &MobyWorld, hand: Option<usize>, fallback: V3) -> V3 {
    match hand.and_then(|h| w.mobys.get(h)).filter(|h| h.state < 0x80) {
        Some(h) => {
            let r = h.rows;
            let v = [-0.15, -0.8, 0.0];
            add(h.pos, std::array::from_fn(|c| r[0][c] * v[0] + r[1][c] * v[1] + r[2][c] * v[2]))
        }
        None => fallback,
    }
}

/// The lightning's hand (`+0x08`) when it is the big one.
fn hand_of(w: &MobyWorld, id: usize) -> Option<usize> {
    let m = &w.mobys[id];
    if m.pv_i(pv::KIND) == 0 { return None; }
    usize::try_from(m.pv_i(pv::HAND)).ok().filter(|&h| w.mobys.get(h).is_some())
}

impl Arcs {
    /// `0x418f40(m, target, n)`: the first free record (none: nothing) for lightning `id`.
    pub fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, target: V3, n: i32) {
        let Some(k) = w.zodiac.0.iter().position(|a| a.owner.is_none()) else { return };
        let big = w.mobys[id].pv_i(pv::KIND) != 0;
        let from = if big { gun_point(w, hand_of(w, id), w.mobys[id].pos) } else { w.mobys[id].pos };
        let g = &mut ctx.game.rng;
        let mut a = Arc { owner: Some(id), start: from, target, ..Arc::default() };
        for c in 0..3 { a.start[c] += g.randf(-0.1, 0.1); }
        let pi = std::f32::consts::PI;
        for v in a.a.iter_mut() { *v = g.randf(-pi, pi); }
        for v in a.b.iter_mut() { *v = g.randf(-pi, pi); }
        for v in a.a_speed.iter_mut() { *v = g.randf_signed(0.001, 0.005); }
        for v in a.b_speed.iter_mut() { *v = g.randf_signed(0.001, 0.005); }
        for v in a.a_len.iter_mut() { *v = g.randf(3.25, 4.75); }
        for v in a.b_len.iter_mut() { *v = g.randf(3.25, 4.75); }
        (a.grow_of, a.fade, a.grown, a.grow) = (n, 0x14, false, n);
        w.zodiac.0[k] = a;
    }

    /// `0x419270(m)`: lightning `id`'s arcs move (module docs).
    pub fn step(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
        use crate::hero::physics::add_rot;
        let big = w.mobys[id].pv_i(pv::KIND) != 0;
        let from = if big { gun_point(w, hand_of(w, id), w.mobys[id].pos) } else { w.mobys[id].pos };
        let g = &mut ctx.game.rng;
        for a in w.zodiac.0.iter_mut().filter(|a| a.owner == Some(id)) {
            for c in 0..3 { a.start[c] += g.randf(-0.1, 0.1); }
            let off = sub(a.target, a.start);
            a.start = from;
            a.target = add(a.start, off);
            let k = if a.grow == 0 { 100.0 } else { 50.0 };
            for c in 0..3 {
                a.a[c] = add_rot(a.a[c], a.a_speed[c] * k);
                a.b[c] = add_rot(a.b[c], a.b_speed[c] * k);
            }
            a.t0 = [a.a[0].cos() * a.a_len[0], a.a[1].sin() * a.a_len[1], -a.a[2].sin() * a.a_len[2]];
            a.t1 = [a.b[0].cos() * a.b_len[0], a.b[1].sin() * a.b_len[1], -a.b[2].sin() * a.b_len[2]];
            let t = if big { 0.75 } else { 1.0 - a.grow as f32 / a.grow_of as f32 };
            a.end = add(a.start, scale(sub(a.target, a.start), t));
            if countdown(&mut a.grow) {
                a.grown = true;
                if countdown(&mut a.fade) { a.owner = None; }
            }
        }
    }
}

/// The Zodiac's row as `0x139568[0x2b]` gives it.
fn cur_row(game: &crate::game::GameState) -> Option<&'static ItemRow> { row(game, ITEM) }

/// `0x2eeab0` on the word at `off`.
fn count(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut n = w.mobys[id].pv_i(off);
    let done = countdown(&mut n);
    w.mobys[id].set_pv_i(off, n);
    done
}

/// `0x40a358` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    // `0x40a920`: its set-up.
    if w.mobys[id].state == 0 {
        let class_scale = w.class(w.mobys[id].o_class).map_or(1.0, |c| c.scale);
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::WHITE, 0x40);
        m.set_pv_i(pv::STEP, 0x32);
        m.set_pv_f(pv::COOL, 0.0);
        m.set_pv_i(0xc, -1);
        m.set_pv_i(pv::MADE, 0);
        m.state = 1;
        m.set_pv_i(0x18, -1);
        m.scale = class_scale * 0.8;
    }
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_a == a.seq_b && a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 2); }
    }
    let gun = w.mobys[id].pos;
    {
        let m = &mut w.mobys[id];
        let c = m.pv_f(pv::COOL);
        m.set_pv_f(pv::COOL, if 1.0 / 60.0 < c { c - 1.0 / 60.0 } else { 0.0 });
    }
    match w.mobys[id].state {
        1 => {
            let it = ctx.hero.items;
            if ctx.game.scenes.mode == crate::scene::PLAY && it.no_use == 0 && it.pressed & it.trigger != 0 && 0x16 < it.item_ticks {
                if !ctx.hero.items.has_ammo(ctx.game) {
                    ctx.host.play_sound(&w.mobys[id], 4, 0);
                } else {
                    ctx.hero.items.use_start(ctx.hero.group);
                    w.mobys[id].state = 2;
                    w.mobys[id].set_pv_i(pv::HITS, HIT_TICKS);
                }
                let rate = cur_row(ctx.game).map_or(1.0, |r| r.rate);
                w.mobys[id].set_pv_f(pv::COOL, 1.0 / rate);
            }
        }
        2 => {
            if count(w, id, pv::STEP) {
                let m = &mut w.mobys[id];
                m.state = 4;
                m.set_pv_i(pv::STEP, 0x1e);
                m.set_pv_i(pv::MADE, 0);
            }
            if w.mobys[id].pv_i(pv::STEP) == 10 {
                ctx.host.set_anim(&w.mobys[id], 2, 0, 3);
                ctx.host.play_sound(&w.mobys[id], 1, 0);
            }
        }
        4 => {
            if !count(w, id, pv::STEP) {
                if w.mobys[id].pv_i(pv::MADE) != 0 { return; }
                let ahead = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or(ctx.hero.rows[0], |h| h.rows[0]);
                make_effect(w, ctx, add(gun, set_len(ahead, 1.0)), 1, Some(id));
                w.mobys[id].set_pv_i(pv::MADE, 1);
                return;
            }
            let m = &mut w.mobys[id];
            m.state = 5;
            m.set_pv_i(pv::MADE, 0);
            m.set_pv_i(pv::STEP, 0x1e);
            ctx.host.set_anim(&w.mobys[id], 3, 0, 3);
        }
        5 => {
            if !count(w, id, pv::WHITE) {
                // `0x2e7920(0x40a980, hand)`: the white-out.
                w.register_draw(id);
                return;
            }
            w.mobys[id].state = 3;
            w.mobys[id].set_pv_i(pv::WHITE, 0x40);
            ctx.host.set_anim(&w.mobys[id], 4, 0, 3);
        }
        3 => hits(w, id, ctx, gun),
        _ => {}
    }
}

/// State 3 of `0x40a358` (module docs).
fn hits(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, gun: V3) {
    {
        let mut c = std::mem::take(&mut w.claims);
        c.age(w, id);
        w.claims = c;
    }
    let cone = aim::Cone { yaw: ALL_ROUND, pitch: PITCH, range: RANGE, near_pitch: 0.0, near_yaw: 0.0 };
    let forward = ctx.host.camera_rows()[0];
    let list = aim::search_list(w, ctx, cone, gun, forward, false, 0x20);
    for t in list {
        if w.mobys.get(t).is_none() { continue; }
        let mut p = w.mobys[t].pos;
        let health = (w.mobys[t].mode & crate::moby::ai::AI != 0).then(|| crate::moby::ai::health(&w.mobys[t]));
        let mut kill = true;
        if let Some(h) = health {
            p[2] += w.mobys[t].pv_f(h + 0x10) * 1.5;
            let hp = w.mobys[t].pv_f(h);
            if 100.0 < hp {
                kill = false;
                w.mobys[t].set_pv_f(h, (hp - 20.0).max(0.0));
            }
        }
        if kill {
            let m = &mut w.mobys[t];
            if !matches!(m.state, 0xfe | 0xfd) {
                match m.o_class {
                    0x0a8d => m.sub_state = 9,
                    0x0a8e => m.sub_state = 7,
                    0x1002 => m.state = 7,
                    _ => {}
                }
            }
            // The health block's `+0xe` (crate::moby::classes::swamp_monster: 1 it dies its own way, 2 asked to, 3 dying).
            let Some(h) = health else { continue };
            let kind = w.mobys[t].pv_b(h + 0xe);
            if kind == 3 { continue; }
            make_effect(w, ctx, p, 0, None);
            let n = w.mobys[id].pv_i(pv::KILLS);
            w.mobys[id].set_pv_i(pv::KILLS, n + 1);
            crate::moby::bolts::drop(w, t, 0, 3, ctx);
            if kind == 1 || kind == 2 {
                w.mobys[t].set_pv_b(h + 0xe, 2);
            } else {
                w.delete(t);
            }
        }
    }
    if !count(w, id, pv::HITS) { return; }
    let sound = if w.mobys[id].pv_i(pv::KILLS) == 0 { 3 } else { 2 };
    ctx.host.play_sound(&w.mobys[id], sound, 0);
    w.mobys[id].set_pv_i(pv::KILLS, 0);
    w.mobys[id].state = 0;
    ctx.hero.items.take_ammo(ctx.game, 1);
}

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// `0x418468(at, kind, hand)` (module docs), when the class is loaded.
pub fn make_effect(w: &mut MobyWorld, ctx: &mut Ctx, at: V3, kind: i32, hand: Option<usize>) -> Option<usize> {
    if w.class(EFFECT).is_none() { return None; }
    let id = w.create(EFFECT);
    let to_cam = set_len(sub(ctx.host.camera_pos(), at), 1.0);
    let up = ctx.hero.gravity.up_at(at);
    let r1 = set_len(cross_ba(to_cam, up), 1.0);
    let r2 = cross_ba(r1, to_cam);
    let m = &mut w.mobys[id];
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.draw_dist = 0xff;
    m.update_dist = 0xff;
    m.state = 0;
    m.pos = at;
    m.mode |= crate::moby::mode::KEEP_ROWS;
    m.rows = [to_cam, r1, r2];
    m.set_pv_i(pv::HAND, hand.map_or(-1, |h| h as i32));
    m.set_pv_i(pv::TICKS, 0x3c);
    m.set_pv_i(pv::KIND, kind);
    Some(id)
}

/// `0x4185a8` (module docs).
pub fn update_effect(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if count(w, id, pv::TICKS) {
        for a in w.zodiac.0.iter_mut() { a.owner = None; }
        if w.mobys[id].pv_i(pv::KIND) == 0 { ash(w, id, ctx); }
        w.delete(id);
        return;
    }
    let mut far = 5.0;
    if w.mobys[id].pv_i(pv::KIND) == 0 {
        sparks(w, id, ctx);
        if w.mobys[id].pv_i(pv::KIND) == 0 { far = 7.0; }
    }
    let (yaw, pitch) = (ctx.game.rng.rand_angle(), ctx.game.rng.rand_angle());
    let pos = w.mobys[id].pos;
    // `0x325100(far, yaw, pitch)`.
    let mut to = add(pos, [yaw.cos() * far * pitch.cos(), yaw.sin() * far * pitch.cos(), pitch.sin() * far]);
    if let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, pos, to, QueryFlags(0), Some(id))) { to = o.point; }
    Arcs::start(w, id, ctx, to, 0x14);
    Arcs::step(w, id, ctx);
    w.register_draw(id);
}

/// `0x4188b0` (module docs): a kill's cinders and puffs.
fn sparks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pos = w.mobys[id].pos;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for _ in 0..3 {
        let pitch = g.randf(f32::from_bits(0xbf75_be0d), f32::from_bits(0xbfb2_b8c4));
        let yaw = g.rand_angle();
        let r = g.randf(0.0, 0.5) * 0.016_666_668;
        let vel = [yaw.cos() * r * pitch.cos(), yaw.sin() * r * pitch.cos(), pitch.sin() * r];
        let size = g.randf(0.65, 0.85) * 210_000.0;
        let at: V3 = std::array::from_fn(|c| pos[c] + g.randf_signed(0.0, 0.3));
        let life = g.rand_range(0x23, 0x32);
        crate::particles::cinder::cinder(sys, g, size, 0.5, at, vel, 0x4f80_8080, 0x0810_1010, life, 1, Some(1), None, 1);
    }
    let vel = set_len(ctx.hero.gravity.up_at(pos), 0.2);
    let m = &w.mobys[id];
    for (k, lift) in [(0.1, 0.4), (0.9, 1.2), (0.8, 1.5), (0.0, 0.0)].into_iter().enumerate() {
        let z = if k < 3 { g.randf(lift.0, lift.1) } else { 0.0 };
        let additive = g.randi(2) != 0;
        if let Some(i) = crate::particles::exhaust(sys, g, m, id, [pos[0], pos[1], pos[2] + z], vel, PUFFS[k]) {
            if !additive { sys.pool.recs[i][3] = 0x44; }
        }
    }
}

/// `0x418718` (module docs): a kill's ash.
fn ash(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pos = w.mobys[id].pos;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for _ in 0..0x18 {
        // The 1..2 drawn and the velocity across the view are computed and left unused (module docs).
        let _ = g.randf_signed(1.0, 2.0);
        let at: V3 = std::array::from_fn(|c| pos[c] + g.randf_signed(-0.25, 0.25));
        let _ = g.randf_signed(0.0, f32::from_bits(0xbfc9_0fdc));
        crate::particles::ash::ash(sys, g, at, [0.0; 3], 300);
    }
}

/// `0x40a980` (module docs): the white-out's fills.
pub fn draw_flash(w: &mut MobyWorld, id: usize) {
    let u = (0x80 - 2 * w.mobys[id].pv_i(pv::WHITE)) as u32 & 0xff;
    for _ in 0..4 { w.screen_fills.push(u * 0x0101_0101); }
}

/// `0x31bca0(t, out, p0, p3, c1, c2)`: the curve's point at `t`.
pub(super) fn curve(t: f32, p0: V3, p3: V3, c1: V3, c2: V3) -> V3 {
    if t == 0.0 { return p0; }
    if t == 1.0 { return p3; }
    let (t2, t3) = (t * t, t * t * t);
    std::array::from_fn(|k| {
        let a = c1[k] - p0[k];
        let d = (c2[k] - p3[k]) - a;
        d * t3 + (a - d) * t2 + (p3[k] - c1[k]) * t + p0[k]
    })
}

/// `0x418c00` (module docs): lightning `id`'s arcs and their ends' glows.
pub fn draw_effect(w: &mut MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let big = w.mobys[id].pv_i(pv::KIND) != 0;
    let first_fade = w.zodiac.0[0].fade;
    for k in 0..ARCS {
        let a = w.zodiac.0[k];
        if a.owner != Some(id) { continue; }
        draw_arc(&a, big, first_fade, rng, cam, out);
        if a.grow != 0 { continue; }
        let s = rng.randf(0.125, 0.25);
        crate::fx::sprite(out, s * 5.0, 1.25, 0.0, a.end, cam, 0x31, 0x3f7f_0000, true);
        let s = rng.randf(0.0625, 0.125);
        crate::fx::sprite(out, s * 5.0, f32::from_bits(0x3f9e_b852), 0.0, a.end, cam, 0x31, 0x5f7f_7f7f, true);
        let s = rng.randf(0.15, 0.35);
        crate::fx::sprite(out, s * 5.0, f32::from_bits(0x3f9d_70a4), 0.0, a.end, cam, 0xb, 0x103f_0000, true);
    }
}

/// `0x418d68(m, arc)` → `0x419730(m, alpha, points, 25)` (module docs).
fn draw_arc(a: &Arc, big: bool, first_fade: i32, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let mut p = [[0.0f32; 3]; 25];
    let (c1, c2) = (add(a.t0, a.start), add(a.t1, a.end));
    for (i, q) in p.iter_mut().take(0x18).enumerate() {
        let x = curve(i as f32 / 23.0, a.start, a.end, c1, c2);
        let j = [rng.randf(-1.0, 1.0), rng.randf(-1.0, 1.0), rng.randf(-1.0, 1.0)];
        *q = add(x, set_len(j, 0.075));
    }
    let alpha = if a.fade < 10 { rng.rand_range(0, 0x40) } else { rng.rand_range(0x40, 0x80) } as u32;
    // `0x419640`: the 25th point one step on.
    p[24] = add(p[23], sub(p[23], p[22]));
    let colour = if big { alpha << 24 | (first_fade as u32).wrapping_mul(0xa_0000) | (0xff - 10 * first_fade) as u32 } else { 0x8000_00ff };
    let side = |i: usize| {
        let f = set_len(sub(p[i + 1], p[i]), 1.0);
        set_len(cross(f, sub(cam, p[i])), 1.0)
    };
    let fix = |c: u32, a: u32| (c & 0xff_ffff) | a.min(0xff) << 24;
    let (a1, a2) = (alpha, alpha / 3);
    let (mut s0, mut s1) = (0.0f32, 0.33f32);
    let d0 = side(0);
    let (mut prev1, mut prev2) = ([add(p[0], scale(d0, 0.04)), sub(p[0], scale(d0, 0.04))], [add(p[0], scale(d0, 0.16)), sub(p[0], scale(d0, 0.16))]);
    for i in 1..24 {
        let d = side(i);
        let cur1 = [add(p[i], scale(d, 0.04)), sub(p[i], scale(d, 0.04))];
        let cur2 = [add(p[i], scale(d, 0.16)), sub(p[i], scale(d, 0.16))];
        out.push(crate::fx::FxQuad {
            fx: 0x2b,
            additive: true,
            corners: [prev1[0], prev1[1], cur1[0], cur1[1]],
            st: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]],
            rgba: [fix(0x7fff_ffff, a1), fix(colour, a1), fix(0x7f7f_7f7f, a1), fix(0x7f7f_7f7f, a1)],
        });
        out.push(crate::fx::FxQuad {
            fx: 0x2b,
            additive: true,
            corners: [prev2[0], prev2[1], cur2[0], cur2[1]],
            st: [[s0, 0.0], [s0, 1.0], [s1, 0.0], [s1, 1.0]],
            rgba: [fix(colour, a2); 4],
        });
        s0 = i as f32 * 0.33;
        s1 = s0 + 0.33;
        if 1.0 <= s0 {
            s0 -= 1.0;
            s1 -= 1.0;
        }
        (prev1, prev2) = (cur1, cur2);
    }
}
