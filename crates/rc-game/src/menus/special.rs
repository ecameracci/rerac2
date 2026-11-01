//! The pause's Special row and its pages (L00 addresses; every level's overlay has the same code and tables).
//!
//! **The row**: the pause list (`0x345740`) enables Special by `0x359888`, which always says yes.
//!
//! **The Specials Menu** (page `0x284698`, descriptor `0x1aaef8`: update `0x3499c8`, draw `0x349ab0`, enter
//! `0x349b10`): the in-game list (`+0x3c160`) in its mode 2 (`0x438538(2)`: the rows `0x28a170`, the frame's keyframe 2,
//! "Specials Menu" `0x3094`), each row with its own icon (`0x4389b0`: the ids at `0x28ee10`, 5 meaning `0xea9e` frame
//! 5); the selected row's description (`0x438480`: `0x28edb0`) in the page's rect 1, and "△ Exit" right-aligned at
//! (455, 256). The rows: Cheats, Skill Points, Movies, then Short Cuts, Random Stuff and First Person Mode, which outside
//! the challenge mode (`0x1a7a12`) read "????" with no page and no description (✕ sound 5). Up / Down wrap (sound 3),
//! ✕ opens the row's page (`0x350030` kind 3), △ back to the pause list, Start / Select close the pause.
//!
//! **Skill Points** (page `0x284950`, `0x1aaf58`: update `0x34a3b8`, draw `0x34a578`): a cursor over the 30 points
//! (`gp−0x3274`, back to 0 when the page is left; Up / Down wrap, sound 3). The page's sprites 0xd7..0xda at (256, 208),
//! "Skill Points" centred at (185, 27), "%d of %d Skill Points" (`0x2dc5`: the count `0x327df0` of 30) at (146, 375),
//! "△ Exit" centred at (429, 375), a rule at (350, 218)..(486, 220), the point's planet (`0x2a153c + 6·k`, if any)
//! centred at (416, 224); the list (font 2, centred at 256 from y 274, 20 apart): the five points around the cursor
//! (`0x2a1538 + 6·k`), orange when earned, fading out from the middle (alpha 0x10 / 0x50 / 0x70 / 0x50 / 0x10), the
//! middle one boxed (`0x349898`); the scroll bar left of it (`0x3493c8`: a box of lines and sprite 0x4a twice,
//! transposed, as the arrows, white while ↑ / ↓ are held, else pulsing over 25 frames). An earned point shows its
//! description (`0x2a153a + 6·k`) in the page's rect 1 and its picture (the panel `0x284ad8`: `BONUS.WAD`'s table
//! `0x14d920`, row k) in rect 2.
//!
//! **Cheats** (page `0x284820`, `0x1aaf28`: update `0x349d80`, draw `0x349fc0`): the cheat rows `0x1ac090` (`{text,
//! cheat, needed}`: the skill points needed, 0x5a for point 0 itself, 100 for a row never shown), the cheat bytes
//! `0x1a7a48` (the save's global chunk 7). A cursor (`gp−0x3280`, back to 0 when the page is left) skipping the hidden
//! rows (Up / Down wrap, sound 3); ✕ toggles an unlocked cheat (sound 4) or refuses (sound 5). The window sprites
//! 0x8b..0x8d at (256, 208) at keyframe 0.84, "Cheats" centred at (256, 65), "✕ Toggle" at (256, 321), "△ Exit" at
//! (256, 346); the rows from y 112, 19 apart, in font 2: the cheat's name at x 43 and "on" / "off" centred at 405, or
//! "????" and what unlocks it ("That's impossible!", or "<n> Skill Points") in grey; the selected row orange.
//!
//! Positions are the NTSC ones (`0x1b2680` 0, `0x1b2684` 1.0: the Progressive Scan mode `0x1a7b98` stretches and
//! moves some, `0x42cbd8`).
//!
//! Not ported: the pages behind Movies (`0x284b40`), Short Cuts (`0x284e70`), Random Stuff (`0x285008`) and First
//! Person Mode (`0x285718`): ✕ on them plays sound 5 as on a locked row; the enter's tally of the weapons had and
//! upgraded (`0x349b10` into `gp−0x4860..−0x4858`, read by those pages). The cheats' effects, the port having none of
//! the systems they switch: Ratchet's, Clank's, the actors' and the enemies' big heads (bytes 1, 3, 0, 7; Ratchet's is
//! hero posture list 24's scale, crate::hero::posture), the mirrored levels (4), the left-handed Ratchet (5), the
//! health's invincibility at the maximum (6: crate::moby::classes::nanotech), the sheep cars (8), and the bodies — the
//! tux, the funny side, Beach Boy, Snow Dude (10, 9, 11, 2: the armour rows 5..8 of `0x2fe988`, put on at once by
//! `0x2fecd0` when toggled; the engine attaches armour row 0 only).

use super::{gui, options, pad, Assets, Ctx, Menus, Out, Page, Quad, Tex, View};
use crate::save::{SaveData, Scope};
use rc_formats::disc::toc::PictureTable;

/// The challenge mode (`0x1a7a12`).
const CHALLENGE: u32 = 0x1a7a12;
/// The skill points' bytes (chunk 8) and the cheats' (chunk 7).
const POINTS: u32 = 0x1a7a68;
const CHEAT_BYTES: u32 = 0x1a7a48;

/// The Specials Menu's rows (`0x28a170`): their texts, those of the challenge mode's rows when it is off ("????",
/// `0x349b10`), the descriptions (`0x28edb0`) and the icons (`0x28ee10`: `0xea9e` frame 5 for 5).
const ROWS: [u32; 6] = [0x2ca8, 0x2ca6, 0x3095, 0x3098, 0x30a2, 0x30d5];
const LOCKED: u32 = 0x2c56;
const DESCRIPTIONS: [u32; 6] = [0x2cb2, 0x2cb0, 0x3096, 0x3099, 0x30a3, 0x30a4];
pub const ICONS: [(u16, u16); 6] = [(0xea83, 0), (0xea68, 0), (0xea9e, 5), (0xea9f, 0), (0xea65, 0), (0xea74, 0)];
/// The rows past Movies need the challenge mode.
const FIRST_LOCKED: usize = 3;
/// "Specials Menu"; "△ Exit"'s place (`0x1abd68`, `0x1abd6c`); the list frame's keyframe in mode 2.
const TITLE: u32 = 0x3094;
const EXIT_AT: [f32; 2] = [455.0, 256.0];
const KEYFRAME: f32 = 2.0;

/// The Specials Menu's row `row`: its page, if the port has it (Cheats, Skill Points).
fn row_page(row: usize) -> Option<Page> {
    match row {
        0 => Some(Page::Cheats),
        1 => Some(Page::SkillPoints),
        _ => None,
    }
}

fn challenge(save: &SaveData) -> bool { save.u8(Scope::Global, CHALLENGE) != 0 }

/// `0x438480` in mode 2: the row's description, none for a locked row.
pub fn description(row: usize, save: &SaveData) -> u32 {
    if FIRST_LOCKED <= row && !challenge(save) { 0 } else { DESCRIPTIONS.get(row).copied().unwrap_or(0) }
}

/// The Skill Points page's layout (`0x34a578`, the page `0x284950`'s rects 1 and 2) and the points' texts.
pub mod skill {
    /// The skill points (`0x34a3b8` wraps at 0x1d).
    pub const COUNT: usize = 30;
    /// The page's sprites, at (256, 208), keyframe 0.
    pub const AT: [f32; 2] = [256.0, 208.0];
    pub const SPRITES: [(u32, u32); 4] = [(0xd7, 0x6044_2d00), (0xd8, 0x6024_1700), (0xd9, 0x55f0_c070), (0xda, 0x55f0_c070)];
    /// "Skill Points" (font 1, centred), "%d of %d Skill Points" (font 1, left), "△ Exit" (font 1, centred).
    pub const TITLE: (u32, [f32; 2]) = (0x2ca6, [185.0, 27.0]);
    pub const COUNT_LINE: (u32, [f32; 2]) = (0x2dc5, [146.0, 375.0]);
    pub const EXIT_AT: [f32; 2] = [429.0, 375.0];
    /// The rule `{x0, y0, x1, y1}` and the planet's place (font 1, centred).
    pub const RULE: [i32; 4] = [350, 218, 486, 220];
    pub const PLANET_AT: [f32; 2] = [416.0, 224.0];
    /// The list: five rows centred on x 256 from y 274, 20 apart, in font 2; their alphas; earned / not.
    pub const LIST_Y: f32 = 274.0;
    pub const LIST_STEP: f32 = 20.0;
    pub const ALPHAS: [u32; 5] = [0x10, 0x50, 0x70, 0x50, 0x10];
    pub const EARNED: u32 = 0x29a1ff;
    pub const NOT_EARNED: u32 = 0xf0f0f0;
    /// The middle row's box (`0x349898`): its bars' rows (y 312..314 and 333..335, the sides 313..334), 5 past the
    /// text's half width.
    pub const BOX_Y: [i32; 4] = [312, 314, 333, 335];
    pub const BOX_PAD: i32 = 5;
    /// The scroll bar (`0x3493c8`): the arrows' sprite and places (each transposed, the second also mirrored), their
    /// pulse (`0x31dc30(0x60442d00, 0x55f0c070, 25)`), the box's lines `{x0, y0, x1, y1}`.
    pub const ARROW: u32 = 0x4a;
    pub const ARROWS_AT: [[f32; 2]; 2] = [[58.0, 457.0], [44.0, 187.0]];
    pub const ARROW_PULSE: (u32, u32, u32) = (0x6044_2d00, 0x55f0_c070, 25);
    pub const LINES: [[i32; 4]; 4] = [[39, 278, 41, 369], [63, 278, 65, 369], [39, 277, 65, 279], [39, 368, 65, 370]];
    pub const LINE: u32 = 0x55f0_c070;
    /// The description panel's box (rect 1, as the pages' descriptions take it) and the picture's (rect 2,
    /// `{182, 152, 310, 178}` centred).
    pub const DESCRIPTION: [i32; 4] = [354, 60, 130, 134];
    pub const PICTURE: [f32; 4] = [27.0, 63.0, 310.0, 178.0];
    /// Per point (`0x2a1538`, six bytes each): its name, its description and its planet's name (−1 none).
    pub const POINTS: [(u32, u32, i32); COUNT] = [
        (0x2cbe, 0x2cdc, 0xb50), (0x2cbf, 0x2cdd, 0xb47), (0x2cc0, 0x2cde, 0xb49), (0x2cc1, 0x2cdf, 0xb50), (0x2cc2, 0x2ce0, 0xb52),
        (0x2cc3, 0x2ce1, 0xb4d), (0x2cc4, 0x2ce2, 0xb58), (0x2cc5, 0x2ce3, 0xb57), (0x2cc6, 0x2ce4, 0xb5b), (0x2cc7, 0x2ce5, 0xb51),
        (0x2cc8, 0x2ce7, 0xb51), (0x2cc9, 0x2ce8, 0xb55), (0x2cca, 0x2ce9, 0xb4e), (0x2ccb, 0x2cea, 0xb46), (0x2ccc, 0x2ceb, 0xb47),
        (0x2ccd, 0x2cec, 0xb46), (0x2cce, 0x2ced, 0xb4c), (0x2ccf, 0x2cee, 0xb4b), (0x2cd0, 0x2cef, 0xb50), (0x2cd1, 0x2cf0, 0xb47),
        (0x2cd2, 0x2cf1, 0xb50), (0x2cd3, 0x2cf2, 0xb48), (0x2cd4, 0x2cf3, 0xb51), (0x2cd5, 0x2cf4, -1), (0x2cd6, 0x2cf5, 0xb4e),
        (0x2cd7, 0x2cf6, 0xb47), (0x2cd8, 0x2cf7, -1), (0x2cd9, 0x2cf8, 0xb57), (0x2cda, 0x2cf9, 0xb48), (0x2cdb, 0x2cfa, -1),
    ];
}

/// The Cheats page's layout (`0x349fc0`) and rows (`0x1ac090`).
pub mod cheats {
    /// The window's sprites at (256, 208), keyframe 0.84 (`0x3f570a3d`).
    pub const AT: [f32; 2] = [256.0, 208.0];
    pub const SPRITES: [(u32, u32); 3] = [(0x8b, 0x6044_2d00), (0x8c, 0x55f0_c070), (0x8d, 0x55f0_c070)];
    pub const KEYFRAME: f32 = 0.84;
    /// "Cheats", "✕ Toggle", "△ Exit": font 1, centred on x 256.
    pub const TITLE: (u32, f32) = (0x2ca8, 65.0);
    pub const TOGGLE: (u32, f32) = (0x2be4, 321.0);
    pub const EXIT: (u32, f32) = (0x2be5, 346.0);
    /// The rows: from y 112, 19 apart; the label left at x 43, the state centred on 405 (font 2).
    pub const FIRST_Y: f32 = 112.0;
    pub const STEP: f32 = 19.0;
    pub const LABEL_X: f32 = 43.0;
    pub const STATE_X: f32 = 405.0;
    pub const SELECTED: u32 = 0x7029_a1ff;
    pub const UNLOCKED: u32 = 0x80f0_f0f0;
    pub const LOCKED: u32 = 0x8080_8080;
    /// "on", "off", "????", "That's impossible!" (point 0's name), "Skill Points".
    pub const ON: u32 = 0x2c5c;
    pub const OFF: u32 = 0x2c5d;
    pub const UNKNOWN: u32 = 0x2c56;
    pub const IMPOSSIBLE: u32 = 0x2cbe;
    pub const POINTS: u32 = 0x2ca6;
    /// The skill points a cheat needs: point 0 itself, or a row never shown.
    pub const NEEDS_POINT_0: u8 = 0x5a;
    pub const HIDDEN: u8 = 100;

    /// A row: its text, its byte (`0x1a7a48 + cheat`), the skill points it needs.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Cheat {
        pub text: u32,
        pub cheat: u8,
        pub needs: u8,
    }

    const fn c(text: u32, cheat: u8, needs: u8) -> Cheat { Cheat { text, cheat, needs } }

    /// `0x1ac090`: big head, tux, Clank's noggin, mirrored levels, actors' craniums, funny side, enemies' domes, Beach
    /// Boy, Snow Dude; never shown: sheep cars, left-handed, invincibility at full health.
    pub const ROWS: [Cheat; 12] = [
        c(0x2cfc, 1, 5), c(0x30c8, 10, 7), c(0x2cfd, 3, 10), c(0x2d00, 4, 12), c(0x2cfe, 0, 15), c(0x30c7, 9, 20),
        c(0x2cff, 7, 25), c(0x30fc, 11, 30), c(0x30fb, 2, NEEDS_POINT_0), c(0x30c9, 8, HIDDEN), c(0x2d01, 5, HIDDEN), c(0x2d02, 6, HIDDEN),
    ];
}

/// The save's skill point bytes.
fn points(save: &SaveData) -> [u8; crate::skill_points::LEN] {
    let mut b = [0; crate::skill_points::LEN];
    if let Some(s) = save.bytes(Scope::Global, POINTS, b.len()) { b.copy_from_slice(s); }
    b
}

fn earned(save: &SaveData, k: usize) -> bool { save.u8(Scope::Global, POINTS + k as u32) != 0 }

/// `0x34a3b8`'s panel text: an earned point's description.
pub fn skill_description(k: usize, save: &SaveData) -> u32 {
    if earned(save, k) { skill::POINTS.get(k).map_or(0, |p| p.1) } else { 0 }
}

/// `0x34a3b8`'s picture: an earned point's (row k).
pub fn skill_picture(k: usize, save: &SaveData) -> Option<(PictureTable, u16)> {
    (k < skill::COUNT && earned(save, k)).then_some((PictureTable::SkillPoints, k as u16))
}

/// A cheat row unlocked (`0x349d80`'s ✕): point 0 had for 0x5a, else that many points earned.
fn unlocked(c: cheats::Cheat, save: &SaveData) -> bool {
    if c.needs == cheats::NEEDS_POINT_0 { earned(save, 0) } else { c.needs as i32 <= crate::skill_points::count(&points(save)) }
}

/// `0x115da8` with `%d`s filled in order (and `%s`, `%%`).
fn fill(fmt: &[u8], ints: &[i32], strs: &[&[u8]]) -> Vec<u8> {
    let (mut out, mut i, mut d, mut s) = (Vec::new(), 0, 0, 0);
    while i < fmt.len() {
        if fmt[i] == b'%' && i + 1 < fmt.len() {
            match fmt[i + 1] {
                b'd' => {
                    out.extend_from_slice(ints.get(d).copied().unwrap_or(0).to_string().as_bytes());
                    (d, i) = (d + 1, i + 2);
                    continue;
                }
                b's' => {
                    out.extend_from_slice(strs.get(s).copied().unwrap_or(b""));
                    (s, i) = (s + 1, i + 2);
                    continue;
                }
                b'%' => {
                    out.push(b'%');
                    i += 2;
                    continue;
                }
                _ => {}
            }
        }
        out.push(fmt[i]);
        i += 1;
    }
    out
}

/// Text in font 1 (`0x2ea570` centred) or font 2 (`0x2ea608` centred, `0x2e9f68` left) at `(x, y)`.
fn text(out: &mut Vec<Quad>, a: &Assets, font2: bool, t: &[u8], [x, y]: [f32; 2], centred: bool, colour: u32) {
    let (glyphs, slot) = if font2 { (&a.glyphs_2, 2) } else { (&a.glyphs, 1) };
    let x = if centred { x - (gui::text_width(glyphs, t, 1.0) >> 1) as f32 } else { x };
    gui::text(out, glyphs, slot, t, x, y, 1.0, gui::rgba(colour));
}

impl Menus {
    /// The Specials Menu (`0x3499c8`: △ back, Start / Select close, the list's update, ✕ the row's page).
    pub(super) fn special_tick(&mut self, keys: u16, cx: &Ctx, out: &mut Vec<Out>) {
        let n = ROWS.len();
        if keys & pad::TRIANGLE != 0 {
            self.back(out);
        } else if keys & (pad::START | pad::SELECT) != 0 {
            self.close(out);
        } else if keys & (pad::UP | pad::DOWN) != 0 {
            out.push(Out::Sound(3));
            self.special_row = if keys & pad::UP != 0 { (self.special_row + n - 1) % n } else { (self.special_row + 1) % n };
        } else if keys & pad::CROSS != 0 {
            let open = (self.special_row < FIRST_LOCKED || challenge(cx.save)).then(|| row_page(self.special_row)).flatten();
            match open {
                Some(p) => {
                    (self.skill_row, self.cheat_row) = (0, 0);
                    self.go(p, out);
                }
                None => out.push(Out::Sound(5)),
            }
        }
    }

    /// The Skill Points page (`0x34a3b8`): △ back, Start / Select close (the cursor back to 0); Up / Down wrap over
    /// the 30 (sound 3).
    pub(super) fn skill_points_tick(&mut self, keys: u16, out: &mut Vec<Out>) {
        let n = skill::COUNT;
        if keys & pad::TRIANGLE != 0 {
            self.skill_row = 0;
            return self.back(out);
        }
        if keys & (pad::START | pad::SELECT) != 0 {
            self.skill_row = 0;
            return self.close(out);
        }
        if keys & pad::UP != 0 {
            out.push(Out::Sound(3));
            self.skill_row = (self.skill_row + n - 1) % n;
        } else if keys & pad::DOWN != 0 {
            out.push(Out::Sound(3));
            self.skill_row = (self.skill_row + 1) % n;
        }
    }

    /// The Cheats page (`0x349d80`): △ back, Start / Select close (the cursor back to 0); Up / Down wrap past the
    /// hidden rows (sound 3); ✕ toggles an unlocked cheat's byte (sound 4), else sound 5.
    pub(super) fn cheats_tick(&mut self, keys: u16, cx: &mut Ctx, out: &mut Vec<Out>) {
        use cheats::*;
        let n = ROWS.len();
        if keys & pad::TRIANGLE != 0 {
            self.cheat_row = 0;
            return self.back(out);
        }
        if keys & (pad::START | pad::SELECT) != 0 {
            self.cheat_row = 0;
            return self.close(out);
        }
        if keys & (pad::UP | pad::DOWN) != 0 {
            out.push(Out::Sound(3));
            let step = if keys & pad::UP != 0 { n - 1 } else { 1 };
            for _ in 0..n {
                self.cheat_row = (self.cheat_row + step) % n;
                if ROWS[self.cheat_row].needs != HIDDEN { break; }
            }
        }
        if keys & pad::CROSS == 0 { return; }
        let c = ROWS[self.cheat_row];
        if unlocked(c, cx.save) {
            out.push(Out::Sound(4));
            let at = CHEAT_BYTES + c.cheat as u32;
            let on = cx.save.u8(Scope::Global, at) != 0;
            cx.save.set_u8(Scope::Global, at, !on as u8);
        } else {
            out.push(Out::Sound(5));
        }
    }

    /// The Specials Menu's draw (`0x349ab0`: the in-game list, then "△ Exit").
    pub(super) fn draw_special(&self, out: &mut Vec<Quad>, a: &Assets, v: &View, icon: &dyn Fn(u16, u16) -> Option<(Tex, [f32; 2])>) {
        let open = challenge(v.save);
        let rows: Vec<(u32, (u16, u16))> =
            ROWS.iter().zip(ICONS).enumerate().map(|(i, (&t, ic))| (if i < FIRST_LOCKED || open { t } else { LOCKED }, ic)).collect();
        self.draw_list(out, a, icon, TITLE, KEYFRAME, &rows, self.special_row, EXIT_AT);
    }

    /// The Skill Points page's draw (`0x34a578`, see the module docs), then its panels: the description and the
    /// picture.
    pub(super) fn draw_skill_points(&self, out: &mut Vec<Quad>, a: &Assets, v: &View) {
        use skill::*;
        for (id, c) in SPRITES {
            if let Some(s) = a.sprites.get(&id) { gui::sprite(out, s, AT, 0.0, [1.0, 1.0], gui::rgba(c)); }
        }
        let white = 0x80f0_f0f0;
        text(out, a, false, a.strings.get(TITLE.0), TITLE.1, true, white);
        let count = crate::skill_points::count(&points(v.save));
        text(out, a, false, &fill(a.strings.get(COUNT_LINE.0), &[count, COUNT as i32], &[]), COUNT_LINE.1, false, white);
        text(out, a, false, a.strings.get(options::EXIT_TEXT), EXIT_AT, true, white);
        let [x0, y0, x1, y1] = RULE;
        gui::rect(out, y0, y1, x0, x1, gui::rgba(LINE));
        let k = self.skill_row.min(COUNT - 1);
        if let Ok(planet) = u32::try_from(POINTS[k].2) { text(out, a, false, a.strings.get(planet), PLANET_AT, true, white); }
        self.draw_skill_scroll(out, a);
        for (i, alpha) in ALPHAS.into_iter().enumerate() {
            let p = (k + COUNT + i - 2) % COUNT;
            let name = a.strings.get(POINTS[p].0);
            let colour = (if earned(v.save, p) { EARNED } else { NOT_EARNED }) | alpha << 24;
            if i == 2 {
                // `0x349898`: the box around the middle row.
                let half = (gui::text_width(&a.glyphs_2, name, 1.0) >> 1) + BOX_PAD;
                let (l, r, c) = (AT[0] as i32 - half, AT[0] as i32 + half, gui::rgba(colour));
                let [t0, t1, b0, b1] = BOX_Y;
                gui::rect(out, t0, t1, l - 2, r + 4, c);
                gui::rect(out, b0, b1, l - 2, r + 4, c);
                gui::rect(out, t0 + 1, b0 + 1, l - 2, l, c);
                gui::rect(out, t0 + 1, b0 + 1, r + 2, r + 4, c);
            }
            text(out, a, true, name, [AT[0], LIST_Y + LIST_STEP * i as f32], true, colour);
        }
        self.draw_description(out, a, DESCRIPTION);
        self.draw_picture(out, v, PICTURE);
    }

    /// `0x3493c8`: the skill list's scroll bar: the arrows (sprite 0x4a with its x and z swapped, the second also
    /// negated; white while ↑ / ↓ are held), then the box's four lines.
    fn draw_skill_scroll(&self, out: &mut Vec<Quad>, a: &Assets) {
        use skill::*;
        let (from, to, frames) = ARROW_PULSE;
        let k = self.pulse % (2 * frames);
        let t = if k < frames { k as f32 / frames as f32 } else { 1.0 - (k - frames) as f32 / frames as f32 };
        let pulse = gui::lerp_colour(t, to, from);
        if let Some(s) = a.sprites.get(&ARROW) {
            for (i, (held, sign)) in [(pad::UP, 1.0), (pad::DOWN, -1.0)].into_iter().enumerate() {
                let colour = if self.held & held != 0 { gui::rgba(0x80f0_f0f0) } else { pulse };
                gui::sprite_mat(out, None, s, ARROWS_AT[i], 0.0, [[0.0, sign], [sign, 0.0]], colour, gui::MENU_TAN);
            }
        }
        for [x0, y0, x1, y1] in LINES { gui::rect(out, y0, y1, x0, x1, gui::rgba(LINE)); }
    }

    /// The Cheats page's draw (`0x349fc0`, see the module docs).
    pub(super) fn draw_cheats(&self, out: &mut Vec<Quad>, a: &Assets, v: &View) {
        use cheats::*;
        for (id, c) in SPRITES {
            if let Some(s) = a.sprites.get(&id) { gui::sprite(out, s, AT, KEYFRAME, [1.0, 1.0], gui::rgba(c)); }
        }
        for (t, y) in [TITLE, TOGGLE, EXIT] { text(out, a, false, a.strings.get(t), [AT[0], y], true, UNLOCKED); }
        let mut y = FIRST_Y;
        for (i, c) in ROWS.iter().enumerate() {
            if c.needs == HIDDEN { continue; }
            let selected = i == self.cheat_row;
            let (label, state, colour) = if unlocked(*c, v.save) {
                let on = v.save.u8(Scope::Global, CHEAT_BYTES + c.cheat as u32) != 0;
                (a.strings.get(c.text).to_vec(), a.strings.get(if on { ON } else { OFF }).to_vec(), UNLOCKED)
            } else if c.needs == NEEDS_POINT_0 {
                (a.strings.get(UNKNOWN).to_vec(), fill(b"%s", &[], &[a.strings.get(IMPOSSIBLE)]), LOCKED)
            } else {
                (a.strings.get(UNKNOWN).to_vec(), fill(b"%d %s", &[c.needs as i32], &[a.strings.get(POINTS)]), LOCKED)
            };
            let colour = if selected { SELECTED } else { colour };
            text(out, a, true, &label, [LABEL_X, y], false, colour);
            text(out, a, true, &state, [STATE_X, y], true, colour);
            y += STEP;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_in_order() {
        assert_eq!(fill(b"%d of %d \x0cSkill Points\x08", &[3, 30], &[]), b"3 of 30 \x0cSkill Points\x08".to_vec());
        assert_eq!(fill(b"%d %s", &[25], &[b"Skill Points"]), b"25 Skill Points".to_vec());
    }

    #[test]
    fn hidden_cheats_and_locks() {
        assert_eq!(cheats::ROWS.iter().filter(|c| c.needs == cheats::HIDDEN).count(), 3);
        assert_eq!(row_page(0), Some(Page::Cheats));
        assert_eq!(row_page(1), Some(Page::SkillPoints));
        assert_eq!(row_page(2), None);
    }
}
