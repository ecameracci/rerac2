//! The weapons' experience and upgrades: every weapon earns experience from its kills and moves up its chain of item
//! records (the Lancer's row 0x1e → the Heavy Lancer 0x3c; in the challenge mode the bought Mega rows on, 0x4f → 0x50).
//!
//! **The state** ([`Upgrades`], kept in the save): `0x139568[item]` the item's current row of the item records
//! (`0x262500 + 0xe0·row`, chunk 30, 0x50 bytes; every lookup of an item's record goes through it:
//! [`GameState::item_row`], [`GameState::record`], crate::hero::items::row); `0x139868 + 4·item` its experience (chunk
//! 40, 0x38 words, in 32nds: the record's `+0x5c` counts whole points). A row's level is its record's `−0x0c`
//! ([`ROW_LEVELS`]: 0 the base row, 1 the earned upgrade, 2 and 3 the challenge mode's Mega rows), `+0x3a` the next
//! row, `+0x3c` the one before.
//!
//! **The experience** (the drop `0x31adc0`'s first part, [`GameState::kill_xp`]): a kill of a moby with experience
//! (`+0xb6`), outside the bodies 1 and 2, gives that much by the uid's break count (`0x2e4670` over `0x1a89d8` /
//! `0x1a89e0`: [`XP_PERCENT`]; in an arena challenge the arena's law instead, [`arena_xp`]), plus a fiftieth of the
//! experience bank `0x1a7a18`; all of it to Ratchet's experience `0x1a7a14` (the health widget shown 720 ticks) and to
//! the weapon that hit the moby last (its health block's `+0x17`, which `0x3228f8` names when a new hitter hits it:
//! [`hitter_item`]).
//!
//! **The tick** (`0x2d6258`, every frame but the 16th ones past the first: [`GameState::upgrade_tick`]): each item on a
//! base row (in the challenge mode on any row but its first upgrade) that has upgrades and not at level 3 of a vendor
//! record upgrades once its experience reaches the row's `+0x5c`: the items 0x1f / 9 / 0x2d and the item in Ratchet's
//! hand (when he is on the ground and it is his slot's last held, `0x18a160` / `0x1a7398 + 4·slot`) through the upgrade
//! screen (game mode 8, `0x37fb10`: crate::menus::upgrade); any other at once ([`GameState::upgrade`] `0x2f4288`, its
//! message [`GameState::upgrade_message`] `0x2f48e0`) — in the challenge mode only from the Mega rows on. Its tail, every
//! 128th frame: skill point 0x1a once every vendor weapon is had and every one with upgrades past its base row
//! ([`GameState::all_upgraded`]).
//!
//! **The Mega purchase** (`0x2f4bf8`, the vendor's challenge-mode Mega entry, crate::vendor): the row two up from the
//! base (one for the R&C1 weapons, [`crate::vendor::SPECIALS`]), its experience cleared ([`GameState::buy_mega`]).
//!
//! Dev switches ([`dev_switches`]): `RC_ITEM_ROWS=item:row,…` puts items on rows, `RC_WEAPON_XP=item:points,…` gives
//! them experience (whole points, ×32); `item:value@frame` waits for that play frame.
//!
//! Not ported: the boss slots' split of the experience by the weapons' damage (`0x1b1ff0` / `0x1b1ff4` and their
//! tallies `0x1fdef8`, which only other levels' bosses keep), the arena statistics `0x2a48f0(0x1aa1b8, …)`, item 0x10's
//! challenge-mode row at its purchase (`0x2f46e0`: its weapon is not ported).

use crate::game::GameState;
use crate::moby::MobyWorld;
use rc_formats::disc::menu_tables::ItemRecord;

/// `0x139568`'s bytes (chunk 30).
pub const ROW_SLOTS: usize = 0x50;
/// `0x139868`'s words (chunk 40) and the items the tick walks.
pub const XP_SLOTS: usize = 0x38;
/// `0x188610`'s words (chunk 41, 0x4c bytes): the arena's kills by drop index.
pub const ARENA_SLOTS: usize = 19;

/// `0x2624f4 + 0xe0·row`: each row's level (rows 0..0x76; the same bytes on every level).
pub const ROW_LEVELS: [u8; 0x77] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1,
    1, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 2, 3, 3, 4, 5, 1, 1, 1,
    1, 1,
];

/// Row `row`'s level ([`ROW_LEVELS`]; 0 past the table).
pub fn level_of(row: usize) -> u8 { ROW_LEVELS.get(row).copied().unwrap_or(0) }

/// `0x1a89d8` (the uid's bit 3 clear) and `0x1a89e0` (set): the experience's percentage by the uid's break count.
pub const XP_PERCENT: [[u8; 8]; 2] = [[100, 50, 40, 30, 25, 20, 15, 10], [100, 30, 10, 10, 10, 10, 10, 10]];
/// `0x1a89e8` / `0x1a8a10` (s16 by drop index): the arena's kills before its experience thins, and its experience.
pub const ARENA_KILLS: [i16; 20] = [48, 131, 30, 84, 60, 79, 346, 243, 27, 88, 105, 74, 67, 121, 90, 75, 787, 100, 100, 0];
pub const ARENA_XP: [i16; 20] = [27, 27, 27, 27, 27, 27, 27, 27, 81, 81, 81, 81, 81, 81, 36, 81, 81, 0, 0, 25];

/// `0x1a8e90`: the upgrade messages by the new row's level (1..3): `(item, text)`.
pub const MESSAGES: [&[(i32, i32)]; 3] = [
    &[(0xa, 0x1295), (0x1e, 0x1236), (0x1f, 0x123e), (0x1d, 0x123b), (0x29, 0x123c), (0x16, 0x1238), (0x17, 0x123a), (0x18, 0x123d), (0x1a, 0x1239), (0x2a, 0x1237), (0x1b, 0x1244), (0x25, 0x1249), (0x1c, 0x128f), (0x19, 0x1290), (0x10, 0x1247), (0x20, 0x1291), (0x2d, 0x12b5), (0x9, 0x12b6)],
    &[(0xa, 0x1295), (0x1e, 0x12c6), (0x1f, 0x12cd), (0x1d, 0x12ce), (0x29, 0x12cb), (0x16, 0x12c8), (0x17, 0x12ca), (0x18, 0x12c9), (0x1a, 0x12cc), (0x2a, 0x12c7), (0x1b, 0x12d0), (0x25, 0x12cf), (0x1c, 0x12d1), (0x19, 0x12d2), (0x10, 0x1b8f), (0x20, 0x12d3), (0x2d, 0x1bc2), (0x12, 0x12d4)],
    &[(0xa, 0x1295), (0x1e, 0x1b96), (0x1f, 0x1b9d), (0x1d, 0x1b9e), (0x29, 0x1b9b), (0x16, 0x1b98), (0x17, 0x1b9a), (0x18, 0x1b99), (0x1a, 0x1b9c), (0x2a, 0x1b97), (0x1b, 0x1ba0), (0x25, 0x1b9f), (0x1c, 0x1ba1), (0x19, 0x1ba2), (0x10, 0x1ba5), (0x20, 0x1ba3), (0x2d, 0x1bc3), (0x12, 0x1ba4)],
];
/// `0x2f48e0`'s help texts: the wrench's (0x9f8, slot 0x4f), item 0x1b's (0x9f7, 0x4e), any other's (0x9aa, slot 1,
/// the first time only: `0x18c964`, slot 1's count).
pub const HELP_WRENCH: (i32, i32) = (0x9f8, 0x4f);
pub const HELP_1B: (i32, i32) = (0x9f7, 0x4e);
pub const HELP_FIRST: (i32, i32) = (0x9aa, 1);
/// The skill point for every weapon upgraded (`0x1a7a82`).
pub const SKILL_POINT: usize = 0x1a;

/// The upgrades' part of the game state (module docs).
#[derive(Clone, Debug)]
pub struct Upgrades {
    /// `0x139568[item]`: the item's row.
    pub rows: [u8; ROW_SLOTS],
    /// `0x139868 + 4·item`: its experience (32nds).
    pub xp: [i32; XP_SLOTS],
    /// `0x188610 + 4·k`: the arena's kills by drop index (crate::game::GameState::arena_drop).
    pub arena_kills: [i32; ARENA_SLOTS],
}

impl Default for Upgrades {
    fn default() -> Self { Upgrades { rows: std::array::from_fn(|i| i as u8), xp: [0; XP_SLOTS], arena_kills: [0; ARENA_SLOTS] } }
}

/// What `0x3228f8` needs of the hand (the item pass keeps it each tick, [`Hitters::refresh`]): the hand item's moby
/// (`0x18b040`), the item held (`0x18b068`) and every item's current row's class (`0x262504 + 0xe0·0x139568[item]`).
#[derive(Clone, Copy, Debug)]
pub struct Hitters {
    pub hand: Option<usize>,
    pub item: i32,
    pub classes: [i16; XP_SLOTS],
}

impl Default for Hitters {
    fn default() -> Self { Hitters { hand: None, item: 0, classes: [-1; XP_SLOTS] } }
}

impl Hitters {
    /// The item pass's copy (its classes from the hero's rows, crate::hero::items::row).
    pub fn refresh(&mut self, game: &GameState, hand: Option<usize>, item: i32) {
        self.hand = hand;
        self.item = item;
        for (i, c) in self.classes.iter_mut().enumerate() {
            *c = crate::hero::items::row(game, i as i32).map_or(-1, |r| r.class);
        }
    }
}

/// `0x3228f8(moby)`: the item a hitter belongs to — by its class (the projectiles that carry their weapon: the Chopper's
/// blade 0x087a → 0x16, the bombs 0x0ce4 / 0x0ce5 / 0x0c07 → 0x2a, the Spiderbots 0x0b58 / 0x0e79 → 0x20, …); else the
/// hand item and what it owns (`+0xb8`) are the item held; else the item whose current row's class is the moby's or its
/// owner's; 0xff none.
pub fn hitter_item(w: &MobyWorld, id: usize) -> u8 {
    let Some(m) = w.mobys.get(id) else { return 0xff };
    let by_class = match m.o_class {
        0xb2e => Some(0x1d),
        0x87a => Some(0x16),
        0xac => Some(0xe),
        0x79 => Some(0xc),
        0x5da => Some(0x1b),
        0xa66 | 0xa82 => Some(0x29),
        0x9a9 | 0xa9b => Some(0x1c),
        0xce4 | 0xce5 | 0xc07 => Some(0x2a),
        0xb58 | 0xe79 => Some(0x20),
        0xe64 | 0xece => Some(0x25),
        0xd01 => Some(0x18),
        _ => None,
    };
    if let Some(i) = by_class { return i; }
    let h = &w.hitters;
    if h.hand == Some(id) || (m.parent.is_some() && m.parent == h.hand) { return h.item as u8; }
    let owner = m.parent.and_then(|p| w.mobys.get(p)).map(|p| p.o_class);
    h.classes.iter().position(|&c| c == m.o_class || owner == Some(c)).map_or(0xff, |i| i as u8)
}

/// `0x31adc0`'s arena law (`0x1aa1b0` = `k`): the arena's experience, thinned once its kills (twice `0x188610[k]`)
/// pass `0x1a89e8[k]`, at least 1.
pub fn arena_xp(kills: i32, k: usize) -> i32 {
    let (Some(&base), Some(&most)) = (ARENA_XP.get(k), ARENA_KILLS.get(k)) else { return 1 };
    let n = kills * 2;
    let v = if (most as i32) < n && n != 0 { base as i32 * most as i32 / n } else { base as i32 };
    v.max(1)
}

/// The record of row `row`.
fn rec(records: &[ItemRecord], row: usize) -> Option<&ItemRecord> { records.get(row) }

impl GameState {
    /// `0x139568[item]`: the item's row (the item's own past the table, or when the save left it 0).
    pub fn item_row(&self, item: i32) -> usize {
        let Ok(i) = usize::try_from(item) else { return 0 };
        match self.upgrades.rows.get(i) {
            Some(&r) if r != 0 || i == 0 => r as usize,
            _ => i,
        }
    }

    /// Item `item`'s record: its current row's (`0x262500 + 0xe0·0x139568[item]`).
    pub fn record<'a>(&self, records: &'a [ItemRecord], item: i32) -> Option<&'a ItemRecord> {
        if item < 0 { return None; }
        rec(records, self.item_row(item))
    }

    /// The level of item `item`'s row (`0x2624f4 + 0xe0·0x139568[item]`).
    pub fn upgrade_level(&self, item: i32) -> u8 { level_of(self.item_row(item)) }

    /// Item `item`'s experience in whole points (`0x139868 + 4·item` >> 5).
    pub fn weapon_xp(&self, item: i32) -> i32 { usize::try_from(item).ok().and_then(|i| self.upgrades.xp.get(i)).map_or(0, |&x| x >> 5) }

    /// `0x2f4288(item)`: the item up to its next row (`+0x3a`; none: nothing) — its experience raised to the old
    /// row's `+0x5c` when it has one, and a vendor record's ammo the new row's most (`+0x7e`).
    pub fn upgrade(&mut self, records: &[ItemRecord], item: i32) {
        let Some(r) = self.record(records, item).copied() else { return };
        if r.next == 0 { return; }
        let i = item as usize;
        if 0 <= r.xp_max {
            if let Some(x) = self.upgrades.xp.get_mut(i) { *x = (*x).max(r.xp_max << 5); }
        }
        if let Some(b) = self.upgrades.rows.get_mut(i) { *b = r.next as u8; }
        if let Some(n) = rec(records, (r.next as u8) as usize) {
            if n.mega {
                if let Some(a) = self.ammo.get_mut(i) { *a = n.max_ammo as i32; }
            }
        }
    }

    /// `0x2f48e0(item)`: the help text (the wrench's, item 0x1b's, or the first time any other's), then the message
    /// for the new row's level ([`MESSAGES`]; the wrench's by its level, 0x1296 / 0x1297; none found: text 0).
    pub fn upgrade_message(&mut self, item: i32) {
        match item {
            0xa => { self.help(HELP_WRENCH.0, HELP_WRENCH.1); }
            0x1b => { self.help(HELP_1B.0, HELP_1B.1); }
            _ => {
                if self.help_counts.get(1).copied().unwrap_or(0) == 0 { self.help(HELP_FIRST.0, HELP_FIRST.1); }
            }
        }
        let lv = self.upgrade_level(item);
        let mut text = lv.checked_sub(1).and_then(|k| MESSAGES.get(k as usize)).and_then(|t| t.iter().find(|&&(i, _)| i == item)).map_or(-1, |&(_, t)| t);
        if item == 0xa {
            let w = self.upgrade_level(0xa);
            for (at, t) in [(1, 0x1296), (2, 0x1297), (3, 0x1296), (4, 0x1297)] {
                if at < w { text = t; }
            }
        }
        self.messages.push(crate::msgbox::Request::Text { text: text.max(0), ticks: None });
    }

    /// `0x2f43d0(item, n)`: the item put `n` rows up from its base, when it has as many upgrades; whether it was.
    pub fn set_upgrade(&mut self, records: &[ItemRecord], item: i32, n: i32) -> bool {
        if crate::vendor::upgrades(records, item) < n { return false; }
        let mut row = item.max(0) as usize;
        for _ in 0..0x80 {
            match rec(records, row).map(|r| r.base) {
                Some(b) if b != 0 => row = b as usize,
                _ => break,
            }
        }
        for _ in 0..n.max(0) {
            row = rec(records, row).map_or(0, |r| r.next as usize);
        }
        if let Some(b) = self.upgrades.rows.get_mut(item.max(0) as usize) { *b = row as u8; }
        true
    }

    /// `0x2f4bf8(item)`: a vendor record's Mega upgrade bought — its row two up from the base (one for the R&C1
    /// weapons) and its experience cleared; whether it took.
    pub fn buy_mega(&mut self, records: &[ItemRecord], item: i32) -> bool {
        if !self.record(records, item).is_some_and(|r| r.mega) { return false; }
        let n = if crate::vendor::SPECIALS.contains(&item) { 1 } else { 2 };
        let ok = self.set_upgrade(records, item, n);
        if ok {
            if let Some(x) = self.upgrades.xp.get_mut(item.max(0) as usize) { *x = 0; }
        }
        ok
    }

    /// `0x31adc0`'s experience (module docs): a kill of a moby with `xp` (`+0xb6`) and uid `uid`, its health block's
    /// last weapon `weapon` (`+0x17`), Ratchet's body `body` (`0x18c0d4`). Before the drop's bolts and its uid count.
    pub fn kill_xp(&mut self, uid: i16, xp: i16, weapon: Option<u8>, body: u8) {
        if (xp == 0 && self.arena_drop == -1) || (1..=2).contains(&body) { return; }
        let bank = (self.xp_bank + 0x31) / 0x32;
        let mut v = if self.arena_drop == -1 {
            let t = &XP_PERCENT[self.uid_flag(uid) as usize];
            t[self.uid_count(uid) as usize & 7] as i32 * xp as i32 / 100
        } else {
            let k = self.arena_drop.max(0) as usize;
            arena_xp(self.upgrades.arena_kills.get(k).copied().unwrap_or(0), k)
        };
        v += bank;
        if let Some(x) = weapon.filter(|&w| w != 0xff).and_then(|w| self.upgrades.xp.get_mut(w as usize)) { *x += v; }
        self.xp_bank -= bank;
        self.xp += v;
        self.show_health = Some(0x2d0);
    }

    /// `0x31adc0`'s tail in an arena challenge: its kill counted (`0x188610[k]`).
    pub fn arena_kill(&mut self) {
        if self.arena_drop == -1 { return; }
        if let Some(n) = self.upgrades.arena_kills.get_mut(self.arena_drop.max(0) as usize) { *n += 1; }
    }

    /// `0x2d6258`'s weapons (module docs), after its health part and under the same frame test. `hand`: the item in
    /// Ratchet's hand (`0x18b068`); `grounded`: he is on the ground (`0x18a160`); `hp` his health (`0x18c2ec`).
    pub fn upgrade_tick(&mut self, records: &[ItemRecord], hand: i32, grounded: bool, hp: i32) {
        if self.frame & 0xf == 1 && self.frame != 1 { return; }
        let special = |i: i32| crate::vendor::SPECIALS.contains(&i);
        for i in 0..XP_SLOTS as i32 {
            let Some(r) = self.record(records, i).copied() else { continue };
            let lv = level_of(self.item_row(i));
            if lv != 0 && !(self.challenge && crate::vendor::upgrades(records, i) != 1) { continue; }
            if r.mega && lv == 3 { continue; }
            if crate::vendor::upgrades(records, i) <= 0 { continue; }
            if !(0 < r.xp_max && r.xp_max <= self.upgrades.xp[i as usize] >> 5) { continue; }
            // Not the original's: a weapon whose next row's hand the port does not run yet (the rows not in
            // hero::items::UPGRADE_ROWS: none now) stays at its
            // row rather than lose its hand.
            // The Clank Zapper's upgraded row names Clank's back moby (crate::hero::back), not a hand: its zaps run.
            let ported = crate::hero::items::row_at(r.next as usize).is_some_and(|n| i == crate::hero::items::ZAPPER || crate::moby::classes::update_for(n.class).is_some());
            if !ported { continue; }
            if matches!(i, 0x1f | 9 | 0x2d) {
                self.upgrade_screen(i, r.next as i32, hp);
            } else if hand == i {
                let slot_held = if r.slot == 0 { self.held } else { 0 };
                if grounded && slot_held == i { self.upgrade_screen(i, r.next as i32, hp); }
            } else if !self.challenge {
                self.upgrade(records, i);
                self.upgrade_message(i);
            } else if r.mega && (2 <= lv || (lv == 1 && special(i))) {
                let n = crate::vendor::upgrades(records, i);
                if 2 <= n || (n == 1 && special(i)) {
                    self.upgrade(records, i);
                    self.upgrade_message(i);
                }
            }
        }
        if self.frame & 0x7f == 0 && !self.has_skill_point(SKILL_POINT) && self.all_upgraded(records) {
            self.award_skill_point(SKILL_POINT);
        }
    }

    /// `0x37fb10(item, row)`: the upgrade screen asked for (`0x1b25f8` / `0x1b25fa`; game mode 8 in place of the
    /// mode, `0x32c5f0(8, 2, ..)`: refused while another is asked, crate::menus::upgrade).
    fn upgrade_screen(&mut self, item: i32, next: i32, hp: i32) {
        self.scenes.request(crate::menus::upgrade::MODE, false, item, next, hp);
    }

    /// `0x2d6258`'s tail: every vendor weapon (not the wrench, 9, nor 0xe; the R&C1 weapons only had) is had, and each
    /// with upgrades is past its base row.
    pub fn all_upgraded(&self, records: &[ItemRecord]) -> bool {
        let had = |i: i32| self.items.get(i as usize).is_some_and(|&b| b != 0);
        for i in 0..XP_SLOTS as i32 {
            let Some(r) = self.record(records, i) else { continue };
            if !r.mega || i == 0xa || i == 9 { continue; }
            if !had(i) { return false; }
            if i == 0xe || crate::vendor::SPECIALS.contains(&i) { continue; }
            if 0 < crate::vendor::upgrades(records, i) && level_of(self.item_row(i)) == 0 { return false; }
        }
        true
    }
}

/// The dev switches (module docs), at the level start; an `@frame` after a pair holds it back until that frame
/// ([`dev_tick`]: an upgrade then meets the weapon in Ratchet's hand, through the upgrade screen).
pub fn dev_switches(game: &mut GameState) {
    let pairs = |name: &str| -> Vec<(usize, i32, i32)> {
        std::env::var(name).ok().map_or(Vec::new(), |v| {
            v.split(',')
                .filter_map(|p| {
                    let (p, at) = p.split_once('@').map_or((p, 0), |(a, f)| (a, f.trim().parse().unwrap_or(0)));
                    let (a, b) = p.split_once(':')?;
                    let num = |s: &str| { let s = s.trim(); s.strip_prefix("0x").map_or_else(|| s.parse().ok(), |h| i32::from_str_radix(h, 16).ok()) };
                    Some((num(a)? as usize, num(b)?, at))
                })
                .collect()
        })
    };
    let mut later = DEV_LATER.lock().unwrap_or_else(|e| e.into_inner());
    for (item, row, at) in pairs("RC_ITEM_ROWS") {
        if 0 < at { later.push((at, item, Some(row), None)); } else if let Some(b) = game.upgrades.rows.get_mut(item) { *b = row as u8; }
    }
    for (item, points, at) in pairs("RC_WEAPON_XP") {
        if 0 < at { later.push((at, item, None, Some(points))); } else if let Some(x) = game.upgrades.xp.get_mut(item) { *x = points << 5; }
    }
}

/// The dev switches held back to a frame: `(frame, item, row, points)`.
static DEV_LATER: std::sync::Mutex<Vec<(i32, usize, Option<i32>, Option<i32>)>> = std::sync::Mutex::new(Vec::new());

/// The held-back dev switches whose frame came (each play frame).
pub fn dev_tick(game: &mut GameState) {
    let mut later = DEV_LATER.lock().unwrap_or_else(|e| e.into_inner());
    later.retain(|&(at, item, row, points)| {
        if game.frame < at { return true; }
        if let Some(b) = row.and_then(|_| game.upgrades.rows.get_mut(item)) { *b = row.unwrap_or(0) as u8; }
        if let Some(x) = points.and_then(|_| game.upgrades.xp.get_mut(item)) { *x = points.unwrap_or(0) << 5; }
        false
    });
}
