/*
    Binary to generate examples in examples/ that can be run.

    I am using the examples both for unit testing and performance analysis.
*/

use state_graph::interface::{
    Example, ExampleInput, ExampleOutput, Transaction,
};

fn gen_line(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        ex_in.push(Transaction::Add(i, i + 1));
        ex_in.push(Transaction::Done(i));
    }
    ex_in.push(Transaction::Done(n));
    let expect = ExampleOutput {
        dead: (0..(n + 1)).collect(),
        unknown: vec![],
        unvisited: vec![],
    };
    Example(format!("line_{}", n), ex_in, expect)
}

fn gen_liveline(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        ex_in.push(Transaction::Add(i, i + 1));
        ex_in.push(Transaction::Done(i));
    }
    let expect = ExampleOutput {
        dead: vec![],
        unknown: (0..n).collect(),
        unvisited: vec![n],
    };
    Example(format!("liveline_{}", n), ex_in, expect)
}

fn gen_reverseline(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in (0..n).rev() {
        ex_in.push(Transaction::Add(i, i + 1));
        ex_in.push(Transaction::Done(i));
    }
    ex_in.push(Transaction::Done(n));
    let expect = ExampleOutput {
        dead: (0..(n + 1)).collect(),
        unknown: vec![],
        unvisited: vec![],
    };
    Example(format!("reverseline_{}", n), ex_in, expect)
}

fn gen_reverseliveline(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in (0..n).rev() {
        ex_in.push(Transaction::Add(i, i + 1));
        ex_in.push(Transaction::Done(i));
    }
    let expect = ExampleOutput {
        dead: vec![],
        unknown: (0..n).collect(),
        unvisited: vec![n],
    };
    Example(format!("reverseliveline_{}", n), ex_in, expect)
}

fn gen_loop(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        ex_in.push(Transaction::Add(i, (i + 1) % n));
        ex_in.push(Transaction::Done(i));
    }
    let expect = ExampleOutput {
        dead: (0..n).collect(),
        unknown: vec![],
        unvisited: vec![],
    };
    Example(format!("loop_{}", n), ex_in, expect)
}

fn gen_liveloop(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        ex_in.push(Transaction::Add(i, (i + 1) % n));
        if i == 0 {
            ex_in.push(Transaction::Add(i, n));
        }
        ex_in.push(Transaction::Done(i));
    }
    let expect = ExampleOutput {
        dead: vec![],
        unknown: (0..n).collect(),
        unvisited: vec![n],
    };
    Example(format!("liveloop_{}", n), ex_in, expect)
}

fn gen_reverseloop(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in (0..n).rev() {
        ex_in.push(Transaction::Add(i, (i + 1) % n));
        ex_in.push(Transaction::Done(i));
    }
    let expect = ExampleOutput {
        dead: (0..n).collect(),
        unknown: vec![],
        unvisited: vec![],
    };
    Example(format!("reverseloop_{}", n), ex_in, expect)
}

fn gen_reverseliveloop(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in (0..n).rev() {
        ex_in.push(Transaction::Add(i, (i + 1) % n));
        if i == 0 {
            ex_in.push(Transaction::Add(i, n));
        }
        ex_in.push(Transaction::Done(i));
    }
    let expect = ExampleOutput {
        dead: vec![],
        unknown: (0..n).collect(),
        unvisited: vec![n],
    };
    Example(format!("reverseliveloop_{}", n), ex_in, expect)
}

fn main() {
    for &i in &[3, 10, 20, 100, 1000, 10000] {
        gen_line(i).save();
        gen_reverseline(i).save();
        gen_liveline(i).save();
        gen_reverseliveline(i).save();
        gen_loop(i).save();
        gen_liveloop(i).save();
        gen_reverseloop(i).save();
        gen_reverseliveloop(i).save();
    }
}
