# Phase-Wise Implementation Plan: Cosmic Defender WASM (Project #100)

Based on the specifications established in [`problemstatement.md`](file:///d:/github%20projects/Rust%20mini%20projects/100-rust-to-wasm-game/problemstatement.md), [`architecture.md`](file:///d:/github%20projects/Rust%20mini%20projects/100-rust-to-wasm-game/architecture.md), and [`edgecase.md`](file:///d:/github%20projects/Rust%20mini%20projects/100-rust-to-wasm-game/edgecase.md), the system is organized into 7 structured implementation phases.

---

## 🗺️ Implementation Roadmap Overview

```mermaid
gantt
    title Cosmic Defender WASM - Phase Breakdown
    dateFormat  YYYY-MM-DD
    section Core Rust Engine
    Phase 1: Zero-Dependency Foundation & Kinematics :done, p1, 2026-08-01, 1d
    Phase 2: Entity Architecture & Collision Engine  :done, p2, after p1, 1d
    section WASM Bridge & Memory
    Phase 3: C-ABI Export & Zero-Copy Serializer    :done, p3, after p2, 1d
    section Presentation & Synth
    Phase 4: Web Arcade HUD & Canvas 2D Renderer    :done, p4, after p3, 1d
    Phase 5: Web Audio API Procedural Synthesizer    :done, p5, after p4, 1d
    section Hardening & Testing
    Phase 6: Edge Case Hardening & Controller Input :done, p6, after p5, 1d
    Phase 7: Test Suite, Benchmarks & Verification  :done, p7, after p6, 1d
```

---

## Phase 1: Zero-Dependency Foundation & Kinematics
- **Goal**: Initialize the Rust crate targeting `wasm32-unknown-unknown` with zero external crate dependencies for maximum speed, minimal binary size, and instant compilation.
- **Key Deliverables**:
  - `Cargo.toml` with `cdylib` and `rlib` targets, LTO, and release optimizations (`opt-level = 3`, `panic = "abort"`).
  - Fast, deterministic PRNG algorithm (Xorshift32) for zero-dependency pseudo-random numbers.
  - Coordinate system setup with normalized bounds.
- **Verification Gate**:
  - `cargo check --target wasm32-unknown-unknown` succeeds cleanly.

---

## Phase 2: Entity Architecture & Collision Engine
- **Goal**: Implement all deterministic simulation objects in pure Rust.
- **Key Deliverables**:
  - `src/collision.rs`: Geometric 2D intersection routines (`Circle`, `Rect`, point containment).
  - `src/player.rs`: Spaceship physics, inertia damping, shield absorption arithmetic, weapon firing rates (Blaster, Plasma, Spread, Beam), and EMP discharges.
  - `src/bullet.rs`: Kinetic projectile vectors and lifetime culling.
  - `src/enemy.rs`: 5 Enemy archetypes (`Scout`, `Corvette`, `Kamikaze`, `Dreadnought`, `Boss`), boss phase transitions (1: Spread, 2: Spiral, 3: Frenzy), and powerup drops (`ShieldBoost`, `WeaponUpgrade`, `EmpCharge`, `ScoreBonus`).
  - `src/particle.rs`: Particle simulation (sparks, debris, shockwaves) with bounded vector capacities.
  - `src/audio.rs`: Sound event queue (`AudioQueue`) with event code enums.
- **Verification Gate**:
  - Unit tests covering collision detection, shield absorption, boss phase triggers, and combo multiplier scoring pass with 100% success.

---

## Phase 3: C-ABI WASM Export & Zero-Copy Memory Serializer
- **Goal**: Expose safe C-ABI functions to WebAssembly that stream simulation data into contiguous linear memory buffers.
- **Key Deliverables**:
  - `src/lib.rs`:
    - Lifecycle hooks: `init_game(w, h)`, `update_game(dt, dx, dy, fire)`, `restart_game()`, `trigger_emp()`.
    - State readers: `get_game_state()`, `get_player_health()`, `get_player_shield()`, `get_player_score()`, `get_screen_shake()`.
    - Contiguous buffer serializers: `get_stars_buffer_ptr()`, `get_enemies_buffer_ptr()`, `get_bullets_buffer_ptr()`, `get_particles_buffer_ptr()`, `get_audio_events_ptr()`.
- **Verification Gate**:
  - `cargo build --target wasm32-unknown-unknown --release` generates a compact binary ($< 100\text{ KB}$).

---

## Phase 4: Web Arcade Presentation & Canvas 2D Renderer
- **Goal**: Build a responsive cyber-arcade web frontend rendered via HTML5 Canvas.
- **Key Deliverables**:
  - `www/index.html`: Retro arcade cabinet layout, glassmorphic HUD, live hull and shield meters, wave badges, combo streak indicators, EMP gauges, and mobile touch buttons.
  - `www/style.css`: Neon styling, glowing animations, and CRT scanlines overlay.
  - `www/app.js`:
    - Asynchronous WASM binary fetcher and instantiation.
    - 60 FPS `requestAnimationFrame` render loop.
    - Zero-copy memory buffer reader using `Float32Array` views on `wasmMemory.buffer`.
    - High-detail Canvas 2D vector drawing for player ships, laser trails, plasma orbs, enemy vessels, boss health bars, and expanding EMP shockwaves.
- **Verification Gate**:
  - Smooth 60 FPS rendering verified in browser without dropped frames or memory leaks.

---

## Phase 5: Web Audio API Procedural Synthesizer
- **Goal**: Implement zero-asset, procedural retro 8-bit sound synthesis directly in JavaScript using the Web Audio API.
- **Key Deliverables**:
  - `www/audio.js`:
    - Sound event dispatcher reacting to Rust's audio queue bytes.
    - Oscillator graphs for laser chirps (`sawtooth`), plasma thuds (`sine`), and spread bursts (`square`).
    - Biquad lowpass filtered white-noise buffers for explosions and EMP rumbles.
    - Multi-note ascending arpeggio pickups for powerups and wave clears.
    - Dynamic 8-step bassline chiptune BGM arpeggiator.
- **Verification Gate**:
  - Audio plays crisply across all game events with zero audio buffer underruns.

---

## Phase 6: Edge Case Hardening & Cross-Platform Input
- **Goal**: Harden the system against all anomalies identified in [`edgecase.md`](file:///d:/github%20projects/Rust%20mini%20projects/100-rust-to-wasm-game/edgecase.md).
- **Key Deliverables**:
  - Delta Time clamping (`Math.min(dt, 0.1)`) to eliminate frame tunneling upon tab restoration.
  - Reconstructing typed array views per frame to prevent detached array buffer errors upon WASM memory growth.
  - Autoplay audio policy handling: Lazily resume suspended `AudioContext` on first user gesture.
  - Full input normalization: Keyboard (`WASD`/Arrows, `Space`, `E`), Mouse/Touch drag controls, and HTML5 Gamepad API with analog deadzones.
  - Safe `localStorage` score persistence with fallback for incognito environments.
- **Verification Gate**:
  - Tab minimization test: Pausing/switching tabs and resuming causes zero coordinate clipping or physics explosions.

---

## Phase 7: Verification, Automated Testing & Documentation
- **Goal**: Full project validation, documentation, and local server tooling.
- **Key Deliverables**:
  - `tests/game_tests.rs`: Complete test suite for game mechanics and state progression.
  - `serve.py`: Standalone Python web server with WebAssembly MIME type mapping (`application/wasm`).
  - `README.md`, `problemstatement.md`, `architecture.md`, `edgecase.md`, and `phase-wise-implementation.md`.
- **Verification Gate**:
  - All test suites passing, WASM artifact verified at 96 KB, and game playable locally via `python serve.py`.
