/*
    Binary to generate examples in examples/ that can be run.

    I am using the examples both for unit testing and performance analysis.
*/

use guided_digraph::constants::{EX_DIR_GENERATED, EX_DIR_RANDOM};
use guided_digraph::example::{Example, ExampleInput, ExampleOutput};
use guided_digraph::interface::Transaction;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::fmt::Display;

/*
    Utility functions
*/

fn paramed_example<P: Display>(
    basename: &str,
    param: P,
    ex_in: ExampleInput,
    expect: ExampleOutput,
) -> Example {
    let pathname = format!("{}/{}_{}", EX_DIR_GENERATED, basename, param);
    println!("created {}", pathname);
    Example::new(&pathname, ex_in, Some(expect))
}

fn random_example<P: Display>(
    basename: &str,
    params: &[P],
    seed: u64,
    ex_in: ExampleInput,
) -> Example {
    let params: Vec<String> = params.iter().map(|s| s.to_string()).collect();
    let params = params.join("_");
    let pathname =
        format!("{}/{}_{}_{}", EX_DIR_RANDOM, basename, params, seed);
    println!("created {}", pathname);
    Example::new(&pathname, ex_in, None)
}

/*
    Structured example generators
    (for specific classes of graphs)
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

fn gen_complete(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        for j in 0..n {
            if i != j {
                ex_in.push(Transaction::Add(i, j));
            }
        }
        ex_in.push(Transaction::Close(i));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: (0..n).collect(),
        unknown: vec![],
        open: vec![],
    };
    paramed_example("complete", n, ex_in, expect)
}

fn gen_completeunk(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        for j in 0..=n {
            if i != j {
                ex_in.push(Transaction::Add(i, j));
            }
        }
        ex_in.push(Transaction::Close(i));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: vec![],
        unknown: (0..n).collect(),
        open: vec![n],
    };
    paramed_example("unkcomplete", n, ex_in, expect)
}

fn gen_completeacyclic(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        for j in (i + 1)..n {
            ex_in.push(Transaction::Add(i, j));
        }
        ex_in.push(Transaction::Close(i));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: (0..n).collect(),
        unknown: vec![],
        open: vec![],
    };
    paramed_example("completeacyclic", n, ex_in, expect)
}

fn gen_completeacyclicunk(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..n {
        for j in (i + 1)..=n {
            ex_in.push(Transaction::Add(i, j));
        }
        ex_in.push(Transaction::Close(i));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: vec![],
        unknown: (0..n).collect(),
        open: vec![n],
    };
    paramed_example("unkcompleteacyclic", n, ex_in, expect)
}

fn gen_completeacyclicrev(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in (0..n).rev() {
        for j in (i + 1)..n {
            ex_in.push(Transaction::Add(i, j));
        }
        ex_in.push(Transaction::Close(i));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: (0..n).collect(),
        unknown: vec![],
        open: vec![],
    };
    paramed_example("revcompleteacyclic", n, ex_in, expect)
}

fn gen_completeacyclicunkrev(n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in (0..n).rev() {
        for j in (i + 1)..=n {
            ex_in.push(Transaction::Add(i, j));
        }
        ex_in.push(Transaction::Close(i));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: vec![],
        unknown: (0..n).collect(),
        open: vec![n],
    };
    paramed_example("unkrevcompleteacyclic", n, ex_in, expect)
}

fn gen_bipartite(m: usize, n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..m {
        for j in m..(m + n) {
            ex_in.push(Transaction::Add(i, j));
        }
        ex_in.push(Transaction::Close(i));
    }
    for j in m..(m + n) {
        ex_in.push(Transaction::Close(j));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: (0..(m + n)).collect(),
        unknown: vec![],
        open: vec![],
    };
    let mn = format!("{}_{}", m, n);
    paramed_example("bipartite", mn, ex_in, expect)
}

fn gen_unkbipartite(m: usize, n: usize) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    for i in 0..m {
        for j in m..=(m + n) {
            ex_in.push(Transaction::Add(i, j));
        }
        ex_in.push(Transaction::Close(i));
    }
    for j in m..(m + n) {
        ex_in.push(Transaction::Close(j));
    }
    let expect = ExampleOutput {
        live: vec![],
        dead: (m..(m + n)).collect(),
        unknown: (0..m).collect(),
        open: vec![m + n],
    };
    let mn = format!("{}_{}", m, n);
    paramed_example("unkbipartite", mn, ex_in, expect)
}

/*
    Random example generators
*/

/*
    Generate an example with constant outdegree deg:
    - vertices 0 through n-1 are closed, n is open
    - 0 through n-1 have 'deg' random out-edges
*/
fn random_sparse(n: usize, deg: usize, seed: u64) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    let mut rng = StdRng::seed_from_u64(seed);

    for u in 0..n {
        for _ in 0..deg {
            // Note that u == v is possible
            let v = rng.gen_range(0..=n);
            ex_in.push(Transaction::Add(u, v));
        }
        ex_in.push(Transaction::Close(u));
    }

    random_example("sparse", &[n, deg], seed, ex_in)
}

/*
    Generate an example with presence/existence of an edge chosen
    independently at random for each vertex pair --
    i.e. the Erdős–Rényi–Gilbert random graph model.

    Probably is given as an integer percent between 0 and 100.
    As before, vertices 0 through n-1 are closed, n is open.
*/
fn random_dense(n: usize, p: usize, seed: u64) -> Example {
    let mut ex_in = ExampleInput(vec![]);
    let mut rng = StdRng::seed_from_u64(seed);

    for u in 0..n {
        for v in 0..=n {
            // Note that u == v is possible
            if rng.gen_range(0..100) < p {
                ex_in.push(Transaction::Add(u, v));
            }
        }
        ex_in.push(Transaction::Close(u));
    }

    random_example("dense", &[n, p], seed, ex_in)
}

/*
    Entrypoint
*/

fn main() {
    // Generate and save parameterized examples
    for &i in &[3, 10, 30, 100, 300, 1000, 3000, 10000, 30000, 100000] {
        gen_line(i).save();
        gen_reverseline(i).save();
        gen_unkline(i).save();
        gen_reverseunkline(i).save();
        gen_loop(i).save();
        gen_unkloop(i).save();
        gen_reverseloop(i).save();
        gen_reverseunkloop(i).save();
    }
    for &i in &[3, 10, 30, 100, 300, 1000] {
        gen_complete(i).save();
        gen_completeunk(i).save();
        gen_completeacyclic(i).save();
        gen_completeacyclicunk(i).save();
        gen_completeacyclicrev(i).save();
        gen_completeacyclicunkrev(i).save();
        gen_bipartite(i, i).save();
        gen_unkbipartite(i, i).save();
        gen_bipartite(i, 10).save();
        gen_unkbipartite(i, 10).save();
        gen_bipartite(10, i).save();
        gen_unkbipartite(10, i).save();
    }
    // Generate and save random examples
    // Use random seeds 1-10
    for &n in &[10, 30, 100, 300, 1000, 3000, 10000, 30000, 100000] {
        for &d in &[1, 2, 3, 10] {
            for seed in 1..=10 {
                random_sparse(n, d, seed).save();
            }
        }
    }
    for &n in &[10, 30, 100, 300, 1000, 3000, 10000] {
        for &d in &[1, 2, 3] {
            for seed in 1..=10 {
                random_dense(n, d, seed).save();
            }
        }
    }
}
