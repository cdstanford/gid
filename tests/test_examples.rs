/*
    Test the unit tests in the examples/ directory.
*/

use state_graph::driver;

#[test]
fn test_1() {
    driver::assert_example("1");
}

#[test]
fn test_line_3() {
    driver::assert_example("line_10");
}
#[test]
fn test_line_10() {
    driver::assert_example("line_10");
}
#[test]
fn test_line_20() {
    driver::assert_example("line_20");
}
#[test]
fn test_line_100() {
    driver::assert_example("line_100");
}

#[test]
fn test_reverseline_3() {
    driver::assert_example("reverseline_10");
}
#[test]
fn test_reverseline_10() {
    driver::assert_example("reverseline_10");
}
#[test]
fn test_reverseline_20() {
    driver::assert_example("reverseline_20");
}
#[test]
fn test_reverseline_100() {
    driver::assert_example("reverseline_100");
}

#[test]
fn test_liveline_3() {
    driver::assert_example("liveline_10");
}
#[test]
fn test_liveline_10() {
    driver::assert_example("liveline_10");
}
#[test]
fn test_liveline_20() {
    driver::assert_example("liveline_20");
}
#[test]
fn test_liveline_100() {
    driver::assert_example("liveline_100");
}

#[test]
fn test_reverseliveline_3() {
    driver::assert_example("reverseliveline_10");
}
#[test]
fn test_reverseliveline_10() {
    driver::assert_example("reverseliveline_10");
}
#[test]
fn test_reverseliveline_20() {
    driver::assert_example("reverseliveline_20");
}
#[test]
fn test_reverseliveline_100() {
    driver::assert_example("reverseliveline_100");
}

#[test]
fn test_loop_3() {
    driver::assert_example("loop_3");
}
#[test]
fn test_loop_10() {
    driver::assert_example("loop_10");
}
#[test]
fn test_loop_20() {
    driver::assert_example("loop_20");
}
#[test]
fn test_loop_100() {
    driver::assert_example("loop_100");
}

#[test]
fn test_liveloop_3() {
    driver::assert_example("liveloop_3");
}
#[test]
fn test_liveloop_10() {
    driver::assert_example("liveloop_10");
}
#[test]
fn test_liveloop_20() {
    driver::assert_example("liveloop_20");
}
#[test]
fn test_liveloop_100() {
    driver::assert_example("liveloop_100");
}

#[test]
fn test_reverseloop_3() {
    driver::assert_example("reverseloop_3");
}
#[test]
fn test_reverseloop_10() {
    driver::assert_example("reverseloop_10");
}
#[test]
fn test_reverseloop_20() {
    driver::assert_example("reverseloop_20");
}
#[test]
fn test_reverseloop_100() {
    driver::assert_example("reverseloop_100");
}

#[test]
fn test_tree_3() {
    driver::assert_example("tree_3");
}
