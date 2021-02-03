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
fn test_10() {
    driver::assert_example(&handwritten("10"));
}
#[test]
fn test_11() {
    driver::assert_example(&handwritten("11"));
}
#[test]
fn test_12() {
    driver::assert_example(&handwritten("12"));
}
#[test]
fn test_13() {
    driver::assert_example(&handwritten("13"));
}
#[test]
fn test_14() {
    driver::assert_example(&handwritten("14"));
}
#[test]
fn test_15() {
    driver::assert_example(&handwritten("15"));
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
fn test_unkline() {
    driver::assert_example(&generated("unkline_3"));
    driver::assert_example(&generated("unkline_10"));
    driver::assert_example(&generated("unkline_20"));
    driver::assert_example(&generated("unkline_100"));
}

#[test]
fn test_reverseunkline() {
    driver::assert_example(&generated("reverseunkline_3"));
    driver::assert_example(&generated("reverseunkline_10"));
    driver::assert_example(&generated("reverseunkline_20"));
    driver::assert_example(&generated("reverseunkline_100"));
}

#[test]
fn test_loop() {
    driver::assert_example(&generated("loop_3"));
    driver::assert_example(&generated("loop_10"));
    driver::assert_example(&generated("loop_20"));
    driver::assert_example(&generated("loop_100"));
}

#[test]
fn test_unkloop() {
    driver::assert_example(&generated("unkloop_3"));
    driver::assert_example(&generated("unkloop_10"));
    driver::assert_example(&generated("unkloop_20"));
    driver::assert_example(&generated("unkloop_100"));
}

#[test]
fn test_reverseloop() {
    driver::assert_example(&generated("reverseloop_3"));
    driver::assert_example(&generated("reverseloop_10"));
    driver::assert_example(&generated("reverseloop_20"));
    driver::assert_example(&generated("reverseloop_100"));
}

#[test]
fn test_reverseunkloop() {
    driver::assert_example(&generated("reverseunkloop_3"));
    driver::assert_example(&generated("reverseunkloop_10"));
    driver::assert_example(&generated("reverseunkloop_20"));
    driver::assert_example(&generated("reverseunkloop_100"));
}

/*
    Regex Examples
    (No expected output -- compares for agreement across algorithms)
*/

#[test]
fn test_regex_comp() {
    driver::assert_example("examples/regex/complement/comp1_inclusion_unsat");
    driver::assert_example("examples/regex/complement/comp2_inclusion_sat");
    driver::assert_example("examples/regex/complement/simple_complement_unsat");
}

#[test]
fn test_regex_date() {
    driver::assert_example("examples/regex/date/date_minimal_sat");
    driver::assert_example("examples/regex/date/date_minimal_unsat");
}

#[test]
fn test_regex_loop() {
    driver::assert_example("examples/regex/loop/deadloop1_sat");
    driver::assert_example("examples/regex/loop/deadloop2_sat");
    driver::assert_example("examples/regex/loop/deadloop3_unsat");
    driver::assert_example("examples/regex/loop/evil1_unsat");
    driver::assert_example("examples/regex/loop/evil2_inter_unsat");
    driver::assert_example("examples/regex/loop/evil2_sat");
    driver::assert_example("examples/regex/loop/nestedloop1_unsat");
    driver::assert_example("examples/regex/loop/nestedloop2_sat");
    driver::assert_example("examples/regex/loop/nestedloop2_unsat");
}

#[test]
fn test_regex_sgeasy() {
    driver::assert_example("examples/regex/state_graph_easy/diamond_chain_10");
    driver::assert_example("examples/regex/state_graph_easy/inter_1_2_3");
    driver::assert_example("examples/regex/state_graph_easy/inter_star_3_3");
    driver::assert_example("examples/regex/state_graph_easy/long_3");
}
