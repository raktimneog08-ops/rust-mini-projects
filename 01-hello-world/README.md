# Project 01: Hello World CLI

> A clean, idiomatic Rust CLI application demonstrating greeting formatting, command-line arguments, and unit tests.

---

## 🚀 Features

- Configurable greeting via CLI arguments (`cargo run -- Ferris`)
- Handles blank/whitespace arguments with safe fallback
- Built-in unit test coverage for edge cases

## 📦 Usage

### Run Default
```bash
cargo run -p hello_world_cli
```
Output:
```
Hello, World! Welcome to Rust Mini Projects.
```

### Run with Custom Name
```bash
cargo run -p hello_world_cli -- Ferris
```
Output:
```
Hello, Ferris! Welcome to Rust Mini Projects.
```

### Run Tests
```bash
cargo test -p hello_world_cli
```
