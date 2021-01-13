/*
    Binary to generate examples in examples/ that can be run.

    I am using the examples both for unit testing and performance analysis.
*/

use state_graph::driver::{self};
use state_graph::interface::{ExampleInput, ExampleOutput, Transaction};

struct Example(String, ExampleInput, ExampleOutput);
impl Example {
    fn save(&self) {
        driver::to_json_file(driver::infile_from_prefix(&self.0), &self.1);
        driver::to_json_file(
            driver::expectedfile_from_prefix(&self.0),
            &self.2,
        );
    }
}

fn gen_line(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        ex_in.0.push(Transaction::Add(i, i + 1));
        ex_in.0.push(Transaction::Done(i));
    }
    ex_in.0.push(Transaction::Done(n));
    let expect = ExampleOutput {
        dead: (0..(n + 1)).collect(),
        unknown: vec![],
        unvisited: vec![],
    };
    Example(format!("line_{}", n), ex_in, expect)
}

fn main() {
    gen_line(10).save();
}
