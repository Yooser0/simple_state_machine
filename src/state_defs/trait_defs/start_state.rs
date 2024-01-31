use crate::state_defs::trait_defs::state::State;

/// The first state the state master will use for entrance into the state
/// machine.
pub trait StartState: State {
    fn new() -> Self;
}
