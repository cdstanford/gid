/*
    Binary to generate examples in examples/ that can be run.

    I am using the examples both for unit testing and performance analysis.
*/

use state_graph::interface::{
    Example, ExampleInput, ExampleOutput, Transaction,
};
use std::fmt::Display;

/*
    Utility functions
*/

const GEN_DIRECTORY: &str = "examples/generated";

fn paramed_example<P: Display>(
    basename: &str,
    param: P,
    ex_in: ExampleInput,
    expect: ExampleOutput,
) -> Example {
    let basename = format!("{}/{}_{}", GEN_DIRECTORY, basename, param);
    Example::new(&basename, ex_in, Some(expect))
}

/*
    Example generators
*/

fn gen_line(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        ex_in.push(Transaction::Add(i, i + 1));
        ex_in.push(Transaction::Close(i));
    }
    ex_in.push(Transaction::Close(n));
    let expect = ExampleOutput {
        live: vec![],
        dead: (0..(n + 1)).collect(),
        unknown: vec![],
        open: vec![],
    };
    paramed_example("line", n, ex_in, expect)
}

fn gen_unkline(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        ex_in.push(Transaction::Add(i, i + 1));
        ex_in.push(Transaction::Close(i));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: vec![],
        unknown: (0..n).collect(),
        open: vec![n],
    };
    paramed_example("unkline", n, ex_in, expect)
}

fn gen_reverseline(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in (0..n).rev() {
        ex_in.push(Transaction::Add(i, i + 1));
        ex_in.push(Transaction::Close(i));
    }
    ex_in.push(Transaction::Close(n));
    let expect = ExampleOutput {
        live: vec![],
        dead: (0..(n + 1)).collect(),
        unknown: vec![],
        open: vec![],
    };
    paramed_example("reverseline", n, ex_in, expect)
}

fn gen_reverseunkline(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in (0..n).rev() {
        ex_in.push(Transaction::Add(i, i + 1));
        ex_in.push(Transaction::Close(i));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: vec![],
        unknown: (0..n).collect(),
        open: vec![n],
    };
    paramed_example("reverseunkline", n, ex_in, expect)
}

fn gen_loop(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        ex_in.push(Transaction::Add(i, (i + 1) % n));
        ex_in.push(Transaction::Close(i));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: (0..n).collect(),
        unknown: vec![],
        open: vec![],
    };
    paramed_example("loop", n, ex_in, expect)
}

fn gen_unkloop(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        ex_in.push(Transaction::Add(i, (i + 1) % n));
        if i == 0 {
            ex_in.push(Transaction::Add(i, n));
        }
        ex_in.push(Transaction::Close(i));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: vec![],
        unknown: (0..n).collect(),
        open: vec![n],
    };
    paramed_example("unkloop", n, ex_in, expect)
}

fn gen_reverseloop(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in (0..n).rev() {
        ex_in.push(Transaction::Add(i, (i + 1) % n));
        ex_in.push(Transaction::Close(i));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: (0..n).collect(),
        unknown: vec![],
        open: vec![],
    };
    paramed_example("reverseloop", n, ex_in, expect)
}

fn gen_reverseunkloop(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in (0..n).rev() {
        ex_in.push(Transaction::Add(i, (i + 1) % n));
        if i == 0 {
            ex_in.push(Transaction::Add(i, n));
        }
        ex_in.push(Transaction::Close(i));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: vec![],
        unknown: (0..n).collect(),
        open: vec![n],
    };
    paramed_example("reverseunkloop", n, ex_in, expect)
}

/*
    Entrypoint
*/

fn main() {
    for &i in &[3, 10, 20, 100, 1000, 10000] {
        gen_line(i).save();
        gen_reverseline(i).save();
        gen_unkline(i).save();
        gen_reverseunkline(i).save();
        gen_loop(i).save();
        gen_unkloop(i).save();
        gen_reverseloop(i).save();
        gen_reverseunkloop(i).save();
    }
}
