# Changelog

All notable changes to the **Rust Mini Projects** monorepo are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- **Project #02 (`fizzbuzz_cli`)**: Implemented customizable FizzBuzz CLI engine with streaming iterators, arbitrary divisor-to-label rules, single number evaluation, and integration test suite.
- **Project #03 (`guessing_game_cli`)**: Implemented interactive number guessing game with difficulty presets (Easy, Medium, Hard, Custom), dynamic parity & quadrant hints, and deterministic state machine.

### Planned
- Incremental implementation of remaining mini projects (Project #04 through #99)
- Benchmark and performance profiling suite for algorithms and data structures

---

## [0.1.0] - 2026-10-10

### Added
- **Monorepo Architecture**: Initialized root Cargo workspace with shared package metadata, strict clippy linter profiles, and release compilation flags.
- **Project #01 (`hello_world_cli`)**: Implemented clean CLI application with custom greeting generation, whitespace handling, and unit test suite.
- **Project #100 (`cosmic_defender_wasm`)**: Integrated high-performance retro-arcade space shooter in Rust compiled to WebAssembly with deterministic physics, particle systems, procedural Web Audio synthesizer, and HTML5 Canvas frontend.
- **CI/CD Quality Gates**: Configured comprehensive GitHub Actions CI matrix verifying formatting (`rustfmt`), linter checks (`clippy -D warnings`), multi-platform compilation, OS test matrix (Ubuntu, Windows, macOS), WASM targets, and security audits (`cargo-audit`).
- **Release Automation**: Multi-platform release packaging workflow building Linux, Windows, and macOS binaries, WASM web distributable packages, consolidated SHA-256 checksums, and automated GitHub Release publishing.
- **Release Drafter & PR Linting**: Configured automated draft release categorization and PR title validation enforcing Conventional Commits.
- **Dependency Automation**: Automated Dependabot checks for Cargo crates and GitHub Actions workflows.
- **Release Runbook**: Added comprehensive Senior Release Manager SOP ([`RELEASE.md`](RELEASE.md)).
