//! The casters' shadows, the game side (G-REN-001): the shadow directions, Ratchet's shadow and the ground slab the
//! class updates fill; the volumes themselves are [`volume`], drawn by the engine (crate `rc-engine`
//! `shadow_render`). Not the blob shadows (crate::shadows): a class whose header byte 0x0f is set carries a proxy of
//! spheres and capsules (`rc_formats::moby_shadow`), and the moby init (`0x311418`) gives its mobys mode 0x400, the
//! range `+0x6f` 0x18, an empty slab (`+0x70` / `+0x74` 0) and direction 0 (`+0xbd`). R&C1's system
//! (`rerac:rc_game::shadows`) with Going Commando's slab: a plane across the moby's up row instead of a height band.
//!
//! | Game function (L00) | Here | Called from |
//! |---|---|---|
//! | `0x312b90` | [`update_dir`] | the play frame `0x3706f0`, the ship mode `0x364260`, the scenes `0x3716f8` |
//! | `0x313360` | [`key_light`] | the two direction updates |
//! | `0x2a97d8` | [`update_hero`] | the hero update `0x2a6dd0` |
//! | `0x31da58` | [`probe_down`] / [`probe`] | the casters' class updates (L01 `0x313af8`) |
//!
//! **The slab** (`+0x70` its offset, `+0x74` its half thickness): the shadow lies across the moby's up row (`+0xe0`)
//! at `up·pos + off`, `±t` about it (crate `rc-engine` `shadow_render` takes them with the posed proxy); `t ≤ 0` = no
//! shadow. Every probe is the line test with flags 0x22 (no moby primitives, not the water).
//!
//! The gravity frames (`0x1a8ff0`, crate::gravity) switch the probes to their up-row branch. Ratchet's slab seeds
//! from the ground under him, else the liquid's or the quicksand's surface the ground probe last found (`0x18a150` /
//! `0x18a154`; Oozla's swamp is quicksand).

pub mod volume;

use crate::hero::physics::{add, add_rot, atan2, dot, scale, sub, sub_rot, V3};
use crate::moby::Moby;

/// The direction index every moby but Ratchet uses (`+0xbd` = 0) and Ratchet's (1); −1 (0xff): the moby's up row
/// reversed (the gravity frames').
pub const DIR_MOBYS: usize = 0;
pub const DIR_HERO: usize = 1;
/// The line test's flags of every probe.
pub const PROBE_FLAGS: u32 = 0x22;
/// The mode bit of a live caster.
pub const MODE_CASTER: u16 = 0x400;
/// `+0x6f` at the init: the range in units (full size within 16, none past 24).
pub const RANGE: u8 = 0x18;

/// The shadow state the game keeps: the direction table `0x1b1e50` (4 × vec4, `+0xbd` picks; 2 and 3 are never
/// written), Ratchet's pitch `0x18acd0` and the level's key lights (each light set's first direction, `0x1c6ad0 +
/// 0x40·set`, for [`key_light`]).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShadowState {
    pub dirs: [[f32; 4]; 4],
    pub pitch: f32,
    pub key_lights: Vec<V3>,
}

impl ShadowState {
    /// A moby's direction (`+0xbd`; −1: its up row reversed; past the table: entry 0).
    pub fn dir_of(&self, m: &Moby) -> V3 {
        if m.shadow_dir < 0 { return scale(m.rows[2], -1.0); }
        let d = self.dirs.get(m.shadow_dir as usize).copied().unwrap_or(self.dirs[0]);
        [d[0], d[1], d[2]]
    }
}

/// `0x313360(moby, out)`: the moby's key light, light set A's first direction (the light word `+0x38`: byte 0 set
/// A, byte 1 set B, byte 2 their cross-fade f), blended `A·(1 − f/256) + B·f/256` when f ≠ 0; a set past the table
/// reads (0, 0, 0).
pub fn key_light(sets: &[V3], light: u32) -> V3 {
    let get = |k: u32| sets.get(k as usize).copied().unwrap_or([0.0; 3]);
    let (a, b, f) = (light & 0xff, (light >> 8) & 0xff, (light >> 16) & 0xff);
    if f == 0 { return get(a); }
    let t = f as f32 / 256.0;
    let (da, db) = (get(a), get(b));
    std::array::from_fn(|k| da[k] * (1.0 - t) + db[k] * t)
}

/// `0x312b90`: direction 0 = `(0.14·cos φ, 0.14·sin φ, −0.99)`, φ the heading of Ratchet's key light: almost
/// straight down, leaning about 8° along the light.
pub fn update_dir(s: &mut ShadowState, hero_light: u32) {
    let l = key_light(&s.key_lights, hero_light);
    let phi = atan2(l[0], l[1]);
    let d = &mut s.dirs[DIR_MOBYS];
    d[0] = phi.cos() * 0.14;
    d[1] = phi.sin() * 0.14;
    d[2] = -0.99;
}

/// The slab no shadow has.
fn clear(m: &mut Moby) { m.shadow_slab = [0.0; 2]; }

/// `0x31da58(moby)`: the slab under a caster. Outside the gravity frames, only an upright moby (its up row's z
/// `+0xe8` at least 0.9): a line from 0.5 above it to 16 below (at least down to 0.5); a hit puts the slab at the
/// hit's height (`+0x70` = hit z − its z), its half thickness a quarter of its sphere's radius (`+0x74` =
/// `+0xc`/4096). In the gravity frames (`gravity`): a line from its sphere's centre 8 down its up row; a hit puts the
/// slab there along the row, 0.2 thick, and the direction is its up row reversed (`+0xbd` = 0xff). No hit: none.
pub fn probe_down(m: &mut Moby, gravity: bool, line: impl FnOnce(V3, V3) -> Option<V3>) {
    if !gravity {
        if m.rows[2][2] < 0.9 { return clear(m); }
        let p = m.pos;
        let end = (p[2] - 16.0).max(0.5);
        match line([p[0], p[1], p[2] + 0.5], [p[0], p[1], end]) {
            Some(h) => m.shadow_slab = [h[2] - p[2], m.bsphere[3] * (1.0 / 4096.0)],
            None => clear(m),
        }
    } else {
        let c = scale([m.bsphere[0], m.bsphere[1], m.bsphere[2]], 1.0 / 1024.0);
        match line(c, add(c, scale(m.rows[2], -8.0))) {
            Some(h) => {
                m.shadow_slab = [dot(sub(h, m.pos), m.rows[2]), 0.2];
                m.shadow_dir = -1;
            }
            None => clear(m),
        }
    }
}

/// [`probe_down`] on moby `id` in a class update (the line test against the world and the mobys, flags
/// [`PROBE_FLAGS`]; the gravity frames' branch while they are on, crate::gravity).
pub fn probe(w: &mut crate::moby::MobyWorld, id: usize, ctx: &crate::moby::Ctx) {
    let line = |a: V3, b: V3| ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, crate::collision::QueryFlags(PROBE_FLAGS), None)).map(|o| o.point);
    probe_down(&mut w.mobys[id], ctx.hero.gravity.on, line);
}

/// The class updates' call: drawn (`+0x31`) and nearer the camera than `reach` (`0x2ef018`), [`probe`], and the
/// class's range (`+0x6f`) set. The slab is left as it was otherwise.
pub fn probe_near(w: &mut crate::moby::MobyWorld, id: usize, ctx: &crate::moby::Ctx, reach: f32, range: u8) {
    let m = &w.mobys[id];
    if m.drawn == 0 || reach <= crate::hero::physics::dist(m.pos, ctx.host.camera_pos()) { return; }
    probe(w, id, ctx);
    w.mobys[id].shadow_range = range;
}

/// The tick's game-side shadow work: Ratchet's shadow ([`update_hero`], his update `0x2a6dd0`'s call) on his moby,
/// then direction 0 from his key light ([`update_dir`], the play frame's).
pub fn hero_pass(w: &mut crate::moby::MobyWorld, ctx: &mut crate::moby::Ctx) {
    let Some(id) = ctx.hero_moby.filter(|&i| i < w.mobys.len()) else { return };
    let h = &*ctx.hero;
    let input = HeroIn {
        group: h.group, state: h.state, body: h.mode, air_ticks: h.air_ticks, ground_z: h.ground_z, water_z: h.surf.water_z,
        sand_z: h.surf.sand_z,
    };
    let gravity = h.gravity.on;
    let (coll, scene) = (ctx.coll, ctx.scene);
    let line = |a: V3, b: V3| coll.and_then(|c| crate::collision::coll_line_m(c, scene, a, b, crate::collision::QueryFlags(PROBE_FLAGS), None)).map(|o| o.point);
    update_hero(&mut w.mobys[id], &mut ctx.game.shadow, &input, gravity, line);
    let light = w.mobys[id].light;
    update_dir(&mut ctx.game.shadow, light);
}

/// What [`update_hero`] reads of Ratchet's block.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HeroIn {
    /// `0x18c0bc` his group, `0x18c0b4` his state, `0x18c0d4` his body (0 = Ratchet).
    pub group: i32,
    pub state: i32,
    pub body: u8,
    /// `0x18a16e`: ticks in the air.
    pub air_ticks: i16,
    /// `0x18a138`: the ground's height under him (his slab's seed); `0x18a150` / `0x18a154`: the liquid's and the
    /// quicksand's surface the ground probe last found (crate::hero::surface; kept when he leaves them).
    pub ground_z: f32,
    pub water_z: f32,
    pub sand_z: f32,
}

/// The pitch targets (0.97 on the ground, 1.56 in the air or in group 4) and the step a tick.
pub const PITCH_GROUND: f32 = f32::from_bits(0x3f78_51ec);
pub const PITCH_AIR: f32 = f32::from_bits(0x3fc7_ae14);
pub const PITCH_STEP: f32 = 0.052;
/// Ratchet's four probes (`$gp − 0x7098`: their turn from his key light's heading; `$gp − 0x70a8`: their height ×
/// his sphere's radius).
pub const HERO_PROBE_TURN: [f32; 4] = [std::f32::consts::PI, std::f32::consts::FRAC_PI_2, -std::f32::consts::FRAC_PI_2, 0.0];
pub const HERO_PROBE_Z: [f32; 4] = [-0.6, 0.7, 0.7, 0.7];

/// `0x2a97d8`, on Ratchet's moby `m`:
/// * no shadow (mode &= ~0x400) in his groups 0x12 (swimming: the pitch then 1.56), 0x15, 0x16, 0x1f and his state
///   0x7a; nothing at all on another body (`0x18c0d4` = 4);
/// * else mode 0x400. Tilted (his up row's z under 0.99, not in group 0x11) or in the gravity frames: the probe of
///   [`probe_down`]'s gravity branch, but 16 down his up row and the half thickness a quarter of his radius
///   (`+0xbd` 0xff, the pitch 1.56);
/// * upright: direction 1, its pitch stepping 0.052 a tick toward 0.97 (1.56 in the air or in group 4): `(cos p·cos
///   φ, cos p·sin φ, −sin p)`, φ his key light's heading. The slab seeds at his ground height when at least 1,
///   else the liquid's surface when at least 1, else the quicksand's when above 1; none else. With the pitch within 0.1 of 1.56 (under him) it is the seed, 0.18 thick; else four probes from his
///   sphere's centre, each turned [`HERO_PROBE_TURN`] from φ at 0.35 of his radius and [`HERO_PROBE_Z`] × it up,
///   along the direction twice its drop to 0.5 below the seed: the slab spans the seed and the hits, 0.12 more on
///   each side, at most 1 (the extra raises its middle).
pub fn update_hero(m: &mut Moby, s: &mut ShadowState, h: &HeroIn, gravity: bool, mut line: impl FnMut(V3, V3) -> Option<V3>) {
    if matches!(h.group, 0x12 | 0x15 | 0x16 | 0x1f) || h.state == 0x7a {
        if h.group == 0x12 { s.pitch = PITCH_AIR; }
        m.mode &= !MODE_CASTER;
        return;
    }
    if h.body == 4 { return; }
    m.mode |= MODE_CASTER;
    if gravity || (m.rows[2][2] < 0.99 && h.group != 0x11) {
        m.shadow_dir = -1;
        let c = scale([m.bsphere[0], m.bsphere[1], m.bsphere[2]], 1.0 / 1024.0);
        match line(c, add(c, scale(m.rows[2], -16.0))) {
            Some(p) => m.shadow_slab = [dot(sub(p, m.pos), m.rows[2]), m.bsphere[3] * (1.0 / 4096.0)],
            None => clear(m),
        }
        s.pitch = PITCH_AIR;
        return;
    }
    m.shadow_dir = DIR_HERO as i8;
    let target = if h.air_ticks != 0 || h.group == 4 { PITCH_AIR } else { PITCH_GROUND };
    let step = sub_rot(target, s.pitch).clamp(-PITCH_STEP, PITCH_STEP);
    s.pitch = add_rot(s.pitch, step);
    let l = key_light(&s.key_lights, m.light);
    let phi = atan2(l[0], l[1]);
    let dir = [phi.cos() * s.pitch.cos(), phi.sin() * s.pitch.cos(), -s.pitch.sin()];
    s.dirs[DIR_HERO] = [dir[0], dir[1], dir[2], 0.0];
    let seed = if 1.0 <= h.ground_z {
        h.ground_z
    } else if 1.0 <= h.water_z {
        h.water_z
    } else if 1.0 < h.sand_z {
        h.sand_z
    } else {
        return clear(m);
    };
    if (s.pitch - PITCH_AIR).abs() < 0.1 {
        m.shadow_slab = [seed - m.pos[2], f32::from_bits(0x3e38_51ec)];
        return;
    }
    let (mut lo, mut hi) = (seed, seed);
    let c = scale([m.bsphere[0], m.bsphere[1], m.bsphere[2]], 1.0 / 1024.0);
    let r = m.bsphere[3] / 1024.0;
    for k in 0..4 {
        let a = add_rot(phi, HERO_PROBE_TURN[k]);
        let st = [c[0] + a.cos() * r * 0.35, c[1] + a.sin() * r * 0.35, c[2] + HERO_PROBE_Z[k] * r];
        let drop = (st[2] - seed) + 0.5;
        let e = add(st, scale(dir, (drop + drop) / -dir[2]));
        if let Some(p) = line(st, e) {
            lo = lo.min(p[2]);
            hi = hi.max(p[2]);
        }
    }
    let mut slab = [(lo + hi) * 0.5 - m.pos[2], (hi - lo) * 0.5 + 0.12];
    if 1.0 < slab[1] {
        slab[0] += (slab[1] - 1.0) * 0.5;
        slab[1] = 1.0;
    }
    m.shadow_slab = slab;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool { (a - b).abs() < 1e-5 }

    fn caster() -> Moby {
        let mut m = Moby::init(1, 0x0b0a, None);
        m.pos = [10.0, 20.0, 60.0];
        m.bsphere = [10.0 * 1024.0, 20.0 * 1024.0, 60.5 * 1024.0, 0.8 * 1024.0];
        m
    }

    #[test]
    fn direction_zero_leans_along_the_key_light() {
        let mut s = ShadowState { key_lights: vec![[1.0, 0.0, -0.3], [0.0, 1.0, 0.0]], ..Default::default() };
        update_dir(&mut s, 0);
        assert!(close(s.dirs[0][0], 0.14) && close(s.dirs[0][1], 0.0) && s.dirs[0][2] == -0.99);
        // Half-way cross-fade to set 1: heading 45°.
        update_dir(&mut s, 0x0080_0100);
        assert!(close(s.dirs[0][0], s.dirs[0][1]));
        assert_eq!(key_light(&s.key_lights, 9), [0.0; 3], "a set past the table");
    }

    #[test]
    fn probe_down_puts_the_slab_on_the_floor() {
        let mut m = caster();
        probe_down(&mut m, false, |a, b| {
            assert_eq!((a[2], b[2]), (60.5, 44.0));
            Some([10.0, 20.0, 59.0])
        });
        assert!(close(m.shadow_slab[0], -1.0) && close(m.shadow_slab[1], 0.2), "{:?}", m.shadow_slab);
        probe_down(&mut m, false, |_, _| None);
        assert_eq!(m.shadow_slab, [0.0; 2]);
        // Tipped over: none.
        m.rows[2] = [0.0, 0.5, 0.866];
        probe_down(&mut m, false, |_, _| Some([0.0; 3]));
        assert_eq!(m.shadow_slab, [0.0; 2]);
        // The gravity frames: down the up row, the reversed row as the direction.
        let mut m = caster();
        probe_down(&mut m, true, |a, b| {
            assert!(close(a[2] - b[2], 8.0));
            Some([10.0, 20.0, 58.0])
        });
        assert!(close(m.shadow_slab[0], -2.0) && m.shadow_slab[1] == 0.2 && m.shadow_dir == -1);
        assert_eq!(ShadowState::default().dir_of(&m), [0.0, 0.0, -1.0]);
    }

    #[test]
    fn hero_pitch_steps_and_the_four_probes_span_the_slab() {
        let mut m = caster();
        let mut s = ShadowState { pitch: PITCH_GROUND, key_lights: vec![[1.0, 0.0, 0.0]], ..Default::default() };
        let h = HeroIn { ground_z: 60.0, ..Default::default() };
        let mut n = 0;
        update_hero(&mut m, &mut s, &h, false, |_, _| {
            n += 1;
            Some([0.0, 0.0, if n == 1 { 59.0 } else { 60.0 }])
        });
        assert_eq!((n, m.shadow_dir, m.mode & MODE_CASTER), (4, 1, MODE_CASTER));
        // Seed 60, hits 59 and 60: middle 59.5, half span 0.5 + 0.12.
        assert!(close(m.shadow_slab[0], -0.5) && close(m.shadow_slab[1], 0.62), "{:?}", m.shadow_slab);
        assert!(close(s.dirs[1][2], -PITCH_GROUND.sin()));
        // In the air the pitch steps toward 1.56; near it the slab is the seed's.
        let air = HeroIn { air_ticks: 1, ..h };
        for _ in 0..20 { update_hero(&mut m, &mut s, &air, false, |_, _| None); }
        assert!(close(s.pitch, PITCH_AIR));
        assert!(close(m.shadow_slab[0], 0.0) && close(m.shadow_slab[1], 0.18));
        // No ground under him: the liquid's surface, else the quicksand's (above 1), else none.
        let over = HeroIn { ground_z: 0.0, water_z: f32::NEG_INFINITY, sand_z: 58.0, ..air };
        update_hero(&mut m, &mut s, &over, false, |_, _| None);
        assert!(close(m.shadow_slab[0], -2.0) && close(m.shadow_slab[1], 0.18), "{:?}", m.shadow_slab);
        update_hero(&mut m, &mut s, &HeroIn { water_z: 59.0, ..over }, false, |_, _| None);
        assert!(close(m.shadow_slab[0], -1.0), "{:?}", m.shadow_slab);
        update_hero(&mut m, &mut s, &HeroIn { sand_z: 1.0, ..over }, false, |_, _| None);
        assert_eq!(m.shadow_slab, [0.0; 2]);
        // Swimming: off.
        update_hero(&mut m, &mut s, &HeroIn { group: 0x12, ..h }, false, |_, _| None);
        assert_eq!(m.mode & MODE_CASTER, 0);
    }
}
