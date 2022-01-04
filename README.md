# Collatz
Computation and visualization of collatz paths using Rust and Python

## Rust
This is mainly a Rust playground project for me to learn some fundamentals.

I have implemented the [Collatz Conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture) in multiple ways:
* Naive: Just compute it
* Memoized: Store collatz paths in a graph data structure and store paths in it

I have validated and benchmarked these approaches using [Euler problem 14](https://projecteuler.net/problem=14).

### Naive
The naive approach just computes

![collatz](https://wikimedia.org/api/rest_v1/media/math/render/svg/ec22031bdc2a1ab2e4effe47ae75a836e7dea459)

until $f(n) == 1$ (assuming the conjecture is true of course).

When benchmarking this approach, I was already blown away with the performance of Rust (on my macbook pro).
Computing all paths for all $n \in [1, 1.000.000]$ took roughly 5 seconds.

### Memoized
Instead of computing the next value for every $n$, we store the path in a graph data structure.
For this, I used the [`petgraph`](https://github.com/petgraph/petgraph) crate.
More specifically, I used a directed graph map.

This implementation is as follows:
1. Get all neighbours of $n$ (which is at-most $1$)
2. If there is a neighbour - return it
  1. Otherwise, add the $n$ to the graph
  2. Compute $f(n)$ and insert it into the graph
  3. Insert the resulting edge into the graph
  4. Return $f(n)$

The obvious drawback of this approach is that, to be faster, the computation of $f(n)$ must be slower than the neighbour lookup overhead.

If we look at the benchmark, we see that this is not the case and it's almost four times slower.
Now, I expected as much since the computation doesn't really take long and this memoization method is quite naive.

It would be way faster if I would add an edge to $1$ with the path length as the edge weight. 
It might be even faster if I was to only store the path length per $n$.
However, for visualization purposes (which is why this type of memoization was chosen in the first place) this wasn't done.

### Python
As a comparison, I also implemented a very simple approach in Python, my programming language of choice.
Comparing these approaches revealed that (of course), Rust is a lot faster.
Even the memoized approach is almost twice as fast.

### Benchmark

| Approach                              | Timing (s) |
| ------------------------------------- |:----------:|
| [Naive](./src/naive.rs)               | 5          |
| [Memoized - Fresh](./src/memoized.rs) | 19         |
| [Memoized - Rerun](./src/memoized.rs) | 18         |
| [Python](./src/naive.py)              | 34         |
| [Python LRU-cache](./src/naive.py)    | 27         |
