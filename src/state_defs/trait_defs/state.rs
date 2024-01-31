/// The subunit for the state machine.
pub trait State {
    /// Runs code for this current state and updates necessary substates for use
    // in the `next` function.
    fn run(&mut self);
    /// Uses this state's current substates to determine the next state to
    /// traverse to, or returns itself.
    fn next(self: Box<Self>) -> Option<Box<dyn State>>;
}
