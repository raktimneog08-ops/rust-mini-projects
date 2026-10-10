//! Project 04: Temperature Converter CLI
//!
//! Interactive and command-line utility for converting between thermodynamic temperature scales.

use std::env;
use std::io::{self, Write};
use temperature_converter_cli::{Temperature, TemperatureUnit};

fn print_banner() {
    println!("╔════════════════════════════════════════════╗");
    println!("║       🌡️  TEMPERATURE CONVERTER CLI        ║");
    println!("╚════════════════════════════════════════════╝");
    println!("Supported Units: C (Celsius), F (Fahrenheit), K (Kelvin), R (Rankine)");
    println!();
}

fn print_usage() {
    println!("Usage:");
    println!(
        "  temperature_converter_cli <VALUE> <FROM_UNIT> <TO_UNIT>  Convert between two units"
    );
    println!("  temperature_converter_cli <VALUE> <FROM_UNIT>            Convert to all scales");
    println!("  temperature_converter_cli <VALUE><UNIT>                  Convert shorthand (e.g. 100C, 32F)");
    println!(
        "  temperature_converter_cli --table <MIN> <MAX> <STEP>     Generate conversion table"
    );
    println!("  temperature_converter_cli --help, -h                     Show help");
}

fn print_all_conversions(source: &Temperature) {
    println!();
    println!("┌───────────────────────┬────────────────────────┐");
    println!("│ Scale                 │ Temperature            │");
    println!("├───────────────────────┼────────────────────────┤");
    for converted in source.convert_all() {
        let is_source = if converted.unit == source.unit {
            " (source)"
        } else {
            ""
        };
        let label = format!("{}{is_source}", converted.unit.name());
        println!(
            "│ {:<21} │ {:>18.2} {} │",
            label,
            converted.value,
            converted.unit.symbol()
        );
    }
    println!("└───────────────────────┴────────────────────────┘");
    println!();
}

fn run_interactive_mode() {
    print_banner();
    println!("Enter values to convert (e.g. '100 C', '32 F to C', '273.15K'), or 'quit' to exit.");
    println!();

    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        print!("Temp > ");
        io::stdout().flush().unwrap_or(());

        line.clear();
        if stdin.read_line(&mut line).is_err() {
            break;
        }

        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("quit") || trimmed.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        if trimmed.is_empty() {
            continue;
        }

        // Try parsing directly
        match trimmed.parse::<Temperature>() {
            Ok(temp) => {
                print_all_conversions(&temp);
            }
            Err(_) => {
                // Try splitting into words: e.g. "100 C F" or "100 C to F"
                let tokens: Vec<&str> = trimmed
                    .split_whitespace()
                    .filter(|&w| !w.eq_ignore_ascii_case("to"))
                    .collect();

                if tokens.len() == 3 {
                    if let (Ok(val), Ok(from_u), Ok(to_u)) = (
                        tokens[0].parse::<f64>(),
                        tokens[1].parse::<TemperatureUnit>(),
                        tokens[2].parse::<TemperatureUnit>(),
                    ) {
                        match Temperature::new(val, from_u) {
                            Ok(t) => {
                                let res = t.to(to_u);
                                println!("👉 {t} = {res}");
                            }
                            Err(e) => eprintln!("❌ Error: {e}"),
                        }
                        continue;
                    }
                } else if tokens.len() == 2 {
                    if let (Ok(val), Ok(from_u)) = (
                        tokens[0].parse::<f64>(),
                        tokens[1].parse::<TemperatureUnit>(),
                    ) {
                        match Temperature::new(val, from_u) {
                            Ok(t) => print_all_conversions(&t),
                            Err(e) => eprintln!("❌ Error: {e}"),
                        }
                        continue;
                    }
                }

                eprintln!("⚠️  Could not parse '{trimmed}'. Example: '100 C' or '32 F C'");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        run_interactive_mode();
        return;
    }

    match args[0].as_str() {
        "--help" | "-h" => {
            print_banner();
            print_usage();
        }
        "--table" => {
            if args.len() < 4 {
                eprintln!("Usage: temperature_converter_cli --table <MIN> <MAX> <STEP>");
                std::process::exit(1);
            }
            let min: f64 = args[1].parse().unwrap_or(0.0);
            let max: f64 = args[2].parse().unwrap_or(100.0);
            let step: f64 = args[3].parse().unwrap_or(10.0);

            if step <= 0.0 || min > max {
                eprintln!("Invalid table range or step.");
                std::process::exit(1);
            }

            println!("┌──────────────┬──────────────┬──────────────┬──────────────┐");
            println!("│   Celsius    │  Fahrenheit  │    Kelvin    │   Rankine    │");
            println!("├──────────────┼──────────────┼──────────────┼──────────────┤");
            let mut current = min;
            while current <= max + 1e-6 {
                if let Ok(c) = Temperature::new(current, TemperatureUnit::Celsius) {
                    let f = c.to(TemperatureUnit::Fahrenheit);
                    let k = c.to(TemperatureUnit::Kelvin);
                    let r = c.to(TemperatureUnit::Rankine);
                    println!(
                        "│ {:>10.2}°C │ {:>10.2}°F │ {:>10.2} K │ {:>10.2}°R │",
                        c.value, f.value, k.value, r.value
                    );
                }
                current += step;
            }
            println!("└──────────────┴──────────────┴──────────────┴──────────────┘");
        }
        _ => {
            let filtered: Vec<&str> = args
                .iter()
                .map(String::as_str)
                .filter(|&w| !w.eq_ignore_ascii_case("to"))
                .collect();

            if filtered.len() == 3 {
                let val: f64 = match filtered[0].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Invalid numeric value '{}'", filtered[0]);
                        std::process::exit(1);
                    }
                };
                let from_u: TemperatureUnit = match filtered[1].parse() {
                    Ok(u) => u,
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                };
                let to_u: TemperatureUnit = match filtered[2].parse() {
                    Ok(u) => u,
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                };
                match Temperature::new(val, from_u) {
                    Ok(source) => {
                        let target = source.to(to_u);
                        println!("{source} = {target}");
                    }
                    Err(e) => {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                }
            } else if filtered.len() == 2 {
                let val: f64 = match filtered[0].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Invalid numeric value '{}'", filtered[0]);
                        std::process::exit(1);
                    }
                };
                let from_u: TemperatureUnit = match filtered[1].parse() {
                    Ok(u) => u,
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                };
                match Temperature::new(val, from_u) {
                    Ok(source) => print_all_conversions(&source),
                    Err(e) => {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                }
            } else if filtered.len() == 1 {
                match filtered[0].parse::<Temperature>() {
                    Ok(source) => print_all_conversions(&source),
                    Err(e) => {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Unrecognized arguments. Run with --help for usage.");
                std::process::exit(1);
            }
        }
    }
}
