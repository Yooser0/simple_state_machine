use crate::state_defs::trait_defs::start_state::StartState;
use crate::state_defs::trait_defs::state::State;

/// A struct that holds a single state as an entrance into the state machine.
pub struct StateMaster<'a> {
    state: Box<dyn State + 'a>,
}

impl<'a> StateMaster<'a> {
    pub fn new<S: StartState + 'a>() -> StateMaster<'a> {
        StateMaster {
            state: Box::new(S::new()),
        }
    }
}

impl<'a> StateMaster<'a> {
    /// The function to be called to start the state machine, running through
    /// each state and finishing on an end state (a state with no next call).
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use state_machine::*;
    ///
    /// struct EntrancePoint {
    ///     // Data members here
    /// }
    ///
    /// impl State for EntrancePoint {
    ///     fn run(&mut self) {
    ///         // Run code here
    ///         todo!();
    ///     }
    ///     fn next(self: Box<Self>) -> Option<Box<dyn State>> {
    ///         // Next state code here
    ///         todo!();
    ///     }
    /// }
    ///
    /// impl StartState for EntrancePoint {
    ///     fn new() -> EntrancePoint {
    ///         // Initialization here
    ///         todo!();
    ///     }
    /// }
    ///
    /// // Additional state definitions other than `EntrancePoint` here
    ///
    /// StateMaster::new::<EntrancePoint>().start();
    /// ```
    pub fn start(mut self) {
        loop {
            self.state.run();
            self.state = match self.state.next() {
                Some(next) => next,
                None => {
                    return;
                }
            };
        }
    }
}
