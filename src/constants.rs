/*
    Constants

    Mainly related to where example inputs and expected
    outputs are stored. Also where result files are stored.
*/

// File extensions
pub const EXAMPLE_IN_EXT: &str = "_in.json";
pub const EXAMPLE_EXPECT_EXT: &str = "_expect.json";

// Folders where examples are stored, optionally with expected output
pub const EX_DIR_GENERATED: &str = "examples/generated";
pub const EX_DIR_HANDWRITTEN: &str = "examples/handwritten";
pub const ALL_EXAMPLE_DIRS: &[&str] = &[
    EX_DIR_GENERATED,
    EX_DIR_HANDWRITTEN,
    "examples/regex/complement",
    "examples/regex/date",
    "examples/regex/loop",
    "examples/regex/state_graph_easy",
    "examples/regex/state_graph_hard",
];

// Output directory used by run_all
pub const RESULTS_DIR: &str = "results";
