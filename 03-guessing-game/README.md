# Project 03: Guessing Game CLI

> An interactive terminal number guessing game in Rust with configurable difficulty levels, dynamic parity & range hints, attempt tracking, and deterministic test suites.

---

## 🎮 Features

- **Difficulty Presets**:
  - `Easy`: Range 1–50 (10 attempts)
  - `Medium`: Range 1–100 (7 attempts)
  - `Hard`: Range 1–500 (9 attempts)
  - `Custom`: Fully configurable boundaries and attempts
- **Dynamic Clues (`hint`)**: Emits parity (even/odd) and quadrant placement hints.
- **Robust Input Handling**: Out-of-bounds checks do not penalize remaining attempts.
- **Deterministic Testing**: Dependency-injectable secrets for unit and integration testing.

---

## 📦 Usage

### Play Default (Medium)
```bash
cargo run -p guessing_game_cli
```

### Select Difficulty
```bash
# Easy Mode (1-50, 10 attempts)
cargo run -p guessing_game_cli -- --easy

# Hard Mode (1-500, 9 attempts)
cargo run -p guessing_game_cli -- --hard
```

### In-Game Commands
- Enter any integer within the range (e.g., `42`)
- Type `hint` to receive an informational clue
- Type `quit` or `exit` to forfeit and reveal the secret

---

## 🧪 Testing

```bash
cargo test -p guessing_game_cli
```
