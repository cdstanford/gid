#!/usr/bin/env python3.7

"""
Script to generate state graph test cases from regexes.

Uses a handle into Z3 to simplify the regex and generate the derivatives,
then outputs the state graph as an incremental list of updates.
"""

import glob
import os

Z3_PATH = '/home/caleb/git/research/z3/release/z3'
INPUT_DIR = '../benchmarks/explore_derivatives/'
INPUT_EXT = '.smt2'
INPUT_FILE_PATTERN = '**/*.smt2'
OUTPUT_DIR = '/home/caleb/git/research/idse/examples/regex/'
OUTPUT_EXT = '_in.json'

# Clean output directory
os.system(f"rm -rf {OUTPUT_DIR}")
# Process files from input directory to output directory
for filepath in glob.glob(INPUT_DIR + INPUT_FILE_PATTERN, recursive=True):
    outpath = OUTPUT_DIR + filepath.replace(INPUT_DIR, "").replace(INPUT_EXT, OUTPUT_EXT)
    print(f"Processing from {filepath} to {outpath}...")
    # print(os.path.dirname(outpath))
    os.makedirs(os.path.dirname(outpath), exist_ok=True)
    # NOTE: If Z3 hangs, should be able to just ctrl-C to continue
    os.system(f"{Z3_PATH} {filepath} | grep -v \">>>\" > {outpath}")
