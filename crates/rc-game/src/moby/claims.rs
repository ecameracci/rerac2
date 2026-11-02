//! The targets' claims (`0x2312c0`: 32 records of 0x10 bytes, `+0x00` the target, `+0x04` its owner (0: free),
//! `+0x08` the ticks the claim lasts): the homing shots spread over the targets a search found instead of all taking the
//! best. A shot's pick (`0x37f338`) takes the first candidate its owner has not claimed yet and claims it
//! (`0x37f158`); with every candidate claimed (and more than one), the one whose claim runs out first, unclaimed again.
//! The owner's hand ages its claims every tick (`0x37f440`).
//!
//! The owner is Ratchet's moby (`0x18c0b0`) for the Seeker Gun (crate::moby::classes::seeker: the hand `0x3a5360`
//! ages them, the missile `0x3abb40` picks).
//!
//! Not ported: the class-type filter of the pick (`0x37f338`'s last argument, −1 at its callers here), the owner's
//! release of all its claims (`0x37f500`: no caller on the ported levels' paths).

use super::{countdown, mode, MobyWorld};

/// The records.
pub const CLAIMS: usize = 0x20;

/// One record: the target and its owner, the ticks left.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Claim {
    pub target: usize,
    pub owner: usize,
    pub ticks: i32,
}

/// The table (`None`: a free record, owner 0).
#[derive(Clone, Debug, Default)]
pub struct Claims(pub [Option<Claim>; CLAIMS]);

impl Claims {
    /// `0x37f158(owner, target, ticks)`: a claim in the first free record (none free: none).
    pub fn claim(&mut self, owner: usize, target: usize, ticks: i32) {
        if let Some(r) = self.0.iter_mut().find(|r| r.is_none()) { *r = Some(Claim { target, owner, ticks }); }
    }

    /// `0x37f1c0`: `owner` has no claim on `target`.
    pub fn free(&self, owner: usize, target: usize) -> bool { !self.0.iter().flatten().any(|c| c.owner == owner && c.target == target) }

    /// `0x37f228`: the ticks left of `owner`'s claim on `target` (−1: none).
    fn ticks(&self, owner: usize, target: usize) -> i32 { self.0.iter().flatten().find(|c| c.owner == owner && c.target == target).map_or(-1, |c| c.ticks) }

    /// `0x37f338(owner, list, n, &fresh, −1)`: the first candidate `owner` has not claimed (fresh), else with more than
    /// one candidate the claimed one whose claim has the fewest ticks left (`0x37f270`, below 9999; not fresh).
    pub fn pick(&self, owner: usize, list: &[usize]) -> Option<(usize, bool)> {
        if let Some(&t) = list.iter().find(|&&t| self.free(owner, t)) { return Some((t, true)); }
        if list.len() < 2 { return None; }
        let mut best: Option<(i32, usize)> = None;
        for &t in list {
            let k = self.ticks(owner, t);
            if k != -1 && k < best.map_or(9999, |b| b.0) { best = Some((k, t)); }
        }
        best.map(|(_, t)| (t, false))
    }

    /// `0x37f440(owner)`: `owner`'s claims age — a target gone, dead (state −2 / −3) or out of the target list (mode
    /// 0x1000) frees its record; else its ticks count down and free it when out.
    pub fn age(&mut self, w: &MobyWorld, owner: usize) {
        for r in self.0.iter_mut() {
            let Some(c) = r.as_mut().filter(|c| c.owner == owner) else { continue };
            let live = w.mobys.get(c.target).is_some_and(|m| m.mode & mode::TARGET != 0 && m.state != 0xfe && m.state != 0xfd);
            if !live || countdown(&mut c.ticks) { *r = None; }
        }
    }
}
