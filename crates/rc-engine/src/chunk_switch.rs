//! The level chunk switch (L02 `0x3082e0`, L01 `0x2f9bd8`): a level in chunks (Maktar's resort and arena) swaps its
//! world — the tfrags and the collision — for chunk k's, keeping its mobys. The game asks for it through
//! rc_game::game::GameState::switch_chunk; after the tick [`apply`] reads chunk k's world from the level file
//! (crate::level_wad::chunk_world), puts it in the level, rebuilds the tfrag draw (crate::tfrag_render, its LOD state
//! with it) and, through [`ChunkGeneration`], the world lights' lists.
//!
//! The switched-in tfrags take their occlusion words as the level load gave the first chunk's (`0x3080a0`,
//! `rc_formats::occlusion::chunk_tfrag_bits`): each tfrag's header `+0x3a` indexes the table `0x1caa94` the occlusion
//! load `0x305380` filled from every chunk's tfrag mappings (1259 on Maktar).
//!
//! Then [`hide_other_chunk`] (`0x3081c8(k)`, which the level load `0x308740` runs too on a level in chunks) hides what
//! lies outside chunk k's region and shows what lies in it: the ties, the shrubs and the mobys.
//!
//! The switch's sound part — the reverb off (`0x368320`), every sound stopped (`0x368398`), the music stopped
//! (`0x335998`), chunk k's sound bank (bank k + 1, `0x334fe0`) and the caller's music track (`0x335630`) — runs in the
//! tick where the game asks for the switch (rc_game::moby::Ctx::switch_chunk → rc_game::audio::chunk).
//!
//! Not ported: the other resets (`0x383fd8`, `0x373960`).

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::render::storage::ShaderBuffer;
use crate::tfrag_render::TfragMaterial;

/// +1 per chunk switch: what was built from the level's tfrags rebuilds when it changes.
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkGeneration(pub u32);

pub struct ChunkSwitchPlugin;

impl Plugin for ChunkSwitchPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkGeneration>().add_systems(Update, (apply, hide_other_chunk).chain());
    }
}

/// The switch asked for this tick (module docs).
#[allow(clippy::too_many_arguments)]
fn apply(
    mut commands: Commands,
    play: Option<ResMut<crate::play::Play>>,
    mut level: ResMut<crate::Level>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<TfragMaterial>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    drawn: Query<Entity, (With<MeshMaterial3d<TfragMaterial>>, Without<RenderLayers>)>,
    mut generation: ResMut<ChunkGeneration>,
) {
    let Some(mut p) = play else { return };
    let Some(k) = p.game.chunk_request.take() else { return };
    let index = crate::level_load::level_index();
    let world = usize::try_from(k).ok().map(|k| crate::level_wad::chunk_world(&crate::level_load::extracted_root(), index, k));
    let (tfrags, collision) = match world {
        Some(Ok(Some(w))) => w,
        Some(Ok(None)) | None => { eprintln!("chunk switch: level {index} has no chunk {k}"); return; }
        Some(Err(e)) => { eprintln!("chunk switch: level {index} chunk {k} not read ({e:#})"); return; }
    };
    let lv = &mut level.0;
    let headers: Vec<_> = tfrags.iter().map(|t| t.header).collect();
    lv.occlusion.objects.tfrag = rc_formats::occlusion::chunk_tfrag_bits(lv.occlusion.objects.chunk_table.as_deref(), &headers);
    lv.tfrags = tfrags;
    lv.collision = Some(collision);
    for e in &drawn { commands.entity(e).despawn(); }
    let s = crate::tfrag_render::spawn_tfrags(&mut commands, lv, &mut meshes, &mut images, &mut materials, &mut buffers);
    generation.0 += 1;
    let mapped = lv.occlusion.objects.tfrag.iter().filter(|b| **b != rc_formats::occlusion::OcclBits::ALWAYS).count();
    println!("chunk switch: level {index} chunk {k}: {} tfrags ({mapped} occlusion-mapped), {} LOD-0 triangles", s.tfrags, s.triangles);
}

/// `0x3081c8(k)` (L02): on a level in chunks, whatever lies outside chunk k's region (`0x308138`, the region of a point
/// by the variant planes: rc_game::level::LevelData::in_variant) is hidden and whatever lies in it shown again:
/// * the ties, by their world sphere's centre (runtime tie `+0x00`): `+0x1e` bit 2, rc_game::moby::MobyWorld::hidden_ties
///   (the bit the level's objects hide ties with; crate::play's move_instances takes it to the renderer);
/// * the shrubs, by their sphere's centre (runtime shrub `+0x00`, rc_formats::shrub_light::instance_centre): `+0x16`
///   bit 0, their meshes (crate::shrub_render::ShrubSway::set_hidden) and their billboards
///   (crate::shrub_billboard::BillboardRecords);
/// * every moby with a class (`+0xaa` ≠ 0, so not Ratchet), by its position `+0x10`: mode `+0x34` bit 0x80, which
///   MobyProc skips (`0x31d4cc`: mode & 0x81).
///
/// It runs once the level's mobys are in place (the load's call: `0x308740` at `0x3093b8`, only with variant planes)
/// and after each switch ([`apply`]).
#[allow(clippy::too_many_arguments)]
fn hide_other_chunk(
    mut commands: Commands,
    play: Option<ResMut<crate::play::Play>>,
    level: Res<crate::Level>,
    shrubs: Option<ResMut<crate::shrub_render::ShrubSway>>,
    billboards: Option<ResMut<crate::shrub_billboard::BillboardRecords>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    level_generation: Res<crate::level_switch::LevelGeneration>,
    generation: Res<ChunkGeneration>,
    mut done: Local<Option<(u64, u32)>>,
) {
    let Some(mut p) = play else { return };
    let key = (level_generation.0, generation.0);
    if *done == Some(key) { return; }
    let k = p.game.variant;
    let Some(m) = p.mobys.as_mut() else { return };
    *done = Some(key);
    let planes = &m.level;
    if planes.variant_planes.is_empty() { return; }
    let w = &mut m.world;
    let mut ties = 0;
    for (i, s) in w.tie_spheres.iter().enumerate() {
        if planes.in_variant([s[0], s[1], s[2]], k) {
            w.hidden_ties.remove(&i);
        } else {
            w.hidden_ties.insert(i);
            ties += 1;
        }
    }
    let sh = &level.0.shrubs;
    let hidden_shrubs: Vec<bool> = sh
        .instances
        .iter()
        .enumerate()
        .map(|(ii, inst)| {
            let c = match sh.class_of.get(ii) {
                Some(Some(ci)) => rc_formats::shrub_light::instance_centre(&sh.classes[*ci].class, inst),
                _ => inst.matrix[3],
            };
            !planes.in_variant([c[0], c[1], c[2]], k)
        })
        .collect();
    if let Some(mut s) = shrubs {
        for (ii, &h) in hidden_shrubs.iter().enumerate() { s.set_hidden(&mut commands, ii, h); }
    }
    if let Some(mut b) = billboards { b.set_hidden(&hidden_shrubs, &mut buffers); }
    let mut mobys = 0;
    for mb in w.mobys.iter_mut().filter(|mb| mb.o_class != 0) {
        if planes.in_variant(mb.pos, k) {
            mb.mode &= !0x80;
        } else {
            mb.mode |= 0x80;
            mobys += 1;
        }
    }
    println!(
        "chunk {k}: hidden {ties} of {} ties, {} of {} shrubs, {mobys} mobys (the other chunk's)",
        w.tie_spheres.len(), hidden_shrubs.iter().filter(|h| **h).count(), hidden_shrubs.len()
    );
}
