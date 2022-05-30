/*
    Constants

    Mainly related to where example inputs and expected
    outputs are stored. Also where result files are stored.
*/

// File extensions
pub const EXAMPLE_IN_EXT: &str = "_in.json";
pub const EXAMPLE_EXPECT_EXT: &str = "_expect.json";

// Output directory used by run_all
pub const RESULTS_DIR: &str = "results";

// Parameters for unit tests
#[cfg(debug_assertions)]
pub const UNIT_TEST_TIMEOUT_SECS: u64 = 30;
#[cfg(not(debug_assertions))]
pub const UNIT_TEST_TIMEOUT_SECS: u64 = 300;

// Main/special directories where examples are stored,
// optionally with expected output
pub const EX_DIR_GENERATED: &str = "examples/generated";
pub const EX_DIR_RANDOM: &str = "examples/random";
pub const EX_DIR_HANDWRITTEN: &str = "examples/handwritten";

// All other directories (no expected output)
pub const EX_DIR_REGEX_COMP: &str = "examples/regex/complement";
pub const EX_DIR_REGEX_DATE: &str = "examples/regex/date";
pub const EX_DIR_REGEX_LOOP: &str = "examples/regex/loop";
pub const EX_DIR_REGEX_SGEASY: &str = "examples/regex/state_graph_easy";
pub const EX_DIR_REGEX_SGHARD: &str = "examples/regex/state_graph_hard";
pub const EX_DIR_REGEX_COUNT: &str = "examples/regex/counting";
pub const EX_DIR_REGEX_BLOWUP: &str = "examples/regex/det_blowup";
pub const EX_DIR_REGEX_INTER: &str = "examples/regex/intersection";
pub const EX_DIR_REGEX_PASSW: &str = "examples/regex/password";
pub const EX_DIR_RLIB_M1: &str = "examples/regex/regexlib/RegexMembership/sat";
pub const EX_DIR_RLIB_INTER1: &str =
    "examples/regex/regexlib/RegexIntersection/sat";
pub const EX_DIR_RLIB_INTER2: &str =
    "examples/regex/regexlib/RegexIntersection/unsat";
pub const EX_DIR_RLIB_SUB1: &str = "examples/regex/regexlib/RegexSubset/sat";
pub const EX_DIR_RLIB_SUB2: &str = "examples/regex/regexlib/RegexSubset/unsat";

// All the above directories in a list
pub const ALL_EXAMPLE_DIRS: &[&str] = &[
    EX_DIR_GENERATED,
    EX_DIR_HANDWRITTEN,
    EX_DIR_RANDOM,
    EX_DIR_REGEX_COMP,
    EX_DIR_REGEX_DATE,
    EX_DIR_REGEX_LOOP,
    EX_DIR_REGEX_SGEASY,
    EX_DIR_REGEX_SGHARD,
    EX_DIR_REGEX_COUNT,
    EX_DIR_REGEX_BLOWUP,
    EX_DIR_REGEX_INTER,
    EX_DIR_REGEX_PASSW,
    EX_DIR_RLIB_M1,
    EX_DIR_RLIB_INTER1,
    EX_DIR_RLIB_INTER2,
    EX_DIR_RLIB_SUB1,
    EX_DIR_RLIB_SUB2,
];

// Unit test to check that all directories exist (does not check vice versa)
#[test]
fn validate_example_dirs() {
    use std::path::Path;

    for dir in ALL_EXAMPLE_DIRS {
        println!("Checking path is directory: {}", dir);
        assert!(Path::new(dir).is_dir());
    }
}
