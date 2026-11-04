//! Level 0x07's own classes (Siberius; L07 addresses, the engine functions they call named by their L07 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own. Level 4's light 0x0cfd (crate::moby::classes::level04::misc::update_0cfd) is this level's
//! `0x3e3698`.

pub mod gunner;
pub mod icicle;
pub mod prowler;
pub mod swarm;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        icicle::CLASS => Some(icicle::update_12a8),
        prowler::CLASS => Some(prowler::update_0d29),
        gunner::CLASS => Some(gunner::update_0d2a),
        gunner::SHOT => Some(gunner::update_0b7f),
        swarm::NEST => Some(swarm::update_0d1c),
        swarm::FLYER => Some(swarm::update_0d1d),
        _ => None,
    }
}
