# Incremental Dead State Detection in Logarithmic Time

Welcome! This repository contains the artifact for the paper *Incremental Dead State Detection in Logarithmic Time,* to appear at CAV 2023.
See `cav23.pdf` for the version of the paper at submission time.
The artifact can also be viewed online [here](https://github.com/cdstanford/gid).

## Overview

The artifact provides a data structure, called guided incremental digraphs (GIDs), implemented as an open-source library in Rust.
The data structure identifies live and dead states in an abstract transition system.

The repository implements, documents, and compares five competing solutions to solve the core problem of live and dead state detection (described below):
- our amortized logarithmic-time per update algorithm (Algorithm 2, `log`)
- an even faster lazy algorithm (Algorithm 3, `jump`), and
- three baseline approaches for comparison: `bfgt` (the state of the art), `naive`, and `simple`.

### Problem statement

More precisely: given as input a list of graph updates like
```
AddEdge(1, 2)
AddEdge(2, 3)
Close(2)
AddEdge(3, 4)
Close(4)
Terminal(2)
```
the problem is to produce as output a list of *live* and *dead* states (vertices) in the transition system.
The meaning of this terminology is as follows:
- In the input, some states are labeled as "closed" meaning they are considered fully explored: no further outgoing edges will be added.
- In the input, some state are labeled as "terminal", meaning they are accepting states.
- In the output, a "live" state is one for which there is a path to a terminal state.
- In the output, a "dead" state is one for which all reachable states are closed and not terminal. In other words, a dead state is one that is known to not be live no matter what future edges are added to the transition system.

In this example, by the above definitions, `1` and `2` should be live and `4` should be dead.

## List of claims supported

The artifact supports the experimental results section of the paper (Section 4), summarized in Figures 4 and 5. In particular, the artifact supports the following claims:

- Algorithm 3 (`jump`) shows significant improvements over the state-of-the-art, solving more benchmarks in a smaller amount of time across basic GIDs, random GIDs, and regex GIDs.
- Algorithm 2 (`log`) also shows state-of-the-art performance, similar to BFGT on basic and regex GIDs and significantly better on random GIDs.
- In particular, Algorithm 3 exhibits up to two orders of magnitude speedup over BFGT for larger GIDs – we measured speedups of 110x to 530x for GIDs in the top five size buckets (GIDs of size nearest to 100K, 200K, 500K, 1M, and 2M).
- Both algorithms, and our implementations of all the baseline approaches, exhibit correct output on all unit tests and benchmarks.

The plots included in Figure 5 were generated using a Google spreadsheet, and cannot be produced automatically from the artifact. However, we also include all information needed to reproduce these figures, if necessary.
(TODO)

## Quick-Start Guide (smoke-test phase)

The artifact is provided as a docker container
and is assigned a DOI on Zenodo.
(TODO)

All commands are run through `cargo`, which is the Rust build system.
We recommend the following two tests to see that everything is working correctly.

### 1. Run the unit tests

Once you have navigated to the `gid` directory, execute the following command:
```
cargo test
```

This should execute all the unit tests and should take under a minute. The output should include the following two important lines:
```
test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
...
test result: ok. 56 passed; 0 failed; 8 ignored; 0 measured; 0 filtered out; finished in 24.44s
```

You will notice that there are 8 tests which are "ignored"; this is because they take an excessively long time to run.
(If you like, you can run these with `cargo test --release -- --ignored`, which should take about 10-15 minutes to complete.)

### 2. Run one of the examples

You can also run the artifact directly on one of the examples
using the provided `run_compare` and `run_example` binaries.
To run the example `examples/handwritten/15`:
```
cargo run --release --bin run_compare -- examples/handwritten/15
```

This should produce the following output:
```
===== examples/handwritten/15 =====
Example size: 8, timeout: 10s
naive: time 0ms
simple: time 0ms
bfgt: time 0ms
log: time 0ms
jump: time 0ms
```

You can also run a specific algorithm with the binary `run_example` instead of `run_compare`, and adding `-a n`, `-a s`, `-a b`, `-a l`, or `-a j`. For example, to run the `log` algorithm:
```
cargo run --release --bin run_example -- examples/handwritten/15 -a l
```

Which should produce the following output:
```
===== examples/handwritten/15 =====
Running algorithm 'log' with timeout 10s...
=== Output ===
ExampleOutput { live: [0, 1], dead: [4], unknown: [2], open: [3] }
=== Result ===
Stastics: time 0ms
Output is correct.
```

The input and output syntax for this example can be viewed in
`examples/handwritten/15_in.json` and `examples/handwritten/15_expect.json`, respectively.
The output `_expect.json` is used for testing correctness.
The syntax used is reproduced below:

Input:
```
[
    {"Add": [2,3]},
    {"Close": 2},
    {"Live": 1},
    {"Add": [0,1]},
    {"Add": [1,2]},
    {"Close": 1},
    {"Add": [3,4]},
    {"Close": 4}
]
```

Output:
```
{
    "live": [0, 1],
    "dead": [4],
    "unknown": [2],
    "open": [3]
}
```

## Directory structure

The repository contains the following files and directories:
- First and foremost, `src/` contains the source code. Within this, `src/algorithm` is the most important folder, containing the implementation of each of the algorithms that are compared. `src/bin` contains the available binaries, including `run_compare` and `run_example` described above. Finally, the rest of the files implement a common interface and shared underlying directed graph structure which are used to ensure a fair comparison between the algorithms.
- `tests/` contains unit tests, in addition to those already included inline in the source code.
- `examples/` contains all of the (handwritten and automatically generated) benchmarks used for evaluation of the paper as well as for correctness.
- `Cargo.toml` and `Cargo.lock` are standard files provided by Rust packages: they document the required Rust package dependencies to run the code, and are used by `cargo` during compilation and linking. `rustfmt.toml` is a configuration file for the code formatter `cargo fmt`.
- Finally, `scripts` contains miscellaneous scripts, and `regex-smt-benchmarks` contains SMT regular expression benchmarks as a submodule: it is not a necessary dependency, but it includes the source `.smt` files that were used to generate some of the benchmarks in `examples/` for the sake of completeness.

## Reproducing the experimental results (full-review phase)

TODO

## "Reusable" badge

In addition to the functional and available badges, we request to be considered for the "reusable" badge. Here are some notes that support reusability.

### Reusing the code

Our library is free and open source under an MIT license, and is also available through the Rust package registry system (`crates.io`), so it can easily be integrated into any Rust project by simply running `cargo add gid` or adding the line `gid = "0.1.1"` to `Cargo.toml`.
Alternatively, the artifact can be downloaded and run as a binary in a project written in any other language.
It can be run outside of a Docker container in any system with Rust installed,
and it is also available as a GitHub repository.

For Rust projects, the library provides an interface to use GIDs directly as a data structure in future applications. For example, one can import `JumpStateGraph` into another code base and use the interface in `interface.rs` to update the data structure. The methods include, for example, `.add_transition()`, `.mark_closed()`, and `.is_dead()` to update and query the data structure.

For non-Rust projects, it is possible to use the data structure in a more black-box fashion by constructing input files in the required `.json` format. This approach is described earlier using the `run_compare` and `run_example` binaries.

### New implementations of existing work

Besides GIDs, the project also provides new implementations of some existing work in online graph algorithms.
In particular, the BFGT algorithm for strong connected component maintenance in `bfgt.rs`, and the Euler Forest data structure in `euler_forest.rs` provided as reusable data structures on their own.
To our knowledge, this is the first implementation of BFGT for strong connected component maintenance, and the Euler Forest is the first implementation in Rust of Henzinger and King's Euler Tour Trees data structure.

### Extending the code

If one wishes to test out their own algorithm for incremental dead state detection, all that must be done is to provide another file in `src/algorithms` and implement the `StateGraph` interface: e.g.
```rust
struct MyNewStateGraph { /* ... */ }
impl StateGraph for MyNewStateGraph { /* ... */ }
```

One of the existing files -- such as `naive.rs` -- can be used as a comparison to see the necessary implementation work.
To compare the state graph with the existing algorithms, one can edit the code in `driver.rs` to add the new state graph to the list of compared algorithms.

Another possibility for extension is to extend the graph interface with additional types of updates. The file `interface.rs` -- a Rust "Trait" -- defines the core algorithmic interface. Additional methods can be added to the interface, and then each algorithm must implement the new methods in the respective file `src/algorithms/*.rs`.

### Documentation and tests

We have taken care to document the source code, where especially helpful, and cleaned up and removed TODOs and old features.
We have also ensured that there are extensive unit tests to demonstrate minimal uses for the tool, see for example `tests/test_examples.rs`.

## Authors

For the artifact: Caleb Stanford `cdstanford` `ucdavis` `edu`

For the paper: Caleb Stanford and Margus Veanes `margus` `microsoft` `com`

## Issues and feedback

Finally, if there are any issues installing, running, or reusing the artifact, we are always open to receiving questions and feedback.

- For the CAV artifact evaluation, please let us know in the comments if you encounter any issues.

- In the future, the GitHub repository will be open for comments, and the best place to submit any issues, bug reports, feature requests, or comments would be through GitHub. Please open an issue or pull request at [https://github.com/cdstanford/gid](https://github.com/cdstanford/gid) and we will be happy to assist you.
