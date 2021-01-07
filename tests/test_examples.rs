/*
    Test the unit tests in the examples/ directory.
*/

use state_graph::driver;

#[test]
fn test_1() {
    driver::assert_example("1");
}
#[test]
fn test_line_10() {
    driver::assert_example("line_10");
}
#[test]
fn test_reverseline_10() {
    driver::assert_example("reverseline_10");
}
#[test]
fn test_line_20() {
    driver::assert_example("line_20");
}
#[test]
fn test_reverseline_20() {
    driver::assert_example("reverseline_20");
}
#[test]
fn test_tree_3() {
    driver::assert_example("tree_3");
}
#[test]
fn test_loop_3() {
    driver::assert_example("loop_3");
}
#[test]
fn test_liveloop_3() {
    driver::assert_example("liveloop_3");
}
#[test]
fn test_reverseloop_3() {
    driver::assert_example("reverseloop_3");
}
