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

/*
    ALL example directories where examples are stored.
    The unit tests below validate that these exist and the list is exhaustive.
*/

// Parent directories (only contain subdirs)
const EX_DIR: &str = "examples";
const EX_DIR_REGEX: &str = "examples/regex";
const EX_DIR_RLIB: &str = "examples/regex/regexlib";
const EX_DIR_RLIB_M: &str = "examples/regex/regexlib/RegexMembership";
const EX_DIR_RLIB_INTER: &str = "examples/regex/regexlib/RegexIntersection";
const EX_DIR_RLIB_SUB: &str = "examples/regex/regexlib/RegexSubset";
// Generated and handwritten examples (optionally with expected output)
pub const EX_DIR_GENERATED: &str = "examples/generated";
pub const EX_DIR_RANDOM: &str = "examples/random";
pub const EX_DIR_HANDWRITTEN: &str = "examples/handwritten";
// Regex examples (no expected output)
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
    EX_DIR,
    EX_DIR_REGEX,
    EX_DIR_RLIB,
    EX_DIR_RLIB_M,
    EX_DIR_RLIB_INTER,
    EX_DIR_RLIB_SUB,
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

/*
    Unit tests to check that all directories exist and vice versa
*/

#[test]
fn validate_example_dirs_exist() {
    use std::path::Path;

    for dir in ALL_EXAMPLE_DIRS {
        println!("Checking path is directory: {}", dir);
        assert!(Path::new(dir).is_dir());
    }
}

#[test]
fn validate_example_dirs_complete() {
    use crate::util;
    use std::path::Path;

    let mut total: usize = 0;

    // Mutably borrows 'total'
    // Interesting that Rc is not needed here, since the closure
    // goes out of scope before we need to access total again.
    let mut validate_dir = |dir: &Path| {
        println!("Checking path is registered in constants.rs: {:?}", dir);
        let dir_str = dir.to_str().unwrap_or_else(|| {
            panic!("Could not convert path to string: {:?}", dir);
        });
        assert!(ALL_EXAMPLE_DIRS.contains(&dir_str));
        total += 1;
    };

    util::walk_dirs_rec(Path::new("examples"), &mut validate_dir)
        .unwrap_or_else(|err| {
            panic!("Error when walking examples dir: {:?}", err);
        });

    // Check total is correct
    assert_eq!(total, ALL_EXAMPLE_DIRS.len());
}
