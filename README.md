# Incremental Dead State Detection in Logarithmic Time

Welcome! This repository contains the artifact for the paper *Incremental Dead State Detection in Logarithmic Time,* to appear at CAV 2023.
See `cav23.pdf` for the version of the paper at submission time.
The artifact can also be viewed online [here](https://github.com/cdstanford/gid).

## Brief Overview

The artifact provides a data structure, called guided incremental digraphs (GIDs), implemented as an open-source library in Rust.
The data structure solves incremental live and dead state detection in an abstract transition system.

More precisely: given as input a list of graph updates like
```
Edge(1, 2)
Edge(2, 3)
Close(2)
Edge(3, 4)
Close(4)
Terminal(2)
```
the problem is to produce as output a list of *live* and *dead* states (vertices) in the transition system.
The meaning of this terminology is as follows:
- In the input, some states are labeled as "closed" meaning they are considered fully explored: no further outgoing edges will be added.
- In the input, some state are labeled as "terminal", meaning they are accepting states.
- In the output, a "live" state is one for which there is a path to a terminal state.
- In the output, a "dead" state is one for which all reachable states are closed and not terminal. In other words, a dead state is one that is known to not be live no matter what future edges are added to the transition system.

In this example, `1` and `2` should be live and `4` should be dead.

The repository implements five competing solutions to the above problem:
- our amortized logarithmic-time per update algorithm (Algorithm 2, `log`)
- an even faster lazy algorithm (Algorithm 3, `jump`), and
- three baseline approaches for comparison: `bfgt` (the state of the art), `naive`, and `simple`.

## List of claims supported

The artifact supports the experimental results section of the paper (Section 4), summarized in Figures 4 and 5. In particular, the artifact supports the following claims:

- Algorithm 3 (`jump`) shows significant improvements over the state-of-the-art, solving more benchmarks in a smaller amount of time across basic GIDs, random GIDs, and regex GIDs.
- Algorithm 2 (`log`) also shows state-of-the-art performance, similar to BFGT on basic and regex GIDs and significantly better on random GIDs.
- In particular, Algorithm 3 exhibits up to two orders of magnitude speedup over BFGT for larger GIDs – we measured speedups of 110x to 530x for GIDs in the top five size buckets (GIDs of size nearest to 100K, 200K, 500K, 1M, and 2M).
- Both algorithms, and our implementations of all the baseline approaches, exhibit correct output on all unit tests and benchmarks.
