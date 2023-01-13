/*
    Test the unit tests in the examples/ directory.
*/

use guided_digraph::constants::*;
use guided_digraph::driver;

/*
    Helper function
*/

fn assert_one(dir: &str, name: &str) {
    driver::assert_example(&format!("{}/{}", dir, name), None);
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
    // this test is a tree graph.
    // originally called tree_3
    assert_one(EX_DIR_HANDWRITTEN, "20");
}
#[test]
fn test_21() {
    assert_one(EX_DIR_HANDWRITTEN, "21");
}
#[test]
fn test_22() {
    assert_one(EX_DIR_HANDWRITTEN, "22");
}
#[test]
fn test_23() {
    assert_one(EX_DIR_HANDWRITTEN, "23");
}
#[test]
fn test_24() {
    assert_one(EX_DIR_HANDWRITTEN, "24");
}
#[test]
fn test_25() {
    assert_one(EX_DIR_HANDWRITTEN, "25");
}

/*
    Auto-generated examples
*/

#[test]
fn test_line() {
    assert_one(EX_DIR_GENERATED, "line_3");
    assert_one(EX_DIR_GENERATED, "line_10");
    assert_one(EX_DIR_GENERATED, "line_30");
    assert_one(EX_DIR_GENERATED, "line_100");
}

#[test]
fn test_reverseline() {
    assert_one(EX_DIR_GENERATED, "reverseline_3");
    assert_one(EX_DIR_GENERATED, "reverseline_10");
    assert_one(EX_DIR_GENERATED, "reverseline_30");
    assert_one(EX_DIR_GENERATED, "reverseline_100");
}

#[test]
fn test_unkline() {
    assert_one(EX_DIR_GENERATED, "unkline_3");
    assert_one(EX_DIR_GENERATED, "unkline_10");
    assert_one(EX_DIR_GENERATED, "unkline_30");
    assert_one(EX_DIR_GENERATED, "unkline_100");
}

#[test]
fn test_reverseunkline() {
    assert_one(EX_DIR_GENERATED, "reverseunkline_3");
    assert_one(EX_DIR_GENERATED, "reverseunkline_10");
    assert_one(EX_DIR_GENERATED, "reverseunkline_30");
    assert_one(EX_DIR_GENERATED, "reverseunkline_100");
}

#[test]
fn test_loop() {
    assert_one(EX_DIR_GENERATED, "loop_3");
    assert_one(EX_DIR_GENERATED, "loop_10");
    assert_one(EX_DIR_GENERATED, "loop_30");
    assert_one(EX_DIR_GENERATED, "loop_100");
}

#[test]
fn test_unkloop() {
    assert_one(EX_DIR_GENERATED, "unkloop_3");
    assert_one(EX_DIR_GENERATED, "unkloop_10");
    assert_one(EX_DIR_GENERATED, "unkloop_30");
    assert_one(EX_DIR_GENERATED, "unkloop_100");
}

#[test]
fn test_reverseloop() {
    assert_one(EX_DIR_GENERATED, "reverseloop_3");
    assert_one(EX_DIR_GENERATED, "reverseloop_10");
    assert_one(EX_DIR_GENERATED, "reverseloop_30");
    assert_one(EX_DIR_GENERATED, "reverseloop_100");
}

#[test]
fn test_reverseunkloop() {
    assert_one(EX_DIR_GENERATED, "reverseunkloop_3");
    assert_one(EX_DIR_GENERATED, "reverseunkloop_10");
    assert_one(EX_DIR_GENERATED, "reverseunkloop_30");
    assert_one(EX_DIR_GENERATED, "reverseunkloop_100");
}

#[test]
fn test_complete() {
    assert_one(EX_DIR_GENERATED, "complete_3");
    assert_one(EX_DIR_GENERATED, "complete_10");
    assert_one(EX_DIR_GENERATED, "complete_30");
    assert_one(EX_DIR_GENERATED, "unkcomplete_3");
    assert_one(EX_DIR_GENERATED, "unkcomplete_10");
    assert_one(EX_DIR_GENERATED, "unkcomplete_30");
}

#[test]
fn test_complete_acyclic() {
    assert_one(EX_DIR_GENERATED, "completeacyclic_3");
    assert_one(EX_DIR_GENERATED, "completeacyclic_10");
    assert_one(EX_DIR_GENERATED, "completeacyclic_30");
    assert_one(EX_DIR_GENERATED, "unkcompleteacyclic_3");
    assert_one(EX_DIR_GENERATED, "unkcompleteacyclic_10");
    assert_one(EX_DIR_GENERATED, "unkcompleteacyclic_30");
    assert_one(EX_DIR_GENERATED, "revcompleteacyclic_3");
    assert_one(EX_DIR_GENERATED, "revcompleteacyclic_10");
    assert_one(EX_DIR_GENERATED, "revcompleteacyclic_30");
    assert_one(EX_DIR_GENERATED, "unkrevcompleteacyclic_3");
    assert_one(EX_DIR_GENERATED, "unkrevcompleteacyclic_10");
    assert_one(EX_DIR_GENERATED, "unkrevcompleteacyclic_30");
}

#[test]
fn test_bipartite() {
    assert_one(EX_DIR_GENERATED, "bipartite_3_3");
    assert_one(EX_DIR_GENERATED, "bipartite_10_10");
    assert_one(EX_DIR_GENERATED, "bipartite_100_10");
    assert_one(EX_DIR_GENERATED, "bipartite_10_100");
    assert_one(EX_DIR_GENERATED, "bipartite_100_100");
    assert_one(EX_DIR_GENERATED, "unkbipartite_3_3");
    assert_one(EX_DIR_GENERATED, "unkbipartite_10_10");
    assert_one(EX_DIR_GENERATED, "unkbipartite_100_10");
    assert_one(EX_DIR_GENERATED, "unkbipartite_10_100");
    assert_one(EX_DIR_GENERATED, "unkbipartite_100_100");
}

#[test]
#[ignore]
fn test_generated_expensive() {
    // Omitted generated examples: _300, _3000, _10000
    assert_one(EX_DIR_GENERATED, "line_1000");
    assert_one(EX_DIR_GENERATED, "reverseline_1000");
    assert_one(EX_DIR_GENERATED, "unkline_1000");
    assert_one(EX_DIR_GENERATED, "reverseunkline_1000");
    assert_one(EX_DIR_GENERATED, "loop_1000");
    assert_one(EX_DIR_GENERATED, "reverseloop_1000");
    assert_one(EX_DIR_GENERATED, "unkloop_1000");
    assert_one(EX_DIR_GENERATED, "reverseunkloop_1000");
}

/*
    Regex Examples
    (No expected output -- compares for agreement across algorithms)
*/

#[test]
fn test_regex_comp() {
    assert_one(EX_DIR_REGEX_COMP, "comp1_inclusion_unsat");
    assert_one(EX_DIR_REGEX_COMP, "comp1_nonempty_trivial_sat");
    assert_one(EX_DIR_REGEX_COMP, "comp2_inclusion_sat");
    assert_one(EX_DIR_REGEX_COMP, "simple_complement_unsat");
}

#[test]
fn test_regex_inter() {
    assert_one(EX_DIR_REGEX_INTER, "demo_unsat");
    assert_one(EX_DIR_REGEX_INTER, "inter_mod2_unsat");
    assert_one(EX_DIR_REGEX_INTER, "inter_mod3_unsat");
    assert_one(
        EX_DIR_REGEX_INTER,
        "zelkova_fmcad18_example_explicit_inter_unsat",
    );
}

#[test]
fn test_regex_count() {
    assert_one(EX_DIR_REGEX_COUNT, "re_count_sat_easy");
    assert_one(EX_DIR_REGEX_COUNT, "re_count_sat_medium");
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
fn test_regex_detblowup() {
    assert_one(EX_DIR_REGEX_BLOWUP, "det_blowup_sat_3");
    assert_one(EX_DIR_REGEX_BLOWUP, "det_blowup_sat_5");
    assert_one(EX_DIR_REGEX_BLOWUP, "det_blowup_sat_10");
    assert_one(EX_DIR_REGEX_BLOWUP, "det_blowup_sat_100");
    assert_one(EX_DIR_REGEX_BLOWUP, "det_blowup_unsat_1");
    assert_one(EX_DIR_REGEX_BLOWUP, "det_blowup_unsat_3");
    assert_one(EX_DIR_REGEX_BLOWUP, "det_blowup_unsat_5");
    assert_one(EX_DIR_REGEX_BLOWUP, "det_blowup_unsat_10");
    assert_one(EX_DIR_REGEX_BLOWUP, "digit05_unsat");
    assert_one(EX_DIR_REGEX_BLOWUP, "digit10_unsat");
    assert_one(EX_DIR_REGEX_BLOWUP, "digit15_unsat");
    assert_one(EX_DIR_REGEX_BLOWUP, "digit20_unsat");
}

#[test]
fn test_regex_date() {
    assert_one(EX_DIR_REGEX_DATE, "date_minimal_sat");
    assert_one(EX_DIR_REGEX_DATE, "date_minimal_unsat");
}

#[test]
fn test_regex_sgeasy() {
    assert_one(EX_DIR_REGEX_SGEASY, "diamond_chain_10");
    assert_one(EX_DIR_REGEX_SGEASY, "inter_1_2_3");
    assert_one(EX_DIR_REGEX_SGEASY, "inter_star_3_3");
    assert_one(EX_DIR_REGEX_SGEASY, "long_3");
}

#[test]
fn test_regex_password() {
    assert_one(EX_DIR_REGEX_PASSW, "passw_sat1");
    assert_one(EX_DIR_REGEX_PASSW, "passw_sat2");
    assert_one(EX_DIR_REGEX_PASSW, "passw_sat3");
    assert_one(EX_DIR_REGEX_PASSW, "passw_sat4");
    assert_one(EX_DIR_REGEX_PASSW, "passw_unsat1");
    assert_one(EX_DIR_REGEX_PASSW, "passw_unsat3");
}

#[test]
#[ignore]
fn test_regex_password_medium() {
    assert_one(EX_DIR_REGEX_PASSW, "passw_very_complex_2_4_unsat");
    assert_one(EX_DIR_REGEX_PASSW, "passw_very_complex_3_4_unsat");
    assert_one(EX_DIR_REGEX_PASSW, "passw_complex_4_10_sat");
    assert_one(EX_DIR_REGEX_PASSW, "passw_complex_5_10_sat");
    assert_one(EX_DIR_REGEX_PASSW, "passw_complex_6_10_sat");
    assert_one(EX_DIR_REGEX_PASSW, "passw_complex_7_10_sat");
    assert_one(EX_DIR_REGEX_PASSW, "passw_complex_8_10_sat");
    assert_one(EX_DIR_REGEX_PASSW, "passw_complex_9_10_sat");
}

#[test]
#[ignore]
fn test_regex_medium() {
    // Medium-length tests, not really necessary to run normally
    assert_one(EX_DIR_REGEX_DATE, "date1_sat");
    assert_one(EX_DIR_REGEX_DATE, "date2_sat");
    assert_one(EX_DIR_REGEX_DATE, "date_unsat");
    assert_one(EX_DIR_REGEX_BLOWUP, "det_blowup_sat_1000");
    assert_one(EX_DIR_REGEX_SGEASY, "diamond_chain_30");
    assert_one(EX_DIR_REGEX_SGEASY, "diamond_chain_100");
    assert_one(EX_DIR_REGEX_SGHARD, "diamond_chain_300");
    assert_one(EX_DIR_REGEX_SGEASY, "inter_3_6_9");
    assert_one(EX_DIR_REGEX_SGHARD, "inter_10_20_30");
    assert_one(EX_DIR_REGEX_SGHARD, "inter_30_60_90");
    assert_one(EX_DIR_REGEX_SGEASY, "inter_star_10_10");
    assert_one(EX_DIR_REGEX_SGEASY, "inter_star_30_30");
    assert_one(EX_DIR_REGEX_SGHARD, "inter_star_100_100");
    assert_one(EX_DIR_REGEX_SGEASY, "long_10");
    assert_one(EX_DIR_REGEX_SGHARD, "long_30");
}

#[test]
#[ignore]
fn test_regex_expensive() {
    // Long expensive tests
    assert_one(EX_DIR_REGEX_BLOWUP, "det_blowup_unsat_100");
    assert_one(EX_DIR_REGEX_PASSW, "passw_very_complex_1_7_unsat");
    assert_one(EX_DIR_REGEX_PASSW, "passw_very_complex_2_7_unsat");
    assert_one(EX_DIR_REGEX_PASSW, "passw_very_complex_3_7_unsat");
    assert_one(EX_DIR_REGEX_PASSW, "passw_very_complex_4_7_unsat");
    assert_one(EX_DIR_REGEX_PASSW, "passw_very_complex_5_7_unsat");
    assert_one(EX_DIR_REGEX_PASSW, "passw_very_complex_6_7_unsat");
}

/*
    More Regex Examples -- from RegexLib
    (No expected output -- compares for agreement across algorithms)
*/

#[test]
fn test_regexlib_membership() {
    assert_one(EX_DIR_RLIB_M1, "membership_0");
    assert_one(EX_DIR_RLIB_M1, "membership_10");
    assert_one(EX_DIR_RLIB_M1, "membership_34");
    assert_one(EX_DIR_RLIB_M1, "membership_56");
    assert_one(EX_DIR_RLIB_M1, "membership_94");
    assert_one(EX_DIR_RLIB_M1, "membership_150");
    assert_one(EX_DIR_RLIB_M1, "membership_191");
    assert_one(EX_DIR_RLIB_M1, "membership_535");
    assert_one(EX_DIR_RLIB_M1, "membership_829");
    assert_one(EX_DIR_RLIB_M1, "membership_1153");
    assert_one(EX_DIR_RLIB_M1, "membership_1395");
    assert_one(EX_DIR_RLIB_M1, "membership_1568");
    assert_one(EX_DIR_RLIB_M1, "membership_1625");
    assert_one(EX_DIR_RLIB_M1, "membership_1780");
    assert_one(EX_DIR_RLIB_M1, "membership_1919");
}

#[test]
fn test_regexlib_inter() {
    assert_one(EX_DIR_RLIB_INTER1, "intersect_0_0");
    assert_one(EX_DIR_RLIB_INTER1, "intersect_1_5");
    assert_one(EX_DIR_RLIB_INTER2, "intersect_0_3");
    assert_one(EX_DIR_RLIB_INTER2, "intersect_1_4");
    assert_one(EX_DIR_RLIB_INTER2, "intersect_2_8");
    assert_one(EX_DIR_RLIB_INTER2, "intersect_3_6");
    assert_one(EX_DIR_RLIB_INTER2, "intersect_3_8");
    assert_one(EX_DIR_RLIB_INTER2, "intersect_5_7");
    assert_one(EX_DIR_RLIB_INTER2, "intersect_5_8");
}

#[test]
fn test_regex_subset() {
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_0_2");
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_0_9");
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_1_5");
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_0_2");
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_3_7");
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_5_9");
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_6_2");
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_6_9");
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_7_8");
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_9_8");
    assert_one(EX_DIR_RLIB_SUB2, "notsubset_2_2");
    assert_one(EX_DIR_RLIB_SUB2, "notsubset_6_6");
    assert_one(EX_DIR_RLIB_SUB2, "notsubset_0_0");
    assert_one(EX_DIR_RLIB_SUB2, "notsubset_9_9");
}

#[test]
#[ignore]
fn test_regexlib_medium() {
    // Medium-length tests
    assert_one(EX_DIR_RLIB_M1, "membership_518");
    assert_one(EX_DIR_RLIB_M1, "membership_553");
    assert_one(EX_DIR_RLIB_M1, "membership_800");
    assert_one(EX_DIR_RLIB_M1, "membership_807");
    assert_one(EX_DIR_RLIB_M1, "membership_1749");
    assert_one(EX_DIR_RLIB_INTER1, "intersect_6_7");
    assert_one(EX_DIR_RLIB_INTER1, "intersect_6_8");
    assert_one(EX_DIR_RLIB_INTER1, "intersect_8_9");
    assert_one(EX_DIR_RLIB_INTER1, "intersect_7_8");
    assert_one(EX_DIR_RLIB_INTER1, "intersect_8_8");
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_8_7");
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_8_6");
    assert_one(EX_DIR_RLIB_SUB1, "notsubset_7_6");
}

#[test]
#[ignore]
fn test_regexlib_expensive() {
    // Long expensive tests
    assert_one(EX_DIR_RLIB_M1, "membership_854");
}

/*
    Randomly generated examples
*/

#[test]
fn test_random_sparse_10() {
    assert_one(EX_DIR_RANDOM, "sparse_10_1_1");
    assert_one(EX_DIR_RANDOM, "sparse_10_1_2");
    assert_one(EX_DIR_RANDOM, "sparse_10_1_3");
    assert_one(EX_DIR_RANDOM, "sparse_10_1_4");
    assert_one(EX_DIR_RANDOM, "sparse_10_1_5");
    assert_one(EX_DIR_RANDOM, "sparse_10_1_6");
    assert_one(EX_DIR_RANDOM, "sparse_10_1_7");
    assert_one(EX_DIR_RANDOM, "sparse_10_1_8");
    assert_one(EX_DIR_RANDOM, "sparse_10_1_9");
    assert_one(EX_DIR_RANDOM, "sparse_10_1_10");
    assert_one(EX_DIR_RANDOM, "sparse_10_2_1");
    assert_one(EX_DIR_RANDOM, "sparse_10_2_2");
    assert_one(EX_DIR_RANDOM, "sparse_10_2_3");
    assert_one(EX_DIR_RANDOM, "sparse_10_2_4");
    assert_one(EX_DIR_RANDOM, "sparse_10_2_5");
    assert_one(EX_DIR_RANDOM, "sparse_10_2_6");
    assert_one(EX_DIR_RANDOM, "sparse_10_2_7");
    assert_one(EX_DIR_RANDOM, "sparse_10_2_8");
    assert_one(EX_DIR_RANDOM, "sparse_10_2_9");
    assert_one(EX_DIR_RANDOM, "sparse_10_2_10");
    assert_one(EX_DIR_RANDOM, "sparse_10_3_1");
    assert_one(EX_DIR_RANDOM, "sparse_10_3_2");
    assert_one(EX_DIR_RANDOM, "sparse_10_3_3");
    assert_one(EX_DIR_RANDOM, "sparse_10_3_4");
    assert_one(EX_DIR_RANDOM, "sparse_10_3_5");
    assert_one(EX_DIR_RANDOM, "sparse_10_3_6");
    assert_one(EX_DIR_RANDOM, "sparse_10_3_7");
    assert_one(EX_DIR_RANDOM, "sparse_10_3_8");
    assert_one(EX_DIR_RANDOM, "sparse_10_3_9");
    assert_one(EX_DIR_RANDOM, "sparse_10_3_10");
    assert_one(EX_DIR_RANDOM, "sparse_10_10_1");
    assert_one(EX_DIR_RANDOM, "sparse_10_10_2");
    assert_one(EX_DIR_RANDOM, "sparse_10_10_3");
    assert_one(EX_DIR_RANDOM, "sparse_10_10_4");
    assert_one(EX_DIR_RANDOM, "sparse_10_10_5");
    assert_one(EX_DIR_RANDOM, "sparse_10_10_6");
    assert_one(EX_DIR_RANDOM, "sparse_10_10_7");
    assert_one(EX_DIR_RANDOM, "sparse_10_10_8");
    assert_one(EX_DIR_RANDOM, "sparse_10_10_9");
    assert_one(EX_DIR_RANDOM, "sparse_10_10_10");
}

#[test]
fn test_random_sparse_100_1() {
    assert_one(EX_DIR_RANDOM, "sparse_100_1_1");
    assert_one(EX_DIR_RANDOM, "sparse_100_1_2");
    assert_one(EX_DIR_RANDOM, "sparse_100_1_3");
    assert_one(EX_DIR_RANDOM, "sparse_100_1_4");
    assert_one(EX_DIR_RANDOM, "sparse_100_1_5");
    assert_one(EX_DIR_RANDOM, "sparse_100_1_6");
    assert_one(EX_DIR_RANDOM, "sparse_100_1_7");
    assert_one(EX_DIR_RANDOM, "sparse_100_1_8");
    assert_one(EX_DIR_RANDOM, "sparse_100_1_9");
    assert_one(EX_DIR_RANDOM, "sparse_100_1_10");
}

#[test]
fn test_random_sparse_100_2() {
    assert_one(EX_DIR_RANDOM, "sparse_100_2_1");
    assert_one(EX_DIR_RANDOM, "sparse_100_2_2");
    assert_one(EX_DIR_RANDOM, "sparse_100_2_3");
    assert_one(EX_DIR_RANDOM, "sparse_100_2_4");
    assert_one(EX_DIR_RANDOM, "sparse_100_2_5");
    assert_one(EX_DIR_RANDOM, "sparse_100_2_6");
    assert_one(EX_DIR_RANDOM, "sparse_100_2_7");
    assert_one(EX_DIR_RANDOM, "sparse_100_2_8");
    assert_one(EX_DIR_RANDOM, "sparse_100_2_9");
    assert_one(EX_DIR_RANDOM, "sparse_100_2_10");
}

#[test]
fn test_random_sparse_100_3() {
    assert_one(EX_DIR_RANDOM, "sparse_100_3_10");
    assert_one(EX_DIR_RANDOM, "sparse_100_3_1");
    assert_one(EX_DIR_RANDOM, "sparse_100_3_2");
    assert_one(EX_DIR_RANDOM, "sparse_100_3_3");
    assert_one(EX_DIR_RANDOM, "sparse_100_3_4");
    assert_one(EX_DIR_RANDOM, "sparse_100_3_5");
    assert_one(EX_DIR_RANDOM, "sparse_100_3_6");
    assert_one(EX_DIR_RANDOM, "sparse_100_3_7");
    assert_one(EX_DIR_RANDOM, "sparse_100_3_8");
    assert_one(EX_DIR_RANDOM, "sparse_100_3_9");
}

#[test]
fn test_random_sparse_100_10() {
    assert_one(EX_DIR_RANDOM, "sparse_100_10_1");
    assert_one(EX_DIR_RANDOM, "sparse_100_10_2");
    assert_one(EX_DIR_RANDOM, "sparse_100_10_3");
    assert_one(EX_DIR_RANDOM, "sparse_100_10_4");
    assert_one(EX_DIR_RANDOM, "sparse_100_10_5");
    assert_one(EX_DIR_RANDOM, "sparse_100_10_6");
    assert_one(EX_DIR_RANDOM, "sparse_100_10_7");
    assert_one(EX_DIR_RANDOM, "sparse_100_10_8");
    assert_one(EX_DIR_RANDOM, "sparse_100_10_9");
    assert_one(EX_DIR_RANDOM, "sparse_100_10_10");
}

#[test]
#[ignore]
fn test_random_sparse_1000() {
    assert_one(EX_DIR_RANDOM, "sparse_1000_1_1");
    assert_one(EX_DIR_RANDOM, "sparse_1000_1_2");
    assert_one(EX_DIR_RANDOM, "sparse_1000_1_3");
    assert_one(EX_DIR_RANDOM, "sparse_1000_1_4");
    assert_one(EX_DIR_RANDOM, "sparse_1000_1_5");
    assert_one(EX_DIR_RANDOM, "sparse_1000_1_6");
    assert_one(EX_DIR_RANDOM, "sparse_1000_1_7");
    assert_one(EX_DIR_RANDOM, "sparse_1000_1_8");
    assert_one(EX_DIR_RANDOM, "sparse_1000_1_9");
    assert_one(EX_DIR_RANDOM, "sparse_1000_1_10");
    assert_one(EX_DIR_RANDOM, "sparse_1000_2_1");
    assert_one(EX_DIR_RANDOM, "sparse_1000_2_2");
    assert_one(EX_DIR_RANDOM, "sparse_1000_2_3");
    assert_one(EX_DIR_RANDOM, "sparse_1000_2_4");
    assert_one(EX_DIR_RANDOM, "sparse_1000_2_5");
    assert_one(EX_DIR_RANDOM, "sparse_1000_2_6");
    assert_one(EX_DIR_RANDOM, "sparse_1000_2_7");
    assert_one(EX_DIR_RANDOM, "sparse_1000_2_8");
    assert_one(EX_DIR_RANDOM, "sparse_1000_2_9");
    assert_one(EX_DIR_RANDOM, "sparse_1000_2_10");
    assert_one(EX_DIR_RANDOM, "sparse_1000_3_1");
    assert_one(EX_DIR_RANDOM, "sparse_1000_3_2");
    assert_one(EX_DIR_RANDOM, "sparse_1000_3_3");
    assert_one(EX_DIR_RANDOM, "sparse_1000_3_4");
    assert_one(EX_DIR_RANDOM, "sparse_1000_3_5");
    assert_one(EX_DIR_RANDOM, "sparse_1000_3_6");
    assert_one(EX_DIR_RANDOM, "sparse_1000_3_7");
    assert_one(EX_DIR_RANDOM, "sparse_1000_3_8");
    assert_one(EX_DIR_RANDOM, "sparse_1000_3_9");
    assert_one(EX_DIR_RANDOM, "sparse_1000_3_10");
    assert_one(EX_DIR_RANDOM, "sparse_1000_10_1");
    assert_one(EX_DIR_RANDOM, "sparse_1000_10_2");
    assert_one(EX_DIR_RANDOM, "sparse_1000_10_3");
    assert_one(EX_DIR_RANDOM, "sparse_1000_10_4");
    assert_one(EX_DIR_RANDOM, "sparse_1000_10_5");
    assert_one(EX_DIR_RANDOM, "sparse_1000_10_6");
    assert_one(EX_DIR_RANDOM, "sparse_1000_10_7");
    assert_one(EX_DIR_RANDOM, "sparse_1000_10_8");
    assert_one(EX_DIR_RANDOM, "sparse_1000_10_9");
    assert_one(EX_DIR_RANDOM, "sparse_1000_10_10");
}

#[test]
fn test_random_dense_10() {
    assert_one(EX_DIR_RANDOM, "dense_10_2_1");
    assert_one(EX_DIR_RANDOM, "dense_10_2_2");
    assert_one(EX_DIR_RANDOM, "dense_10_2_3");
    assert_one(EX_DIR_RANDOM, "dense_10_2_4");
    assert_one(EX_DIR_RANDOM, "dense_10_2_5");
    assert_one(EX_DIR_RANDOM, "dense_10_2_6");
    assert_one(EX_DIR_RANDOM, "dense_10_2_7");
    assert_one(EX_DIR_RANDOM, "dense_10_2_8");
    assert_one(EX_DIR_RANDOM, "dense_10_2_9");
    assert_one(EX_DIR_RANDOM, "dense_10_2_10");
    assert_one(EX_DIR_RANDOM, "dense_10_3_1");
    assert_one(EX_DIR_RANDOM, "dense_10_3_2");
    assert_one(EX_DIR_RANDOM, "dense_10_3_3");
    assert_one(EX_DIR_RANDOM, "dense_10_3_4");
    assert_one(EX_DIR_RANDOM, "dense_10_3_5");
    assert_one(EX_DIR_RANDOM, "dense_10_3_6");
    assert_one(EX_DIR_RANDOM, "dense_10_3_7");
    assert_one(EX_DIR_RANDOM, "dense_10_3_8");
    assert_one(EX_DIR_RANDOM, "dense_10_3_9");
    assert_one(EX_DIR_RANDOM, "dense_10_3_10");
}

#[test]
fn test_random_dense_100_1() {
    assert_one(EX_DIR_RANDOM, "dense_100_1_1");
    assert_one(EX_DIR_RANDOM, "dense_100_1_2");
    assert_one(EX_DIR_RANDOM, "dense_100_1_3");
    assert_one(EX_DIR_RANDOM, "dense_100_1_4");
    assert_one(EX_DIR_RANDOM, "dense_100_1_5");
    assert_one(EX_DIR_RANDOM, "dense_100_1_6");
    assert_one(EX_DIR_RANDOM, "dense_100_1_7");
    assert_one(EX_DIR_RANDOM, "dense_100_1_8");
    assert_one(EX_DIR_RANDOM, "dense_100_1_9");
    assert_one(EX_DIR_RANDOM, "dense_100_1_10");
}

#[test]
fn test_random_dense_100_2() {
    assert_one(EX_DIR_RANDOM, "dense_100_2_1");
    assert_one(EX_DIR_RANDOM, "dense_100_2_2");
    assert_one(EX_DIR_RANDOM, "dense_100_2_3");
    assert_one(EX_DIR_RANDOM, "dense_100_2_4");
    assert_one(EX_DIR_RANDOM, "dense_100_2_5");
    assert_one(EX_DIR_RANDOM, "dense_100_2_6");
    assert_one(EX_DIR_RANDOM, "dense_100_2_7");
    assert_one(EX_DIR_RANDOM, "dense_100_2_8");
    assert_one(EX_DIR_RANDOM, "dense_100_2_9");
    assert_one(EX_DIR_RANDOM, "dense_100_2_10");
}

#[test]
fn test_random_dense_100_3() {
    assert_one(EX_DIR_RANDOM, "dense_100_3_1");
    assert_one(EX_DIR_RANDOM, "dense_100_3_2");
    assert_one(EX_DIR_RANDOM, "dense_100_3_3");
    assert_one(EX_DIR_RANDOM, "dense_100_3_4");
    assert_one(EX_DIR_RANDOM, "dense_100_3_5");
    assert_one(EX_DIR_RANDOM, "dense_100_3_6");
    assert_one(EX_DIR_RANDOM, "dense_100_3_7");
    assert_one(EX_DIR_RANDOM, "dense_100_3_8");
    assert_one(EX_DIR_RANDOM, "dense_100_3_9");
    assert_one(EX_DIR_RANDOM, "dense_100_3_10");
}

#[test]
#[ignore]
fn test_random_dense_1000_1() {
    assert_one(EX_DIR_RANDOM, "dense_1000_1_1");
    assert_one(EX_DIR_RANDOM, "dense_1000_1_2");
    assert_one(EX_DIR_RANDOM, "dense_1000_1_3");
    assert_one(EX_DIR_RANDOM, "dense_1000_1_4");
    assert_one(EX_DIR_RANDOM, "dense_1000_1_5");
    assert_one(EX_DIR_RANDOM, "dense_1000_1_6");
    assert_one(EX_DIR_RANDOM, "dense_1000_1_7");
    assert_one(EX_DIR_RANDOM, "dense_1000_1_8");
    assert_one(EX_DIR_RANDOM, "dense_1000_1_9");
    assert_one(EX_DIR_RANDOM, "dense_1000_1_10");
}
