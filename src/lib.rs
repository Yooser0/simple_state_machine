/// Definitions for all state-like functionality.
pub mod state_defs {
    /// A module that holds the struct `StateMaster`. The state master is a
    /// struct that holds a single state as a gateway into the state machine.
    pub mod state_master;
    pub mod trait_defs {
        /// A module for the `StartState` trait. This is the first state the
        /// state master will use for entrance into the state machine.
        pub mod start_state;
        /// A module for the `State` trait. The state is the subunit for the
        /// state machine.
        pub mod state;
    }
}

pub use state_defs::{
    state_master::StateMaster,
    trait_defs::{start_state::StartState, state::State},
};
