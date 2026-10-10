//! Project 04: Temperature Converter Engine
//!
//! Provides high-precision multi-scale temperature conversions, thermodynamic
//! absolute zero boundary validation, and string parsing across Celsius,
//! Fahrenheit, Kelvin, and Rankine scales.

use std::fmt;
use std::str::FromStr;

/// Supported temperature scales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
}

impl TemperatureUnit {
    /// Returns the symbol or abbreviation of the scale.
    #[must_use]
    pub fn symbol(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
            TemperatureUnit::Kelvin => "K",
            TemperatureUnit::Rankine => "°R",
        }
    }

    /// Returns the standard display name.
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "Celsius",
            TemperatureUnit::Fahrenheit => "Fahrenheit",
            TemperatureUnit::Kelvin => "Kelvin",
            TemperatureUnit::Rankine => "Rankine",
        }
    }

    /// All supported units in a static array.
    pub const ALL: [TemperatureUnit; 4] = [
        TemperatureUnit::Celsius,
        TemperatureUnit::Fahrenheit,
        TemperatureUnit::Kelvin,
        TemperatureUnit::Rankine,
    ];
}

impl fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for TemperatureUnit {
    type Err = TemperatureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim().to_uppercase();
        match trimmed.as_str() {
            "C" | "CELSIUS" | "°C" => Ok(TemperatureUnit::Celsius),
            "F" | "FAHRENHEIT" | "°F" => Ok(TemperatureUnit::Fahrenheit),
            "K" | "KELVIN" => Ok(TemperatureUnit::Kelvin),
            "R" | "RANKINE" | "°R" => Ok(TemperatureUnit::Rankine),
            _ => Err(TemperatureError::InvalidUnit(s.to_string())),
        }
    }
}

/// Errors occurring during temperature conversion or parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum TemperatureError {
    BelowAbsoluteZero { value: f64, unit: TemperatureUnit },
    InvalidUnit(String),
    ParseError(String),
}

impl fmt::Display for TemperatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemperatureError::BelowAbsoluteZero { value, unit } => {
                write!(
                    f,
                    "Temperature {value:.2} {} is below physical absolute zero (0 K / -273.15 °C)",
                    unit.symbol()
                )
            }
            TemperatureError::InvalidUnit(u) => {
                write!(
                    f,
                    "Invalid temperature unit '{u}'. Expected C (Celsius), F (Fahrenheit), K (Kelvin), or R (Rankine)"
                )
            }
            TemperatureError::ParseError(msg) => write!(f, "Failed to parse temperature: {msg}"),
        }
    }
}

impl std::error::Error for TemperatureError {}

/// Represents a measured temperature value associated with a scale.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature {
    pub value: f64,
    pub unit: TemperatureUnit,
}

impl Temperature {
    /// Creates a new temperature, verifying that it is at or above physical absolute zero.
    pub fn new(value: f64, unit: TemperatureUnit) -> Result<Self, TemperatureError> {
        let temp = Self { value, unit };
        let kelvin = temp.to_kelvin();
        // Allow a tiny epsilon for floating-point inaccuracies near 0K
        if kelvin < -1e-6 {
            return Err(TemperatureError::BelowAbsoluteZero { value, unit });
        }
        Ok(temp)
    }

    /// Converts internal temperature to Kelvin (the base SI thermodynamic scale).
    #[must_use]
    pub fn to_kelvin(&self) -> f64 {
        match self.unit {
            TemperatureUnit::Kelvin => self.value,
            TemperatureUnit::Celsius => self.value + 273.15,
            TemperatureUnit::Fahrenheit => (self.value - 32.0) * (5.0 / 9.0) + 273.15,
            TemperatureUnit::Rankine => self.value * (5.0 / 9.0),
        }
    }

    /// Converts the current temperature to another unit.
    pub fn to(&self, target_unit: TemperatureUnit) -> Self {
        let kelvin = self.to_kelvin();
        let new_value = match target_unit {
            TemperatureUnit::Kelvin => kelvin,
            TemperatureUnit::Celsius => kelvin - 273.15,
            TemperatureUnit::Fahrenheit => (kelvin - 273.15) * (9.0 / 5.0) + 32.0,
            TemperatureUnit::Rankine => kelvin * (9.0 / 5.0),
        };
        Self {
            value: new_value,
            unit: target_unit,
        }
    }

    /// Converts this temperature into all supported scales.
    #[must_use]
    pub fn convert_all(&self) -> Vec<Temperature> {
        TemperatureUnit::ALL
            .iter()
            .map(|&unit| self.to(unit))
            .collect()
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2} {}", self.value, self.unit.symbol())
    }
}

impl FromStr for Temperature {
    type Err = TemperatureError;

    /// Parses strings such as "100C", "32 F", "-40°C", "373.15 K".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(TemperatureError::ParseError("Empty input".to_string()));
        }

        // Split number and unit from end of string
        let split_pos = trimmed
            .rfind(|c: char| c.is_ascii_digit())
            .map(|pos| pos + 1)
            .ok_or_else(|| {
                TemperatureError::ParseError("No numeric component found".to_string())
            })?;

        let (num_part, unit_part) = trimmed.split_at(split_pos);
        let value: f64 = num_part
            .trim()
            .parse()
            .map_err(|e| TemperatureError::ParseError(format!("Invalid number: {e}")))?;

        let unit_str = unit_part.trim();
        let unit = if unit_str.is_empty() {
            TemperatureUnit::Celsius // Default fallback unit
        } else {
            unit_str.parse()?
        };

        Self::new(value, unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-4;

    #[test]
    fn test_water_freezing_point_conversions() {
        let freezing_c = Temperature::new(0.0, TemperatureUnit::Celsius).unwrap();
        let fahrenheit = freezing_c.to(TemperatureUnit::Fahrenheit);
        let kelvin = freezing_c.to(TemperatureUnit::Kelvin);
        let rankine = freezing_c.to(TemperatureUnit::Rankine);

        assert!((fahrenheit.value - 32.0).abs() < EPSILON);
        assert!((kelvin.value - 273.15).abs() < EPSILON);
        assert!((rankine.value - 491.67).abs() < EPSILON);
    }

    #[test]
    fn test_water_boiling_point_conversions() {
        let boiling_c = Temperature::new(100.0, TemperatureUnit::Celsius).unwrap();
        let fahrenheit = boiling_c.to(TemperatureUnit::Fahrenheit);
        let kelvin = boiling_c.to(TemperatureUnit::Kelvin);

        assert!((fahrenheit.value - 212.0).abs() < EPSILON);
        assert!((kelvin.value - 373.15).abs() < EPSILON);
    }

    #[test]
    fn test_minus_40_intersection() {
        // -40 °C is uniquely equal to -40 °F
        let t_c = Temperature::new(-40.0, TemperatureUnit::Celsius).unwrap();
        let t_f = t_c.to(TemperatureUnit::Fahrenheit);
        assert!((t_f.value - -40.0).abs() < EPSILON);
    }

    #[test]
    fn test_absolute_zero_validation() {
        assert!(Temperature::new(-273.15, TemperatureUnit::Celsius).is_ok());
        assert!(Temperature::new(0.0, TemperatureUnit::Kelvin).is_ok());
        assert!(Temperature::new(-459.67, TemperatureUnit::Fahrenheit).is_ok());

        // Below absolute zero must error
        assert!(Temperature::new(-273.16, TemperatureUnit::Celsius).is_err());
        assert!(Temperature::new(-0.1, TemperatureUnit::Kelvin).is_err());
        assert!(Temperature::new(-460.0, TemperatureUnit::Fahrenheit).is_err());
    }

    #[test]
    fn test_round_trip_precision() {
        let initial = Temperature::new(36.6, TemperatureUnit::Celsius).unwrap();
        let f = initial.to(TemperatureUnit::Fahrenheit);
        let k = f.to(TemperatureUnit::Kelvin);
        let r = k.to(TemperatureUnit::Rankine);
        let back_to_c = r.to(TemperatureUnit::Celsius);

        assert!((initial.value - back_to_c.value).abs() < EPSILON);
    }

    #[test]
    fn test_parsing_valid_strings() {
        let t1: Temperature = "100C".parse().unwrap();
        assert_eq!(t1.unit, TemperatureUnit::Celsius);
        assert!((t1.value - 100.0).abs() < EPSILON);

        let t2: Temperature = "32 F".parse().unwrap();
        assert_eq!(t2.unit, TemperatureUnit::Fahrenheit);
        assert!((t2.value - 32.0).abs() < EPSILON);

        let t3: Temperature = "273.15 K".parse().unwrap();
        assert_eq!(t3.unit, TemperatureUnit::Kelvin);
        assert!((t3.value - 273.15).abs() < EPSILON);
    }

    #[test]
    fn test_convert_all_length() {
        let t = Temperature::new(25.0, TemperatureUnit::Celsius).unwrap();
        let all = t.convert_all();
        assert_eq!(all.len(), 4);
    }
}
