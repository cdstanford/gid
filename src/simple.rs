/*
    Slightly less naive implemenation implementing the
    state graph interface, this time using Union Find.

    Parallels state_graph.cpp in the Z3 code base -- found in
        z3/src/util/state_graph.h
    with one improvement: we used LinkedList instead of HashSet for
    storing edges, because it allows merging edge sets in O(1).
*/

// use partitions::PartitionVec;
// use std::collections::LinkedList;
// use super::interface::{StateGraph, Status};

// TODO
