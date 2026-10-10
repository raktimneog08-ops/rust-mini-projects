# Problem Statement: Project 100 - Rust-to-WASM Arcade Game ("Cosmic Defender WASM")

## 1. Executive Summary
Develop a production-grade, high-performance 2D space arcade combat game written in pure **Rust** and compiled to **WebAssembly (WASM)**. The system must achieve deterministic 60 FPS gameplay, low-latency zero-overhead host-guest memory interop with Javascript, synthesized procedural audio via the Web Audio API without external audio asset downloads, and responsive cross-platform controls (keyboard, gamepad, mouse, and touch).

---

## 2. Core Problem & Objectives

### 2.1 Technical Challenges
1. **Host-Guest Memory Overhead**: Traditional WASM-JS interop with heavy JSON serialization or complex wrapper object allocations causes garbage collection pauses and frame drops. The game engine must use direct, contiguous linear memory buffers (flat `Float32Array` / `Uint8Array` views) to stream hundreds of entities per frame at 60 FPS without per-frame allocations.
2. **Determinism & Performance**: Physics simulation, projectile trajectories, bounding volume collisions, and particle lifetime must execute with fixed-step precision in Rust independent of variable monitor refresh rates (60Hz, 120Hz, 144Hz).
3. **Asset-Free Audio Synthesis**: Games requiring large asset bundles suffer from network latency and load failures. The engine must emit discrete sound event codes that a Web Audio API synthesizer translates into procedural 8-bit retro audio waveforms (lasers, sweeps, explosions, alarms) on the fly.
4. **Resilience & Fault Tolerance**: The system must gracefully handle extreme scenarios like window minimization, high entity saturation (bullet hell), canvas resizing, audio context suspension, and missing gamepad disconnections.

---

## 3. Functional Requirements

### 3.1 Gameplay & Mechanics
- **Player Ship**:
  - 2D directional movement with inertia, acceleration, and viewport clamping.
  - Multi-tier defense: Auto-recharging energy shields and structural hull health.
  - 4 Weapon Systems: *Plasma Blaster*, *Plasma Cannon*, *Spread Shot*, and *Laser Beam* with upgrade levels.
  - Emergency EMP Shockwave: Screen-clearing superweapon that destroys hostile projectiles and damages all active hostiles.
  - Combo Scoring System: Consecutive hit streaks provide exponential point multipliers with decay timers.
- **Enemy AI & Wave Director**:
  - *Scout*: Agile sinusoidal wave motion with light pulse blasters.
  - *Corvette*: Heavy armored patrol craft firing twin spread pulses.
  - *Kamikaze*: High-speed diving swarmers tracking player coordinates.
  - *Dreadnought*: Mini-bosses with three-way heavy spread cannons.
  - *Mothership Boss*: Multi-phase boss encounter (Phase 1: Spread barrage; Phase 2: Spiral bullet ring; Phase 3: Frenzy storm) with visual core state changes.
- **Particle & Visual FX**:
  - Multi-layer parallax starfield simulation.
  - Thruster spark trails, explosive debris rings, EMP shockwave rings, and screen-shake impact effects.

### 3.2 Web Interface & UX
- Cyber-retro neon arcade presentation with responsive full-viewport Canvas.
- Real-time HUD displaying hull integrity, shield percentage, score, high score persistence in `localStorage`, wave indicator, combo streak, weapon stats, and EMP orb gauges.
- Toggles for CRT scanlines shader, procedural audio mute, and game pause.
- Start / Game Over / Pause modals with restart workflows.

---

## 4. Non-Functional Requirements

- **Performance**: Constant 60 FPS runtime under load (300+ particles, 100+ projectiles, 30+ enemies simultaneously).
- **Binary Footprint**: Release `.wasm` footprint under 150 KB.
- **Portability**: Zero external native system dependencies; standalone compilation targeting `wasm32-unknown-unknown`.
- **Cross-Platform Input**: Full parity across Keyboard (`WASD`/Arrows, `Space`, `E`/`Shift`), Mouse/Touch, and HTML5 Gamepad API.
