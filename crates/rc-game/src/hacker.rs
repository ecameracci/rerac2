//! The Hacker's puzzle: the game's mode 7 as the hacking panels (crate::moby::classes::hacker_panel, 0x0b29) open it,
//! ported from Endako's copy (L03 `0x3e37c0..0x3e5c90`; the same unit on levels 3, 11, 12, 14, 17, 19 and 20). The
//! panel names itself the mode's moby and makes the board, a sphere moby **0x0b28** (`0x3e6680`), whose update
//! `0x3e37c0` the panel runs while the mode lasts ([`tick`]); the game's mode 7 runs nothing else (crate docs of
//! crate::moby::classes::electrolyzer).
//!
//! **The board.** 14 nodes on a unit sphere (`0x29bc70`, normalized at the start: the cube's 6 face centres and 8
//! corners, node 0 the top face's centre), each joined to the nodes within 60° of it (a corner to those within 95°:
//! `dot` above 0.5, or −0.1 from a corner; the table `0x24a7b0`, 6 slots, −1 for none). The panel's list (its pvars
//! `+0xc..`, 14 at most, −1 ends it: `0x29bd50`, the count `$gp+0x1b20`) gives the path's length and, step by step,
//! its kind; its seed `+0x44` seeds the game's random stream (`srand`), so a panel's path is always the same.
//! * The path (`0x3e3f78`): node 0, then two corners of the top face side by side (one of four pairs), then nodes
//!   put in between two path nodes side by side (from a random place on: the first free node joined to both), until
//!   the list's length (or no node fits: the length is cut there). Its last node is joined to node 0: the path is a
//!   loop, and the board is solved back at node 0.
//! * The false turns (`0x3e4218`, [`Board::decoys`]): at each step, by its kind, 1: one node joined to the step's
//!   node, off the path so far; 2: a chain of two (the second joined to the first); 3: two side by side from the
//!   step's node (`0x3e43a8`: a random first slot of six, the next free one).
//!
//! **Play** (`0x3e37c0`). The sphere turns so the node Ratchet is on faces the camera, its first neighbour upward on
//! screen kept per depth (`0x3e4628`, [`Board::orient`]), eased there (`0x3e4c88`). The left stick picks among the
//! ways on (`0x3e4e10`: within 90° of the stick on screen): the path's next node, the step's false turns
//! (`0x3e4d60`) or back the way he came; a pick moves there (`0x3e44d8`) and starts the clock. Every new depth adds
//! `+0x4c` ticks to the clock (at most the start `+0x48` + 120). With the clock out, the board winds back to the
//! start node by node and the clock is reset. Back at node 0 with the whole loop walked: solved (sound 5), two
//! seconds on, the panel told (`0x3e6828`), the mode left (`0x32de20`). △ leaves at any time (solved if it was).
//!
//! **Its draw** (`0x3e5010`, registered each tick): the walked trail as arcs over the sphere (FX 14, `0x3e5758`:
//! 0.475 out, a ribbon 0.03 wide, a segment each 5°, its texture scrolling at 1.5 a second, a spark zig-zagging
//! along it), the ways on as short green arcs 20° out toward the node; on the screen the frame's GUI sprites 0xcb /
//! 0xcc, the clock's bar (25..38 across, up to 286 high, flashing), the depth in `n` boxes (473..487 across) and four
//! labels in font 2 ([`draw_hud`]). The board's glow pulses by the clock (`0x3e5b98`).
//!
//! The mode's view (the panel, state 5 / 6): the camera at the world's origin facing +x (`0x1b9980` zeroed,
//! `0x1b9bb0` the identity) with the field of view 0x3f8fea69 (crate::moby::classes::electrolyzer::tan), the board
//! 1.2 in front of it (`0x3e66f0`) — [`HackerState::view`] for the engine.
//!
//! Not ported: the sound's pitch by the depth on a move (`0x3621e8(h, depth/n · 1024 + 0x400)`: the port's sounds have
//! no pitch control), the owner check of the clock's ticking loop (the slot's moby `0x1886e8`), the help voice line
//! on level 11 (`0x2f6270(0xa43, 0x9a)` at the board's start, once in 1081 ticks: no help voice in the port) and its
//! word `0x18d090` on the way out, the word `0x1b2768` = 1 each tick, the PAL offsets of the labels (`0x1b3280`) and
//! the screen's y scale (`0x1b3284`: 1 here), the half-pixel offset of the boxes (`0x2fd380`'s −0x10 against the GUI's −8),
//! the reads past the end of the tables when a false turn has no node (a chain's second from −1 reads before
//! `0x24a7b0`; here none).

use crate::fx::FxQuad;
use crate::hero::physics::{add, add_rot, approach, atan2, cross, dot, ease, len, rot_dist, scale, set_len, sub, sub_rot, V3};
use crate::moby::{mode, Ctx, MobyWorld};
use crate::pad::button;
use crate::quat::{self, Q};

/// The game's mode of the puzzle (`0x1a8f00`).
pub const MODE: i32 = 7;
/// The board, the sphere (`0x3e6680`'s `0x312738(0xb28)`).
pub const BOARD: i16 = 0x0b28;
/// The list's length at most (`0x29bd50`'s 14 records).
pub const STEPS: usize = 14;
const NODES: usize = 14;

/// `0x29bc70`: the nodes (normalized by the board's start).
const NODE_RAW: [V3; NODES] = [
    [0.0, 0.0, 1.0],
    [1.0, 1.0, 1.0],
    [-1.0, 1.0, 1.0],
    [-1.0, -1.0, 1.0],
    [1.0, -1.0, 1.0],
    [0.0, 1.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, -1.0, 0.0],
    [-1.0, 0.0, 0.0],
    [1.0, 1.0, -1.0],
    [-1.0, 1.0, -1.0],
    [-1.0, -1.0, -1.0],
    [1.0, -1.0, -1.0],
    [0.0, 0.0, -1.0],
];

/// The mode's camera rows (`0x1b9bb0`, the identity: forward +x, left +y, up +z).
pub const VIEW_ROWS: [V3; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
const FWD: V3 = VIEW_ROWS[0];
const LEFT: V3 = VIEW_ROWS[1];
const UP: V3 = VIEW_ROWS[2];
/// `$gp+0x1c70`: the board's distance in front of the camera.
pub const BOARD_DIST: f32 = 1.2;

/// `$gp+0x1b48` / `+0x1b4c` (per second², `/3600`) and `+0x1b50` (per second, `/60`): the turn's ease.
const EASE_ACCEL: f32 = 24.0 * 0.000_277_777_78;
const EASE_BRAKE: f32 = 24.0 * 0.000_277_777_78;
const EASE_MAX: f32 = 8.0 * 0.016_666_668;
/// `$gp+0x1b44`: the arcs' texture scroll a second.
const SCROLL: f32 = 1.5;

/// Node `i` on the unit sphere (`0x2f00f8(1, v, v)` at the start); a bad index none.
pub fn node(i: i32) -> V3 { usize::try_from(i).ok().and_then(|i| NODE_RAW.get(i)).map_or([0.0; 3], |&v| set_len(v, 1.0)) }

/// `0x3e3cd8`'s table `0x24a7b0`: each node's neighbours in order (6 slots, −1 after the last).
pub fn neighbours() -> [[i32; 6]; NODES] {
    let mut t = [[-1; 6]; NODES];
    for (i, row) in t.iter_mut().enumerate() {
        let a = node(i as i32);
        let corner = a[0] != 0.0 && a[1] != 0.0 && a[2] != 0.0;
        let floor = if corner { -0.1 } else { 0.5 };
        let mut k = 0;
        for j in 0..NODES {
            if i != j && floor < dot(a, node(j as i32)) && k < 6 {
                row[k] = j as i32;
                k += 1;
            }
        }
    }
    t
}

/// `0x2f02d8(out, v, rows)`: `v` through the rows (no translation).
fn through(rows: &[V3; 3], v: V3) -> V3 { std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]) }

/// `0x2eff28(out, a, b)`: the VU's outer product as the level code calls it, `b × a`.
fn outer(a: V3, b: V3) -> V3 { cross(b, a) }

/// The quaternion of `axis` (any length) and the angle `a`: xyz `axis` at length sin(a/2), w cos(a/2).
fn turn(axis: V3, a: f32) -> Q {
    let v = set_len(axis, (a * 0.5).sin());
    [v[0], v[1], v[2], (a * 0.5).cos()]
}

/// The left stick as the puzzle reads it (`0x1382c8` / `0x1382cc`: the record's un-mirrored copy `+0x148` / `+0x14c`).
pub fn stick(pad: &crate::pad::PadState) -> [f32; 2] { [pad.analog_copy[2].to_f32(), pad.analog_copy[3].to_f32()] }

/// The board's update state (its pvars, `0x3e3cd8`'s layout) and the tables it reads (`0x29bd50`, `$gp+0x1b20..`).
#[derive(Clone, Debug)]
pub struct Board {
    /// The board's moby (0x0b28) and the panel that made it (`0x1ac0c0`, the mode's moby).
    pub moby: usize,
    pub panel: usize,
    /// `+0x20` / `+0x96` and the first tick of a state (`0x32bec8`).
    pub state: u8,
    pub ticks: i16,
    first: bool,
    nb: [[i32; 6]; NODES],
    /// `$gp+0x1b20`: the path's length.
    pub n: usize,
    /// `0x29bd50`: per step its kind (the panel's list) and its false turns.
    pub steps: [[i32; 4]; STEPS],
    /// `$gp+0x1b2c` the clock's start, `$gp+0x1b28` its most, `$gp+0x1b30` a new depth's bonus (ticks).
    pub start: i16,
    pub max: i16,
    pub bonus: i16,
    /// `+0x6c`: the seed.
    pub seed: i16,
    /// `+0x00`: the nodes walked (0xff after the last); `+0x10`: the walked flags by node.
    trail: [u8; 16],
    pub visited: [u8; 16],
    /// `+0x20`: the path (the byte after the last 0: node 0, the loop's end).
    pub path: [u8; 16],
    /// `+0x30`: per depth the neighbour kept upward on screen (0xff: not chosen yet).
    refs: [u8; 16],
    /// `+0x40`: the ways on (0xff ends them).
    cands: [u8; 8],
    /// `+0x48` / `+0x4c`: the turn's speed and its share done.
    vel: f32,
    t: f32,
    /// `+0x50` the node he is on, `+0x52` the pick, `+0x56` the turn done, `+0x58` the arcs' scroll, `+0x5c` the node
    /// before, `+0x5e` the depth, `+0x60` the deepest so far, `+0x62` the path's steps walked, `+0x64` the steps off it,
    /// `+0x66` a pick kept for later (−1 none), `+0x68` the clock, `+0x6a` the clock running.
    pub cur: i16,
    pub cursor: i16,
    done: bool,
    scroll: f32,
    pub prev: i16,
    pub depth: i16,
    max_depth: i16,
    pub progress: i16,
    pub off: i16,
    pending: i16,
    pub timer: i16,
    pub running: bool,
    /// `+0x78`: the glow's phase; `+0x7c`: the clock's ticking sound (−1 none).
    phase: f32,
    tick_sound: i32,
    /// `0x1b36b0` / `0x1b36c0`: the sphere's turn now and the one it eases to.
    q: Q,
    target: Q,
    /// The board's rows (its `+0xc0`).
    pub rows: [V3; 3],
}

/// What the screen's draw reads (`0x3e5010`'s HUD part).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hud {
    pub timer: i16,
    pub max: i16,
    pub depth: i16,
    pub n: usize,
    /// The game's frame (the bar's flash).
    pub frame: i32,
}

/// The puzzle's part of the game state.
#[derive(Clone, Debug, Default)]
pub struct HackerState {
    /// The board while the mode runs.
    pub board: Option<Board>,
    /// The mode's camera (eye, rows) while a panel holds the mode (`0x1b9980`, `0x1b9bb0`).
    pub view: Option<(V3, [V3; 3])>,
    /// The left stick for the board (set by the engine's mode-7 frame; [`stick`]).
    pub stick: [f32; 2],
    /// `$gp+0x3474`: the white over the screen (0..1; the frame draw `0x2e7250` fills it at alpha × 128).
    pub white: f32,
    /// The field of view's ease (`0x2d8630` mode 1 / `0x2d8740`): ticks left, ticks, start, target.
    pub zoom: Option<(i16, i16, f32, f32)>,
    /// This tick's arcs (FX quads, the frame's second list).
    pub quads: Vec<FxQuad>,
    /// The screen's draw this tick.
    pub hud: Option<Hud>,
}

impl HackerState {
    /// `0x2d8740`'s mode 1 step: the ticks down, the field of view `target + (start − target)·left/ticks`; the view's
    /// tangent of half of it (crate::moby::classes::electrolyzer::PuzzleState::tan carries it to the frame).
    pub fn zoom_step(&mut self) -> Option<f32> {
        let (left, n, start, target) = self.zoom.as_mut()?;
        if *left != 0 { *left = (*left).max(1) - 1; }
        let fov = *target + (*start - *target) * *left as f32 * (1.0 / *n as f32);
        if *left == 0 { self.zoom = None; }
        Some((fov * 0.5).sin() / (fov * 0.5).cos())
    }
}

impl Board {
    /// The panel's state 5 (L03 `0x3e5c90`): the board made, its tables written; it starts on its first tick.
    pub fn new(moby: usize, panel: usize, kinds: &[i32], seed: i16, start: i16, bonus: i16) -> Board {
        let mut steps = [[-1; 4]; STEPS];
        for (s, &k) in steps.iter_mut().zip(kinds) { s[0] = k; }
        Board {
            moby, panel, state: 0, ticks: 0, first: true, nb: neighbours(), n: kinds.len().min(STEPS), steps, start, max: start.wrapping_add(0x78), bonus, seed,
            trail: [0; 16], visited: [0; 16], path: [0; 16], refs: [0; 16], cands: [0; 8], vel: 0.0, t: 0.0, cur: 0, cursor: 0, done: false, scroll: 0.0,
            prev: 0, depth: 0, max_depth: 0, progress: 0, off: 0, pending: 0, timer: 0, running: false, phase: 0.0, tick_sound: -1,
            q: quat::IDENTITY, target: quat::IDENTITY, rows: VIEW_ROWS,
        }
    }

    fn set_state(&mut self, s: u8) {
        self.state = s;
        self.ticks = 0;
        self.first = true;
    }

    fn nbs(&self, n: i32) -> [i32; 6] { usize::try_from(n).ok().and_then(|i| self.nb.get(i)).copied().unwrap_or([-1; 6]) }

    /// `path[i]` (the byte after the path's last is node 0).
    fn path_at(&self, i: i32) -> i32 { usize::try_from(i).ok().and_then(|i| self.path.get(i)).map_or(0, |&b| b as i32) }

    fn step(&self, i: i32) -> [i32; 4] { usize::try_from(i).ok().and_then(|i| self.steps.get(i)).copied().unwrap_or([0, -1, -1, -1]) }

    /// `0x3e3cd8`: the start (the sphere's turn from its Euler angles, the seed, the path, its false turns, the ways
    /// on, the pick; the trail at node 0; the clock at its start).
    pub fn init(&mut self, rng: &mut crate::rng::Rng, rows: [V3; 3]) {
        self.refs = [0xff; 16];
        self.refs[15] = 0;
        self.rows = rows;
        // `0x3214a8` (the rows' Euler angles) then `0x31eca0` (their quaternion).
        self.q = quat::from_rows(&rows);
        rng.srand(self.seed as i32 as u32);
        self.make_path(rng);
        self.decoys(rng);
        self.ways_on();
        self.default_pick();
        self.trail = [0xff; 16];
        self.visited = [0; 16];
        self.trail[0] = 0;
        self.visited[0] = 1;
        self.pending = -1;
        self.depth = 0;
        self.timer = self.start;
        self.cur = 0;
        self.prev = 0;
        self.cursor = 0;
        self.progress = 0;
        self.off = 0;
    }

    /// `0x3e3f78`: the path (module docs).
    fn make_path(&mut self, rng: &mut crate::rng::Rng) {
        self.path[0] = 0;
        (self.path[1], self.path[2]) = match rng.randi(4) {
            0 => (1, 2),
            1 => (2, 3),
            2 => (3, 4),
            _ => (4, 1),
        };
        let mut len = 3usize;
        loop {
            if self.n <= len { return; }
            let r = rng.randi(len as i32) as usize;
            let mut l = len;
            let mut k = 0;
            'tries: while k < l {
                let (i, j) = ((r + k) % l, (r + k + 1) % l);
                for &u in self.nbs(self.path[i] as i32).iter() {
                    if u == -1 { break; }
                    if self.path[..l].contains(&(u as u8)) { continue; }
                    if self.nbs(u).contains(&(self.path[j] as i32)) {
                        for s in (i + 1..l).rev() { self.path[s + 1] = self.path[s]; }
                        self.path[i + 1] = u as u8;
                        l += 1;
                        break 'tries;
                    }
                }
                k += 1;
            }
            if l == len {
                self.n = l;
                return;
            }
            len = l;
        }
    }

    /// `0x3e43a8(p, i, excl)`: a neighbour of `p` off the path's first `i + 2` and not in `excl`, from a random slot on.
    fn pick(&self, rng: &mut crate::rng::Rng, p: i32, i: usize, excl: &[i32]) -> i32 {
        let r = rng.randi(6) as usize;
        let row = self.nbs(p);
        for k in 0..6 {
            let u = row[(r + k) % 6];
            if u == -1 { continue; }
            let on_path = self.path[..(i + 2).min(16)].iter().any(|&b| b as i32 == u);
            if !on_path && !excl.contains(&u) { return u; }
        }
        -1
    }

    /// `0x3e4218`: each step's false turns by its kind (module docs).
    fn decoys(&mut self, rng: &mut crate::rng::Rng) {
        for i in 0..self.n {
            self.steps[i][1..].fill(-1);
            let p = self.path[i] as i32;
            match self.steps[i][0] {
                1 => self.steps[i][1] = self.pick(rng, p, i, &[]),
                2 => {
                    let a = self.pick(rng, p, i, &[]);
                    self.steps[i][1] = a;
                    self.steps[i][2] = if a == -1 { -1 } else { self.pick(rng, a, i, &[a]) };
                }
                3 => {
                    let a = self.pick(rng, p, i, &[]);
                    self.steps[i][1] = a;
                    self.steps[i][2] = self.pick(rng, p, i, &[a]);
                }
                _ => {}
            }
        }
    }

    /// `0x3e4d60(u)`: `u` is a false turn from here (not at the path's last node; kinds 1 / 2 the chain's next, kind 3
    /// either of the pair while still on the path).
    fn false_turn(&self, u: i32) -> bool {
        if self.depth as i32 == self.n as i32 - 1 { return false; }
        let s = self.step(self.progress as i32);
        match s[0] {
            1 | 2 => s.get(self.off as usize + 1).is_some_and(|&v| v == u),
            3 => self.off == 0 && (u == s[1] || u == s[2]),
            _ => false,
        }
    }

    /// `0x3e4918`: the ways on from the node he is on (its neighbours that are the path's next or a false turn).
    fn ways_on(&mut self) {
        let row = self.nbs(self.cur as i32);
        let mut c = 0;
        for &u in &row {
            let ok = (self.off == 0 && u == self.path_at(self.progress as i32 + 1)) || self.false_turn(u);
            if ok {
                self.cands[c] = u as u8;
                c += 1;
            }
        }
        self.cands[c] = 0xff;
    }

    fn ways(&self, most: usize) -> impl Iterator<Item = u8> + '_ { self.cands.iter().copied().take(most).take_while(|&c| c != 0xff) }

    /// A node's place around the screen's centre, from left toward up (`0x2f04d0(left, up)`).
    fn screen_angle(&self, n: i32) -> f32 {
        let v = through(&self.rows, node(n));
        atan2(dot(v, LEFT), dot(v, UP))
    }

    /// `0x3e4a40`: the pick by default: the only way on, else the next one around from the pick (clockwise on screen).
    fn default_pick(&mut self) {
        let n = self.ways(8).count();
        match n {
            0 => self.cursor = self.cur,
            1 => self.cursor = self.cands[0] as i16,
            _ => {
                let a0 = self.screen_angle(self.cursor as i32);
                let mut best = -10.0f32;
                let mut pick = -1;
                for c in self.ways(n) {
                    if c as i16 == self.cursor { continue; }
                    let mut d = sub_rot(self.screen_angle(c as i32), a0);
                    if 0.0 < d { d -= std::f32::consts::TAU; }
                    if best < d {
                        pick = c as i32;
                        best = d;
                    }
                }
                if pick != -1 { self.cursor = pick as i16; }
            }
        }
    }

    /// `0x3e4e10`: with the stick out (0.9 of its square), the way on (or back) nearest its direction on screen within
    /// 90°: the pick; whether there was one.
    fn stick_pick(&mut self, s: [f32; 2]) -> bool {
        let (x, y) = (-s[0], -s[1]);
        if x * x + y * y < 0.9 { return false; }
        let a = atan2(x, y);
        let mut best = f32::from_bits(0x3fc8_f5c3);
        let mut pick = -1i32;
        for c in self.ways(8) {
            let d = rot_dist(self.screen_angle(c as i32), a);
            if d < best {
                best = d;
                pick = c as i32;
            }
        }
        if self.prev != self.cur && rot_dist(self.screen_angle(self.prev as i32), a) < best { pick = self.prev as i32; }
        if pick == -1 { return false; }
        self.cursor = pick as i16;
        true
    }

    /// `0x3e44d8`: a move to the pick: back to the node before (the trail shortened), or on (onto the path or off
    /// it); then the ways on, the turn restarted and eased to the new node ([`Self::orient`]).
    fn advance(&mut self) {
        if self.cursor == self.prev {
            self.visited[(self.cur as usize) & 15] = 0;
            self.trail[(self.depth as usize) & 15] = 0xff;
            self.cur = self.prev;
            self.prev = if self.depth < 2 { 0 } else { self.trail[(self.depth as usize - 2) & 15] as i16 };
            self.refs[(self.depth as usize) & 15] = 0xff;
            self.depth -= 1;
            if self.off == 0 { self.progress -= 1; } else { self.off -= 1; }
        } else {
            self.visited[(self.cursor as usize) & 15] = 1;
            if self.off == 0 && self.cursor as i32 == self.path_at(self.progress as i32 + 1) { self.progress += 1; } else { self.off += 1; }
            self.prev = self.cur;
            self.cur = self.cursor;
            self.depth += 1;
            self.trail[(self.depth as usize) & 15] = self.cursor as u8;
        }
        self.ways_on();
        self.done = false;
        self.vel = 0.0;
        self.t = 0.0;
        self.orient();
    }

    /// `0x3e4628`: the turn to ease to: the node he is on turned to face the camera, then about the view's axis so the
    /// depth's kept neighbour (the highest on screen, chosen once a depth) points up; the rows back at the turn now.
    fn orient(&mut self) {
        let c = scale(FWD, -1.0);
        let b = through(&self.rows, node(self.cur as i32));
        let a = outer(b, c);
        let th = atan2(dot(b, c), len(a));
        let q1 = turn(a, th);
        let rows1 = quat::turn_rows(&self.rows, q1);
        let d = (self.depth as usize) & 15;
        if self.refs[d] == 0xff {
            let mut best = -1.0f32;
            for &u in &self.nbs(self.cur as i32) {
                if u == -1 { continue; }
                let f = dot(through(&rows1, node(u)), UP);
                if best < f {
                    self.refs[d] = u as u8;
                    best = f;
                }
            }
        }
        let v = through(&rows1, node(self.refs[d] as i32));
        let roll = atan2(dot(v, UP), dot(v, LEFT));
        let q2 = turn(c, roll);
        self.target = quat::mul(quat::mul(self.q, q1), q2);
        self.rows = quat::rows(self.q);
    }

    /// `0x3e4c88`: the turn eased on (`0x320220(1, …)`), the rows at the blend of the two (`0x2f0ad8`); at the end
    /// the turn is the target.
    fn ease_turn(&mut self) {
        ease(1.0, EASE_ACCEL, EASE_BRAKE, EASE_MAX, &mut self.t, &mut self.vel);
        self.rows = quat::rows(quat::nlerp(self.q, self.target, self.t));
        if 1.0 <= self.t {
            self.done = true;
            self.q = self.target;
        }
    }
}

/// `RC_HACKER` (crate::moby::classes::hacker_panel): 1 the first panel opened at once, the board's path and its ways
/// on printed; 2 also walked by itself (each pick the path's next node); 3 printed only.
pub fn dev() -> u8 {
    static ON: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("RC_HACKER").ok().and_then(|v| v.trim().parse().ok()).unwrap_or(0))
}

impl Board {
    /// `RC_HACKER=2`: the pick the path's next node (back first when off it).
    fn auto_pick(&mut self) -> bool {
        self.cursor = if self.off == 0 { self.path_at(self.progress as i32 + 1) as i16 } else { self.prev };
        true
    }
}

/// How the board's tick ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Playing,
    /// The mode left (`0x32de20(0, 0)`) and the board deleted; solved or not.
    Left { solved: bool },
}

/// `0x3e37c0`: the board's tick in the game's mode 7 (module docs), then its glow (`0x3e5b98`), its scroll and its
/// draw (`0x3e5010`, [`draw`]). The panel calls it with the board placed (`0x3e66f0`).
pub fn tick(w: &mut MobyWorld, ctx: &mut Ctx) -> Outcome {
    let Some(mut b) = ctx.game.hacker.board.take() else { return Outcome::Playing };
    ctx.game.hacker.hud = None;
    approach(0.0, f32::from_bits(0x3e2a_aaab), &mut ctx.game.hacker.white);
    let first = std::mem::replace(&mut b.first, false);
    let sound = |w: &MobyWorld, ctx: &mut Ctx, b: &Board, i: i32, mode: i32| w.mobys.get(b.moby).map_or(-1, |m| ctx.host.play_sound(m, i, mode));
    match b.state {
        0 => {
            for s in 0..0x34 { ctx.host.stop_sound(s); }
            let rows = w.mobys.get(b.moby).map_or(VIEW_ROWS, |m| m.rows);
            b.init(&mut ctx.game.rng, rows);
            if let Some(m) = w.mobys.get_mut(b.moby) { m.mode |= mode::KEEP_ROWS; }
            b.orient();
            // Level 11 first asks for its help line (state 1; module docs).
            b.set_state(if ctx.game.level == 0xb { 1 } else { 3 });
            if dev() != 0 { println!("hacker: board: path {:?} steps {:?} clock {} (+{} a depth, at most {})", &b.path[..b.n], &b.steps[..b.n], b.timer, b.bonus, b.max); }
        }
        1 => b.set_state(3),
        2 => {
            if b.timer == 0 {
                b.set_state(4);
            } else if (dev() == 2 && b.auto_pick()) || b.stick_pick(ctx.game.hacker.stick) {
                b.running = true;
                b.advance();
                sound(w, ctx, &b, 4, 0);
                b.set_state(3);
            }
        }
        3 => {
            b.ease_turn();
            if b.done {
                if b.pending == -1 { b.default_pick(); } else { b.cursor = b.pending; b.pending = -1; }
                if b.cands[0] == 0xff { sound(w, ctx, &b, 0, 0); } else { sound(w, ctx, &b, 1, 0); }
                if b.max_depth < b.depth {
                    let t = b.timer.wrapping_add(b.bonus);
                    b.max_depth = b.depth;
                    b.timer = if b.max < t { b.max } else { t };
                }
                if b.depth as i32 == b.n as i32 {
                    sound(w, ctx, &b, 5, 0);
                    b.set_state(5);
                } else {
                    b.set_state(2);
                    if dev() != 0 {
                        let ways: Vec<String> = b.ways(8).map(|c| c as i32).chain((b.prev != b.cur).then_some(b.prev as i32)).map(|c| {
                            let a = b.screen_angle(c);
                            format!("{c} stick ({:.2}, {:.2})", -a.cos(), -a.sin())
                        }).collect();
                        println!("hacker: frame {}: on node {} depth {} (path step {}, off {}), clock {}; ways: {}", ctx.game.frame, b.cur, b.depth, b.progress, b.off, b.timer, ways.join(", "));
                    }
                }
            }
        }
        4 => {
            if !b.done {
                b.ease_turn();
            } else if b.prev == b.cur {
                b.running = false;
                b.max_depth = 0;
                b.timer = b.start;
                b.set_state(2);
            } else {
                b.cursor = b.prev;
                b.advance();
                sound(w, ctx, &b, 2, 0);
            }
        }
        5 => {
            if first { b.ticks = 0x78; }
            if b.ticks == 0 {
                leave(w, ctx, &b, true);
                return Outcome::Left { solved: true };
            }
            b.ticks -= 1;
        }
        _ => {}
    }
    if ctx.hero.items.pressed & button::TRIANGLE != 0 {
        let solved = b.state == 5;
        leave(w, ctx, &b, solved);
        return Outcome::Left { solved };
    }
    // The clock: down while it runs (not once solved); its ticking loops while it has time.
    if b.state != 5 && b.running && b.timer != 0 { b.timer = b.timer.max(1) - 1; }
    if b.running && b.timer != 0 {
        if b.tick_sound < 0 || !ctx.host.sound_playing(b.tick_sound) { b.tick_sound = sound(w, ctx, &b, 3, 4); }
    } else {
        if 0 <= b.tick_sound && ctx.host.sound_playing(b.tick_sound) { ctx.host.stop_sound(b.tick_sound); }
        b.tick_sound = -1;
    }
    glow(w, &mut b);
    b.scroll -= SCROLL * 0.016_666_668;
    if b.scroll < -1.0 { b.scroll += 1.0; }
    if let Some(m) = w.mobys.get_mut(b.moby) {
        m.rows = b.rows;
        w.rebuild(b.moby, &*ctx.host);
        w.changed = true;
    }
    let pos = w.mobys.get(b.moby).map_or([BOARD_DIST, 0.0, 0.0], |m| m.pos);
    let mut quads = std::mem::take(&mut ctx.game.hacker.quads);
    quads.clear();
    draw(&b, pos, &mut ctx.game.rng, &mut quads);
    ctx.game.hacker.quads = quads;
    ctx.game.hacker.hud = Some(Hud { timer: b.timer, max: b.max, depth: b.depth, n: b.n, frame: ctx.game.frame });
    ctx.game.hacker.board = Some(b);
    Outcome::Playing
}

/// The way out of the board: the panel told when solved (`0x3e6828`: its `+0x60`), the mode popped, the board
/// deleted (`0x312aa0`).
fn leave(w: &mut MobyWorld, ctx: &mut Ctx, b: &Board, solved: bool) {
    if solved {
        if let Some(p) = w.mobys.get_mut(b.panel) { p.set_pv_i(crate::moby::classes::hacker_panel::pv::SOLVED, 1); }
    }
    ctx.game.scenes.pop(ctx.hero.hp);
    if dev() != 0 { println!("hacker: frame {}: the board left, solved {solved}", ctx.game.frame); }
    if w.mobys.get(b.moby).is_some_and(|m| m.state < 0x80 && m.o_class == BOARD) { w.delete(b.moby); }
    w.changed = true;
    let h = &mut ctx.game.hacker;
    h.board = None;
    h.hud = None;
    h.quads.clear();
}

/// `0x3e5b98`: the board's glow between a pair by its phase (solved or the clock stopped: 0x80a08000 / 0x80804000
/// slowly, 300 × π/10800 a tick; out of time 0x800000c0 / 0x80000000, else 0x800038c0 / 0x800000c0, 900).
fn glow(w: &mut MobyWorld, b: &mut Board) {
    let (a, c, speed) = if b.state == 5 || !b.running {
        (0x80a0_8000, 0x8080_4000, 300.0)
    } else if b.timer == 0 {
        (0x8000_00c0, 0x8000_0000, 900.0)
    } else {
        (0x8000_38c0, 0x8000_00c0, 900.0)
    };
    b.phase = add_rot(b.phase, speed * 0.000_290_888_25);
    if let Some(m) = w.mobys.get_mut(b.moby) { m.glow = crate::hud::lerp_rgba(b.phase.sin() * 0.5 + 0.5, a, c); }
}

/// `0x3e5758(scroll, board, a, b, from, to, spark)`: an arc over the sphere from `from` to `to` (board space, unit):
/// a ribbon 0.475 out and 0.03 wide (FX 14, blended), a segment each 5° coloured from `a` to `b` along it, its
/// texture's s shifted by `scroll`; with `spark` ≠ 0 a thin quad of that colour zig-zagging inside it (each
/// segment's far edge between 0.3..0.9 and 0.2 less of the way across).
#[allow(clippy::too_many_arguments)]
fn arc(out: &mut Vec<FxQuad>, rng: &mut crate::rng::Rng, rows: &[V3; 3], pos: V3, scroll: f32, a: u32, b: u32, from: V3, to: V3, spark: u32) {
    let world = |v: V3| add(through(rows, v), pos);
    let axis = outer(from, to);
    let ang = atan2(dot(from, to), len(axis));
    let mut n = (ang * 11.459_154) as i32;
    if n <= 0 { n = 1; }
    let step = turn(axis, ang / n as f32);
    let side = set_len(axis, 0.015);
    let st = [[scroll, 0.0], [scroll, 1.0], [1.0 + scroll, 0.0], [1.0 + scroll, 1.0]];
    let base = set_len(from, 0.475);
    let mut v = [sub(base, side), add(base, side), sub(base, side), add(base, side)];
    v[2] = quat::rotate(step, v[2]);
    v[3] = quat::rotate(step, v[3]);
    let mut sv = v;
    let mut f = 0.0f32;
    let k = 1.0 / n as f32;
    for _ in 0..n {
        let f2 = f + k;
        let (c0, c1) = (crate::hud::lerp_rgba(f, a, b), crate::hud::lerp_rgba(f2, a, b));
        out.push(FxQuad { fx: 0xe, additive: false, corners: v.map(world), st, rgba: [c0, c0, c1, c1] });
        if spark != 0 {
            let r = rng.randf(f32::from_bits(0x3e99_999a), f32::from_bits(0x3f66_6666));
            sv[2] = add(v[2], scale(sub(v[3], v[2]), r));
            sv[3] = add(v[2], scale(sub(v[3], v[2]), r - 0.2));
            out.push(FxQuad { fx: 0xe, additive: false, corners: sv.map(world), st, rgba: [spark; 4] });
            (sv[0], sv[1]) = (sv[2], sv[3]);
        }
        f = f2;
        v = v.map(|p| quat::rotate(step, p));
    }
}

/// `0x3e5010`'s arcs: the trail walked (0x60ff5000, 0x60f08000 with the loop closed; sparks 0x60ffc0c0) and, from
/// the node he is on, each way on (not the way back; with the loop closed not the path's first step again) as a
/// green arc from 20° out toward the node (0x0000ff00 to 0x6000ff00, scrolling the other way).
fn draw(b: &Board, pos: V3, rng: &mut crate::rng::Rng, out: &mut Vec<FxQuad>) {
    let closed = b.progress as i32 == b.n as i32;
    let c = if closed { 0x60f0_8000 } else { 0x60ff_5000 };
    let mut k = 1usize;
    while k < 16 && b.trail[k] != 0xff {
        arc(out, rng, &b.rows, pos, b.scroll, c, c, node(b.trail[k - 1] as i32), node(b.trail[k] as i32), 0x60ff_c0c0);
        if 14 < k { break; }
        k += 1;
    }
    let cur = node(b.cur as i32);
    for &u in &b.nbs(b.cur as i32) {
        if u == b.prev as i32 || u == -1 { continue; }
        if closed && u == b.path[1] as i32 { continue; }
        if !b.ways(6).any(|c| c as i32 == u) { continue; }
        // `0x323060(sin 10°, axis)`: 20° from the node toward the way.
        let q = quat::axis_quat(f32::from_bits(0x3e32_b8c4), outer(cur, node(u)));
        let p = set_len(quat::rotate(q, cur), 1.0);
        arc(out, rng, &b.rows, pos, -b.scroll, 0x0000_ff00, 0x6000_ff00, p, cur, 0);
    }
}

/// `0x3e5010`'s screen part: the frame's GUI sprites 0xcb (0x60442d00) and 0xcc (0x55f0c070) at (254, 210); the clock's
/// bar 25..38 across from 362 up by 286 × time / most, flashing between 0x80f0c070 and 0x40f0c070 a second; the
/// depth in `n` boxes 473..487 across stacked from 362 up to 82 (reached: filled 0x80f0c070; else an outline
/// 0x55f0c070 round a 0x60442d00 box); the labels 0x2be5 at (400, 25), 0x2df9 at (115, 380), 0x3077 at (120, 25) and
/// 0x2c53 at (395, 380), centred, in font 2 in 0x80808080 (`0x2eb200`). `font`: font 2's glyphs and texture slot.
pub fn draw_hud(h: &Hud, text: &dyn Fn(u32) -> Vec<u8>, font: (&[rc_formats::disc::boot::Glyph; 256], usize), draws: &mut Vec<crate::hud::Draw>, out: &mut Vec<crate::gui::Quad>) {
    use crate::gui::{rect, rgba};
    for (id, colour) in [(0xcb, 0x6044_2d00), (0xcc, 0x55f0_c070)] {
        draws.push(crate::hud::Draw::Sprite { id, pos: [254.0, 210.0], scale: [1.0, 1.0], frame: 0.0, colour });
    }
    let f = ((h.frame.rem_euclid(60)) as f32 * 0.104_719_77 - std::f32::consts::PI).sin();
    let bar = crate::hud::lerp_rgba(f * 0.5 + 0.5, 0x80f0_c070, 0x40f0_c070);
    let top = ((82 - 364 - 4) as f32 * h.timer as f32 / h.max as f32 + (364 - 2) as f32) as i32;
    rect(out, top, 364 - 2, 23 + 2, 40 - 2, rgba(bar));
    let n = h.n.max(1) as f32;
    let seg = (82 - 362) as f32 / n;
    for i in 0..h.n {
        let y = (362.0 + i as f32 * seg) as i32;
        let y2 = (y as f32 + seg + 3.0) as i32;
        if (i as i32) < h.depth as i32 {
            rect(out, y2, y, 473, 487, rgba(0x80f0_c070));
        } else {
            rect(out, y2, y, 473, 487, rgba(0x55f0_c070));
            rect(out, y2 + 2, y - 2, 475, 485, rgba(0x6044_2d00));
        }
    }
    let (glyphs, slot) = font;
    for (id, x, y) in [(0x2be5, 400.0, 25.0), (0x2df9, 115.0, 380.0), (0x3077, 120.0, 25.0), (0x2c53, 395.0, 380.0)] {
        let t = text(id);
        let t = t.split(|&c| c == 0).next().unwrap_or(&[]);
        let width = crate::gui::text_width(glyphs, t, 1.0);
        crate::gui::text(out, glyphs, slot, t, (x as i32 - (width >> 1)) as f32, (y + 0.5f32) as i32 as f32, 1.0, rgba(0x8080_8080));
    }
}

/// The frame draw's white (`0x2e7250`: `$gp+0x3474` above 0, at most 1, a full-screen fill at alpha × 128).
pub fn draw_white(white: f32, out: &mut Vec<crate::gui::Quad>) {
    if white <= 0.0 { return; }
    let a = (white.min(1.0) * 128.0) as i32;
    crate::gui::fill(out, [0xff, 0xff, 0xff, a as u8]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corners_have_six_neighbours_and_faces_four() {
        let t = neighbours();
        for (i, row) in t.iter().enumerate() {
            let n = row.iter().filter(|&&u| u != -1).count();
            let corner = NODE_RAW[i].iter().all(|&c| c != 0.0);
            assert_eq!(n, if corner { 6 } else { 4 }, "node {i}");
        }
    }

    /// A board walked along its path closes the loop at node 0 at depth n.
    #[test]
    fn the_path_is_a_loop_and_walks_to_the_end() {
        for seed in [0i16, 7, 1234, -5] {
            let mut rng = crate::rng::Rng::default();
            let kinds = [3, 1, 3, 1, 3, 2, 2, 1];
            let mut b = Board::new(0, 0, &kinds, seed, 180, 30);
            let rows = crate::hero::physics::euler_rows([0.0, -std::f32::consts::FRAC_PI_2, 0.0]);
            b.init(&mut rng, rows);
            let n = b.n;
            assert!(3 <= n && n <= kinds.len());
            for i in 0..n {
                let (a, c) = (b.path[i] as i32, b.path_at(i as i32 + 1));
                assert!(b.nbs(a).contains(&c), "seed {seed}: {a} → {c} not joined");
            }
            b.orient();
            for _ in 0..n {
                let next = b.path_at(b.progress as i32 + 1);
                assert!(b.ways(8).any(|c| c as i32 == next), "seed {seed}: the path's next {next} is not a way on");
                b.cursor = next as i16;
                b.advance();
                while !b.done { b.ease_turn(); }
            }
            assert_eq!((b.cur, b.depth as usize, b.progress as usize), (0, n, n));
            // The node he is on faces the camera.
            let v = through(&b.rows, node(b.cur as i32));
            assert!(v[0] < -0.99, "seed {seed}: {v:?}");
        }
    }

    #[test]
    fn a_false_turn_and_back() {
        let mut rng = crate::rng::Rng::default();
        let mut b = Board::new(0, 0, &[1, 1, 1, 1, 1], 3, 180, 30);
        b.init(&mut rng, VIEW_ROWS);
        b.orient();
        let wrong = b.steps[0][1];
        if wrong == -1 { return; }
        assert!(b.ways(8).any(|c| c as i32 == wrong));
        b.cursor = wrong as i16;
        b.advance();
        assert_eq!((b.depth, b.off, b.progress), (1, 1, 0));
        b.cursor = b.prev;
        b.advance();
        assert_eq!((b.cur, b.depth, b.off), (0, 0, 0));
    }
}
