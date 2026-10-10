# Project 100: Cosmic Defender WASM (Rust-to-WASM Game)

> A high-performance, retro-arcade space shooter built in **Rust** and compiled to **WebAssembly (WASM)**. Features pure deterministic physics, particle systems, multi-tiered enemy AI, boss battles, a procedural Web Audio synthesizer, and a cyber-arcade HTML5 Canvas frontend.

---

## 🎮 Features

- **Rust Game Engine Core**:
  - Deterministic physics and velocity integration.
  - 4 Weapon Systems: *Plasma Blaster*, *Plasma Cannon*, *Spread Shot*, and *Continuous Laser*.
  - Defensive Systems: Auto-regenerating shields and EMP Shockwaves that destroy hostile bullets.
  - Multi-tiered Enemy AI:
    - **Scouts**: Fast zig-zag interceptors.
    - **Corvettes**: Heavy armor gunboats firing twin pulse cannons.
    - **Kamikazes**: Aggressive suicide diving drones.
    - **Dreadnoughts**: Heavy armor mini-bosses.
    - **Mothership Boss**: 3 distinct combat phases with rotating spiral bullet rings and frenzy storms.
  - High-performance particle engine (thrusters, explosions, shockwaves, parallax starfield).
  - Procedural sound events emitted directly from Rust to the Web Audio API.

- **Web Frontend**:
  - Zero-dependency WASM loading with direct float/byte buffer sharing for silky smooth 60 FPS rendering.
  - Glassmorphic neon HUD with live hull integrity, shield bar, combo multiplier, weapon gauges, and EMP charges.
  - Procedural Web Audio 8-bit synthesizer (no external audio files needed!).
  - CRT Scanline shader overlay toggle.
  - Multi-input support: **Keyboard** (`WASD`/Arrows, `Space`, `E`/`Shift`), **Mouse/Touch** controls, and **Gamepad API** (Xbox/PlayStation controller support).

---

## 📁 Project Structure

```
100-rust-to-wasm-game/
├── Cargo.toml               # Rust package configuration with cdylib / rlib targets
├── src/
│   ├── lib.rs               # WASM exported ABI functions & memory buffer serializers
│   ├── game.rs              # Main game loop, state machine, wave director, and scoring
│   ├── player.rs            # Player spaceship physics, weapon types, and shields
│   ├── enemy.rs             # Enemy types, AI movement patterns, powerups, and boss phases
│   ├── bullet.rs            # Projectile physics and life cycle
│   ├── particle.rs          # 2D particle simulation and starfield parallax
│   ├── collision.rs         # Geometric collision detection (Circle, Rect)
│   └── audio.rs             # Sound event queue system
├── tests/
│   └── game_tests.rs        # Comprehensive integration test suite
├── www/
│   ├── index.html           # Arcade web interface & neon HUD
│   ├── style.css            # Cyber-retro styling and CRT scanlines
│   ├── app.js               # Canvas renderer, input dispatcher, and WASM memory consumer
│   ├── audio.js             # Web Audio API 8-bit chiptune synthesizer
│   └── cosmic_defender_wasm.wasm # Compiled release WebAssembly binary (96 KB)
├── serve.py                 # Local Python web server runner
└── README.md                # Documentation and guide
```

---

## 🚀 Quick Start & How to Play

### 1. Run with Local Python Server
```bash
python serve.py
```
Open [http://localhost:8080](http://localhost:8080) in your web browser.

### 2. Build WASM from Rust Source
```bash
# Add WebAssembly compilation target (one-time setup)
rustup target add wasm32-unknown-unknown

# Compile optimized release WASM
cargo build --target wasm32-unknown-unknown --release

# Copy to www directory
cp target/wasm32-unknown-unknown/release/cosmic_defender_wasm.wasm www/
```

### 3. Run Unit & Integration Tests
```bash
cargo test
```

---

## 🕹️ Controls

| Action | Keyboard | Gamepad | Mobile |
| :--- | :--- | :--- | :--- |
| **Move** | `W`, `A`, `S`, `D` or Arrow Keys | Left Stick / D-Pad | Touch drag |
| **Fire Weapons** | `Space` / Left Click | `A` / `RT` / `R2` | `FIRE` Button |
| **EMP Shockwave**| `E` / `Shift` | `B` / `LT` / `L2` | `EMP` Button |
| **Pause** | `P` | `Start` / `Options` | Pause Icon |
| **Toggle Sound** | Sound Icon Top Right | - | Sound Icon |
| **Toggle CRT** | CRT Icon Top Right | - | CRT Icon |

---

## 🧪 Testing Summary
All core game logic, damage calculation, spatial collisions, boss phase transitions, and combo multipliers are covered by automated unit tests in `tests/game_tests.rs`.
