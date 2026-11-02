//! The first-person camera, class 4 (L00 camera class table `0x2a3a00` row 1: activation `0x428188`, init `0x428518`
//! → `0x427b90` / `0x427a70`, update `0x427bf8` with the eye `0x427990`, release `0x4285d0`), and the camera switch
//! blend it asks for (`0x1b8e90`: the capture `0x2d8ab0`, the step `0x2d8d00`, the end `0x2d9488`, run by the camera
//! update `0x2d9f38`). R&C1's first-person camera (rerac:rc_game::follow_camera `FirstPerson`, `CamBlend`) is the map;
//! R&C2's is the same camera as a level camera record of class 4 chosen by `0x2d8618` (crate::cameras).
//!
//! **The activation** (`0x428188`, the record's pvars `+0x20` count, `+0x22` counting, `+0x24` the turn back): with
//! Ratchet's camera mode (`0x18c2c0`) 4 (the look stance 1) or 0x51 (its twin 0x1d), and the best and current
//! cameras of the classes 0 / 2 / 10 / 14, it counts; past 6 ticks it is chosen (taking priority 5 over the follow
//! camera), until then the follow camera turns toward Ratchet's facing at 4° a tick (`0x426410`). The stance over
//! while counting: with the follow camera current and its view 80° or more off Ratchet's facing, the follow camera
//! is reset behind him (`0x2da160`); else it turns there at 8° a tick for 30 ticks.
//!
//! **The init** (`0x428518`): the blend in (`0x1b8e90` = 1, or 2 during a blend; kind 0, rates 0.075 a tick), its
//! data cleared, the eye ([`eye_height`] above the feet along his up) and rows facing the old camera's forward about
//! his up. **The update** (`0x427bf8`): the eye (plus the tick's platform carry); once the blend is over, the view lock
//! `0x18c0d5` (Ratchet then hides and faces the view: crate::hero); the yaw from the left stick's x, else the right's,
//! else the d-pad (← +1, → −1), eased (`0x2d7e98`: 0.03, 0.4) and turned at up to 1.5° a tick about his up; the
//! platform's turn; the pitch from the left stick's y, else the right's, else the d-pad (↑ +1, ↓ −1; none while
//! blending in from a run; inverted unless the Camera page's pitch is normal), eased the same, slowed within 20° of
//! 64° down and stopped at 84°; looking down past 63.9° pushes the eye 0.5 forward (eased 0.005 / 0.2, back when a
//! 0.3 sphere there meets the world). **The release** (`0x4285d0`, when the camera mode is no longer 4 / 0x51): a
//! blend out at 0.018 a tick (release 2), or a cut when a blend is running and the view is 80° or more off his facing
//! (release 4).
//!
//! **The blend** (kind 0; record `0x1b8ea0`): at the switch the old camera's place and rotation are captured (a blend
//! started during one starts again from the first capture: `0x2d8a78`); every tick the view is the capture — carried
//! with the platform (`0x189f70`) — moved toward the new camera's place by the cosine blend of `t_pos` and turned
//! toward its rows by that of `t_rot` (`0x2efad8`), each `t` growing by its rate to 1; both at 1, the camera's own view.
//!
//! Not ported: the camera grid's "no first person" cells (`0x2db718` bit 1: no camera grid), the veto `$gp+0x258c`
//! and the record's input freeze `+0x26` (no writer found), the stats counters (`0x18c8e8..`), the camera moby's sound
//! (`0x3605f0(0, 0)` when `0x1b8e64`), the orbit blend (kind 2, `0x2d8ea8`: the Swingshot's camera), the First Person
//! Mode (`0x1a790c`, its own aim `0x2d54f8` and view `0x2d9f38`'s tail: the Specials page is not ported).

use crate::camera::{cos_interp, interp, rot, FollowCamera};
use crate::hero::physics::{add, add_rot, dot, len, scale, set_len, sub, sub_rot, V3};
use crate::hero::Hero;
use crate::pad::button;
use crate::quat::{self, Q};

/// The class.
pub const CLASS: i16 = 4;
/// `0x2d8168` / `0x4285d0`: the blend out's rate (0x3c9374bc).
pub const RATE_OUT: f32 = 0.018;
/// `0x428518`: the blend in's rate (0x3d99999a).
const RATE_IN: f32 = 0.075;
/// `gp+0x257c`: the most the view turns a tick (1.5°); `gp+0x2580` / `+0x2584`: the input's ease ([`Tune`]).
pub const TURN: f32 = 0.026_179_94;
pub const EASE_K: f32 = 0.03;
pub const EASE_D: f32 = 0.4;

/// The view's turn as globals others retune (`gp+0x257c` / `+0x2580` / `+0x2584`; the Pulse Rifle's zoom slows it,
/// crate::moby::classes::pulse_rifle): the most it turns a tick, the input's ease.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tune {
    pub turn: f32,
    pub k: f32,
    pub d: f32,
}

impl Default for Tune {
    fn default() -> Self { Tune { turn: TURN, k: EASE_K, d: EASE_D } }
}
/// 80°: the view and Ratchet's facing too far apart (a reset or a cut).
const APART: f32 = 1.396_263_6;

/// What the camera reads of the pad: the sticks (`0x138280..0x13828c`: right x, y, left x, y) and the held buttons.
#[derive(Clone, Copy, Debug, Default)]
pub struct Pad {
    pub right: [f32; 2],
    pub left: [f32; 2],
    pub held: u32,
}

/// The first-person camera's state: its D block and view; the activation's record words.
#[derive(Clone, Copy, Debug, Default)]
pub struct FirstPerson {
    /// D +0x00..+0x18: the yaw input and its velocity, the pitch input and its velocity, the pitch so far (down
    /// positive), the eye push and its velocity.
    pub d: [f32; 7],
    /// The view (the record's rows and `+0x30`).
    pub rows: [V3; 3],
    pub pos: V3,
    /// The record's `+0x20` (ticks counted), `+0x22` (counting), `+0x24` (the turn back's ticks).
    pub count: i16,
    pub counting: bool,
    pub turn_back: i16,
    /// The activation's verdict of this tick (`0x428188` returned 1), taken by the choice.
    pub want: bool,
    /// The turn's globals ([`Tune`]).
    pub tune: Tune,
}

/// The switch blend (`0x1b8e90..0x1b8f00`), kind 0 only.
#[derive(Clone, Copy, Debug, Default)]
pub struct Blend {
    /// `0x1b8e90`: 0 none, 1 / 2 capture this tick (2: one was running), 3 blending.
    pub mode: u8,
    /// `0x1b8ea8` / `0x1b8eb4`: the next blend's rotation and position rates.
    pub next_rot: f32,
    pub next_pos: f32,
    /// `0x1b8ea0`: t and rate of the rotation, `+0x0c` / `+0x10` of the position.
    pub t_rot: f32,
    pub rate_rot: f32,
    pub t_pos: f32,
    pub rate_pos: f32,
    /// `0x1b8ec0` / `0x1b8ed0`: the running blend's start rotation and place.
    pub start_q: Q,
    pub start_pos: V3,
    /// `0x1b8ef0` / `0x1b8ee0`: the capture.
    pub cap_q: Q,
    pub cap_pos: V3,
}

impl Blend {
    /// The blend asked for at a switch (`0x2d8168` / `0x428518`): 1, or 2 while one runs.
    pub fn ask(&mut self) { self.mode = if self.mode == 0 { 1 } else { 2 }; }

    /// `0x2d9f38`'s blend part: the capture of `old` (the old camera's view: the one shown last tick) when asked
    /// (`0x2d8ab0`), then the step toward `cur` (`0x2d8d00`; `0x2d9488` once both blends are done). Returns the view.
    pub fn step(&mut self, old: (V3, [V3; 3]), cur: (V3, [V3; 3]), carry: V3) -> (V3, [V3; 3]) {
        if self.mode == 1 || self.mode == 2 {
            // `0x2d8ab0` (kind 0): mode 1 captures the old camera; mode 2 keeps the first capture (`0x2d8a78`).
            if self.mode == 1 {
                self.cap_pos = old.0;
                self.cap_q = quat::from_rows(&old.1);
            }
            self.mode = 3;
            (self.t_rot, self.rate_rot, self.t_pos, self.rate_pos) = (0.0, self.next_rot, 0.0, self.next_pos);
            self.start_pos = self.cap_pos;
            self.start_q = self.cap_q;
        }
        if self.mode != 3 { return cur; }
        if self.t_pos == 1.0 && self.t_rot == 1.0 {
            self.mode = 0;
            return cur;
        }
        let k = cos_interp(0.0, 1.0, self.t_pos);
        self.start_pos = add(self.start_pos, carry);
        let pos = add(self.start_pos, scale(sub(cur.0, self.start_pos), k));
        let r = cos_interp(0.0, 1.0, self.t_rot);
        let q = quat::nlerp(self.start_q, quat::from_rows(&cur.1), r);
        self.t_pos = (self.t_pos + self.rate_pos).min(1.0);
        self.t_rot = (self.t_rot + self.rate_rot).min(1.0);
        (pos, quat::rows(q))
    }
}

/// `0x427990`'s height by the body (`0x18c0d4`): 1.6 Ratchet, 0.75 Clank, 9.5 Giant Clank.
pub fn eye_height(hero: &Hero) -> f32 {
    match hero.mode {
        2 => 9.5,
        1 => 0.75,
        _ => 1.6,
    }
}

/// The eye without the platform carry (`0x427a70`): the feet + his up · [`eye_height`].
fn eye(hero: &Hero) -> V3 { add(hero.pos, scale(set_len(hero.rows[2], 1.0), eye_height(hero))) }

/// The angle between `a` and `b` across `up` (both flattened), as `0x428188` / `0x4285d0` work it out: 90° − asin of
/// the cosine; `None` when one flattens to nothing.
fn flat_angle(a: V3, b: V3, up: V3) -> Option<f32> {
    let flat = |v: V3| sub(v, scale(up, dot(up, v)));
    let (fa, fb) = (flat(a), flat(b));
    let (la, lb) = (len(fa), len(fb));
    if la == 0.0 || la * lb == 0.0 { return None; }
    Some(std::f32::consts::FRAC_PI_2 - (dot(fa, fb) / (la * lb)).clamp(-1.0, 1.0).asin())
}

/// `0x2eef88(out, a, b)` = b × a.
fn cross_ba(a: V3, b: V3) -> V3 { crate::hero::physics::cross(b, a) }

/// The camera classes the activation counts over (`0x428188`: 0 the follow camera, 2, 10, 14).
fn counts_over(class: i16) -> bool { matches!(class, 0 | 2 | 10 | 14) }

impl FirstPerson {
    /// `0x428188` while it is not the current camera (run before the follow camera's update, as the choice
    /// `0x2d8618` is): `best` / `current` the classes the choice compares with. Sets [`FirstPerson::want`]; returns
    /// true when the follow camera must be reset behind Ratchet (`0x2da160`).
    pub fn activation(&mut self, hero: &Hero, follow: &mut FollowCamera, best: i16, current: i16) -> bool {
        self.want = false;
        let facing = [hero.rot[2].cos(), hero.rot[2].sin(), 0.0];
        if hero.cam_mode == CLASS as u8 || hero.cam_mode == 0x51 {
            if counts_over(best) && counts_over(current) {
                self.counting = true;
                self.turn_back = 0;
                self.count += 1;
                if 6 < self.count {
                    self.want = true;
                    return false;
                }
                follow.face_toward(0.069_813_17, facing, 0.0);
                return false;
            }
            self.count = 0;
            self.counting = false;
        } else {
            self.count = 0;
            if self.counting {
                self.counting = false;
                if current == crate::cameras::CLASS_FOLLOW {
                    if flat_angle(follow.rows[0], hero.rows[0], follow.up()).is_some_and(|a| APART <= a) { return true; }
                    self.turn_back = 30;
                }
            }
        }
        if self.turn_back != 0 {
            self.turn_back -= 1;
            follow.face_toward(0.139_626_34, facing, 0.0);
        }
        false
    }

    /// `0x4285d0` while it is current: the release when the camera mode is no longer 4 / 0x51 — 2 (a blend out at
    /// [`RATE_OUT`]), or 4 (a cut, the blend dropped) when a blend runs and the view is [`APART`] off his facing.
    /// `None`: it stays.
    pub fn release(&self, hero: &Hero, up: V3, blend: &mut Blend) -> Option<i16> {
        if hero.cam_mode == CLASS as u8 || hero.cam_mode == 0x51 { return None; }
        if blend.mode != 0 && flat_angle(self.rows[0], hero.rows[0], up).is_some_and(|a| APART <= a) {
            blend.mode = 0;
            return Some(4);
        }
        (blend.next_rot, blend.next_pos) = (RATE_OUT, RATE_OUT);
        Some(2)
    }

    /// `0x428518`: the blend in, the data cleared, the eye and the rows facing the old camera's forward `prev_fwd`.
    pub fn init(&mut self, hero: &Hero, prev_fwd: V3, blend: &mut Blend) {
        // The activation's verdict is taken: the choice asks `0x428188` afresh, so a stale yes must not take the camera
        // back at its release (the hook does not run while it is current).
        self.want = false;
        blend.ask();
        (blend.next_rot, blend.next_pos) = (RATE_IN, RATE_IN);
        self.d = [0.0; 7];
        let up = set_len(hero.rows[2], 1.0);
        self.pos = eye(hero);
        let left = set_len(cross_ba(prev_fwd, up), 1.0);
        self.rows = [cross_ba(up, left), left, up];
    }

    /// `0x427bf8`. `blending`: `0x1b8e90` ≠ 0; `pitch_normal`: the Camera page's option `0x1a7bac`; `blocked(c)`: the
    /// 0.3 sphere at `c` meets the world (`0x2df8d8(0.3, c, 0x12, Ratchet)`). Returns true when it sets the view lock
    /// `0x18c0d5` (the blend over).
    pub fn update(&mut self, hero: &Hero, pad: &Pad, blending: bool, pitch_normal: bool, blocked: &dyn Fn(V3) -> bool) -> bool {
        let up = set_len(hero.rows[2], 1.0);
        self.pos = add(eye(hero), hero.carry);
        let lock = !blending;
        let tune = self.tune;
        let d = &mut self.d;
        let bit = |b: u32| if pad.held & b != 0 { 1.0 } else { 0.0 };
        // The yaw.
        let mut v = -pad.left[0];
        if v == 0.0 { v = -pad.right[0]; }
        if v == 0.0 {
            v = bit(button::LEFT);
            if v == 0.0 { v = -bit(button::RIGHT); }
        }
        let rate = tune.turn * d[0].abs();
        let (mut v0, mut v1) = (d[0], d[1]);
        v0 = interp(v0, v, tune.k, tune.d, 0.0, &mut v1);
        (d[0], d[1]) = (v0, v1);
        let mut fwd = self.rows[0];
        if d[0] != 0.0 { fwd = rot(fwd, rate * d[0], up); }
        if hero.carry_yaw != 0.0 && hero.surf.boots == 0 && !hero.gravity.on { fwd = rot(fwd, hero.carry_yaw, up); }
        // The pitch.
        let mut v = -pad.left[1];
        if v == 0.0 { v = -pad.right[1]; }
        if v == 0.0 { v = bit(button::UP); }
        if v == 0.0 { v = -bit(button::DOWN); }
        if blending && hero.prev_state == 2 { v = 0.0; }
        if !pitch_normal { v = -v; }
        let rate = tune.turn * d[2].abs();
        let (mut v2, mut v3) = (d[2], d[3]);
        v2 = interp(v2, v, tune.k, tune.d, 0.0, &mut v3);
        (d[2], d[3]) = (v2, v3);
        if d[2] != 0.0 {
            let left = cross_ba(fwd, up);
            let mut step = rate * d[2];
            let a = add_rot(d[4], step).abs();
            if 1.117_010_8 < a && d[4].abs() < a { step += -step * sub_rot(a, 1.117_010_7) * 2.864_788_5; }
            let push = if a < 1.115_265_5 || d[4] <= 0.0 { 0.0 } else { 0.5 };
            let (mut p, mut pv) = (d[5], d[6]);
            p = interp(p, push, 0.005, 0.2, 0.0, &mut pv);
            (d[5], d[6]) = (p, pv);
            let flat = set_len([fwd[0], fwd[1], 0.0], d[5]);
            self.pos = add(self.pos, flat);
            if blocked(self.pos) {
                let (mut p, mut pv) = (d[5], d[6]);
                p = interp(p, 0.0, 0.005, 0.2, 0.0, &mut pv);
                (d[5], d[6]) = (p, pv);
            }
            let a2 = add_rot(d[4], step);
            if 1.466_076_7 < a2.abs() { step = sub_rot(if a2 <= 0.0 { -1.466_076_7 } else { 1.466_076_7 }, d[4]); }
            fwd = rot(fwd, step, left);
            d[4] = add_rot(d[4], step);
        }
        let fwd = set_len(fwd, 1.0);
        let left = set_len(cross_ba(fwd, up), 1.0);
        self.rows = [fwd, left, cross_ba(left, fwd)];
        lock
    }
}
