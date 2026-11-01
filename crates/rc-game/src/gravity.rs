//! The gravity frames (G-LVL-003): R&C2's gravity spheres — places where "down" is toward (or away from) a point
//! instead of world −z, so Ratchet walks round a small world. One global flag (`0x1a8ff0` = `$gp − 0x6000`) turns
//! them on; Ratchet's frame mode (`0x18c0e3`) and his gravity direction (`0x18a0e0`) follow from it each tick, and
//! about 110 readers across the engine test it (Ratchet's physics, his camera, the shadows' probes, the AI, the
//! aims, the shards, the particles).
//!
//! **The data.** The block at `0x1a8ff0`: `+0` on, `+4` the cylinder mode (gravity across a vertical axis through
//! `0x1a9000`, heights measured as 100 − the distance from it), `+0x10` the cylinder's axis point. The sphere's
//! centre is in Ratchet's block at `0x189ed0` (its w `0x189edc` > 0: down toward the centre, else away from it);
//! the heights (`0x326820`, L02 `0x32ad88`) are distances from a second copy, `0x1b1a90` (L02 `0x1b3210`).
//!
//! **The writers.** The level load (`0x3012a0`) clears the flag and sets it from the level settings (their word
//! after the death height, with the centre `0x1b1a90` after it); Maktar's upper area (class 0x0b25, L02 `0x3f6820`,
//! crate::moby::classes::maktar::update_0b25) sets both centres to (500, 200, 500) and turns the flag on when
//! Ratchet rises past z 420 within 150 of it, off when he drops back below 420 (or rises past it farther away). Nothing of Maktar's world lies there (no collision
//! or tfrag of the level or its chunks within 150 of the centre; the arena, the only world above z 420, is 400 away), so
//! in its play the frames never turn on; the dev pin `RC_GRAVITY_AT=x,y,z` (crate `rc-engine` play.rs) tries them.
//!
//! **Ratchet's frame** (`0x2b37e8` → `0x2d37a0`): mode 2 while the sphere is on (not the cylinder). In a frame
//! mode his Euler angles (`0x189eb0`) carry a pitch and roll, the rows `0x189e20` (and their transpose `0x189e60`)
//! are his local frame, and the helpers `0x2d3860` … `0x2d4248` (crate::hero::physics, crate::hero::frame) do
//! their world-z work along his local up instead; his ground probe looks along the last tick's down, and the
//! post-move (`0x2c3330` → `0x2b9488` → `0x2b92c8` / `0x2b9008`) springs his up toward −down.
//!
//! Ported: the flag, the sphere's centres, [`Gravity::height`] (`0x326820`), [`Gravity::up_at`] (`0x326448` /
//! L02 `0x3864f0` without the magnetic walkway), Ratchet's mode 2 and down, the hero helpers' frame branches, his
//! camera's up (`0x2d9760`) and its frame branches (`0x421dc8`, `0x4227b8`, `0x4239d0`), the shadow probes'
//! frame branch (`0x31da58`, `0x2a97d8`).
//!
//! Not ported: the cylinder mode (`0x1a8ff4`, `0x1a9000`, `0x3263d0`; no consumer yet), the level settings' sphere
//! (`0x3012a0`; no level known to use it), Ratchet's frame mode 1 (the magnetic walkways with the Gravity Boots
//! `0x18a168`, the states 0x3b / 0x6d / 0x6e and `0x18a038`: their own smoothed normal `0x18a110` / `0x18a11c`), the
//! other readers (listed in docs/plan/gaps.md G-LVL-003).

use crate::hero::physics::{dist, dot, len, scale, sub, unit, V3};

/// The gravity frames' state. Lives in Ratchet's block (crate::hero::Hero::gravity): the readers outside the hero
/// (the class updates, the camera, the shadows) reach it through him.
#[derive(Clone, Copy, Debug)]
pub struct Gravity {
    /// `0x1a8ff0`: the sphere on.
    pub on: bool,
    /// `0x189ed0`: the sphere's centre; `0x189edc` > 0 (`inward`): down is toward it.
    pub centre: V3,
    pub inward: bool,
    /// `0x1b1a90` (L02 `0x1b3210`): the centre the heights are measured from.
    pub height_centre: V3,
    /// `0x18c0e3`: Ratchet's frame mode this tick (0 world, 2 the sphere; 1 the walkways, not ported).
    pub mode: u8,
    /// `0x18a0e0`: Ratchet's gravity direction (unit; (0, 0, −1) in mode 0), set at the end of his ground probe.
    pub down: V3,
    /// `0x18be50`: Ratchet's position at the start of the tick (the post-move holds him there when he stands still in
    /// a frame).
    pub tick_start: V3,
    /// Ratchet's moby's rows (its angles `+0xf0` as the last tick's write-back left them): the helpers `0x2d3b38`,
    /// `0x2d3c40`, `0x2d3df8` turn through them instead of `0x189e20`.
    pub moby_rows: [V3; 3],
    /// `0x189fa4`: the length of this tick's push step (`0x189f70`; 0 without a push).
    pub push_step: f32,
    /// The port's: Ratchet still carries a frame's pitch and roll. Out of the frames `0x2b9488`'s mode-0 branch springs
    /// them back to 0; the port runs it only while this is set (crate::hero::frame), so the world path of a level
    /// without frames is unchanged.
    pub tilted: bool,
}

impl Default for Gravity {
    fn default() -> Self {
        Gravity {
            on: false,
            centre: [0.0; 3],
            inward: false,
            height_centre: [0.0; 3],
            mode: 0,
            down: [0.0, 0.0, -1.0],
            tick_start: [0.0; 3],
            moby_rows: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            push_step: 0.0,
            tilted: false,
        }
    }
}

impl Gravity {
    /// `0x2d37a0`'s sphere part: mode 2 while the sphere is on (the walkways' mode 1 is not ported).
    pub fn frame_mode(&self) -> u8 { if self.on { 2 } else { 0 } }

    /// `0x326820(p)` (L02 `0x32ad88`): the height in the frames — the distance from the centre `0x1b1a90`.
    pub fn height(&self, p: V3) -> f32 { dist(p, self.height_centre) }

    /// `0x2b4a08`'s end in mode 2: the down at `p`, toward the centre (`0x189edc` > 0) or away from it.
    pub fn down_at(&self, p: V3) -> V3 {
        let d = unit(sub(self.centre, p));
        if self.inward { d } else { scale(d, -1.0) }
    }

    /// `0x326448(p, out, 1)` / L02 `0x3864f0`: the up at `p` — (0, 0, 1) with the frames off, else away from the
    /// centre (toward it when `0x189edc` ≤ 0). The Gravity Boots' magnetic walkway (Ratchet's up row) is not ported.
    pub fn up_at(&self, p: V3) -> V3 {
        if !self.on { return [0.0, 0.0, 1.0]; }
        scale(self.down_at(p), -1.0)
    }
}

/// A vector into the frame of `rows` (row i = image of axis i): `0x2ef340(out, v, transpose)`.
pub fn to_local(v: V3, rows: &[V3; 3]) -> V3 { [dot(v, rows[0]), dot(v, rows[1]), dot(v, rows[2])] }

/// A local vector back into the world: `0x2ef340(out, l, rows)`.
pub fn to_world(l: V3, rows: &[V3; 3]) -> V3 { std::array::from_fn(|k| l[0] * rows[0][k] + l[1] * rows[1][k] + l[2] * rows[2][k]) }

/// `v` turned by `angle` about the unit `axis` (right-handed).
pub fn rotate(v: V3, axis: V3, angle: f32) -> V3 {
    let (s, c) = angle.sin_cos();
    let k = axis;
    let kv = [k[1] * v[2] - k[2] * v[1], k[2] * v[0] - k[0] * v[2], k[0] * v[1] - k[1] * v[0]];
    let d = dot(k, v) * (1.0 - c);
    std::array::from_fn(|i| v[i] * c + kv[i] * s + k[i] * d)
}

/// Whether `axis` can be made unit.
pub fn has_len(v: V3) -> bool { 0.0 < len(v) }

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: V3, b: V3) -> bool { (0..3).all(|i| (a[i] - b[i]).abs() < 1e-5) }

    #[test]
    fn up_is_world_z_off_and_radial_on() {
        let mut g = Gravity::default();
        assert_eq!(g.up_at([3.0, 4.0, 5.0]), [0.0, 0.0, 1.0]);
        assert_eq!(g.frame_mode(), 0);
        g.on = true;
        g.inward = true;
        g.centre = [500.0, 200.0, 500.0];
        g.height_centre = g.centre;
        assert!(close(g.up_at([500.0, 200.0, 510.0]), [0.0, 0.0, 1.0]));
        assert!(close(g.down_at([510.0, 200.0, 500.0]), [-1.0, 0.0, 0.0]));
        assert!((g.height([500.0, 210.0, 500.0]) - 10.0).abs() < 1e-5);
        assert_eq!(g.frame_mode(), 2);
    }

    #[test]
    fn frames_round_trip_and_rotate() {
        let rows = crate::hero::physics::euler_rows([0.3, -0.2, 1.1]);
        let v = [0.4, -1.5, 2.0];
        assert!(close(to_world(to_local(v, &rows), &rows), v));
        assert!(close(rotate([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], std::f32::consts::FRAC_PI_2), [1.0, 0.0, 0.0]));
    }
}
