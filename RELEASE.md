# Release Management Runbook

> **Role & Authority**: Autonomous Senior Software Engineer & GitHub Release Manager  
> **Repository**: [Rust Mini Projects](https://github.com/raktimneog08-ops/rust-mini-projects)

This runbook documents the standard release lifecycle, versioning contract, artifact verification, and emergency procedures for the Rust Mini Projects monorepo.

---

## 1. Versioning Contract (SemVer 2.0.0)

We strictly follow [Semantic Versioning (SemVer 2.0.0)](https://semver.org/):

$$\text{MAJOR}.\text{MINOR}.\text{PATCH}$$

- **MAJOR (`X.0.0`)**: Incompatible API breaking changes, CLI interface breaking shifts, or workspace architecture overhauls.
- **MINOR (`0.Y.0` / `1.Y.0`)**: Backwards-compatible additions, such as completing new mini project implementations or new game/CLI modules.
- **PATCH (`0.0.Z` / `1.0.Z`)**: Backwards-compatible bug fixes, performance optimizations, or internal refactoring.

---

## 2. Release Pipeline Overview

```
[Feature Branch] ──> [PR to main] ──> [CI Quality Gates] ──> [Squash & Merge]
                                                                     │
                                                           [Draft Release Updated]
                                                                     │
                                                         [Create Tag: vX.Y.Z]
                                                                     │
                                                   ┌─────────────────┴─────────────────┐
                                                   ▼                                   ▼
                                        [Multi-Platform Matrix]             [WASM Web Bundle]
                                        - Linux x86_64 (.tar.gz)            - HTML5 + Canvas
                                        - Windows x86_64 (.zip)             - Web Audio Synthesizer
                                        - macOS x86_64 (.tar.gz)            - cosmic_defender_wasm.wasm
                                        - macOS aarch64 (.tar.gz)                      │
                                                   │                                   │
                                                   └─────────────────┬─────────────────┘
                                                                     ▼
                                                      [Compute SHA-256 Checksums]
                                                                     │
                                                                     ▼
                                                     [Publish GitHub Release Assets]
```

---

## 3. Release Lifecycle & Step-by-Step Procedure

### Phase 1: Pre-Release Verification (Local Gate)
Before triggering any release:
```bash
# 1. Format check
cargo fmt --all -- --check

# 2. Lint check with zero warnings
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo clippy --target wasm32-unknown-unknown -p cosmic_defender_wasm -- -D warnings

# 3. Compilation check
cargo check --workspace --all-targets
cargo check --target wasm32-unknown-unknown -p cosmic_defender_wasm

# 4. Run test suites
cargo test --workspace --all-targets
```

### Phase 2: Prepare Release Metadata
1. Update `version` in the root [`Cargo.toml`](file:///d:/github%20projects/Rust%20mini%20projects/Cargo.toml):
   ```toml
   [workspace.package]
   version = "0.2.0"
   ```
2. Update [`CHANGELOG.md`](file:///d:/github%20projects/Rust%20mini%20projects/CHANGELOG.md) moving entries from `[Unreleased]` into the new version header with the release date.
3. Commit the version bump:
   ```bash
   git commit -am "chore(release): bump version to v0.2.0"
   git push origin main
   ```

### Phase 3: Tag and Trigger Pipeline
Create an annotated Git tag and push it to GitHub:
```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```

The [`.github/workflows/release.yml`](file:///d:/github%20projects/Rust%20mini%20projects/.github/workflows/release.yml) action will automatically:
1. Build native release binaries across Linux, Windows, macOS Intel, and macOS Apple Silicon.
2. Build optimized WASM binaries and bundle web assets for WebAssembly projects.
3. Generate cryptographic `SHA256SUMS.txt`.
4. Create and publish the GitHub Release with all archives attached.

---

## 4. Artifact Verification Guide

Consumers and operators should verify release asset integrity using the published checksums:

### On Linux & macOS:
```bash
shasum -a 256 -c SHA256SUMS.txt
```

### On Windows (PowerShell):
```powershell
Get-FileHash -Algorithm SHA256 .\rust-mini-projects-windows-x86_64.zip
# Compare with published hash in SHA256SUMS.txt
```

---

## 5. Emergency Hotfix & Rollback Protocol

### Hotfix Workflow
If a critical regression is discovered in a release (e.g., `v1.2.0`):
1. Branch directly from the release tag:
   ```bash
   git checkout -b hotfix/v1.2.1 v1.2.0
   ```
2. Apply the minimal atomic fix and write regression tests.
3. Open a priority PR into `main`.
4. Once merged, create tag `v1.2.1` and push to trigger automated release packaging.

### Rollback Workflow
GitHub releases are immutable points in history. If a released build must be revoked:
1. Mark the GitHub Release as **Pre-release** or update its release notes with a prominent security advisory.
2. Do **not** delete the Git tag if external consumers have pulled it; instead, immediately publish a patched version (`vX.Y.Z+1`) that reverts the breaking commit.
