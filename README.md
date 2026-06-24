# Two Sets

A Rust implementation of the CSES Two Sets problem using two different algorithms.

## Problem Description

Given an integer n, divide the numbers from 1 to n into two sets with equal sum if such a division is possible.

If the total sum cannot be divided equally, output that no solution exists.

## Algorithm 1: Greedy Approach

### Description

The greedy algorithm first computes the target sum, which is half of the total sum of numbers from 1 to n.

Starting from n and moving downward, the algorithm selects the largest possible value that does not exceed the remaining target sum.

This process continues until the target sum becomes zero.

### Time Complexity

O(n)

### Space Complexity

O(n)

### Advantages

* Optimal runtime
* Simple implementation
* Excellent cache locality
* Scales efficiently for large inputs

---

## Algorithm 2: Naive Approach

### Description

The naive algorithm iteratively searches for values that contribute toward the target sum using repeated checks and comparisons.

It reaches the same solution but performs more work than necessary.

### Time Complexity

O(n²)

### Space Complexity

O(n)

### Advantages

* Easy to understand
* Useful as a reference implementation

### Disadvantages

* Additional iterations
* Poor scalability for larger inputs

---

## Benchmark Results

| Algorithm | Execution Time |
| --------- | -------------- |
| Greedy    | 1.01 ns        |
| Naive     | 29.33 ns       |

---

## Benchmark Interpretation

Both algorithms correctly determine whether the numbers from 1 to n can be partitioned into two sets of equal sum.

The benchmark results show a substantial performance difference between the implementations.

The greedy algorithm completes execution in approximately 1.01 ns, while the naive algorithm requires approximately 29.33 ns.

This means the greedy implementation is roughly 29 times faster.

### Assessment of Time Complexity

Greedy Algorithm:

* O(n)

Naive Algorithm:

* O(n²)

The greedy approach processes each value at most once and makes direct decisions based on the remaining target sum.

The naive implementation performs additional repeated operations and comparisons, increasing computational cost.

### Assessment of Space Complexity

Greedy Algorithm:

* O(n)

Naive Algorithm:

* O(n)

Both algorithms store the generated subsets and therefore require memory proportional to the number of selected elements.

### Memory Impact

Memory consumption is similar for both implementations because both store the resulting partitions in vectors.

No large auxiliary data structures are allocated.

### Cache Behaviour

The greedy implementation exhibits better cache locality because it performs sequential access patterns and fewer instructions.

The naive implementation performs additional iterations and repeated checks, increasing instruction count and reducing cache efficiency.

As input size grows, the performance gap becomes increasingly significant.

### Conclusion

The greedy algorithm is the preferred solution.

It achieves linear runtime, efficient memory usage, superior cache behaviour, and significantly better benchmark performance.

The naive implementation remains valuable for comparison and validation purposes but is not suitable for large-scale inputs.

---

## Project Structure

```text
two_sets/
├── src/
│   ├── lib.rs
│   └── main.rs
├── tests/
│   └── two_sets_tests.rs
├── benches/
│   └── benchmark.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Running the Program

```bash
cargo run
```

## Running Tests

```bash
cargo test
```

## Running Benchmarks

```bash
cargo bench
```
