# 🦀 100 Rust Mini Projects

[![CI](https://github.com/raktimneog08-ops/rust-mini-projects/actions/workflows/ci.yml/badge.svg)](https://github.com/raktimneog08-ops/rust-mini-projects/actions/workflows/ci.yml)
[![Release](https://github.com/raktimneog08-ops/rust-mini-projects/actions/workflows/release.yml/badge.svg)](https://github.com/raktimneog08-ops/rust-mini-projects/actions/workflows/release.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.97%2B-orange.svg)](https://www.rust-lang.org)

An enterprise-grade collection of **100 Rust Mini Projects** managed as a unified Cargo monorepo with automated CI/CD quality gates, multi-platform cross-compilation, automated release bundling, and WebAssembly deployment.

---

## 🏗️ Architecture & Monorepo Structure

The monorepo uses a root [Cargo workspace](Cargo.toml) to coordinate build profiles, shared metadata, and dependency trees across all 100 projects.

```
rust-mini-projects/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml               # Multi-platform CI quality gates (fmt, clippy, test, audit)
│   │   ├── release.yml          # Multi-platform binary cross-compilation & release publishing
│   │   ├── release-drafter.yml  # Automated changelog & release drafting
│   │   └── pr-lint.yml          # Conventional Commits PR title validation
│   ├── dependabot.yml           # Automated dependency updates
│   ├── pull_request_template.md # PR submission checklist
│   └── release-drafter.config.yml
├── 01-hello-world/              # Project 01: CLI Greeting Generator
├── ...                          # Projects 02-99: Skeletons & Implementations
├── 100-rust-to-wasm-game/       # Project 100: Cosmic Defender WASM Game
├── Cargo.toml                   # Root Workspace Manifest
├── CHANGELOG.md                 # Keep a Changelog documentation
├── RELEASE.md                   # Release Manager Standard Operating Procedure
└── README.md
```

---

## 📊 Projects Status Overview

| # | Project Name | Directory | Type | Status |
|:---:|:---|:---|:---:|:---:|
| **01** | [Hello World CLI](01-hello-world) | [`01-hello-world/`](01-hello-world/) | CLI | ✅ Complete |
| **02** | [FizzBuzz CLI](02-fizzbuzz) | [`02-fizzbuzz/`](02-fizzbuzz/) | CLI | ✅ Complete |
| **03** | [Guessing Game CLI](03-guessing-game) | [`03-guessing-game/`](03-guessing-game/) | CLI | ✅ Complete |
| **04** | [Temperature Converter](04-temperature-converter) | [`04-temperature-converter/`](04-temperature-converter/) | CLI | ✅ Complete |
| **05** | Fibonacci Generator | [`05-fibonacci-generator/`](05-fibonacci-generator/) | CLI | 📋 Planned |
| ... | *Projects 06 - 99* | ... | Varied | 📋 Planned |
| **100** | [Cosmic Defender WASM](100-rust-to-wasm-game) | [`100-rust-to-wasm-game/`](100-rust-to-wasm-game/) | WASM Game | ✅ Complete |

---

## 🚀 Quick Start

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (1.97+)
- WebAssembly Target (for WASM crates):
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

### Build & Run
```bash
# Check all workspace crates
cargo check --workspace

# Run tests across all crates
cargo test --workspace

# Run Project 01 (Hello World CLI)
cargo run -p hello_world_cli -- Ferris

# Serve Project 100 (Cosmic Defender WASM Game)
cd 100-rust-to-wasm-game
python serve.py
# Open http://localhost:8000 in your browser
```

---

## 🛠️ CI/CD Quality Gates

Every pull request and merge to `main` must pass strict automated gates:
1. **Formatting**: `cargo fmt --all -- --check`
2. **Clippy Linting**: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. **WASM Clippy**: `cargo clippy --target wasm32-unknown-unknown -p cosmic_defender_wasm -- -D warnings`
4. **Compilation**: `cargo check --workspace --all-targets`
5. **Cross-Platform Tests**: Matrix execution on `ubuntu-latest`, `windows-latest`, and `macos-latest`.
6. **Security Audit**: `cargo-audit` scanning against known RustSec advisories.

---

## 📦 Release Management

Releases are automated via GitHub Actions on annotated Git tags matching `v*.*.*`:
- **Binaries**: Packaged for Linux x86_64, Windows x86_64, macOS Intel, and macOS Apple Silicon.
- **WASM Assets**: Pre-compiled WebAssembly game and HTML5 canvas bundle.
- **Integrity**: Cryptographic `SHA256SUMS.txt` generated for all distribution assets.

For release lifecycle steps, hotfixes, and rollback procedures, refer to [`RELEASE.md`](RELEASE.md).

---

## 📄 License

Dual-licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.
