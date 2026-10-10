# Project 02: FizzBuzz CLI

> An extensible, high-performance FizzBuzz engine and CLI in Rust supporting standard rules, range evaluations, streaming iterators, and custom modular divisor rules.

---

## 🚀 Features

- **Standard Rules**:
  - Multiples of 3 $\rightarrow$ `Fizz`
  - Multiples of 5 $\rightarrow$ `Buzz`
  - Multiples of 15 $\rightarrow$ `FizzBuzz`
- **Streaming Iterator**: `fizzbuzz_iter()` for zero-allocation streaming over unbounded ranges.
- **Custom Modular Rules**: Pass any arbitrary divisor-to-label mappings (e.g. `3:Fizz, 5:Buzz, 7:Bazz`).
- **Flexible CLI Interface**:
  - Run with default range (1..=100)
  - Run up to a specified limit
  - Run a specific `[START] [END]` range
  - Evaluate a single number with `--number`
  - Supply dynamic rules via `--custom`
- **100% Test Coverage**: Unit tests and integration tests covering edge cases and boundary conditions.

---

## 📦 Usage

### Run Standard (1 to 100)
```bash
cargo run -p fizzbuzz_cli
```

### Run Custom Range (1 to 20)
```bash
cargo run -p fizzbuzz_cli -- 20
# Or
cargo run -p fizzbuzz_cli -- 1 20
```

### Evaluate Single Number
```bash
cargo run -p fizzbuzz_cli -- --number 15
# Output: FizzBuzz
```

### Run with Custom Modular Rules (e.g. Fizz, Buzz, Bazz)
```bash
cargo run -p fizzbuzz_cli -- --custom 35 "3:Fizz,5:Buzz,7:Bazz"
```

---

## 🧪 Testing

```bash
cargo test -p fizzbuzz_cli
```
