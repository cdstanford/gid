/*
    Test the unit tests in the examples/ directory.
*/

use state_graph::constants::{EX_DIR_GENERATED, EX_DIR_HANDWRITTEN};
use state_graph::driver;

/*
    Handwritten examples
*/

#[test]
fn test_1() {
    driver::assert_example(EX_DIR_HANDWRITTEN, "1");
}
#[test]
fn test_2() {
    driver::assert_example(EX_DIR_HANDWRITTEN, "2");
}
#[test]
fn test_3() {
    driver::assert_example(EX_DIR_HANDWRITTEN, "3");
}
#[test]
fn test_tree() {
    driver::assert_example(EX_DIR_HANDWRITTEN, "tree_3");
}

/*
    Auto-generated examples
*/

#[test]
fn test_line() {
    driver::assert_example(EX_DIR_GENERATED, "line_3");
    driver::assert_example(EX_DIR_GENERATED, "line_10");
    driver::assert_example(EX_DIR_GENERATED, "line_20");
    driver::assert_example(EX_DIR_GENERATED, "line_100");
}

#[test]
fn test_reverseline() {
    driver::assert_example(EX_DIR_GENERATED, "reverseline_3");
    driver::assert_example(EX_DIR_GENERATED, "reverseline_10");
    driver::assert_example(EX_DIR_GENERATED, "reverseline_20");
    driver::assert_example(EX_DIR_GENERATED, "reverseline_100");
}

#[test]
fn test_liveline() {
    driver::assert_example(EX_DIR_GENERATED, "liveline_3");
    driver::assert_example(EX_DIR_GENERATED, "liveline_10");
    driver::assert_example(EX_DIR_GENERATED, "liveline_20");
    driver::assert_example(EX_DIR_GENERATED, "liveline_100");
}

#[test]
fn test_reverseliveline() {
    driver::assert_example(EX_DIR_GENERATED, "reverseliveline_3");
    driver::assert_example(EX_DIR_GENERATED, "reverseliveline_10");
    driver::assert_example(EX_DIR_GENERATED, "reverseliveline_20");
    driver::assert_example(EX_DIR_GENERATED, "reverseliveline_100");
}

#[test]
fn test_loop() {
    driver::assert_example(EX_DIR_GENERATED, "loop_3");
    driver::assert_example(EX_DIR_GENERATED, "loop_10");
    driver::assert_example(EX_DIR_GENERATED, "loop_20");
    driver::assert_example(EX_DIR_GENERATED, "loop_100");
}

#[test]
fn test_liveloop() {
    driver::assert_example(EX_DIR_GENERATED, "liveloop_3");
    driver::assert_example(EX_DIR_GENERATED, "liveloop_10");
    driver::assert_example(EX_DIR_GENERATED, "liveloop_20");
    driver::assert_example(EX_DIR_GENERATED, "liveloop_100");
}

#[test]
fn test_reverseloop() {
    driver::assert_example(EX_DIR_GENERATED, "reverseloop_3");
    driver::assert_example(EX_DIR_GENERATED, "reverseloop_10");
    driver::assert_example(EX_DIR_GENERATED, "reverseloop_20");
    driver::assert_example(EX_DIR_GENERATED, "reverseloop_100");
}

#[test]
fn test_reverseliveloop() {
    driver::assert_example(EX_DIR_GENERATED, "reverseliveloop_3");
    driver::assert_example(EX_DIR_GENERATED, "reverseliveloop_10");
    driver::assert_example(EX_DIR_GENERATED, "reverseliveloop_20");
    driver::assert_example(EX_DIR_GENERATED, "reverseliveloop_100");
}
