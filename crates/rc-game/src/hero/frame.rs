//! Ratchet in the gravity frames (crate::gravity; L00 addresses): the frame branches of the helpers the states
//! call (`0x2d3860` … `0x2d4248`: each mode-0 branch is crate::hero::physics's world-z code), the turn about his own
//! up (`0x2b3f48` / `0x2b3ec8`), the yaw with no stick (`0x2d3630`), the post-move's spring of his up toward −down
//! (`0x2b92c8` / `0x2b9008`) and the hold when he stands still (`0x2c3330`).
//!
//! In a frame mode (`0x18c0e3` ≠ 0) his rows `0x189e20` (`rows`, from his Euler angles, pitch and roll included)
//! are his local frame and `0x189e60` their transpose: a helper takes a vector into it, does the mode-0 work on its
//! local z, and turns it back. The ones that read his moby's angles (`+0xf0`) turn through
//! [`crate::gravity::Gravity::moby_rows`] instead.
//!
//! Not ported: mode 1 (the magnetic walkways, crate::gravity), the jump attack's frame fall cap (`0x2ba8e0`: −1.1667
//! instead of −0.6167 with the frames on), `0x18a038`'s soft spring of the up (0.001 in the air).

use super::physics::*;
use super::Hero;
use crate::collision::{coll_sphere_hero_groups, coll_sphere_m, QueryFlags};
use crate::gravity::{rotate, to_local, to_world};

impl Hero {
    /// `0x2d3ea0(v)`: v's local z (mode 0: v.z).
    pub fn local_z(&self, v: V3) -> f32 {
        if self.gravity.mode == 0 { v[2] } else { to_local(v, &self.rows)[2] }
    }

    /// `0x2d3ef8(z, out, v)`: v with its local z set to `z`.
    pub fn with_local_z(&self, v: V3, z: f32) -> V3 {
        if self.gravity.mode == 0 { return [v[0], v[1], z]; }
        let mut l = to_local(v, &self.rows);
        l[2] = z;
        to_world(l, &self.rows)
    }

    /// `0x2d3f78(l, out, v)`: v with its local xy at length `l` (`0x2ef188`; 0: only its local z kept).
    pub fn with_flat_len(&self, v: V3, l: f32) -> V3 {
        if self.gravity.mode == 0 { return set_len2(v, l); }
        to_world(set_len2(to_local(v, &self.rows), l), &self.rows)
    }

    /// `0x2d3df8(v)`: the length of v's local xy, through his moby's rows (mode 0: `0x2eefd0`).
    pub fn flat_len(&self, v: V3) -> f32 {
        if self.gravity.mode == 0 { len2(v) } else { len2(to_local(v, &self.gravity.moby_rows)) }
    }

    /// `0x2d3c40(k, out, v)`: v + his up · k (his moby's up row; mode 0: z + k).
    pub fn add_up(&self, v: V3, k: f32) -> V3 {
        if self.gravity.mode == 0 { return add(v, [0.0, 0.0, k]); }
        add(v, scale(self.gravity.moby_rows[2], k))
    }

    /// `0x2d3a80(0, 0, h, out, 1)`: the point `h` up his rows `0x189e20` from the feet (every mode; the mode-0 code
    /// in crate::hero::physics takes his rows as upright).
    pub fn up_point(&self, h: f32) -> V3 { add(self.pos, scale(self.rows[2], h)) }

    /// `0x2d3d00(out, v)`: v without its part along the gravity (mode 2: `0x18a0e0`; mode 0: z = 0).
    pub fn flat(&self, v: V3) -> V3 {
        if self.gravity.mode == 0 { return [v[0], v[1], 0.0]; }
        let d = self.gravity.down;
        sub(v, scale(d, dot(d, v)))
    }

    /// `0x2d4248(n)`: the slope of a normal in his frame, `atan2(|local xy|, local z)`.
    pub fn frame_slope(&self, n: V3) -> f32 {
        if self.gravity.mode == 0 { return slope_of(n); }
        let l = to_local(n, &self.rows);
        len2(l).atan2(l[2])
    }

    /// `0x2d3860(g, out, src)` in mode 2: `out` keeps its local xy, takes `src`'s local z and gains `g` along the gravity
    /// `0x18a0e0` (the climb 0x3a on the ground: `g` into the ground normal `0x18a0c0` instead).
    pub(super) fn frame_gravity(&mut self, src: V3, g: f32) {
        let pull = if self.state == 0x3a && self.air_ticks == 0 { set_len(self.ground_normal, -g) } else { set_len(self.gravity.down, g) };
        let up = self.with_flat_len(src, 0.0);
        let v = self.with_local_z(self.vel, 0.0);
        self.vel = add(add(v, up), pull);
    }

    /// `0x2b3f48(k, d, max, mode)`: the turn in a frame — the stick's direction `0x189fb0` taken into his frame
    /// (`0x189e60`), its local yaw the target of the angle spring from 0 (velocity `0x189fc4`), the step a turn about
    /// his own up ([`Hero::turn_local`]); the remaining difference to `0x189fc8`.
    pub(super) fn frame_turn(&mut self, k: f32, d: f32, max: f32, mode: i32) {
        let l = to_local(self.stick_dir, &self.rows);
        let t = atan2(l[0], l[1]);
        let (mut a, mut v) = (0.0, self.yaw_vel);
        self.yaw_residual = turn_spring(t, k, d, max, &mut a, &mut v, mode);
        self.yaw_vel = v;
        self.turn_local(a);
    }

    /// `0x2b3ec8(0, 0, a)`: his angles turned by `a` about his own up (`0x2efa50(R, R, Rz(a))` → `0x320000`).
    pub fn turn_local(&mut self, a: f32) {
        let r = euler_rows(self.rot);
        let z = euler_rows([0.0, 0.0, a]);
        let out = z.map(|row| to_world(row, &r));
        self.rot = crate::moby::attach::angles_of(&out);
    }

    /// `0x2d3630`: the target yaw with no stick in a frame — his facing (from his angles) taken off the ground's slope
    /// (`0x18a0c0`: turned by −its yaw, by −its tilt, back by its yaw), its yaw.
    pub(super) fn frame_yaw(&self) -> f32 {
        let mut u = euler_rows(self.rot)[0];
        let n = self.ground_normal;
        if n[0] != 0.0 || n[1] != 0.0 {
            let yn = atan2(n[0], n[1]);
            let rz = euler_rows([0.0, 0.0, -yn]);
            u = to_world(u, &rz);
            let f = to_world(n, &rz);
            let tilt = atan2(f[2], f[0]);
            u = to_world(u, &euler_rows([0.0, -tilt, 0.0]));
            u = to_world(u, &euler_rows([0.0, 0.0, yn]));
        }
        atan2(u[0], u[1])
    }

    /// `0x2b92c8` (mode 2) with `0x2b9008(0.035, 0.3, 0, −down)`: the angle between his up row and −down springs from 0
    /// (velocity `0x189ec4`) and his rows turn by the step about their cross product (`0x2eef88(up, row)` = row × up;
    /// about z when its z is 0.99 or more: the original's yaw / tilt decomposition degenerates there); the angles
    /// (`0x320000`) go to `0x189eb0`.
    pub(super) fn frame_align(&mut self) {
        let up = scale(self.gravity.down, -1.0);
        let m = self.rows;
        let c = dot(up, m[2]);
        let s = (1.0 - c * c).max(0.0).sqrt();
        let angle = atan2(c, s);
        let axis = cross(m[2], up);
        let (mut step, mut v) = (0.0, self.rot_vel[1]);
        turn_spring(angle, 0.035, 0.3, 0.0, &mut step, &mut v, 0);
        self.rot_vel[1] = v;
        let n = if axis[2] < 0.99 {
            if !crate::gravity::has_len(axis) { return; }
            unit(axis)
        } else {
            [0.0, 0.0, 1.0]
        };
        let out = m.map(|row| rotate(row, n, step));
        self.rot = crate::moby::attach::angles_of(&out);
    }

    /// `0x2b9488`'s mode-0 branch, after a frame: outside the states 0x24 / 0x25 / 0x74 / 0x30 / 0x2c / 0x2b and the
    /// groups 0x11 / 0x15 / 0x16 / 0x1f, his pitch and roll spring to 0 (0.015, 0.3, at most 0.0873; velocities
    /// `0x189ec0` / `0x189ec4`) and the velocities turn him about his own axes (`0x2b3ec8(vx, vy, 0)`). Done once both
    /// velocities are 0.
    pub(super) fn frame_release(&mut self) {
        let skip = matches!(self.state, 0x24 | 0x25 | 0x74 | 0x30 | 0x2c | 0x2b) || matches!(self.group, 0x11 | 0x15 | 0x16 | 0x1f);
        if skip { return; }
        let (mut x, mut y) = (self.rot[0], self.rot[1]);
        let [mut vx, mut vy] = self.rot_vel;
        turn_spring(0.0, 0.015, 0.3, f32::from_bits(0x3db2_b8c4), &mut x, &mut vx, 0);
        turn_spring(0.0, 0.015, 0.3, f32::from_bits(0x3db2_b8c4), &mut y, &mut vy, 0);
        self.rot_vel = [vx, vy];
        let r = euler_rows(self.rot);
        let d = euler_rows([vx, vy, 0.0]);
        self.rot = crate::moby::attach::angles_of(&d.map(|row| to_world(row, &r)));
        if vx == 0.0 && vy == 0.0 { self.gravity.tilted = false; }
    }

    /// `0x2c3330`: in a frame, idle (group 0) on the ground with neither a step (`0x189f90`) nor a push (`0x189fa4`) of
    /// 1/600, he stays where the tick began (`0x18be50`).
    pub(super) fn frame_hold(&mut self) {
        if self.gravity.mode == 0 || self.group != 0 || self.grounded_ticks == 0 { return; }
        if self.eff_len < 1.0 / 600.0 && self.gravity.push_step < 1.0 / 600.0 { self.pos = self.gravity.tick_start; }
    }

    /// `0x2b4560(always_forward)` in a frame: below 1/600 of local speed (`0x2d3df8`) nothing; the two lines start up
    /// his rows (0.37 and 0.111; Clank 0.35, 0.27 ahead), ahead along his forward row (or the velocity without its
    /// local z, when its xy is 0.01 or more and `always_forward` is 0); a hit steeper than 50° in his frame (or surface
    /// 8 / 0xc) takes the velocity's part into the wall's normal (its local z dropped) away.
    pub(super) fn frame_wall_check(&mut self, env: &Env, always_forward: bool) {
        if self.flat_len(self.vel) < 0.1 * DT { return; }
        let (h, ahead) = if matches!(self.group, 0x11 | 0x12) { (0.0, 0.7) } else if self.mode == 1 { (0.35, 0.27) } else { (0.37, 0.5) };
        let a = self.up_point(h);
        let a2 = self.up_point(h * 0.3);
        let (fwd, fwd2) = if always_forward || len2(self.vel) < 0.01 {
            (set_len(self.rows[0], ahead), set_len(self.rows[0], ahead * 1.4))
        } else {
            let f = set_len(self.with_local_z(self.vel, 0.0), ahead);
            (f, scale(f, 1.4))
        };
        let low = env.line(a2, add(a2, fwd2), 2);
        let low_surface = low.map(|o| o.surface_id());
        let high = env.line(a, add(a, fwd), 2);
        let slope = high.map_or(0.0, |o| self.frame_slope(o.normal));
        let hit = if SLOPE_MAX <= slope { high } else if matches!(low_surface, Some(8) | Some(0xc)) { low } else { None };
        if let Some(o) = hit {
            self.wall_moby = o.moby;
            let n = unit(self.with_local_z(o.normal, 0.0));
            let f = dot(n, self.vel);
            if f < 0.0 { self.vel = add(self.vel, scale(n, -f)); }
        }
    }

    /// `0x2ba008(k, min)` in a frame: the step ahead (velocity · k, at least `min` along his facing) without its local
    /// z, a line (flags 4) from 0.3 up his moby's up row to 0.35 down it (Clank 0.7); no floor flatter than 50° in his
    /// frame brakes the move (his velocity keeps only its local z).
    pub(super) fn frame_edge_brake(&mut self, env: &Env, k: f32, min: f32) {
        if self.flat_len(self.vel) == 0.0 || !(self.stick_mag <= 0.5 || self.group == 6) || self.air_ticks != 0 { return; }
        let k = if self.state == 0x1f { 2.0 } else { k };
        let mut d = scale(self.vel, k);
        if 0.0 < min && self.flat_len(d) < min { d = self.along_facing(self.with_flat_len(d, 0.0), min); }
        let p = add(self.with_local_z(d, 0.0), self.pos);
        let a = self.add_up(p, 0.3);
        let b = self.add_up(p, if self.mode == 2 { -0.7 } else { -0.35 });
        let floor = env.line(a, b, 4).is_some_and(|o| self.frame_slope(o.normal) <= SLOPE_MAX);
        if !floor { self.vel = self.with_flat_len(self.vel, 0.0); }
    }

    /// `0x2b56f8` in mode 2: the capsule is a sphere of his radius 0.77 up his moby's up row (`0x2d3c40(0.77)`), up to
    /// 8 passes (flags 0x24) and the hero-only groups; `false` when the total push exceeds 1.5·r.
    pub(super) fn frame_capsule_try(&mut self, env: &Env) -> bool {
        let old = self.pos;
        let off = scale(self.gravity.moby_rows[2], 0.77);
        self.pos = add(self.pos, off);
        let r = self.cap_radius;
        let flags = QueryFlags::exclude_surface(0) | QueryFlags::MOBY_SUBMASK;
        for _ in 0..8 {
            let mut o = coll_sphere_m(env.coll, env.scene, self.pos, r, flags, env.hero_moby);
            if let Some(g) = coll_sphere_hero_groups(env.coll, self.pos, r) { o = Some(g); }
            let Some(o) = o else { break };
            self.push_moby = o.moby;
            if let Some(pc) = o.pushed_centre { self.pos = pc; }
            self.contact_normal = o.normal;
            self.contact_point = o.point;
            self.cap_hit = 1;
        }
        self.pos = sub(self.pos, off);
        len(sub(self.pos, old)) <= r * 1.5
    }

    /// `0x2b5358` in a frame: on the ground and below it (height < 0): within his local vertical speed + 0.01 (the
    /// capsule clear of the contact) onto the ground point, else the point spring toward it (`0x31ec88(0.05, 0.21,
    /// 1/12)`).
    pub(super) fn frame_step_snap(&mut self) {
        if self.air_ticks != 0 || 0.0 <= self.height {
            self.snap_vel = 0.0;
            return;
        }
        let depth = self.height.abs();
        if depth < self.local_z(self.vel).abs() + 0.01 && (self.cap_hit == 0 || self.cap_radius * 0.5 < dist2(self.contact_point, self.pos)) {
            self.pos = self.ground_point;
            return;
        }
        let (mut p, mut v) = (self.pos, self.snap_vel);
        Self::spring_point(0.05, 0.21, 1.0 / 12.0, &mut p, self.ground_point, &mut v);
        self.pos = p;
        self.snap_vel = v;
    }

    /// `0x31ec88(k, d, max, &x, &t, &v)`: the point `x` springs toward `t` along the line between them ([`spring_vel`]
    /// on the distance); within `0.01·max` of the rest it lands on `t`.
    pub(super) fn spring_point(k: f32, d: f32, max: f32, x: &mut V3, t: V3, v: &mut f32) {
        let e = sub(t, *x);
        let l = len(e);
        spring_vel(l, k, d, max, v);
        *x = add(*x, set_len(e, *v));
        if (l - *v).abs() < max * 0.01 {
            *x = t;
            *v = 0.0;
        }
    }
}
