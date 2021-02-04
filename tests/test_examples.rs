/*
    Test the unit tests in the examples/ directory.
*/

use state_graph::constants::*;
use state_graph::driver;

/*
    Helper functions
*/
fn assert_one(dir: &str, name: &str) {
    driver::assert_example(
        &format!("{}/{}", dir, name),
        UNIT_TEST_TIMEOUT_SECS,
    );
}
fn assert_expensive(dir: &str, name: &str) {
    driver::assert_example(
        &format!("{}/{}", dir, name),
        UNIT_TEST_TIMEOUT_EXPENSIVE,
    );
}
#[allow(dead_code)]
fn assert_all(dir: &str) {
    for basename in driver::example_basenames_in_dir(dir) {
        driver::assert_example(&basename, UNIT_TEST_TIMEOUT_SECS);
    }
}

/*
    Handwritten unit tests
*/

#[test]
fn test_1() {
    assert_one(EX_DIR_HANDWRITTEN, "1");
}
#[test]
fn test_2() {
    assert_one(EX_DIR_HANDWRITTEN, "2");
}
#[test]
fn test_3() {
    assert_one(EX_DIR_HANDWRITTEN, "3");
}
#[test]
fn test_4() {
    assert_one(EX_DIR_HANDWRITTEN, "4");
}
#[test]
fn test_5() {
    assert_one(EX_DIR_HANDWRITTEN, "5");
}
#[test]
fn test_6() {
    assert_one(EX_DIR_HANDWRITTEN, "6");
}
#[test]
fn test_7() {
    assert_one(EX_DIR_HANDWRITTEN, "7");
}
#[test]
fn test_8() {
    assert_one(EX_DIR_HANDWRITTEN, "8");
}
#[test]
fn test_9() {
    assert_one(EX_DIR_HANDWRITTEN, "9");
}
#[test]
fn test_10() {
    assert_one(EX_DIR_HANDWRITTEN, "10");
}
#[test]
fn test_11() {
    assert_one(EX_DIR_HANDWRITTEN, "11");
}
#[test]
fn test_12() {
    assert_one(EX_DIR_HANDWRITTEN, "12");
}
#[test]
fn test_13() {
    assert_one(EX_DIR_HANDWRITTEN, "13");
}
#[test]
fn test_14() {
    assert_one(EX_DIR_HANDWRITTEN, "14");
}
#[test]
fn test_15() {
    assert_one(EX_DIR_HANDWRITTEN, "15");
}
#[test]
fn test_16() {
    assert_one(EX_DIR_HANDWRITTEN, "16");
}
#[test]
fn test_17() {
    assert_one(EX_DIR_HANDWRITTEN, "17");
}
#[test]
fn test_18() {
    assert_one(EX_DIR_HANDWRITTEN, "18");
}
#[test]
fn test_19() {
    assert_one(EX_DIR_HANDWRITTEN, "19");
}
#[test]
fn test_20() {
    assert_one(EX_DIR_HANDWRITTEN, "20");
}
#[test]
fn test_tree() {
    assert_one(EX_DIR_HANDWRITTEN, "tree_3");
}

/*
    Auto-generated examples
*/

#[test]
fn test_line() {
    assert_one(EX_DIR_GENERATED, "line_3");
    assert_one(EX_DIR_GENERATED, "line_10");
    assert_one(EX_DIR_GENERATED, "line_20");
    assert_one(EX_DIR_GENERATED, "line_100");
}

#[test]
fn test_reverseline() {
    assert_one(EX_DIR_GENERATED, "reverseline_3");
    assert_one(EX_DIR_GENERATED, "reverseline_10");
    assert_one(EX_DIR_GENERATED, "reverseline_20");
    assert_one(EX_DIR_GENERATED, "reverseline_100");
}

#[test]
fn test_unkline() {
    assert_one(EX_DIR_GENERATED, "unkline_3");
    assert_one(EX_DIR_GENERATED, "unkline_10");
    assert_one(EX_DIR_GENERATED, "unkline_20");
    assert_one(EX_DIR_GENERATED, "unkline_100");
}

#[test]
fn test_reverseunkline() {
    assert_one(EX_DIR_GENERATED, "reverseunkline_3");
    assert_one(EX_DIR_GENERATED, "reverseunkline_10");
    assert_one(EX_DIR_GENERATED, "reverseunkline_20");
    assert_one(EX_DIR_GENERATED, "reverseunkline_100");
}

#[test]
fn test_loop() {
    assert_one(EX_DIR_GENERATED, "loop_3");
    assert_one(EX_DIR_GENERATED, "loop_10");
    assert_one(EX_DIR_GENERATED, "loop_20");
    assert_one(EX_DIR_GENERATED, "loop_100");
}

#[test]
fn test_unkloop() {
    assert_one(EX_DIR_GENERATED, "unkloop_3");
    assert_one(EX_DIR_GENERATED, "unkloop_10");
    assert_one(EX_DIR_GENERATED, "unkloop_20");
    assert_one(EX_DIR_GENERATED, "unkloop_100");
}

#[test]
fn test_reverseloop() {
    assert_one(EX_DIR_GENERATED, "reverseloop_3");
    assert_one(EX_DIR_GENERATED, "reverseloop_10");
    assert_one(EX_DIR_GENERATED, "reverseloop_20");
    assert_one(EX_DIR_GENERATED, "reverseloop_100");
}

#[test]
fn test_reverseunkloop() {
    assert_one(EX_DIR_GENERATED, "reverseunkloop_3");
    assert_one(EX_DIR_GENERATED, "reverseunkloop_10");
    assert_one(EX_DIR_GENERATED, "reverseunkloop_20");
    assert_one(EX_DIR_GENERATED, "reverseunkloop_100");
}

#[test]
#[ignore]
fn test_generated_expensive() {
    assert_expensive(EX_DIR_GENERATED, "line_1000");
    assert_expensive(EX_DIR_GENERATED, "reverseline_1000");
    assert_expensive(EX_DIR_GENERATED, "unkline_1000");
    assert_expensive(EX_DIR_GENERATED, "reverseunkline_1000");
    assert_expensive(EX_DIR_GENERATED, "loop_1000");
    assert_expensive(EX_DIR_GENERATED, "reverseloop_1000");
    assert_expensive(EX_DIR_GENERATED, "unkloop_1000");
    assert_expensive(EX_DIR_GENERATED, "reverseunkloop_1000");
}

/*
    Regex Examples
    (No expected output -- compares for agreement across algorithms)
*/

#[test]
fn test_regex_comp() {
    assert_one(EX_DIR_REGEX_COMP, "comp1_inclusion_unsat");
    assert_one(EX_DIR_REGEX_COMP, "comp2_inclusion_sat");
    assert_one(EX_DIR_REGEX_COMP, "simple_complement_unsat");
}

#[test]
fn test_regex_date() {
    assert_one(EX_DIR_REGEX_DATE, "date_minimal_sat");
    assert_one(EX_DIR_REGEX_DATE, "date_minimal_unsat");
}

#[test]
fn test_regex_loop() {
    assert_one(EX_DIR_REGEX_LOOP, "deadloop1_sat");
    assert_one(EX_DIR_REGEX_LOOP, "deadloop2_sat");
    assert_one(EX_DIR_REGEX_LOOP, "deadloop3_unsat");
    assert_one(EX_DIR_REGEX_LOOP, "evil1_unsat");
    assert_one(EX_DIR_REGEX_LOOP, "evil2_inter_unsat");
    assert_one(EX_DIR_REGEX_LOOP, "evil2_sat");
    assert_one(EX_DIR_REGEX_LOOP, "nestedloop1_unsat");
    assert_one(EX_DIR_REGEX_LOOP, "nestedloop2_sat");
    assert_one(EX_DIR_REGEX_LOOP, "nestedloop2_unsat");
}

#[test]
fn test_regex_sgeasy() {
    assert_one(EX_DIR_REGEX_SGEASY, "diamond_chain_10");
    assert_one(EX_DIR_REGEX_SGEASY, "inter_1_2_3");
    assert_one(EX_DIR_REGEX_SGEASY, "inter_star_3_3");
    assert_one(EX_DIR_REGEX_SGEASY, "long_3");
}

#[test]
#[ignore]
fn test_regex_extra() {
    // Medium-length tests, not really necessary to run normally
    assert_one(EX_DIR_REGEX_COMP, "comp1_nonempty_trivial_sat");
    assert_one(EX_DIR_REGEX_SGEASY, "diamond_chain_30");
    assert_one(EX_DIR_REGEX_SGEASY, "inter_3_6_9");
    assert_one(EX_DIR_REGEX_SGHARD, "inter_10_20_30");
    assert_one(EX_DIR_REGEX_SGEASY, "inter_star_10_10");
    assert_one(EX_DIR_REGEX_SGEASY, "inter_star_30_30");
    assert_one(EX_DIR_REGEX_SGEASY, "long_10");
    assert_one(EX_DIR_REGEX_SGHARD, "long_30");
}

#[test]
#[ignore]
fn test_regex_expensive() {
    // Long expensive tests
    assert_expensive(EX_DIR_REGEX_DATE, "date1_sat");
    assert_expensive(EX_DIR_REGEX_DATE, "date2_sat");
    assert_expensive(EX_DIR_REGEX_DATE, "date_unsat");
    assert_expensive(EX_DIR_REGEX_SGEASY, "diamond_chain_100");
    assert_expensive(EX_DIR_REGEX_SGHARD, "diamond_chain_300");
    assert_expensive(EX_DIR_REGEX_SGHARD, "inter_30_60_90");
    assert_expensive(EX_DIR_REGEX_SGHARD, "inter_star_100_100");
}
