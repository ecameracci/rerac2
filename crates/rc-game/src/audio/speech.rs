//! The dialogue request (L02 `0x1a63cc`) and its start `0x335110(id, flags, moby, vol)` on the dialogue player
//! (`0x1a6410`, the player the scenes' speech plays on too: crate::audio::scene).
//!
//! An object asks for a line by writing its id to `0x1a63cc` when the dialogue is free — the player idle and nothing
//! asked ([`AudioSystem::request_speech`]; Maktar arena's announcer `0x401d98`: its line's id + 2000). music_Update
//! (`0x335da0`) then starts it once the player is idle (a line still playing is stopped first, state 5) with
//! `0x335110(id, 0, owner 0, 0x400)` ([`AudioSystem::speech_update`]): the id picks the stream from the level's
//! track table (crate::audio::music::MusicTrack::table) — 2000..2999 stream id − 2000 plus the language's offset
//! (`0x1a7bbc`: English 0), 3000..3999 stream id − 3000 — played alone in group 2.
//!
//! Not ported: the other ranges of `0x335110` (below 2000 the scenes' table `0x151858` and `0x14db68`, 4000.. the
//! space tables `0xb038` / `0x14fed0`, 6000.. `0x1481a0`), the sound slot it takes for the line's place (a line
//! without an owner sits at the listener, 2-D) and its stereo second stream, the other languages' offset, the line
//! started paused and continued once buffered (`0x401e20` → `0x3354e0`: the port starts it at once).

use super::AudioSystem;
use std::sync::Arc;

impl AudioSystem {
    /// The dialogue is free: the player idle (`0x1a6410` = 0) and nothing asked (`0x1a63cc` = −1).
    pub fn speech_free(&self) -> bool { self.speech_req.is_none() && !self.speech_playing() }

    /// `0x1a63cc = id` when the dialogue is free; whether it was taken.
    pub fn request_speech(&mut self, id: i32) -> bool {
        if !self.speech_free() { return false; }
        self.speech_req = Some(id);
        true
    }

    /// music_Update's dialogue step (`0x335da0`): the request starts once the player is idle; a line still playing is
    /// stopped first.
    pub(crate) fn speech_update(&mut self) {
        let Some(id) = self.speech_req else { return };
        if self.speech_playing() {
            self.stop_speech();
            return;
        }
        self.speech_req = None;
        self.start_dialogue(id);
    }

    /// `0x335110(id, 0, 0, 0x400)` for the track table's ranges (module docs).
    fn start_dialogue(&mut self, id: i32) {
        let entry = match id {
            2000..=2999 => id - 2000,
            3000..=3999 => id - 3000,
            _ => return,
        };
        let Some(t) = usize::try_from(entry).ok().and_then(|e| self.data.music.get(e)).and_then(Option::as_ref) else { return };
        let (rate, body) = (t.rate, Arc::clone(&t.body));
        self.stop_speech();
        self.start_speech_body(rate, body);
    }
}
