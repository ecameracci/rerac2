//! Maktar's arena (the level's chunk 1, above the resort; L02): its master 0x0b59 and the pieces of the floor and wall
//! it works. The fighters are crate::moby::classes::fighters.
//!
//! * **The master 0x0b59** (`0x3fb1b0`, [`update_master`]). Its pvars: the groups `+0x40` (the fighters and bosses,
//!   pooled above the arena), `+0x44` (the spawners), `+0x48` (the flame columns), `+0x4c` (the floor plates), `+0xc4`
//!   / `+0xc8` (the crates 0x01ff / 0x01f5) and `+0xcc` (the crate lifts); the turns to attack `+0x50..+0x5c`; the
//!   desk's cuboid `+0x70` and cylinder `+0x74` (the teleporter, once the first challenge is won), the way back
//!   `+0x78`, the arena's cylinders `+0x7c` / `+0x80` and its floor's `+0x84`, its middle `+0x88` and entry `+0x8c`
//!   (cuboids); the announcer's lines `+0x130..` and the scene triggers `+0x108` (the welcome) / `+0x10c` (the first
//!   win). **1** the desk: the prompt "Enter the Battle Arena" (owner 4) and △ → **2**: after the wipe (31 ticks)
//!   chunk 1 (crate::game::GameState::switch_chunk), Ratchet at the entry, the checkpoint, the camera behind him; the
//!   first time the welcome scene and straight into the first challenge, else the overview once he is at the middle.
//!   **4 / 5** (5 after a win: its reward, [`prize`]) the fighters pooled, the columns and plates down, Ratchet frozen
//!   at the middle, the script camera moving to the big screen → **7** the menu ([`Menu`], [`draw_menu`]: ← / → the
//!   challenges open, [`on_menu`]; ✕ starts one, □ the vendor's weapons, △ back to the resort) or **6** the win's screen (✕ the menu at the
//!   next challenge). **8** the challenge: the waves of the level's challenge table ([`read_challenges`], `$gp+0x2160`) — each its fighters sent out through the
//!   spawners (the round's spawner far from Ratchet; then its doors opened), its columns and plates up and its crates
//!   lifted (the crates put away at the reset, the overview and each challenge's start); a wave is over with all sent and none left; every fourth tick the two gladiators or bladed robots nearest
//!   Ratchet get the turns; a fighter that leaves the arena dies, Ratchet leaving it dies. Lost (time out in 3 / 5,
//!   out of shots in 7 / 8, hurt in 12 / 13): its message and **10**, then the overview. **0xb** the first win's scene.
//!   The wins count in the save's flag bytes (`0x1395b8 + 0x31..`, [`win_flag`]). It draws the moat's energy
//!   ([`draw_moat`], the arena's floor there is a plain dark texture) and the lamps' glows ([`draw_lamps`]).
//! * **The spawners 0x0a15** (`0x3d8898`, six in the rim wall, the master's group `+0x44`): each holds up to three
//!   fighters (`+0x10` / `+0x14` / `+0x18`, [`attach`]) and, opened by the master ([`open`]), slides back 2 along its
//!   second row over 19 ticks, plays its sequence 1 and lets them out ([`release`]), waits 90 ticks, plays 2 and 3 and
//!   slides home. On its first update it steps 2 forward (its closed place) and makes its frame (class 0x1172, at its
//!   joint 0 plus `(0, 9.4, 0.65)` in its rows; `0x315438`), which follows it while its sequence plays.
//! * **The floor plates 0x0a03** (`0x3d8128`, twelve, the master's group `+0x4c`): the master wants a plate up or down
//!   (`+0xbc`, [`raise`]); it turns half a turn about its first row at 180° a second (60 ticks, `$gp+0x1350`) carrying
//!   its tie (the one of its tie group `+0xb0` within 1 of it, `0x3d8448`), a hum looping (its sound 0, sound 1 as it
//!   starts up). Up (state 1) and turning it hits what stands on it ([`hurt`]); up and in view, now and then sparks.
//!
//! * **The flame columns 0x0b89** ([`update_column`]) and **the crate lifts 0x0ccc** ([`update_lift`]).
//! * **The scene props 0x0f15** ([`update_props`], [`draw_props`]): in the arena's scenes the master's moat and lamps,
//!   the big screens and a moby moved aside.
//!
//! The bosses 0x09f8 / 0x0a44 are crate::moby::classes::bosses (sent by [`fight`] through `send_wave_bosses`).
//!
//! The music (`0x335928(track, stinger)`, crate::audio::chunk through the host): the chunk switches carry the arena's
//! track `+0x220` in and the resort's `+0x224` out, and the master asks for them after (stingers `+0x228` / `+0x22c`);
//! the bosses' `+0x230` / `+0x234` as a wave's boss is sent, the arena's again at a wave without bosses.
//!
//! The Spiderbot Glove's bot ends at a challenge's end (`0x3fb028(1, −1)`, crate::moby::classes::spiderbot::end) and
//! on Ratchet's warps (`0x2bd368`'s `0x3fb028(0, 0)`).
//!
//! The announcer ([`say`], `0x401d98`): its 39 lines (`+0x130..`) through the dialogue (crate::audio::speech, id +
//! 2000) — at the desk's goodbye, the wins, the menu's first tick, the win screen, the bosses sent; its remarks on the
//! fight ([`remark`], `0x401e68`) and on the boss ([`boss_remarks`], `0x402090`). The level's sounds
//! (crate::audio::level_sounds): the crowd's gasp 0x17 ([`score`]), the columns' hum 0x11, the menu's 3 / 4.
//!
//! Not ported: the upgraded glove's end (`0x42e670`: its bot 0x0e79 has no port), the restart's path straight into the
//! wipe (`0x1a7904` = 2), the challenges' weapon lock (`0x18b6f4`) and the fixed held items of 7 / 8 / 13 beyond the
//! hand request; the plates' sparks (`0x3d85a8`: the particle burst `0x345d88` has no port).

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, set_len, sub, V3};
use crate::moby::attach::{angles_of, rows_of, turn, Carrier, Kind, Member};
use crate::moby::hits::HitTemplate;
use crate::moby::{ai, mode, Ctx, MobyWorld};
use super::props::{set_seq, wrapped};

/// The spawner, its frame, the floor plate.
pub const SPAWNER: i16 = 0x0a15;
const FRAME: i16 = 0x1172;
pub const PLATE: i16 = 0x0a03;
/// The spawner's riders (links, `ai::link`).
const RIDERS: [usize; 3] = [0x10, 0x14, 0x18];
/// The spawner's frame (`+0x1c`), slide timer (`+0x22`), wait (`+0x20`).
const FRAME_LINK: usize = 0x1c;
/// `$gp+0x13b0`: the frame's place from the spawner's joint 0, in its rows.
const FRAME_AT: V3 = [0.0, 9.4, 0.65];
/// The slide's ticks (0x13) and length.
const SLIDE: i16 = 0x13;
/// The plate's turn: 180° a second (`$gp+0x1350`), so 60 ticks for the half turn (`0x1b3fdc`).
const TURN_SPEED: f32 = 180.0;
const TURN_TICKS: i32 = 60;
/// `$gp+0x135c` / `+0x1358`: the plate's reach and push.
const REACH: f32 = 1.5;
const PUSH: f32 = 2.0;

/// The fighters a spawner can let out (`0x3d8e68`).
pub const FIGHTERS: [i16; 4] = [0x09ca, 0x0b22, 0x0c3b, 0x0c3e];

fn rider(w: &MobyWorld, id: usize, k: usize) -> Option<usize> { ai::link(&w.mobys[id], RIDERS[k]) }

/// `0x3d8d20`: a spawner that can take a fighter (closed, a slot free).
pub fn ready(w: &MobyWorld, id: usize) -> bool { w.mobys[id].state == 2 && (0..3).any(|k| rider(w, id, k).is_none()) }

/// `0x3d8d60`: it holds a fighter.
pub fn loaded(w: &MobyWorld, id: usize) -> bool { (0..3).any(|k| rider(w, id, k).is_some()) }

/// `0x3d8dd0`: fighter `f` takes the first free slot and waits in it (state 2).
pub fn attach(w: &mut MobyWorld, id: usize, f: usize) {
    if let Some(k) = (0..3).find(|&k| rider(w, id, k).is_none()) { ai::set_link(&mut w.mobys[id], RIDERS[k], Some(f)); }
    w.mobys[f].state = 2;
}

/// `0x3d8e10`: the master opens it: its sequence 0 at speed 1, state 3.
pub fn open(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.host.set_anim_speed(&w.mobys[id], 1.0);
    if seq_of(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
    w.mobys[id].set_state(3, -1);
}

fn seq_of(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { super::props::seq(w, id, ctx) }

fn anim_speed(w: &MobyWorld, id: usize, ctx: &Ctx) -> f32 { ctx.host.anim(&w.mobys[id]).map_or(0.0, |a| a.speed) }

/// The frame at the spawner's joint 0 plus [`FRAME_AT`] in its rows, with its rows (`0x317e88`, `0x323c50`,
/// `0x318028`; mode |= 6).
fn place_frame(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let Some(f) = ai::link(&w.mobys[id], FRAME_LINK) else { return };
    let m = &w.mobys[id];
    let j = ctx.host.joint(m, 0).unwrap_or(m.pos);
    let r = m.rows;
    let o = [0, 1, 2].map(|k| FRAME_AT[0] * r[0][k] + FRAME_AT[1] * r[1][k] + FRAME_AT[2] * r[2][k]);
    let x = &mut w.mobys[f];
    x.pos = add(j, o);
    x.rows = r;
    x.rot = angles_of(&r);
    x.mode |= mode::KEEP_ROWS | mode::NO_UPDATE | mode::NO_MATRIX;
    w.rebuild(f, ctx.host);
}

/// 0x0a15 (`0x3d8898`).
pub fn update_spawner(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            for o in RIDERS { w.mobys[id].set_pv_i(o, 0); }
            w.mobys[id].set_state(2, -1);
            if seq_of(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
            ctx.host.set_anim_speed(&w.mobys[id], 0.0);
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, set_len(m.rows[1], 2.0));
            let home = m.pos;
            m.set_pv_v(0, home);
            if w.class(FRAME).is_none() { return; }
            let f = w.create(FRAME);
            let (ud, rows) = (w.mobys[id].update_dist, w.mobys[id].rows);
            let x = &mut w.mobys[f];
            x.mode |= 0x41;
            x.update_dist = ud;
            x.draw_dist = 0;
            x.rows = rows;
            ai::set_link(&mut w.mobys[id], FRAME_LINK, Some(f));
            w.changed = true;
            place_frame(w, id, ctx);
            return;
        }
        s @ (1 | 3) => {
            // Back (3) or home (1): 2 along the second row over 19 ticks, then the sequence's end.
            if w.mobys[id].first() { w.mobys[id].set_pv_s(0x22, SLIDE); }
            let mut t = w.mobys[id].pv_s(0x22) as i32;
            let done = crate::moby::countdown(&mut t);
            w.mobys[id].set_pv_s(0x22, t as i16);
            if !done {
                let k = if s == 1 { 2.0 } else { -2.0 } / SLIDE as f32;
                let m = &mut w.mobys[id];
                m.pos = add(m.pos, set_len(m.rows[1], k));
            }
            if wrapped(w, id, ctx) {
                ctx.host.set_anim_speed(&w.mobys[id], 0.0);
                if s == 1 {
                    let m = &mut w.mobys[id];
                    m.pos = m.pv_v(0);
                    m.set_state(2, -1);
                } else {
                    w.mobys[id].set_state(4, -1);
                }
            }
        }
        4 => {
            w.mobys[id].set_state(5, -1);
            ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            set_seq(w, id, ctx, 1, 0, 0);
            release(w, id, ctx);
        }
        5 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].set_pv_s(0x20, 0x5a);
                ctx.host.set_anim_speed(&w.mobys[id], 0.0);
                w.mobys[id].set_state(6, -1);
            }
        }
        6 => {
            let t = w.mobys[id].pv_s(0x20);
            if t != 0 {
                let mut t = t as i32;
                crate::moby::countdown(&mut t);
                w.mobys[id].set_pv_s(0x20, t as i16);
            } else {
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                set_seq(w, id, ctx, 2, 0, 0);
                w.mobys[id].set_state(7, -1);
            }
        }
        7 => {
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 3, 0, 0);
                w.mobys[id].set_state(1, -1);
            }
        }
        _ => {}
    }
    if anim_speed(w, id, ctx) != 0.0 { place_frame(w, id, ctx); }
}

/// `0x3d8e68`: the riders out, below and in front of it (`(0, −9, −3.3)` in its rows; the first 2 back in x, the
/// others 1 either way in y), each facing Ratchet and started by its class.
fn release(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let r = m.rows;
    let l = [0.0, -9.0, f32::from_bits(0xc053_3333)];
    let p = add(m.pos, [0, 1, 2].map(|k| l[0] * r[0][k] + l[1] * r[1][k] + l[2] * r[2][k]));
    let at = [[p[0] - 2.0, p[1], p[2]], [p[0], p[1] + 1.0, p[2]], [p[0], p[1] - 1.0, p[2]]];
    for k in 0..3 {
        let Some(f) = rider(w, id, k) else { continue };
        let hero = ctx.hero.pos;
        let x = &mut w.mobys[f];
        x.pos = at[k];
        x.rot = [0.0, 0.0, atan2(hero[0] - x.pos[0], hero[1] - x.pos[1])];
        super::fighters::start(w, f, ctx);
    }
    for o in RIDERS { w.mobys[id].set_pv_i(o, 0); }
}

/// `0x3d8868`: the plate is wanted up.
pub fn raised(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_i(0xbc) != 0 }

/// `0x3d8878` / `0x3d8888`: want it up / down.
pub fn raise(w: &mut MobyWorld, id: usize, up: bool) { w.mobys[id].set_pv_i(0xbc, up as i32); }

/// `0x3d8448`: the plate carries the tie of its group `+0xb0` within 1 of it.
fn take_tie(w: &mut MobyWorld, id: usize) {
    let g = w.mobys[id].pv_i(0xb0);
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.tie_groups.get(g)).cloned() else { return };
    let (cp, cr) = (w.mobys[id].pos, rows_of(w.mobys[id].rot));
    let dot = |a: V3, b: V3| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    for i in list {
        let Some(t) = w.ties.get(i as usize) else { continue };
        let tp = [t[3][0], t[3][1], t[3][2]];
        if 1.0 <= dist2(cp, tp) { continue; }
        let d = sub(tp, cp);
        let rows = [0, 1, 2].map(|r| { let v = [t[r][0], t[r][1], t[r][2]]; [dot(v, cr[0]), dot(v, cr[1]), dot(v, cr[2])] });
        let member = Member { kind: Kind::Tie, index: i as usize, offset: [dot(d, cr[0]), dot(d, cr[1]), dot(d, cr[2])], rows };
        w.attached.insert(id, Carrier { members: vec![member], use_rows: false });
        return;
    }
}

/// 0x0a03 (`0x3d8128`).
pub fn update_plate(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_b(9, 0);
        m.set_pv_b(8, 3);
        m.set_pv_i(0x10, 0);
        m.mode &= !mode::TARGET;
        m.set_state(2, -1);
        m.mode |= mode::KEEP_ROWS;
        let a = TURN_SPEED.to_radians() / 60.0;
        m.set_pv_f(0x80, a);
        take_tie(w, id);
        w.carry(id, ctx.host);
        let m = &mut w.mobys[id];
        m.set_pv_i(0xbc, 0);
        m.set_pv_s(0xc0, -1);
    }
    match w.mobys[id].state {
        1 => {
            hurt(w, id, ctx);
            if w.mobys[id].pv_i(0xbc) == 0 { turn_start(w, id, 4); }
        }
        2 => {
            if w.mobys[id].pv_i(0xbc) != 0 { turn_start(w, id, 3); }
        }
        s @ (3 | 4) => {
            if s == 3 && w.mobys[id].pv_i(0xb4) == TURN_TICKS { ctx.host.play_sound(&w.mobys[id], 1, 0); }
            let a = w.mobys[id].pv_f(0x80);
            let m = &mut w.mobys[id];
            m.rows = turn(&rows_of([a, 0.0, 0.0]), &m.rows);
            m.rot = angles_of(&m.rows);
            w.carry(id, ctx.host);
            hurt(w, id, ctx);
            let mut t = w.mobys[id].pv_i(0xb4);
            crate::moby::countdown(&mut t);
            w.mobys[id].set_pv_i(0xb4, t);
            let h = w.mobys[id].pv_s(0xc0) as i32;
            if s == 3 {
                if t == 0 { w.mobys[id].set_state(1, -1); }
                if !ctx.host.sound_playing(h) {
                    let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                    w.mobys[id].set_pv_s(0xc0, h as i16);
                }
            } else if t == 0 {
                if h != -1 { ctx.host.stop_sound(h); }
                w.mobys[id].set_pv_s(0xc0, -1);
                w.mobys[id].set_state(2, -1);
            }
        }
        _ => {}
    }
}

fn turn_start(w: &mut MobyWorld, id: usize, s: u8) {
    let m = &mut w.mobys[id];
    m.set_state(s, -1);
    m.set_pv_i(0xb4, TURN_TICKS);
}

/// `0x3d86a8`: what touches it (a sphere of 1.5) is hit for its damage (`+0x14`) and pushed 2 round it: Ratchet
/// only on the ground; a fighter only seen and with it in view, unless its movement block's `+0xc0` is clear; the
/// bosses 0x0a44 / 0x0a4b never.
fn hurt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pos = w.mobys[id].pos;
    let Some(sc) = ctx.scene else { return };
    let ids = crate::collision::coll_sphere_mobys(sc, pos, REACH, crate::collision::QueryFlags(0), Some(id));
    let damage = w.mobys[id].pv_b(0x14) as f32;
    let seen = w.mobys[id].drawn != 0;
    for o in ids {
        if Some(o) == ctx.hero_moby {
            if ctx.hero.grounded_ticks == 0 { continue; }
        } else {
            let x = &w.mobys[o];
            if x.mode & ai::AI != 0 {
                let mv = ai::mv(x);
                if mv != 0 && x.pv_s(mv + 0xc0) == 0 { continue; }
                if !seen || x.drawn == 0 || x.o_class == 0x0a44 || x.o_class == 0x0a4b { continue; }
            }
        }
        let d = sub(w.mobys[o].pos, pos);
        let p = set_len([-d[1], d[0], 0.0], PUSH);
        let mut t = HitTemplate::new(damage, Some(id), 0x1_0001, [p[0], p[1], 1.0, 5_627.987]);
        t.kind = 0;
        let at = w.mobys[o].pos;
        w.deliver_hit_any(o, &t, [at[0], at[1], at[2], 1.0]);
    }
}

// ---------------------------------------------------------------------------------------------------------------
// The arena master 0x0b59

/// One wave (0x14 bytes): how many of each fighter (0x09ca, 0x0b22, 0x0c3b, 0x0c3e), the floor plates and 0x0b89s
/// up, the crates lifted (0x01f5 then 0x01ff), the bosses (0x09f8, 0x0a44), the spawners it uses.
#[derive(Clone, Copy, Debug)]
pub struct Wave {
    pub fighters: [i16; 4],
    pub plates: i16,
    pub b89: i16,
    pub crates: [i16; 2],
    pub bosses: [u8; 2],
    pub spawners: u8,
    pub x13: u8,
}

/// One challenge (`$gp+0x2160[n]`, 0x18 bytes `{i32 waves, u32 → waves, i32 needs, i32 bolts, i32 xp, 0}`): its waves,
/// the challenge whose win opens it (−1 none), the bolts and the XP (×32) its first win pays (each later win a third,
/// at least 500 bolts and 5 XP).
#[derive(Clone, Debug)]
pub struct Challenge {
    pub needs: i32,
    pub bolts: i32,
    pub xp: i32,
    pub waves: Vec<Wave>,
}

/// `$gp+0x2160`: the challenges' table, 14 pointers to their records (Maktar's overlay; L02 `0x1b1150`).
pub const CHALLENGE_TABLE: u32 = 0x1aef_f0 + 0x2160;
pub const CHALLENGE_COUNT: usize = 14;

/// The challenges as the level's overlay holds them ([`CHALLENGE_TABLE`]; `read(address, length)` its bytes): the
/// data the master plays, read at load (crate::level::LevelData::arena) rather than kept in the port, so a level's
/// own table (or a mod's) is the one played. None when the table or a record is not there.
pub fn read_challenges(read: &dyn Fn(u32, usize) -> Option<Vec<u8>>) -> Option<Vec<Challenge>> {
    let word = |b: &[u8], o: usize| i32::from_le_bytes(b[o..o + 4].try_into().unwrap());
    let half = |b: &[u8], o: usize| i16::from_le_bytes(b[o..o + 2].try_into().unwrap());
    let table = read(CHALLENGE_TABLE, 4 * CHALLENGE_COUNT)?;
    (0..CHALLENGE_COUNT).map(|n| {
        let rec = read(word(&table, 4 * n) as u32, 0x18)?;
        let count = usize::try_from(word(&rec, 0)).ok().filter(|&c| c <= 0x40)?;
        let waves = read(word(&rec, 4) as u32, 0x14 * count)?;
        let waves = (0..count).map(|k| {
            let w = &waves[0x14 * k..0x14 * (k + 1)];
            Wave {
                fighters: std::array::from_fn(|i| half(w, 2 * i)),
                plates: half(w, 8),
                b89: half(w, 0xa),
                crates: [half(w, 0xc), half(w, 0xe)],
                bosses: [w[0x10], w[0x11]],
                spawners: w[0x12],
                x13: w[0x13],
            }
        }).collect();
        Some(Challenge { needs: word(&rec, 8), bolts: word(&rec, 0xc), xp: word(&rec, 0x10), waves })
    }).collect()
}

/// `0x3fb110(n)`: challenge `n`'s win count in the flags (`0x1395b8 + k`).
pub fn win_flag(n: i32) -> Option<usize> {
    Some(match n {
        0 => 0x31, 1 => 0x34, 2 => 0x32, 3 => 0x33, 4 => 0x35, 5 => 0x36, 6 => 0x37, 7 => 0x38, 8 => 0x91, 9 => 0x39,
        10 => 0x3a, 11 => 0x3b, 12 => 0x92, 13 => 0x93,
        _ => return None,
    })
}

/// `0x3ff768(n)`: challenge `n`'s name.
pub fn challenge_name(n: i32) -> u32 {
    [0x2dde, 0x2de4, 0x2ddf, 0x2de3, 0x2de0, 0x2de5, 0x2de2, 0x2e04, 0x30d6, 0x2de1, 0x2de6, 0x2de7, 0x30d7, 0x30d8].get(n as usize).copied().unwrap_or(0x2dde)
}

fn wins(game: &crate::game::GameState, n: i32) -> u8 { win_flag(n).and_then(|k| game.flags.get(k).copied()).unwrap_or(0) }

/// The challenges' weapons: 7 is played with item 0x1e (300 shots), 8 with 0x2a (8).
const WEAPON_7: i32 = 0x1e;
const WEAPON_8: i32 = 0x2a;

/// Whether challenge `n` is on the menu (`0x3fee58`'s test): the challenge before it won; from 12 the challenge mode;
/// 7 and 8 their weapons (`0x264a74[0x139586 / 0x139592 · 0xe0]`: here the items had, inferred).
pub fn on_menu(cs: &[Challenge], game: &crate::game::GameState, n: i32) -> bool {
    let Some(c) = cs.get(n as usize) else { return false };
    if 0 < n && wins(game, c.needs) == 0 { return false; }
    if 0xb < n && !game.challenge { return false; }
    let had = |i: i32| game.items.get(i as usize).is_some_and(|&b| b != 0);
    if n == 7 && !had(WEAPON_7) { return false; }
    if n == 8 && !had(WEAPON_8) { return false; }
    true
}

/// `0x3fe398(n)`: the bolts a win of `n` pays — its prize, a third less for each win before (×0.3), at least 500; ten
/// times that in the challenge mode.
pub fn prize(cs: &[Challenge], game: &crate::game::GameState, n: i32) -> i32 {
    let base = cs.get(n as usize).map_or(0, |c| c.bolts) as f32;
    let mut f = base;
    for _ in 0..wins(game, n) { f *= 0.3; }
    let mut v = base;
    if f <= v { v = f.max(500.0); }
    if game.challenge { v *= 10.0; }
    v as i32
}

/// `0x3fe4a8(n)`: the XP a win of `n` pays (its XP, a third for each win before, at least 5; ×32).
fn xp(cs: &[Challenge], game: &crate::game::GameState, n: i32) -> i32 {
    let base = cs.get(n as usize).map_or(0, |c| c.xp) as f32;
    let mut f = base;
    for _ in 0..wins(game, n) { f *= 0.33333; }
    let mut v = base;
    if f <= v { v = f.max(5.0); }
    (v as i32) << 5
}

/// What the engine draws of the challenge menu (`0x3ff808`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Menu {
    /// The selection, its prize (none for the first challenge not won yet) and wins.
    pub selected: i32,
    pub prize: Option<i32>,
    pub wins: u8,
    /// A challenge before / after it on the menu.
    pub prev: bool,
    pub next: bool,
    /// The pad's held buttons (the arrows lit while their way is held) and the frame (their pulse).
    pub held: u32,
    pub frame: i32,
}

/// The master's pvars (the module's docs).
mod pv {
    pub const ENEMIES: usize = 0x40;
    pub const SPAWNERS: usize = 0x44;
    pub const B89: usize = 0x48;
    pub const PLATES: usize = 0x4c;
    pub const TURNS: usize = 0x50;
    pub const ALIVE: usize = 0x60;
    pub const LIFE: usize = 0x64;
    pub const HEALTH: usize = 0x68;
    pub const SCORE: usize = 0x6c;
    pub const DESK: usize = 0x70;
    pub const DESK_WON: usize = 0x74;
    pub const BACK: usize = 0x78;
    pub const ARENA: usize = 0x7c;
    pub const KEEP: usize = 0x80;
    pub const MIDDLE: usize = 0x88;
    pub const ENTRY: usize = 0x8c;
    pub const SPAWNED: usize = 0x90;
    pub const GATES: usize = 0xa8;
    /// The gates' six words (`0xa8..0xbc`, cleared so by `0x3fc530`; `+0xc4..` are the crate and lift groups).
    pub const GATE_SLOTS: usize = 6;
    pub const CRATES_B: usize = 0xc4;
    pub const CRATES_A: usize = 0xc8;
    pub const LIFTS: usize = 0xcc;
    pub const BOSS: usize = 0xd0;
    pub const TICKS: usize = 0xd4;
    pub const GATE: usize = 0xd8;
    pub const INTRO: usize = 0x108;
    pub const FIRST_WIN: usize = 0x10c;
    pub const PRIZE: usize = 0x110;
    pub const SELECTED: usize = 0x118;
    pub const REPEAT: usize = 0x11a;
    pub const RING: usize = 0x11c;
    pub const WINDOW: usize = 0x11e;
    pub const CHALLENGE: usize = 0x120;
    pub const WAVE: usize = 0x122;
    pub const WIPE: usize = 0x124;
    pub const SAID: usize = 0x1d0;
    /// The remarks (`0x401e68`): the wait `+0xe4`, the damage and kills summed `+0xe8` / `+0xec`, the sums' restart
    /// `+0x100`; the announcer's 39 line ids `+0x130..` (crate::audio::speech).
    pub const REMARK_WAIT: usize = 0xe4;
    pub const REMARK_SUM: usize = 0xe8;
    pub const REMARK_KILLS: usize = 0xec;
    pub const REMARK_RESTART: usize = 0x100;
    pub const LINES: usize = 0x130;
    /// The music (`0x335928(track, stinger)`): the arena's track and stinger `+0x220` / `+0x228`, the resort's
    /// `+0x224` / `+0x22c`, the bosses' `+0x230` / `+0x234`.
    pub const MUSIC_ARENA: usize = 0x220;
    pub const MUSIC_RESORT: usize = 0x224;
    pub const STINGER_ARENA: usize = 0x228;
    pub const STINGER_RESORT: usize = 0x22c;
    pub const MUSIC_BOSS: usize = 0x230;
    pub const STINGER_BOSS: usize = 0x234;
}

/// `$gp+0x2198`: the turns to attack each fighter class gets at a challenge's start.
const TURNS: [i32; 4] = [3, 4, 0, 5];
/// `$gp+0x21a0` / `+0x21b0`: where the menu's camera comes to rest and its angles; `+0x21c0` / `+0x21d0` where it
/// starts; `+0x21e0` its ticks.
const MENU_EYE: V3 = [500.0, 571.5, 509.5];
const MENU_ANGLES: V3 = [0.0, 0.0, -std::f32::consts::FRAC_PI_2];
const MENU_START: V3 = [508.0, 614.0, 513.0];
const MENU_TICKS: i32 = 0xb4;
/// `$gp+0x21e4`: the score's window (ticks); `+0x21e8..+0x21f0` its weights (damage, kills, health lost).
const WINDOW: i16 = 600;
const WEIGHTS: [f32; 3] = [0.2, 1.0, -2.0];
/// The desk's prompt text (`0x32cb88(4, 0x2dd2, 1)`), its owner.
const DESK_TEXT: u32 = 0x2dd2;
const DESK_OWNER: i32 = 4;
/// The fail messages: "… kills" (time out), out of shots (7 / 8), hurt (12 / 13).
const FAIL_KILLS: (u32, u32) = (0x30ff, 0x2df7);
const FAIL_SHOTS_7: u32 = 0x3101;
const FAIL_SHOTS_8: u32 = 0x3102;
const FAIL_HURT: u32 = 0x3100;

fn group(w: &MobyWorld, id: usize, off: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].pv_i(off)).ok().filter(|&g| g != 0xff).and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// The placed moby of instance `i` (the master keeps moby indices).
/// A moby the master's pvars name (`+0x108`, `+0x10c`): the loader made them runtime indices (crate::moby's spawn).
fn placed(w: &MobyWorld, i: i32) -> Option<usize> { usize::try_from(i).ok().filter(|&i| w.mobys.get(i).is_some_and(|m| m.state < 0x80)) }

fn cuboid_at(ctx: &Ctx, w: &MobyWorld, id: usize, off: usize) -> Option<(V3, V3)> { ctx.level.cuboid_place(w.mobys[id].pv_i(off)) }

/// `0x401d98(m, n)`: line `n` of the announcer — with the dialogue free its id (`+0x130 + 4n`, the low 14 bits) + 2000
/// asked for (`0x1a63cc`, crate::audio::speech: the stream id of the level's table) and the line marked said this
/// challenge (`+0x1d0 + 2n`); whether it was.
fn say(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, n: i32) -> bool {
    if !(0..=0x26).contains(&n) { return false; }
    let line = (w.mobys[id].pv_i(pv::LINES + 4 * n as usize) & 0x3fff) + 2000;
    if !ctx.host.request_speech(line) { return false; }
    w.mobys[id].set_pv_s(pv::SAID + 2 * n as usize, 1);
    true
}

fn said(w: &MobyWorld, id: usize, n: i32) -> bool { w.mobys[id].pv_s(pv::SAID + 2 * n as usize) != 0 }

/// `$gp+0x2570..+0x2584`: the remarks' damage and kills to pass (9, 5), the hurt (0.5), the low health (a quarter), the
/// wait after a remark (240 + up to 240 ticks), a boss's low life (0.4 of its most).
const REMARK_DAMAGE: f32 = 9.0;
const REMARK_KILLS: i32 = 5;
const REMARK_HURT: f32 = 0.5;
const REMARK_LOW: f32 = 0.25;
const REMARK_WAIT: i32 = 240;
const REMARK_BOSS_LOW: u32 = 0x3ecc_cccd;
/// The remarks' sum restart (`+0x100`): every 300 ticks.
const REMARK_RESTART: i32 = 300;

/// `0x401e68(damage, hurt, m, kills)`: the announcer's remarks on the fight, with the dialogue free and the last one's
/// wait `+0xe4` over — the damage and the kills summed (`+0xe8`, `+0xec`; restarted every 300 ticks, `+0x100`): past 9
/// or 5 its lines 0x1d.. 0x22 not said (0x1d only without a boss), and the sums restarted; hurt more than 0.5 its lines
/// 0x1a..0x1c; Ratchet under a quarter of his health 0x23; down 0x24. One of them at random, then a wait of 240 + up
/// to 240 ticks.
fn remark(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, damage: f32, hurt: f32, kills: i32) {
    if !ctx.host.speech_free() || w.mobys[id].pv_i(pv::REMARK_WAIT) != 0 { return; }
    let m = &mut w.mobys[id];
    let sum = m.pv_f(pv::REMARK_SUM) + damage;
    let k = m.pv_i(pv::REMARK_KILLS) + kills;
    m.set_pv_i(pv::REMARK_KILLS, k);
    m.set_pv_f(pv::REMARK_SUM, sum);
    let boss = m.pv_i(pv::BOSS) != 0;
    let mut lines = Vec::new();
    if REMARK_DAMAGE < sum || REMARK_KILLS < k {
        lines.extend((0..6).filter(|&j| !(j == 0 && boss)).map(|j| 0x1d + j).filter(|&n| !said(w, id, n)));
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::REMARK_KILLS, 0);
        m.set_pv_f(pv::REMARK_SUM, 0.0);
    }
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(pv::REMARK_RESTART);
    let over = crate::moby::countdown(&mut t);
    m.set_pv_i(pv::REMARK_RESTART, t);
    if over {
        m.set_pv_i(pv::REMARK_KILLS, 0);
        m.set_pv_i(pv::REMARK_RESTART, REMARK_RESTART);
        m.set_pv_f(pv::REMARK_SUM, 0.0);
    }
    if REMARK_HURT < hurt { lines.extend((0x1a..0x1d).filter(|&n| !said(w, id, n))); }
    if (ctx.hero.hp as f32) < ctx.game.max_health as f32 * REMARK_LOW && !said(w, id, 0x23) { lines.push(0x23); }
    if ctx.hero.hp < 1 && !said(w, id, 0x24) { lines.push(0x24); }
    if lines.is_empty() { return; }
    let n = lines[ctx.game.rng.randi(lines.len() as i32) as usize];
    say(w, id, ctx, n);
    let wait = REMARK_WAIT + ctx.game.rng.randi(REMARK_WAIT);
    w.mobys[id].set_pv_i(pv::REMARK_WAIT, wait);
}

/// `0x402090`: the remarks on the challenge's boss (`+0xd0`): Ratchet down, one of its two lines (0x10 / 0x11 for
/// 0x09f8, 0x16 / 0x17 for 0x0a44) unless said; its life under 0.4 of its most, 0x12 / 0x18; gone, 0x13 / 0x19 — each
/// once.
fn boss_remarks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(b) = ai::link(&w.mobys[id], pv::BOSS) else { return };
    let (down, low, gone) = match w.mobys[b].o_class {
        0x09f8 => (0x10, 0x12, 0x13),
        0x0a44 => (0x16, 0x18, 0x19),
        _ => return,
    };
    let (life, most) = (w.mobys[b].pv_f(0x20), w.mobys[b].pv_s(0x24) as f32);
    let mut n = -1;
    if ctx.hero.hp as f32 <= 0.0 {
        let k = down + ctx.game.rng.randi(2);
        n = if said(w, id, k) { -1 } else { k };
    }
    if !said(w, id, low) && life < most * f32::from_bits(REMARK_BOSS_LOW) { n = low; }
    if !said(w, id, gone) && life <= 0.0 { n = gone; }
    if 0 < n { say(w, id, ctx, n); }
}

/// `0x3fc530`: the challenge and wave none, the counts cleared, state 1.
fn reset_master(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_s(pv::WAVE, -1);
    m.set_pv_s(pv::CHALLENGE, -1);
    m.set_pv_s(pv::SELECTED, 0);
    for k in 0..6 { m.set_pv_i(pv::SPAWNED + 4 * k, 0); }
    m.set_pv_i(pv::GATE, 0);
    m.set_pv_i(pv::BOSS, 0);
    m.set_state(1, -1);
}

/// `0x3fe610`: the crates of both kinds (`+0xc8`, `+0xc4`) put away for the lifts (crate::moby::classes::crate_::put_away).
fn put_crates_away(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for c in [group(w, id, pv::CRATES_A), group(w, id, pv::CRATES_B)].concat() {
        if w.mobys[c].state < 0x80 { super::crate_::put_away(w, c, ctx); }
    }
}

/// `0x3fd130`: every fighter and boss back to the pool.
fn pool_all(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for f in group(w, id, pv::ENEMIES) {
        if w.mobys[f].state >= 0x80 { continue; }
        match w.mobys[f].o_class {
            super::fighters::C9CA => super::fighters::pool_09ca(w, f, ctx),
            super::fighters::C3B | super::fighters::C3E | super::fighters::B22 => super::fighters::pool(w, f, ctx),
            super::bosses::C9F8 | super::bosses::CA44 => super::bosses::pool(w, f, ctx),
            _ => {}
        }
    }
}

/// `0x3fd808` / `0x3fdab8(m, n)`: `n` of the group's 0x0b89s / floor plates up (the first ones down raised, or the
/// last ones up lowered).
fn raise_group(w: &mut MobyWorld, id: usize, off: usize, n: i32) {
    let g = group(w, id, off);
    let up = |w: &MobyWorld, p: usize| if off == pv::PLATES { raised(w, p) } else { column_on(w, p) };
    let set = |w: &mut MobyWorld, p: usize, on: bool| if off == pv::PLATES { raise(w, p, on) } else { column_set(w, p, on) };
    let mut count = g.iter().filter(|&&p| up(w, p)).count() as i32;
    if count < n {
        for &p in &g {
            if !up(w, p) {
                set(w, p, true);
                count += 1;
                if n <= count { return; }
            }
        }
    } else if n < count {
        for &p in &g {
            if up(w, p) {
                set(w, p, false);
                count -= 1;
                if count <= n { return; }
            }
        }
    }
}

/// `0x3fc5a0`: the fighters out now (not pooled) and their health, the time (health), the score's window restarted.
fn count_out(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let (mut n, mut life) = (0, 0.0);
    for f in group(w, id, pv::ENEMIES) {
        let x = &w.mobys[f];
        if x.state >= 0x80 || x.state == 1 { continue; }
        if is_fighter_or_boss(x.o_class) {
            n += 1;
            life += x.pv_f(0x20);
        }
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::ALIVE, n);
    m.set_pv_f(pv::LIFE, life);
    m.set_pv_s(pv::RING, 0);
    m.set_pv_f(pv::HEALTH, ctx.hero.hp as f32);
    m.set_pv_s(pv::WINDOW, WINDOW);
}

fn is_fighter_or_boss(c: i16) -> bool { super::fighters::FIGHTERS_AND_BOSSES.contains(&c) }

/// `0x3fc6c0`: the score — over a ring of four windows of 600 ticks, the damage done, the kills and the health lost,
/// weighted 1/1..1/4 newest first, then `0.2·damage + kills − 2·health lost` (`+0x6c`). Before that the announcer's
/// remarks ([`remark`]) and, Ratchet hurt, the crowd's gasp (the level's sound 0x17 past the split, `0x367ff0(0x17, 0,
/// 0)`).
fn score(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (mut n, mut life) = (0, 0.0);
    for f in group(w, id, pv::ENEMIES) {
        let x = &w.mobys[f];
        if x.state >= 0x80 || x.state == 1 || !is_fighter_or_boss(x.o_class) { continue; }
        life += x.pv_f(0x20);
        if 0.0 < x.pv_f(0x20) || matches!(x.o_class, 0x09f8 | 0x0a44) { n += 1; }
    }
    let hp = ctx.hero.hp as f32;
    let m = &w.mobys[id];
    let (was_life, was_hp, was_n) = (m.pv_f(pv::LIFE), m.pv_f(pv::HEALTH), m.pv_i(pv::ALIVE));
    remark(w, id, ctx, was_life - life, was_hp - hp, was_n - n);
    if 0.0 < was_hp - hp { ctx.host.play_level_sound_after(None, 0x17, 0); }
    let m = &mut w.mobys[id];
    let r = (m.pv_s(pv::RING) as usize & 3) * 0x10;
    if n < m.pv_i(pv::ALIVE) { m.set_pv_i(r, m.pv_i(r) + m.pv_i(pv::ALIVE) - n); }
    if life < m.pv_f(pv::LIFE) { m.set_pv_f(r + 4, m.pv_f(r + 4) + m.pv_f(pv::LIFE) - life); }
    if hp < m.pv_f(pv::HEALTH) { m.set_pv_f(r + 8, m.pv_f(r + 8) + m.pv_f(pv::HEALTH) - hp); }
    m.set_pv_f(pv::LIFE, life);
    m.set_pv_i(pv::ALIVE, n);
    m.set_pv_f(pv::HEALTH, hp);
    let (mut dmg, mut hurt, mut kills) = (0.0, 0.0, 0.0);
    let ring = m.pv_s(pv::RING) as i32;
    for k in (1..=4).rev() {
        let f = 1.0 / (5.0 - k as f32);
        let o = ((ring + k).rem_euclid(4) as usize) * 0x10;
        dmg += f * m.pv_f(o + 4);
        hurt += f * m.pv_f(o + 8);
        kills += f * m.pv_i(o) as f32;
    }
    m.set_pv_f(pv::SCORE, WEIGHTS[0] * dmg + WEIGHTS[1] * kills + WEIGHTS[2] * hurt);
    let mut t = m.pv_s(pv::WINDOW) as i32;
    if crate::moby::countdown(&mut t) {
        let r = ((m.pv_s(pv::RING) + 1) & 3) as i16;
        m.set_pv_s(pv::RING, r);
        let o = r as usize * 0x10;
        m.set_pv_i(o, 0);
        m.set_pv_f(o + 4, 0.0);
        m.set_pv_f(o + 8, 0.0);
        t = WINDOW as i32;
    }
    m.set_pv_s(pv::WINDOW, t as i16);
}

/// `0x3fe190`: the wave is over — every fighter it calls for sent out, its bosses out, none left.
fn wave_over(w: &MobyWorld, id: usize, wave: &Wave) -> bool {
    let m = &w.mobys[id];
    let mut done = (0..4).all(|k| m.pv_i(pv::SPAWNED + 4 * k) == wave.fighters[k] as i32);
    if wave.bosses[0] != 0 { done &= m.pv_i(pv::SPAWNED + 0x10) != 0; }
    if wave.bosses[1] != 0 { done &= m.pv_i(pv::SPAWNED + 0x14) != 0; }
    done && m.pv_i(pv::ALIVE) == 0
}

fn wave_of(w: &MobyWorld, id: usize, cs: &[Challenge]) -> Option<Wave> {
    let m = &w.mobys[id];
    cs.get(m.pv_s(pv::CHALLENGE) as usize).and_then(|c| c.waves.get(m.pv_s(pv::WAVE) as usize)).copied()
}

fn gate(w: &MobyWorld, id: usize) -> Option<usize> {
    let k = w.mobys[id].pv_i(pv::GATE).clamp(0, pv::GATE_SLOTS as i32 - 1) as usize;
    ai::link(&w.mobys[id], pv::GATES + 4 * k)
}

/// `0x3fd070`: once every spawner taken this round has let its fighters out, the round starts again.
fn gates_reset(w: &mut MobyWorld, id: usize) {
    let n = w.mobys[id].pv_i(pv::GATE).clamp(0, pv::GATE_SLOTS as i32 - 1) as usize;
    for k in 0..=n {
        if let Some(g) = ai::link(&w.mobys[id], pv::GATES + 4 * k) {
            if !ready(w, g) { return; }
        }
    }
    for k in 0..=n { w.mobys[id].set_pv_i(pv::GATES + 4 * k, 0); }
    w.mobys[id].set_pv_i(pv::GATE, 0);
}

/// `0x3fcd80`: the round's spawner — the current one, or once it has let its fighters out the next (up to the wave's
/// count): one of the group closed and more than 25 from Ratchet (its joint 0), else any closed one more than 30.
fn gate_pick(w: &mut MobyWorld, id: usize, ctx: &Ctx, wave: &Wave) {
    let mut pick = gate(w, id).is_none();
    if let Some(g) = gate(w, id) {
        if !ready(w, g) && w.mobys[id].pv_i(pv::GATE) < wave.spawners as i32 - 1 {
            pick = true;
            let n = w.mobys[id].pv_i(pv::GATE) + 1;
            w.mobys[id].set_pv_i(pv::GATE, n);
        }
    }
    if !pick { return; }
    let k = w.mobys[id].pv_i(pv::GATE).clamp(0, pv::GATE_SLOTS as i32 - 1) as usize;
    let gs = group(w, id, pv::SPAWNERS);
    let far = |w: &MobyWorld, g: usize, r: f32| { let p = ctx.host.joint(&w.mobys[g], 0).unwrap_or(w.mobys[g].pos); w.mobys[g].drawn != 0 && r < dist(p, ctx.hero.pos) };
    let first = gs.iter().copied().find(|&g| far(w, g, 25.0) && ready(w, g));
    let pickd = first.or_else(|| gs.iter().copied().find(|&g| ready(w, g) && 30.0 < dist(w.mobys[g].pos, ctx.hero.pos)));
    if let Some(g) = pickd { ai::set_link(&mut w.mobys[id], pv::GATES + 4 * k, Some(g)); }
}

/// `0x3fca78(m, class)` → `0x3fcc60`: a pooled fighter of `class` into the round's spawner (its life 1 until it is let
/// out), counted.
fn send(w: &mut MobyWorld, id: usize, class: i16) -> bool {
    let Some(g) = gate(w, id).filter(|&g| ready(w, g)) else { return false };
    let Some(f) = group(w, id, pv::ENEMIES).into_iter().find(|&f| { let x = &w.mobys[f]; x.state == 1 && x.o_class == class }) else { return false };
    w.mobys[f].parent = Some(id);
    attach(w, g, f);
    let k = FIGHTERS.iter().position(|&c| c == class).unwrap_or(0);
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::SPAWNED + 4 * k, m.pv_i(pv::SPAWNED + 4 * k) + 1);
    w.mobys[f].set_pv_f(0x20, 1.0);
    true
}

/// `0x3fcff8`: each spawner of the round holding fighters opened.
fn open_gates(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let n = w.mobys[id].pv_i(pv::GATE).clamp(0, pv::GATE_SLOTS as i32 - 1) as usize;
    for k in 0..=n {
        if let Some(g) = ai::link(&w.mobys[id], pv::GATES + 4 * k) {
            if loaded(w, g) { open(w, g, ctx); }
        }
    }
}

/// `0x3fd298` (every fourth frame): the gladiators' and bladed robots' turns given back, then the two of them nearest
/// Ratchet (out, not pooled) given one each.
fn deal_turns(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let g = group(w, id, pv::ENEMIES);
    let turn_of = |c: i16| match c { super::fighters::C9CA => Some((0x2f6, 0x50)), super::fighters::B22 => Some((0x4ea, 0x54)), _ => None };
    for &f in &g {
        if let Some((o, k)) = turn_of(w.mobys[f].o_class) {
            if w.mobys[f].pv_s(o) == 1 {
                w.mobys[f].set_pv_s(o, 0);
                let m = &mut w.mobys[id];
                m.set_pv_i(k, m.pv_i(k) + 1);
            }
        }
    }
    for _ in 0..2 {
        let mut best: Option<(usize, f32)> = None;
        for &f in &g {
            let x = &w.mobys[f];
            let Some((o, _)) = turn_of(x.o_class) else { continue };
            if x.state >= 0x80 || x.state < 4 || x.pv_s(o) == 1 { continue; }
            let d = dist(x.pos, ctx.hero.pos);
            if best.is_none_or(|(_, b)| d < b) { best = Some((f, d)); }
        }
        let Some((f, _)) = best else { continue };
        let (o, k) = turn_of(w.mobys[f].o_class).unwrap();
        w.mobys[f].set_pv_s(o, 1);
        let m = &mut w.mobys[id];
        m.set_pv_i(k, m.pv_i(k) - 1);
    }
}

/// `0x3fdd68`: a fighter in the moat (the cylinder `+0x80`) or out of the arena (the cylinder `+0x7c`) dies; Ratchet
/// out of the arena dies (in the moat he only burns: its surface).
fn keep_in(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (keep, arena) = (w.mobys[id].pv_i(pv::KEEP), w.mobys[id].pv_i(pv::ARENA));
    for f in group(w, id, pv::ENEMIES) {
        let x = &w.mobys[f];
        if x.state >= 0x80 || x.state < 3 || x.mode & mode::NO_UPDATE != 0 { continue; }
        // In the moat (`+0x80`) or out of the arena (`+0x7c`) it dies.
        if !ctx.level.in_cylinder(keep, x.pos) && ctx.level.in_cylinder(arena, x.pos) { continue; }
        match x.o_class {
            super::fighters::C9CA => super::fighters::die_09ca_pub(w, f, ctx),
            super::fighters::B22 | super::fighters::C3B | super::fighters::C3E => {
                // `0x3fd7e8`: `0x31ee88(m, 0, 3)` whatever it is passed.
                crate::moby::bolts::drop(w, f, 0, 3, ctx);
                super::fighters::pool(w, f, ctx);
            }
            _ => {}
        }
    }
    if !ctx.level.in_cylinder(arena, ctx.hero.pos) { ctx.hero.die(); }
}

/// The overview camera (`0x3fed10`): the script camera at `MENU_START`, moving to the menu's place over 180 ticks.
fn menu_camera(ctx: &mut Ctx) {
    ctx.game.cameras.camera_script(MENU_START, MENU_ANGLES, crate::script_camera::MODE_MOVE, MENU_TICKS, false, ctx.hero);
    ctx.game.cameras.camera_script_targets(Some(MENU_EYE), Some(MENU_ANGLES));
}

/// `0x3fe020`: back to the resort — the announcer's goodbye, state 1, chunk 0 with the resort's music, Ratchet at the
/// cuboid `+0x78` (before the first win, facing 2.16) or the desk's `+0x74` (facing −1.59), the checkpoint, the
/// fighters pooled, the plates and 0x0b89s down, the resort's music asked for.
fn leave(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let n = 0x25 + ctx.game.rng.randi(2);
    say(w, id, ctx, n);
    w.mobys[id].set_state(1, -1);
    ctx.switch_chunk(0, w.mobys[id].pv_i(pv::MUSIC_RESORT));
    let m = &w.mobys[id];
    let place = if wins(ctx.game, 0) == 0 {
        ctx.level.cuboid_place(m.pv_i(pv::BACK)).map(|c| (c.0, f32::from_bits(0x400a_3d71)))
    } else {
        usize::try_from(m.pv_i(pv::DESK_WON)).ok().and_then(|i| ctx.level.cylinders.get(i)).map(|c| ([c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]], f32::from_bits(0xbfcb_851f)))
    };
    if let Some((p, yaw)) = place {
        // `0x2bd368(.., 0, 1)`: its last argument the camera reset (`0x2dd6f8`).
        ctx.hero.teleport(p, [0.0, 0.0, yaw]);
        super::spiderbot::end(w, false, 0);
        ctx.game.cameras.reset();
        ctx.hero.request_state(0, true);
        let (p, r) = (ctx.hero.pos, ctx.hero.rot);
        ctx.game.save_checkpoint(p, r);
    }
    pool_all(w, id, ctx);
    raise_group(w, id, pv::B89, 0);
    raise_group(w, id, pv::PLATES, 0);
    let m = &w.mobys[id];
    ctx.host.music_request(m.pv_i(pv::MUSIC_RESORT), m.pv_i(pv::STINGER_RESORT));
    ctx.game.arena_menu = None;
}

/// `0x3fe240(m, n)`: challenge `n` won — the first win of the first challenge (its own line and the Electrolyzer,
/// `0x2f8668(0x26)`; its `0x326bd0(0x26)` leaves the item asked for as it was, so only the slots of the other kinds
/// would change), else the bolts and XP (`0x3fe398`, `0x3fe4a8`); its win counted. Not ported: the autosave after
/// (`0x311738(0, -1)`).
fn reward(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, n: i32) {
    if n == 0 && wins(ctx.game, 0) == 0 {
        say(w, id, ctx, 7);
        ctx.game.get_item(super::electrolyzer::ITEM);
    } else {
        let line = match n { 6 => 8, 0xb => 9, _ => 2 + ctx.game.rng.randi(5) };
        say(w, id, ctx, line);
        ctx.game.xp += xp(&ctx.level.arena, ctx.game, n);
        let b = prize(&ctx.level.arena, ctx.game, n);
        w.mobys[id].set_pv_i(pv::PRIZE, b);
        ctx.game.bolt_bank += b;
        ctx.game.show_bolts = Some(0x168);
    }
    if let Some(k) = win_flag(n) {
        if let Some(f) = ctx.game.flags.get_mut(k) { *f = f.saturating_add(1); }
    }
}

/// `0x3fe7c8`: the wave's crates lifted — for each kind (`+0xc8` 0x01f5, then `+0xc4` 0x01ff) as many out as it asks
/// for: the kind's broken ones (state 6, `0x3a84f8`; the rest count as out), the last first, each put on a free lift
/// (0x0ccc, `0x417fb8`; one in six passed over) by passes over the lifts from the first, at most 999 passes
/// ([`lift`]).
fn lift_crates(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, wave: &Wave) {
    for (want, from) in [(wave.crates[0], pv::CRATES_A), (wave.crates[1], pv::CRATES_B)] {
        if want == 0 { continue; }
        let crates = group(w, id, from);
        let mut broken: Vec<usize> = crates.iter().copied().filter(|&c| w.mobys[c].state == 6).collect();
        let mut out = (crates.len() - broken.len()) as i32;
        if broken.is_empty() || want as i32 <= out { continue; }
        let lifts = group(w, id, pv::LIFTS);
        if lifts.is_empty() { continue; }
        let mut passes = 0;
        loop {
            passes += 1;
            for &l in &lifts {
                let Some(&c) = broken.last() else { break };
                if !lift_free(w, l) || ctx.game.rng.randi(6) != 0 { continue; }
                broken.pop();
                out += 1;
                lift(w, l, c, 3.0 / 60.0, 720.0_f32.to_radians() / 60.0);
                break;
            }
            if 999 < passes || want as i32 <= out || broken.is_empty() { break; }
        }
    }
}

/// `0x3fdab8`/`0x3fd808` for a wave and its crates (the start of a wave).
fn wave_start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(wave) = wave_of(w, id, &ctx.level.arena) else { return };
    lift_crates(w, id, ctx, &wave);
    raise_group(w, id, pv::B89, wave.b89 as i32);
    raise_group(w, id, pv::PLATES, wave.plates as i32);
}

fn fail_kills(w: &MobyWorld, id: usize) -> i32 {
    let m = &w.mobys[id];
    let sent: i32 = (0..6).map(|k| m.pv_i(pv::SPAWNED + 4 * k)).sum();
    m.pv_s(pv::WAVE) as i32 * 0xf + sent - m.pv_i(pv::ALIVE)
}

/// The challenge lost (its message shown, `0x313098`): state 10.
fn lose(w: &mut MobyWorld, id: usize) {
    w.mobys[id].set_state(10, -1);
    // `0x3fb028(1, −1)`: a Spiderbot out blows up.
    super::spiderbot::end(w, true, -1);
}

/// The port's: the arena's middle (the cuboid `+0x88`'s place) kept for the moat's draw ([`draw_moat`] reads it as
/// `0x3fd638` does).
pub const MIDDLE_AT: usize = 0x240;

/// 0x0b59 (`0x3fb1b0`), Maktar's arena master (module docs).
pub fn update_master(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if let Some((p, _)) = cuboid_at(ctx, w, id, pv::MIDDLE) { w.mobys[id].set_pv_v(MIDDLE_AT, p); }
    let fighting = w.mobys[id].state == 8;
    ctx.game.arena_wipe = None;
    ctx.game.arena_results = None;
    // `0x1a9010`: set on every tick, cleared at the desk (state 1); the lava's electric death reads it.
    ctx.game.arena_on = w.mobys[id].state != 1;
    match w.mobys[id].state {
        0 => {
            reset_master(w, id);
            put_crates_away(w, id, ctx);
            ctx.game.arena_menu = None;
        }
        1 => {
            // The desk: Ratchet already in the arena (a restart there), the overview; at the desk (the sphere, or once
            // the first challenge is won the cuboid too) the prompt and △ to enter.
            let hero = ctx.hero.pos;
            let m = &w.mobys[id];
            if ctx.level.in_cylinder(m.pv_i(pv::ARENA), hero) {
                w.mobys[id].set_state(4, -1);
            } else if ctx.level.in_cuboid(m.pv_i(pv::DESK), hero) || (wins(ctx.game, 0) != 0 && ctx.level.in_cylinder(m.pv_i(pv::DESK_WON), hero)) {
                ctx.game.prompt.request(DESK_OWNER, DESK_TEXT, 1);
                if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 && ctx.game.prompt.owner == DESK_OWNER {
                    w.mobys[id].set_state(2, -1);
                    w.mobys[id].timer = 0;
                }
            }
        }
        2 => enter(w, id, ctx),
        s @ (4 | 5) => {
            if s == 5 && w.mobys[id].state_ticks < 0xb4 {
                raise_group(w, id, pv::B89, 0);
                raise_group(w, id, pv::PLATES, 0);
                keep_in(w, id, ctx);
            } else {
                if w.mobys[id].first() {
                    if s == 5 {
                        let n = w.mobys[id].pv_s(pv::CHALLENGE) as i32;
                        reward(w, id, ctx, n);
                    }
                    put_crates_away(w, id, ctx);
                    pool_all(w, id, ctx);
                    raise_group(w, id, pv::B89, 0);
                    raise_group(w, id, pv::PLATES, 0);
                    if let Some((p, _)) = cuboid_at(ctx, w, id, pv::MIDDLE) {
                        ctx.hero.teleport(p, [0.0; 3]);
                        super::spiderbot::end(w, false, 0);
                        ctx.hero.request_state(0x6f, true);
                        let (p, r) = (ctx.hero.pos, ctx.hero.rot);
                        ctx.game.save_checkpoint(p, r);
                    }
                    menu_camera(ctx);
                }
                // `0x3fed68`: the camera come to rest, the menu (4) or the results (5).
                let (eye, _) = ctx.game.cameras.view_euler();
                if crate::hero::physics::dist2(eye, MENU_EYE) < 0.0001 {
                    w.mobys[id].set_state(if s == 4 { 7 } else { 6 }, -1);
                } else if !ctx.game.cameras.script_current() {
                    menu_camera(ctx);
                }
            }
        }
        6 => {
            // After a win: its screen ([`draw_results`]); ✕ the menu at the next challenge, △ back to the resort.
            if ctx.hero.state != 0x6f { ctx.hero.request_state(0x6f, true); }
            // Its first tick, the challenge's win not counted: the announcer's line on it (`0x3fb1b0`).
            let ch = w.mobys[id].pv_s(pv::CHALLENGE) as i32;
            if w.mobys[id].first() && wins(ctx.game, ch) == 0 {
                let n = match ch {
                    0 => Some(10),
                    1 | 2 | 4 | 5 | 6 | 10 => Some(0xb + ctx.game.rng.randi(2)),
                    3 => Some(0xd),
                    _ => None,
                };
                if let Some(n) = n { say(w, id, ctx, n); }
            }
            let m = &w.mobys[id];
            ctx.game.arena_results = Some(Results { first: m.pv_s(pv::SELECTED) == 0 && wins(ctx.game, 0) < 2, prize: m.pv_i(pv::PRIZE) });
            let pressed = ctx.hero.items.pressed;
            if pressed & crate::pad::button::CROSS != 0 {
                w.mobys[id].set_state(7, -1);
                let sel = w.mobys[id].pv_s(pv::SELECTED) as i32;
                if let Some(n) = (sel + 1..0xe).find(|&n| on_menu(&ctx.level.arena, ctx.game, n)) { w.mobys[id].set_pv_s(pv::SELECTED, n as i16); }
            } else if pressed & crate::pad::button::TRIANGLE != 0 {
                leave(w, id, ctx);
            }
        }
        7 => menu(w, id, ctx),
        8 => fight(w, id, ctx),
        10 => {
            ctx.game.arena_menu = None;
            if 0x167 < w.mobys[id].state_ticks {
                w.mobys[id].set_state(4, -1);
            } else {
                raise_group(w, id, pv::B89, 0);
                raise_group(w, id, pv::PLATES, 0);
                keep_in(w, id, ctx);
            }
        }
        0xb => {
            // The first win of the first challenge: its scene (the trigger `+0x10c` hooked after 3 seconds), then the
            // reward and the overview.
            if w.mobys[id].first() {
                raise_group(w, id, pv::B89, 0);
                raise_group(w, id, pv::PLATES, 0);
                w.mobys[id].timer = 0;
            }
            if w.mobys[id].timer == 0 {
                if 0xb3 < w.mobys[id].state_ticks {
                    if let Some(t) = placed(w, w.mobys[id].pv_i(pv::FIRST_WIN)) { w.mobys[t].once |= 4; }
                    w.mobys[id].timer = 1;
                } else {
                    keep_in(w, id, ctx);
                }
            } else if 0 < ctx.hero.hp && !(ctx.game.scenes.mode == crate::scene::SCENE) && 0xf0 < w.mobys[id].state_ticks {
                w.mobys[id].set_state(5, -1);
                w.mobys[id].once &= !1;
            }
        }
        _ => {}
    }
    // The moat (`0x4014f0`) in a challenge, its first three seconds after a win, its lost state and the first win's;
    // the lamps' glows (`0x401c20`) past the desk.
    let s = w.mobys[id].state;
    if s == 8 || (s == 5 && w.mobys[id].state_ticks < 0xb4) || s == 10 || s == 0xb { w.register_draw_first(id); }
    if 1 < s { w.register_draw(id); }
    // The challenge's HUD (`0x3ffee0`, registered on the first list by the challenge's tick).
    ctx.game.arena_hud = fighting.then(|| challenge_hud(w, id, &ctx.level.arena));
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    // `0x3fc4c8`: the remarks' wait (`+0xe4`) counts down.
    let mut t = m.pv_i(pv::REMARK_WAIT);
    crate::moby::countdown(&mut t);
    m.set_pv_i(pv::REMARK_WAIT, t);
}

/// What the challenge's HUD (`0x3ffee0`) reads from the master.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChallengeHud {
    pub challenge: i16,
    /// `+0x122` and the challenge's wave count.
    pub wave: i16,
    pub waves: i16,
    /// `+0xd4`: the challenge's ticks.
    pub ticks: i32,
    /// The timed challenges' count: 15 a wave past, the wave's sent (`+0x90..+0xa4`) less those alive (`+0x60`).
    pub count: i32,
    /// The boss (`+0xd0`): its class and its health over its most.
    pub boss: Option<(i16, f32)>,
}

fn challenge_hud(w: &MobyWorld, id: usize, cs: &[Challenge]) -> ChallengeHud {
    let m = &w.mobys[id];
    let (challenge, wave) = (m.pv_s(pv::CHALLENGE), m.pv_s(pv::WAVE));
    let sent: i32 = (0..6).map(|k| m.pv_i(pv::SPAWNED + 4 * k)).sum();
    // The boss's health over its most (its pvars' `+0x20` / `+0x24`).
    let boss = ai::link(m, pv::BOSS).and_then(|b| w.mobys.get(b)).map(|b| (b.o_class, b.pv_f(0x20) / b.pv_s(0x24) as f32));
    ChallengeHud {
        challenge,
        wave,
        waves: cs.get(challenge as usize).map_or(0, |c| c.waves.len() as i16),
        ticks: m.pv_i(pv::TICKS),
        count: wave as i32 * 15 + sent - m.pv_i(pv::ALIVE),
        boss,
    }
}

/// The crowd's class.
pub const CROWD: i16 = 0x0e96;
/// `$gp+0x3470..+0x3480`: the crowd's score levels (below 2 the wave, under 3 calm again, above 4 cheering), the
/// wave's round (frames) and its width either side (4°).
const CROWD_LEVELS: [f32; 3] = [2.0, 3.0, 4.0];
const WAVE_FRAMES: i32 = 600;
const WAVE_WIDTH: f32 = f32::from_bits(0x3d8e_fa36);

/// 0x0e96 (`0x4301b0`): one of the crowd in the stands (60 placed), following its master's score (`+0x6c` of the
/// moby its `+0x0` names) while the master is past its state 1: in view (`0x2f4008(255, its place, 5)`) its draw
/// distance 0xff and it acts, else 0 and it waits. **1** sitting (sequence 0 / 1 at random on sitting down): a score
/// above 4, one tick in 60, it cheers (**2**: sequence 2 / 3, back to 1 one tick in 60 under 3); under 2, one tick in
/// 60, the wave (**3**): round the arena's middle (500, 600) once every 600 frames, it stands (sequence 4) while the
/// wave or its opposite is within 4° of it, else sits on; back to 1 one tick in 60 above 3.
pub fn update_crowd(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let master = usize::try_from(w.mobys[id].pv_i(0)).ok().filter(|&ms| w.mobys.get(ms).is_some_and(|m| 1 < m.state));
    let Some(ms) = master else { return };
    let p = w.mobys[id].pos;
    if ctx.view.is_some_and(|v| v.culled(255.0, [p[0], p[1], p[2], 5.0])) {
        w.mobys[id].draw_dist = 0;
        return;
    }
    w.mobys[id].draw_dist = 0xff;
    // `0x430570`: the start.
    if w.mobys[id].state == 0 {
        w.mobys[id].set_state(1, -1);
        let s = if ctx.game.rng.randi(2) == 0 { 1 } else { 0 };
        set_seq(w, id, ctx, s, 0, 10);
    }
    let score = w.mobys[ms].pv_f(pv::SCORE);
    let sit_again = |w: &MobyWorld, ctx: &mut Ctx| {
        // Each sitting sequence's end: another at random (two draws, as the original).
        let now = super::props::seq(w, id, ctx) as i32;
        if now != ctx.game.rng.randi(2) {
            let s = ctx.game.rng.randi(2) as u8;
            ctx.host.set_anim(&w.mobys[id], s, 0, 10);
        }
    };
    match w.mobys[id].state {
        1 => {
            if w.mobys[id].first() { sit_again(w, ctx); }
            if CROWD_LEVELS[2] < score && ctx.game.rng.randi(0x3c) == 0 {
                w.mobys[id].set_state(2, -1);
            } else if score < CROWD_LEVELS[0] && ctx.game.rng.randi(0x3c) == 0 {
                w.mobys[id].set_state(3, -1);
            }
        }
        2 => {
            if w.mobys[id].first() {
                let now = super::props::seq(w, id, ctx) as i32;
                if now != ctx.game.rng.randi(2) + 2 {
                    let s = ctx.game.rng.randi(2) as u8 + 2;
                    ctx.host.set_anim(&w.mobys[id], s, 0, 10);
                }
                return;
            }
            if score < CROWD_LEVELS[1] && ctx.game.rng.randi(0x3c) == 0 { w.mobys[id].set_state(1, -1); }
        }
        3 => {
            let wave = ((ctx.game.frame % WAVE_FRAMES) as f32 * -std::f32::consts::TAU) / WAVE_FRAMES as f32 + std::f32::consts::PI;
            let me = atan2(p[0] - 500.0, p[1] - 600.0);
            let near = |a: f32| crate::hero::physics::rot_dist(me, a) < WAVE_WIDTH;
            if near(wave) || near(add_rot(wave, std::f32::consts::PI)) {
                set_seq(w, id, ctx, 4, 0, 10);
            } else if wrapped(w, id, ctx) {
                sit_again(w, ctx);
            }
            if CROWD_LEVELS[1] < score && ctx.game.rng.randi(0x3c) == 0 { w.mobys[id].set_state(1, -1); }
        }
        _ => {}
    }
}

/// What the win's screen (`0x400e10`) shows: the first challenge's first win (its own line) or the prize (`+0x110`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Results {
    pub first: bool,
    pub prize: i32,
}

/// `0x400e10` (the master's draw after a win, on the big screen): the title 0x2ddb (font 1 at 1.1, (180, 93)), the
/// first win's line 0x2ddc or the prize (`"%s %d %s"`: 0x2df8, the bolts, 0x2df4) in font 3 centred on (260, 135),
/// and 0x2ddd in font 1 centred on (250, 160), in the box 10..500 × 125..200 (`$gp+0x2468..+0x24b8`; the title's x
/// by language, the first's here), all `0x90ffffff`, on the panel ([`draw_panel`], no arrows).
pub fn draw_results(r: &Results, text: &dyn Fn(u32) -> Vec<u8>, fonts: [(&[rc_formats::disc::boot::Glyph; 256], usize); 3], draws: &mut Vec<crate::hud::Draw>, out: &mut Vec<crate::gui::Quad>) {
    use crate::gui::text_box::{CENTRE, MIDDLE};
    draw_panel(None, draws);
    let first = |id: u32| { let t = text(id); t.split(|&c| c == 0).next().unwrap_or(&[]).to_vec() };
    let colour = 0x90ff_ffffu32.to_le_bytes();
    let (g1, s1) = fonts[0];
    crate::gui::text(out, g1, s1, &first(0x2ddb), 180.0, 93.0, 1.1, colour);
    let boxed = |x: i32, y: i32| crate::gui::TextBox { top: 125, bottom: 200, left: 10, right: 500, x, y, line: 16, flags: CENTRE | MIDDLE, ..Default::default() };
    let line = if r.first { first(0x2ddc) } else { [first(0x2df8), format!(" {} ", r.prize).into_bytes(), first(0x2df4)].concat() };
    let (g3, s3) = fonts[2];
    crate::gui::text_in_box(out, g3, s3, &mut boxed(260, 135), &line, colour);
    crate::gui::text_in_box(out, g1, s1, &mut boxed(250, 160), &first(0x2ddd), colour);
}

/// The HUD's font 3: FX texture 3, its glyphs the boot's `0x264250` (crate::front_end::Assets::glyphs_3).
pub const FONT_3: u16 = 3;
/// `$gp+0x2368`: the HUD's text colour.
const HUD_TEXT: u32 = 0x80f0_f0f0;
/// `$gp+0x2454` / `+0x2450`: the panels' back and outline colours; `+0x2420` the boss's picture's.
const PANEL_BACK: u32 = 0x6044_2d00;
const PANEL_LINE: u32 = 0x55f0_c070;
const BOSS_PICTURE: u32 = 0x8080_8080;
/// `$gp+0x2340..+0x2360` (`0x3ffd90`): the boss bar's left, top, right and bottom, its colours (full, half, empty).
const BOSS_BAR: [f32; 4] = [74.0, 388.0, 191.0, 400.0];
const BOSS_BAR_COLOURS: [u32; 3] = [0x8000_aa00, 0x8000_aaaa, 0x8000_00aa];

/// A text box of the HUD (`0x2ee560(box, top, bottom, left, right, x, y, 0, 3)`: centred on `x` and on `y`), the first
/// layout's (`$gp+0x2368..`; the second, for the languages past the second, is not ported).
fn hud_box(top: i32, bottom: i32, left: i32, right: i32, x: i32, y: i32) -> crate::gui::TextBox {
    use crate::gui::text_box::{CENTRE, MIDDLE};
    crate::gui::TextBox { top, bottom, left, right, x, y, flags: CENTRE | MIDDLE, ..Default::default() }
}

/// `0x3ffee0` (the master, first list, in a challenge): the panels along the screen's bottom. A timed challenge
/// (3 / 5) shows its seconds left on the right (30 / 60 less the elapsed, font 3, with "time" 0x2df9 in font 2) and
/// its count on the left (font 3, with 0x2df7 in font 1); challenge 12 its seconds left of 120 only (font 1 from
/// 100); the others the wave of their count on the right (`"%2d/%2d"`, font 3, with 0x2df6 in font 1) and, with a
/// boss, its bar and picture on the left (0x09f8 frame 2, 0x0a44 frame 1). The panels are the GUI sprites 0x4c / 0x4d
/// (0x4e / 0x4f for the wave's), the right one mirrored. `fonts`: the glyphs and texture slot of fonts 1, 2 and 3.
///
/// Not ported: the draw's depth word (`0x1beab0` = 0.62 while it draws) and its GS state (`0x384420`, `0x384178`),
/// PAL's screen scale (`0x1b3e04`) and the bar's PAL offset (`$gp+0x2350`).
pub fn draw_hud(h: &ChallengeHud, text: &dyn Fn(u32) -> Vec<u8>, fonts: [(&[rc_formats::disc::boot::Glyph; 256], usize); 3], draws: &mut Vec<crate::hud::Draw>, out: &mut Vec<crate::gui::Quad>) {
    use crate::hud::Draw;
    let first = |id: u32| { let t = text(id); t.split(|&c| c == 0).next().unwrap_or(&[]).to_vec() };
    let colour = HUD_TEXT.to_le_bytes();
    let mut print = |font: usize, mut b: crate::gui::TextBox, t: &[u8]| {
        let (g, slot) = fonts[font - 1];
        crate::gui::text_in_box(out, g, slot, &mut b, t, colour);
    };
    let panel = |draws: &mut Vec<Draw>, ids: [u32; 2], x: f32, sx: f32| {
        for (id, colour) in [(ids[0], PANEL_BACK), (ids[1], PANEL_LINE)] {
            draws.push(Draw::Sprite { id, pos: [x, 407.0], scale: [sx, 1.0], frame: 0.0, colour });
        }
    };
    let (seconds, timed) = match h.challenge { 3 => (30, true), 5 => (60, true), 0xc => (120, false), _ => (0, false) };
    if h.challenge == 3 || h.challenge == 5 || h.challenge == 0xc {
        let left = seconds - h.ticks / 60;
        panel(draws, [0x4c, 0x4d], 502.0, -1.0);
        let font = if h.challenge == 0xc && 100 <= left { 1 } else { 3 };
        print(font, hud_box(380, 400, 430, 500, 474, 380), left.to_string().as_bytes());
        print(2, hud_box(380, 400, 330, 430, 380, 384), &first(0x2df9));
        if timed {
            panel(draws, [0x4c, 0x4d], 10.0, 1.0);
            print(3, hud_box(380, 400, 25, 70, 40, 380), h.count.to_string().as_bytes());
            print(1, hud_box(380, 400, 70, 200, 130, 384), &first(0x2df7));
        }
        return;
    }
    if let Some((class, fraction)) = h.boss {
        panel(draws, [0x4c, 0x4d], 10.0, 1.0);
        draws.push(Draw::Sprite { id: 0x50, pos: [12.0, 407.0], scale: [1.0, 1.0], frame: 0.0, colour: PANEL_LINE });
        let frame = match class { 0x09f8 => Some(2.0), 0x0a44 => Some(1.0), _ => None };
        if let Some(frame) = frame {
            // `0x3ffd90(fraction)`: the bar, red to yellow to green.
            let f = fraction.max(0.0);
            let width = ((BOSS_BAR[2] - BOSS_BAR[0]) * f) as i32 as f32;
            let c = if 0.5 < f {
                crate::hud::lerp_rgba(f + f - 1.0, BOSS_BAR_COLOURS[1], BOSS_BAR_COLOURS[0])
            } else {
                crate::hud::lerp_rgba(f + f, BOSS_BAR_COLOURS[2], BOSS_BAR_COLOURS[1])
            };
            draws.push(Draw::Bar { pos: [BOSS_BAR[0], BOSS_BAR[1]], size: [width, BOSS_BAR[3] - BOSS_BAR[1]], colours: [c, c] });
            draws.push(Draw::Sprite { id: 0x51, pos: [10.0, 406.0], scale: [1.0, 1.0], frame, colour: BOSS_PICTURE });
        }
    }
    panel(draws, [0x4e, 0x4f], 502.0, -1.0);
    print(3, hud_box(380, 400, 420, 500, 457, 380), format!("{:2}/{:2}", h.wave + 1, h.waves).as_bytes());
    print(1, hud_box(380, 400, 320, 420, 360, 384), &first(0x2df6));
}

/// State 2: through the wipe into the arena — chunk 1 with the arena's music, Ratchet at the entry cuboid `+0x8c`, the
/// checkpoint, the arena's music asked for, the camera behind him; the first time the announcer's welcome (the trigger
/// `+0x108` hooked) and straight into the first challenge; else the overview once he has walked to the middle.
fn enter(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        w.mobys[id].timer = 0;
        w.mobys[id].state_ticks = 0;
    }
    match w.mobys[id].timer {
        0 => {
            // The fade to black, a twentieth a tick.
            let t = w.mobys[id].state_ticks;
            if t < 0x1f {
                let f = t as u16 as f32 * 0.05;
                ctx.game.fade_black = if f < 1.0 { f } else { 1.0 };
                return;
            }
            // The wipe from the screen's full height (`0x401a20`, [`draw_wipe`]).
            w.mobys[id].set_pv_s(pv::WIPE, crate::gui::SCREEN_H as i16);
            wipe_tick(w, id, ctx);
            ctx.switch_chunk(1, w.mobys[id].pv_i(pv::MUSIC_ARENA));
            if let Some((p, r)) = cuboid_at(ctx, w, id, pv::ENTRY) {
                ctx.hero.teleport(p, r);
                super::spiderbot::end(w, false, 0);
                ctx.hero.request_state(0x6f, true);
                let (p, r) = (ctx.hero.pos, ctx.hero.rot);
                ctx.game.save_checkpoint(p, r);
                ctx.game.cameras.camera_script([p[0], p[1], p[2] + 2.0], r, crate::script_camera::MODE_SNAP, 0, false, ctx.hero);
            }
            let m = &w.mobys[id];
            ctx.host.music_request(m.pv_i(pv::MUSIC_ARENA), m.pv_i(pv::STINGER_ARENA));
            w.mobys[id].timer = 2;
        }
        2 => {
            // Through the wipe, the black fade off.
            let h = wipe_tick(w, id, ctx);
            ctx.game.fade_black = 0.0;
            if 0 < h { return; }
            if ctx.game.flags[5] == 0 {
                ctx.game.flags[5] = 1;
                w.mobys[id].timer = 1;
                if let Some(t) = placed(w, w.mobys[id].pv_i(pv::INTRO)) { w.mobys[t].once |= 4; }
                w.mobys[id].state_ticks = 0;
            } else {
                to_middle(w, id, ctx);
            }
        }
        1 => {
            if 0x3c < w.mobys[id].state_ticks && !(ctx.game.scenes.mode == crate::scene::SCENE) { to_middle(w, id, ctx); }
        }
        3 => {
            if wins(ctx.game, 0) == 0 {
                w.mobys[id].set_pv_s(pv::CHALLENGE, 0);
                w.mobys[id].set_pv_s(pv::WAVE, 0);
                w.mobys[id].set_state(8, -1);
                ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_BLEND, ctx.hero);
                ctx.hero.request_state(0, true);
                return;
            }
            let Some((mid, _)) = cuboid_at(ctx, w, id, pv::MIDDLE) else { return };
            if dist(ctx.hero.pos, mid) < 10.0 {
                w.mobys[id].set_state(4, -1);
            } else if 0x3c < w.mobys[id].state_ticks {
                ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_BLEND, ctx.hero);
            }
        }
        _ => {}
    }
}

/// The wipe's registration for this tick: its draw ([`draw_wipe`]) lowers `+0x124` by its step (to 0) and draws
/// that; returns the height before.
fn wipe_tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> i16 {
    let before = w.mobys[id].pv_s(pv::WIPE);
    let h = (before as u16).wrapping_sub(WIPE_STEP) as i16;
    let h = if h < 0 { 0 } else { h };
    w.mobys[id].set_pv_s(pv::WIPE, h);
    ctx.game.arena_wipe = Some(h);
    before
}

/// `$gp+0x2534..+0x2550` (the wipe `0x401a20`): the curtain's colour, the flash's (its alpha by the square of the
/// curtain's height over the screen's) and its draws, the step a frame, the edge's most height and its overhang.
const WIPE_CURTAIN: u32 = 0xff00_0000;
const WIPE_FLASH: u32 = 0xffff_ffff;
const WIPE_FLASHES: usize = 4;
const WIPE_STEP: u16 = 2;
const WIPE_EDGE: i32 = 150;
const WIPE_OVERHANG: i32 = 100;
/// The wipe's edge: FX texture 0x10, its 32 × 32 texels.
pub const WIPE_FX: u16 = 0x10;

/// `0x401a20(m)` (the master, first list, entering the arena): the flash over the screen ([`WIPE_FLASHES`] times),
/// the black curtain over its top `h` rows, and its edge (FX 0x10, `edge` its texture slot) across the curtain's
/// bottom, overhanging both sides, as tall as the screen opened (at most 150) and centred on the curtain's edge.
///
/// Not ported: the edge quad's colour (`0x2ebee8`'s stack arguments; neutral 0x80808080 here) and the draw's GS states
/// (`0x384178`).
pub fn draw_wipe(h: i16, edge: Option<usize>, out: &mut Vec<crate::gui::Quad>) {
    use crate::gui::{rect, Quad, Tex, SCREEN_H, SCREEN_W};
    let (sw, sh) = (SCREEN_W as i32, SCREEN_H as i32);
    let h = h as i32;
    let f = h as f32 / sh as f32;
    // An alpha past 0x80 on the GS is a blend factor past 1 with the colour clamped: on pure white or black sources it
    // comes to the same as 0x80.
    let gs = |w: u32| { let mut c = w.to_le_bytes(); c[3] = c[3].min(0x80); c };
    let a = (f * f * (WIPE_FLASH >> 24) as f32) as u32;
    let flash = gs((WIPE_FLASH & 0xff_ffff) | a << 24);
    for _ in 0..WIPE_FLASHES { rect(out, 0, sh, 0, sw, flash); }
    rect(out, 0, h, 0, sw, gs(WIPE_CURTAIN));
    if let Some(slot) = edge {
        let tall = (sh - h).min(WIPE_EDGE);
        let (x, y, wd) = (-WIPE_OVERHANG, h - (tall >> 1), sw + WIPE_OVERHANG * 2);
        let (l, t, r, b) = (x as f32, y as f32, (x + wd) as f32, (y + tall) as f32);
        out.push(Quad { pos: [[l, t], [r, t], [l, b], [r, b]], uv: [[0.0, 0.0], [32.0, 0.0], [0.0, 32.0], [32.0, 32.0]], tex: Tex::Slot(slot), rgba: [0x80; 4] });
    }
}

/// Sub 3 of the entry: Ratchet walked toward the middle (`0x2d7940(0, middle, 0)`).
fn to_middle(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].timer = 3;
    ctx.hero.request_state(0, true);
    if let Some((p, _)) = cuboid_at(ctx, w, id, pv::MIDDLE) { ctx.hero.script_walk_to(0.0, p, false); }
}

/// State 7, `0x3fee58`: the menu — on its first tick the announcer's line 0 or 1 (`0x3fb1b0`); ← / → the challenges
/// open (auto-repeat every 15 ticks) with the level's sound 3 (`0x367f58(3, 0, 0)`: → when it found one, ← from any but
/// the first, twice from the second), ✕ the selected one starts (its sound 4), △ back to the resort.
fn menu(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        let n = ctx.game.rng.randi(2);
        say(w, id, ctx, n);
    }
    if ctx.hero.state != 0x6f { ctx.hero.request_state(0x6f, true); }
    let pressed = ctx.hero.items.pressed;
    let held = ctx.hero.items.held;
    let sel = w.mobys[id].pv_s(pv::SELECTED) as i32;
    let mut rep = w.mobys[id].pv_s(pv::REPEAT) as i32;
    let right = held & crate::pad::button::RIGHT != 0 && (pressed & crate::pad::button::RIGHT != 0 || rep == 0);
    let left = held & crate::pad::button::LEFT != 0 && (pressed & crate::pad::button::LEFT != 0 || rep == 0);
    if right || left {
        let next = if right { (sel + 1..0xe).find(|&n| on_menu(&ctx.level.arena, ctx.game, n)) } else { (0..sel).rev().find(|&n| on_menu(&ctx.level.arena, ctx.game, n)).or(Some(0)) };
        if let Some(n) = next.filter(|&n| n != sel) {
            w.mobys[id].set_pv_s(pv::SELECTED, n as i16);
        }
        let sounds = if right { next.is_some() as usize } else { [0, 2, 1][(sel.clamp(0, 2)) as usize] };
        for _ in 0..sounds { ctx.host.play_level_sound(None, 3, 0); }
        rep = 0xf;
    } else if rep != 0 {
        rep -= 1;
    }
    w.mobys[id].set_pv_s(pv::REPEAT, rep as i16);
    let sel = w.mobys[id].pv_s(pv::SELECTED) as i32;
    ctx.game.arena_menu = Some(Menu {
        selected: sel,
        prize: (sel != 0 || wins(ctx.game, 0) != 0).then(|| prize(&ctx.level.arena, ctx.game, sel)),
        wins: wins(ctx.game, sel),
        prev: sel != 0,
        next: (sel + 1..0xe).any(|n| on_menu(&ctx.level.arena, ctx.game, n)),
        held: ctx.hero.items.held,
        frame: ctx.game.frame,
    });
    if pressed & crate::pad::button::CROSS != 0 && on_menu(&ctx.level.arena, ctx.game, sel) {
        ctx.game.flags[0x3c] = sel as u8;
        let m = &mut w.mobys[id];
        m.set_pv_s(pv::CHALLENGE, sel as i16);
        m.set_pv_s(pv::WAVE, 0);
        m.set_state(8, -1);
        ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_BLEND, ctx.hero);
        ctx.hero.request_state(0, true);
        ctx.host.play_level_sound(None, 4, 0);
        ctx.game.arena_menu = None;
    } else if pressed & crate::pad::button::SQUARE != 0 {
        // `0x332158(5, 1, 0, 0, 0)`: the vendor (its weapons) with no vendor moby; the menu's draw not this tick.
        ctx.game.vendor_virtual = Some(0);
        ctx.game.arena_menu = None;
    } else if pressed & crate::pad::button::TRIANGLE != 0 {
        leave(w, id, ctx);
    }
}

/// State 8: the challenge (module docs).
fn fight(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let ch = w.mobys[id].pv_s(pv::CHALLENGE) as i32;
    if w.mobys[id].first() {
        let m = &mut w.mobys[id];
        m.set_pv_s(pv::WAVE, 0);
        for k in 0..6 { m.set_pv_i(pv::SPAWNED + 4 * k, 0); }
        m.set_pv_i(pv::TICKS, 0);
        m.set_pv_i(pv::ALIVE, 0);
        for k in 0..pv::GATE_SLOTS { m.set_pv_i(pv::GATES + 4 * k, 0); }
        m.set_pv_i(pv::GATE, 0);
        put_crates_away(w, id, ctx);
        count_out(w, id, ctx);
        wave_start(w, id, ctx);
        let m = &mut w.mobys[id];
        for k in 0..0x28 { m.set_pv_s(pv::SAID + 2 * k, 0); }
        for (k, &t) in TURNS.iter().enumerate() { m.set_pv_i(pv::TURNS + 4 * k, t); }
        if ch == 7 { if let Some(a) = ctx.game.ammo.get_mut(WEAPON_7 as usize) { *a = 300; } }
        if ch == 8 { if let Some(a) = ctx.game.ammo.get_mut(WEAPON_8 as usize) { *a = 8; } }
        ctx.hero.hp = ctx.game.max_health;
    }
    score(w, id, ctx);
    let wave = wave_of(w, id, &ctx.level.arena);
    if wave.is_some_and(|wv| wave_over(w, id, &wv)) {
        let waves = ctx.level.arena.get(ch as usize).map_or(0, |c| c.waves.len() as i16);
        let m = &mut w.mobys[id];
        m.set_pv_s(pv::WAVE, m.pv_s(pv::WAVE) + 1);
        for k in 0..6 { m.set_pv_i(pv::SPAWNED + 4 * k, 0); }
        m.set_pv_i(pv::ALIVE, 0);
        m.set_pv_i(pv::BOSS, 0);
        if waves <= m.pv_s(pv::WAVE) {
            if ch == 0 && wins(ctx.game, 0) == 0 { m.set_state(0xb, -1); } else { m.set_state(5, -1); }
            m.set_pv_s(pv::SELECTED, ch as i16);
            // `0x3fb028(1, −1)`: a Spiderbot out blows up.
            super::spiderbot::end(w, true, -1);
            return;
        }
        wave_start(w, id, ctx);
    }
    let ticks = w.mobys[id].pv_i(pv::TICKS) + 1;
    w.mobys[id].set_pv_i(pv::TICKS, ticks);
    let held = ctx.hero.items.item;
    let shots = |n: i32| ctx.game.ammo.get(n as usize).copied().unwrap_or(0);
    if (ch == 3 && 0x708 < ticks) || (ch == 5 && 0xe10 < ticks) {
        let n = fail_kills(w, id);
        ctx.game.prompt.announce_count(FAIL_KILLS.0, n, FAIL_KILLS.1, -1);
        lose(w, id);
        return;
    }
    let shot_alive = |w: &MobyWorld, cs: &[i16]| w.mobys.iter().any(|m| m.state < 0x80 && cs.contains(&m.o_class));
    if ch == 7 && held == WEAPON_7 && shots(WEAPON_7) < 1 && !shot_alive(w, &[0x0c89, 0x0bfe]) {
        ctx.game.prompt.announce(FAIL_SHOTS_7, -1);
        lose(w, id);
        return;
    }
    if ch == 8 && held == WEAPON_8 && shots(WEAPON_8) < 1 && !shot_alive(w, &[0x0c06, 0x0c07]) {
        ctx.game.prompt.announce(FAIL_SHOTS_8, -1);
        lose(w, id);
        return;
    }
    if (ch == 0xc || ch == 0xd) && ctx.hero.hp < ctx.game.max_health {
        w.mobys[id].set_pv_i(pv::BOSS, 0);
        ctx.game.prompt.announce(FAIL_HURT, -1);
        lose(w, id);
        return;
    }
    if ch == 0xc && 0x1c20 < ticks {
        w.mobys[id].set_pv_i(pv::BOSS, 0);
        w.mobys[id].set_state(5, -1);
        super::spiderbot::end(w, true, -1);
        return;
    }
    // Sending fighters: after a second (or at once with a boss), the round's spawner, the wave's fighters into it
    // (0x0c3e, 0x0b22, 0x0c3b, 0x09ca), then its doors opened.
    if let Some(wave) = wave_of(w, id, &ctx.level.arena) {
        if 0x3c < w.mobys[id].state_ticks || wave.bosses != [0, 0] {
            gates_reset(w, id);
            gate_pick(w, id, ctx, &wave);
            let mut sent = false;
            if gate(w, id).is_some_and(|g| ready(w, g)) {
                for (k, class) in [(3, 0x0c3e), (1, 0x0b22), (2, 0x0c3b), (0, 0x09ca)] {
                    while w.mobys[id].pv_i(pv::SPAWNED + 4 * k) < wave.fighters[k] as i32 {
                        if !send(w, id, class) { break; }
                        sent = true;
                    }
                }
            }
            // `0x3fc148..0x3fc364`: the wave's bosses (crate::moby::classes::bosses), the announcer's line, the bosses'
            // music.
            if let Some(n) = super::bosses::send_wave_bosses(w, id, ctx, wave.bosses) {
                say(w, id, ctx, n as i32);
                let m = &w.mobys[id];
                ctx.host.music_request(m.pv_i(pv::MUSIC_BOSS), m.pv_i(pv::STINGER_BOSS));
            }
            // `0x3fc364`: a wave without bosses asks for the arena's music unless it plays or is pending (the low bytes
            // `0x1a63f0` / `0x1a63e4` against the whole word).
            let track = w.mobys[id].pv_i(pv::MUSIC_ARENA);
            if wave.bosses == [0, 0] && ctx.host.music_tracks().is_none_or(|(now, next)| now as i32 != track && next as i32 != track) {
                ctx.host.music_request(track, w.mobys[id].pv_i(pv::STINGER_ARENA));
            }
            if sent { open_gates(w, id, ctx); }
        }
    }
    match ch {
        7 => { ctx.hero.items.ask(ctx.game.held, WEAPON_7); }
        8 => { ctx.hero.items.ask(ctx.game.held, WEAPON_8); }
        0xd => { ctx.hero.items.ask(ctx.game.held, 10); }
        _ => {}
    }
    boss_remarks(w, id, ctx);
    if ctx.game.frame & 3 == 0 { deal_turns(w, id, ctx); }
    keep_in(w, id, ctx);
}

// ---------------------------------------------------------------------------------------------------------------
// The flame columns 0x0b89 and the crate lifts 0x0ccc

pub const COLUMN: i16 = 0x0b89;
pub const LIFT: i16 = 0x0ccc;
/// `$gp+0x2810..+0x2838`: the column's rest (60), the gap before a warning (60 + 60), the warnings (2), its burn (60),
/// its height (5), its reach at the floor and at the top (1, 0.5), its glow's push and sizes, colour.
const COLUMN_REST: i16 = 60;
const COLUMN_GAP: i16 = 60;
const COLUMN_BLINK: i16 = 60;
const COLUMN_WARNINGS: i16 = 2;
const COLUMN_BURN: i16 = 60;
const COLUMN_HEIGHT: f32 = 5.0;
const COLUMN_REACH: [f32; 2] = [1.0, 0.5];
const COLUMN_GLOW: (f32, f32, f32, u32) = (0.2, 0.3, 0.2, 0x66aa_6600);

/// `0x403be8`: the column is on.
pub fn column_on(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_s(0x30) != 0 }

/// `0x403bf8` / `0x403c08`: on / off.
pub fn column_set(w: &mut MobyWorld, id: usize, on: bool) { w.mobys[id].set_pv_s(0x30, on as i16); }

/// 0x0b89 (`0x403620`): a flame column in the floor. Off (1) it waits; on, after its rest it warns (2: its glow
/// flashes twice, 60 ticks apart) and burns (3) for 60 ticks — in view, Ratchet within 4 is hit for `+0x14` on the
/// column's line (its reach 1 at the floor, 0.5 at its top), else what is at its foot; two rising puffs a tick (the
/// exhaust puffs) and its hum (the level's looped sound 0x11) — then rests again (the hum stopped).
pub fn update_column(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.rows = rows_of(m.rot);
    match m.state {
        0 => {
            m.set_pv_b(9, 0);
            m.set_pv_b(8, 3);
            m.set_pv_i(0x10, 0);
            m.drawn = 0;
            m.mode = (m.mode & !mode::TARGET) | 1;
            m.set_pv_s(0x30, 0);
            m.set_pv_s(0x36, -1);
            m.set_pv_s(0x32, COLUMN_REST);
            m.set_state(1, -1);
            return;
        }
        1 => {
            if m.pv_s(0x32) == 0 {
                if m.pv_s(0x30) == 0 { return; }
                m.set_pv_s(0x34, COLUMN_WARNINGS);
                m.set_pv_s(0x32, COLUMN_GAP + COLUMN_BLINK);
                m.set_state(2, -1);
                return;
            }
            if m.pv_s(0x30) == 0 { return; }
        }
        2 => {
            if m.pv_s(0x34) == 0 {
                m.set_pv_s(0x32, COLUMN_BURN);
                m.set_state(3, -1);
            } else if COLUMN_BLINK < m.pv_s(0x32) {
                w.register_draw(id);
            } else if m.pv_s(0x32) == 0 {
                m.set_pv_s(0x32, COLUMN_GAP + COLUMN_BLINK);
                let mut t = m.pv_s(0x34) as i32;
                crate::moby::countdown(&mut t);
                m.set_pv_s(0x34, t as i16);
            }
        }
        3 => {
            if m.pv_s(0x32) == 0 {
                let h = m.pv_s(0x36) as i32;
                m.set_pv_s(0x32, COLUMN_REST);
                if h != -1 { ctx.host.stop_sound(h); }
                m.set_pv_s(0x36, -1);
                m.set_state(1, -1);
                return;
            }
            let pos = m.pos;
            if !ctx.view.is_some_and(|v| v.culled(128.0, [pos[0], pos[1], pos[2], 1.0])) {
                let dmg = m.pv_b(0x14) as f32;
                if crate::hero::physics::dist(pos, ctx.hero.pos) < 4.0 {
                    let top = [pos[0], pos[1], pos[2] + COLUMN_HEIGHT];
                    let line = sub(top, pos);
                    let l2 = crate::hero::physics::dot(line, line);
                    let k = if 0.0 < l2 { (crate::hero::physics::dot(sub(ctx.hero.pos, pos), line) / l2).clamp(0.0, 1.0) } else { 0.0 };
                    let at = add(pos, crate::hero::physics::scale(line, k));
                    let r = COLUMN_REACH[0] + (COLUMN_REACH[1] - COLUMN_REACH[0]) * k;
                    ai::attack_sphere(w, id, ctx, r, dmg, 1.0, at, 0x1_0001, 0, 1, 0);
                } else {
                    ai::attack_sphere(w, id, ctx, COLUMN_REACH[0], dmg, 1.0, pos, 0x1_0001, 0, 1, 0);
                }
                column_puffs(w, id, ctx);
                w.register_draw(id);
            }
            let m = &mut w.mobys[id];
            let mut t = m.pv_s(0x32) as i32;
            crate::moby::countdown(&mut t);
            m.set_pv_s(0x32, t as i16);
            // Its hum: the level's looped sound 0x11 past the split (`0x367ff0(0x11, 4, m)`) unless its slot `+0x36` still
            // plays it (`0x367ab0`).
            let h = m.pv_s(0x36) as i32;
            if !(0 <= h && ctx.host.sound_playing(h)) {
                let k = ctx.host.play_level_sound_after(Some(&w.mobys[id]), 0x11, 4);
                w.mobys[id].set_pv_s(0x36, k as i16);
            }
            return;
        }
        _ => return,
    }
    let m = &mut w.mobys[id];
    let mut t = m.pv_s(0x32) as i32;
    crate::moby::countdown(&mut t);
    m.set_pv_s(0x32, t as i16);
}

/// `0x403a48`: the burning column's two puffs — one at its foot pulled 0.15 toward the camera rising 20..30 a second,
/// one 0.6 up (jittered 0.2) rising 30..50 a second.
fn column_puffs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let cam = ctx.host.camera_pos();
    let pos = w.mobys[id].pos;
    let mut d = sub(cam, pos);
    d[2] = 0.0;
    let a = add(pos, set_len(d, 0.15));
    let g = &mut ctx.game.rng;
    let va = [0.0, 0.0, g.randf(20.0, 30.0) / 60.0];
    let add_a = g.randi(2) != 0;
    let mut b = [pos[0], pos[1], pos[2] + 0.6];
    for k in 0..3 { b[k] += g.randf(-0.2, 0.2); }
    let vb = [0.0, 0.0, g.randf(30.0, 50.0) / 60.0];
    let add_b = g.randi(2) != 0;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let m = &w.mobys[id];
    for (p, v, desc, additive) in [(a, va, 7, add_a), (b, vb, 6, add_b)] {
        if let Some(i) = crate::particles::exhaust(sys, &mut ctx.game.rng, m, id, p, v, desc) {
            if !additive { sys.pool.recs[i][3] = 0x44; }
        }
    }
}

/// `0x403b88` (0x0b89 warning or burning): its glow at its foot (`0x32bbe0`: a camera-facing quad of FX 0xb, pushed
/// 0.2 toward the camera; 0.2 warning, 0.3 burning).
pub fn draw_column(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let size = match m.state { 3 => COLUMN_GLOW.1, 2 => COLUMN_GLOW.2, _ => return };
    crate::fx::sprite(out, size, COLUMN_GLOW.0, 0.0, m.pos, cam, 0xb, COLUMN_GLOW.3, true);
}

/// `0x417fb8`: a lift free (resting, no crate, not moving).
pub fn lift_free(w: &MobyWorld, id: usize) -> bool { let m = &w.mobys[id]; m.state == 1 && m.pv_i(0) == 0 && m.pv_f(4) == 0.0 }

/// `0x417ed8(rise, spin, lift, crate)`: the crate put back (`0x3a8468`) 2 under the lift, rising `rise` and turning
/// `spin` a tick with it (state 2); the crate's flags 2 (held), its `+0xb0` 1, shown.
pub fn lift(w: &mut MobyWorld, id: usize, c: usize, rise: f32, spin: f32) {
    w.mobys[id].set_state(2, -1);
    let p = w.mobys[id].pos;
    super::crate_::restore(w, c);
    let x = &mut w.mobys[c];
    x.pos = [p[0], p[1], p[2] - 2.0];
    x.mode &= !0x81;
    x.set_pv_f(0xb0, 1.0);
    x.set_pv_i(0xac, 2);
    let m = &mut w.mobys[id];
    ai::set_link(m, 0, Some(c));
    m.set_pv_f(4, rise);
    m.set_pv_f(8, spin);
}

/// 0x0ccc (`0x417bf0`): a crate lift. With its crate broken (state 6) it lets go. Resting (1) it keeps its crate's
/// height (the crate held, flags 2, while it stays); lifting (2: sequence 1) and rising (3: the speed 0 after the
/// sequence) the crate climbs `+4` and turns `+8` a tick until 0.2 over the lift (flags 8: dropped); then down (4,
/// sequence 2) and resting again (sequence 0).
pub fn update_lift(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let crate_ = ai::link(&w.mobys[id], 0).filter(|&c| w.mobys.get(c).is_some_and(|x| x.state < 0x80));
    if let Some(c) = crate_ {
        if w.mobys[c].state == 6 {
            ai::set_link(&mut w.mobys[id], 0, None);
            w.mobys[id].set_pv_f(4, 0.0);
        }
    }
    let crate_ = ai::link(&w.mobys[id], 0);
    let climb = |w: &mut MobyWorld, id: usize, c: usize| {
        let (rise, spin) = (w.mobys[id].pv_f(4), w.mobys[id].pv_f(8));
        let x = &mut w.mobys[c];
        x.pos[2] += rise;
        x.rot[2] = add_rot(x.rot[2], spin);
    };
    match w.mobys[id].state {
        0 => {
            ai::set_link(&mut w.mobys[id], 0, None);
            w.mobys[id].set_pv_f(4, 0.0);
        }
        1 => {
            let Some(c) = crate_ else { return };
            let z = w.mobys[c].pos[2];
            if z != w.mobys[id].pv_f(0xc) {
                w.mobys[id].set_pv_f(0xc, z);
                return;
            }
            w.mobys[c].set_pv_i(0xac, 2);
            return;
        }
        2 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 1, 0, 7);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            let Some(c) = crate_ else {
                w.mobys[id].set_state(4, -1);
                return;
            };
            climb(w, id, c);
            if wrapped(w, id, ctx) { w.mobys[id].set_state(3, -1); }
            return;
        }
        3 => {
            if w.mobys[id].first() { ctx.host.set_anim_speed(&w.mobys[id], 0.0); }
            if let Some(c) = crate_ {
                climb(w, id, c);
                w.mobys[c].set_pv_i(0xac, 2);
                if w.mobys[c].pos[2] <= w.mobys[id].pos[2] + 0.2 { return; }
                w.mobys[c].set_pv_i(0xac, 8);
            }
            w.mobys[id].set_state(4, -1);
            return;
        }
        4 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 2, 0, 7);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            if !wrapped(w, id, ctx) { return; }
            if let Some(c) = crate_ {
                let z = w.mobys[c].pos[2];
                w.mobys[id].set_pv_f(0xc, z);
            }
        }
        _ => return,
    }
    w.mobys[id].set_state(1, -1);
    set_seq(w, id, ctx, 0, 0, 7);
}

/// `$gp+0x2230..+0x225c` (`0x3ff388`): the big screen's panel (GUI sprites 0x49 back / 0x48 outline at (255, 210), 1.2
/// wide) and the menu's arrows (0x4b the previous at (339, 210), 0x4a the next at (168, 210)), pulsing between the
/// panel's colours every 50 frames, `0x80dddddd` while their way (← / →) is held.
const PANEL: [f32; 2] = [255.0, 210.0];
const PANEL_WIDE: f32 = 1.2;
const PANEL_COLOURS: (u32, u32) = (0x80dd_dddd, 0x6544_2d00);
const ARROWS: [[f32; 2]; 2] = [[339.0, 210.0], [168.0, 210.0]];
const ARROW_HELD: u32 = 0x80dd_dddd;

/// `0x3ff388(m, arrows)`: the panel, and with `arrows` the menu's (previous / next, when there is one).
fn draw_panel(arrows: Option<&Menu>, draws: &mut Vec<crate::hud::Draw>) {
    use crate::hud::Draw;
    for (id, colour) in [(0x49, PANEL_COLOURS.1), (0x48, PANEL_COLOURS.0)] {
        draws.push(Draw::Sprite { id, pos: PANEL, scale: [PANEL_WIDE, 1.0], frame: 0.0, colour });
    }
    let Some(m) = arrows else { return };
    let k = m.frame.rem_euclid(0x32);
    let t = if k < 0x19 { k as f32 * 0.04 } else { 1.0 - (k - 0x19) as f32 * 0.04 };
    let pulse = crate::hud::lerp_rgba(t, PANEL_COLOURS.0, PANEL_COLOURS.1);
    let lit = |button: u32| if m.held & button != 0 { ARROW_HELD } else { pulse };
    if m.prev {
        draws.push(Draw::Sprite { id: 0x4b, pos: ARROWS[0], scale: [PANEL_WIDE, 1.0], frame: 0.0, colour: lit(crate::pad::button::LEFT) });
    }
    if m.next {
        draws.push(Draw::Sprite { id: 0x4a, pos: ARROWS[1], scale: [PANEL_WIDE, 1.0], frame: 0.0, colour: lit(crate::pad::button::RIGHT) });
    }
}

/// `0x3ff808` (the menu's draw, all `0x90ffffff`, `$gp+0x2260..+0x2334`): the panel and its arrows ([`draw_panel`]); the
/// challenge's name in font 3 centred in the box 10..460 × 125..185 on (250, 150); its prize ("%s %d %s": 0x2dda, the
/// bolts, 0x2df4; font 2 at (90, 184)) and wins ("%s %d": 0x2dd4; font 2 at (275, 184)); in font 1 "previous" 0x2dd6 at
/// (166, 208), "next" 0x2dd7 at (305, 208), and the buttons' labels 0x2dd5 at (75, 95), 0x2dd9 at (210, 95), 0x2dd8 at
/// (380, 95) (the first layout's places; the other languages' are not ported). A prize of 10,000 or more prints the
/// label alone (the original's format there has no number for its `%d`).
pub fn draw_menu(m: &Menu, text: &dyn Fn(u32) -> Vec<u8>, fonts: [(&[rc_formats::disc::boot::Glyph; 256], usize); 3], draws: &mut Vec<crate::hud::Draw>, out: &mut Vec<crate::gui::Quad>) {
    use crate::gui::text_box::{CENTRE, MIDDLE};
    draw_panel(Some(m), draws);
    let colour = 0x90ff_ffffu32.to_le_bytes();
    let first = |id: u32| { let t = text(id); t.split(|&c| c == 0).next().unwrap_or(&[]).to_vec() };
    let [(g1, s1), (g2, s2), (g3, s3)] = fonts;
    let mut name = crate::gui::TextBox { top: 125, bottom: 185, left: 10, right: 460, x: 250, y: 150, line: 20, flags: CENTRE | MIDDLE, ..Default::default() };
    crate::gui::text_in_box(out, g3, s3, &mut name, &first(challenge_name(m.selected)), colour);
    if let Some(p) = m.prize {
        let t = if p < 10000 { [first(0x2dda), format!(" {p} ").into_bytes(), first(0x2df4)].concat() } else { first(0x2dda) };
        crate::gui::text(out, g2, s2, &t, 90.0, 184.0, 1.0, colour);
    }
    if m.wins != 0 {
        let t = [first(0x2dd4), format!(" {}", m.wins).into_bytes()].concat();
        crate::gui::text(out, g2, s2, &t, 275.0, 184.0, 1.0, colour);
    }
    if m.prev { crate::gui::text(out, g1, s1, &first(0x2dd6), 166.0, 208.0, 1.0, colour); }
    if m.next { crate::gui::text(out, g1, s1, &first(0x2dd7), 305.0, 208.0, 1.0, colour); }
    for (id, x) in [(0x2dd5, 75.0), (0x2dd9, 210.0), (0x2dd8, 380.0)] {
        crate::gui::text(out, g1, s1, &first(id), x, 95.0, 1.0, colour);
    }
}

/// `$gp+0x24d0..+0x2530` (the moat's draw `0x4014f0`): 50 streaks from 55 to 22 about the arena's middle, 4 under
/// it; the scroll a tick (its texture's s, `+0xe0`) and the turn (`+0xdc`, 0° a second here); the streak's colour
/// (alpha `randf(0, 1)` of 0x88) and its wide glow's, their half widths 1 and 2; the outer end's two glows
/// (`randf(3, 7)`: half of it in `0x807070` at alpha `0x50..0x60`, FX 0x31; 0.7 of it in `0x802020` at `8..0x16`, FX 0xb).
const MOAT_STREAKS: usize = 50;
const MOAT_RADII: (f32, f32) = (55.0, 22.0);
const MOAT_DROP: f32 = 4.0;
const MOAT_SCROLL: f32 = 0.15;
const MOAT_COLOURS: (u32, u32) = (0x88ff_7040, 0x10ff_0000);

/// `0x4014f0` (the master, first list; in a challenge, after one and in its lost state): the moat — the streaks of
/// energy across it (FX 0x17, scrolling, added) with a wider faint one each (FX 0x2b) and the glows at their outer
/// ends; the scroll moves on.
pub fn draw_moat(w: &mut MobyWorld, id: usize, middle: V3, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    use crate::hero::physics::cross;
    let m = &w.mobys[id];
    let step = std::f32::consts::TAU / MOAT_STREAKS as f32;
    let mut a = m.pv_f(0xdc);
    let sc = m.pv_f(0xe0);
    let z = middle[2] - MOAT_DROP;
    let st = [[-3.0 + sc, 0.0], [-3.0 + sc, 1.0], [3.0 + sc, 0.0], [3.0 + sc, 1.0]];
    for _ in 0..MOAT_STREAKS {
        let (c, s) = (a.cos(), a.sin());
        let outer = [middle[0] + MOAT_RADII.0 * c, middle[1] + MOAT_RADII.0 * s, z];
        let inner = [middle[0] + MOAT_RADII.1 * c, middle[1] + MOAT_RADII.1 * s, z];
        let d = sub(outer, inner);
        let ao = cross(sub(outer, cam), d);
        let ai = cross(sub(inner, cam), d);
        for (k, (half, fx, rgb)) in [(1.0, 0x17, MOAT_COLOURS.0), (2.0, 0x2b, MOAT_COLOURS.1)].into_iter().enumerate() {
            let (po, pi) = (set_len(ao, half), set_len(ai, half));
            let rgba = if k == 0 { (rgb & 0xff_ffff) | ((rng.randf(0.0, 1.0) * (rgb >> 24) as f32) as u32) << 24 } else { rgb };
            out.push(crate::fx::FxQuad { fx, additive: true, corners: [sub(outer, po), add(outer, po), sub(inner, pi), add(inner, pi)], st, rgba: [rgba; 4] });
        }
        let size = rng.randf(3.0, 7.0);
        let alpha = rng.rand_range(0x50, 0x60) as u32;
        let roll = rng.rand_angle();
        crate::fx::sprite(out, size * 0.5, 0.0, roll, outer, cam, 0x31, 0x0080_7070 | alpha << 24, true);
        let alpha = rng.rand_range(8, 0x16) as u32;
        let roll = rng.rand_angle();
        crate::fx::sprite(out, size * 0.7, 0.0, roll, outer, cam, 0xb, 0x0080_2020 | alpha << 24, true);
        a = add_rot(a, step);
    }
    let m = &mut w.mobys[id];
    let mut sc = m.pv_f(0xe0) + MOAT_SCROLL;
    if 1.0 < sc { sc -= 2.0; }
    if sc < -1.0 { sc += 2.0; }
    m.set_pv_f(0xe0, sc);
}

/// `0x401c20` (the master, second list, past its state 1): a glow on each lamp of its tie group `+0x104`, 3 below it
/// in its rows, when its face (−third row) is turned toward the camera by more than 60°: size 3 pushed 1 toward the
/// camera (`0x32bbe0`, FX 0xb), from `0xff8020` to `0x88ff8020` by the square of how far past.
pub fn draw_lamps(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let g = w.mobys[id].pv_i(0x104);
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.tie_groups.get(g)) else { return };
    for &i in list {
        let Some(t) = w.ties.get(i as usize) else { continue };
        let r = [[t[0][0], t[0][1], t[0][2]], [t[1][0], t[1][1], t[1][2]], [t[2][0], t[2][1], t[2][2]]];
        let p = add([t[3][0], t[3][1], t[3][2]], [-3.0 * r[2][0], -3.0 * r[2][1], -3.0 * r[2][2]]);
        let to_cam = set_len(sub(cam, p), 1.0);
        let face = [-r[2][0], -r[2][1], -r[2][2]];
        let f = crate::hero::physics::dot(to_cam, face);
        if f <= 0.5 { continue; }
        let k = (f - 0.5) * 2.0;
        let rgba = crate::hud::lerp_rgba(k * k, 0x00ff_8020, 0x88ff_8020);
        crate::fx::sprite(out, 3.0, 1.0, 0.0, p, cam, 0xb, rgba, true);
    }
}

/// The arena's scene props.
pub const PROPS: i16 = 0x0f15;
/// The port's: what the props' first-list callback draws this tick (`0x43a3c8`'s registrations, [`draw_props`]): 1
/// with its own screens `0x43a0d8` (the scene's record 0), 0 the master's moat and lamps alone (record 1).
const PROPS_SCREENS: usize = 0x100;
/// `0x43a0d8`: its two screens — the scene ticks (`0x1be234`, exclusive) each is up in, the place its tie is nearest,
/// its FX texture.
const SCREENS: [(i32, i32, V3, usize); 2] = [(0x3b5, 0x547, [495.0, 587.0, 513.0], 0x3f), (0x4af, 0x5dd, [515.0, 596.0, 512.0], 0x40)];
/// `0x1b2680`: a screen's corners in its tie's frame (GS strip order).
const SCREEN_CORNERS: [[u32; 3]; 4] = [
    [0x4056_6666, 0xc031_eb85, 0x4017_5c29],
    [0x4056_6666, 0xc031_eb85, 0xc017_5c29],
    [0x4056_6666, 0x4033_9581, 0x4017_5c29],
    [0x4056_6666, 0x4033_9581, 0xc017_5c29],
];

/// 0x0f15 (`0x43a3c8`), the arena's scene props: pvars `+0` a tie group (the big screens), `+4` a moby, `+8` that
/// moby moved, `+0xc` the master (both runtime moby indices). **0** → 1, update distance 0xff. **1** in a scene (game
/// mode 2) by its record in the level (`0x1be230`, `0x371778`, [`rc_formats::disc::scene::record_of`]): record 1 the
/// master's moat (`0x4014f0`) and lamps (`0x401c20`) on the first list; record 0 those and its own screens
/// (`0x43a0d8`, [`draw_props`]), and once the moby `+4` at sequence 1 frame 7 (unless it plays 1), stopped, and 2
/// back along its second row. Out of a scene, a moved moby goes back: sequence 0 frame 0 (unless it plays 0), stopped,
/// 2 forward.
///
/// Not ported: record 6 (the master's score `+0x6c` 100 and its mode |= 2, the colours `0x258a50..58` and the scene's
/// fourth actor's `+0x78` = 0x40ff00ea, its `+0xb8` the fifth, and that actor drawn as the boss 0x09f8's parts
/// `0x43a360` → `0x3dc3c8` / `0x3dc6c0` / `0x3dc4f8` / `0x3dc598`): Maktar's scenes are ids 0x2b..0x30, records 0..5,
/// so the branch never runs here (the code is shared with another arena's level).
pub fn update_props(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state = 1;
            m.update_dist = 0xff;
            m.set_pv_i(8, 0);
        }
        1 => {
            let moved = w.mobys[id].pv_i(8) != 0;
            let (seq, step) = if ctx.game.scenes.mode == crate::scene::SCENE {
                let record = ctx.game.scenes.player.as_ref().map(|p| rc_formats::disc::scene::record_of(p.id));
                if !matches!(record, Some(0 | 1)) { return; }
                let own = record == Some(0);
                w.mobys[id].set_pv_i(PROPS_SCREENS, own as i32);
                w.register_draw_first(id);
                if !own || moved { return; }
                w.mobys[id].set_pv_i(8, 1);
                ((1, 7), -2.0)
            } else {
                if !moved { return; }
                w.mobys[id].set_pv_i(8, 0);
                ((0, 0), 2.0)
            };
            let Some(p) = placed(w, w.mobys[id].pv_i(4)) else { return };
            if super::props::seq(w, p, ctx) != seq.0 {
                ctx.host.set_anim(&w.mobys[p], seq.0, seq.1, 0);
                ctx.host.set_anim_speed(&w.mobys[p], 0.0);
            }
            let m = &mut w.mobys[p];
            m.pos = add(m.pos, set_len(m.rows[1], step));
        }
        _ => {}
    }
}

/// The props' first-list callbacks (`0x43a3c8` registers them in this order): the master's moat ([`draw_moat`]) and
/// lamps ([`draw_lamps`]), then in the scene's record 0 `0x43a0d8`: each screen within its ticks of the scene
/// (`tick`, `0x1be234`) on the tie of the group `+0` nearest its place (the first nearest), its corners
/// ([`SCREEN_CORNERS`]) on that tie's rows at its place, FX 0x3f / 0x40 at 0x808080 (`0x2f1eb0`: blend 2, the
/// texture as it is).
pub fn draw_props(w: &mut MobyWorld, id: usize, rng: &mut crate::rng::Rng, tick: Option<i32>, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    if let Some(master) = placed(w, w.mobys[id].pv_i(0xc)) {
        let mid = w.mobys[master].pv_v(MIDDLE_AT);
        draw_moat(w, master, mid, rng, cam, out);
        draw_lamps(w, master, cam, out);
    }
    let m = &w.mobys[id];
    if m.pv_i(PROPS_SCREENS) == 0 { return; }
    let (Some(tick), Some(list)) = (tick, usize::try_from(m.pv_i(0)).ok().and_then(|g| w.tie_groups.get(g))) else { return };
    for (lo, hi, at, fx) in SCREENS {
        if !(lo < tick && tick < hi) { continue; }
        let mut best = (None, 99999.0);
        for &i in list {
            let Some(t) = w.ties.get(i as usize) else { break };
            let d = dist([t[3][0], t[3][1], t[3][2]], at);
            if d < best.1 { best = (Some(t), d); }
        }
        let Some(t) = best.0 else { continue };
        let corners = SCREEN_CORNERS.map(|c| {
            let c = c.map(f32::from_bits);
            std::array::from_fn(|k| c[0] * t[0][k] + c[1] * t[1][k] + c[2] * t[2][k] + t[3][k])
        });
        out.push(crate::fx::FxQuad { fx, additive: false, corners, st: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]], rgba: [0x8080_8080; 4] });
    }
}
