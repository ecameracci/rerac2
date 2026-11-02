//! Type 0x4e: a swirl (`0x33d968` / `0x33daf8`; the Sheepinator's beam, crate::moby::classes::sheepinator): a twinkle
//! that flies its velocity and, near the moby it follows, circles it and closes in.
//!
//! **Made** ([`swirl`], `0x33d968(size, end, at, life, rgba, rot, spin, vel, moby)`): ALPHA 0x48, near 4 / far 32
//! (`+0x09` 0x24), frame `*def[0x4e]`, at `at`, `rgba`, from `end`·210000 to `size`·210000 (`+0x24` / `+0x20`) as a
//! twinkle swells (crate::particles::twinkle), its roll by `rot` (0: 0, 1: 0x20, else random) turning `spin` a tick,
//! `vel` (`+0x2c`), the moby it follows (`+0x38`) and that moby's aim height (`+0x3c`, read by nothing).
//!
//! **Each tick** ([`update`], `0x33daf8`): it turns; over its first 6 ticks it grows from its end size halfway to its
//! full size while its alpha halves, then on to the full size as the alpha runs out with the life; it moves its
//! velocity (`0x380e08`, the gravity frames' bend, not ported). With a moby to follow whose place is at most 2 ahead
//! of it in x and in y (the differences as they are, not their sizes): it steps 0.2 across (gravity × the way to the
//! moby) and its velocity turns to the moby — kept its speed while more than 0.5 off in x or y, else 0.03 — and rises
//! 0.02. Its life out: gone.

use super::{dec_life, rec, Particles, GRAVITY};
use crate::hero::physics::{add, cross, len, set_len, sub};
use crate::rng::Rng;

/// Type 0x4e.
pub const SWIRL: u8 = 0x4e;

/// `0x33d968` (module docs): `follow` the moby and its aim height.
#[allow(clippy::too_many_arguments)]
pub fn swirl(sys: &mut Particles, rng: &mut Rng, size: f32, end: f32, at: [f32; 3], life: i16, rgba: u32, rot: u8, spin: i8, vel: [f32; 3], follow: Option<(usize, f32)>) -> Option<usize> {
    let i = sys.create_part(SWIRL)?;
    let frame = sys.def_first(SWIRL);
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, at);
    rec::set_u32(r, 4, rgba);
    r[3] = 0x48;
    r[1] = 0;
    r[9] = 4 + 0x20;
    rec::set_i16(r, 0xa, life);
    r[2] = frame;
    rec::set_ff(r, 0xc, size * 210000.0);
    r[8] = match rot { 0 => 0, 1 => 0x20, _ => rng.randi(0xff) as u8 };
    rec::set_i16(r, 0x28, life);
    rec::set_ff(r, 0x20, size * 210000.0);
    r[0x2a] = spin as u8;
    rec::set_ff(r, 0x24, end * 210000.0);
    r[0x2b] = (rgba >> 24) as u8;
    rec::set_u32(r, 0x38, follow.map_or(0, |(m, _)| m as u32 + 1));
    rec::set_ff(r, 0x3c, follow.map_or(0.0, |(_, h)| h));
    rec::set_v3(r, 0x2c, vel);
    Some(i)
}

/// `0x33daf8` (module docs).
pub fn update(sys: &mut Particles, i: usize, mobys: &crate::moby::MobyWorld) {
    let r = &mut sys.pool.recs[i];
    r[8] = r[8].wrapping_add(r[0x2a]);
    let (life0, life) = (rec::i16(r, 0x28) as i32, rec::i16(r, 0xa) as i32);
    let (full, end, a) = (rec::ff(r, 0x20), rec::ff(r, 0x24), r[0x2b] as i32);
    let e = life0 - life;
    let alpha = if e < 6 {
        let f = e as f32 * 0.2;
        rec::set_ff(r, 0xc, end + ((full + end) * 0.5 - end) * f);
        a as f32 + ((a >> 1) - a) as f32 * f
    } else {
        let f = life as f32 / (life0 - 6) as f32;
        rec::set_ff(r, 0xc, full + ((full + end) * 0.5 - full) * f);
        (a >> 1) as f32 * f
    };
    let c = rec::u32(r, 4);
    rec::set_u32(r, 4, (alpha as i32 as u32) << 24 | c & 0xff_ffff);
    let mut v = rec::v3(r, 0x2c);
    let mut p = add(rec::pos(r), v);
    let follow = rec::u32(r, 0x38).checked_sub(1).and_then(|m| mobys.mobys.get(m as usize));
    if let Some(m) = follow {
        let d = sub(m.pos, p);
        if d[0] <= 2.0 && d[1] <= 2.0 {
            p = add(p, set_len(cross(GRAVITY, d), 0.2));
            v = if 0.5 < d[0] || 0.5 < d[1] { set_len(d, len(v)) } else { set_len(d, f32::from_bits(0x3cf5_c28f)) };
            v[2] += 0.02;
            rec::set_v3(r, 0x2c, v);
        }
    }
    rec::set_v3(r, 0x10, p);
    if dec_life(r) { sys.kill_part(i); }
}
