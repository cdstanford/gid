/*
    Test the unit tests in the examples/ directory.
*/

use state_graph::constants::{EX_DIR_GENERATED, EX_DIR_HANDWRITTEN};
use state_graph::driver;

/*
    Helper functions
*/
fn handwritten(name: &str) -> String {
    format!("{}/{}", EX_DIR_HANDWRITTEN, name)
}
fn generated(name: &str) -> String {
    format!("{}/{}", EX_DIR_GENERATED, name)
}

/*
    Handwritten examples
*/

#[test]
fn test_1() {
    driver::assert_example(&handwritten("1"));
}
#[test]
fn test_2() {
    driver::assert_example(&handwritten("2"));
}
#[test]
fn test_3() {
    driver::assert_example(&handwritten("3"));
}
#[test]
fn test_4() {
    driver::assert_example(&handwritten("4"));
}
#[test]
fn test_5() {
    driver::assert_example(&handwritten("5"));
}
#[test]
fn test_6() {
    driver::assert_example(&handwritten("6"));
}
#[test]
fn test_7() {
    driver::assert_example(&handwritten("7"));
}
#[test]
fn test_8() {
    driver::assert_example(&handwritten("8"));
}
#[test]
fn test_9() {
    driver::assert_example(&handwritten("9"));
}
#[test]
fn test_tree() {
    driver::assert_example(&handwritten("tree_3"));
}

/*
    Auto-generated examples
*/

#[test]
fn test_line() {
    driver::assert_example(&generated("line_3"));
    driver::assert_example(&generated("line_10"));
    driver::assert_example(&generated("line_20"));
    driver::assert_example(&generated("line_100"));
}

#[test]
fn test_reverseline() {
    driver::assert_example(&generated("reverseline_3"));
    driver::assert_example(&generated("reverseline_10"));
    driver::assert_example(&generated("reverseline_20"));
    driver::assert_example(&generated("reverseline_100"));
}

#[test]
fn test_liveline() {
    driver::assert_example(&generated("liveline_3"));
    driver::assert_example(&generated("liveline_10"));
    driver::assert_example(&generated("liveline_20"));
    driver::assert_example(&generated("liveline_100"));
}

#[test]
fn test_reverseliveline() {
    driver::assert_example(&generated("reverseliveline_3"));
    driver::assert_example(&generated("reverseliveline_10"));
    driver::assert_example(&generated("reverseliveline_20"));
    driver::assert_example(&generated("reverseliveline_100"));
}

#[test]
fn test_loop() {
    driver::assert_example(&generated("loop_3"));
    driver::assert_example(&generated("loop_10"));
    driver::assert_example(&generated("loop_20"));
    driver::assert_example(&generated("loop_100"));
}

#[test]
fn test_liveloop() {
    driver::assert_example(&generated("liveloop_3"));
    driver::assert_example(&generated("liveloop_10"));
    driver::assert_example(&generated("liveloop_20"));
    driver::assert_example(&generated("liveloop_100"));
}

#[test]
fn test_reverseloop() {
    driver::assert_example(&generated("reverseloop_3"));
    driver::assert_example(&generated("reverseloop_10"));
    driver::assert_example(&generated("reverseloop_20"));
    driver::assert_example(&generated("reverseloop_100"));
}

#[test]
fn test_reverseliveloop() {
    driver::assert_example(&generated("reverseliveloop_3"));
    driver::assert_example(&generated("reverseliveloop_10"));
    driver::assert_example(&generated("reverseliveloop_20"));
    driver::assert_example(&generated("reverseliveloop_100"));
}
