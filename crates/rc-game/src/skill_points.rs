//! The skill points: one byte each at `0x1a7a68` (the save's global chunk 8, 0x20 bytes, boot table `0x2560b8`;
//! [`GameState::skill_points`]), 30 of them used (the debug code entry `0x358550` gives its codes 0x50..0x6d as
//! points 0..0x1d).
//!
//! **The award** has no function of its own: every site writes the same four steps inline,
//! `if !0x1a7a68[k] { 0x1a7a68[k] = 1; PlayLevelSound(1, 0, 0); ShowMessage(0x1233, −1); 0x1a903c = 1 }` (L00
//! `0x360750` / `0x327448`, L01 `0x356ad8` / `0x31d0e0`, L02 `0x367f58` / `0x32c310`, L03 `0x361f28` / `0x327be8`):
//! the byte, the level's sound 1 at the listener, the "Skill Point" message in the first message box (crate::msgbox),
//! and the reward flag ([`GameState::award_skill_point`]).
//!
//! **The count** (`0x327df0`): the bytes set among the 0x20, at most 30 ([`GameState::skill_point_count`]); the
//! Special menu's Cheats page unlocks its cheats by it (`0x349d80` / `0x349fc0`, cheat rows `0x1ac090`: the count each
//! needs, 0x5a = point 0) and the reward hint reads it (`0x2b22d0`).
//!
//! **Sites** (the point, its byte, where): the port awards
//! * 0x01 (`0x1a7a69`): Maktar's arena boss 0x09f8 beaten with nothing but the wrench (L02 `0x3d4d6c`,
//!   crate::moby::classes::bosses);
//! * 0x0d (`0x1a7a75`): Oozla's path riders 0x061c knocked off four times (L01 `0x3a7260`, crate::moby::classes::oozla);
//! * 0x0e (`0x1a7a76`): Maktar's resort down to 10 breakables (0x12cf, L02 `0x38b5e8`, crate::moby::classes::maktar);
//! * 0x0f (`0x1a7a77`): Oozla's store's breakables all broken (0x12dc, L01 `0x379e6c`, crate::moby::classes::smash);
//! * 0x13 (`0x1a7a7b`): the arena boss 0x0a44 beaten without Ratchet hurt (L02 `0x3da250`);
//! * 0x19 (`0x1a7a81`): a slot machine's jackpot (0x0a9c, L02 `0x3e8f04`, crate::moby::classes::maktar);
//! * 0x1a (`0x1a7a82`): every weapon at its top level (`0x2d6258`'s tail every 128th frame, L01 `0x2cb9c8`, L02
//!   `0x2d97a8`, L03 `0x2d6e28`: crate::upgrades, GameState::all_upgraded);
//! * 0x1c (`0x1a7a84`): Endako's sentry 0x0ffd's death (`0x4201b8`, crate::moby::classes::endako_c::crane_kill) —
//!   the remote crane that makes the state 0x31 it counts in is not ported yet, so it does not come.
//!
//! Not ported (no class or system for them yet): the play frame's checks (`0x2a67f8`, every level's copy: L01
//! `0x2a8770`, L02 `0x2a9b78`, L03 `0x2ad068`): 0x1d (`0x1a7a85`) once the ship's upgrades `0x1a7af8 & 0xbb50` are
//! all had, 0x17 (`0x1a7a7f`) at the health limit 80; item
//! 0x10's weapon (two hand classes, in every overlay: L00 `0x3c76e0` / `0x406cf8`, L01 `0x3c6f40` / `0x4085f0`, L02
//! `0x3e0a18` / `0x432288`, L03 `0x3d74a0` / `0x412de8`): 0x16 (`0x1a7a7e`) its 16th class-0xfb4 target on level 0xc
//! (`$gp+0x1de0`); level 0x10's count (L02 `0x38b3c0`, L03 `0x385200`, no class placed on these levels): 0xb
//! (`0x1a7a73`) past 13; level 3's 0x12cf count (L03 `0x385298`): 0x15 (`0x1a7a7d`); 0x1c from Endako's 0x0ff3
//! (`0x41d510` → `0x41d838`, `0x41e138` → `0x41e480`: its update does not call the count yet); the debug code entry (`0x358550`). The reward flag `0x1a903c` (L01 `0x1a907c`) and its hint
//! (`0x2b22d0`: the vendor and armour hints 0x1b90 / 0x1b91 / 0x1b93 after a reward, by the count and the planet, 300
//! ticks apart). The pause's Skill Points and Cheats pages read the bytes and the count: crate::menus::special.

use crate::game::GameState;

/// `0x1a7a68`: the skill points' bytes (chunk 8).
pub const LEN: usize = 0x20;
/// The points the game uses (`0x327df0`'s cap).
pub const USED: i32 = 0x1e;
/// The message the award shows ("Skill Point").
pub const MESSAGE: i32 = 0x1233;
/// The level sound the award plays.
pub const SOUND: i32 = 1;

/// The points the port awards (index into `0x1a7a68`).
pub mod point {
    /// The arena boss 0x09f8 beaten with the wrench alone.
    pub const BOSS_9F8: usize = 0x01;
    /// Oozla's path riders knocked off four times.
    pub const RIDERS: usize = 0x0d;
    /// Maktar's resort's breakables down to 10.
    pub const RESORT: usize = 0x0e;
    /// Oozla's store's breakables all broken.
    pub const STORE: usize = 0x0f;
    /// The arena boss 0x0a44 beaten without Ratchet hurt.
    pub const BOSS_A44: usize = 0x13;
    /// A slot machine's jackpot.
    pub const JACKPOT: usize = 0x19;
    /// Ten Endako enemies killed while Ratchet works the remote crane (state 0x31).
    pub const CRANE: usize = 0x1c;
}

impl GameState {
    /// Skill point `k` earned (`0x1a7a68[k]`).
    pub fn has_skill_point(&self, k: usize) -> bool { self.skill_points.get(k).is_some_and(|&b| b != 0) }

    /// The award (module docs): unless had, the byte, the level's sound 1 (at the listener, `PlayLevelSound(1, 0, 0)`)
    /// and the message 0x1233 (`ShowMessage(0x1233, −1)`); whether it was new.
    pub fn award_skill_point(&mut self, k: usize) -> bool {
        let Some(b) = self.skill_points.get_mut(k) else { return false };
        if *b != 0 { return false; }
        *b = 1;
        self.level_sounds.push(SOUND);
        self.messages.push(crate::msgbox::Request::Text { text: MESSAGE, ticks: None });
        true
    }

    /// `0x327df0`: the points earned, at most 30.
    pub fn skill_point_count(&self) -> i32 { count(&self.skill_points) }
}

/// `0x327df0` over the bytes `0x1a7a68..`: those set, at most 30.
pub fn count(bytes: &[u8]) -> i32 { (bytes.iter().take(LEN).filter(|&&b| b != 0).count() as i32).clamp(0, USED) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn award_once_with_sound_and_message() {
        let mut g = GameState::default();
        assert!(!g.has_skill_point(point::BOSS_9F8));
        assert!(g.award_skill_point(point::BOSS_9F8));
        assert!(g.has_skill_point(point::BOSS_9F8));
        assert_eq!(g.level_sounds, vec![SOUND]);
        assert_eq!(g.messages, vec![crate::msgbox::Request::Text { text: MESSAGE, ticks: None }]);
        assert!(!g.award_skill_point(point::BOSS_9F8), "a point is awarded once");
        assert_eq!(g.level_sounds.len(), 1);
        assert_eq!(g.messages.len(), 1);
        assert!(!g.award_skill_point(LEN), "past the bytes nothing happens");
    }

    #[test]
    fn points_ride_in_chunk_8() {
        use crate::save::{SaveData, Scope};
        const AT: u32 = 0x1a7a68;
        use rc_formats::disc::save_game::{Chunk, ChunkDesc, ChunkTables, Section};
        let tables = ChunkTables { global: vec![ChunkDesc { addr: AT, size: LEN as u32, id: 8 }], level: Vec::new() };
        let mut s = SaveData { tables, global: Section::new(vec![Chunk::new(8, vec![0; LEN])]), levels: Vec::new() };
        let mut g = GameState::default();
        g.award_skill_point(point::BOSS_A44);
        g.store(&mut s);
        assert_eq!(s.u8(Scope::Global, AT + point::BOSS_A44 as u32), 1);
        let mut back = GameState::default();
        back.load(&s);
        assert!(back.has_skill_point(point::BOSS_A44) && !back.has_skill_point(point::BOSS_9F8));
        back.enter_level(2);
        assert!(back.has_skill_point(point::BOSS_A44), "kept across levels");
    }

    #[test]
    fn count_is_capped_at_30() {
        let mut g = GameState::default();
        assert_eq!(g.skill_point_count(), 0);
        g.skill_points[3] = 1;
        g.skill_points[0x1f] = 2;
        assert_eq!(g.skill_point_count(), 2);
        g.skill_points = [1; LEN];
        assert_eq!(g.skill_point_count(), USED);
    }
}
