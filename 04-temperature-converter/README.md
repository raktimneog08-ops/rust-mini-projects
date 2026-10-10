# Project 04: Temperature Converter CLI

> A high-precision thermodynamic temperature conversion engine and CLI in Rust supporting Celsius, Fahrenheit, Kelvin, and Rankine scales with physical absolute zero validation.

---

## 🌡️ Features

- **Supported Scales**:
  - **Celsius (`°C`)**: Metric standard scale.
  - **Fahrenheit (`°F`)**: Imperial standard scale.
  - **Kelvin (`K`)**: Base SI thermodynamic temperature.
  - **Rankine (`°R`)**: Thermodynamic scale based on Fahrenheit degrees.
- **Thermodynamic Guardrails**: Strictly validates physical absolute zero ($0\text{ K}$, $-273.15\text{ °C}$, $-459.67\text{ °F}$, $0\text{ °R}$).
- **Interactive & Batch Interface**:
  - Interactive shell when launched without arguments.
  - Point-to-point conversion (`temp 100 C F`).
  - Shorthand parsing (`temp 100C`, `temp 32F to C`).
  - All-in-one formatted summary tables (`temp 25 C`).
  - Range table generator (`temp --table 0 100 20`).
- **100% Test Coverage**: Full suite of unit and integration tests.

---

## 📦 Usage

### Direct Conversion
```bash
cargo run -p temperature_converter_cli -- 100 C F
# Output: 100.00 °C = 212.00 °F
```

### Full Scale Breakdown
```bash
cargo run -p temperature_converter_cli -- 37 C
# Output: Formatted table with Celsius, Fahrenheit, Kelvin, and Rankine equivalents
```

### Generate Conversion Range Table
```bash
cargo run -p temperature_converter_cli -- --table 0 100 25
```

### Interactive Shell
```bash
cargo run -p temperature_converter_cli
```

---

## 🧪 Testing

```bash
cargo test -p temperature_converter_cli
```
