//! The cheats: one byte each at `0x1a7a48` (the save's global chunk 7; [`GameState::cheats`]), toggled by the pause's
//! Cheats page (crate::menus::special, `0x349d80`) once enough skill points are had, read by the game while it plays.
//!
//! **The bytes** ([`cheat`]): 0 the actors' big heads, 1 Ratchet's, 2 Snow Dude, 3 Clank's, 4 the mirrored levels, 5
//! the left-handed Ratchet, 6 the invincibility at full health, 7 the enemies' big heads, 8 the sheep cars, 9 the funny
//! side, 10 the tux, 11 Beach Boy.
//!
//! **The bodies** (`0x2fe988`, the armour loader): the armour row it streams is the worn armour's (`0x1a7a20`), unless
//! a body cheat is on: byte 9 → row 6, else 2 → row 8, else 11 → row 7, else 10 → row 5 ([`body_row`]). The page's
//! toggle of 2, 9, 10 or 11 re-dresses Ratchet at once (`0x2fecd0(0x1a7a20)`: the loader, the textures' upload, the mesh
//! relocated, `0x1a7290` the row dressed); the level entry (`0x2a3b88`) re-dresses when the worn armour changed. The
//! engine dresses class 0 with the row [`GameState::body_row`] asks for (crate::armour_dress in rc-engine).
//!
//! **The mirror** (byte 4): the pad read (`0x3306b0`) negates both sticks' x and swaps ← / → (crate::pad's mirror);
//! the camera update (`0x2d9f38`) rebuilds the camera matrix's second row from the other two the other way round
//! (`0x2eef88(0x1b8f80, 0x1b8f90, 0x1b8f70)`: the row negated, the world drawn as its mirror image), as the ship's
//! transition (`0x2f2af0` mode 6) does with `0x2eeee0(−1, 0x1b8f80)`; the hero's stick direction (`0x2b3af8`) takes
//! the stick's x unnegated against that row, so Ratchet runs the way the stick points on the mirrored screen. The
//! engine mirrors the world's frame (crate::mirror_render in rc-engine) and the pad.
//!
//! Not ported here (other systems read them): the big heads (byte 1: Ratchet's head joint scale `gp−0x72fc` 1.57 for
//! 0.875, `0x2d51f8`, `0x2ab498`; byte 3: Clank's `gp−0x72f8` 1.8 for 1.0, `0x2d51f8`, `0x2a5840`, `0x2a5cb0`,
//! `0x2a7170`, `0x2a7ac8`, `0x2ab498`, `0x2b8dc0`; byte 0: the scene actors' head joints from the table `0x2a0e40`,
//! `0x2fd5b0`, `0x371538`; byte 7: the enemies' head scale `+0x70`, `0x327308`), the left-handed Ratchet (byte 5:
//! `0x2c4560`, `0x2d2508`), the invincibility at full health (byte 6: L00 `0x3a0800`), the hero's back items hidden
//! under a body or armour 4 (`0x2b13c8`, `0x2afc68`), the first-person weapon's offset mirrored (`0x2d5ce0`), two
//! level classes' stick reads (L00 `0x3df6d8`, `0x402450`); the sheep cars (byte 8: no reader on levels 0..3).

use crate::game::GameState;

/// The bytes the cheat rows use (`0x1a7a48..0x1a7a54`).
pub const LEN: usize = 12;

/// The cheat bytes (index into `0x1a7a48`).
pub mod cheat {
    pub const ACTOR_HEADS: usize = 0;
    pub const BIG_HEAD: usize = 1;
    pub const SNOW_DUDE: usize = 2;
    pub const CLANK_HEAD: usize = 3;
    pub const MIRROR: usize = 4;
    pub const LEFT_HANDED: usize = 5;
    pub const INVINCIBLE: usize = 6;
    pub const ENEMY_HEADS: usize = 7;
    pub const SHEEP_CARS: usize = 8;
    pub const FUNNY_SIDE: usize = 9;
    pub const TUX: usize = 10;
    pub const BEACH_BOY: usize = 11;
}

/// `0x349d80`: the bytes whose toggle re-dresses Ratchet (`b − 9 < 2 || b == 2 || b == 0xb`).
pub fn redresses(b: usize) -> bool { matches!(b, cheat::FUNNY_SIDE | cheat::TUX | cheat::SNOW_DUDE | cheat::BEACH_BOY) }

/// `0x2fe988`'s row: a body cheat's (in its order), else the worn armour's.
pub fn body_row(worn: i16, cheats: &[u8; LEN]) -> usize {
    let on = |b: usize| cheats[b] != 0;
    if on(cheat::FUNNY_SIDE) {
        6
    } else if on(cheat::SNOW_DUDE) {
        8
    } else if on(cheat::BEACH_BOY) {
        7
    } else if on(cheat::TUX) {
        5
    } else {
        worn.max(0) as usize
    }
}

impl GameState {
    /// Cheat `b` on (`0x1a7a48[b]`).
    pub fn cheat(&self, b: usize) -> bool { self.cheats.get(b).is_some_and(|&c| c != 0) }

    /// The Cheats page's toggle of byte `b`.
    pub fn set_cheat(&mut self, b: usize, on: bool) {
        if let Some(c) = self.cheats.get_mut(b) { *c = on as u8; }
    }

    /// The armour row Ratchet wears (`0x2fe988`, [`body_row`]).
    pub fn body_row(&self) -> usize { body_row(self.armor, &self.cheats) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bodies_in_the_loaders_order() {
        let mut c = [0; LEN];
        assert_eq!(body_row(3, &c), 3);
        c[cheat::TUX] = 1;
        assert_eq!(body_row(3, &c), 5);
        c[cheat::BEACH_BOY] = 1;
        assert_eq!(body_row(3, &c), 7);
        c[cheat::SNOW_DUDE] = 1;
        assert_eq!(body_row(3, &c), 8);
        c[cheat::FUNNY_SIDE] = 1;
        assert_eq!(body_row(3, &c), 6);
        assert!(redresses(cheat::TUX) && redresses(cheat::SNOW_DUDE) && !redresses(cheat::MIRROR));
    }
}
