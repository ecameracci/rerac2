//! The Mini-Nuke's blast effect (its blast 0x0c06, crate::moby::classes::gravity_bomb::update_mini_blast): the smoke
//! records of its cloud and its draw — the Gravity Bomb's effect (crate::moby::classes::gravity_bomb::Flare) in its own
//! group and pool, with a stem and a cap.
//!
//! **The group** (`0x1b2ad0`, one of 0x50 bytes) and the pool (`0x1b2ad4`, 320 records of 0x80; [`Mem`]): made at the
//! blast's start ([`start`], `0x3e7978(1/6, blast, 20, place, normal)`): its normal, its centre the place, its axis
//! `(place + 2·normal) × place`, its countdown 20, its rise `+0x3c` 1/6, its counts 0; then 48 records of kind 2 on a
//! ring ([`spawn`], `0x3e7bf8`: from a random angle, 7.66° apart). The pool's records and the cap's 144 slots
//! (`0x1b2ad8`) are cleared first.
//!
//! **A record** ([`spawn`], `0x3e7bf8(angle, group, kind, at)`): the first free of 320 (kinds 0 and 1 at most 150 each,
//! `+0x40` / `+0x44`), the group's count up; its alpha 0x5f..0x7f, its colour `0x103080` easing to a grey (`n ·
//! 0x10101`), its roll's speed ±0.314..0.356 easing to its own, a random roll, blend 2 (additive), phase 0. By kind:
//! * **0** (the column, two on an odd random draw each tick of the group's countdown, [`tick`]): out a random way about
//!   the normal, rising `+0x3c · randf(0, 1.25)` a tick easing to 0, grey 0..12, 0.25..0.35 across growing to
//!   0.75..1, 100..120 ticks, at the centre 0.15..0.75 out.
//! * **1** (the cap's, [`draw`]'s): a random way, no rise, grey 0..12, 0.25..0.5 growing to 1..1.5, out
//!   0.0667..0.0833 a tick, 100..120 ticks, at the cap's quad.
//! * **2** (the ring): along the axis turned by the angle about the normal, no rise, grey 1..9, `randf(0.15, 0.25) ·
//!   0.5` growing to `randf(0.5, 0.8) · 1.15`, out 0.0583 a tick, 90 ticks, 0.2 along the normal and 0.75 out from the
//!   centre; its roll's speed to ±0.0524..0.1047.
//!
//! **Each tick** ([`tick`], `0x3e7aa8` / `0x3e8090`): the group's countdown runs (out with no records: free); each of
//! its records as the Gravity Bomb's flares (growing then fading, its alpha `α₀ − α₀·t²`, out `out·(1 − (1 − t)²)`
//! along its way but kind 1, rising along the blast's up), and while the blast is in its state 2 under 38 ticks
//! left, a growing record of kind 0 or 1 fades at once (its values kept, 30..45 ticks).
//!
//! **The draw** ([`draw`], `0x3e6ad0`: in view, 100 and a sphere of 2): outside its state 6, the dome twice
//! (`0x3e6b90` / `0x3e6df8`: the Gravity Bomb's mesh, FX 0x2b, `0x204080` at the blast's `+0x24`) and the white
//! flash over the screen at `+0x24` (`0x3e6a30`); always the stem (`0x3e7058`: 215 quads of FX 0x32, `0x808080` at
//! `+0x1c` but on its bottom ring, scaled (1, 1, 0.865 · size), widened `cos z + 2`, its T down z/8 and both scrolls,
//! its faces turned away left out), the cap (`0x3e74b0`: 143 quads, (size/16 + 1) across, lifted `ease(⌊r⌋ · 2/3) ·
//! size · 0.012375` (r = 1.5 − |v|·2/3) and 0.15·size, its T scrolled; each quad facing the camera keeps a record of
//! kind 1 at its centre, made anew when it went, while the blast is not in state 6) and the records (`0x3e83c8`).
//!
//! Not ported: the trail slot it takes (`0x32b1b8`: the port keeps no such slots for it), the GS test register of the
//! flash (`0x37c948(0x4e, ..)`), the record's word `+0x20` (0.000556, read by nothing here).

use crate::hero::physics::{add, add_rot, cross, dot, len, scale, set_len, sub, unit, V3};
use crate::moby::classes::gravity_bomb::{flare_quad, Flare};
use crate::moby::{countdown, Ctx, MobyWorld};

/// The pool's records, the kinds' most, the cap's slots.
const RECORDS: usize = 320;
const KIND_MOST: i32 = 0x96;
const CAP_SLOTS: usize = 0x90;
/// `$gp+0x15b4..`: the dome's FX and colour, `+0x15d4..` the stem's, `+0x15f4..` the cap's.
const DOME_FX: usize = 0x2b;
const DOME_RGB: u32 = 0x20_4080;
const CLOUD_FX: usize = 0x32;
const CLOUD_RGB: u32 = 0x80_8080;
/// The stem's bottom ring: its vertices at this height draw clear.
const STEM_FOOT: f32 = -0.001_052;

/// One mesh of the cloud: points, STs and quads of `{point, ST}` pairs.
#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub verts: Vec<V3>,
    pub st: Vec<[f32; 2]>,
    pub quads: Vec<[(u16, u16); 4]>,
}

impl Mesh {
    fn read(read: &impl Fn(u32, usize) -> Option<Vec<u8>>, quads_at: u32, n: usize, verts_at: u32, st_at: u32) -> Option<Mesh> {
        let q = read(quads_at, n * 16)?;
        let half = |b: &[u8], i: usize| u16::from_le_bytes([b[2 * i], b[2 * i + 1]]);
        let quads: Vec<[(u16, u16); 4]> = (0..n).map(|i| std::array::from_fn(|k| (half(&q, i * 8 + 2 * k), half(&q, i * 8 + 2 * k + 1)))).collect();
        let nv = quads.iter().flatten().map(|&(v, _)| v as usize + 1).max()?;
        let ns = quads.iter().flatten().map(|&(_, s)| s as usize + 1).max()?;
        let f = |b: &[u8], i: usize| f32::from_le_bytes(b[4 * i..4 * i + 4].try_into().unwrap());
        let v = read(verts_at, nv * 16)?;
        let s = read(st_at, ns * 8)?;
        Some(Mesh { verts: (0..nv).map(|i| [f(&v, 4 * i), f(&v, 4 * i + 1), f(&v, 4 * i + 2)]).collect(), st: (0..ns).map(|i| [f(&s, 2 * i), f(&s, 2 * i + 1)]).collect(), quads })
    }
}

/// The stem (`0x298370`: 215 quads, points `0x297690`, STs `0x2990f0`) and the cap (`0x29a050`: 143 quads, `0x2997f0`,
/// `0x29a950`) at L00's addresses, read from the level's copy (crate's engine maps them).
#[derive(Clone, Debug, PartialEq)]
pub struct CloudMesh {
    pub stem: Mesh,
    pub cap: Mesh,
}

impl CloudMesh {
    pub fn read(read: impl Fn(u32, usize) -> Option<Vec<u8>>) -> Option<CloudMesh> {
        Some(CloudMesh { stem: Mesh::read(&read, 0x0029_8370, 0xd7, 0x0029_7690, 0x0029_90f0)?, cap: Mesh::read(&read, 0x0029_a050, 0x8f, 0x0029_97f0, 0x0029_a950)? })
    }
}

/// The group (`0x1b2ad0`).
#[derive(Clone, Copy, Debug, Default)]
pub struct Group {
    pub blast: usize,
    left: i32,
    pub count: i32,
    kinds: [i32; 2],
    rise: f32,
    normal: V3,
    axis: V3,
    centre: V3,
}

/// The effect's globals: the group, the pool, the cap's slots, the scrolls (`$gp+0x1594` / `+0x1598`), the meshes (set
/// by the engine).
#[derive(Clone, Debug)]
pub struct Mem {
    pub group: Option<Group>,
    pub recs: Vec<Option<Flare>>,
    pub caps: [Option<usize>; CAP_SLOTS],
    pub scroll: [f32; 2],
    pub mesh: Option<std::sync::Arc<CloudMesh>>,
}

impl Default for Mem {
    fn default() -> Self { Mem { group: None, recs: vec![None; RECORDS], caps: [None; CAP_SLOTS], scroll: [0.0; 2], mesh: None } }
}

/// `0x321c00(angle, out, v, axis)`.
fn rotate(v: V3, angle: f32, axis: V3) -> V3 { crate::moby::classes::gravity_bomb::rotate(v, angle, axis) }

/// `0x3e7978` (module docs).
pub fn start(w: &mut MobyWorld, ctx: &mut Ctx, id: usize) {
    let (centre, normal) = (w.mobys[id].pos, w.mobys[id].pv_v(0));
    let e = &mut w.nuke;
    e.recs.iter_mut().for_each(|r| *r = None);
    e.caps = [None; CAP_SLOTS];
    let p2 = add(set_len(normal, 2.0), centre);
    e.group = Some(Group { blast: id, left: 0x14, count: 0, kinds: [0; 2], rise: f32::from_bits(0x3e2a_aaab), normal, axis: cross(p2, centre), centre });
    let mut a = ctx.game.rng.rand_angle();
    for _ in 0..48 {
        spawn(w, ctx, a, 2, [0.0; 3]);
        a = add_rot(a, f32::from_bits(0x3e08_e4ab));
    }
}

/// `0x3e7bf8(angle, group, kind, at)` (module docs): the record's index.
fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, angle: f32, kind: u8, at: V3) -> Option<usize> {
    let e = &mut w.nuke;
    let slot = e.recs.iter().position(|r| r.is_none())?;
    let g = e.group.as_mut()?;
    if kind < 2 {
        if KIND_MOST <= g.kinds[kind as usize] + 1 { return None; }
        g.kinds[kind as usize] += 1;
    }
    g.count += 1;
    let r = &mut ctx.game.rng;
    let alpha = r.rand_range(0x5f, 0x7f);
    let mut f = Flare { alpha0: alpha, alpha, kind, blend: 2, ..Default::default() };
    match kind {
        0 => {
            f.dir = rotate(g.axis, r.rand_angle(), g.normal);
            let k = r.randf(0.0, 1.25);
            let rise = g.rise * k;
            f.rise = [rise, rise, 0.0];
            f.colour[2] = r.rand_range(0, 0xd) as u32 * 0x1_0101;
            let s = r.randf(0.25, f32::from_bits(0x3eb3_3333));
            f.size[0] = s;
            f.size[1] = s;
            f.size[2] = r.randf(0.75, 1.0);
            f.out = 0.0;
            let t = r.rand_range(100, 0x78);
            (f.left, f.ticks) = (t, t);
            let d = r.randf(0.15, 0.75);
            f.pos = add(g.centre, set_len(f.dir, d));
            f.spin[2] = 0.0;
        }
        1 => {
            f.dir = rotate(g.axis, r.rand_angle(), g.normal);
            f.colour[2] = r.rand_range(0, 0xd) as u32 * 0x1_0101;
            let s = r.randf(0.25, 0.5);
            f.size[0] = s;
            f.size[1] = s;
            f.size[2] = r.randf(1.0, 1.5);
            f.out = r.randf(f32::from_bits(0x3d88_8889), f32::from_bits(0x3daa_aaab));
            let t = r.rand_range(100, 0x78);
            (f.left, f.ticks) = (t, t);
            f.pos = at;
            f.spin[2] = 0.0;
        }
        _ => {
            f.dir = rotate(g.axis, angle, g.normal);
            f.colour[2] = r.rand_range(1, 10) as u32 * 0x1_0101;
            let s = r.randf(0.15, 0.25) * 0.5;
            f.size[0] = s;
            f.size[1] = s;
            f.size[2] = r.randf(0.5, 0.8) * 1.15;
            f.out = f32::from_bits(0x3d6e_eef0);
            (f.left, f.ticks) = (0x5a, 0x5a);
            f.pos = add(add(g.centre, set_len(g.normal, 0.2)), set_len(f.dir, 0.75));
            f.spin[2] = r.randf_signed(f32::from_bits(0x3d56_7752), f32::from_bits(0x3dd6_7752));
        }
    }
    f.colour[0] = 0x10_3080;
    f.colour[1] = 0x10_3080;
    let s = r.randf_signed(f32::from_bits(0x3ea0_d97d), f32::from_bits(0x3eb6_71f3));
    f.spin[0] = s;
    f.spin[1] = s;
    f.roll = r.rand_angle();
    f.phase = 0;
    w.nuke.recs[slot] = Some(f);
    Some(slot)
}

/// `0x3e6070`'s tail: the scrolls, `0x3e7aa8` and `0x3e8090` (module docs). Whether the blast's effect is over (its
/// group's records all gone: state 6 deletes it then).
pub fn tick(w: &mut MobyWorld, ctx: &mut Ctx, id: usize) -> bool {
    let e = &mut w.nuke;
    e.scroll[1] -= 0.04;
    if e.scroll[0] < -8.0 { e.scroll[0] += 8.0; }
    if e.scroll[1] < -8.0 { e.scroll[1] += 8.0; }
    let Some(g) = e.group.as_mut().filter(|g| g.blast == id) else { return true };
    let (out, empty) = (countdown(&mut g.left), g.count == 0);
    if !out {
        if ctx.game.rng.rand() & 1 == 0 {
            spawn(w, ctx, 0.0, 0, [0.0; 3]);
            spawn(w, ctx, 0.0, 0, [0.0; 3]);
        }
    } else if empty {
        w.nuke.group = None;
        return true;
    }
    records(w, ctx, id);
    w.nuke.group.is_none_or(|g| g.count == 0)
}

/// `0x3e8090` (module docs).
fn records(w: &mut MobyWorld, ctx: &mut Ctx, id: usize) {
    let up = ctx.hero.gravity.up_at(w.mobys[id].pos);
    let (state, left) = (w.mobys[id].state, w.mobys[id].pv_i(0x14));
    let e = &mut w.nuke;
    let Some(g) = e.group.as_mut() else { return };
    for slot in e.recs.iter_mut() {
        let Some(f) = slot.as_mut() else { continue };
        let t = 1.0 - f.left as f32 / f.ticks.max(1) as f32;
        let lerps = |f: &mut Flare| {
            f.colour[1] = crate::hud::lerp_rgba(t, f.colour[0], f.colour[2]);
            f.spin[1] = f.spin[0] + (f.spin[2] - f.spin[0]) * t;
            f.rise[1] = f.rise[0] + (f.rise[2] - f.rise[0]) * t;
            f.size[1] = f.size[0] + (f.size[2] - f.size[0]) * t;
        };
        let push = match f.phase {
            0 => {
                lerps(f);
                set_len(f.dir, f.out * (1.0 - (1.0 - t) * (1.0 - t)))
            }
            1 => {
                f.alpha = (f.alpha0 as f32 - f.alpha0 as f32 * t * t) as i32;
                if f.kind != 2 {
                    lerps(f);
                    set_len(f.dir, f.out * (1.0 - (1.0 - t) * (1.0 - t)))
                } else {
                    set_len(f.dir, f.out)
                }
            }
            _ => [0.0; 3],
        };
        f.pos = add(f.pos, scale(up, f.rise[1]));
        if f.kind != 1 { f.pos = add(f.pos, push); }
        f.roll = add_rot(f.roll, f.spin[1]);
        if state == 2 && left < 0x26 && f.kind != 2 {
            f.phase = 1;
            let n = ctx.game.rng.rand_range(0x1e, 0x2d);
            (f.left, f.ticks) = (n, n);
            f.size[0] = f.size[1];
            f.colour[0] = f.colour[1];
            f.rise[0] = f.rise[1];
            f.alpha0 = f.alpha;
            f.spin[0] = f.spin[1];
        }
        if countdown(&mut f.left) {
            if f.phase == 0 {
                f.phase = 1;
                f.left = if f.kind == 1 { 0x1e } else { 0x37 };
                f.ticks = f.left;
            } else if f.alpha < 1 {
                g.count -= 1;
                if f.kind < 2 { g.kinds[f.kind as usize] -= 1; }
                *slot = None;
            } else {
                f.phase = 2;
                f.alpha -= 1;
            }
        }
    }
}

/// `0x3e6ad0` (module docs).
pub fn draw(w: &mut MobyWorld, id: usize, rng: &mut crate::rng::Rng, view: &crate::particles::BSphereView, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if view.culled(100.0, [m.pos[0], m.pos[1], m.pos[2], 2.0]) { return; }
    let (pos, rows, state) = (m.pos, m.rows, m.state);
    let (size, flash, alpha) = (m.pv_f(0x10), m.pv_i(0x24).clamp(0, 0xff) as u32, m.pv_i(0x1c).clamp(0, 0xff) as u32);
    let through = |v: V3| add(pos, add(add(scale(rows[0], v[0]), scale(rows[1], v[1])), scale(rows[2], v[2])));
    let up = rows[2];
    if state != 6 {
        if let Some(mesh) = &w.gravity_bomb.mesh {
            let rgba = DOME_RGB | flash << 24;
            let lift = scale(up, 0.5);
            for k in [[size * 2.0, size * 2.0, size * 0.25], [size, size, size * 0.125]] {
                for q in &mesh.quads {
                    let corner = |i: usize| {
                        let v = mesh.verts.get(q[i].0 as usize).copied().unwrap_or_default();
                        add(through([v[0] * k[0], v[1] * k[1], v[2] * k[2]]), lift)
                    };
                    let st = |i: usize| mesh.st.get(q[i].1 as usize).copied().unwrap_or_default();
                    out.push(crate::fx::FxQuad { fx: DOME_FX, additive: true, corners: std::array::from_fn(corner), st: std::array::from_fn(st), rgba: [rgba; 4] });
                }
            }
        }
        if flash != 0 { w.screen_fills.push(flash << 24 | 0xff_ffff); }
    }
    let Some(mesh) = w.nuke.mesh.clone() else { return };
    let scroll = w.nuke.scroll;
    let facing = |c: &[V3; 4], centre: V3| {
        let view_dir = unit(sub(centre, cam));
        let n = unit(cross(sub(c[2], c[1]), sub(c[2], c[0])));
        0.0 < dot(view_dir, n)
    };
    // `0x3e7058`: the stem.
    for q in &mesh.stem.quads {
        let mut c = [[0.0f32; 3]; 4];
        let mut st = [[0.0f32; 2]; 4];
        let mut rgba = [0u32; 4];
        let mut centre = [0.0; 3];
        for i in 0..4 {
            let v = mesh.stem.verts.get(q[i].0 as usize).copied().unwrap_or_default();
            let s = mesh.stem.st.get(q[i].1 as usize).copied().unwrap_or_default();
            st[i] = [s[0] + scroll[0], s[1] + scroll[1]];
            rgba[i] = if v[2] == STEM_FOOT { CLOUD_RGB } else { CLOUD_RGB | alpha << 24 };
            let mut v = [v[0], v[1], v[2] * size * 0.865];
            st[i][1] -= v[2] * 0.125;
            let k = v[2].cos() + 2.0;
            v[0] *= k;
            v[1] *= k;
            c[i] = through(v);
            centre = add(centre, c[i]);
        }
        if facing(&c, scale(centre, 0.25)) {
            out.push(crate::fx::FxQuad { fx: CLOUD_FX, additive: true, corners: c, st, rgba });
        }
    }
    // `0x3e74b0`: the cap.
    let across = size * 0.0625 + 1.0;
    for (k, q) in mesh.cap.quads.iter().enumerate() {
        let mut c = [[0.0f32; 3]; 4];
        let mut st = [[0.0f32; 2]; 4];
        let mut centre = [0.0; 3];
        for i in 0..4 {
            let v = mesh.cap.verts.get(q[i].0 as usize).copied().unwrap_or_default();
            let s = mesh.cap.st.get(q[i].1 as usize).copied().unwrap_or_default();
            st[i] = [s[0], s[1] - scroll[1]];
            let mut v = [v[0] * across, v[1] * across, v[2]];
            let r = 1.5 - len(v) * f32::from_bits(0x3f2a_aaab);
            let e = ((r as i32) * (0.0 < r) as i32) as f32 * f32::from_bits(0x3f2a_aaab);
            let e = 1.0 - (1.0 - e) * (1.0 - e);
            v[2] += e * size * 1.5 * 0.008_25;
            v[2] += size * 0.15;
            c[i] = through(v);
            centre = add(centre, c[i]);
        }
        let centre = scale(centre, 0.25);
        if !facing(&c, centre) { continue; }
        if let Some(slot) = w.nuke.caps.get(k).copied() {
            let live = slot.is_some_and(|s| w.nuke.recs.get(s).is_some_and(|r| r.is_some()));
            let rec = if live {
                slot
            } else if state != 6 {
                cap_record(w, rng, centre)
            } else {
                None
            };
            if let Some(c) = w.nuke.caps.get_mut(k) { *c = rec; }
            if let Some(f) = rec.and_then(|s| w.nuke.recs[s].as_mut()) { f.pos = centre; }
        }
        out.push(crate::fx::FxQuad { fx: CLOUD_FX, additive: true, corners: c, st, rgba: [CLOUD_RGB | alpha << 24; 4] });
    }
    // `0x3e83c8`: the records.
    if w.nuke.group.is_some_and(|g| g.blast == id) {
        for f in w.nuke.recs.iter().flatten() { out.push(flare_quad(f, cam)); }
    }
}

/// The cap's record of kind 1 at `at` (`0x3e7bf8(0, 0, group, 1, at)` from the draw).
fn cap_record(w: &mut MobyWorld, rng: &mut crate::rng::Rng, at: V3) -> Option<usize> {
    let e = &mut w.nuke;
    let slot = e.recs.iter().position(|r| r.is_none())?;
    let g = e.group.as_mut()?;
    if KIND_MOST <= g.kinds[1] + 1 { return None; }
    g.kinds[1] += 1;
    g.count += 1;
    let alpha = rng.rand_range(0x5f, 0x7f);
    let mut f = Flare { alpha0: alpha, alpha, kind: 1, blend: 2, ..Default::default() };
    f.dir = rotate(g.axis, rng.rand_angle(), g.normal);
    f.colour[2] = rng.rand_range(0, 0xd) as u32 * 0x1_0101;
    let s = rng.randf(0.25, 0.5);
    f.size[0] = s;
    f.size[1] = s;
    f.size[2] = rng.randf(1.0, 1.5);
    f.out = rng.randf(f32::from_bits(0x3d88_8889), f32::from_bits(0x3daa_aaab));
    let t = rng.rand_range(100, 0x78);
    (f.left, f.ticks) = (t, t);
    f.pos = at;
    f.colour[0] = 0x10_3080;
    f.colour[1] = 0x10_3080;
    let sp = rng.randf_signed(f32::from_bits(0x3ea0_d97d), f32::from_bits(0x3eb6_71f3));
    f.spin[0] = sp;
    f.spin[1] = sp;
    f.roll = rng.rand_angle();
    e.recs[slot] = Some(f);
    Some(slot)
}

/// `0x3e67b0`: the group let go (the blast deleted).
pub fn free(w: &mut MobyWorld) { w.nuke.group = None; }
