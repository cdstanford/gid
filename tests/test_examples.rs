/*
    Test the unit tests in the examples/ directory.
*/

use state_graph::driver;

#[test]
fn test_1() {
    driver::assert_example("1");
}

#[test]
fn test_2() {
    driver::assert_example("2");
}

#[test]
fn test_3() {
    driver::assert_example("3");
}

#[test]
fn test_line() {
    driver::assert_example("line_3");
    driver::assert_example("line_10");
    driver::assert_example("line_20");
    driver::assert_example("line_100");
}

#[test]
fn test_reverseline() {
    driver::assert_example("reverseline_3");
    driver::assert_example("reverseline_10");
    driver::assert_example("reverseline_20");
    driver::assert_example("reverseline_100");
}

#[test]
fn test_liveline() {
    driver::assert_example("liveline_3");
    driver::assert_example("liveline_10");
    driver::assert_example("liveline_20");
    driver::assert_example("liveline_100");
}

#[test]
fn test_reverseliveline() {
    driver::assert_example("reverseliveline_3");
    driver::assert_example("reverseliveline_10");
    driver::assert_example("reverseliveline_20");
    driver::assert_example("reverseliveline_100");
}

#[test]
fn test_loop() {
    driver::assert_example("loop_3");
    driver::assert_example("loop_10");
    driver::assert_example("loop_20");
    driver::assert_example("loop_100");
}

#[test]
fn test_liveloop() {
    driver::assert_example("liveloop_3");
    driver::assert_example("liveloop_10");
    driver::assert_example("liveloop_20");
    driver::assert_example("liveloop_100");
}

#[test]
fn test_reverseloop() {
    driver::assert_example("reverseloop_3");
    driver::assert_example("reverseloop_10");
    driver::assert_example("reverseloop_20");
    driver::assert_example("reverseloop_100");
}

#[test]
fn test_reverseliveloop() {
    driver::assert_example("reverseliveloop_3");
    driver::assert_example("reverseliveloop_10");
    driver::assert_example("reverseliveloop_20");
    driver::assert_example("reverseliveloop_100");
}

#[test]
fn test_tree() {
    driver::assert_example("tree_3");
}
