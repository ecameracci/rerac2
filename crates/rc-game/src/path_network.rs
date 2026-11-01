//! Path networks (L02): a walker's map of the level's floor polygons — closed paths (`0x1ff1c0`, crate::level's
//! paths) that share edges — and the walk through them toward a goal. On the levels the port plays only Maktar's
//! inspection robot 0x09bc (crate::moby::classes::maktar_pads) has one; the code is in Maktar's overlay alone (the
//! goal clamp `0x3302b8` also on level 15).
//!
//! **The network** (`0x32f8b0(m, start, out, max)`, [`build`]): a breadth-first walk over the level's paths from the
//! path `start`, up to 32 rings: a ring's paths, in path order, each take the next record while there are fewer than
//! `max` (a path past `max` is not recorded and its neighbours are not looked for), and mark every unmarked path that
//! meets one of them ([`meeting`]) and is a closed polygon of at most 64 points (its last point within 0.2 of its
//! first). Then every two recorded paths that meet are linked both ways (up to four links each, the meeting edge's
//! point index beside each link) and **their meeting points' `w` set to 0** in the level's path data — the opening
//! between them is no wall for the fences (crate::moby::ai::fence) and the wall crossings ([`first_crossing`]) — and
//! each record gets its points' mean (`/ count`, the closing point counted) and the farthest point's distance across
//! from it. A record, 0x20 bytes in the walker's pvars: `+0x00` the path, `+0x04..+0x08` the four linked records
//! (0xff none), `+0x08..+0x0c` the meeting edge (point index) in this path toward each, `+0x0c` flags (1: closed to
//! walking; set by the pvar data, never by the code), `+0x10` the centre, `+0x1c` the radius.
//!
//! **Queries**:
//! * [`meeting`] (`0x34af88(a, b, &ea, &eb)`): two paths share an edge — a point of `a` within 0.5 across of a point
//!   of `b`, and the next or previous point of `a` within 0.5 of the next or previous of `b` (in that order of tests;
//!   the previous of a path's first point is its last but one).
//! * [`locate`] (`0x330180(p, nodes, n, &cur)`): the record whose polygon holds `p` across (`0x321168`, the even-odd
//!   ray test, crate::level::LevelData::in_polygon): the current one first, then the others whose circle (centre,
//!   radius across) holds `p`; none: −1.
//! * [`steer`] (`0x32fd40(offset, r, from, to, &yaw, nodes, n, &at, &goal)`): the heading from `from` toward `to`
//!   through the network. The straight heading, unless both lie in records (`at` / `goal` kept between calls) and the
//!   goal's is open (flag 1 clear): then, from different records, a breadth-first search from the goal's record (rings
//!   numbered from 1, up to 50, closed records skipped) stops at the first record of a ring that links to `at`'s; the
//!   heading is clamped into the opening of its edge toward `at`'s record ([`ease_through`]), after (when that record
//!   is two rings or more from the goal's) the opening of its edge toward the next ring in. Returns whether both ends
//!   are in the network and the goal's record is reachable (or the same).
//! * [`ease_through`] (`0x330040(yaw, r, a, b, from)`): an edge `a → b` narrowed by `r` at both ends; the two yaws from
//!   `from` to its ends, and `yaw` clamped to within half the angle between them of their bisector.
//! * [`keep_in`] (`0x3302b8(r, from, to, &out, path)`): a point toward `to` kept in a path: stopped short (0.99) of
//!   the first wall crossed (`0x34aa08`, [`first_crossing`]), then kept `r` from the walls (`0x343678`); its height
//!   `to`'s.
//!
//! Not ported: [`steer`]'s yaw offset (its one caller passes 0). The level's path data stays as loaded (shared, read
//! only): the meeting points' `w` cleared by the build are applied to the walker's own copies of its records' paths
//! ([`opened`]) — any other moby using those paths as a fence or wall would see them closed (none does on Maktar).

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, len2, scale, set_len, sub, sub_rot, V3};
use crate::level::LevelData;

/// A record's size in the walker's pvars.
pub const NODE: usize = 0x20;

/// A network record (module docs).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Node {
    pub path: i32,
    pub links: [u8; 4],
    pub edges: [u8; 4],
    pub flags: u16,
    pub centre: V3,
    pub radius: f32,
}

impl Node {
    /// The record at `b[..0x20]` (missing bytes: 0).
    pub fn read(b: &[u8]) -> Node {
        let byte = |o: usize| b.get(o).copied().unwrap_or(0);
        let word = |o: usize| u32::from_le_bytes([byte(o), byte(o + 1), byte(o + 2), byte(o + 3)]);
        Node {
            path: word(0) as i32,
            links: [byte(4), byte(5), byte(6), byte(7)],
            edges: [byte(8), byte(9), byte(10), byte(11)],
            flags: u16::from_le_bytes([byte(0xc), byte(0xd)]),
            centre: [f32::from_bits(word(0x10)), f32::from_bits(word(0x14)), f32::from_bits(word(0x18))],
            radius: f32::from_bits(word(0x1c)),
        }
    }

    /// Written back over `b[..0x20]` (bytes `+0x0e`, `+0x1c`'s neighbours untouched).
    pub fn write(&self, b: &mut [u8]) {
        let mut put = |o: usize, v: &[u8]| {
            if let Some(d) = b.get_mut(o..o + v.len()) { d.copy_from_slice(v); }
        };
        put(0, &self.path.to_le_bytes());
        put(4, &self.links);
        put(8, &self.edges);
        put(0xc, &self.flags.to_le_bytes());
        for k in 0..3 { put(0x10 + 4 * k, &self.centre[k].to_le_bytes()); }
        put(0x1c, &self.radius.to_le_bytes());
    }

    /// Flag 1: closed to walking.
    pub fn closed(&self) -> bool { self.flags & 1 != 0 }
}

/// The `n` records from pvar offset `at`.
pub fn read_nodes(pvars: &[u8], at: usize, n: usize) -> Vec<Node> {
    (0..n).map(|k| Node::read(pvars.get(at + NODE * k..).unwrap_or(&[]))).collect()
}

/// The records written back from pvar offset `at`.
pub fn write_nodes(pvars: &mut [u8], at: usize, nodes: &[Node]) {
    for (k, n) in nodes.iter().enumerate() {
        if let Some(b) = pvars.get_mut(at + NODE * k..) { n.write(b); }
    }
}

fn p3(p: &[f32; 4]) -> V3 { [p[0], p[1], p[2]] }

fn points(level: &LevelData, path: i32) -> &[[f32; 4]] {
    usize::try_from(path).ok().and_then(|i| level.paths.get(i)).map_or(&[], |p| p.as_slice())
}

/// A polygon the build takes: at most 64 points, the last within 0.2 of the first.
fn polygon(pts: &[[f32; 4]]) -> bool {
    match (pts.first(), pts.last()) {
        (Some(a), Some(b)) => pts.len() < 0x41 && dist(p3(a), p3(b)) <= 0.2,
        _ => false,
    }
}

/// `0x34af88(a, b, &ea, &eb)`: the edges (start point indices) where paths `a` and `b` meet (module docs).
pub fn meeting(level: &LevelData, a: i32, b: i32) -> Option<(usize, usize)> {
    let (pa, pb) = (points(level, a), points(level, b));
    let (na, nb) = (pa.len(), pb.len());
    if na < 2 { return None; }
    let near = |i: usize, j: usize| dist2(p3(&pa[i]), p3(&pb[j])) < 0.5;
    for i in 0..na - 1 {
        for j in 0..nb.saturating_sub(1) {
            if !near(i, j) { continue; }
            let ip = if i == 0 { na - 2 } else { i - 1 };
            let jp = if j == 0 { nb - 2 } else { j - 1 };
            if near(i + 1, j + 1) { return Some((i, j)); }
            if near(ip, j + 1) { return Some((ip, j)); }
            if near(i + 1, jp) { return Some((i, jp)); }
            if near(ip, jp) { return Some((ip, jp)); }
        }
    }
    None
}

/// `0x32f8b0(m, start, out, max)`: the network from path `start` into `nodes` (its length the most; the records'
/// flags and unfilled edge bytes are kept); returns the count (module docs).
pub fn build(level: &LevelData, start: i32, nodes: &mut [Node]) -> usize {
    let n_paths = level.paths.len();
    let max = nodes.len();
    let mut marks = vec![0u8; n_paths.max(1)];
    if let Some(m) = usize::try_from(start).ok().and_then(|s| marks.get_mut(s)) { *m = 1; }
    let mut count = 0;
    let mut ring = 1u32;
    loop {
        let mut more = false;
        let next = ring + 1;
        for i in 0..n_paths {
            if marks[i] as u32 != ring || max <= count { continue; }
            nodes[count].path = i as i32;
            count += 1;
            for j in 0..n_paths {
                if i == j || marks[j] != 0 { continue; }
                if polygon(&level.paths[j]) && meeting(level, i as i32, j as i32).is_some() {
                    marks[j] = next as u8;
                    more = true;
                }
            }
        }
        ring = next;
        if !(more && ring < 0x20) { break; }
    }
    for n in nodes[..count].iter_mut() { n.links = [0xff; 4]; }
    for i in 0..count {
        for j in i + 1..count {
            let Some((ei, ej)) = meeting(level, nodes[i].path, nodes[j].path) else { continue };
            if let Some(s) = (0..4).find(|&s| nodes[i].links[s] == 0xff) {
                nodes[i].links[s] = j as u8;
                nodes[i].edges[s] = ei as u8;
            }
            if let Some(s) = (0..4).find(|&s| nodes[j].links[s] == 0xff) {
                nodes[j].links[s] = i as u8;
                nodes[j].edges[s] = ej as u8;
            }
        }
    }
    for n in nodes[..count].iter_mut() {
        let pts = points(level, n.path);
        let k = pts.len().max(1) as f32;
        let sum = pts.iter().fold([0.0f32; 3], |s, p| add(s, p3(p)));
        let c = scale(sum, 1.0 / k);
        n.centre = c;
        n.radius = pts.iter().fold(0.0f32, |r, p| r.max(dist2(c, p3(p))));
    }
    count
}

/// Record `k`'s path as the build leaves it: the points where it meets the network's other paths with `w` 0.
pub fn opened(level: &LevelData, nodes: &[Node], k: usize) -> Vec<[f32; 4]> {
    let Some(me) = nodes.get(k) else { return Vec::new() };
    let mut pts = points(level, me.path).to_vec();
    for (j, other) in nodes.iter().enumerate() {
        if j == k { continue; }
        let e = if k < j { meeting(level, me.path, other.path).map(|e| e.0) } else { meeting(level, other.path, me.path).map(|e| e.1) };
        if let Some(p) = e.and_then(|e| pts.get_mut(e)) { p[3] = 0.0; }
    }
    pts
}

/// `0x330180(p, nodes, n, &cur)`: the record whose polygon holds `p` (module docs).
pub fn locate(level: &LevelData, nodes: &[Node], p: V3, cur: &mut i32) -> bool {
    if let Some(n) = usize::try_from(*cur).ok().and_then(|c| nodes.get(c)) {
        if level.in_polygon(n.path, p) { return true; }
    }
    for (i, n) in nodes.iter().enumerate() {
        if i as i32 != *cur && dist2(p, n.centre) <= n.radius && level.in_polygon(n.path, p) {
            *cur = i as i32;
            return true;
        }
    }
    *cur = -1;
    false
}

/// `0x330040(yaw, r, a, b, from)`: `yaw` clamped into the opening `a → b` seen from `from` (module docs).
pub fn ease_through(yaw: f32, r: f32, a: V3, b: V3, from: V3) -> f32 {
    let d = set_len(sub(b, a), r);
    let (p, q) = (add(a, d), sub(b, d));
    let y1 = atan2(p[0] - from[0], p[1] - from[1]);
    let y2 = atan2(q[0] - from[0], q[1] - from[1]);
    let half = sub_rot(y2, y1) * 0.5;
    let mid = add_rot(half, y1);
    let lim = half.abs();
    let dev = sub_rot(yaw, mid).min(lim).max(-lim);
    add_rot(dev, mid)
}

/// The edge from point `e` of record `n`'s path to the next.
fn edge(level: &LevelData, n: &Node, e: u8) -> Option<(V3, V3)> {
    let pts = points(level, n.path);
    let e = e as usize;
    Some((p3(pts.get(e)?), p3(pts.get(e + 1)?)))
}

/// `0x32fd40(0, r, from, to, &yaw, nodes, n, &at, &goal)`: the heading through the network (module docs).
pub fn steer(level: &LevelData, nodes: &[Node], r: f32, from: V3, to: V3, yaw: &mut f32, at: &mut i32, goal: &mut i32) -> bool {
    *yaw = atan2(to[0] - from[0], to[1] - from[1]);
    if !locate(level, nodes, from, at) || !locate(level, nodes, to, goal) { return false; }
    let t = *goal as usize;
    if nodes[t].closed() { return false; }
    if *at == *goal { return true; }
    let mut marks = vec![0u8; nodes.len()];
    marks[t] = 1;
    let mark = |m: &[u8], i: u8| m.get(i as usize).copied().unwrap_or(0);
    let (mut found, mut via) = (None, 0u8);
    let mut ring = 1u8;
    while found.is_none() {
        for k in 0..nodes.len() {
            if marks[k] == ring {
                for s in 0..4 {
                    let nb = nodes[k].links[s];
                    if nb & 0x80 != 0 { continue; }
                    if mark(&marks, nb) == 0 && nodes.get(nb as usize).is_some_and(|n| !n.closed()) {
                        marks[nb as usize] = ring + 1;
                        if nb as i32 == *at {
                            via = nodes[k].edges[s];
                            found = Some(k);
                        }
                    }
                }
            }
            if found.is_some() { break; }
        }
        ring += 1;
        if found.is_none() && 0x32 <= ring { return false; }
    }
    let k = found.unwrap_or(0);
    let n = &nodes[k];
    let mut y = *yaw;
    if 3 <= marks[k] {
        if let Some(s) = (0..4).find(|&s| mark(&marks, n.links[s]) as i32 == marks[k] as i32 - 1) {
            if let Some((a, b)) = edge(level, n, n.edges[s]) { y = ease_through(y, r, a, b, from); }
        }
    }
    if let Some((a, b)) = edge(level, n, via) { y = ease_through(y, r, a, b, from); }
    *yaw = y;
    true
}

/// `0x34aa08(path, from, to, &out)`: the first crossing of the path's walls on the way `from → to` (in the frame along
/// it; a segment is a wall unless both its points' `w` are 0), the way's fraction there.
pub fn first_crossing(path: &[[f32; 4]], from: V3, to: V3) -> Option<f32> {
    let d = sub(to, from);
    let l = len2(d);
    if l == 0.0 { return None; }
    let (ax, ay) = (d[0] / l, d[1] / l);
    let local = |p: [f32; 4]| { let q = sub([p[0], p[1], p[2]], from); [q[0] * ax + q[1] * ay, -q[0] * ay + q[1] * ax, p[3]] };
    let mut best = None;
    let mut f = 1.0;
    let mut prev = local(*path.first()?);
    for p in &path[1..] {
        let cur = local(*p);
        if (prev[2] != 0.0 || cur[2] != 0.0) && cur[1] * prev[1] < 0.0 {
            let x = ((prev[0] - cur[0]) / (prev[1] - cur[1])) * -cur[1] + cur[0];
            if 0.0 < x && x < f {
                f = x;
                best = Some(f);
            }
        }
        prev = cur;
    }
    best
}

/// `0x3302b8(r, from, to, &out, path)`: a point toward `to` kept inside the path's points (module docs).
pub fn keep_in(pts: &[[f32; 4]], r: f32, from: V3, to: V3) -> V3 {
    let p = match first_crossing(pts, from, to) {
        Some(f) => {
            let hit = add(from, scale(sub(to, from), f));
            add(from, scale(sub(hit, from), f32::from_bits(0x3f7d_70a4)))
        }
        None => to,
    };
    let mut out = crate::moby::ai::fence(pts, r, p).unwrap_or(p);
    out[2] = to[2];
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Maktar's robot's network from the level's data: built from its pvars' start path, every record a closed
    /// polygon linked both ways, and the walk from each record's centre to every other's finds a way. Writes a dump to
    /// `work/scratch/path_network_l02.txt` (skipped without the data folder).
    #[test]
    fn maktar_robot_network() {
        use rc_formats::disc::gameplay as g;
        use rc_formats::disc::level::{LevelWad, Part};
        use std::fmt::Write as _;
        let Ok(file) = std::fs::read(rc_formats::test_data::root().join("G/LEVEL2.WAD")) else { return };
        let wad = LevelWad::parse(&file).unwrap();
        let gp = rc_formats::wad::decompress(wad.part_bytes(Part::Gameplay).unwrap().unwrap()).unwrap();
        let level = LevelData::load(&gp);
        let mobys = g::moby_instances(&gp).unwrap();
        let blocks = g::pvar_blocks(&gp).unwrap();
        let fixups = g::pvar_fixups(&gp, "pvar_pointer_fixups").unwrap();
        let groups = g::moby_groups(&gp).unwrap();
        let (k, robot) = mobys.iter().enumerate().find(|(_, m)| m.base.o_class == 0x09bc).expect("Maktar's robot");
        let pi = robot.base.pvar_index as usize;
        let pv = &blocks[pi];
        let word = |o: usize| i32::from_le_bytes([pv[o], pv[o + 1], pv[o + 2], pv[o + 3]]);
        let mut out = String::new();
        writeln!(out, "robot: instance {k} at {:?} mode {:#x} pvar index {pi} size {:#x}", robot.base.position, robot.base.mode(), pv.len()).unwrap();
        writeln!(out, "pointer fixups: {:x?}", fixups.iter().filter(|f| f.0 == pi).map(|f| f.1).collect::<Vec<_>>()).unwrap();
        for o in (0..pv.len().min(0x3d0)).step_by(0x10) {
            writeln!(out, "+{o:03x}: {:08x} {:08x} {:08x} {:08x}", word(o), word(o + 4), word(o + 8), word(o + 12)).unwrap();
        }
        let start = word(0x390);
        writeln!(out, "start path {start}, fence {}, cuboid {}, group {} ({:?}), idle {}, near {}, at {} goal {}", word(0x394), word(0x398), word(0x39c), groups.get(word(0x39c) as usize), word(0x3c0), word(0x3c4), word(0x3b8), word(0x3bc)).unwrap();
        let mut nodes = read_nodes(pv, 0x290, 8);
        let n = build(&level, start, &mut nodes);
        let nodes = &nodes[..n];
        for (i, nd) in nodes.iter().enumerate() {
            let pts = points(&level, nd.path);
            writeln!(out, "node {i}: path {} ({} points, polygon {}) links {:?} edges {:?} flags {:#x} centre {:?} radius {}", nd.path, pts.len(), polygon(pts), nd.links, nd.edges, nd.flags, nd.centre, nd.radius).unwrap();
            writeln!(out, "  w: {:?}", pts.iter().map(|p| p[3]).collect::<Vec<_>>()).unwrap();
            writeln!(out, "  opened w: {:?}", opened(&level, nodes, i).iter().map(|p| p[3]).collect::<Vec<_>>()).unwrap();
        }
        writeln!(out, "fence in network: {}", nodes.iter().any(|nd| nd.path == word(0x394))).unwrap();
        let mut routes = 0;
        for (i, a) in nodes.iter().enumerate() {
            for (j, b) in nodes.iter().enumerate() {
                if i == j || !level.in_polygon(a.path, a.centre) || !level.in_polygon(b.path, b.centre) { continue; }
                let (mut yaw, mut at, mut goal) = (0.0, -1, -1);
                let ok = steer(&level, nodes, 0.825, a.centre, b.centre, &mut yaw, &mut at, &mut goal);
                writeln!(out, "route {i} -> {j}: {ok} yaw {yaw:.3} (straight {:.3}) at {at} goal {goal}", atan2(b.centre[0] - a.centre[0], b.centre[1] - a.centre[1])).unwrap();
                assert!(ok || b.closed(), "no route {i} -> {j}");
                routes += 1;
            }
        }
        let _ = std::fs::write(concat!(env!("CARGO_MANIFEST_DIR"), "/../../work/scratch/path_network_l02.txt"), &out);
        assert!(n >= 1, "{out}");
        for (i, a) in nodes.iter().enumerate() {
            assert!(i == 0 || polygon(points(&level, a.path)), "node {i} not a polygon");
            for (s, &l) in a.links.iter().enumerate() {
                if l == 0xff { continue; }
                assert!(nodes[l as usize].links.contains(&(i as u8)), "link {i}.{s} -> {l} one way");
            }
        }
        eprintln!("{routes} routes");
    }
}
