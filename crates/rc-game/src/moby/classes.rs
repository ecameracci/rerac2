//! The class updates the port has, by o_class: the level's update table (L00 `0x2a2e80`, records `{o_class,
//! update, 0x2a3978}` up to o_class −1, written to the slot table `0x1da240` by `0x32e1f8`) as Rust ports.

pub mod arena;
pub mod area_switch;
pub mod beast;
pub mod bolt;
pub mod bosses;
pub mod carriers;
pub mod chute;
pub mod crate_;
pub mod critter;
pub mod dropship;
pub mod dynamo;
pub mod electrolyzer;
pub mod flower;
pub mod flyer;
pub mod debris;
pub mod enemies;
pub mod explosion;
pub mod fighters;
pub mod flash;
pub mod flock;
pub mod gate;
pub mod gravity_bomb;
pub mod keeper;
pub mod lancer;
pub mod leaper;
pub mod oozla;
pub mod light;
pub mod nanotech;
pub mod pickup;
pub mod platinum;
pub mod death_fade;
pub mod props;
pub mod space;
pub mod spitter;
pub mod shot;
pub mod smash;
pub mod spawn_point;
pub mod spiderbot;
pub mod swingshot;
pub mod teleport;
pub mod tie_lights;
pub mod telepad;
pub mod maktar;
pub mod maktar_hints;
pub mod maktar_pads;
pub mod swamp_monster;
pub mod tractor;
pub mod vendor;
pub mod weather;
pub mod wrench;
pub mod endako_a;
pub mod endako_a_enemies;
pub mod endako_b;
pub mod endako_clank;
pub mod endako_c;
pub mod endako_crane;
pub mod endako_d;
pub mod endako_boss;

use super::UpdateFn;

/// The port of `o_class`'s update (`None`: not ported, the moby gets mode 2 like a class without one).
/// The ported classes without a model that are only created at run time (the space fights' laser, the teleport-in
/// carrier: its model-less update, `0x315530`'s table `0x1ddfa0`, is in 18 overlays from level 2 to 21, not Aranos's,
/// where none of its callers runs; the big explosion 0x0d4e, L02's record `0x2a69ec`), added to every level's class table
/// as the overlay's table holds them.
/// The Spiderbot 0x0b58 is listed for the levels whose class list lacks it (`0x315530` creates it without a model, hidden).
/// Endako's boss's shots (crate::moby::classes::endako_boss: the laser 0x09df, the tracer 0x0a72, the rocket 0x0b1b;
/// L03's update table `0x2a9604` holds them) are listed so too, for when the level's class list lacks their models.
pub const CREATED_LOGIC: &[i16] = &[crate::ship::LASER_CLASS, teleport::CLASS, explosion::CLASS, spiderbot::BOT_CLASS, endako_boss::BEAM, endako_boss::TRACER, endako_boss::ROCKET];

pub fn update_for(o_class: i16) -> Option<UpdateFn> {
    match o_class {
        0x47 => Some(wrench::update),
        0x5d2 => Some(lancer::update),
        gravity_bomb::HAND_CLASS => Some(gravity_bomb::update),
        gravity_bomb::BOMB_CLASS => Some(gravity_bomb::update_bomb),
        gravity_bomb::BLAST_CLASS => Some(gravity_bomb::update_blast),
        spiderbot::HAND_CLASS => Some(spiderbot::update_hand),
        spiderbot::BOT_CLASS => Some(spiderbot::update_bot),
        swingshot::HAND_CLASS => Some(swingshot::update),
        swingshot::HOOK_CLASS => Some(swingshot::update_hook),
        0xbfe => Some(shot::update),
        flash::CLASS | flash::BOMB_CLASS => Some(flash::update),
        500 | 501 | 505 | 511 => Some(crate_::update),
        13..=19 => Some(bolt::update),
        0x12f5 => Some(props::update_12f5),
        0x0c8b => Some(props::update_0c8b),
        0x0ac0 => Some(props::update_0ac0),
        0x099c => Some(enemies::update_099c),
        0x0b0a => Some(enemies::update_0b0a),
        0x0b0b => Some(enemies::update_0b0b),
        0x0b41 | 0x0b42 | 0x0b45 | 0x0b4d | 0x0b51 | 0x0b52 | 0x0b53 | 0x0c02 | 0x0c3f => Some(props::update_stub),
        0x0f6e => Some(props::update_0f6e),
        0x0ddf => Some(props::update_0ddf),
        0x0a38 => Some(carriers::update_0a38),
        0x09c7 => Some(carriers::update_09c7),
        0x0c66 => Some(carriers::update_0c66),
        0x0ac1 => Some(props::update_0ac1),
        0x0ac2 => Some(props::update_0ac2),
        0x0aca => Some(props::update_0aca),
        0x0adb => Some(props::update_0adb),
        0x0ac4 => Some(props::update_0ac4),
        0x1330 => Some(props::update_1330),
        0x0f13 => Some(props::update_0f13),
        0x0ac9 | 0x0c0a..=0x0c0d | 0x0e01 | 0x0e02 => Some(props::update_wreck),
        0x0ac3 => Some(props::update_0ac3),
        0x0ac5 => Some(props::update_0ac5),
        0x0ac6 => Some(props::update_0ac6),
        0x0c4c => Some(props::update_0c4c),
        0x0c63 => Some(props::update_0c63),
        0x0c5c => Some(props::update_0c5c),
        0x0cda => Some(props::update_0cda),
        0x0ce9 => Some(tie_lights::update),
        0x0340 => Some(props::update_0340),
        0x0ac8 => Some(props::update_0ac8),
        0x0d1e => Some(props::update_0d1e),
        0x12f9 => Some(props::update_12f9),
        0x12fa | 0x1143 => Some(props::update_breakable),
        arena::SPAWNER => Some(arena::update_spawner),
        arena::PLATE => Some(arena::update_plate),
        arena::COLUMN => Some(arena::update_column),
        arena::LIFT => Some(arena::update_lift),
        arena::CROWD => Some(arena::update_crowd),
        arena::PROPS => Some(arena::update_props),
        fighters::MASTER => Some(arena::update_master),
        fighters::C3E => Some(fighters::update_0c3e),
        fighters::C3B => Some(fighters::update_0c3b),
        fighters::C9CA => Some(fighters::update_09ca),
        fighters::B22 => Some(fighters::update_0b22),
        fighters::SHIELD => Some(fighters::update_shield),
        bosses::C9F8 => Some(bosses::update_09f8),
        bosses::CA44 => Some(bosses::update_0a44),
        bosses::SHOT => Some(bosses::update_shot),
        0x0e93 => Some(maktar::update_0e93),
        maktar::NODE => Some(maktar::update_0c82),
        0x0c81 => Some(maktar::update_0c81),
        0x0ab0 => Some(maktar::update_0ab0),
        maktar::BOMB => Some(maktar::update_09c8),
        0x0c1b => Some(maktar::update_0c1b),
        0x0aa6 => Some(maktar::update_0aa6),
        0x0aa4 => Some(maktar::update_0aa4),
        0x12cf => Some(maktar::update_12cf),
        0x1259 => Some(maktar::update_1259),
        0x0a9c => Some(maktar::update_0a9c),
        keeper::CLASS => Some(keeper::update),
        keeper::SHOT | keeper::ROCKET => Some(keeper::update_shot),
        0x132d => Some(props::update_132d),
        0x0919 => Some(props::update_0919),
        0x0c8c => Some(props::update_0c8c),
        0x0325 => Some(spawn_point::update),
        0x02a7 => Some(chute::update),
        0x0ac7 => Some(gate::update),
        0x0c22 => Some(props::update_0c22),
        0x0102 => Some(area_switch::update),
        0x08fb => Some(oozla::update_08fb),
        0x0f14 => Some(oozla::update_0f14),
        0x0a6a | 0x0a6b | 0x0aa0 | 0x0aa1 => Some(oozla::update_mover),
        0x0c58 => Some(oozla::update_0c58),
        0x0e89 => Some(oozla::update_0e89),
        0x0578 => Some(weather::update),
        teleport::CLASS => Some(teleport::update),
        critter::CLASS => Some(critter::update),
        dropship::CLASS => Some(dropship::update),
        explosion::CLASS => Some(explosion::update),
        c if flock::CLASSES.contains(&c) => Some(flock::update),
        0x0ca0 => Some(oozla::update_0ca0),
        0x046f => Some(oozla::update_046f),
        0x0a17 => Some(oozla::update_0a17),
        0x0a92 | 0x0a93 => Some(oozla::update_float),
        0x09a2 => Some(leaper::update),
        0x0b2b => Some(spitter::update),
        0x0a49 => Some(flyer::update),
        swamp_monster::CLASS => Some(swamp_monster::update),
        swamp_monster::TRANSPORTER => Some(swamp_monster::update_0e38),
        swamp_monster::BALL => Some(swamp_monster::update_ball),
        swamp_monster::REWARD => Some(swamp_monster::update_reward),
        0x09fc => Some(beast::update),
        0x0a53 => Some(oozla::update_0a53),
        0x0a56 => Some(oozla::update_0a56),
        0x0a8d => Some(flower::update_arm),
        0x0a8e => Some(flower::update_core),
        0x0b1a => Some(flower::update_0b1a),
        0x0cb5 => Some(oozla::update_0cb5),
        0x0936 | 0x08fa | 0x0a00 => Some(oozla::update_dynamo_platform),
        0x0aae => Some(oozla::update_0aae),
        0x0a1e => Some(oozla::update_0a1e),
        0x0a9d | 0x0aa7 => Some(oozla::update_gate),
        0x0a9e | 0x0a9f => Some(oozla::update_sent),
        0x0a95 => Some(oozla::update_0a95),
        0x0ace => Some(oozla::update_0ace),
        0x061c => Some(oozla::update_061c),
        0x0a7b => Some(oozla::update_0a7b),
        0x0f02 | 0x0f08 | 0x0f2e | 0x0f33 => Some(smash::update),
        0x0f32 => Some(smash::update_0f32),
        0x0af7 => Some(smash::update_0af7),
        0x10ef => Some(smash::update_10ef),
        0x10fa => Some(smash::update_10fa),
        0x12dc => Some(smash::update_12dc),
        0x0a26 => Some(smash::update_0a26),
        0x0a27 | 0x0a28 => Some(smash::update_plant_piece),
        0x0a29 => Some(smash::update_0a29),
        0x0fa0 => Some(oozla::update_0fa0),
        0x0ab3 => Some(oozla::update_0ab3),
        0x0b07 => Some(oozla::update_0b07),
        0x0b21 => Some(oozla::update_0b21),
        0x02e1 => Some(oozla::update_02e1),
        0x0c23 => Some(oozla::update_0c23),
        0x0af0 => Some(oozla::update_0af0),
        platinum::CLASS => Some(platinum::update),
        death_fade::CLASS => Some(death_fade::update),
        0x0825 => Some(dynamo::update),
        0x00bc => Some(tractor::update),
        0x0acb | 0x0acc | 0x0e2a => Some(tractor::update_box),
        0x0af3 => Some(maktar::update_0af3),
        0x0e9c => Some(maktar::update_0e9c),
        0x0c24 => Some(maktar_hints::update_0c24),
        maktar_pads::CRITTER => Some(maktar_pads::update_critter),
        maktar_pads::PAD => Some(maktar_pads::update_pad),
        0x0afc => Some(maktar::update_0afc),
        0x0b25 => Some(maktar::update_0b25),
        0x0aaf => Some(maktar::update_0aaf),
        0x0873 => Some(maktar::update_0873),
        0x0a24 => Some(maktar::update_0a24),
        0x0a55 => Some(maktar::update_0a55),
        electrolyzer::CLASS => Some(electrolyzer::update),
        0x0a4b => Some(maktar::update_0a4b),
        0x0adf => Some(maktar::update_0adf),
        0x0aab | 0x0aac => Some(maktar::update_0aab),
        0x0d0d => Some(maktar::update_0d0d),
        0x0c51 => Some(maktar::update_0c51),
        0x0bf6 => Some(maktar::update_0bf6),
        0x0c8f => Some(maktar::update_0c8f),
        0x0a57 => Some(maktar::update_0a57),
        telepad::CLASS => Some(telepad::update),
        crate::ship::LASER_CLASS => Some(crate::ship::update_laser),
        space::FIGHTER | space::GUNSHIP => Some(space::update_fighter),
        crate::ship::MISSION_CLASS => Some(crate::ship::update_mission),
        space::FIELD => Some(space::update_field),
        0x0953..=0x0956 | 0x0971 => Some(space::update_asteroid),
        crate::ship::MISSILE_CLASS => Some(crate::ship::update_missile),
        space::SHIELD_PICKUP | space::BOLT_PICKUP => Some(space::update_pickup),
        pickup::CLASS => Some(pickup::update),
        nanotech::CLASS => Some(nanotech::update),
        crate::vendor::CLASS => Some(vendor::update),
        // Endako (endakoA).
        0x09b2 => Some(endako_a::update_09b2),
        0x09a3 | 0x09a4 | 0x09a6 => Some(endako_a::update_barrier),
        0x0d40 => Some(endako_a::update_0d40),
        0x02f6 | endako_a::HOOK_NODE => Some(endako_a::update_swing_node),
        0x09b6 => Some(endako_a::update_09b6),
        0x0979 => Some(endako_a::update_0979),
        0x0994 => Some(endako_a::update_0994),
        0x09ff => Some(endako_a::update_09ff),
        0x09b1 => Some(endako_a::update_09b1),
        0x0985 => Some(endako_a::update_0985),
        0x0995 => Some(endako_a::update_0995),
        endako_a_enemies::CLASS => Some(endako_a_enemies::update),
        // Endako (endakoB).
        0x0c25 => Some(endako_b::update_0c25),
        0x0a08 => Some(endako_clank::update_0a08),
        endako_clank::CLANK_CLASS => Some(endako_clank::update_0057),
        0x0982 | 0x0983 | 0x0bf5 => Some(endako_b::update_lift),
        0x0e87 | 0x0961 => Some(endako_b::update_0e87),
        0x097a => Some(endako_b::update_097a),
        0x0ef9 => Some(endako_b::update_0ef9),
        0x099d => Some(endako_b::update_099d),
        0x094e => Some(endako_b::update_empty),
        0x0a0a => Some(endako_b::update_0a0a),
        0x0a0b => Some(endako_b::update_0a0b),
        0x11e7 => Some(endako_b::update_11e7),
        0x11c4 => Some(endako_b::update_11c4),
        0x12ea => Some(endako_b::update_12ea),
        // Endako (endakoC).
        endako_c::SENTRY => Some(endako_c::update_0ffd),
        endako_c::FLAMER => Some(endako_c::update_0ffc),
        endako_c::MINE => Some(endako_c::update_0b23),
        endako_crane::PANEL => Some(endako_crane::update_0949),
        endako_crane::CRANE => Some(endako_crane::update_094a),
        endako_crane::BOX | endako_crane::BOMB | endako_crane::BOX_094C => Some(endako_crane::update_box),
        endako_crane::WALL => Some(endako_crane::update_10f6),
        // Endako's boss.
        endako_boss::CLASS => Some(endako_boss::update),
        endako_boss::TRACER => Some(endako_boss::update_tracer),
        endako_boss::BEAM => Some(endako_boss::update_beam),
        // Endako (endakoD).
        0x0f37 => Some(endako_d::update_0f37),
        0x0f3e => Some(endako_d::update_0f3e),
        0x0f38 => Some(endako_d::update_0f38),
        endako_d::ROBOT_93C | endako_d::ROBOT_940 | endako_d::ROBOT_964 | endako_d::ROBOT_A59 => Some(endako_d::update_robot),
        endako_d::CAGE => Some(endako_d::update_cage),
        0x0972 => Some(endako_d::update_0972),
        endako_d::NETWORK => Some(endako_d::update_0f2d),
        _ => None,
    }
}

/// The Swingshot's targets for the hero's searches: the nodes (class 0x2f6, `0x2c4098`: shown ones, mode 0x1000), the
/// hook nodes (0x323, `0x2c3e30`) and the pull targets (0x99d, `0x2c4340`), with their pvars `+0x68`.
pub fn swing_nodes(w: &super::MobyWorld, class: i16) -> Vec<crate::hero::swing::Node> {
    w.mobys
        .iter()
        .enumerate()
        .filter(|(_, m)| m.o_class == class && m.state < 0x80 && (class != 0x2f6 || m.mode & 0x1000 != 0))
        .map(|(id, m)| crate::hero::swing::Node {
            id,
            pos: m.pos,
            busy: m.pv_i(0) != 0,
            radius: m.pv_f(4),
            kind: m.pv_f(8),
            range: m.pv_f(0x20),
            drawn: m.drawn != 0,
            hidden: m.mode & 1 != 0,
            cap: m.pv_f(0x10),
            spring: [m.pv_f(0x24), m.pv_f(0x28), m.pv_f(0x2c)],
            keep_yaw: m.pv_i(0x34) != 0,
            yaw: m.rot[2],
        })
        .collect()
}
