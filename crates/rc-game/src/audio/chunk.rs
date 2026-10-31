//! The chunk switch's sound part (L02 `0x3082e0(k, track)`, L01 `0x2f9bd8`) and the objects' music request
//! (L02 `0x335928(track, stinger)`).
//!
//! A level in chunks (Maktar's resort and arena) has a sound bank per chunk (WAD header `+0x48 + 8k`, `0x150820`):
//! chunk k's is bank handle k + 1 (`0x189e00[k + 1]`), and a class whose remap list opens with k plays its defs from
//! it (rc_formats::sound_bank::parse_level_sounds: bank handle k + 1). The level start loads chunk 0's (bank 1,
//! `0x304420` stage 4 → `0x32e4a0(0)`). The switch, around the world's swap (rc-engine's chunk_switch), does
//! ([`AudioSystem::chunk_switch`]):
//! 1. `0x368320(0, 0, 0, 0, 0)`: the reverb request off (owner none, type 0, depth 0, delay 0, feedback 0; the same
//!    request as crate::audio::reverb's at `0x1886c0`);
//! 2. `0x368398`: every sound stopped (`snd_StopAllSounds` `0x132ac8`, once the IOP has answered the batch) and every
//!    sound slot (`0x188660`, 30 × 0x70) freed;
//! 3. `0x335998` music_Stop: the IOP's answers waited for, the streams stopped (`0x133310`), every music player idle
//!    (main `0x1a63ec`, `0x1a6410`, transition `0x1a6434`), a pending track (`0x1a63e4`) the current one;
//! 4. `0x334fe0(k)`: the three chunk banks unloaded, bank k + 1 loaded when the level has it (a def on a bank that is
//!    not loaded plays nothing);
//! 5. `0x335630(track, 1, 0x400)`: music_start_track with the caller's track (the arena master its `+0x220` into the
//!    arena and `+0x224` back, the teleporter its `+0x40`, 0 when negative).
//!
//! The port's 989snd plays from one bank and one sample RAM, so the level load appends the three chunk banks to the
//! level's once each ([`ChunkBanks`]) and the bank load is a remap: a chunk def's `index` points into its bank when
//! that bank is the loaded one, else 0xffff (nothing plays).
//!
//! The music request `0x335928(track, stinger)` ([`AudioSystem::music_request`]) is R&C1's `FUN_0027a248`
//! (crate::audio::music::Music::request) with R&C2's 16-bit track: a request that changes nothing (the current track,
//! nothing pending) is dropped; else the track is cut to its low byte (unless it is 0xffff with no stinger) and, both
//! players idle (main and transition state 0), becomes the current track at once, otherwise the pending one with its
//! stinger for music_Update. Its tracks are stream indices of the level's table (crate::audio::music: Maktar's arena
//! asks for 4 and 0, the stinger 228 and the bosses' 230).
//!
//! Not ported: the switch's `0x332140(2)` → `0x335b10(0)` (in game mode 2, R&C2's music players' `+0xc` / `+0xe`
//! words `0x8000` / 0: their pause timers, which the port's music does not have), and music_Stop's `0x133490(1)`.

use super::voices::Slot;
use super::AudioSystem;
use rc_formats::sound_bank::LevelSounds;

/// The level's chunk banks as the port holds them (module docs).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChunkBanks {
    /// Chunk k's bank (handle k + 1): the id its sound 0 takes in the appended bank; None: the level has none.
    pub base: [Option<u16>; 3],
    /// The chunk whose bank is loaded (`0x189e00[k + 1]` ≠ 0); None: none.
    pub loaded: Option<usize>,
    /// Every class def on a chunk bank: `(class, def, bank handle, its sound id in its own bank)`, in class order.
    defs: Vec<(usize, usize, u32, u16)>,
}

impl ChunkBanks {
    /// The chunk defs of `sounds` (bank handles 1..3) before any bank is loaded: their `index` is still their own
    /// bank's sound id. `base`: where each chunk's bank was appended.
    pub fn new(sounds: &LevelSounds, base: [Option<u16>; 3]) -> Self {
        let defs = sounds
            .classes
            .iter()
            .enumerate()
            .flat_map(|(c, cs)| cs.defs.iter().enumerate().filter(|(_, d)| (1..=3).contains(&d.bank_handle)).map(move |(j, d)| (c, j, d.bank_handle, d.index)))
            .collect();
        ChunkBanks { base, loaded: None, defs }
    }

    /// `0x334fe0(k)`: every chunk bank unloaded, then chunk `k`'s loaded when the level has it. Each chunk def points
    /// into its bank when that is the loaded one, else at nothing (0xffff).
    pub fn load(&mut self, sounds: &mut LevelSounds, k: usize) {
        self.loaded = self.base.get(k).copied().flatten().map(|_| k);
        for &(c, j, handle, own) in &self.defs {
            let Some(d) = sounds.classes.get_mut(c).and_then(|cs| cs.defs.get_mut(j)) else { continue };
            d.index = match (self.loaded, self.base[handle as usize - 1]) {
                (Some(l), Some(b)) if l + 1 == handle as usize && own != 0xffff => b + own,
                _ => 0xffff,
            };
        }
    }
}

impl ChunkBanks {
    /// The classes with a def on the loaded bank that plays (for the logs): `(o_class, defs)`.
    pub fn loaded_classes(&self, sounds: &LevelSounds) -> Vec<(i32, usize)> {
        let Some(l) = self.loaded else { return Vec::new() };
        let mut out: Vec<(i32, usize)> = Vec::new();
        for &(c, _, handle, own) in &self.defs {
            if handle as usize != l + 1 || own == 0xffff { continue; }
            let o = sounds.classes.get(c).map_or(-1, |cs| cs.o_class);
            match out.last_mut() {
                Some(x) if x.0 == o => x.1 += 1,
                _ => out.push((o, 1)),
            }
        }
        out
    }
}

impl AudioSystem {
    /// The chunk switch's sound part (`0x3082e0(k, track)`, module docs): the reverb off, every sound stopped, the
    /// music stopped, chunk `k`'s bank loaded, `music_start_track(track, 1, 0x400)`.
    pub fn chunk_switch(&mut self, k: i32, track: i32) {
        self.reverb.set(None, 0, 0, 0, 0);
        self.stop_all_slots();
        self.music_stop();
        if let Ok(k) = usize::try_from(k) {
            let d = &mut self.data;
            d.chunk_banks.load(&mut d.sounds, k);
        }
        if let Ok(t) = i16::try_from(track) { self.music.start_track(t, 1, 0x400, &mut self.stream_cmds); }
    }

    /// `0x368398`: the batch flushed and answered (the EE waits for the IOP), `snd_StopAllSounds`, every slot free.
    pub(crate) fn stop_all_slots(&mut self) {
        self.run_snd_commands();
        for r in std::mem::take(&mut self.snd_replies) { self.slots.apply_reply(r); }
        self.snd.stop_all(&mut self.spu);
        for s in &mut self.slots.slots { *s = Slot::default(); }
    }

    /// `0x335998` music_Stop: the stream commands run and answered (the EE waits for the pending handles), every
    /// stream stopped, the players reset (crate::audio::music::Music::stop).
    pub(crate) fn music_stop(&mut self) {
        for r in std::mem::take(&mut self.stream_replies) { self.music.reply(r); }
        self.run_stream_commands();
        for r in std::mem::take(&mut self.stream_replies) { self.music.reply(r); }
        self.streams.stop_all(&mut self.snd.vm, &mut self.spu);
        self.music.stop();
    }

    /// `0x335928(track, stinger)`: the music request (module docs).
    pub fn music_request(&mut self, track: i32, stinger: i32) {
        let t = track as u16;
        if self.music.main.track as u16 == t && self.music.pending_track == -1 { return; }
        let t = if t == 0xffff && stinger == -1 { -1 } else { (t & 0xff) as i16 };
        self.music.request(t, stinger as i16);
    }

    /// The low bytes of the current track `0x1a63f0` and the pending one `0x1a63e4` (0xff: none), as the arena master
    /// compares them with its track.
    pub fn music_tracks(&self) -> (u8, u8) { (self.music.main.track as u8, self.music.pending_track as u8) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rc_formats::sound_bank::{ClassSounds, SoundDef};

    fn def(handle: u32, index: u16) -> SoundDef { SoundDef { bank_handle: handle, index, ..Default::default() } }

    #[test]
    fn chunk_bank_load_points_into_the_loaded_bank_only() {
        let mut sounds = LevelSounds::default();
        sounds.classes = vec![
            ClassSounds { o_class: 1, defs: vec![def(0, 5), def(1, 2)], ..Default::default() },
            ClassSounds { o_class: 2, defs: vec![def(2, 3), def(2, 0xffff), def(4, 7)], ..Default::default() },
        ];
        let mut banks = ChunkBanks::new(&sounds, [Some(100), Some(200), None]);
        banks.load(&mut sounds, 0);
        let idx = |s: &LevelSounds| s.classes.iter().flat_map(|c| c.defs.iter().map(|d| d.index)).collect::<Vec<_>>();
        assert_eq!(idx(&sounds), [5, 102, 0xffff, 0xffff, 7]);
        banks.load(&mut sounds, 1);
        assert_eq!(banks.loaded, Some(1));
        assert_eq!(idx(&sounds), [5, 0xffff, 203, 0xffff, 7]);
        // A chunk without a bank: nothing loaded, every chunk def silent.
        banks.load(&mut sounds, 2);
        assert_eq!(banks.loaded, None);
        assert_eq!(idx(&sounds), [5, 0xffff, 0xffff, 0xffff, 7]);
    }
}
