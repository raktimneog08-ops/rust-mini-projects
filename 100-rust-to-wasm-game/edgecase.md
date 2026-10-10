# Edge Cases & Mitigation Strategies: Cosmic Defender WASM

This document outlines critical edge cases, boundary conditions, platform anomalies, and their architectural mitigations for the `Cosmic Defender WASM` game engine.

---

## 1. WASM Memory & Interop Edge Cases

### 1.1 WASM Memory Growth & Detached Array Buffers
- **Scenario**: If Rust allocates additional memory dynamically on the heap during runtime, the underlying `WebAssembly.Memory` buffer expands, which immediately **detaches and invalidates** all previous JavaScript `Float32Array` or `Uint8Array` view references.
- **Impact**: Any JavaScript read from a stale typed array view throws a `TypeError: Cannot perform %TypedArray%.prototype.slice on a detached ArrayBuffer` or silently reads zeroes.
- **Architectural Mitigation**:
  - Pre-allocate static buffer capacities in Rust (`Vec::with_capacity(4000)`) during `init_game()`.
  - In JavaScript, always reconstruct the typed array view from `wasmMemory.buffer` on every frame (`new Float32Array(wasmMemory.buffer, ptr, length)`). Never cache the typed array instances across animation frames.

### 1.2 Unaligned Buffer Pointer Access
- **Scenario**: Accessing 32-bit floats (`f32`) on an unaligned memory boundary.
- **Architectural Mitigation**: All Rust export buffers are typed `Vec<f32>` or `Vec<u8>` directly, ensuring natural 4-byte or 1-byte alignment from Rust's global allocator.

---

## 2. Timing, Framerate & Simulation Edge Cases

### 2.1 Tab Inactivity & Large Delta Time ("Spiral of Death")
- **Scenario**: When the user switches browser tabs or minimizes the window, `requestAnimationFrame` halts. Upon tab reactivation, the accumulated `dt` (e.g., 5.0 to 30.0 seconds) is passed into `update_game()`.
- **Impact**: Bullets phase completely through enemy hitboxes (tunneling), player physics shoots outside the screen boundaries, and enemy spawn loops trigger dozens of simultaneous waves.
- **Architectural Mitigation**:
  - In `app.js`, clamp `dt`: `const dt = Math.min((time - lastTime) / 1000, 0.1);`.
  - In `game.rs`, apply strict spatial clamping after position updates (`self.x = self.x.clamp(margin, screen_w - margin)`).

### 2.2 Variable Refresh Rates (120Hz, 144Hz, 240Hz Monitors)
- **Scenario**: Modern displays run at refresh rates higher than 60Hz. If game speed relies on frame counts rather than elapsed delta time `dt`, the game runs at 2x-4x speed.
- **Architectural Mitigation**: All kinematics, cooldowns, timers, drag coefficients, and particle lifetimes are multiplied by `dt` in floating-point seconds.

---

## 3. High Entity Saturation & Stress Limits (Bullet Hell Edge Cases)

### 3.1 Particle & Bullet Buffer Exhaustion
- **Scenario**: A boss enters Phase 3, the player detonates an EMP, and multiple dreadnoughts explode concurrently, generating over 1,000 entities in a single frame.
- **Impact**: Unbounded memory growth, GC pressure, frame stutters.
- **Architectural Mitigation**:
  - Hard caps enforced in Rust:
    - Max active particles: `512` (subsequent emissions dropped).
    - Max audio queue events per frame: `32` (prevent audio buffer flooding).
  - Out-of-bounds entity culling runs every frame with `retain()` (`b.is_alive(screen_w, screen_h)`).
  - Reverse-index swap-removal (`swap_remove(idx)`) ensures $O(1)$ entity deallocation without array shifting.

### 3.2 Simultaneous Double-Kill (Player & Boss Die on the Same Frame)
- **Scenario**: A player laser strikes and destroys the boss on the exact frame an enemy pulse impacts the player hull, reducing player HP to 0.
- **Impact**: Race condition between `GameState::Victory` and `GameState::GameOver`.
- **Architectural Mitigation**: Player health evaluation takes precedence. The game marks `GameOver` and records high score before wave advancement can occur.

---

## 4. Audio Engine & Browser Policy Edge Cases

### 4.1 Browser Autoplay Policy Blocking
- **Scenario**: Modern browsers (Chrome, Firefox, Safari) block `AudioContext` playback unless triggered directly by a user gesture.
- **Impact**: Game starts with complete silence or throws `DOMException: The AudioContext was not allowed to start`.
- **Architectural Mitigation**:
  - `ArcadeAudio.init()` is lazily called on the first user interaction (`mousedown`, `keydown`, `touchstart`, or clicking the Start Button).
  - Automatically invokes `ctx.resume()` if `ctx.state === 'suspended'`.

### 4.2 Audio Node Leakage & Memory Exhaustion
- **Scenario**: Rapidly firing weapons generates hundreds of oscillator nodes per minute. If oscillator and gain nodes are not properly disconnected and stopped, memory leaks occur.
- **Architectural Mitigation**: All procedural nodes call `osc.stop(t + duration)` with short finite envelopes ($< 0.6s$) and are garbage collected automatically upon termination.

---

## 5. Input Device Edge Cases

### 5.1 Gamepad Stick Drift & Sudden Disconnection
- **Scenario**: Analog stick on older controllers has slight resting offset ($0.05 - 0.15$), causing unintended slow ship drifting. Controller battery dies or unplugged mid-game.
- **Architectural Mitigation**:
  - Deadzone threshold enforced in `pollGamepad()`: `Math.abs(gp.axes[0]) > 0.2`.
  - Missing gamepads (`navigator.getGamepads()[0] == null`) safely return without mutating input states.

### 5.2 Simultaneous Opposite Key Pressing (Left + Right, Up + Down)
- **Scenario**: User presses `A` and `D` simultaneously.
- **Architectural Mitigation**: Input delta summation (`dx = (right ? 1 : 0) - (left ? 1 : 0)`). Pressing both results in `dx = 0` (cancels cleanly).

---

## 6. Storage & State Persistence Edge Cases

### 6.1 `localStorage` Quota Exceeded or Incognito Mode Blocking
- **Scenario**: In private/incognito browsing or locked storage environments, `localStorage.setItem()` throws a security error.
- **Architectural Mitigation**: `localStorage` read and write operations are wrapped in safe fallback handlers, falling back to in-memory runtime session scores if storage is blocked.
