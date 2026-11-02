//! The Lava Gun (item 0x1d, crate::hero::items::ROWS; L00's record `0x263e60`: attachment 0, stance 2 (both hands),
//! icon 0xea69, name 0x32e8, damage 6 (`+0x60`), 4 splashed (`+0x64`), 200 ammo): the hand moby **0x05c9**
//! (`0x41b198`, called by the item pass while it is the hand item). No projectile moby: the stream's points, the burning
//! pools and the effects are the hand's own (its trail slot's memory, [`Mem`]), drawn by its callback (`0x41c328`).
//!
//! **The hand** (pvars: `+0x00` the muzzle (joint 0), `+0x10` the stream's start, `+0x34` its trail slot, `+0x38` its
//! loop sound, `+0x3c` the next point, `+0x40` the points wrapped, `+0x44` the points live, `+0x48` the newest, `+0x50`
//! the flow (0..10) and `+0x54` its speed, `+0x58` / `+0x5c` the stream's texture scroll, `+0x60` the ticks firing).
//! State **0** takes the class's trail slot (crate::moby::classes::chopper::Pool, `0x32b1b8`), clears the points and
//! the pools, → **1**. Put away (`0x18b064` = 3) its sound stops and nothing else runs. Each tick: the muzzle; its
//! sequence run out, its sequence 1 (blend 2). The look wait as the other weapons' (`0x18c354`: counted in the look
//! stance's twin, else 0x28). ○ held, the wait out, ammo left and the items not kept away: the flow springs toward 10
//! (`0x31ebd8(10, 0.0075, 0.002, 10)`), every tenth tick firing one ammo, its loop sound (class sound 0), its sequence 2,
//! and every moby but Ratchet within 0.5 of the muzzle (`0x2e14b0`, flags 0x10) is hit — the row's damage, flags
//! 0x910000, from the hand, its class sound 1. Else: the sound stops, ○ pressed with no ammo clicks (class sound 2),
//! the flow springs back to 0 (0.06, 0.01, 0.125), the firing ticks 0. In the look stance 1 Ratchet goes to its twin
//! 0x1d. With a flow, a point is shot from the muzzle ([`emit`], `0x41c0b8`) along the gun's −left row raised 0.02, at
//! flow · 0.05 a tick (Ratchet not standing: turned −11.25° about his left row); the texture scrolls 0.1 / 0.025 a tick.
//! With no flow the stream is cleared; else its points step ([`step_points`], `0x41bac0`) and its tail sheds
//! ([`tail`], `0x41ba00`). The pools age ([`step_pools`], `0x41cfe8`). Drawn (not in a scene), its draw is asked for.
//!
//! **The stream** (35 points of 0x40 bytes: place, velocity, the face's push, 40 ticks, stuck), newest first: each
//! moves; a free one tests its step (flags 0x10, the hand left out): none met, it falls 0.006 a tick; met, it sticks
//! there 0.1 out along the face, a spark leaves the face (crate::particles::lava, 0.01 a tick turned up to 45° each way
//! about each axis); a moby that is a target (mode 0x1000 or class type 5, not Ratchet) takes the row's damage and every
//! moby but Ratchet within 1.5 of it 4 (`+0x64`), with its class sound 1; else, one time in 11, a burning pool starts
//! there ([`start_pool`], `0x41cd40`). Every point: one time in 7 a drop (crate::particles::lava: 0.05..0.1 · flow ·
//! 0.1, 0.05 a tick along its way), one in 22 a spark (0.05 a tick). The tail (the oldest but two): one time in 7 a
//! still drop of flow · 0.025, one in 22 a spark at its velocity.
//!
//! **A pool** (ten of 0x530 bytes): two flames 0.1 about the point, each a trapezoid 0.3..0.5 wide at the foot, 0.4 of
//! that at the top, 0.9..1 · 2 high, its texture scrolling −0.01..−0.02 a tick from −8..0, flipped one time in two. Its
//! size springs to 0.5 (0.002, 0.0002, 0.02), holds 60 ticks, then springs to 0 (gone).
//!
//! **The draw** (`0x41c328`): the stream ([`draw_stream`], `0x41c6b8`): from the muzzle through the points, three
//! strips across the view — FX 10 `0x9f9f9f` blended at a fixed 0x40, FX 0xe `0x7f7f7f` added at 0x40 (both
//! `0.025..0.05` wide at the muzzle, ×0.94 each point on, a random width a point), FX 0x2b `0x103080` added at 0x50 six
//! times as wide; with a flow, two glows at the muzzle (`0x41c4d8`: FX 0x2c, 0.25 `0x7f10307f` and 0.5 `0x3f08183f`,
//! 0.25 toward the camera, a random roll); the pools ([`draw_pools`], `0x41d1c8`): at the first flame two glows lying
//! on the face (`0x41d660`: FX 0xb, 0.35 `0x103080` and 0.075 white), and each flame upright facing the camera (FX
//! 0x17, the first blended, the second added, `0x103080` fading up), at alpha 0x40 (shrinking: its size · 128).
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x1d, crate::hero::items): its word pulses `0xff00ffff` .. `0xff104080` and
//! it sheds a glow sprite at (0.06, −0.509, 0).
//!
//! Not ported: the gravity frames' trackers and bends (`0x37f538` / `0x37f618` / `0x37f5f8`, `0x380e08`), the lock
//! cheat (`0x18b270`, `0x32b4d0`), the weapon mods' effects on a hit (`0x3f9f70`), the GS test register the draw sets
//! (`0x37c948`), the 125 records past the pools no code reads (`$gp+0x23f8`). Its upgrade (rows 0x3f on) is the Meteor Gun,
//! crate::moby::classes::meteor.

use crate::hero::items::{row, ItemRow, LAVA};
use crate::hero::physics::{add, cross, dist2, scale, set_len, spring, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{countdown, Ctx, MobyWorld};

/// The hand's class (item 0x1d's row `+0x04`).
pub const HAND_CLASS: i16 = 0x5c9;
/// The stream's points and the pools.
pub const POINTS: usize = 0x23;
pub const POOLS: usize = 10;
/// The splash's damage (the row's `+0x64`; crate::hero::items::ItemRow keeps only `+0x60`).
const SPLASH: f32 = 4.0;
/// `t[3]` of the hits' template (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.925;
/// `0x1b13f8`: the drops' ALPHA, in turn.
const DROP_ALPHA: [u8; 2] = [0x48, 0x44];

/// One point of the stream (`+0x00` place, `+0x10` velocity, `+0x20` the face's push, `+0x30` ticks, `+0x34` stuck).
#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub pos: V3,
    pub vel: V3,
    pub push: V3,
    pub life: i32,
    pub stuck: bool,
}

/// One flame of a pool (`+0x20 + 0x10·k` its place, `+0x60 + 0x90·k` its quad's corners, `+0x4e0` / `+0x4f0` /
/// `+0x500` its flip, scroll speed and scroll).
#[derive(Clone, Copy, Debug, Default)]
pub struct Flame {
    pub pos: V3,
    pub corners: [V3; 4],
    pub flip: bool,
    pub speed: f32,
    pub scroll: f32,
}

/// A burning pool (`+0x00` the face's push, `+0x510` its size and `+0x51c` its speed, `+0x514` live, `+0x518` its
/// phase: 0 growing, 2 holding (`+0x520` ticks), 1 shrinking).
#[derive(Clone, Copy, Debug, Default)]
pub struct Pool {
    pub normal: V3,
    pub flames: [Flame; 2],
    pub size: f32,
    pub speed: f32,
    pub live: bool,
    pub phase: i32,
    pub hold: i32,
}

/// The gun's part of its trail slot's memory (`$gp+0x2400` the pools, `$gp+0x23fc` the points) and the drops' turns
/// (`$gp+0x2410` / `+0x2414`).
#[derive(Clone, Debug, Default)]
pub struct Mem {
    pub points: Vec<Point>,
    pub pools: Vec<Pool>,
    pub still_turn: i32,
    pub drop_turn: i32,
}

impl Mem {
    fn ready(&mut self) {
        if self.points.len() != POINTS { self.points = vec![Point::default(); POINTS]; }
        if self.pools.len() != POOLS { self.pools = vec![Pool::default(); POOLS]; }
    }
}

/// The Lava Gun's row as `0x139568[0x1d]` gives it (crate::upgrades).
fn cur_row(game: &crate::game::GameState) -> Option<&'static ItemRow> { row(game, LAVA) }

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { cross(b, a) }

/// The loop sound let go when still playing (`0x3602f0`).
fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = w.mobys[id].pv_i(0x38);
    if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
    w.mobys[id].set_pv_i(0x38, -1);
}

/// A hit on `target` from the hand (`0x31d158` + `0x31d378`): along the hand to it, flags 0x910000, its kind the
/// target's class `<< 16 | 0x0109`; then the hand's class sound 1.
fn hit(w: &mut MobyWorld, ctx: &mut Ctx, hand: usize, target: usize, damage: f32) {
    let d = unit(sub(w.mobys[target].pos, w.mobys[hand].pos));
    let mut t = HitTemplate::new(damage, Some(hand), 0x91_0000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
    t.kind = (w.mobys[target].o_class as u16 as u32) << 16 | 0x0109;
    w.deliver_hit_any(target, &t, [0.0; 4]);
    ctx.host.play_sound(&w.mobys[hand], 1, 0);
}

/// `0x41b198` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if w.mobys[id].state == 0 {
        let class = w.mobys[id].o_class;
        let mut pool = std::mem::take(&mut w.chopper);
        let (slot, _) = pool.slot(w, class);
        w.chopper = pool;
        w.lava.ready();
        for p in w.lava.points.iter_mut() { *p = Point::default(); }
        for p in w.lava.pools.iter_mut() { p.live = false; }
        let m = &mut w.mobys[id];
        m.set_pv_i(0x34, slot as i32);
        for o in [0x3c, 0x48, 0x40, 0x44, 0x54, 0x50, 0x58, 0x5c, 0x60, 0x64] { m.set_pv_i(o, 0); }
        m.state = 1;
        m.set_pv_i(0x38, -1);
    }
    w.lava.ready();
    if ctx.hero.items.ready == 3 {
        stop_sound(w, id, ctx);
        return;
    }
    let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    w.mobys[id].set_pv_v(0, muzzle);
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_a == a.seq_b && a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 2); }
    }
    let ok = if ctx.hero.state == 0x1d {
        countdown(&mut ctx.hero.items.look_wait)
    } else {
        ctx.hero.items.look_wait = 0x28;
        true
    };
    let it = ctx.hero.items;
    let firing = it.held & it.trigger != 0 && ok && it.has_ammo(ctx.game) && it.no_use == 0;
    if firing {
        fire(w, id, ctx, muzzle);
    } else {
        stop_sound(w, id, ctx);
        let it = ctx.hero.items;
        if it.pressed & it.trigger != 0 && !it.has_ammo(ctx.game) && it.no_use == 0 { ctx.host.play_sound(&w.mobys[id], 2, 0); }
        let m = &mut w.mobys[id];
        let (mut f, mut v) = (m.pv_f(0x50), m.pv_f(0x54));
        spring(0.0, f32::from_bits(0x3d75_c28f), f32::from_bits(0x3c23_d70a), 0.125, &mut f, &mut v);
        m.set_pv_f(0x50, f);
        m.set_pv_f(0x54, v);
        m.set_pv_i(0x60, 0);
    }
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    let muzzle = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let flow = w.mobys[id].pv_f(0x50);
    let mut dir = scale(w.mobys[id].rows[1], -1.0);
    dir[2] += 0.02;
    dir = set_len(dir, flow * 0.05);
    {
        let m = &mut w.mobys[id];
        m.set_pv_v(0, muzzle);
        m.set_pv_v(0x10, muzzle);
    }
    if ctx.hero.state != 0 {
        let left = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or(ctx.hero.rows[1], |h| h.rows[1]);
        dir = crate::gravity::rotate(dir, unit(left), f32::from_bits(0xbdc9_0fdb));
    }
    if flow != 0.0 { emit(w, id, muzzle, dir); }
    {
        let m = &mut w.mobys[id];
        for (o, step) in [(0x58usize, 0.1f32), (0x5c, 0.025)] {
            let s = m.pv_f(o) - step;
            m.set_pv_f(o, if s <= -8.0 { s + 8.0 } else { s });
        }
    }
    if flow == 0.0 {
        let m = &mut w.mobys[id];
        for o in [0x40, 0x44, 0x3c, 0x48] { m.set_pv_i(o, 0); }
    } else {
        step_points(w, id, ctx);
        tail(w, id, ctx);
    }
    step_pools(w);
    if ctx.game.scenes.mode != crate::scene::SCENE && w.mobys[id].drawn != 0 { w.register_draw(id); }
}

/// The firing part of `0x41b198` (module docs).
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, muzzle: V3) {
    let damage = cur_row(ctx.game).map_or(0.0, |r| r.damage);
    {
        let m = &mut w.mobys[id];
        let (mut f, mut v) = (m.pv_f(0x50), m.pv_f(0x54));
        spring(10.0, f32::from_bits(0x3bf5_c28f), f32::from_bits(0x3b03_126f), 10.0, &mut f, &mut v);
        m.set_pv_f(0x50, f);
        m.set_pv_f(0x54, v);
    }
    let n = w.mobys[id].pv_i(0x60);
    if n % 10 == 0 { ctx.hero.items.take_ammo(ctx.game, 1); }
    w.mobys[id].set_pv_i(0x60, n + 1);
    let s = w.mobys[id].pv_i(0x38);
    if s == -1 || !ctx.host.sound_playing(s) {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(0x38, h);
    }
    if ctx.host.anim(&w.mobys[id]).is_none_or(|a| a.seq_b != 2) { ctx.host.set_anim(&w.mobys[id], 2, 0, 0); }
    let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, muzzle, 0.5, crate::collision::QueryFlags(0x10), Some(id))).unwrap_or_default();
    for t in near {
        if Some(t) == ctx.hero_moby { continue; }
        hit(w, ctx, id, t, damage);
    }
}

/// `0x41c0b8(hand, at, vel, 40)`: the next point shot from `at`.
fn emit(w: &mut MobyWorld, id: usize, at: V3, vel: V3) {
    let head = w.mobys[id].pv_i(0x3c).clamp(0, POINTS as i32 - 1);
    w.lava.points[head as usize] = Point { pos: at, vel, push: [0.0; 3], life: 0x28, stuck: false };
    let m = &mut w.mobys[id];
    m.set_pv_i(0x48, head);
    let next = head + 1;
    if (next as usize) < POINTS {
        m.set_pv_i(0x3c, next);
        if m.pv_i(0x40) != 1 { m.set_pv_i(0x44, m.pv_i(0x44) + 1); }
    } else {
        m.set_pv_i(0x3c, 0);
        m.set_pv_i(0x40, 1);
    }
}

/// The index before `k` in the ring of points.
fn before(k: i32) -> i32 { if k - 1 < 0 { POINTS as i32 - 1 } else { k - 1 } }

/// `0x41bac0` (module docs).
fn step_points(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (count, mut k) = (w.mobys[id].pv_i(0x44), w.mobys[id].pv_i(0x48));
    let flow = w.mobys[id].pv_f(0x50);
    let damage = cur_row(ctx.game).map_or(0.0, |r| r.damage);
    for _ in 0..count.max(0) {
        let ki = k.clamp(0, POINTS as i32 - 1) as usize;
        let mut p = w.lava.points[ki];
        if !p.stuck {
            let next = add(p.pos, p.vel);
            match ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, p.pos, next, crate::collision::QueryFlags(0x10), Some(id))) {
                None => p.vel[2] -= f32::from_bits(0x3bc4_9ba6),
                Some(o) => {
                    let push = set_len(o.normal, 0.1);
                    p.pos = add(o.point, push);
                    p.vel = [0.0; 3];
                    p.stuck = true;
                    p.push = push;
                    let g = &mut ctx.game.rng;
                    let quarter = f32::from_bits(0x3f49_0fdb);
                    let e = [g.randf(-quarter, quarter), g.randf(-quarter, quarter), g.randf(-quarter, quarter)];
                    let v = crate::gravity::to_world(set_len(push, 0.01), &crate::hero::physics::euler_rows(e));
                    if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::lava::spark(sys, g, -1.0, p.pos, v, false); }
                    let target = o.moby.filter(|&m| Some(m) != ctx.hero_moby && w.mobys.get(m).is_some_and(|x| x.mode & crate::moby::mode::TARGET != 0 || w.class(x.o_class).is_some_and(|c| c.kind == 5)));
                    match target {
                        None => {
                            if ctx.game.rng.rand() % 11 == 0 { start_pool(w, ctx, o.point, push); }
                        }
                        Some(t) => {
                            hit(w, ctx, id, t, damage);
                            let at = w.mobys[t].pos;
                            let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, at, 1.5, crate::collision::QueryFlags(0x10), Some(id))).unwrap_or_default();
                            for n in near {
                                if Some(n) == ctx.hero_moby { continue; }
                                hit(w, ctx, id, n, SPLASH);
                            }
                        }
                    }
                }
            }
        }
        p.pos = add(p.pos, p.vel);
        w.lava.points[ki] = p;
        k = before(k);
        let g = &mut ctx.game.rng;
        if g.rand() % 7 == 0 {
            // `0x41c240`: a drop along its way.
            let v = set_len(p.vel, f32::from_bits(0x3d4c_cccd));
            let size = g.randf(f32::from_bits(0x3d4c_cccd), 0.1) * flow * 0.1;
            let a = DROP_ALPHA[(w.lava.drop_turn % 2) as usize];
            w.lava.drop_turn += 1;
            if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::lava::drop(sys, g, size, p.pos, v, a); }
        }
        if g.rand() % 0x16 == 0 {
            let v = set_len(p.vel, f32::from_bits(0x3d4c_cccd));
            if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::lava::spark(sys, g, -1.0, p.pos, v, false); }
        }
    }
}

/// `0x41ba00` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let mut k = m.pv_i(0x48) + 2 - m.pv_i(0x44);
    if k < 0 { k += POINTS as i32; }
    let p = w.lava.points[k.clamp(0, POINTS as i32 - 1) as usize];
    let flow = m.pv_f(0x50);
    let g = &mut ctx.game.rng;
    if g.rand() % 7 == 0 {
        // `0x41c1a0`: a still drop.
        let a = DROP_ALPHA[(w.lava.still_turn % 2) as usize];
        w.lava.still_turn += 1;
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::lava::drop(sys, g, flow * 0.025, p.pos, [0.0; 3], a); }
    }
    if g.rand() % 0x16 == 0 {
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::lava::spark(sys, g, -1.0, p.pos, p.vel, false); }
    }
}

/// `0x41c460`: a flame's corners before its shaping.
const FLAME: [V3; 4] = [[0.0, -1.0, 0.0], [0.0, -1.0, 2.0], [0.0, 1.0, 0.0], [0.0, 1.0, 2.0]];
/// `0x41c3f8`: the ±1 square.
const SQUARE: [V3; 4] = [[0.0, -1.0, -1.0], [0.0, -1.0, 1.0], [0.0, 1.0, -1.0], [0.0, 1.0, 1.0]];
const ST: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];

/// `0x41cd40(hand, at, push, angles)`: the first free pool started at `at` (module docs).
fn start_pool(w: &mut MobyWorld, ctx: &mut Ctx, at: V3, push: V3) {
    let Some(k) = w.lava.pools.iter().position(|p| !p.live) else { return };
    let g = &mut ctx.game.rng;
    let mut flames = [Flame::default(); 2];
    for f in flames.iter_mut() {
        let mut c = FLAME;
        let wide = g.randf(f32::from_bits(0x3e99_999a), 0.5);
        for q in c.iter_mut() { q[1] *= wide; }
        let tall = g.randf(f32::from_bits(0x3f66_6666), 1.0);
        c[1][2] *= tall;
        c[3][2] *= tall;
        c[1][1] *= 0.4;
        c[3][1] *= 0.4;
        let x = at[0] + g.randf(-0.1, f32::from_bits(0x3dcc_cccd));
        let y = at[1] + g.randf(-0.1, f32::from_bits(0x3dcc_cccd));
        let speed = g.randf(f32::from_bits(0xbca3_d70a), f32::from_bits(0xbc23_d70a));
        let scroll = g.randf(f32::from_bits(0xc0ff_ae14), 0.0);
        let flip = g.rand() % 2 != 0;
        *f = Flame { pos: [x, y, at[2]], corners: c, flip, speed, scroll };
    }
    w.lava.pools[k] = Pool { normal: push, flames, size: 0.0, speed: 0.0, live: true, phase: 0, hold: 0 };
}

/// `0x41cfe8` (module docs).
fn step_pools(w: &mut MobyWorld) {
    for p in w.lava.pools.iter_mut().filter(|p| p.live) {
        for f in p.flames.iter_mut() {
            let s = f.scroll + f.speed;
            f.scroll = if s <= -8.0 { s + 8.0 } else { s };
        }
        let (k, d, most) = (f32::from_bits(0x3b03_126f), f32::from_bits(0x3951_b717), f32::from_bits(0x3ca3_d70a));
        match p.phase {
            0 => {
                spring(0.5, k, d, most, &mut p.size, &mut p.speed);
                if p.size == 0.5 {
                    p.hold = 0x3c;
                    p.phase = 2;
                }
            }
            1 => {
                spring(0.0, k, d, most, &mut p.size, &mut p.speed);
                if p.size == 0.0 { p.live = false; }
            }
            _ => {
                if countdown(&mut p.hold) { p.phase = 1; }
            }
        }
    }
}

/// A quad from `local` corners in the frame `rows` at `at`.
fn quad(fx: usize, additive: bool, rows: [V3; 3], at: V3, local: [V3; 4], st: [[f32; 2]; 4], rgba: [u32; 4]) -> crate::fx::FxQuad {
    let corners = local.map(|l| add(at, crate::gravity::to_world(l, &rows)));
    crate::fx::FxQuad { fx, additive, corners, st, rgba }
}

/// `0x41c328` (module docs).
pub fn draw(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    if w.lava.points.len() != POINTS { return; }
    draw_stream(w, id, rng, cam, out);
    let m = &w.mobys[id];
    if m.pv_f(0x50) != 0.0 {
        let at = m.pv_v(0x10);
        for (size, rgba) in [(0.25, 0x7f10_307fu32), (0.5, 0x3f08_183f)] {
            // `0x41c4d8`: facing the camera, a random roll, 0.25 toward it.
            let d = sub(cam, at);
            let yaw = d[1].atan2(d[0]);
            let pitch = -d[2].atan2(dist2(at, cam));
            let roll = rng.rand_angle();
            let rows = crate::hero::physics::euler_rows([roll, pitch, yaw]);
            let p = add(at, set_len(d, 0.25));
            out.push(quad(0x2c, true, rows, p, SQUARE.map(|c| scale(c, size)), ST, [rgba; 4]));
        }
    }
    draw_pools(w, cam, out);
}

/// `0x41c6b8(0.05, hand)` (module docs).
fn draw_stream(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let count = m.pv_i(0x44);
    if count == 0 { return; }
    let (u, v) = (m.pv_f(0x58), m.pv_f(0x5c));
    let st_a = [[u, v], [u, v + 1.0], [u + 1.0, v], [u + 1.0, v + 1.0]];
    let st_b = [[u, 0.0], [u, 1.0], [u + 1.0, 0.0], [u + 1.0, 1.0]];
    let (ca, cb, cc) = (0x409f_9f9fu32, 0x407f_7f7fu32, 0x5010_3080u32);
    let pt = |k: i32| w.lava.points[k.clamp(0, POINTS as i32 - 1) as usize].pos;
    let across = |from: V3, to: V3| {
        let d = unit(sub(to, from));
        unit(cross_ba(sub(cam, from), d))
    };
    let mut wide = 0.05f32;
    let mut half = wide * 0.5;
    let mut d = rng.randf(half, wide);
    let mut k = m.pv_i(0x48);
    let start = m.pv_v(0);
    let a = across(start, pt(k));
    let mut prev = (add(start, scale(a, d)), sub(start, scale(a, d)), add(start, scale(a, d * 6.0)), sub(start, scale(a, d * 6.0)));
    for _ in 1..count {
        let cur = pt(k);
        let a = across(cur, pt(before(k)));
        let now = (add(cur, scale(a, d)), sub(cur, scale(a, d)), add(cur, scale(a, d * 6.0)), sub(cur, scale(a, d * 6.0)));
        out.push(crate::fx::FxQuad { fx: 10, additive: false, corners: [prev.0, prev.1, now.0, now.1], st: st_a, rgba: [ca; 4] });
        out.push(crate::fx::FxQuad { fx: 0xe, additive: true, corners: [prev.0, prev.1, now.0, now.1], st: st_b, rgba: [cb; 4] });
        out.push(crate::fx::FxQuad { fx: 0x2b, additive: true, corners: [prev.2, prev.3, now.2, now.3], st: st_b, rgba: [cc; 4] });
        prev = now;
        k = before(k);
        wide *= 0.94;
        half *= 0.94;
        d = rng.randf(half, wide);
    }
}

/// `0x41d1c8` (module docs).
fn draw_pools(w: &MobyWorld, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    for p in w.lava.pools.iter().filter(|p| p.live) {
        let alpha = if p.phase == 1 { (p.size * 64.0 * 2.0) as i32 as u32 } else { 0x40 };
        let col = alpha << 24;
        // `0x41d660`: two glows lying on the face at the first flame.
        let n = p.normal;
        let yaw = n[1].atan2(n[0]);
        let pitch = -n[2].atan2((n[0] * n[0] + n[1] * n[1]).sqrt());
        let rows = crate::hero::physics::euler_rows([0.0, pitch, yaw]);
        for (size, rgb) in [(f32::from_bits(0x3eb3_3333), 0x10_3080u32), (f32::from_bits(0x3d99_999a), 0xff_ffff)] {
            out.push(quad(0xb, true, rows, p.flames[0].pos, SQUARE.map(|c| scale(c, size)), ST, [col | rgb; 4]));
        }
        for (k, f) in p.flames.iter().enumerate() {
            let s = f.scroll;
            let st = if f.flip { [[s, 0.0], [s + 1.0, 0.0], [s, 1.0], [s + 1.0, 1.0]] } else { [[s, 1.0], [s + 1.0, 1.0], [s, 0.0], [s + 1.0, 0.0]] };
            let to_cam = unit(sub(cam, f.pos));
            let side = unit(cross_ba(to_cam, [0.0, 0.0, 1.0]));
            let up = cross_ba(side, to_cam);
            let size = if p.phase == 1 { 0.5 } else { p.size };
            let c = col | 0x10_3080;
            out.push(quad(0x17, k == 1, [to_cam, side, up], f.pos, f.corners.map(|q| scale(q, size)), st, [c, 0x10_3080, c, 0x10_3080]));
        }
    }
}
