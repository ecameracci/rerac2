//! Ratchet's body (rc_game::cheats): the armour row the game asks for (`0x2fe988`: the worn armour `0x1a7a20`, or a
//! body cheat's row 5..8) put on class 0, his level class with no packets of its own. The level starts dressed in
//! row 0 (crate::level_wad); when the row asked for differs from the one on (the level entry's `0x1a7290 ≠ 0x1a7a20`
//! test, `0x2a3b88`; the Cheats page's toggle of a body and the vendor's armour, `0x2fecd0`), the row is read from
//! `G/ARMOR.WAD` and dressed at once: its mesh replaces the class's packets (`0x2ffd48`'s relocation), its textures
//! go to the class's texture slots (`0x1a72a0`'s TEX0s: here appended to the level's moby textures the first time a
//! row is worn on the level, its slots kept for the next time), and the class's draws are rebuilt
//! (crate::moby_render::MobyOcclusion::rebuild_class).
//!
//! Not ported: the shadow volume and the other passes that took the class's mesh at the level's load keep row 0's.

use bevy::prelude::*;
use std::collections::HashMap;

/// The row on class 0, and each worn row's texture slots (the class entry's 16), for this level.
#[derive(Resource, Default)]
pub(crate) struct Dressed {
    row: usize,
    slots: HashMap<usize, [u8; 16]>,
}

pub(crate) struct ArmourDressPlugin;

impl Plugin for ArmourDressPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Dressed>()
            .add_systems(Update, dress)
            .add_systems(crate::level_switch::LevelUnload, crate::level_switch::reset::<Dressed>);
    }
}

/// `0x2fecd0` when the row asked for is not the one on.
#[allow(clippy::too_many_arguments)]
fn dress(
    mut commands: Commands,
    play: Option<Res<crate::play::Play>>,
    level: Option<ResMut<crate::Level>>,
    occl: Option<ResMut<crate::moby_render::MobyOcclusion>>,
    mut dressed: ResMut<Dressed>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut metal: ResMut<Assets<crate::moby_render::MobyMetalMaterial>>,
) {
    let (Some(play), Some(mut level), Some(mut occl)) = (play, level, occl) else { return };
    let want = play.game.body_row();
    if want == dressed.row { return; }
    let lv = &mut level.0;
    let Some(ci) = lv.mobys.classes.iter().position(|c| c.o_class == 0) else { return };
    // Row 0's slots, as the level's load gave them.
    let worn = lv.mobys.classes[ci].entry.textures;
    let d = &mut *dressed;
    d.slots.entry(d.row).or_insert(worn);
    let armour = match crate::level_wad::read_armour(&crate::level_load::extracted_root(), want) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("armour: row {want} not read ({e:#}); Ratchet keeps row {}", d.row);
            d.row = want;
            return;
        }
    };
    let slots = match d.slots.get(&want) {
        Some(&s) => s,
        None => {
            // New slots past the level's moby textures (as the load's armour, wrench and gadgets took theirs).
            use rc_formats::texture::{LevelTexture, TextureSource, TextureTable};
            let base = lv.textures.iter().filter(|t| t.table == TextureTable::Moby).map(|t| t.index + 1).max().unwrap_or(0);
            let mut s = d.slots[&0];
            for (k, t) in armour.textures.into_iter().enumerate().take(16) {
                let index = base + k;
                let Ok(slot) = u8::try_from(index) else {
                    eprintln!("armour: row {want}'s textures past moby texture 255; Ratchet keeps row {}", d.row);
                    d.row = want;
                    return;
                };
                s[k] = slot;
                let entry = rc_formats::level::TextureEntry { data_offset: 0, width: 0, height: 0, ty: 0, palette: 0, mipmap: 0, pad: 0 };
                lv.textures.push(LevelTexture { table: TextureTable::Moby, index, source: TextureSource::Entry(entry), texture: t });
            }
            d.slots.insert(want, s);
            s
        }
    };
    let class = &mut lv.mobys.classes[ci];
    if let Err(e) = rc_formats::moby::attach_packets(&mut class.class, &armour.mesh) {
        eprintln!("armour: row {want}'s mesh ({e:#}); Ratchet keeps row {}", d.row);
        d.row = want;
        return;
    }
    class.entry.textures = slots;
    occl.rebuild_class(&mut commands, lv, ci, &mut meshes, &mut images, &mut metal);
    println!("armour: Ratchet dressed in row {want} (was {})", d.row);
    d.row = want;
}
