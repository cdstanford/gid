/*
    Interface (Trait) for tracking live/dead states
    in an abstract transition system.

    Different data structure and algorithms for the same problem can then
    implement this interface.
*/

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    Dead,
    Unknown,
    Unvisited,
}
pub trait StateGraph {
    /*
        Functions that need to be implemented.

        For convenience, these versions are unchecked:
        - add_transition_unchecked can assume both its vertices are distinct and that the
          source is Unvisited.
        - mark_done_unchecked can assume that its vertex is Unvisited.
    */
    // Add a new transition to the graph from an Unvisited state to any state.
    // (If the vertex doesn't exist yet, create it and mark it unvisited.)
    fn add_transition_unchecked(&mut self, v1: usize, v2: usize);
    // Mark an unvisited state as visited.
    fn mark_done_unchecked(&mut self, v: usize);
    // Return whether v is Unvisited, or v is Visited but there is a path from
    // v to an Unvisited state (Unknown), or there is no such path (Dead).
    fn get_status(&self, v: usize) -> Status;

    // Safe, checked versions
    // These should always be used over the unchecked versions for code that
    // uses the interface naively.
    // TODO: If this becomes a real library, return Result instead of panicking.
    fn add_transition(&mut self, v1: usize, v2: usize) {
        assert!(self.get_status(v1) == Status::Unvisited);
        if v1 != v2 {
            self.add_transition_unchecked(v1, v2);
        }
    }
    fn mark_done(&mut self, v: usize) {
        assert!(self.get_status(v) == Status::Unvisited);
        self.mark_done_unchecked(v);
    }
}
