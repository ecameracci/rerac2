//! The level's sound table (L02 `0x367f58` / `0x367ff0`; L00 `0x360750`): the level defs (the core's remap,
//! rc_formats::sound_bank::LevelSounds::level_defs, in memory at `0x1b30ec`, count `0x1b30e8`, 0x20 each) played into a
//! sound slot (`0x367bd0`, crate::audio::voices::SoundSlots::play) with the volume 0x400; the slot keeps the def's index
//! (`+0xe`) and its owner moby (`+0x18`).
//!
//! The table is split at `$gp − 0x60e4` (`0x1a8f0c`, the level settings' word after their variant blocks:
//! [`crate::audio::LevelAudio::level_split`]):
//! * `0x367f58(i, flags, moby)` plays def i of the part below the split: the menus' and the objects' own sounds (the
//!   GUI's set: the quick select's 3, the ship pages', Maktar arena's menu 3 / 4);
//! * `0x367ff0(i, flags, moby)` plays def split + i: the part the footsteps (`0x368128`: split + surface·4 + foot·2 +
//!   variant) share (Maktar arena's crowd gasp 0x17, the flame columns' looped 0x11).
//!
//! Each returns the slot, −1 when nothing plays (an index past its part, the loop flag not the def's, no free slot, too
//! far: `0x367bd0`'s refusals). A play without a moby sits at the listener, 2-D (flags | 0x11).

use super::class_sounds::LEVEL_SOUND_LOG_CLASS;
use super::voices::{Listener, Owner};
use super::AudioSystem;
use crate::rng::Rng;

impl AudioSystem {
    /// `0x367f58(i, flags, moby)`: def `i` below the split (module docs); `owner` the moby and its position.
    pub fn play_level_sound(&mut self, i: i32, flags: u8, owner: Option<(Owner, [f32; 3])>, listener: &Listener, rng: &mut Rng, tick: u64) -> i32 {
        match usize::try_from(i) {
            Ok(i) if i < self.data.level_split => self.play_level_def(i, flags, owner, listener, rng, tick),
            _ => -1,
        }
    }

    /// `0x367ff0(i, flags, moby)`: def split + `i` (module docs).
    pub fn play_level_sound_after(&mut self, i: i32, flags: u8, owner: Option<(Owner, [f32; 3])>, listener: &Listener, rng: &mut Rng, tick: u64) -> i32 {
        match usize::try_from(i) {
            Ok(i) => self.play_level_def(self.data.level_split + i, flags, owner, listener, rng, tick),
            _ => -1,
        }
    }

    /// `0x367bd0(def, flags, moby, 0, 0x400)` with the def's index in the slot (`+0xe`).
    fn play_level_def(&mut self, i: usize, flags: u8, owner: Option<(Owner, [f32; 3])>, listener: &Listener, rng: &mut Rng, tick: u64) -> i32 {
        let Some(def) = self.data.sounds.level_defs.get(i).copied() else { return -1 };
        let k = self.slots.play(&def, flags, owner.map(|o| o.0), owner.map(|o| o.1), None, 0x400, listener, rng);
        if k >= 0 { self.slots.slots[k as usize].class_index = i as u16; }
        if let Some(log) = self.play_log.as_mut() { log.push((tick, LEVEL_SOUND_LOG_CLASS, i as i32, flags as u32, k)); }
        k
    }
}
