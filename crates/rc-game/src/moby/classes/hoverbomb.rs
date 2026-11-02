//! The Hoverbomb Gun (item 0x19, crate::hero::items::ROWS; L00's record `0x263ae0`: attachment 0, stance 2 (both
//! hands), use sequence 0x89, no firing arm's sequences, icon 0xea72, name 0x32f7, the bomb's speed 7 a second (`+0x40`),
//! 3 a second (`+0x44`), its class 0x05d8 (`+0x50`), damage 30 (`+0x60`), 10 bombs; its upgrade row 0x46): the hand
//! moby **0x0a5b** (`0x3c4860`, called by the item pass while it is the hand item) and the bomb it launches **0x05d8**
//! (`0x3ab088`, made by `0x3aad18`), steered while ○ is held and set off by hand.
//!
//! **The hand** (pvars: `+0x00` its bomb, `+0x08` the cooldown (seconds), `+0x0c` (s16) the launch's wait, `+0x0e`
//! (s16) the use's ticks). State **0** (`0x3c4be8`): all cleared, **1**. Each tick: Ratchet's look stance 1 goes to its
//! twin 0x1d; its sequence run out, back to 1 (`0x31b760(hand, 1, 0, 10, 0)`); the use's ticks out end the use
//! (`0x2af848`); the muzzle at (0.2, −0.9, 0) in its frame (`$gp+0xeb0`); the cooldown runs down. In 0x1d the look
//! wait (`0x18c354`) counts down — out, the screen-centre crosshair (`0x2db498`, FX 0x1f: red `0x600000ff` while the
//! cooldown runs, with no bomb left or not ready, else green `0x600fff0f`) — else it is 0x28. **1**: in play, the
//! cooldown out, the items not kept away (`0x18c0dc`), its sequence A at most 1, ○ held, in the hand past 0x16 ticks and
//! the look wait out: with a bomb, one taken, the latch (`$gp+0x387c`), its sequence 2 from frame 4 (blend 2), its class
//! sound 0, the use stance (`0x2af5e0`), **2** with the launch's wait 2 and the use's ticks 16 (`$gp+0xec0` /
//! `$gp+0xec4`) — none left, ○ pressed or the latch set: its class sound 1 and the latch cleared; either way the
//! cooldown 1 / the row's rate. **2**: the wait out, the launch ([`launch`], `0x3c4c10`): along Ratchet's way
//! (`0x18a580`; the arm raised at a wall: the gun's −row 1) at the row's speed, the muzzle's smoke ([`smoke`],
//! `0x3c4d68`), the bomb ([`spawn`]), **3**. **3**: while its bomb lives or ○ is held nothing; then **1**.
//!
//! **The bomb** (pvars: `+0x00` its velocity, `+0x10` the hand, `+0x14` (s16) its age, `+0x16` (s16) its fuse,
//! `+0x18` its damage, `+0x1c` its speed, `+0x20` its hover height (0.5), `+0x24` / `+0x28` its bob's angle and
//! height, `+0x2c` (s16) its smoke's ticks, `+0x2e` (s16) ○ held, `+0x30` its spins, `+0x40` its loop sound, `+0x44`
//! the bounce sound's wait, `+0x50` the way it was launched). Made ([`spawn`], `0x3aad18`): drawn, updated and drawn at
//! any distance, turned three random ways, half its class's size, its spins 0.05..0.25°, 0.1..0.5° and 0.4..2° a tick
//! either way, 12 ticks of smoke. The line from Ratchet (at the muzzle's height) to the muzzle meeting a wall (not the
//! classes 0xc87 / 0xecc): it goes off there at once (`0x320538`: flashes 6 and 3, light 15, the shake; then the
//! blast's hit) and no bomb is made. Each tick ([`update_bomb`]): it moves; while its smoke runs, three grey puffs
//! (`0x335bb8`) along the step, smaller as they run out; in its first 7 ticks a line down 0.5 along the up holds it
//! that high off a floor, its sphere growing from 0 to 0.45; it bobs (0.25 along the up, its angle 0.025 a tick) and
//! spins; its sphere (`0x2df8d8`, Ratchet left out) pushes it out and turns it off the face (a moby: off the line from
//! its axis), at a quarter of its speed (at least 1/30 a tick; a floor or a ceiling keeps it all) — its class sound 2
//! at most every 15 ticks; its way eases 1% a tick back to its launch's; its loop sound (class sound 0) plays. Set off
//! — past 20 ticks old, by a new press of ○, by letting go after holding ○ over 20 ticks, by another item in the hand —
//! its class sound 1, the blast (crate::moby::blast, `0x3f8348(1.5, 20, …, 1, 0x28, 0x14, 0x1e, 0x28, 0x32, …)`, its
//! flash ball and the shake) and the hit ([`hit`], `0x3aabc8`: every moby within 6 but Ratchet, flags 1, the bomb's
//! damage, flags 0x810000 along its way, kind `0x05d8 << 16 | 0x0203`), and it is gone. While ○ is held it is
//! steered: Ratchet's landing lag 2 (`0x18a000`, he does not move; not in his state 0x31), the left stick turning its
//! way along the camera's left row (x / 240) and up row (y / 1200), its speed kept. Out of the world's 2..1021 box it
//! is gone; else its blob shadow 0.5 across.
//!
//! Its glow in the hand (`0x2a7ed8`'s case 0x19, crate::hero::items): its word pulses `0xff5f5f5f` .. `0xffdfdfdf` and
//! it sheds a glow sprite at (0.05, −0.45, 0).
//!
//! Not ported: the lock cheat (`0x18b270`, `0x32b4d0`), the gravity frames' trackers and bends (`0x37f538` /
//! `0x37f618` / `0x37f5f8`, `0x380e08`, the way flattened along the frame's up at the launch), the camera's flag the
//! steering sets (`0x428588`: the current camera of class 4 or 0x51 marked), the bounce sound's volume (`0x3605f0`'s
//! f12), the weapon mods' effects on a hit (`0x3f9f70`), the blast's screen flash (crate::moby::blast), the small
//! blast of a set fuse (`+0x16`: every bomb is made with none), the upgraded Hoverbomb Gun (row 0x46: its hand class
//! 0x0e2f, `0x3ff9c8`).

use crate::hero::items::{row, ItemRow, HOVERBOMB};
use crate::hero::physics::{add, add_rot, dot, len, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{countdown, Ctx, MobyWorld};

/// The hand's class (item 0x19's row `+0x04`) and the bomb's (`+0x50`).
pub const HAND_CLASS: i16 = 0xa5b;
pub const BOMB: i16 = 0x5d8;
/// `$gp+0xeb0`: the muzzle in the hand's frame.
const MUZZLE: V3 = [0.2, -0.9, 0.0];
/// The crosshair's FX texture.
const CROSSHAIR: u16 = 0x1f;
/// `t[3]` of the blast's hit (`0x45afdf66`).
const NO_KNOCK_LIMIT: f32 = 5627.925;

/// The Hoverbomb Gun's global: the empty latch (`$gp+0x387c`).
#[derive(Clone, Debug, Default)]
pub struct Mem {
    pub latch: bool,
}

/// The Hoverbomb Gun's row as `0x139568[0x19]` gives it (crate::upgrades).
fn cur_row(game: &crate::game::GameState) -> Option<&'static ItemRow> { row(game, HOVERBOMB) }

/// `0x2eeae0` on an s16 pvar: true when it was out or ran out.
fn count_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut n = w.mobys[id].pv_s(off) as i32;
    let out = countdown(&mut n);
    w.mobys[id].set_pv_s(off, n as i16);
    out
}

/// `0x2ef2a8(out, v, n)`: `v` reflected off the face of normal `n` (made unit) when it goes into it, else kept.
fn reflect(v: V3, n: V3) -> V3 {
    let n = unit(n);
    let d = dot(v, n);
    if d <= 0.0 { sub(v, scale(n, 2.0 * d)) } else { v }
}

/// `0x3c4860` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_i(0, 0);
        m.set_pv_f(8, 0.0);
        m.set_pv_s(0xe, 0);
        m.state = 1;
    }
    if ctx.hero.state == 1 { ctx.hero.request_state(0x1d, true); }
    if let Some(a) = ctx.host.anim(&w.mobys[id]) {
        if a.seq_a == a.seq_b && a.flags & 2 != 0 { ctx.host.set_anim(&w.mobys[id], 1, 0, 10); }
    }
    if w.mobys[id].pv_s(0xe) != 0 && count_s(w, id, 0xe) {
        let idle = ctx.hero.idle_seq();
        ctx.hero.items.use_end(ctx.hero.state, ctx.hero_view.seq_b, idle);
    }
    let muzzle = {
        let m = &w.mobys[id];
        add(crate::gravity::to_world(MUZZLE, &m.rows), m.pos)
    };
    {
        let c = w.mobys[id].pv_f(8);
        w.mobys[id].set_pv_f(8, if 1.0 / 60.0 < c { c - 1.0 / 60.0 } else { 0.0 });
    }
    let mut ok = true;
    if ctx.hero.state == 0x1d {
        if countdown(&mut ctx.hero.items.look_wait) {
            let red = 0.0 < w.mobys[id].pv_f(8) || !ctx.hero.items.has_ammo(ctx.game) || w.mobys[id].state != 1;
            crate::hud::marker(&mut ctx.game.markers, 1.0, id, if red { 0x6000_00ff } else { 0x600f_ff0f }, None, CROSSHAIR);
        } else {
            ok = false;
        }
    } else {
        ctx.hero.items.look_wait = 0x28;
    }
    let Some(r) = cur_row(ctx.game).copied() else { return };
    match w.mobys[id].state {
        1 => {
            let it = ctx.hero.items;
            let seq_a = ctx.host.anim(&w.mobys[id]).map_or(0, |a| a.seq_a);
            if ctx.game.scenes.mode == crate::scene::PLAY && w.mobys[id].pv_f(8) == 0.0 && it.no_use == 0 && seq_a < 2 && it.held & it.trigger != 0 && 0x16 < it.item_ticks && ok {
                if !it.has_ammo(ctx.game) {
                    if it.pressed & it.trigger != 0 || w.hoverbomb.latch {
                        ctx.host.play_sound(&w.mobys[id], 1, 0);
                        w.hoverbomb.latch = false;
                    }
                } else {
                    ctx.hero.items.take_ammo(ctx.game, 1);
                    w.hoverbomb.latch = true;
                    ctx.host.set_anim(&w.mobys[id], 2, 4, 2);
                    ctx.host.play_sound(&w.mobys[id], 0, 0);
                    ctx.hero.items.use_start(ctx.hero.group);
                    let m = &mut w.mobys[id];
                    m.state = 2;
                    m.set_pv_s(0xc, 2);
                    m.set_pv_s(0xe, 0x10);
                }
                w.mobys[id].set_pv_f(8, 1.0 / r.rate);
            }
        }
        2 => {
            if count_s(w, id, 0xc) {
                launch(w, id, ctx, muzzle, &r);
                w.mobys[id].state = 3;
            }
        }
        _ => {
            let bomb = w.mobys[id].pv_link(0);
            if bomb.and_then(|b| w.mobys.get(b)).is_some_and(|b| b.o_class == BOMB && b.state != 0xfe && b.state != 0xfd) { return; }
            let it = ctx.hero.items;
            if it.held & it.trigger != 0 { return; }
            let m = &mut w.mobys[id];
            m.set_pv_i(0, 0);
            m.state = 1;
        }
    }
}

/// `0x3c4c10(hand, muzzle)` (module docs).
fn launch(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, muzzle: V3, r: &ItemRow) {
    let f = r.speed * 0.016_666_668;
    let dir = if ctx.hero.items.raised { set_len(w.mobys[id].rows[1], -f) } else { set_len(ctx.hero.rows[0], f) };
    smoke(ctx, muzzle);
    let b = spawn(w, ctx, len(dir), r.damage, 0.5, muzzle, dir);
    w.mobys[id].set_pv_i(0, b.map_or(0, |b| b as i32));
    if let Some(b) = b { w.mobys[b].parent = Some(id); }
}

/// `0x3c4d68(hand, at)`: twenty grey puffs at the muzzle (`0x335bb8(0.3, 1, 1.01, 70000..150000, at, 0, a random way
/// 1/60000..1/150 a tick, grey 10..0x7f, 1)`: 120 ticks), two draws of `0x31b9a0(2)` before each left unused.
fn smoke(ctx: &mut Ctx, at: V3) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for _ in 0..20 {
        g.randi(2);
        g.randi(2);
        let size = g.randf(70_000.0, 150_000.0);
        let c = g.rand_range(10, 0x7f) as u32;
        let v = g.rand_vec(f32::from_bits(0x378b_cf66), f32::from_bits(0x3bda_740f));
        if let Some(i) = crate::particles::puff(sys, g, f32::from_bits(0x3e99_999a), 1.0, f32::from_bits(0x3f81_47ae), size, at, 0, v, 0x7f00_0000 | c << 16 | c << 8 | c, 1) {
            let r = &mut sys.pool.recs[i];
            crate::particles::rec::set_i16(r, 0xa, 0x78);
            crate::particles::rec::set_u32(r, 0x24, 2);
            r[0x2a] = 0x7f;
            r[0x2b] = r[0xa];
        }
    }
}

/// `0x3aad18(speed, damage, height, at, dir, hand, fuse 0)` (module docs): the bomb, or None (its class not loaded, or
/// a wall between Ratchet and the muzzle: it goes off there).
#[allow(clippy::too_many_arguments)]
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, speed: f32, damage: f32, height: f32, at: V3, dir: V3) -> Option<usize> {
    if w.class(BOMB).is_none() {
        eprintln!("rc_game::hoverbomb: class {BOMB:#x} is not loaded: no bomb");
        return None;
    }
    let class_scale = w.class(BOMB).map_or(1.0, |c| c.scale);
    let id = w.create(BOMB);
    let g = &mut ctx.game.rng;
    let rz = g.rand_angle();
    let ry = g.rand_angle();
    let rx = g.rand_angle();
    let vel = set_len(dir, speed);
    let spins = [
        g.randf_signed(f32::from_bits(0x3a64_c38a), f32::from_bits(0x3b8e_fa36)),
        g.randf_signed(f32::from_bits(0x3ae4_c38a), f32::from_bits(0x3c0e_fa36)),
        g.randf_signed(f32::from_bits(0x3be4_c38a), f32::from_bits(0x3d0e_fa36)),
    ];
    {
        let m = &mut w.mobys[id];
        m.state = 0;
        m.update_dist = 0xff;
        m.draw_dist = 0xff;
        m.drawn = 1;
        m.rot = [rx, ry, rz];
        m.scale = class_scale * 0.5;
        m.pos = at;
        m.set_pv_v(0, vel);
        m.set_pv_v(0x50, vel);
        m.set_pv_f(0x1c, speed);
        m.set_pv_s(0x16, 0);
        m.set_pv_f(0x18, damage);
        m.set_pv_f(0x20, height);
        m.set_pv_s(0x14, 0);
        m.set_pv_s(0x2e, 0);
        m.set_pv_f(0x24, 0.0);
        m.set_pv_f(0x28, 0.0);
        m.set_pv_s(0x2c, 0xc);
        for (k, s) in spins.iter().enumerate() { m.set_pv_f(0x30 + 4 * k, *s); }
        m.set_pv_i(0x40, -1);
        m.set_pv_i(0x44, 0);
    }
    // The line from Ratchet at the muzzle's height to the muzzle.
    let hero = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or(ctx.hero.pos, |h| h.pos);
    let met = w.coll_line(ctx, [hero[0], hero[1], at[2]], at, 0, ctx.hero_moby);
    let wall = met.filter(|o| o.moby.and_then(|m| w.mobys.get(m)).is_none_or(|m| m.o_class != 0xc87 && m.o_class != 0xecc));
    let Some(o) = wall else { return Some(id) };
    w.mobys[id].pos = o.point;
    let e = crate::moby::blast::Explosion { flags: 0x81_0001, flash: 6.0, flash2: 3.0, near: 9.0, light: 15.0, embers: 10, billows: 3, flares: 9, sound: -1, shake: true, debris: 1, lod: 0, ..Default::default() };
    crate::moby::blast::explode(w, ctx, Some(id), None, &e);
    hit(w, ctx, id, o.point);
    w.delete(id);
    None
}

/// `0x3aabc8(bomb, pvars, at, _)`: every moby within 6 of `at` but Ratchet (flags 1) takes the bomb's damage (flags
/// 0x810000, along its way, kind `0x05d8 << 16 | 0x0203`).
fn hit(w: &mut MobyWorld, ctx: &Ctx, id: usize, at: V3) {
    let m = &w.mobys[id];
    let d = set_len(m.pv_v(0), 1.0);
    let mut t = HitTemplate::new(m.pv_f(0x18), Some(id), 0x81_0000, [d[0], d[1], d[2], NO_KNOCK_LIMIT]);
    t.kind = (BOMB as u16 as u32) << 16 | 0x0203;
    w.sphere_hit(ctx.scene, at, 6.0, 1, ctx.hero_moby, Some(&t));
}

/// The loop sound let go when still playing (`0x3602f0`).
fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = w.mobys[id].pv_i(0x40);
    if s != -1 && ctx.host.sound_playing(s) { ctx.host.stop_sound(s); }
    w.mobys[id].set_pv_i(0x40, -1);
}

/// `0x3ab088` (module docs).
pub fn update_bomb(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let up = ctx.hero.gravity.up_at(w.mobys[id].pos);
    let age = w.mobys[id].pv_s(0x14) + 1;
    w.mobys[id].set_pv_s(0x14, age);
    let start = w.mobys[id].pos;
    let mut pos = add(start, w.mobys[id].pv_v(0));
    w.mobys[id].pos = pos;
    count_s(w, id, 0x2c);
    let smoke = w.mobys[id].pv_s(0x2c);
    if smoke != 0 {
        if let Some(sys) = ctx.parts.as_deref_mut() {
            let g = &mut ctx.game.rng;
            for i in 0..3 {
                let t = i as f32 * f32::from_bits(0x3eaa_a64c);
                let p: V3 = std::array::from_fn(|k| start[k] + (pos[k] - start[k]) * t);
                let n = g.randi(6);
                let spin = if g.randi(2) != 0 { -n } else { n };
                let size = (smoke as f32 / 50.0) * 70_000.0 + 10_000.0;
                if let Some(k) = crate::particles::puff(sys, g, f32::from_bits(0x3ccc_cccd), 1.0, 1.0, size, p, spin, [0.0; 3], 0x8080_8080, 1) {
                    let r = &mut sys.pool.recs[k];
                    crate::particles::rec::set_i16(r, 0xa, 0x32);
                    crate::particles::rec::set_u32(r, 0x24, 2);
                    r[0x2a] = 0x7f;
                    r[0x2b] = r[0xa];
                }
            }
        }
    }
    let mut radius = 0.45f32;
    if age < 7 {
        radius = (age as f32 / 7.0) * 0.45;
        let a = scale(up, w.mobys[id].pv_f(0x20));
        if let Some(o) = w.coll_line(ctx, pos, sub(pos, a), 0x12, Some(id)) {
            pos = add(o.point, a);
            w.mobys[id].pos = pos;
        }
    }
    {
        let m = &mut w.mobys[id];
        let a = add_rot(m.pv_f(0x24), 0.025);
        m.set_pv_f(0x24, a);
        let mut p = sub(m.pos, scale(up, m.pv_f(0x28)));
        let b = a.sin() * 0.25;
        m.set_pv_f(0x28, b);
        p = add(p, scale(up, b));
        m.pos = p;
        for k in 0..3 { m.rot[k] = add_rot(m.rot[k], m.pv_f(0x30 + 4 * k)); }
        let mut t = m.pv_i(0x44);
        countdown(&mut t);
        m.set_pv_i(0x44, t);
    }
    let pos = w.mobys[id].pos;
    if let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, radius, crate::collision::QueryFlags(0), ctx.hero_moby)) {
        let p = o.pushed_centre.unwrap_or(pos);
        w.mobys[id].pos = p;
        let vel = w.mobys[id].pv_v(0);
        let speed = len(vel);
        let mut f = (speed * 0.25).max(f32::from_bits(0x3d08_8889));
        let vel = match o.moby.filter(|&m| w.mobys.get(m).is_some()) {
            None => {
                let n = unit(o.normal);
                if 0.8 <= dot(n, up).abs() { f = speed; }
                reflect(vel, n)
            }
            Some(hm) => {
                let c = w.mobys[hm].pos;
                let axis = add(c, scale(up, dot(up, sub(p, c))));
                reflect(vel, sub(p, axis))
            }
        };
        if w.mobys[id].pv_i(0x44) == 0 {
            ctx.host.play_sound(&w.mobys[id], 2, 0);
            w.mobys[id].set_pv_i(0x44, 0xf);
        }
        w.mobys[id].set_pv_v(0, set_len(vel, f));
    }
    {
        let m = &mut w.mobys[id];
        let (v, want) = (m.pv_v(0), m.pv_v(0x50));
        m.set_pv_v(0, std::array::from_fn(|k| v[k] + (want[k] - v[k]) * 0.01));
    }
    let s = w.mobys[id].pv_i(0x40);
    if s == -1 || !ctx.host.sound_playing(s) {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(0x40, h);
    }
    // Set off.
    let it = ctx.hero.items;
    let (pressed, held) = (it.pressed & it.trigger != 0, it.held & it.trigger != 0);
    let check = pressed || (!held && 0x14 < w.mobys[id].pv_s(0x2e)) || !matches!(it.item, 0x19 | 0x46) || w.mobys[id].state == 1;
    if check && 0x14 < w.mobys[id].pv_s(0x14) {
        w.mobys[id].state = 1;
        if count_s(w, id, 0x16) {
            stop_sound(w, id, ctx);
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            let at = w.mobys[id].pos;
            let b = crate::moby::blast::Blast { size: 1.5, bursts: [0x28, 0x14, 0x1e], puffs: 0x28, streaks: 0x32, light: true, shake: true };
            crate::moby::blast::burst(w, ctx, at, &b);
            hit(w, ctx, id, at);
            w.delete(id);
            return;
        }
    }
    // Steered.
    if held {
        let n = w.mobys[id].pv_s(0x2e);
        w.mobys[id].set_pv_s(0x2e, n.wrapping_add(1));
        if ctx.hero.state != 0x31 { ctx.hero.land_lag = 2; }
        let [sx, sy] = ctx.hero.pad_stick;
        if sx != 0.0 || sy != 0.0 {
            let v = w.mobys[id].pv_v(0);
            let speed = len(v);
            if speed != 0.0 {
                let rows = ctx.host.camera_rows();
                let v = add(v, scale(rows[1], -sx * f32::from_bits(0x3b88_8889)));
                let v = add(v, scale(rows[2], -sy * f32::from_bits(0x3a5a_740f)));
                w.mobys[id].set_pv_v(0, set_len(v, speed));
            }
        }
    }
    let p = w.mobys[id].pos;
    if p.iter().all(|&x| (2.0..=1021.0).contains(&x)) {
        if !ctx.hero.gravity.on { crate::shadows::request(w, ctx, 0.5, id); }
    } else {
        stop_sound(w, id, ctx);
        w.delete(id);
    }
}
