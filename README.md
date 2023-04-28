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
- Both algorithms, and our implementations of all the baseline approaches, exhibit correct output on all unit tests and benchmarks used in the evaluation.

The plots included in Figure 5, as well as the Qty column of Figure 4, were generated using a Google spreadsheet,
and cannot be produced automatically.
For completeness, we include the spreadsheet under `spreadsheets/`, as an `.xlsx` file,
and we also include a link to a Google sheet template
which can be used to regenerate the plots.
We also include instructions for how to copy the raw results into the spreadsheet; if this is done, the plots should update based on the new data.
However, to save time, we recommend focusing the evaluation on running the command-line tools to reproduce the raw results (in CSV form)
as well as checking the output to see that it agrees with what is reported in the paper.
To aid in this process, we also provide the raw results that are used in the spreadsheet, under `results/`, for comparison.

## Running the Docker container

The artifact is provided as a docker container, available on Zenodo:

- [Zenodo Link](https://zenodo.org/record/7877718)

- DOI: 10.5281/zenodo.7877718

## Quick-Start Guide (smoke-test phase)

All commands to build and run the artifact are through `cargo`, the Rust package manager and build system.
Once you have obtained the artifact, we recommend the following two tests to see that everything is working correctly.

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

This command runs all of the algorithms on the input files: `examples/handwritten/15_in.json` and `examples/handwritten/15_expect.json`.
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

You can also run with `-h` to view command help, description, and options:
```
cargo run --release --bin run_compare -- -h
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
- `src/` contains the complete source code. Within this, `src/algorithm` is the most important folder, containing the implementation of each of the algorithms that are compared. `src/bin` contains the available binaries, including `run_compare` and `run_example` described above. Finally, the rest of the files implement a common interface and shared underlying directed graph structure which are used to ensure a fair comparison between the algorithms.
- `tests/` contains unit tests, in addition to those already included inline in the source code.
- `examples/` contains all of the (handwritten and automatically generated) benchmarks used for evaluation of the paper as well as for correctness.
- `Cargo.toml` and `Cargo.lock` are standard files provided by Rust packages: they document the required Rust package dependencies to run the code, and are used by `cargo` during compilation and linking. `rustfmt.toml` is a configuration file for the code formatter `cargo fmt`.
- `results/` stores the experimental results generated by our scripts.
- `spreadsheets/` contains the Google sheet (downloaded as a `.xlsx` file) that can be used to produce the plots in Figure 5.
- Finally, `scripts` contains miscellaneous scripts, and `regex-smt-benchmarks` contains SMT regular expression benchmarks as a submodule: it is not a necessary dependency, but it includes the source `.smt` files that were used to generate some of the benchmarks in `examples/` for the sake of completeness.

## Reproducing the experimental results (full-review phase)

This section contains detailed instructions to reproduce the experimental results described in Section 4.

### Important caveats

The plots in Figure 5 were generated using the help of a Google spreadsheet.
Rather than reproducing the plots directly, we suggest commands here to run and reproduce the raw results, which include, for each benchmark, the running time of each of the five algorithms on the benchmark.
For the sake of completeness, we also include instructions on how to use a template of the Google spreadsheet if it is desired to regenerate the plots.

In addition, many of our experiments take a long time to reproduce in their entirety.
For this reason, throughout this section we suggest commands that can be used to run a subset of the benchmarks if desired for a quicker evaluation.

### 1. Lines of Source Code (Fig. 4, left)

On the left, Figure 4 summarizes all the source code used for the evaluation.
To normalize number of lines of code, all code is automatically formatted with `cargo fmt`.
You can verify this by running `cargo fmt` followed by `git status` to see that no source files change.
Then you can run the script
```
./scripts/update_loc.sh
```
to output the number of lines in the source code to the file `loc.txt` (or run `rm loc.txt` first, and it will be recreated with the above command).
We recommend checking the rows to see that they agree.
Each row in the table corresponds to one or more lines in `loc.txt`.
The "Common framework" row includes the sum of `interface.rs`, `search.rs`, `driver.rs`, `example.rs`, and `graph.rs`.
The "Other" row includes `mod.rs`, `lib.rs`, and `debug_counter.rs`.
The "Experimental Scripts" row includes `run_compare.rs`, `run_example.rs`, `run_all.rs`, and `example_gen.rs`.
The "Additional Unit Tests" row includes `constants.rs` and `test_examples.rs`.
The "Euler Tour Trees" row includes the implementation of Euler Forests and associated AVL trees in `hashy.rs`, `euler_forest.rs`, and `avl_forest.rs`.
All the other rows correspond directly to a single source file.
A minor discrepancy is that "Additional Unit Tests" row now has 2 additional LoC (800 instead of 798) due to a minor update since the experiments were first run;
we will update this row in the final version of the paper.

### 2. Generating the benchmarks used (Fig. 4, right)

The Basic and Random benchmarks are automatically generated. To generate these, run
```
cargo run --release --bin example_gen
```

which should take 1-2 minutes to complete, and generates the examples in `examples/generated` and `examples/random`.
Note that this runs with a fixed set of 10 different random seeds, to ensure reproducibility.
For completeness sake, it is also possible to edit the random seeds in the `main()` function at the bottom of `src/bin/example_gen.rs`; if you were to do this, this would change only the benchmarks generated in `examples/random`.

#### A note on "trivial" benchmarks:

In the table in Figure 4, on the right, it is not yet possible at this stage to reproduce the "Qty" column. This is because this column reports the number of benchmarks *after* filtering for trivially small examples.
By this we mean any example for which all algorithms take less than 10 milliseconds to complete, as described on Page 15, third-to-last paragraph:

> From both the Q2 and Q3 benchmarks, we filter out any benchmark which takes under 10 milliseconds for all of the algorithms to solve (including Naive), and we use a 60 second timeout.

The only rows for which we report the total number of examples prior to filtering is the Regex examples,
as these are not our benchmarks created for this paper, so we wanted to contextualize any modifications to the original number.
For benchmarks that were created for this paper, we excluded the trivial examples entirely as they should not be considered valid benchmarks for performance comparison.
Also note that `examples/handwritten` are also trivial, so are only used for unit tests
and are not used for the evaluation
(the "Handwritten" row of the table refers to handwritten regex examples in `examples/regex`).

#### Suggested sanity checks on the benchmarks

Since the Qty column is not yet reproducible,
instead, at this stage, we suggest verifying that some smallest examples will be excluded.
For example, running
```
cargo run --release --bin run_compare -- examples/generated/line_3
```
you should verify that all five algorithms take 0s on this example.
For this reason, the example `line_3` (and others like it) was filtered out of the final results, and not reported in the table.
Here is a complete list of excluded `generated` benchmarks:
```
examples/generated/line_3
examples/generated/line_100
examples/generated/unkrevcompleteacyclic_3
examples/generated/revcompleteacyclic_3
examples/generated/unkcompleteacyclic_10
examples/generated/unkline_3
examples/generated/complete_10
examples/generated/unkloop_30
examples/generated/reverseunkloop_10
examples/generated/unkline_100
examples/generated/loop_10
examples/generated/bipartite_10_10
examples/generated/unkbipartite_10_10
examples/generated/reverseline_3
examples/generated/unkbipartite_3_10
examples/generated/unkbipartite_3_3
examples/generated/bipartite_100_10
examples/generated/reverseunkline_100
examples/generated/reverseloop_30
examples/generated/reverseline_100
examples/generated/unkrevcompleteacyclic_10
examples/generated/line_10
examples/generated/unkbipartite_10_100
examples/generated/completeacyclic_10
examples/generated/unkbipartite_30_30
examples/generated/reverseunkloop_3
examples/generated/unkline_30
examples/generated/revcompleteacyclic_10
examples/generated/reverseunkline_10
examples/generated/bipartite_30_30
examples/generated/reverseline_30
examples/generated/unkbipartite_10_3
examples/generated/bipartite_3_3
examples/generated/unkcomplete_10
examples/generated/bipartite_10_100
examples/generated/reverseloop_10
examples/generated/unkloop_100
examples/generated/unkbipartite_10_30
examples/generated/bipartite_3_10
examples/generated/loop_100
examples/generated/unkcomplete_3
examples/generated/reverseloop_100
examples/generated/completeacyclic_3
examples/generated/loop_30
examples/generated/reverseunkloop_100
examples/generated/bipartite_10_30
examples/generated/reverseunkline_3
examples/generated/unkcompleteacyclic_30
examples/generated/complete_30
examples/generated/unkloop_10
examples/generated/complete_3
examples/generated/reverseunkloop_30
examples/generated/reverseloop_3
examples/generated/unkcomplete_30
examples/generated/bipartite_10_3
examples/generated/bipartite_30_10
examples/generated/unkcompleteacyclic_3
examples/generated/reverseline_10
examples/generated/completeacyclic_30
examples/generated/unkbipartite_30_10
examples/generated/loop_3
examples/generated/unkbipartite_100_10
examples/generated/unkline_10
examples/generated/revcompleteacyclic_30
examples/generated/unkloop_3
examples/generated/reverseunkline_30
examples/generated/unkrevcompleteacyclic_30
examples/generated/line_30
```

Additionally, you can check that the `examples` folder contains all of the benchmarks described on Page 15, prior to filtering, as follows:

- running `ls examples/generated`, you should see files for line graphs (`line`), cycle graphs (`loop`), complete graphs (`complete`), bipartite graphs (`bipartite`), in multiples of 3 and 10 (as described in the file name) up to size 100000.

- running `ls examples/random`, you should see files for sparse graphs and dense graphs in multiples of 3 and 10 (as described in the file name) up to size 30000.
The remaining numbers in the file name describe the generation parameters:
for example, sparse benchmarks have a number of outgoing edges from each state (1, 2, 3, or 10) and a random seed (between 1 and 10).
Dense benchmarks have a probability of generating each edge (independently), as a percentage, and a random seed (between 1 and 10).

- Lastly, running `ls examples/regex` you should see several folders. Each folder contains the benchmarks described in the table.
Running
```
find examples/regex/regexlib -name "*_in.json" | wc -l
```
you should see 2061 benchmarks (first row).
`state_graph_easy` and `state_graph_hard` are the "Additional" benchmarks;
there are 19 benchmarks prior to filtering for the 11 described in the table.
(As in the prior cases as described earlier, we don't report the pre-filtering number here as these were created for the present paper.)
Running
```
find examples/regex -name "*_in.json" | wc -l
```
the output is 2150 examples total:
2061 regexlib, 19 additional state graph benchmarks (prior to filtering), and
70 handwritten benchmarks.

### 3. Reproducing the raw results (Fig. 5, top right)

Now that you have generated the benchmarks, the raw results are produced using the
`run_all` binary.
This runs all algorithms on all known examples.
First, though, we need to run the following command to avoid a stack overflow on some larger examples for some of the algorithms:
```
ulimit -s hard
```

then running `ulimit -s`, the output should be 65520 or larger (TODO).

#### Quick version

For a quicker version, we recommend running all the results *excluding* the Naive and Simple algorithms, which are generally quite slow.
The `-e` flag can be used to exclude these. Run the following command:
```
time cargo run --release --bin run_all -- -e n s
```

#### Full results

To generate the full experimental results,

TODO

### 4. Generating the plots (Fig. 5) (not recommended)

Finally, we include the spreadsheet which can be used to replicate the plots.
This step is not recommended unless the reviewer is adept with Google sheets.

Please make sure to access the following link from a **private** browser or while not logged in, so as not to reveal your identity:

[Google Sheets Link](https://docs.google.com/spreadsheets/d/18niEXFnQnqUQGxJ5eOrYE701LyjJVlTVqKezSNir8HI/edit?usp=sharing)

The spreadsheet is also packaged with the artifact, in the `spreadsheet/` folder, as a `.xlsx` file.
Once you have opened the spreadsheet, make a copy of spreadsheet for editing.
Then, you can paste in the raw experimental data under the `raw` tab.
The data in the other tabs should automatically update and generate the plot images.

In the `raw` tab, the "include?" column includes a Boolean which computes whether or not the benchmark should be included in the plots: it is included if it is not a handwritten (unit test) example, *and* if it is not trivial, i.e. at least one algorithm takes at least 10 milliseconds to solve the benchmark.
The total number of rows in this sheet is what is used to determine the number of included benchmarks in the Qty column in Fig. 4 (right).

In the tab `plots2`, the cell O20 controls which plot is generated:
it should be toggled between `generated`, `random` and `regex`.
This will cause the data to update to include only the selected benchmark category.

## "Reusable" badge

In addition to the functional and available badges, we request to be considered for the "reusable" badge. Here are some notes that support reusability.

### Reusing the code

Our library is free and open source under an MIT license, and is also available through the Rust package registry system (`crates.io`), so it can easily be integrated into any Rust project by simply running `cargo add gid` or adding the line `gid = "0.1.1"` to `Cargo.toml`.
Alternatively, the artifact can be downloaded and run as a binary in a project written in any other language.
It can be run outside of a Docker container in any system with Rust installed,
and it is also available as a GitHub repository.

For Rust projects, the library provides an interface to use GIDs directly as a data structure in future applications. For example, one can import `JumpStateGraph` into another code base and use the interface in `interface.rs` to update the data structure. The methods include, for example, `.add_transition()`, `.mark_closed()`, and `.is_dead()` to update and query the data structure.

For non-Rust projects, it is possible to use the data structure in a more black-box fashion by constructing input files in the required `.json` format. This approach is described earlier using the `run_compare` and `run_example` binaries.

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
We have documented the binaries including `-h` options to display help.
We have also ensured that there are extensive unit tests to demonstrate minimal uses for the tool, see for example `tests/test_examples.rs`.

### New implementations of existing work

Besides GIDs, this project also provides new implementations of some existing work in online graph algorithms, which are reusable on their own.
In particular, the BFGT algorithm for strong connected component maintenance in `bfgt.rs`, and the Euler Forest data structure in `euler_forest.rs` are implemented as separate modules that can be exported.
To our knowledge, this is the first implementation of BFGT for strong connected component maintenance, and the Euler Forest is the first implementation in Rust of Henzinger and King's Euler Tour Trees data structure.

## Authors

For the artifact: Caleb Stanford `cdstanford` `ucdavis` `edu`

For the paper: Caleb Stanford and Margus Veanes `margus` `microsoft` `com`

## Issues and feedback

Finally, if there are any issues installing, running, or reusing the artifact, we are always open to receiving questions and feedback.

- For the CAV artifact evaluation, please let us know in the comments if you encounter any issues.

- In the future, the GitHub repository will be open for comments, and the best place to submit any issues, bug reports, feature requests, or comments would be through GitHub. Please open an issue or pull request at [https://github.com/cdstanford/gid](https://github.com/cdstanford/gid) and we will be happy to assist you.
