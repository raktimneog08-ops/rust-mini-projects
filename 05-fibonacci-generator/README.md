# Project 05: Fibonacci Generator & Mathematical Analytics CLI

> A high-performance Fibonacci sequence engine and CLI in pure Rust featuring zero-dependency arbitrary-precision arithmetic (`BigUint`), 6 algorithmic evaluation strategies, sequence iterators, and mathematical analytics.

---

## 🌀 Features

- **Zero-Dependency Arbitrary Precision (`BigUint`)**:
  - Implements multi-limb base-$10^9$ big integer arithmetic in pure standard library Rust.
  - Computes massive Fibonacci terms (e.g. $F(1000)$, $F(10000)$) without numeric overflow or third-party dependencies.
  - Seamlessly handles native primitive fast-paths (`u64` up to $F(93)$, `u128` up to $F(186)$) with strict overflow guardrails.

- **6 Algorithmic Strategies**:
  - **Fast Doubling ($O(\log n)$)**: High-speed logarithmic evaluation using doubling identities.
  - **Iterative Accumulator ($O(n)$)**: Minimal memory footprint $O(1)$ space loop.
  - **$2 \times 2$ Matrix Exponentiation ($O(\log n)$)**: Fast binary matrix power $\begin{pmatrix} 1 & 1 \\ 1 & 0 \end{pmatrix}^n$.
  - **Dynamic Programming Memoization ($O(n)$)**: Top-down cached recurrence table.
  - **Binet's Analytic Closed Form ($O(1)$)**: Direct floating-point closed form using $\phi = \frac{1+\sqrt{5}}{2}$ ($n \le 78$).
  - **Naive Recursive ($O(2^n)$)**: Classical branching recursion with safety ceiling ($n \le 40$) to prevent CPU hangs.

- **Mathematical Analytics & Tools**:
  - **Fibonacci Membership Verification**: Tests if any number $x$ belongs to the sequence and retrieves its index $F(n)$.
  - **Golden Ratio ($\phi$) Convergence**: Formatted table analyzing the limit $\lim_{n \to \infty} \frac{F(n+1)}{F(n)} = \phi \approx 1.6180339887...$.
  - **Pisano Period Calculator**: Computes the exact cycle length $\pi(m)$ of the Fibonacci sequence modulo $m$.
  - **Primality & Sequence Analytics**: Identifies prime Fibonacci terms and sequence properties.
  - **Multi-Algorithm Benchmarker**: Micro-benchmark comparing execution times and digit counts across all algorithms for a given $n$.

- **Interactive Shell & Unix-Friendly CLI**:
  - Full-featured interactive terminal shell with menu-driven navigation.
  - Composable CLI flags for batch pipelines and scripting.

---

## 🧮 Algorithmic Complexity Comparison

| Algorithm | Time Complexity | Space Complexity | Best Suited For | Maximum Safe $n$ |
| :--- | :---: | :---: | :--- | :---: |
| **Fast Doubling** | $\mathcal{O}(\log n)$ | $\mathcal{O}(\log n)$ | Massive terms ($n > 1,000$) | Arbitrary (RAM bound) |
| **Matrix Exponentiation** | $\mathcal{O}(\log n)$ | $\mathcal{O}(1)$ | Algebraic proofs & fast evaluation | Arbitrary (RAM bound) |
| **Iterative Accumulator** | $\mathcal{O}(n)$ | $\mathcal{O}(1)$ | General sequential generation | Arbitrary (RAM bound) |
| **Memoized (DP)** | $\mathcal{O}(n)$ | $\mathcal{O}(n)$ | Repeated queries in memory | $n \le 50,000$ |
| **Binet's Closed Form** | $\mathcal{O}(1)$ | $\mathcal{O}(1)$ | Instant estimate | $n \le 78$ (IEEE 754 precision) |
| **Naive Recursive** | $\mathcal{O}(2^n)$ | $\mathcal{O}(n)$ | Educational tree recursion | $n \le 40$ (guarded) |

---

## 📦 Usage

### 1. Calculate N-th Fibonacci Term
```bash
# Fast evaluation using default Fast Doubling algorithm
cargo run -p fibonacci_generator_cli -- 100

# Specify an alternative algorithmic strategy
cargo run -p fibonacci_generator_cli -- 50 --algo matrix
cargo run -p fibonacci_generator_cli -- 35 --algo recursive
```

### 2. Generate Sequence Table
```bash
# Print first 20 Fibonacci terms with digit counts
cargo run -p fibonacci_generator_cli -- --seq 20
```

### 3. Check Fibonacci Membership
```bash
# Test if a number is in the sequence and find its index
cargo run -p fibonacci_generator_cli -- --is-fib 6765
# Output: ✓ 6765 is a Fibonacci number! (Index: F(20))

cargo run -p fibonacci_generator_cli -- --is-fib 100
# Output: ✗ 100 is NOT a Fibonacci number.
```

### 4. Golden Ratio Convergence Table
```bash
# Inspect F(n+1)/F(n) convergence toward phi
cargo run -p fibonacci_generator_cli -- --ratio 15
```

### 5. Calculate Pisano Period
```bash
# Calculate cycle length of F(n) modulo 10
cargo run -p fibonacci_generator_cli -- --pisano 10
# Output: Pisano period π(10) = 60
```

### 6. Benchmark All Algorithms
```bash
# Profile execution speed across all 6 strategies at n=35
cargo run -p fibonacci_generator_cli -- --benchmark 35
```

### 7. Interactive Terminal Shell
```bash
# Launch interactive menu
cargo run -p fibonacci_generator_cli
```

---

## 🧪 Testing & Verification

```bash
# Check code style, types, and compiler warnings
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets
cargo fmt --check

# Run test suite
cargo test -p fibonacci_generator_cli
```
