/*
    debug_counter

    A simple thin wrapper for incrementing counters for the purposes of
    debugging and logging statistics.

    Features:
    - No overhead in release mode (struct is empty and methods are no-ops)
    - Interior mutability: allows counter to be incremented from an immutable
      reference. This is necessary to transparently log information when the
      counter is used as a field in the struct and the struct method only has
      &self, not &mut self as an argument.
      (Using Cell is enough for this simple use case, don't need RefCell.)
*/

#[cfg(debug_assertions)]
use std::cell::Cell;

#[derive(Debug, Default)]
pub struct DebugCounter {
    #[cfg(debug_assertions)]
    val: Cell<usize>,
}
impl DebugCounter {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(debug_assertions)]
    pub fn inc(&self) {
        // There is a nice nightly feature that does this a bit cleaner:
        // self.val.update(|x| x + 1);
        self.val.set(self.val.get() + 1);
    }
    #[cfg(not(debug_assertions))]
    pub fn inc(&self) {}

    #[cfg(debug_assertions)]
    pub fn get(&self) -> usize {
        self.val.get()
    }
    #[cfg(not(debug_assertions))]
    pub fn get(&self) -> usize {
        panic!("Tried to get debug counter in release mode.")
    }
}
