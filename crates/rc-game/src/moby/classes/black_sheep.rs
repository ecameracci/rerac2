//! The black sheep, class **0x0eb0** (`0x409328`; made by the Black Sheepinator's transform `0x409cf0`,
//! [`transform`]): a sheep that hunts. It idles, grazes and wanders as the sheep does (crate::moby::classes::sheep), but
//! now and then picks the nearest enemy and charges it; blocked, or hit, it bursts and blasts what is about it.
//!
//! **Made** ([`transform`], `0x409cf0(target)`): updated and drawn to 0x7f, drawn, playing at 0.85..1.15 (`+0x48`), at
//! the target's place facing away from Ratchet (in the gravity frames the target's rows), state 4; pvars `+0x56` 0,
//! `+0x54` 120..300, `+0x74` 15, `+0x70` 600 (its state's ticks), `+0x5c` 0, `+0x78` 0 (its ticks), `+0x68` 1.0 (4.0
//! on the item's row past its base), `+0x60` −1; its collision on (`+0x98`). The target as the sheep's transform
//! leaves it: the bolts of a target (mode 0x1000, `0x31adc0(m, 0, 3)`), deleted unless it dies its own way (`+0xe` 1
//! or 2: marked 2). (Listed as a decoy, `0x3216e0(m, 0x1b1fb0, 0x1f)`: not ported, as no enemy reads the list.)
//!
//! **Each tick** ([`update`], `0x409328`): a hit (`0x31dce8(m, 0x4010000, 0)`) bursts it ([`blast`]); its ticks up;
//! its blob shadow (0.5). By state:
//! * **0**: to 1.
//! * **1** (idle): its sequence run out, a new one by `randf(0, 100)`: under 25 sequence 0, under 50 4, under 75 1
//!   (blend 10), else (outside the gravity frames) the nearest enemy ([`prey`], `0x325680`) — found, **8**, sequence 5
//!   (blend 1), playing 1.7; Ratchet within 2 (across) — **2**, sequence 5, playing 1.7; its ticks out (outside the
//!   frames) — at random **6** for 180..360 (sequence 3), else **5** for 400..600 (sequence 2; blend 10).
//! * **2**: the nearest enemy, when any, turned toward (`0x31c078`: 0.000218, 0.000436, 1° a tick at most) and stepped
//!   at 1/24 along its facing (`0x31c278(0.5, 0.6, 0.25, 30°, …)`); blocked, it bursts; its facing more than 30° off
//!   the way from Ratchet, it turns that way (0.000388, at most 0.75°).
//! * **4** (just made): the line 1 below to 4 above along its up puts it where it meets anything but Ratchet; on a
//!   face of a type above 0 and not 10 (outside the frames) it bursts; else **1**.
//! * **5** (wandering): its sequence run out, sequence 2 (blend 2); Ratchet within 2 — **2**, sequence 5 (blend 1);
//!   its ticks out — **1** for 400..600; it turns ±30° (0.000145) and steps 1/240 (`0x31c278(0.1, 0.5, …)`); blocked,
//!   it turns 20..135° off (0.0000485).
//! * **6** (grazing): its sequence run out, sequence 3 (blend 10); its ticks out — **1** for 120..600, sequence 0 or 1.
//! * **8** (charging): turned toward its prey (`+0x7c`) and stepped as in 2; blocked, it bursts.
//!
//! Then, outside the gravity frames, it stands on the ground under it (`0x31ce20(0.5)` from 0.5 up).
//!
//! **The burst** ([`blast`], `0x409ef0`): its class sound 3; 30 puffs (crate::particles::smoke, as the sheep's pop)
//! and 49 tufts of wool (crate::particles::wool, within ±0.15, flung `$gp+0x1f20` 8 a second toward the camera's side
//! within 90° and `$gp+0x1f24` 9 up, 320 ticks); a blast (`0x3f8348(1, 20, .., 1, 0x19, 0x14, 0x14, 0x14, 0x2d, 1)`);
//! every moby within 3 (`0x2e14b0`) of class type 5 or line mode (0x4000) takes 10 (55 on a Mega row at level 3) away
//! from it (flags 0x810000, kind `class << 16 | 0x0305`); deleted.
//!
//! Not ported: the level's moving frame (`0x1adb0c`, `0x322058`: none on these levels), the decoy list.

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, rot_dist, scale, set_len, sub, V3};
use crate::moby::{ai, countdown, Ctx, MobyWorld};

/// The class.
pub const CLASS: i16 = 0xeb0;
/// `0x31dce8`'s mask: the hits that burst it.
const HIT_MASK: u32 = 0x401_0000;
const DEG: f32 = 0.017_453_294;
/// The item's records with a Mega upgrade (`−0x10` ≠ 0): its base row and its upgrades.
const MEGA_ROWS: [usize; 4] = [0x10, 0x48, 0x6d, 0x6e];

/// The item's row's level and whether it has a Mega upgrade (`0x139568[0x10]`'s record).
fn row_level(game: &crate::game::GameState) -> (u8, bool) {
    let r = game.item_row(crate::moby::classes::sheepinator::ITEM);
    (crate::upgrades::level_of(r), MEGA_ROWS.contains(&r))
}

/// `0x409cf0(target)` (module docs).
pub fn transform(w: &mut MobyWorld, ctx: &mut Ctx, target: usize) -> Option<usize> {
    w.class(CLASS)?;
    let id = w.create(CLASS);
    let speed = ctx.game.rng.randf(f32::from_bits(0x3f59_999a), f32::from_bits(0x3f93_3333));
    let (pos, rows) = (w.mobys[target].pos, w.mobys[target].rows);
    let frames = ctx.hero.gravity.on;
    let pick = ctx.game.rng.rand_range(0x78, 300) as i16;
    let up = row_level(ctx.game).0 != 0;
    let away = atan2(pos[0] - ctx.hero.pos[0], pos[1] - ctx.hero.pos[1]);
    {
        let m = &mut w.mobys[id];
        m.pvars.resize(crate::moby::PVAR_BYTES, 0);
        m.update = Some(update);
        m.draw_dist = 0x7f;
        m.update_dist = 0x7f;
        m.drawn = 1;
        m.pos = pos;
        m.rot = if frames { crate::moby::attach::angles_of(&rows) } else { [0.0, 0.0, away] };
        m.set_pv_s(0x56, 0);
        m.set_pv_s(0x54, pick);
        m.set_pv_i(0x74, 0xf);
        m.set_pv_i(0x70, 600);
        m.set_pv_i(0x5c, 0);
        m.set_pv_i(0x78, 0);
        m.state = 4;
        m.set_pv_f(0x68, if up { 4.0 } else { 1.0 });
        m.set_pv_i(0x60, -1);
    }
    ctx.host.set_anim_speed(&w.mobys[id], speed);
    let c = w.class(CLASS).is_some_and(|c| c.has_collision);
    w.mobys[id].has_collision = c;
    let block = (w.mobys[target].mode & ai::AI != 0).then(|| ai::health(&w.mobys[target]));
    if w.mobys[target].mode & crate::moby::mode::TARGET != 0 { crate::moby::bolts::drop(w, target, 0, 3, ctx); }
    match block.map(|h| (h, w.mobys[target].pv_b(h + 0xe))) {
        Some((h, 1 | 2)) => w.mobys[target].set_pv_b(h + 0xe, 2),
        _ => w.delete(target),
    }
    w.changed = true;
    w.rebuild(id, &*ctx.host);
    Some(id)
}

/// `0x325680(π/2, 999, −1, −1, place)`: over the target list, the moby with a health block and health left whose score
/// (`0x325508`: its distance d, plus d · its yaw from the place off π/2, plus 8 for a crate) is the least, within 999.
fn prey(w: &MobyWorld, ctx: &Ctx, from: V3) -> Option<usize> {
    let mut best = 1e8f32;
    let mut found = None;
    for t in w.targets(ctx.host.camera_pos()) {
        let m = &w.mobys[t.id];
        if m.mode & ai::AI == 0 || m.pv_f(ai::health(m)) == 0.0 { continue; }
        let d = dist(from, m.pos);
        if 999.0 < d { continue; }
        let off = rot_dist(atan2(m.pos[0] - from[0], m.pos[1] - from[1]), std::f32::consts::FRAC_PI_2);
        let mut score = d + off * d;
        if (500..=540).contains(&m.o_class) { score += 8.0; }
        if score < best {
            best = score;
            found = Some(t.id);
        }
    }
    found
}

/// The charge's turn toward `yaw` and its step at 1/24 along its facing (states 2 and 8). Whether it moved. (With no
/// prey the original steps toward a point it never set, left on its stack; the port steps along its facing.)
fn charge(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, yaw: Option<f32>) -> bool {
    let m = &mut w.mobys[id];
    if let Some(y) = yaw {
        ai::turn_to(m, y, f32::from_bits(0x3964_c38a), f32::from_bits(0x39e4_c38a), f32::from_bits(0x3c8e_fa37), 0x58);
    }
    let mut target = add(m.pos, set_len(m.rows[0], f32::from_bits(0x3d2a_aaab)));
    let mut pos = m.pos;
    let ok = ai::step_walk(ctx, id, 0.5, f32::from_bits(0x3f19_999a), 0.25, f32::from_bits(0x3f06_0a93), &mut pos, &mut target, 0);
    w.mobys[id].pos = pos;
    w.changed = true;
    ok
}

/// `0x409328` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    if let Some(h) = w.hit_message(id) {
        if h.tmpl.flags & HIT_MASK == 0 {
            w.clear_hit(id);
        } else {
            blast(w, ctx, id);
            return;
        }
    }
    let t = w.mobys[id].pv_i(0x78);
    w.mobys[id].set_pv_i(0x78, t + 1);
    let hero = ctx.hero.pos;
    let pos = w.mobys[id].pos;
    let away = atan2(pos[0] - hero[0], pos[1] - hero[1]);
    crate::shadows::request(w, ctx, 0.5, id);
    let frames = ctx.hero.gravity.on;
    let seq_done = |w: &MobyWorld, ctx: &Ctx| ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.flags & 2 != 0);
    let count = |w: &mut MobyWorld| {
        let mut t = w.mobys[id].pv_i(0x70);
        let out = countdown(&mut t);
        w.mobys[id].set_pv_i(0x70, t);
        out
    };
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if seq_done(w, ctx) {
                let r = ctx.game.rng.randf(0.0, 100.0);
                if r < 25.0 {
                    ctx.host.set_anim(&w.mobys[id], 0, 0, 10);
                } else if r < 50.0 {
                    ctx.host.set_anim(&w.mobys[id], 4, 0, 10);
                } else if r < 75.0 {
                    ctx.host.set_anim(&w.mobys[id], 1, 0, 10);
                } else if r < 100.0 && !frames {
                    let p = prey(w, ctx, w.mobys[id].pos);
                    w.mobys[id].set_pv_i(0x7c, p.map_or(-1, |p| p as i32));
                    if p.is_some() {
                        w.mobys[id].set_state(8, -1);
                        ctx.host.set_anim(&w.mobys[id], 5, 0, 1);
                        ctx.host.set_anim_speed(&w.mobys[id], f32::from_bits(0x3fd9_999a));
                    }
                }
            }
            if dist2(w.mobys[id].pos, hero) < 2.0 && !frames {
                w.mobys[id].set_state(2, -1);
                ctx.host.set_anim(&w.mobys[id], 5, 0, 1);
                ctx.host.set_anim_speed(&w.mobys[id], f32::from_bits(0x3fd9_999a));
            }
            if count(w) && !frames {
                if ctx.game.rng.randi(2) == 0 {
                    w.mobys[id].set_state(6, -1);
                    let n = ctx.game.rng.rand_range(0xb4, 0x168);
                    w.mobys[id].set_pv_i(0x70, n);
                    ctx.host.set_anim(&w.mobys[id], 3, 0, 10);
                } else {
                    w.mobys[id].set_state(5, -1);
                    let n = ctx.game.rng.rand_range(400, 600);
                    w.mobys[id].set_pv_i(0x70, n);
                    ctx.host.set_anim(&w.mobys[id], 2, 0, 10);
                }
            }
        }
        2 => {
            let p = prey(w, ctx, w.mobys[id].pos);
            w.mobys[id].set_pv_i(0x7c, p.map_or(-1, |p| p as i32));
            let yaw = p.map(|p| { let (a, b) = (w.mobys[p].pos, w.mobys[id].pos); atan2(a[0] - b[0], a[1] - b[1]) });
            if !charge(w, id, ctx, yaw) {
                blast(w, ctx, id);
                return;
            }
            if 30.0 * DEG < rot_dist(away, w.mobys[id].rot[2]) {
                let m = &mut w.mobys[id];
                ai::turn_to(m, away, f32::from_bits(0x39cb_587b), f32::from_bits(0x39e4_c38a), f32::from_bits(0x3c56_7752), 0x58);
            }
        }
        4 => {
            let up = ctx.hero.gravity.up_at(w.mobys[id].pos);
            let pos = w.mobys[id].pos;
            let (top, bottom) = (add(pos, scale(up, 4.0)), add(pos, scale(up, -1.0)));
            let hit = w.coll_line(ctx, bottom, top, 0, Some(id));
            if let Some(h) = hit.filter(|h| h.moby != ctx.hero_moby || h.moby.is_none()) { w.mobys[id].pos = h.point; }
            let kind = hit.map_or(-1, |h| if 0 <= h.kind && h.kind & 0x1f != 0x1f { h.kind & 0x1f } else { -1 });
            if 0 < kind && kind != 10 && !frames {
                blast(w, ctx, id);
                return;
            }
            w.mobys[id].set_state(1, -1);
        }
        5 => {
            if seq_done(w, ctx) { ctx.host.set_anim(&w.mobys[id], 2, 0, 2); }
            if dist2(w.mobys[id].pos, hero) < 2.0 {
                w.mobys[id].set_state(2, -1);
                ctx.host.set_anim(&w.mobys[id], 5, 0, 1);
            }
            if count(w) {
                w.mobys[id].set_state(1, -1);
                let n = ctx.game.rng.rand_range(400, 600);
                w.mobys[id].set_pv_i(0x70, n);
            }
            let r = ctx.game.rng.randf(-30.0, 30.0);
            let m = &mut w.mobys[id];
            let yaw = add_rot(m.rot[2], r * DEG);
            ai::turn_to(m, yaw, f32::from_bits(0x3918_825c), f32::from_bits(0x39e4_c38a), f32::from_bits(0x3c56_7752), 0x58);
            let mut pos = m.pos;
            let mut target = add(pos, set_len(m.rows[0], f32::from_bits(0x3b88_8889)));
            let ok = ai::step_walk(ctx, id, 0.1, 0.5, 0.25, f32::from_bits(0x3f06_0a93), &mut pos, &mut target, 0);
            w.mobys[id].pos = pos;
            w.changed = true;
            if !ok {
                let r = ctx.game.rng.randf(20.0, 135.0);
                let m = &mut w.mobys[id];
                let yaw = add_rot(m.rot[2], r * DEG);
                ai::turn_to(m, yaw, f32::from_bits(0x384b_587b), f32::from_bits(0x39e4_c38a), f32::from_bits(0x3c56_7752), 0x58);
            }
        }
        6 => {
            if seq_done(w, ctx) { ctx.host.set_anim(&w.mobys[id], 3, 0, 10); }
            if count(w) {
                w.mobys[id].set_state(1, -1);
                let n = ctx.game.rng.rand_range(0x78, 600);
                w.mobys[id].set_pv_i(0x70, n);
                let s = ctx.game.rng.randi(2) as u8;
                ctx.host.set_anim(&w.mobys[id], s, 0, 10);
            }
        }
        8 => {
            let prey = usize::try_from(w.mobys[id].pv_i(0x7c)).ok().filter(|&p| w.mobys.get(p).is_some_and(|m| m.state < 0x80));
            let yaw = prey.map(|p| { let (a, b) = (w.mobys[p].pos, w.mobys[id].pos); atan2(a[0] - b[0], a[1] - b[1]) });
            if !charge(w, id, ctx, yaw) {
                blast(w, ctx, id);
                return;
            }
        }
        _ => {}
    }
    if !frames && w.mobys[id].state < 0x80 {
        let p = w.mobys[id].pos;
        let z = ai::ground_z(ctx, 0.5, [p[0], p[1], p[2] + 0.5], 0);
        w.mobys[id].pos[2] = z;
        w.changed = true;
    }
}

/// `0x325100(r, yaw, pitch)`.
fn polar(r: f32, yaw: f32, pitch: f32) -> V3 { [yaw.cos() * r * pitch.cos(), yaw.sin() * r * pitch.cos(), pitch.sin() * r] }

/// `0x409ef0` (module docs).
pub fn blast(w: &mut MobyWorld, ctx: &mut Ctx, id: usize) {
    ctx.host.play_sound(&w.mobys[id], 3, 0);
    let pos = w.mobys[id].pos;
    let cam = ctx.host.camera_pos();
    if let Some(sys) = ctx.parts.as_deref_mut() {
        let g = &mut ctx.game.rng;
        for _ in 0..30 {
            let (r, a, b) = (g.randf(0.0, f32::from_bits(0x3ebd_70a4)), g.rand_angle(), g.rand_angle());
            let mut p = polar(r, a, b);
            p[2] += g.randf(f32::from_bits(0x3eb3_3333), f32::from_bits(0x3f19_999a));
            let p = add(p, pos);
            let (s, a, b) = (g.randf(f32::from_bits(0x3ada_740f), f32::from_bits(0x3c88_8889)), g.rand_angle(), g.rand_angle());
            let v = polar(s, a, b);
            let life = g.rand_range(0x1e, 0x39);
            crate::particles::smoke::smoke(sys, g, 150_000.0, p, v, 0x4f20_507f, 0x102f, life);
        }
        for _ in 0..49 {
            let yaw = atan2(cam[0] - pos[0], cam[1] - pos[1]);
            // `0x2effb8(randf_signed(1, 2))`: drawn and dropped.
            let _ = g.randf_signed(1.0, 2.0);
            let p = [pos[0] + g.randf_signed(0.0, 0.15), pos[1] + g.randf_signed(0.0, 0.15), pos[2] + g.randf_signed(0.0, 0.15)];
            let a = add_rot(yaw, g.randf_signed(0.0, f32::from_bits(0xbfc9_0fdc)));
            let v = [a.cos() * 0.016_666_668 * 8.0, a.sin() * 0.016_666_668 * 8.0, 9.0 * 0.016_666_668];
            crate::particles::wool::wool(sys, g, p, v, 0x140);
        }
    }
    let b = crate::moby::blast::Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true };
    crate::moby::blast::burst(w, ctx, pos, &b);
    let (level, mega) = row_level(ctx.game);
    let damage = if mega && level == 3 { 55.0 } else { 10.0 };
    if let Some(sc) = ctx.scene {
        for m in crate::collision::coll_sphere_mobys(sc, pos, 3.0, crate::collision::QueryFlags(0), Some(id)) {
            let Some(o) = w.mobys.get(m) else { continue };
            let kind = w.class(o.o_class).map_or(0, |c| c.kind);
            if kind != 5 && o.mode & crate::moby::mode::LINE == 0 { continue; }
            let d = set_len(sub(o.pos, pos), 1.0);
            let mut t = crate::moby::hits::HitTemplate::new(damage, Some(id), 0x81_0000, [d[0], d[1], d[2], 5627.925]);
            t.kind = (o.o_class as u16 as u32) << 16 | 0x0305;
            w.deliver_hit_any(m, &t, [0.0, 0.0, 0.0, 1.0]);
        }
    }
    w.delete(id);
}
