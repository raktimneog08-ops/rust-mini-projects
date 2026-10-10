use temperature_converter_cli::{Temperature, TemperatureError, TemperatureUnit};

const EPSILON: f64 = 1e-3;

#[test]
fn test_common_reference_temperatures() {
    // Human body temperature: 37 °C = 98.6 °F = 310.15 K
    let body = Temperature::new(37.0, TemperatureUnit::Celsius).unwrap();
    let body_f = body.to(TemperatureUnit::Fahrenheit);
    let body_k = body.to(TemperatureUnit::Kelvin);

    assert!((body_f.value - 98.6).abs() < EPSILON);
    assert!((body_k.value - 310.15).abs() < EPSILON);

    // Absolute zero in all scales
    let abs_zero_k = Temperature::new(0.0, TemperatureUnit::Kelvin).unwrap();
    assert!((abs_zero_k.to(TemperatureUnit::Celsius).value - -273.15).abs() < EPSILON);
    assert!((abs_zero_k.to(TemperatureUnit::Fahrenheit).value - -459.67).abs() < EPSILON);
    assert!((abs_zero_k.to(TemperatureUnit::Rankine).value - 0.0).abs() < EPSILON);
}

#[test]
fn test_rankine_conversions() {
    // 540 °R = 80.33 °F = 26.85 °C = 300.0 K
    let r = Temperature::new(540.0, TemperatureUnit::Rankine).unwrap();
    let k = r.to(TemperatureUnit::Kelvin);
    assert!((k.value - 300.0).abs() < EPSILON);
}

#[test]
fn test_invalid_temperature_rejected() {
    let err = Temperature::new(-300.0, TemperatureUnit::Celsius).unwrap_err();
    match err {
        TemperatureError::BelowAbsoluteZero { value, unit } => {
            assert_eq!(value, -300.0);
            assert_eq!(unit, TemperatureUnit::Celsius);
        }
        _ => panic!("Expected BelowAbsoluteZero error"),
    }
}

#[test]
fn test_shorthand_string_parsing() {
    let t: Temperature = "212F".parse().unwrap();
    let c = t.to(TemperatureUnit::Celsius);
    assert!((c.value - 100.0).abs() < EPSILON);
}
