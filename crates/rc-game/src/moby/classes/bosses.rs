//! Maktar's arena bosses (L02, the arena's chunk 1): pooled with the fighters in the master's group `+0x40` (state 1,
//! hidden, no updates: mode | 3) until the master sends one ([`send_wave_bosses`], `0x3fcb40`: its master `+0xb8`, its
//! start, counted in the master's `+0xa0` / `+0xa4`, the master's boss `+0xd0` the HUD's bar reads: health `+0x20` over
//! `(s16) +0x24`). In a wave with both, they fight in turns: the one out hands over (state 3) each time its health falls
//! under the next third (`0x3d5fc8` / `0x3dceb8`, the thirds counted in `+0x31a` / `+0x282`), and tells the other to
//! come out (`0x3d5ef8` / `0x3dcde8`, [`call_back`]); the master's boss and the announcer's line follow (`0x3fd5c0`).
//!
//! **0x09f8** (`0x3d4348`; senses `0x3d5870`, tail `0x3d5c20`, start `0x3d5d18`, back to the pool `0x3d5e68`): a hover
//! robot with two arms that each throw a chain (the links 0x0a19..0x0a1d, `+0x2a0..`). Its sub-state is the ground (1:
//! the reaction block makes a hit state 8) or the air (2: 0x12). Out of its pen (`$gp+0x1190`) it dodges (10, one of
//! the sequences 5..8: [`dodge`]), flies in (2) and lands near the middle (0x14). On the ground (6) it walks to its
//! target and within 9 facing him swings both arms (7, the blades from its joints 0 and 1 along their first rows,
//! 5.3 long); after a swing or a knock (8), a while without hurting Ratchet takes it up (0x13) to hover (0xb) at 13 from
//! him: within 5..40 it lunges to the height its chain reaches him from (`0x3d6810`) and throws a chain from either arm
//! (0xc..0x11, [`chain`]: the links run out and back along the arm, the end blasts and hits where it meets the floor).
//! Its time in the air over, it lands (0x14; knocked down in the air it lands stunned: 8 sub 3). Dead, its bolts, a
//! second, then the fade (crate::moby::classes::death_fade) back to the pool. In challenge 12 it takes no hits
//! (`+0x330`). It draws its eyes and, flying, its thrusters' glows ([`draw_09f8`]).
//!
//! **0x0a44** (`0x3d9188`; senses `0x3da708`, its move by its sequence `0x3dac90`, its turret `0x3db3d0`, tail
//! `0x3dab70`, start `0x3dc878`, back to the pool `0x3dcc40`): a walker carrying the placed turret 0x0a4b (`+0x244`, at
//! its joint 0, turning to the target on its own: `+0x248` / `+0x24c`) and its gun 0x0a4c (`+0x260`, made at the start,
//! at the turret's joint 4). It jumps out of its pen (2; `$gp+0x13c0`) to the middle, walks at its target (5; turning
//! with its sequence 2 in place), and every 600..1200 ticks spins (6: sequence 7, its legs hitting what they sweep,
//! drifting toward its target, its spin faster as its health falls) until the count runs out again; a hurt Ratchet
//! takes it to its taunt (7) and back to the spin. While walking (or spinning under a fifth of its health) the turret
//! fires two shots (0x0d08, [`spawn_shot`]) every 20..60 ticks within 45. A hit of more than 3 while it spins
//! staggers the turret (9); its health out (10), the turret's death sequence, then (0xb) its gun thrown, its bolts, three
//! seconds, five blasts and its pieces and the turret's thrown, back to the pool. Hits only count from state 5 on.
//! It draws the turret's lights (`0x3dc2d0`, [`draw_0a44`]) in a colour that fades between its states' and pulses.
//!
//! **The shot 0x0d08** (`0x423038`, made by `0x422ec8`): 40 a second for 150 ticks; a line along its step meeting the
//! world or a moby other than its owner: its sound 0, sparks (`0x423578`) and a sphere of 0.8 there hitting for its
//! damage (flags 0x810001); its last 16 ticks it shrinks with a sparkle every third; its time out, a burst of sparkles.
//! It draws a glow and five sparks between its two spinning points ([`draw_shot`]).
//!
//! **Skill points** (crate::skill_points): 0x09f8 gives point 1 as its fade starts (`0x3d4d6c`) when nothing but the
//! wrench (class 0x47) or Ratchet hit it since it came out (`$gp+0x11b0`, set at its set-up and its state 2, cleared by
//! a hit from anything else, its health block's last hitter `+0x18`: crate::moby::classes::maktar::Globals); 0x0a44
//! gives point 0x13 on its death's first tick (`0x3da250`) unless Ratchet was hurt (movement group 7) or hit by it or
//! its shot since its jump in (`+0x290`, set every tick before its state, cleared through state 2).
//!
//! Not ported: the bosses' damage tallies by weapon for the weapons' XP (`0x1ff6b8` / `0x1ff798`, `0x1b3770` / `0x1b3774`, `0x1ba644`), the
//! scale manipulators (`0x316598`: 0x09f8's hand balls hidden while a chain is out, 0x0a44's turret joint 4 growing and
//! shrinking as it dies), 0x0a44's arm aims (rotation manipulators on the turret's joints 2 and 3, `0x321bf0` /
//! `0x322f28`), the joint aim `0x32c1d0`, the chain's lightning (`0x3d7258` → `0x33db10`), the particle bursts
//! `0x345d88` (0x09f8's thrusters `0x3d74b8`, 0x0a44's landing dust `0x3dcf48` and stagger smoke `0x3dc768`), 0x09f8's
//! swing trail (`0x3d7bf8`, `0x3d7f88`), 0x0a44's death sparks (`0x3dd0d8` → `0x339300`), the shot's beam (`0x423ef8`),
//! its spark streak (`0x424888`) and its impact flashes and lights (`0x423a48`), the moby word `+0x9c` 0x0a44 sets while
//! it spins.

use crate::hero::physics::{add, add_rot, approach, atan2, cross, dist, dist2, dot, ease, len, rot_dist, scale, set_len, sub, sub_rot, turn_ease, V3};
use crate::moby::{ai, mode, Ctx, MobyHost, MobyWorld};
use super::enemies::place_part;
use super::props::{seq, wrapped};

pub const C9F8: i16 = 0x09f8;
pub const CA44: i16 = 0x0a44;
/// 0x0a44's turret (placed) and gun (made).
pub const TURRET: i16 = 0x0a4b;
const GUN: i16 = 0x0a4c;
/// 0x09f8's first chain link (0x0a19..0x0a1d).
const CHAIN: i16 = 0x0a19;
/// 0x0a44's shot.
pub const SHOT: i16 = 0x0d08;

/// `$gp+0x1180`: the arena's middle on its floor (`$gp+0x1188`).
const MIDDLE: V3 = [500.0, 600.0, 500.0];
const FLOOR: f32 = 500.0;
/// `$gp+0x1190` / `+0x11a0`: 0x09f8's pen and the point it flies in to.
const PEN_9F8: V3 = [500.0, 642.0, 510.0];
const ENTRY_9F8: V3 = [500.0, 600.0, 512.0];
/// `$gp+0x13c0` / `+0x13d0`: 0x0a44's pen and the gate it walks to before it jumps in.
const PEN_A44: V3 = [500.0, 558.0, 507.5];
const GATE_A44: V3 = [500.0, 588.0, 500.0];

/// The master's pvars this module reads and writes (crate::moby::classes::arena).
const M_BOSS: usize = 0xd0;
const M_SENT_9F8: usize = 0xa0;
const M_SENT_A44: usize = 0xa4;
const M_CHALLENGE: usize = 0x120;
const M_SAID: usize = 0x1d0;

fn set_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, blend: u8) {
    if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, blend); }
}

fn seq_a(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a) }

fn frame(w: &MobyWorld, id: usize, ctx: &Ctx) -> f32 { ctx.host.frame_time(&w.mobys[id]) }

fn yaw_to(from: V3, to: V3) -> f32 { atan2(to[0] - from[0], to[1] - from[1]) }

/// `0x2eeab0` on an s16 pvar.
fn countdown_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut t = w.mobys[id].pv_s(off) as i32;
    let over = crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_s(off, t as i16);
    over
}

/// `0x321390(out, a, b, p)`: the point of the segment `a`..`b` nearest `p`.
fn closest(a: V3, b: V3, p: V3) -> V3 {
    let ab = sub(b, a);
    let l2 = dot(ab, ab);
    let t = if 0.0 < l2 { (dot(sub(p, a), ab) / l2).clamp(0.0, 1.0) } else { 0.0 };
    add(a, scale(ab, t))
}

/// `v` turned `a` about the unit axis `n`.
fn turn_about(v: V3, n: V3, a: f32) -> V3 {
    let (s, c) = a.sin_cos();
    let d = dot(n, v);
    let x = cross(n, v);
    std::array::from_fn(|k| v[k] * c + x[k] * s + n[k] * d * (1.0 - c))
}

/// `0x2f3260(out, v, m)`: `v` in a joint's frame.
fn in_frame(r: &[V3; 3], v: V3) -> V3 { add(add(scale(r[0], v[0]), scale(r[1], v[1])), scale(r[2], v[2])) }

/// The master (`+0xb8`).
fn master_of(w: &MobyWorld, id: usize) -> Option<usize> { super::fighters::master(w, id) }

/// A partner out (`+0x310` / `+0x264`: not dead, past its start).
fn partner(w: &MobyWorld, id: usize, off: usize) -> Option<usize> {
    ai::link(&w.mobys[id], off).filter(|&p| w.mobys.get(p).is_some_and(|x| x.state < 0x80 && 2 < x.state))
}

/// `0x3d5ef8` (0x09f8: `go` `+0x318`, `thirds` `+0x31a`) / `0x3dcde8` (0x0a44: `+0x280`, `+0x282`): told to come back
/// out (once) — its count of thirds gone moves past where its health is now.
fn call_back(w: &mut MobyWorld, id: usize, go: usize, thirds: usize) {
    let m = &mut w.mobys[id];
    if m.pv_s(go) != 0 { return; }
    let most = m.pv_s(0x24) as f32;
    let mut n = m.pv_s(thirds) + 1;
    while m.pv_f(0x20) < most * (3.0 - n as f32) * 0.333_333_34 { n += 1; }
    m.set_pv_s(thirds, n);
    m.set_pv_s(go, 1);
}

/// `0x3d5fc8` / `0x3dceb8`: its partner is out and its health has fallen under its next third.
fn hand_over_due(w: &MobyWorld, id: usize, link: usize, thirds: usize) -> bool {
    partner(w, id, link).is_some() && {
        let m = &w.mobys[id];
        m.pv_f(0x20) < m.pv_s(0x24) as f32 * (3.0 - m.pv_s(thirds) as f32) * 0.333_333_34
    }
}

/// `0x401d98(master, n)` as the port keeps it (crate::moby::classes::arena: the line said, `+0x1d0 + 2n`).
fn say(w: &mut MobyWorld, ms: usize, n: usize) { w.mobys[ms].set_pv_s(M_SAID + 2 * n, 1); }

/// `0x3fd5c0(master, b)`: the master's boss is `b`, with the announcer's line for it (0xf for 0x09f8, 0x15 for 0x0a44).
fn boss_is(w: &mut MobyWorld, ms: Option<usize>, b: usize) {
    let Some(ms) = ms else { return };
    ai::set_link(&mut w.mobys[ms], M_BOSS, Some(b));
    match w.mobys[b].o_class {
        C9F8 => say(w, ms, 0xf),
        CA44 => say(w, ms, 0x15),
        _ => {}
    }
}

/// Its partner told to come back out and made the master's boss (the hand-over: the partner's `0x3d5ef8` /
/// `0x3dcde8`, then `0x3fd5c0`), when it is out.
fn hand_over(w: &mut MobyWorld, id: usize) {
    let (link, go, thirds) = if w.mobys[id].o_class == C9F8 { (PARTNER_9F8, 0x280, 0x282) } else { (PARTNER_A44, 0x318, 0x31a) };
    let Some(p) = partner(w, id, link) else { return };
    call_back(w, p, go, thirds);
    let ms = master_of(w, id);
    boss_is(w, ms, p);
}

/// `0x3fcb40(master, class, arg)`: the first pooled boss of `class` (state 1) in the master's group: its master, its
/// start (0x09f8 taking no hits in challenge 12; 0x0a44 waiting in its pen with `arg`), counted.
fn send(w: &mut MobyWorld, ms: usize, ctx: &mut Ctx, class: i16, arg: bool) -> Option<usize> {
    let g = w.mobys[ms].pv_i(0x40);
    let list = usize::try_from(g).ok().filter(|&g| g != 0xff).and_then(|g| w.groups.get(g)).cloned()?;
    let b = list.into_iter().find(|&f| w.mobys.get(f).is_some_and(|x| x.state == 1 && x.o_class == class))?;
    w.mobys[b].parent = Some(ms);
    if class == C9F8 {
        let calm = w.mobys[ms].pv_s(M_CHALLENGE) == 0xc;
        start_09f8(w, b, ctx, calm);
        let n = w.mobys[ms].pv_i(M_SENT_9F8);
        w.mobys[ms].set_pv_i(M_SENT_9F8, n + 1);
    } else {
        start_0a44(w, b, ctx, arg);
        let n = w.mobys[ms].pv_i(M_SENT_A44);
        w.mobys[ms].set_pv_i(M_SENT_A44, n + 1);
    }
    Some(b)
}

/// The master's boss wave (`0x3fb1b0` at `0x3fc148..0x3fc364`, each tick it sends): with both bosses called for and
/// neither sent, 0x09f8 out and 0x0a44 waiting in its pen, each the other's partner, 0x09f8 the master's boss (`+0xd0`)
/// and called back once (its first third); else the one called for and not sent yet. Returns the announcer's line to
/// say (0xf both, 0xe 0x09f8, 0x14 0x0a44); the master then asks for its bosses' music (`0x335928(+0x230, +0x234)`,
/// crate::moby::classes::arena).
pub fn send_wave_bosses(w: &mut MobyWorld, ms: usize, ctx: &mut Ctx, bosses: [u8; 2]) -> Option<usize> {
    let (a, b) = (w.mobys[ms].pv_i(M_SENT_9F8), w.mobys[ms].pv_i(M_SENT_A44));
    if a == 0 && bosses[0] != 0 && b == 0 && bosses[1] != 0 {
        let s0 = send(w, ms, ctx, C9F8, false);
        let s2 = send(w, ms, ctx, CA44, true);
        if let Some(x) = s0 { ai::set_link(&mut w.mobys[x], PARTNER_9F8, s2); }
        if let Some(y) = s2 { ai::set_link(&mut w.mobys[y], PARTNER_A44, s0); }
        if let Some(x) = s0 { call_back(w, x, 0x318, 0x31a); }
        ai::set_link(&mut w.mobys[ms], M_BOSS, s0);
        return Some(0xf);
    }
    if a == 0 && bosses[0] != 0 {
        let s = send(w, ms, ctx, C9F8, false);
        ai::set_link(&mut w.mobys[ms], M_BOSS, s);
        return Some(0xe);
    }
    if b == 0 && bosses[1] != 0 {
        let s = send(w, ms, ctx, CA44, false);
        ai::set_link(&mut w.mobys[ms], M_BOSS, s);
        return Some(0x14);
    }
    None
}

/// A boss back to the pool (the master's `0x3fd130`: `0x3d5e68` / `0x3dcc40`).
pub fn pool(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].o_class {
        C9F8 => pool_09f8(w, id, ctx),
        CA44 => pool_0a44(w, id, ctx),
        _ => {}
    }
}

/// A sprite of FX 0xb facing the camera (`0x32bbe0(size, push, p, rgba)`: no roll, additive).
fn glow(out: &mut Vec<crate::fx::FxQuad>, size: f32, push: f32, p: V3, cam: V3, rgba: u32) {
    crate::fx::sprite(out, size, push, 0.0, p, cam, 0xb, rgba, true);
}

/// `rgb` with the alpha `f²·alpha`.
fn faded(rgba: u32, f: f32) -> u32 { (rgba & 0xff_ffff) | (((f * f * (rgba >> 24) as f32) as i32 as u32) << 24) }

// ---------------------------------------------------------------------------------------------------------------
// 0x09f8

/// Its chain's links (`+0x2a0..+0x2b0`) and their places along the throw (`$gp+0x1160`).
const LINKS: usize = 0x2a0;
const LINK_AT: [f32; 5] = [0.0, 1.5, 2.5, 3.5, 4.5];
/// The hover's goal (`+0x2d0`), stepped toward its aim (`+0x2c0`) turned `+0x308` off the line.
const GOAL: usize = 0x2d0;
const AIM: usize = 0x2c0;
/// Its target's moby (`+0x30c`, the target block's `+0x210`) and its partner 0x0a44 (`+0x310`).
const TARGET_9F8: usize = 0x30c;
const PARTNER_9F8: usize = 0x310;
/// `0x3d6538`'s ease (0.025, at most 1.5) and turn (`0x3b64c38a`, at most `0x3e626181`).
const HOVER_ACC: u32 = 0x3ccc_cccd;
const TURN_ACC: u32 = 0x3b64_c38a;
const TURN_MAX: u32 = 0x3e62_6181;
/// The arm's tilt the chain falls at (`0x3f933333`, 1.15 rad).
const TILT: u32 = 0x3f93_3333;

fn target_9f8(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    ai::link(&w.mobys[id], TARGET_9F8).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos)
}

fn links(w: &MobyWorld, id: usize) -> [Option<usize>; 5] {
    std::array::from_fn(|k| ai::link(&w.mobys[id], LINKS + 4 * k).filter(|&p| w.mobys.get(p).is_some_and(|x| x.state < 0x80 && x.o_class == CHAIN + k as i16)))
}

/// `0x3d6ad8`: its five links made (classes 0x0a19..0x0a1d: no updates, its draw distance and light, hidden).
fn make_links(w: &mut MobyWorld, id: usize) {
    for k in 0..5 {
        let c = CHAIN + k as i16;
        let l = w.class(c).is_some().then(|| w.create(c));
        if let Some(l) = l {
            let (dd, light, amb) = { let m = &w.mobys[id]; (m.draw_dist, m.light, m.ambient) };
            let x = &mut w.mobys[l];
            x.update_dist = 0;
            x.draw_dist = dd;
            (x.light, x.ambient) = (light, amb);
            x.mode |= 1;
            x.drawn = 0;
        }
        ai::set_link(&mut w.mobys[id], LINKS + 4 * k, l);
    }
    w.changed = true;
}

/// `0x3d7188`: its links deleted.
fn drop_links(w: &mut MobyWorld, id: usize) {
    for (k, l) in links(w, id).into_iter().enumerate() {
        if let Some(l) = l { w.delete(l); }
        ai::set_link(&mut w.mobys[id], LINKS + 4 * k, None);
    }
}

/// `0x3d70f8`: its links hidden (the hands' manipulators off: `+0x222` / `+0x262`).
fn hide_links(w: &mut MobyWorld, id: usize) {
    w.mobys[id].set_pv_b(0x222, 0);
    w.mobys[id].set_pv_b(0x262, 0);
    for l in links(w, id).into_iter().flatten() {
        let x = &mut w.mobys[l];
        x.mode |= 1;
        x.drawn = 0;
    }
}

/// `0x3d6bc8(m, arm)`: its links shown at the arm's joint, the throw's ticks (`+0x31e`) and its blow (`+0x320`) from 0.
fn show_links(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, arm: usize) {
    w.mobys[id].set_pv_b(if arm == 1 { 0x262 } else { 0x222 }, 1);
    w.mobys[id].set_pv_s(0x320, 0);
    for l in links(w, id).into_iter().flatten() {
        let x = &mut w.mobys[l];
        x.mode &= !1;
        x.drawn = 1;
        place_part(w, id, l, arm, ctx);
    }
    w.mobys[id].set_pv_s(0x31e, 0);
}

/// `0x3d6cb0(m, arm)`: the chain's throw from the arm's joint, along the arm's first row to the floor (its length
/// `L` the joint's height over the floor / cos 1.15). Each link out to `t·7/6` behind the others while the ticks `t`
/// are under `s = (L − 5.3)·6/7`, held at `s·k·7/24` five ticks, then back 1.5 a tick; each is placed from the last's
/// arc start (the first link's place less 0.2 along the joint's second and third rows, as the original's base moves).
/// At `s + 1` ticks the end blasts (0.5) and hits for `+0x35` there, once. Returns 1 past `s + 6`, 2 past
/// `s + (L − 5.3)·2/3 + 7`, else 0.
fn chain(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, arm: usize) -> i32 {
    let (Some(p0), Some((r, _))) = (ctx.host.joint(&w.mobys[id], arm), ctx.host.joint_matrix(&w.mobys[id], arm)) else { return 2 };
    let rows = r.map(|x| set_len(x, 1.0));
    let dir = rows[0];
    let reach = (p0[2] - FLOOR) / f32::from_bits(TILT).cos();
    let end = add(p0, scale(dir, reach));
    let out = reach - 5.3;
    let span = out * 0.857_142_8;
    let t = w.mobys[id].pv_s(0x31e) as f32;
    let mut base = p0;
    for (k, l) in links(w, id).into_iter().enumerate() {
        let Some(l) = l else { continue };
        place_part(w, id, l, arm, ctx);
        let kf = k as f32;
        let mut d = if t < span {
            (t - span * (4.0 - kf) * 0.25) * 1.166_666_7
        } else if t < span + 5.0 {
            span * kf * 0.291_666_7
        } else {
            span * kf * 0.291_666_7 - (t - (span + 5.0)) * 1.500_000_1
        };
        if d < 0.0 { d = 0.0; }
        w.mobys[l].pos = add(base, scale(dir, d + LINK_AT[k]));
        w.rebuild(l, ctx.host);
        if let Some(l0) = links(w, id)[0] { base = add(add(w.mobys[l0].pos, scale(rows[2], -0.2)), scale(rows[1], -0.2)); }
    }
    let n = w.mobys[id].pv_s(0x31e) + 1;
    w.mobys[id].set_pv_s(0x31e, n);
    let n = n as i32;
    if (out * 0.666_666_6) as i32 + span as i32 + 7 < n { return 2; }
    if span as i32 + 6 < n { return 1; }
    if span as i32 + 1 < n && w.mobys[id].pv_s(0x320) == 0 {
        crate::moby::blast::burst(w, ctx, end, &crate::moby::blast::Blast { size: 0.5, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: false, shake: false });
        let dmg = w.mobys[id].pv_b(0x35) as f32;
        ai::attack_sphere(w, id, ctx, 0.5, dmg, 1.0, end, 0x1_0001, 0, 1, 0);
        w.mobys[id].set_pv_s(0x320, 1);
    }
    0
}

/// `0x3d6538(yaw, roll, m, goal)`: the hover — its place eased to `goal` (`+0x1b0..`, 0.025, at most 1.5), its yaw
/// turned to `yaw` (`+0x1c4`), its roll to `roll` (`+0x2f0`, the limits `+0x124..+0x12c`), and its bob (`0x328918`:
/// 0.5 up and down, 5° a tick, `+0x2f4` / `+0x304`).
fn hover(w: &mut MobyWorld, id: usize, yaw: f32, roll: f32, goal: V3) {
    let m = &mut w.mobys[id];
    let acc = f32::from_bits(HOVER_ACC);
    for k in 0..3 {
        let (mut x, mut v) = (m.pos[k], m.pv_f(0x1b0 + 4 * k));
        ease(goal[k], acc, acc, f32::from_bits(0x3fc0_0001), &mut x, &mut v);
        m.pos[k] = x;
        m.set_pv_f(0x1b0 + 4 * k, v);
    }
    let (mut a, mut v) = (m.rot[2], m.pv_f(0x1c4));
    turn_ease(yaw, f32::from_bits(TURN_ACC), f32::from_bits(TURN_ACC), f32::from_bits(TURN_MAX), &mut a, &mut v, 0);
    m.rot[2] = a;
    m.set_pv_f(0x1c4, v);
    let (mut a, mut v) = (m.rot[1], m.pv_f(0x2f0));
    let (ra, rb, rm) = (m.pv_f(0x124), m.pv_f(0x128), m.pv_f(0x12c));
    turn_ease(roll, ra, rb, rm, &mut a, &mut v, 0);
    m.rot[1] = a;
    m.set_pv_f(0x2f0, v);
    // `0x328918(0.5, 5°, −1, m, +0x2f4, +0x304)`.
    let phase = add_rot(m.pv_f(0x2f4), f32::from_bits(0x3db2_b8c4));
    m.set_pv_f(0x2f4, phase);
    m.pos[2] -= m.pv_f(0x304);
    let b = phase.sin() * 0.5;
    m.set_pv_f(0x304, b);
    m.pos[2] += b;
}

/// `0x3d6438`: the goal stepped toward the aim (at once within 5/6, else 5/6 along the way turned by `+0x308`; the turn
/// reads its new x for its y, as the original's).
fn step_goal(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let (g, a) = (m.pv_v(GOAL), m.pv_v(AIM));
    let step = f32::from_bits(0x3f55_5556);
    if dist(g, a) < step {
        m.set_pv_v(GOAL, a);
        return;
    }
    let mut d = set_len(sub(a, g), step);
    let t = m.pv_f(0x308);
    d[0] = d[0] * t.cos() - d[1] * t.sin();
    d[1] = d[0] * t.sin() + d[1] * t.cos();
    m.set_pv_v(GOAL, add(g, d));
}

/// `0x3d6368`: a new aim 13 (±0.2) from its target at ±45° off where it is, 1 (±0.2) above him (`0x3d73a8`); more than
/// 10 across from the middle, the middle 1 up.
fn pick_aim(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut c = target_9f8(w, id, ctx);
    c[2] += 1.0;
    let pos = w.mobys[id].pos;
    let g = &mut ctx.game.rng;
    let yaw = atan2(pos[0] - c[0], pos[1] - c[1]) + g.randf(-45.0 * 0.017_453_294, 45.0 * 0.017_453_294);
    let r = 13.0 + g.randf(-0.2, 0.2);
    let mut a = [c[0] + r * yaw.cos(), c[1] + r * yaw.sin(), 0.0];
    a[2] = c[2] + g.randf(-0.2, 0.2);
    if 10.0 < dist2(a, MIDDLE) { a = [MIDDLE[0], MIDDLE[1], MIDDLE[2] + 1.0]; }
    w.mobys[id].set_pv_v(AIM, a);
}

/// `0x3d6058(m, how)`: a dodge (10), its state and sub-state kept (`+0x324` / `+0x326`) to go back to. On the ground
/// (sub 1): `how` 1, or (not −1) Ratchet's hurt share over its own lost share less up to 0.2, one of 7 / 8; else 5 / 6.
/// In the air (2) one of 0x13 / 0x14.
fn dodge(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, how: i32) {
    if w.mobys[id].sub_state == 1 {
        let most_hp = ctx.game.max_health.max(1);
        let hurt = ((most_hp - ctx.hero.hp as i32) / most_hp) as f32;
        let m = &w.mobys[id];
        let most = m.pv_s(0x24) as f32;
        let lost = (most - m.pv_f(0x20)) / most;
        let r = ctx.game.rng.randf(-0.2, 0.2);
        let away = how == 1 || (0.0 <= (hurt - lost) + r && how != -1);
        let pick = ctx.game.rng.randi(2);
        let s = match (away, pick) { (true, 0) => 8, (true, _) => 7, (false, 0) => 6, (false, _) => 5 };
        set_seq(w, id, ctx, s, 10);
    }
    if w.mobys[id].sub_state == 2 {
        let s = if ctx.game.rng.randi(2) == 0 { 0x13 } else { 0x14 };
        set_seq(w, id, ctx, s, 10);
    }
    let m = &mut w.mobys[id];
    let (s, b) = (m.state, m.sub_state);
    m.set_pv_s(0x324, s as i16);
    m.set_pv_s(0x326, b as i16);
    m.set_state(10, b as i32);
}

/// `0x3d6268`: time to fly — after 300 ticks (`+0x32e`), from 500 at once, between them one tick in `500 − t` since it
/// last hurt Ratchet (`+0x32a`; at once past 500 of those).
fn fly_due(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool { due(w, id, ctx, 300, 500) }

/// `0x3d62d0`: time to land — within 18 of the middle, as [`fly_due`] from 500 to 900.
fn land_due(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if 18.0 < dist2(w.mobys[id].pos, MIDDLE) { return false; }
    due(w, id, ctx, 500, 900)
}

fn due(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, from: i16, to: i16) -> bool {
    let (t, since) = (w.mobys[id].pv_s(0x32e), w.mobys[id].pv_s(0x32a));
    if t < from { return false; }
    if t <= to && 0 < to - since && ctx.game.rng.randi((to - since) as i32) != 0 { return false; }
    w.mobys[id].set_pv_s(0x32e, 0);
    w.mobys[id].set_pv_s(0x32a, 0);
    true
}

/// `0x3d6680(m, arm)`: the swing's blow — a sphere on the arm's blade (from its joint 5.3 along its first row) at the
/// point nearest its target, 0.3..0.5 wide by how far out, hitting for `+0x34`.
fn swing_hit(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, arm: usize) {
    let (Some(p0), Some((r, _))) = (ctx.host.joint(&w.mobys[id], arm), ctx.host.joint_matrix(&w.mobys[id], arm)) else { return };
    let e = add(p0, set_len(r[0], 5.3));
    let tgt = target_9f8(w, id, ctx);
    let c = closest(p0, e, tgt);
    let f = (len(sub(c, p0)) / len(sub(e, p0)).max(1e-6)).clamp(0.0, 1.0);
    let dmg = w.mobys[id].pv_b(0x34) as f32;
    ai::attack_sphere(w, id, ctx, f * 0.2 + 0.3, dmg, 1.0, c, 0x1_0001, 0, 1, 0);
}

/// `0x3d6810`: the lunge to the height its chain reaches its target from (the floor plus the distance / tan 1.15, less
/// 4.2): too high (over 5) 0; under 0.5 it backs off across (by 4.7 / (h + 4.2) of its way from him), else it rises or
/// sinks to it, turning to him. 2 once there, else 1.
fn lunge(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> i32 {
    let tgt = target_9f8(w, id, ctx);
    let pos = w.mobys[id].pos;
    let a = f32::from_bits(TILT);
    let mut goal = pos;
    goal[2] = FLOOR + dist(pos, tgt) * a.cos() / a.sin() - 4.2;
    if FLOOR + 5.0 < goal[2] { return 0; }
    let acc = f32::from_bits(HOVER_ACC);
    let max = f32::from_bits(0x4020_0001);
    let yaw = yaw_to(pos, tgt);
    let m = &mut w.mobys[id];
    if goal[2] < FLOOR + 0.5 {
        let h = goal[2] - FLOOR;
        let mut v = sub(pos, tgt);
        v[2] = 0.0;
        goal = add(goal, scale(v, 4.7 / (h + 4.2)));
        for k in 0..2 {
            let (mut x, mut s) = (m.pos[k], m.pv_f(0x1b0 + 4 * k));
            ease(goal[k], acc, acc, max, &mut x, &mut s);
            m.pos[k] = x;
            m.set_pv_f(0x1b0 + 4 * k, s);
        }
    } else {
        let (mut x, mut s) = (m.pos[2], m.pv_f(0x1b8));
        ease(goal[2], acc, acc, max, &mut x, &mut s);
        m.pos[2] = x;
        m.set_pv_f(0x1b8, s);
    }
    let (mut r, mut v) = (m.rot[2], m.pv_f(0x1c4));
    turn_ease(yaw, f32::from_bits(TURN_ACC), f32::from_bits(TURN_ACC), f32::from_bits(TURN_MAX), &mut r, &mut v, 0);
    m.rot[2] = r;
    m.set_pv_f(0x1c4, v);
    if 0.0001 <= dist(m.pos, goal) { return 1; }
    m.set_pv_f(0x1b8, 0.0);
    2
}

/// `0x3d5d18(m, calm)`: out of the pen — its health full, at the pen (`$gp+0x1190`) facing Ratchet, the ground's
/// sub-state, straight into a dodge (back to 2), shown and a target, no partner, its links made, the colour flash
/// reset; `calm` (challenge 12) it takes no hits (`+0x330`).
fn start_09f8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, calm: bool) {
    let coll = w.class(C9F8).is_some_and(|c| c.has_collision);
    let a = ctx.game.rng.randf(0.0, f32::from_bits(0x3fc9_0fdc));
    let m = &mut w.mobys[id];
    let most = m.pv_s(0x24) as f32;
    m.set_pv_f(0x20, most);
    m.set_pv_f(0x304, 0.0);
    m.set_pv_s(0x32c, 0);
    m.set_pv_i(0x300, 0);
    m.set_pv_s(0x32a, 0);
    m.set_pv_s(0x98, 0);
    m.set_pv_f(0x2f4, a);
    m.set_state(2, 1);
    dodge(w, id, ctx, 1);
    let hero = ctx.hero.pos;
    let m = &mut w.mobys[id];
    m.set_pv_b(0x7a, 0);
    m.set_pv_b(0x2e, 1);
    m.drawn = 1;
    m.mode = (m.mode & !3) | mode::TARGET;
    m.has_collision = coll;
    m.drops = 0;
    m.pos = PEN_9F8;
    m.rot[2] = yaw_to(PEN_9F8, hero);
    m.set_pv_i(PARTNER_9F8, 0);
    m.set_pv_s(0x318, 0);
    m.set_pv_s(0x31a, 0);
    make_links(w, id);
    let m = &mut w.mobys[id];
    if m.pv_s(0x60) != 0 {
        m.set_pv_s(0x60, 0);
        m.ambient = [m.pv_b(0x64), m.pv_b(0x65), m.pv_b(0x66)];
        m.set_pv_s(0x62, 1);
    }
    m.set_pv_s(0x330, calm as i16);
    w.changed = true;
}

/// `0x3d5e68`: back to the pool — its links deleted, sequence 0, state 1, hidden, no updates.
pub fn pool_09f8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ai::link(&w.mobys[id], LINKS).is_some() { drop_links(w, id); }
    ctx.host.set_anim(&w.mobys[id], 0, 0, 3);
    let m = &mut w.mobys[id];
    m.set_state(1, 0);
    m.drawn = 0;
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 3;
    w.changed = true;
}

/// `0x3d5870`: set-up, its shadow, the hits (none before state 5 or in an early dodge; the bomb and the shot 0xbfe /
/// 0xc89 do 0.3 of theirs), the knocked sequences, back to the pool when told, its target.
fn senses_09f8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 24);
        m.set_pv_b(0x1f9, 1);
        m.set_pv_b(0xd7, 1);
        m.set_pv_b(0x29, 1);
        m.set_pv_s(0x32e, 0);
        m.set_pv_i(PARTNER_9F8, 0);
        m.set_pv_s(0x318, 0);
        m.set_pv_s(0x31a, 0);
        m.set_pv_i(0x300, 0);
        m.set_pv_f(0x8c, f32::from_bits(0x3b88_8889));
        m.set_pv_f(0x94, 5.0);
        m.set_pv_f(0x9c, f32::from_bits(0x3e19_999a));
        m.set_pv_f(0xbc, 0.25);
        m.set_pv_f(0xb8, f32::from_bits(0x3dcc_cccd));
        m.set_pv_b(0x28, 3);
        m.set_pv_f(0x110, 3.0);
        m.set_pv_f(0x114, 2.0);
        m.set_pv_f(0x11c, 0.5);
        m.set_pv_f(0x120, f32::from_bits(0x3f91_361f));
        m.set_pv_f(0x140, f32::from_bits(0x3f06_0a93));
        m.set_pv_f(0x144, f32::from_bits(0x3c23_d70a));
        m.set_pv_f(0x13c, f32::from_bits(0x3bb6_0b61));
        m.set_pv_f(0x90, 3.0);
        m.set_pv_f(0x1e4, 0.5);
        m.set_pv_f(0x118, 0.5);
        w.maktar.wrench_only = true;
        pool_09f8(w, id, ctx);
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 60.0, 0x38);
    if w.mobys[id].pv_s(0x330) == 0 {
        if let Some(h) = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0) {
            let m = &w.mobys[id];
            let early = m.state < 5 || (m.state == 10 && m.pv_s(0x324) < 5);
            let weak = h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == 0x0c89 || s.o_class == 0x0bfe);
            let slot = m.hit_slot as usize;
            if let Some(r) = w.hits.records.get_mut(slot) {
                if early { r.tmpl.damage = 0.0; }
                if weak { r.tmpl.damage *= 0.3; }
            }
        }
        if ai::take_damage(w, id, ctx, false) != 0 {
            if !w.mobys[id].has_collision { w.mobys[id].has_collision = w.class(C9F8).is_some_and(|c| c.has_collision); }
            // Its last hitter (the health block's `+0x18`) other than the wrench (0x47) or Ratchet (0): no skill point.
            let by = ai::link(&w.mobys[id], ai::health(&w.mobys[id]) + 0x18).and_then(|s| w.mobys.get(s));
            if by.is_some_and(|s| s.o_class != crate::hero::items::WRENCH_CLASS && s.o_class != 0) { w.maktar.wrench_only = false; }
            let m = &w.mobys[id];
            if m.sub_state == 4 { set_seq(w, id, ctx, 0x10, 3); }
            let m = &w.mobys[id];
            if m.sub_state == 3 && m.pv_f(0x20) <= 0.0 { set_seq(w, id, ctx, 4, 3); }
        }
        if w.mobys[id].state != 0 && w.mobys[id].pv_b(0x2e) == 2 {
            hand_over(w, id);
            pool_09f8(w, id, ctx);
        }
    }
    let area = w.mobys[id].pv_i(0x314);
    let found = ai::find_target(w, id, ctx, 100.0, 100.0, area, None, false);
    let m = &mut w.mobys[id];
    m.set_pv_s(0x322, found as i16);
    let t = m.pv_i(0x210);
    m.set_pv_i(TARGET_9F8, t);
}

/// `0x3d5c20`: its draw, the ticks; the ticks since it last hurt Ratchet (`+0x32a`, a hurt by any 0x09f8 counts) and
/// in its mode (`+0x32e`).
fn tail_09f8(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    w.register_draw(id);
    let by = ctx.hero.damage.hit_by;
    let mine = by.is_some_and(|b| b == id || w.mobys.get(b).is_some_and(|x| x.o_class == C9F8));
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    if mine { m.set_pv_s(0x32a, -1); }
    m.set_pv_s(0x32a, m.pv_s(0x32a).wrapping_add(1));
    m.set_pv_s(0x32e, m.pv_s(0x32e).wrapping_add(1));
}

/// The sub-states' once blocks: the air's (the reaction to a hit 0x12, its pvars), the ground's (8; its limits, its
/// links hidden), the knocks' (links hidden); in the air it stays above the floor less 1.
fn sub_once(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let first = m.once & 2 == 0;
    match m.sub_state {
        2 => {
            if first {
                m.once |= 2;
                m.set_pv_b(0x70, 0x12);
                m.set_pv_i(0xa8, 4);
                m.set_pv_b(0xd6, 1);
                m.set_pv_i(0x148, m.pv_i(0x148) | 4);
                m.set_pv_f(0x30, 5.0);
                m.set_pv_i(0x14c, -1);
            }
            if m.pos[2] < FLOOR - 1.0 { m.pos[2] = FLOOR - 1.0; }
        }
        1 => {
            if first {
                m.once |= 2;
                m.set_pv_b(0xd6, 0);
                m.set_pv_b(0x70, 8);
                m.set_pv_i(0xa8, 3);
                m.set_pv_i(0x148, m.pv_i(0x148) & !4);
                m.set_pv_f(0x30, 3.0);
                ai::senses(m, f32::from_bits(0x3b91_a2b4), f32::from_bits(0x3e08_8889), f32::from_bits(0x3c0a_3623), f32::from_bits(0x3e56_7752));
                hide_links(w, id);
                w.mobys[id].set_pv_i(0x14c, -1);
            }
        }
        3 | 4 => {
            if first {
                m.once |= 2;
                hide_links(w, id);
            }
        }
        _ => {}
    }
}

fn shake(ctx: &mut Ctx, amp: u32, ticks: i32) { ctx.game.shakes.ask(0, f32::from_bits(amp), ticks); }

/// The chain attack's first part (0xc arm 0 / 0xf arm 1, sequence `s` 10 / 0xd): the lunge (too high: back to 0xb; there:
/// the sequence), then from frame 19 the links out ([`show_links`], `+0x328`) and the throw: out → `next` (0xe / 0x11);
/// back → links hidden. Past frame 27 of `s`, `wind` (0xd / 0x10).
fn chain_start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, arm: usize, s: u8, next: u8, wind: u8) {
    if w.mobys[id].first() { w.mobys[id].set_pv_s(0x328, 0); }
    if w.mobys[id].pv_s(0x328) == 0 {
        match lunge(w, id, ctx) {
            0 => {
                hide_links(w, id);
                w.mobys[id].set_state(0xb, -1);
            }
            2 => {
                if seq_a(w, id, ctx) != s && seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, 7); }
            }
            _ => {}
        }
    } else {
        let (r, p) = (w.mobys[id].rot[2], w.mobys[id].pos);
        hover(w, id, r, 0.0, p);
    }
    if w.mobys[id].pv_s(0x328) == 0 && seq_a(w, id, ctx) == s && 19.0 < frame(w, id, ctx) {
        w.mobys[id].set_pv_v(0x1b0, [0.0; 3]);
        show_links(w, id, ctx, arm);
        w.mobys[id].set_pv_s(0x328, 1);
    }
    if w.mobys[id].pv_s(0x328) != 0 {
        match chain(w, id, ctx, arm) {
            1 => w.mobys[id].set_state(next, 2),
            2 => {
                hide_links(w, id);
                w.mobys[id].set_pv_s(0x328, 0);
            }
            _ => {}
        }
    }
    if w.mobys[id].pv_s(0x328) == 1 && seq_a(w, id, ctx) == s && 27.0 < frame(w, id, ctx) { w.mobys[id].set_state(wind, 2); }
}

/// The chain's wind-up (0xd / 0x10, sequence 0xb / 0xe): hovering in place, the throw back → `next`.
fn chain_wind(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, arm: usize, s: u8, next: u8) {
    if w.mobys[id].first() { set_seq(w, id, ctx, s, 7); }
    let (r, p) = (w.mobys[id].rot[2], w.mobys[id].pos);
    hover(w, id, r, 0.0, p);
    if w.mobys[id].pv_s(0x328) != 0 && chain(w, id, ctx, arm) == 2 {
        w.mobys[id].set_state(next, 2);
        hide_links(w, id);
        w.mobys[id].set_pv_s(0x328, 0);
    }
}

/// The chain's end (0xe / 0x11, sequence 0xc / 0xf): the throw run out, hovering in place; past frame 6, back to 0xb.
fn chain_end(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, arm: usize, s: u8) {
    if w.mobys[id].first() { set_seq(w, id, ctx, s, 7); }
    if w.mobys[id].pv_s(0x328) != 0 && chain(w, id, ctx, arm) == 2 {
        hide_links(w, id);
        w.mobys[id].set_pv_s(0x328, 0);
    }
    let (r, p) = (w.mobys[id].rot[2], w.mobys[id].pos);
    hover(w, id, r, 0.0, p);
    if w.mobys[id].pv_s(0x328) == 0 && seq_a(w, id, ctx) == s && 6.0 < frame(w, id, ctx) {
        hide_links(w, id);
        w.mobys[id].set_state(0xb, 2);
    }
}

/// 0x09f8 (`0x3d4348`; module docs).
pub fn update_09f8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    senses_09f8(w, id, ctx);
    let tgt = target_9f8(w, id, ctx);
    let pos = w.mobys[id].pos;
    let d = dist2(pos, tgt);
    let yaw_t = yaw_to(pos, tgt);
    let ft = frame(w, id, ctx);
    sub_once(w, id);
    match w.mobys[id].state {
        1 => return,
        2 => {
            if w.mobys[id].first() {
                w.maktar.wrench_only = true;
                w.mobys[id].state_ticks = 0;
                set_seq(w, id, ctx, 0, 10);
            }
            if 0x78 < w.mobys[id].state_ticks as u16 {
                if seq_a(w, id, ctx) == 9 {
                    step_goal(w, id);
                    let g = w.mobys[id].pv_v(GOAL);
                    hover(w, id, yaw_t, 0.0, g);
                } else {
                    set_seq(w, id, ctx, 9, 10);
                    let m = &mut w.mobys[id];
                    m.sub_state = 2;
                    let p = m.pos;
                    m.set_pv_v(GOAL, p);
                    m.set_pv_v(AIM, ENTRY_9F8);
                }
            }
            if dist2(w.mobys[id].pos, MIDDLE) < 18.0 {
                w.mobys[id].set_pv_s(0x32e, 0);
                w.mobys[id].set_state(0x14, 2);
            }
        }
        3 => {
            // Handing over: back to the pen, its partner out.
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 9, 10);
                let a = ctx.game.rng.randf(20.0, 40.0) * 0.017_453_294;
                let neg = ctx.game.rng.randi(2) != 0;
                let m = &mut w.mobys[id];
                let p = m.pos;
                m.set_pv_v(GOAL, p);
                m.set_pv_f(0x308, if neg { -a } else { a });
                m.set_pv_v(AIM, PEN_9F8);
                m.set_pv_s(0x318, 0);
                if let Some(p) = ai::link(&w.mobys[id], PARTNER_9F8).filter(|&p| w.mobys.get(p).is_some()) {
                    call_back(w, p, 0x280, 0x282);
                    let ms = master_of(w, id);
                    boss_is(w, ms, p);
                }
            }
            step_goal(w, id);
            let g = w.mobys[id].pv_v(GOAL);
            let y = yaw_to(w.mobys[id].pos, ctx.hero.pos);
            hover(w, id, y, 0.0, g);
            if dist(w.mobys[id].pos, w.mobys[id].pv_v(AIM)) < 0.5 { w.mobys[id].set_state(4, 1); }
        }
        4 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0, 0xf); }
            ai::hold(w, id, ctx);
            if w.mobys[id].pv_s(0x318) != 0 { w.mobys[id].set_state(2, 1); }
        }
        6 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 1, 10); }
            if ctx.game.rng.randi(400) == 0 { dodge(w, id, ctx, -1); }
            let near = { let d = sub(tgt, MIDDLE); dot(d, d) < 529.0 };
            if near {
                let p = w.mobys[id].pos;
                let y = yaw_to(p, tgt);
                ai::move_step(w, id, ctx, y, tgt);
            } else {
                dodge(w, id, ctx, 1);
            }
            if (seq_a(w, id, ctx) == 1 && 1.0 < ft && ft < 3.0) || (16.0 < ft && ft < 18.0) { shake(ctx, 0x3d4c_cccd, 0x19); }
            if d < 9.0 && rot_dist(yaw_t, w.mobys[id].rot[2]) < 0.087_266_475 && 3.0 < ft && ft < 15.0 { w.mobys[id].set_state(7, 1); }
        }
        7 => {
            // The swing: the first arm's blade over frames 19..23, the second's over 32..36.
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 2, 10);
                w.mobys[id].timer = 0;
            }
            let swinging = seq_a(w, id, ctx) == 2;
            let t = w.mobys[id].timer;
            if swinging && 19.0 < ft && ft < 23.0 {
                swing_hit(w, id, ctx, 0);
            } else if t == 0 && swinging && 23.0 < ft {
                w.mobys[id].timer = 1;
            } else if swinging && 32.0 < ft && ft < 36.0 && t != 0 {
                swing_hit(w, id, ctx, 1);
            } else if t == 1 && swinging && 36.0 < ft {
                w.mobys[id].timer = 2;
            }
            if wrapped(w, id, ctx) {
                let up = fly_due(w, id, ctx) || hand_over_due(w, id, PARTNER_9F8, 0x31a);
                w.mobys[id].set_state(if up { 0x13 } else { 6 }, 1);
                let mine = ctx.hero.damage.hit_by == Some(id);
                if ctx.game.rng.randi(3) == 0 || mine { dodge(w, id, ctx, if mine { 0 } else { -1 }); }
            }
        }
        8 => {
            // Knocked: its flight (it stays within 18 of the middle); dead, its death sequence to frame 110.
            let keep = w.mobys[id].pos;
            if 0.0 < w.mobys[id].pv_f(0x20) {
                if ai::flight(w, id, ctx) & 2 != 0 {
                    let up = fly_due(w, id, ctx) || hand_over_due(w, id, PARTNER_9F8, 0x31a);
                    w.mobys[id].set_state(if up { 0x13 } else { 6 }, 1);
                }
            } else if ft <= 95.0 || 97.0 <= ft {
                if 110.0 < ft { w.mobys[id].set_state(9, -1); }
            } else {
                shake(ctx, 0x3ecc_cccd, 0x19);
            }
            if 18.0 < dist2(w.mobys[id].pos, MIDDLE) { w.mobys[id].pos = keep; }
        }
        9 => {
            // Dead: its bolts, a second still, then (beaten with the wrench alone: skill point 1, `0x3d4d6c`) the
            // fade back to the pool, its partner out.
            if w.mobys[id].first() {
                ctx.host.set_anim_speed(&w.mobys[id], 0.0);
                w.mobys[id].state_ticks = 0;
                crate::moby::bolts::drop(w, id, 0, 3, ctx);
                w.mobys[id].has_collision = w.class(C9F8).is_some_and(|c| c.has_collision);
            }
            if 0x3c < w.mobys[id].state_ticks as u16 {
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                if w.maktar.wrench_only { ctx.game.award_skill_point(crate::skill_points::point::BOSS_9F8); }
                super::death_fade::start(w, id, ctx, 0.0, Some(pool_09f8));
                hand_over(w, id);
            }
        }
        10 => {
            if w.mobys[id].sub_state == 2 {
                let a = w.mobys[id].pv_v(AIM);
                hover(w, id, yaw_t, 0.0, a);
            } else {
                let (r, p) = (w.mobys[id].rot[2], w.mobys[id].pos);
                ai::move_step(w, id, ctx, r, p);
            }
            if wrapped(w, id, ctx) {
                let (s, b) = (w.mobys[id].pv_s(0x324), w.mobys[id].pv_s(0x326));
                w.mobys[id].set_state(s as u8, b as i32);
                ctx.host.set_anim_speed(&w.mobys[id], 0.0);
            }
        }
        0xb => {
            // Hovering 13 from its target; within 5..40 a chain from either arm.
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 9, 10);
                let a = ctx.game.rng.randf(20.0, 40.0) * 0.017_453_294;
                let neg = ctx.game.rng.randi(2) != 0;
                let m = &mut w.mobys[id];
                m.set_pv_s(0x31c, 0);
                let p = m.pos;
                m.set_pv_v(GOAL, p);
                m.set_pv_f(0x308, if neg { -a } else { a });
            }
            pick_aim(w, id, ctx);
            step_goal(w, id);
            let g = w.mobys[id].pv_v(GOAL);
            hover(w, id, yaw_t, 0.0, g);
            if ctx.game.rng.randi(400) == 0 { dodge(w, id, ctx, 0); }
            if d < 40.0 && 5.0 < d && countdown_s(w, id, 0x31c) {
                let s = if ctx.game.rng.randi(2) == 0 { 0xf } else { 0xc };
                w.mobys[id].set_state(s, 2);
                if ctx.game.rng.randi(6) == 0 { dodge(w, id, ctx, 0); }
            }
            if land_due(w, id, ctx) { w.mobys[id].set_state(0x14, 2); }
            if hand_over_due(w, id, PARTNER_9F8, 0x31a) { w.mobys[id].set_state(3, 2); }
        }
        0xc => chain_start(w, id, ctx, 0, 10, 0xe, 0xd),
        0xd => chain_wind(w, id, ctx, 0, 0xb, 0xe),
        0xe => chain_end(w, id, ctx, 0, 0xc),
        0xf => chain_start(w, id, ctx, 1, 0xd, 0x11, 0x10),
        0x10 => chain_wind(w, id, ctx, 1, 0xe, 0x11),
        0x11 => chain_end(w, id, ctx, 1, 0xf),
        0x12 => {
            // Knocked in the air: its flight; down, it lands stunned.
            let g = w.mobys[id].pv_v(GOAL);
            let r = w.mobys[id].rot[2];
            hover(w, id, r, 0.0, g);
            let f = ai::flight(w, id, ctx);
            if f & 1 != 0 {
                w.mobys[id].set_state(0x14, 2);
                w.mobys[id].set_pv_s(0x32c, 1);
            } else if f & 2 != 0 {
                w.mobys[id].set_state(0xb, 2);
            }
        }
        0x13 => {
            // Taking off.
            if w.mobys[id].first() { set_seq(w, id, ctx, 0x11, 10); }
            if seq_a(w, id, ctx) == 0x11 && 9.0 < frame(w, id, ctx) {
                let m = &mut w.mobys[id];
                m.sub_state = 2;
                m.once &= !2;
            }
            if wrapped(w, id, ctx) { w.mobys[id].set_state(0xb, 2); }
        }
        0x14 => {
            // Landing: down to the floor, turning to its target; on it, the ground (or, knocked down, stunned: 8 sub 3).
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.set_pv_i(0x148, m.pv_i(0x148) & !4);
                set_seq(w, id, ctx, 9, 10);
                hide_links(w, id);
            }
            let p = w.mobys[id].pos;
            if FLOOR + 1.0 <= p[2] || p[2] <= FLOOR {
                hover(w, id, yaw_t, 0.0, [p[0], p[1], FLOOR]);
            } else {
                if seq_a(w, id, ctx) != 0x12 && seq(w, id, ctx) != 0x12 { ctx.host.set_anim(&w.mobys[id], 0x12, 0, 10); }
                ai::move_step(w, id, ctx, yaw_t, p);
            }
            if seq_a(w, id, ctx) == 0x12 && (w.mobys[id].pv_s(0x1d0) != 0 || w.mobys[id].pos[2] < FLOOR + 0.2) {
                if w.mobys[id].pv_s(0x32c) == 0 {
                    w.mobys[id].set_state(6, 1);
                } else {
                    ctx.host.set_anim(&w.mobys[id], 4, 0, 3);
                    w.mobys[id].set_pv_b(0x7a, 0);
                    w.mobys[id].set_state(8, 3);
                }
                if seq_a(w, id, ctx) == 0x12 && 8.0 < ft && ft < 10.0 { shake(ctx, 0x3e4c_cccd, 0x19); }
            }
        }
        _ => {}
    }
    tail_09f8(w, id, ctx);
}

/// `0x3d7858` (0x09f8, every tick): its eyes (`0x3d7ad8` at joints 4 and 5: 0.2 across, 0.5 toward the camera, dark red
/// `0x22000066` by how much they face it, over 0.3) and, flying, its thrusters (`0x3d78f0` at joints 2 and 3, the second
/// mirrored: 0.8 at the joint and 0.3 at `(−1, ±0.05, −0.1)` in its frame, 2 toward the camera, blue `0x88ff8020` by how
/// much its third row points away). Not ported: the swing's trail (sequence 2, `0x3d7f88`).
pub fn draw_09f8(w: &MobyWorld, host: &dyn MobyHost, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    for j in [4, 5] {
        let (Some(p), Some((r, _))) = (host.joint(m, j), host.joint_matrix(m, j)) else { continue };
        let f = dot(set_len(sub(cam, p), 1.0), r[0]);
        if 0.3 < f { glow(out, 0.2, 0.5, p, cam, faded(0x2200_0066, f)); }
    }
    if m.sub_state != 2 { return; }
    for (j, mirror) in [(2, false), (3, true)] {
        let (Some(p0), Some((r, _))) = (host.joint(m, j), host.joint_matrix(m, j)) else { continue };
        let p1 = add(p0, in_frame(&r, [-1.0, if mirror { -0.05 } else { 0.05 }, -0.1]));
        let f = dot(set_len(sub(cam, p0), 1.0), set_len(r[2], -1.0));
        if 0.0 < f {
            let c = faded(0x88ff_8020, f);
            glow(out, 0.8, 2.0, p0, cam, c);
            glow(out, 0.3, 2.0, p1, cam, c);
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0a44

/// Its target's place (`+0x230`, from its moby `+0x240`, the target block's `+0x160`), its turret (`+0x244`), gun
/// (`+0x260`), partner 0x09f8 (`+0x264`), its spin's sound (`+0x26c`).
const GOAL_A44: usize = 0x230;
const TARGET_A44: usize = 0x240;
const TURRET_AT: usize = 0x244;
const GUN_LINK: usize = 0x260;
const PARTNER_A44: usize = 0x264;
const SPIN_SOUND: usize = 0x26c;
/// `+0x290`: Ratchet hurt since its jump in (its skill point lost).
const HURT_A44: usize = 0x290;
/// The turret's light colour now, from and to, and the fade (`+0x250..+0x25c`).
const LIGHT: usize = 0x250;
/// Its light's colours: walking, firing, spinning, its taunt's spin, staggered, dead.
const LIGHT_IDLE: u32 = 0x40ff_00ea;
const LIGHT_FIRE: u32 = 0x80ff_00ea;
const LIGHT_SPIN: u32 = 0x30ff_00ea;
const LIGHT_TAUNT: u32 = 0x8080_6060;
const LIGHT_HIT: u32 = 0x8000_ffff;
const LIGHT_DEAD: u32 = 0xff00_0000;

fn target_a44(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    ai::link(&w.mobys[id], TARGET_A44).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos)
}

/// Its turret 0x0a4b (`+0x244`: a moby index; the placed record's when the level's links left it so).
pub fn turret(w: &MobyWorld, id: usize) -> Option<usize> {
    let i = usize::try_from(w.mobys[id].pv_i(TURRET_AT)).ok()?;
    let ok = |t: usize| w.mobys.get(t).is_some_and(|x| x.o_class == TURRET && x.state < 0x80);
    if ok(i) { return Some(i); }
    w.mobys.iter().position(|m| m.instance == Some(i)).filter(|&t| ok(t))
}

fn gun(w: &MobyWorld, id: usize) -> Option<usize> {
    ai::link(&w.mobys[id], GUN_LINK).filter(|&g| w.mobys.get(g).is_some_and(|x| x.state < 0x80 && x.o_class == GUN))
}

fn tset(w: &MobyWorld, t: Option<usize>, ctx: &mut Ctx, s: u8, blend: u8) {
    if let Some(t) = t { set_seq(w, t, ctx, s, blend); }
}

fn stop_spin_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(SPIN_SOUND);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(SPIN_SOUND, -1);
}

/// `0x333d78(speed, gravity, m, goal)`: a jump to `goal` — across at `speed` a tick, up so it lands there under
/// `gravity` (`0x3223a0`), the velocity in the movement block (`+0xf0`).
fn jump(w: &mut MobyWorld, id: usize, speed: f32, gravity: f32, goal: V3) {
    let m = &mut w.mobys[id];
    m.set_pv_f(0x7c, gravity);
    let mut v = sub(goal, m.pos);
    v[2] = 0.0;
    v = set_len(v, speed);
    let d = sub(goal, m.pos);
    let t = (d[0] * d[0] + d[1] * d[1]).sqrt() / speed;
    v[2] = if t != 0.0 { -((m.pos[2] - goal[2]) + -gravity * t * t * 0.5) / t } else { 0.0 };
    m.set_pv_v(0xf0, v);
}

/// `0x322f28(step, &p, goal)`: `p` moved toward `goal` by at most `step`.
fn approach_v(p: V3, goal: V3, step: f32) -> V3 {
    let d = sub(goal, p);
    let l = len(d);
    if l <= step { goal } else { add(p, scale(d, step / l)) }
}

/// `0x3dc878(m, wait)`: out — its health full, shown and a target, at the pen (`$gp+0x13c0`); its gun made (0x0a4c,
/// no collision, no updates); the turret at its joint 0 and both facing Ratchet, shown; waiting (4) or straight to
/// its jump (2, no partner); its spin 600..1200 ticks away; its colour flash reset.
fn start_0a44(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, wait: bool) {
    let coll = w.class(CA44).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    let most = m.pv_s(0x24) as f32;
    m.set_pv_f(0x20, most);
    m.drawn = 1;
    m.mode = (m.mode & !3) | mode::TARGET;
    m.has_collision = coll;
    m.drops = 0;
    m.set_pv_b(0x2e, 1);
    m.pos = PEN_A44;
    if w.class(GUN).is_some() {
        let g = w.create(GUN);
        let (ud, dd, light, amb) = { let m = &w.mobys[id]; (m.update_dist, m.draw_dist, m.light, m.ambient) };
        let x = &mut w.mobys[g];
        (x.update_dist, x.draw_dist, x.drawn, x.light, x.ambient) = (ud, dd, 1, light, amb);
        x.has_collision = false;
        x.mode = (x.mode | mode::NO_UPDATE) & !1;
        w.rebuild(g, ctx.host);
        ai::set_link(&mut w.mobys[id], GUN_LINK, Some(g));
    }
    let yaw = yaw_to(PEN_A44, ctx.hero.pos);
    w.mobys[id].rot[2] = yaw;
    if let Some(t) = turret(w, id) {
        let at = ctx.host.joint_matrix(&w.mobys[id], 0).map_or(PEN_A44, |(_, p)| p);
        let tc = w.class(TURRET).is_some_and(|c| c.has_collision);
        let x = &mut w.mobys[t];
        x.pos = at;
        x.rot[2] = yaw;
        x.mode &= !1;
        x.drawn = 1;
        x.has_collision = tc;
        w.rebuild(t, ctx.host);
    }
    let m = &mut w.mobys[id];
    if wait {
        m.set_state(4, 0);
    } else {
        m.set_state(2, 0);
        m.set_pv_i(PARTNER_A44, 0);
    }
    let n = ctx.game.rng.randf(600.0, 1200.0) as i32;
    let m = &mut w.mobys[id];
    m.set_pv_s(0x288, n as i16);
    m.set_pv_s(0x280, 0);
    m.set_pv_s(0x282, 0);
    if m.pv_s(0x140) != 0 {
        m.set_pv_s(0x140, 0);
        m.ambient = [m.pv_i(0x270) as u8, m.pv_i(0x274) as u8, m.pv_i(0x278) as u8];
        m.set_pv_s(0x142, 1);
    }
    w.changed = true;
}

/// `0x3dcc40`: back to the pool — state 1, its spin's sound stopped, sequence 0, hidden, no updates; the turret
/// hidden, its gun deleted.
pub fn pool_0a44(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_state(1, 0);
    stop_spin_sound(w, id, ctx);
    ctx.host.set_anim(&w.mobys[id], 0, 0, 3);
    let m = &mut w.mobys[id];
    m.drawn = 0;
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 3;
    if let Some(t) = turret(w, id) {
        let x = &mut w.mobys[t];
        x.mode |= 1;
        x.drawn = 0;
        x.has_collision = false;
    }
    if let Some(g) = gun(w, id) { w.delete(g); }
    ai::set_link(&mut w.mobys[id], GUN_LINK, None);
    w.changed = true;
}

/// `0x3da708`: set-up; the hits (from state 5 on, not dying: the bomb and the shot 0xbfe / 0xc89 do 0.35; a hit of more
/// than 3 while it spins staggers the turret, 9, its state, timer and the turret's sequence kept; its health out, 10),
/// its turret's hits dropped, its gun at the turret's joint 4, its target, back to the pool when told, then its move
/// ([`move_a44`]) and its turret ([`turret_a44`]).
fn senses_0a44(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 24);
        m.set_pv_f(0x124, 0.5);
        m.set_pv_f(0x5c, 1.0);
        m.set_pv_i(0x88, 2);
        m.set_pv_f(0x60, f32::from_bits(0x3f91_361f));
        m.set_pv_f(0x80, f32::from_bits(0x3f06_0a93));
        m.set_pv_f(0x84, f32::from_bits(0x3c23_d70a));
        m.set_pv_f(0x7c, f32::from_bits(0x3c63_8e39));
        m.set_pv_f(0x30, 11.0);
        m.set_pv_f(0x54, 1.0);
        m.set_pv_f(0x58, 1.0);
        m.set_pv_b(0x139, 1);
        m.set_pv_b(0x28, 3);
        m.set_pv_b(0x29, 0);
        m.set_pv_f(0x50, 0.0);
        m.set_pv_s(0x286, 0x3c);
        let a = m.ambient;
        m.set_pv_i(0x270, a[0] as i32);
        m.set_pv_i(0x274, a[1] as i32);
        m.set_pv_i(0x278, a[2] as i32);
        if let Some(t) = turret(w, id) { w.mobys[t].mode |= 6; }
        pool_0a44(w, id, ctx);
        if let Some(t) = turret(w, id) { w.mobys[t].parent = Some(id); }
        let m = &mut w.mobys[id];
        for o in [LIGHT + 4, LIGHT, LIGHT + 8] { m.set_pv_i(o, LIGHT_IDLE as i32); }
        m.set_pv_s(0x282, 0);
        m.set_pv_i(PARTNER_A44, 0);
        m.set_pv_s(0x280, 0);
        return;
    }
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
    let mut dmg = ai::damage_of(w, id, hit.as_ref(), false);
    if hit.and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == 0x0c89 || s.o_class == 0x0bfe) { dmg *= 0.35; }
    let t = turret(w, id);
    let s = w.mobys[id].state;
    if 0.0 < dmg && 5 <= s && s != 10 && s != 0xb {
        let m = &mut w.mobys[id];
        m.set_pv_f(0x20, m.pv_f(0x20) - dmg);
        if m.pv_s(0x140) == 0 { ai::flash_start(m, 0x140); }
        if 3.0 < dmg && s == 6 && seq_a(w, id, ctx) == 7 {
            let ts = t.and_then(|t| ctx.host.anim(&w.mobys[t])).map_or(0, |a| if a.seq_b == 0xff { a.seq_a } else { a.seq_b });
            let m = &mut w.mobys[id];
            m.set_pv_s(0x28a, s as i16);
            m.set_pv_s(0x28c, ts as i16);
            m.set_pv_s(0x28e, m.timer as i16);
            m.set_state(9, -1);
            m.set_pv_f(0x24c, f32::from_bits(0x3e46_9468));
        }
        let m = &mut w.mobys[id];
        if m.pv_f(0x20) <= 0.0 {
            m.set_pv_f(0x24c, f32::from_bits(0x3ec6_9468));
            m.set_state(10, -1);
        }
    }
    w.clear_hit(id);
    if let Some(t) = t { w.clear_hit(t); }
    if let (Some(t), Some(g)) = (t, gun(w, id)) { place_part(w, t, g, 4, ctx); }
    let area = w.mobys[id].pv_i(0x268);
    let found = ai::find_target(w, id, ctx, 100.0, 100.0, area, None, false);
    let m = &mut w.mobys[id];
    m.set_pv_s(0x284, found as i16);
    let tl = m.pv_i(0x160);
    m.set_pv_i(TARGET_A44, tl);
    if w.mobys[id].state != 3 {
        let p = target_a44(w, id, ctx);
        w.mobys[id].set_pv_v(GOAL_A44, p);
    }
    if w.mobys[id].pv_b(0x2e) == 2 {
        hand_over(w, id);
        pool_0a44(w, id, ctx);
    }
    move_a44(w, id, ctx);
    turret_a44(w, id, ctx);
}

/// The middle (the master's cuboid `+0x88`, `0x3fd638`).
fn middle(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<V3> { master_of(w, id).and_then(|ms| super::fighters::middle(w, ms, ctx)) }

/// `0x3dc0d0` (sequence 7): its legs (joints 1–2, 3–4, 5–6, 7–8) hit what they sweep — a sphere of 0.65 at each leg's
/// point nearest its target and 0.95 at its foot, for `+0x35`, pushed across its spin (flags 0x10001).
fn legs_hit(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let tgt = target_a44(w, id, ctx);
    let pos = w.mobys[id].pos;
    let dmg = w.mobys[id].pv_b(0x35) as f32;
    for (a, b) in [(1, 2), (3, 4), (5, 6), (7, 8)] {
        let (Some(pa), Some(pb)) = (ctx.host.joint(&w.mobys[id], a), ctx.host.joint(&w.mobys[id], b)) else { continue };
        let c = closest(pa, pb, tgt);
        let d = sub(c, pos);
        let dir = [-d[1] * 2.0, d[0] * 2.0, 1.0, 5_627.987];
        let mut t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x1_0001, dir);
        t.kind = (CA44 as u16 as u32) << 16 | 1 << 8;
        w.sphere_hit(ctx.scene, c, f32::from_bits(0x3f26_6666), 0, Some(id), Some(&t));
        w.sphere_hit(ctx.scene, pb, f32::from_bits(0x3f73_3333), 0, Some(id), Some(&t));
    }
}

/// `0x3dac90`: its move by its sequence (the turret out of the collision meanwhile): 0 in its pen eased in (state 4) or
/// still; 1 walking at its target in the floor's cylinder (else the middle); 2 turning in place on its planted foot
/// (joint 4 over frames 12..25, joint 8 over 42..55); 3 (from frame 23) and 4 its jump; 7 the spin (unless dying: its
/// yaw `160 − 90·health` degrees a second, the legs' blow, drifting at its target by `3 − 2.2·health` a second across
/// when more than 12 away, at the middle by 20 a second when out of the floor's cylinder). The footfalls shake the
/// camera. It stays on the middle's height or above.
fn move_a44(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let goal = w.mobys[id].pv_v(GOAL_A44);
    let pos = w.mobys[id].pos;
    let d = dist2(pos, goal);
    let yaw_g = yaw_to(pos, goal);
    let ft = frame(w, id, ctx);
    let t = turret(w, id);
    let keep = t.map(|t| std::mem::replace(&mut w.mobys[t].has_collision, false));
    ai::senses(&mut w.mobys[id], f32::from_bits(0x3ada_740e), f32::from_bits(0x3d53_a06e), f32::from_bits(0x3b64_c38a), f32::from_bits(0x3dd6_7752));
    let mid = middle(w, id, ctx);
    let floor_cyl = master_of(w, id).map_or(-1, |ms| w.mobys[ms].pv_i(0x84));
    let within = |w: &MobyWorld, ctx: &Ctx| ctx.level.in_cylinder(floor_cyl, w.mobys[id].pos);
    match seq_a(w, id, ctx) {
        0 => {
            if w.mobys[id].state == 4 {
                w.mobys[id].pos = approach_v(pos, PEN_A44, f32::from_bits(0x3e2a_aaab));
            } else {
                let r = w.mobys[id].rot[2];
                ai::move_step(w, id, ctx, r, pos);
            }
        }
        1 => {
            let to = if within(w, ctx) { goal } else { mid.unwrap_or(goal) };
            let y = yaw_to(pos, to);
            ai::move_step(w, id, ctx, y, to);
            if (0.0 < ft && ft < 2.0) || (15.0 < ft && ft < 17.0) || (30.0 < ft && ft < 32.0) || (45.0 < ft && ft < 47.0) { shake(ctx, 0x3d4c_cccd, 0xf); }
        }
        2 => {
            let foot = if 12.0 < ft && ft < 25.0 { Some(4) } else if 42.0 < ft && ft < 55.0 { Some(8) } else { None };
            if let Some(j) = foot {
                let a = ctx.host.joint(&w.mobys[id], j);
                ai::move_step(w, id, ctx, yaw_g, pos);
                let m = &mut w.mobys[id];
                m.rows = crate::moby::attach::rows_of(m.rot);
                if let (Some(a), Some(b)) = (a, ctx.host.joint(&w.mobys[id], j)) { w.mobys[id].pos = add(w.mobys[id].pos, sub(a, b)); }
            }
            if (0.0 < ft && ft < 2.0) || (29.0 < ft && ft < 31.0) { shake(ctx, 0x3dcc_cccd, 0xf); }
        }
        s @ (3 | 4) => {
            if s == 4 || 23.0 <= ft {
                let m = &mut w.mobys[id];
                let v = m.pv_v(0xf0);
                let heading = atan2(v[0], v[1]);
                let (mut a, mut sv) = (m.rot[2], m.pv_f(0x104));
                let (ta, tb, tm) = (m.pv_f(0x64), m.pv_f(0x68), m.pv_f(0x6c));
                turn_ease(heading, ta, tb, tm, &mut a, &mut sv, 0);
                m.rot[2] = a;
                m.set_pv_f(0x104, sv);
                let vz = m.pv_f(0xf8) - m.pv_f(0x7c);
                m.set_pv_f(0xf8, vz);
                let v = m.pv_v(0xf0);
                m.pos = add(m.pos, v);
            }
        }
        5 => if 3.0 < ft && ft < 5.0 { shake(ctx, 0x3ecc_cccd, 0x19) },
        6 => if 20.0 < ft && ft < 22.0 { shake(ctx, 0x3e4c_cccd, 0x19) },
        7 => {
            let st = w.mobys[id].state;
            if !(10..=11).contains(&st) {
                let m = &mut w.mobys[id];
                let f = m.pv_f(0x20) / m.pv_s(0x24) as f32;
                m.rot[2] = add_rot(m.rot[2], (f * -90.0 + 160.0) * 0.000_290_888_25);
                m.rows = crate::moby::attach::rows_of(m.rot);
                legs_hit(w, id, ctx);
                if 12.0 < d && st != 10 {
                    let m = &mut w.mobys[id];
                    if !ctx.level.in_cylinder(floor_cyl, m.pos) {
                        if let Some(c) = mid {
                            approach(c[0], 1.0 / 3.0, &mut m.pos[0]);
                            approach(c[1], 1.0 / 3.0, &mut m.pos[1]);
                        }
                    } else {
                        let s = (f * -2.2 + 3.0) * 0.016_666_668;
                        approach(goal[0], s, &mut m.pos[0]);
                        approach(goal[1], s, &mut m.pos[1]);
                    }
                }
            }
        }
        8 => if 30.0 < ft && ft < 32.0 { shake(ctx, 0x3e4c_cccd, 0x19) },
        _ => {}
    }
    if let Some(c) = mid {
        if w.mobys[id].pos[2] < c[2] { w.mobys[id].pos[2] = c[2]; }
    }
    if let (Some(t), Some(k)) = (t, keep) { w.mobys[t].has_collision = k; }
}

/// `0x321f68(t, a, b)`: colour `a` to `b` by `t`.
fn lerp_colour(t: f32, a: u32, b: u32) -> u32 {
    let ch = |s: u32| { let (x, y) = (((a >> s) & 0xff) as f32, ((b >> s) & 0xff) as f32); ((x + (y - x) * t) as u32 & 0xff) << s };
    ch(0) | ch(8) | ch(16) | ch(24)
}

/// `0x3db3d0`: the turret on its joint 0; its light faded to its state's colour (by 0.1 a tick) and pulsing (every two
/// seconds, the colour's alpha × sin²); unless staggered or dying it turns to the target (its turn's limits by its
/// health: `360 − 330·h` arc-seconds a tick², at most `240 − 225·h` degrees a second); walking (or spinning under a
/// fifth of its health), every `20 + 40·h` ticks (60 spinning) within 45 the turret fires (its sequence 5: frames 3..6,
/// a shot from its joints 0 and 1 each, [`spawn_shot`]); its sequences pick the light (5 firing, 7 its taunt's spin,
/// 10 the spin, 0xc / 0xd staggered or dead) and the turret's spin after a stagger (`+0x24c`, slowing 3% a tick).
fn turret_a44(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let tgt = target_a44(w, id, ctx);
    let pos = w.mobys[id].pos;
    let d = dist2(pos, tgt);
    let yaw_t = yaw_to(pos, tgt);
    let Some(t) = turret(w, id) else { return };
    let off = rot_dist(w.mobys[t].rot[2], yaw_t);
    if let Some((_, at)) = ctx.host.joint_matrix(&w.mobys[id], 0) {
        w.mobys[t].pos = at;
        w.rebuild(t, ctx.host);
    }
    {
        let m = &mut w.mobys[id];
        let (cur, from, to) = (m.pv_i(LIGHT) as u32, m.pv_i(LIGHT + 4) as u32, m.pv_i(LIGHT + 8) as u32);
        let mut cur = cur;
        if cur & 0xff_ffff == to & 0xff_ffff {
            m.set_pv_i(LIGHT + 4, to as i32);
            m.set_pv_f(LIGHT + 0xc, 0.0);
        } else {
            let k = m.pv_f(LIGHT + 0xc);
            cur = lerp_colour(k, from, to);
            m.set_pv_f(LIGHT + 0xc, (k + 0.1).min(1.0));
        }
        let s = ((ctx.game.frame % 0x78) as f32 * 0.052_359_883 - std::f32::consts::PI).sin();
        let cur = (cur & 0xff_ffff) | (((s * s * (to >> 24) as f32) as i32 as u32) << 24);
        m.set_pv_i(LIGHT, cur as i32);
        w.mobys[t].glow = cur;
    }
    let st = w.mobys[id].state;
    let h = { let m = &w.mobys[id]; m.pv_f(0x20) / m.pv_s(0x24) as f32 };
    if !(9..=11).contains(&st) && st != 7 {
        let m = &mut w.mobys[id];
        let acc = (h * -330.0 + 360.0) * 4.848_137_5e-6;
        let (mut a, mut v) = (m.pv_f(0x248), m.pv_f(0x24c));
        turn_ease(yaw_t, acc, acc, (h * -225.0 + 240.0) * 0.000_290_888_25, &mut a, &mut v, 0);
        m.set_pv_f(0x248, a);
        m.set_pv_f(0x24c, v);
        w.mobys[t].rot = [0.0, 0.0, a];
    }
    if st == 5 || (st == 6 && h < 0.2) {
        if countdown_s(w, id, 0x286) && d < 45.0 && 0.0 < d && off < 3.141_593 {
            set_seq(w, t, ctx, 5, 3);
            w.mobys[t].timer = 0;
            let n = if st == 5 { (h * 40.0 + 20.0) as i16 } else { 0x3c };
            w.mobys[id].set_pv_s(0x286, n);
        }
    }
    let spin = |w: &mut MobyWorld| {
        let m = &mut w.mobys[id];
        let a = add_rot(m.pv_f(0x248), m.pv_f(0x24c));
        m.set_pv_f(0x248, a);
        m.set_pv_f(0x24c, m.pv_f(0x24c) * 0.97);
        w.mobys[t].rot = [0.0, 0.0, a];
    };
    let set_light = |w: &mut MobyWorld, c: u32| w.mobys[id].set_pv_i(LIGHT + 8, c as i32);
    match seq_a(w, t, ctx) {
        0 => set_light(w, LIGHT_IDLE),
        1..=4 => {
            if 0.0 < d && d < 45.0 && off < 3.141_593 && seq(w, t, ctx) != 0 { ctx.host.set_anim(&w.mobys[t], 0, 0, 10); }
        }
        5 => {
            set_light(w, LIGHT_FIRE);
            let tf = frame(w, t, ctx);
            if w.mobys[t].timer == 0 && 3.0 < tf && tf < 6.0 {
                let dmg = w.mobys[id].pv_b(0x34) as f32;
                for j in [0, 1] {
                    let (Some(p), Some((r, _))) = (ctx.host.joint(&w.mobys[t], j), ctx.host.joint_matrix(&w.mobys[t], j)) else { continue };
                    let v = set_len(sub(tgt, p), 1.0);
                    let dir = set_len([r[0][0], r[0][1], v[2]], 1.0);
                    spawn_shot(w, ctx, dmg, p, id, dir);
                }
                w.mobys[t].timer = 1;
            }
            if wrapped(w, t, ctx) && seq(w, t, ctx) != 0 { ctx.host.set_anim(&w.mobys[t], 0, 0, 3); }
        }
        7 => {
            if st == 7 {
                set_light(w, LIGHT_TAUNT);
                spin(w);
            }
        }
        10 => set_light(w, LIGHT_SPIN),
        0xc | 0xd => {
            set_light(w, LIGHT_DEAD);
            if st != 0xb {
                set_light(w, LIGHT_HIT);
                spin(w);
            }
        }
        _ => {}
    }
}

/// `0x3dab70`: its colour flash (`+0x140`) and, once over, its own colour back (`+0x270..`); the turret lit as it is;
/// its draw; its shadow (within 64, range 0x3c); the ticks.
fn tail_0a44(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    ai::flash(m, 0x140);
    if m.pv_s(0x140) == 0 {
        if m.pv_s(0x27e) == 0 {
            m.ambient = [m.pv_i(0x270) as u8, m.pv_i(0x274) as u8, m.pv_i(0x278) as u8];
            m.set_pv_s(0x27e, 1);
        }
    } else {
        m.set_pv_s(0x27e, 0);
    }
    let a = m.ambient;
    if let Some(t) = turret(w, id) { w.mobys[t].ambient = a; }
    w.register_draw(id);
    crate::shadow_volumes::probe_near(w, id, ctx, 64.0, 0x3c);
    let m = &mut w.mobys[id];
    m.shadow_slab[1] = 1.0;
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

/// 0x0a44 (`0x3d9188`; module docs).
pub fn update_0a44(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    senses_0a44(w, id, ctx);
    let tgt = target_a44(w, id, ctx);
    let pos = w.mobys[id].pos;
    let d = dist2(pos, tgt);
    let yaw_t = yaw_to(pos, tgt);
    let ft = frame(w, id, ctx);
    let t = turret(w, id);
    let tw = |w: &MobyWorld, ctx: &Ctx| t.is_some_and(|t| wrapped(w, t, ctx));
    // `+0x290`: Ratchet hurt (his movement group 7), or hit by it or its shot, since its jump in: no skill point.
    let by = ctx.hero.damage.hit_by;
    if ctx.hero.group == 7 || by == Some(id) || by.and_then(|b| w.mobys.get(b)).is_some_and(|b| b.o_class == SHOT) {
        w.mobys[id].set_pv_s(HURT_A44, 1);
    }
    match w.mobys[id].state {
        2 => {
            w.mobys[id].set_pv_s(HURT_A44, 0);
            // The jump in: a second still, then (sequence 3) up and 30 across toward the middle down onto its height;
            // landed (5), a thud and the camera's shake, then its walk.
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                w.mobys[id].state_ticks = 0;
                ctx.host.set_anim(&w.mobys[id], 0, 0, 3);
                tset(w, t, ctx, 0, 10);
            } else {
                let tm = w.mobys[id].timer;
                if tm == 0 && 0x3c < w.mobys[id].state_ticks as u16 {
                    w.mobys[id].timer = 1;
                    set_seq(w, id, ctx, 3, 10);
                    tset(w, t, ctx, 6, 10);
                    let z = ai::ground_z(ctx, 10.0, w.mobys[id].pos, 0);
                    if z != 0.0 { w.mobys[id].pos[2] = z; }
                    if let Some(c) = middle(w, id, ctx) {
                        let p = w.mobys[id].pos;
                        let mut v = sub(c, p);
                        v[2] = 0.0;
                        let mut goal = add(set_len(v, 30.0), p);
                        goal[2] = c[2];
                        let g = w.mobys[id].pv_f(0x7c);
                        jump(w, id, f32::from_bits(0x3eaa_aaab), g, goal);
                    }
                } else if tm == 1 && wrapped(w, id, ctx) {
                    w.mobys[id].timer = 2;
                    set_seq(w, id, ctx, 4, 7);
                    tset(w, t, ctx, 7, 10);
                } else if tm == 2 && middle(w, id, ctx).is_some_and(|c| w.mobys[id].pos[2] < c[2] + 0.05) {
                    w.mobys[id].set_pv_v(0xf0, [0.0; 3]);
                    w.mobys[id].timer = 3;
                    w.mobys[id].set_pv_s(0x292, 0);
                    if seq(w, id, ctx) != 5 { ctx.host.set_anim(&w.mobys[id], 5, 3, 2); }
                    tset(w, t, ctx, 8, 10);
                } else if tm == 3 {
                    if 2.0 < frame(w, id, ctx) && w.mobys[id].pv_s(0x292) == 0 {
                        w.mobys[id].set_pv_s(0x292, 1);
                        ctx.host.play_sound(&w.mobys[id], 0, 0);
                    }
                    if wrapped(w, id, ctx) {
                        shake(ctx, 0x3eb3_3333, 0x19);
                        w.mobys[id].set_pv_v(0xf0, [0.0; 3]);
                        if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
                        tset(w, t, ctx, 0, 0);
                        w.mobys[id].set_state(5, 1);
                        let n = ctx.game.rng.randf(600.0, 1200.0) as i16;
                        w.mobys[id].set_pv_s(0x288, n);
                    }
                }
            }
        }
        3 => {
            // Handing over: to the gate, a jump back into the pen, its partner out; then it waits (4).
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                w.mobys[id].set_pv_v(GOAL_A44, GATE_A44);
                set_seq(w, id, ctx, 1, 10);
                w.mobys[id].set_pv_s(0x280, 0);
                if let Some(p) = ai::link(&w.mobys[id], PARTNER_A44).filter(|&p| w.mobys.get(p).is_some()) {
                    call_back(w, p, 0x318, 0x31a);
                    let ms = master_of(w, id);
                    boss_is(w, ms, p);
                }
                w.mobys[id].set_pv_v(0xf0, [0.0; 3]);
                w.mobys[id].state_ticks = 0;
            } else {
                let tm = w.mobys[id].timer;
                if tm == 0 && dist2(w.mobys[id].pos, w.mobys[id].pv_v(GOAL_A44)) < 0.01 {
                    w.mobys[id].timer = 1;
                    set_seq(w, id, ctx, 3, 10);
                    tset(w, t, ctx, 6, 10);
                    w.mobys[id].set_pv_v(0xf0, [0.0; 3]);
                    let g = w.mobys[id].pv_f(0x7c);
                    jump(w, id, f32::from_bits(0x3eaa_aaab), g, PEN_A44);
                } else if tm == 1 && wrapped(w, id, ctx) {
                    w.mobys[id].timer = 2;
                    w.mobys[id].state_ticks = 0;
                    set_seq(w, id, ctx, 4, 10);
                    tset(w, t, ctx, 7, 10);
                } else if tm == 2 {
                    if !(1.0 <= dist(w.mobys[id].pos, PEN_A44) && (w.mobys[id].state_ticks as u16) < 0x12d) {
                        w.mobys[id].timer = 3;
                        set_seq(w, id, ctx, 5, 10);
                        tset(w, t, ctx, 8, 10);
                    }
                } else if tm == 3 && wrapped(w, id, ctx) {
                    w.mobys[id].set_state(4, -1);
                }
            }
        }
        4 => {
            // In the pen until called back.
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 0, 10);
                tset(w, t, ctx, 0, 10);
            }
            let p = approach_v(w.mobys[id].pos, PEN_A44, f32::from_bits(0x3e2a_aaab));
            w.mobys[id].pos = p;
            if w.mobys[id].pv_s(0x280) == 1 && dist2(p, PEN_A44) < 0.001 { w.mobys[id].set_state(2, 1); }
        }
        5 => {
            // Walking at its target (the timer: 0 still within 10, 1 walking from 20, 2 turning past 45° until 10°).
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 1, 0xf);
                tset(w, t, ctx, 1, 10);
                w.mobys[id].timer = 1;
            }
            let off = rot_dist(yaw_t, w.mobys[id].rot[2]);
            if w.mobys[id].timer == 1 && 0.785_398_24 < off {
                w.mobys[id].timer = 2;
                set_seq(w, id, ctx, 2, 0xf);
                tset(w, t, ctx, 3, 10);
            }
            if w.mobys[id].timer == 2 && off < 0.174_532_95 {
                w.mobys[id].timer = 1;
                set_seq(w, id, ctx, 1, 0xf);
                tset(w, t, ctx, 1, 10);
            }
            if w.mobys[id].timer == 1 && d < 10.0 {
                w.mobys[id].timer = 0;
                set_seq(w, id, ctx, 0, 0xf);
            }
            if w.mobys[id].timer == 0 && 20.0 < d {
                w.mobys[id].timer = 1;
                set_seq(w, id, ctx, 1, 10);
            }
            if ctx.hero.damage.hit_by.is_some() { w.mobys[id].set_state(7, -1); }
            if countdown_s(w, id, 0x288) {
                let n = ctx.game.rng.randf(600.0, 1200.0) as i16;
                w.mobys[id].set_pv_s(0x288, n);
                w.mobys[id].set_state(6, -1);
            }
            if hand_over_due(w, id, PARTNER_A44, 0x282) { w.mobys[id].set_state(3, 0); }
        }
        6 => {
            // The spin: wound up (sequence 6, to frame 28), spinning (7, its loop sound) until its count runs out or it
            // hands over, then unwound (8, to frame 45) and back to its walk.
            let mut hurt_check = true;
            if w.mobys[id].first() {
                if seq_a(w, id, ctx) == 7 {
                    w.mobys[id].timer = 1;
                    tset(w, t, ctx, 10, 10);
                } else {
                    if seq_a(w, id, ctx) != 6 {
                        set_seq(w, id, ctx, 6, 0);
                        tset(w, t, ctx, 9, 10);
                    }
                    w.mobys[id].timer = 0;
                    w.mobys[id].mode &= !mode::TARGET;
                }
            } else if w.mobys[id].timer == 0 && 28.0 < ft {
                set_seq(w, id, ctx, 7, 0);
                tset(w, t, ctx, 10, 10);
                let m = &mut w.mobys[id];
                m.timer = 1;
                m.set_pv_f(0x30, 1.5);
                m.mode |= mode::TARGET;
                let h = m.pv_i(SPIN_SOUND);
                if !ctx.host.sound_playing(h) {
                    let h = ctx.host.play_sound(&w.mobys[id], 6, 4);
                    w.mobys[id].set_pv_i(SPIN_SOUND, h);
                }
            } else if w.mobys[id].timer == 1 {
                let over = countdown_s(w, id, 0x288);
                if over || hand_over_due(w, id, PARTNER_A44, 0x282) {
                    set_seq(w, id, ctx, 8, 0);
                    tset(w, t, ctx, 0xb, 10);
                    w.mobys[id].timer = 2;
                    w.mobys[id].mode &= !mode::TARGET;
                }
            }
            if w.mobys[id].timer == 1 && ctx.hero.damage.hit_by.is_some() {
                w.mobys[id].set_state(7, -1);
                hurt_check = false;
            }
            if hurt_check && w.mobys[id].timer == 2 && 45.0 < ft {
                let n = ctx.game.rng.randf(600.0, 1200.0) as i16;
                let m = &mut w.mobys[id];
                m.set_pv_f(0x30, 11.0);
                m.mode |= mode::TARGET;
                m.set_pv_s(0x288, n);
                m.set_state(5, -1);
                stop_spin_sound(w, id, ctx);
            }
        }
        7 => {
            // Ratchet hurt: the turret's taunt (spinning it), three seconds, then the spin.
            if w.mobys[id].first() {
                tset(w, t, ctx, 7, 0xf);
                let m = &mut w.mobys[id];
                m.set_pv_f(0x24c, m.pv_f(0x24c) + 0.193_925_5);
                m.state_ticks = 0;
            } else if 0xb5 <= w.mobys[id].state_ticks as u16 {
                w.mobys[id].set_state(6, -1);
            }
        }
        8 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 8, 0x14);
                tset(w, t, ctx, 9, 0xf);
                let m = &mut w.mobys[id];
                m.set_pv_f(0x24c, m.pv_f(0x24c) + 0.193_925_5);
            } else if 45.0 < ft {
                let s = w.mobys[id].pv_s(0x28a);
                w.mobys[id].set_state(s as u8, -1);
            }
        }
        9 => {
            // Staggered: the turret's sequence 0xc, then back as it was.
            if w.mobys[id].first() { tset(w, t, ctx, 0xc, 0xf); }
            if tw(w, ctx) {
                let m = &mut w.mobys[id];
                m.state = m.pv_s(0x28a) as u8;
                m.timer = m.pv_s(0x28e) as u8;
                let s = m.pv_s(0x28c) as u8;
                tset(w, t, ctx, s, 0xf);
            }
        }
        10 => {
            // Dying: the turret's death sequence (0xd) once its own is through, then 0xb.
            if w.mobys[id].first() {
                stop_spin_sound(w, id, ctx);
                if seq_a(w, id, ctx) == 7 {
                    w.mobys[id].timer = 1;
                } else {
                    w.mobys[id].timer = 0;
                    if seq_a(w, id, ctx) != 6 && seq(w, id, ctx) != 9 { ctx.host.set_anim(&w.mobys[id], 9, 0, 0xf); }
                }
                tset(w, t, ctx, 0xd, 0xf);
            } else if w.mobys[id].timer == 0 && wrapped(w, id, ctx) {
                w.mobys[id].timer = 1;
                tset(w, t, ctx, 0xd, 0xf);
                if seq(w, id, ctx) != 7 { ctx.host.set_anim(&w.mobys[id], 7, 0, 10); }
            } else if w.mobys[id].timer == 1 && tw(w, ctx) {
                w.mobys[id].set_state(0xb, -1);
            }
        }
        0xb => {
            // Dead: still, its gun thrown at the camera, its bolts; after three seconds five blasts, its pieces and the
            // turret's thrown, back to the pool, its partner out. Ratchet unhurt: skill point 0x13 (`0x3da250`).
            if w.mobys[id].first() {
                if w.mobys[id].pv_s(HURT_A44) == 0 { ctx.game.award_skill_point(crate::skill_points::point::BOSS_A44); }
                ctx.host.set_anim_speed(&w.mobys[id], 0.0);
                if let Some(t) = t { ctx.host.set_anim_speed(&w.mobys[t], 0.0); }
                w.mobys[id].state_ticks = 0;
                let tp = t.map_or(w.mobys[id].pos, |t| w.mobys[t].pos);
                let mut v = set_len(sub(ctx.host.camera_pos(), tp), ctx.game.rng.randf(f32::from_bits(0x3daa_aaab), f32::from_bits(0x3dee_eef0)));
                v[2] += 0.066_666_67;
                if let Some(g) = gun(w, id) {
                    let (gp, gr) = (w.mobys[g].pos, w.mobys[g].rot);
                    crate::moby::shards::spawn_whole(w, ctx, GUN, Some(g), gp, gr, 0.0, f32::from_bits(0x3ab6_0b61), v, 0xff);
                    w.delete(g);
                    ai::set_link(&mut w.mobys[id], GUN_LINK, None);
                    ctx.host.play_sound_of(&w.mobys[id], CA44, 5, 0);
                }
                crate::moby::bolts::drop(w, id, 0, 3, ctx);
                w.mobys[id].set_pv_b(0x1f2, 1);
                w.mobys[id].mode &= !mode::TARGET;
            }
            if 0xb4 < w.mobys[id].state_ticks as u16 {
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                if let Some(t) = t { ctx.host.set_anim_speed(&w.mobys[t], 1.0); }
                use crate::moby::blast::{burst, Blast};
                let p = w.mobys[id].pos;
                burst(w, ctx, p, &Blast { size: 2.0, bursts: [0x19, 0x14, 0x1e], puffs: 0x28, streaks: 0x2d, light: true, shake: true });
                for (j, b) in [(2, [10, 5, 10, 5, 0x14]), (6, [0, 10, 0x14, 0xf, 0x2d]), (4, [0x14, 0, 0xf, 10, 0x1e]), (8, [5, 0xf, 5, 0x19, 0x19])] {
                    let at = ctx.host.joint(&w.mobys[id], j).unwrap_or(p);
                    burst(w, ctx, at, &Blast { size: f32::from_bits(0x3f33_3333), bursts: [b[0], b[1], b[2]], puffs: b[3], streaks: b[4], light: true, shake: false });
                }
                let v = ctx.game.rng.rand_vec(f32::from_bits(0x3dcc_ccce), f32::from_bits(0x3e4c_ccce));
                crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0xfff, v, 0xff);
                if let Some(t) = t {
                    let v = ctx.game.rng.rand_vec(f32::from_bits(0x3e08_8889), f32::from_bits(0x3e88_8889));
                    crate::moby::shards::break_into(w, t, ctx, (3.0, 6.0), (3.0, 6.0), 0x1f, v, 0xff);
                }
                ctx.host.play_sound_of(&w.mobys[id], CA44, 5, 0);
                pool_0a44(w, id, ctx);
                w.mobys[id].set_pv_b(0x1f2, 0);
                hand_over(w, id);
            }
        }
        _ => {}
    }
    tail_0a44(w, id, ctx);
}

/// `0x3dc2d0` (0x0a44, every tick: on its turret, in its light's colour): lights at the turret's joints 0 and 1 (1.6
/// out along the joint's first row: 0.4 across, 0.6 toward the camera, faded by how much they face it, over 0.3), at
/// joints 5 and 6 (0.5, 0.2), and two at joint 4 (`(−1.9, 2.2, 0)`: 1.5, 0.5; `(2.3, 0, −0.45)`: 1.0, 0.8, faced).
pub fn draw_0a44(w: &MobyWorld, host: &dyn MobyHost, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let Some(t) = turret(w, id) else { return };
    let c = w.mobys[id].pv_i(LIGHT) as u32;
    let tm = &w.mobys[t];
    let at = |j: usize, o: V3| -> Option<(V3, [V3; 3])> {
        let (p, (r, _)) = (host.joint(tm, j)?, host.joint_matrix(tm, j)?);
        Some((add(p, in_frame(&r, o)), r))
    };
    let mut faced = |j: usize, o: V3, size: f32, push: f32| {
        let Some((p, r)) = at(j, o) else { return };
        let f = dot(set_len(sub(cam, p), 1.0), r[0]);
        if 0.3 < f { glow(out, size, push, p, cam, faded(c, f)); }
    };
    faced(0, [1.6, 0.0, 0.0], 0.4, 0.6);
    faced(1, [1.6, 0.0, 0.0], 0.4, 0.6);
    for j in [5, 6] {
        if let Some((p, _)) = at(j, [0.0; 3]) { glow(out, 0.5, 0.2, p, cam, c); }
    }
    if let Some((p, _)) = at(4, [-1.9, 2.2, 0.0]) { glow(out, 1.5, 0.5, p, cam, c); }
    let Some((p, r)) = at(4, [2.3, 0.0, -0.45]) else { return };
    let f = dot(set_len(sub(cam, p), 1.0), r[0]);
    if 0.3 < f { glow(out, 1.0, 0.8, p, cam, faded(c, f)); }
}

// ---------------------------------------------------------------------------------------------------------------
// The shot 0x0d08

/// `$gp+0x2ff0` / `+0x2ff4`: its speed (a second) and its range; `+0x2fe8` its size; `+0x3008` its sparkles' fall;
/// `+0x3018` / `+0x301c` their colours; `+0x302c` / `+0x3030` its glow's.
const SHOT_SPEED: f32 = 40.0;
const SHOT_RANGE: f32 = 100.0;
const SHOT_SIZE: f32 = 3.0;
const SPARK_FALL: u32 = 0x3c03_126f;
const SPARK_PINK: u32 = 0x607f_207f;
const SPARK_WHITE: u32 = 0x7fff_ffff;
const SHOT_PINK: u32 = 0x007f_207f;
const SHOT_WHITE: u32 = 0x00ff_ffff;

/// `0x422ec8(damage, p, owner, dir)`: a shot at `p` along `dir` (40 a second, `+0x40`) for 100 (its life `+0x58`, its
/// share a tick `+0x5c`), its owner (`+0x50`), damage (`+0x64`) and size (`+0x60`); its spin (`0x423ca0`: an offset
/// a quarter across `+0x00`, its angle `+0x30` at random); its sound 1.
pub fn spawn_shot(w: &mut MobyWorld, ctx: &mut Ctx, damage: f32, p: V3, owner: usize, dir: V3) -> Option<usize> {
    w.class(SHOT)?;
    let s = w.create(SHOT);
    let vel = set_len(dir, SHOT_SPEED * 0.016_666_668);
    let life = (SHOT_RANGE / (SHOT_SPEED * 0.016_666_668)) as i32;
    let a = ctx.game.rng.rand_angle();
    let x = &mut w.mobys[s];
    x.state = 0;
    x.update_dist = 0xff;
    x.pos = p;
    x.set_pv_f(0x64, damage);
    x.set_pv_v(0x40, vel);
    x.set_pv_i(0x58, life);
    x.set_pv_f(0x5c, 1.0 / life as f32);
    ai::set_link(x, 0x50, Some(owner));
    x.set_pv_f(0x60, SHOT_SIZE);
    x.set_pv_v(0, set_len(cross(add(p, vel), p), 0.25));
    x.set_pv_f(0x30, a);
    w.rebuild(s, ctx.host);
    ctx.host.play_sound(&w.mobys[s], 1, 0);
    w.changed = true;
    Some(s)
}

/// The sparkle pair of FX twinkles (pink, then a white one smaller) the shot leaves.
#[allow(clippy::too_many_arguments)]
fn sparkle(ctx: &mut Ctx, s0: f32, s1: f32, t0: f32, t1: f32, p: V3, life: i16, spin: i8, spin2: i8, v: V3) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let fall = f32::from_bits(SPARK_FALL);
    crate::particles::twinkle(sys, &mut ctx.game.rng, s0, s1, fall, p, life, SPARK_PINK, 0, spin, v, 0);
    crate::particles::twinkle(sys, &mut ctx.game.rng, t0, t1, fall, p, life, SPARK_WHITE, 1, spin2, v, 0);
}

fn spin_of(r: i32) -> i8 { if r == 0 { -1 } else { r as i8 } }

/// A way 0.025 long, its parts each `randf(−1, 1)` (z, y, x).
fn spark_way(ctx: &mut Ctx) -> V3 {
    let g = &mut ctx.game.rng;
    let z = g.randf(-1.0, 1.0);
    let y = g.randf(-1.0, 1.0);
    let x = g.randf(-1.0, 1.0);
    set_len([x, y, z], f32::from_bits(0x3ccc_cccd))
}

/// `0x423578(p, n, kind)`: `n` spark pairs off a hit (on the world 0, on a solid moby 1): flung up `randf(5, 10)` a
/// second and `randf(3, 6)` across within 60° of the way to the camera, half of them (by a coin) 1.5 times as fast and
/// the others thrown toward the camera too, their sizes and lives by the kind. Not ported: the particles' byte 9 the
/// original sets after (`0x70 + 4`).
fn impact(ctx: &mut Ctx, p: V3, n: i32, kind: u8) {
    let cam = ctx.host.camera_pos();
    let base = { let d = sub(cam, p); [d[0], d[1], 0.0] };
    for _ in 0..n {
        let g = &mut ctx.game.rng;
        let s = g.randf(1.0, 3.0);
        let spin = spin_of(g.randi(2));
        let a = g.randf(-f32::from_bits(0x3f86_0a93), f32::from_bits(0x3f86_0a93));
        let dir = turn_about(base, [0.0, 0.0, 1.0], a);
        let spd = g.randf(3.0, 6.0) * 0.016_666_668;
        let mut v = set_len(dir, spd);
        v[2] = g.randf(5.0, 10.0) * 0.016_666_668;
        let q = [p[0] + g.randf_signed(0.0, 0.15), p[1] + g.randf_signed(0.0, 0.15), p[2] + g.randf_signed(0.0, 0.15)];
        let coin = g.rand() & 1;
        let toward = |v: V3| add(v, set_len(sub(cam, q), spd));
        match (kind, coin) {
            (0, 0) => sparkle(ctx, s * 0.35, s, s * 0.245, s * 0.7, q, 0x3c, spin, -spin, scale(v, 1.5)),
            (0, _) => sparkle(ctx, s * 2.05, s, s * 1.435, s * 0.7, q, 0x28, spin, -spin, toward(v)),
            (_, 0) => sparkle(ctx, s * 0.3, s, s * 0.21, s * 0.7, q, 0x3c, spin, -spin, scale(v, 1.5)),
            _ => sparkle(ctx, s * 0.25, s, s * 0.175, s * 0.7, q, 0x28, spin, -spin, toward(v)),
        }
    }
}

/// 0x0d08 (`0x423038`; module docs): its step and the line along it; its spin's two points (`0x423d10`: its offset × its
/// size turned by `+0x30` about its way, at `+0x10` and opposite at `+0x20`; 48° a tick), drawn on the first list.
pub fn update_shot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let prev = w.mobys[id].pos;
    if w.mobys[id].state == 0 {
        let vel = w.mobys[id].pv_v(0x40);
        let new = add(prev, vel);
        w.mobys[id].pos = new;
        let owner = ai::link(&w.mobys[id], 0x50);
        if let Some(h) = w.coll_line(ctx, prev, new, 0, Some(id)).filter(|h| h.moby != owner || owner.is_none()) {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            let p = h.point;
            match h.moby {
                None => impact(ctx, p, 5, 0),
                Some(o) if w.mobys.get(o).is_some_and(|x| x.has_collision) => impact(ctx, p, 10, 1),
                _ => {}
            }
            let dmg = w.mobys[id].pv_f(0x64);
            let d = set_len(vel, 1.0);
            let mut t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x81_0001, [d[0], d[1], d[2], 5_627.987]);
            let oc = owner.and_then(|o| w.mobys.get(o)).map_or(0, |o| o.o_class);
            t.kind = (oc as u16 as u32) << 16 | 1 << 8 | 2;
            w.sphere_hit(ctx.scene, p, 0.8, 0x10, Some(id), Some(&t));
            w.delete(id);
            return;
        }
    }
    let mut life = w.mobys[id].pv_i(0x58);
    let over = crate::moby::countdown(&mut life);
    w.mobys[id].set_pv_i(0x58, life);
    let pos = w.mobys[id].pos;
    if !over {
        if life < 0x10 {
            w.mobys[id].set_pv_f(0x60, life as f32 / 15.0);
            if ctx.game.rng.rand() % 3 == 0 {
                let spin = spin_of(ctx.game.rng.randi(2));
                let s = ctx.game.rng.randf(1.0, 3.0);
                let v = spark_way(ctx);
                sparkle(ctx, s * 0.05, s, s * 0.0035, s * 0.035, pos, 0x3c, spin, spin, v);
            }
        }
        if pos.iter().all(|&c| (2.0..=1021.0).contains(&c)) {
            let m = &mut w.mobys[id];
            let way = set_len(m.pv_v(0x40), 1.0);
            let o = turn_about(scale(m.pv_v(0), m.pv_f(0x60)), way, m.pv_f(0x30));
            m.set_pv_v(0x10, add(pos, o));
            m.set_pv_v(0x20, sub(pos, o));
            m.set_pv_f(0x30, sub_rot(m.pv_f(0x30), 2880.0 * 0.000_290_888_25));
            w.register_draw_first(id);
            return;
        }
    } else {
        let s = ctx.game.rng.randf(1.0, 3.0);
        let mut spin = spin_of(ctx.game.rng.randi(2));
        for _ in 0..5 {
            let mut v = spark_way(ctx);
            v[2] = ctx.game.rng.randf(0.0, 0.1);
            sparkle(ctx, s * 0.35, s, s * 0.245, s * 0.7, pos, 0x3c, spin, spin, v);
            spin = -spin;
        }
    }
    w.delete(id);
}

/// `0x423e10` (the shot, first list): its glow (`0x424390`: FX 0xb, `randf(0.35, 0.45)` × its size, pink at alpha
/// 0x20..0x30, a random roll) and five sparks between its two points (`0x424540`: `randf(0.05, 0.15)` × its size,
/// pink then half as big white at alpha 0x40..0x7f, normal or additive by a coin). Not ported: its beam (`0x423ef8`) and
/// streak (`0x424888`).
pub fn draw_shot(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let k = m.pv_f(0x60);
    let a = rng.rand_range(0x20, 0x30) as u32;
    let roll = rng.rand_angle();
    let s = rng.randf(0.35, 0.45) * k;
    crate::fx::sprite(out, s, 0.0, roll, m.pos, cam, 0xb, SHOT_PINK | a << 24, true);
    let (p0, p1) = (m.pv_v(0x10), m.pv_v(0x20));
    for _ in 0..5 {
        let t = rng.randf(0.0, 1.0);
        let mut p = add(p0, scale(sub(p1, p0), t));
        for c in p.iter_mut() { *c += rng.randf(-0.1, 0.1) * k; }
        let additive = rng.rand() % 2 == 1;
        let a = rng.rand_range(0x40, 0x7f) as u32;
        let roll = rng.rand_angle();
        let s = rng.randf(0.05, 0.15) * k;
        crate::fx::sprite(out, s, 0.0, roll, p, cam, 0xb, SHOT_PINK | a << 24, additive);
        crate::fx::sprite(out, s * 0.5, 0.0, roll, p, cam, 0xb, SHOT_WHITE | a << 24, additive);
    }
}
