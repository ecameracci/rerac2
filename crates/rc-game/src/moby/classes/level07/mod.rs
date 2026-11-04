//! Level 0x07's own classes (Siberius; L07 addresses, the engine functions they call named by their L07 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.

pub mod icicle;
pub mod prowler;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        icicle::CLASS => Some(icicle::update_12a8),
        prowler::CLASS => Some(prowler::update_0d29),
        _ => None,
    }
}
