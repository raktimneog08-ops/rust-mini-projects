//! Project 02: FizzBuzz CLI
//!
//! Interactive command-line interface for running FizzBuzz with configurable ranges
//! and custom rules.

use fizzbuzz_cli::{fizzbuzz, fizzbuzz_iter};
use std::env;

fn print_usage() {
    println!("🦀 FizzBuzz CLI (Project #02)");
    println!();
    println!("Usage:");
    println!("  fizzbuzz_cli [LIMIT]                  Run standard FizzBuzz from 1 to LIMIT (default: 100)");
    println!("  fizzbuzz_cli <START> <END>            Run standard FizzBuzz from START to END");
    println!("  fizzbuzz_cli --number <N>             Evaluate FizzBuzz for a single number");
    println!("  fizzbuzz_cli --custom <LIMIT> <RULES> Run with custom rules: divisor:label,... (e.g. 3:Fizz,5:Buzz,7:Bazz)");
    println!("  fizzbuzz_cli --help                   Display this help message");
}

fn parse_custom_rules(raw: &str) -> Result<Vec<(u64, String)>, String> {
    let mut rules = Vec::new();
    for part in raw.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let tokens: Vec<&str> = part.splitn(2, ':').collect();
        if tokens.len() != 2 {
            return Err(format!(
                "Invalid rule format '{part}'. Expected 'divisor:label'"
            ));
        }
        let divisor: u64 = tokens[0]
            .trim()
            .parse()
            .map_err(|_| format!("Invalid divisor integer in '{part}'"))?;
        if divisor == 0 {
            return Err(format!("Divisor cannot be 0 in '{part}'"));
        }
        rules.push((divisor, tokens[1].trim().to_string()));
    }
    if rules.is_empty() {
        Err("At least one custom rule must be provided.".to_string())
    } else {
        Ok(rules)
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        for val in fizzbuzz_iter(1, 100, &[(3, "Fizz"), (5, "Buzz")]) {
            println!("{val}");
        }
        return;
    }

    match args[0].as_str() {
        "--help" | "-h" => {
            print_usage();
        }
        "--number" | "-n" => {
            if args.len() < 2 {
                eprintln!("Error: Missing number argument.");
                std::process::exit(1);
            }
            match args[1].parse::<u64>() {
                Ok(n) => println!("{}", fizzbuzz(n)),
                Err(_) => {
                    eprintln!("Error: '{}' is not a valid positive integer.", args[1]);
                    std::process::exit(1);
                }
            }
        }
        "--custom" => {
            if args.len() < 3 {
                eprintln!("Error: Usage: fizzbuzz_cli --custom <LIMIT> <divisor:label,...>");
                std::process::exit(1);
            }
            let limit: u64 = match args[1].parse() {
                Ok(l) => l,
                Err(_) => {
                    eprintln!("Error: Invalid limit '{}'", args[1]);
                    std::process::exit(1);
                }
            };
            let parsed_rules = match parse_custom_rules(&args[2]) {
                Ok(r) => r,
                Err(err) => {
                    eprintln!("Error: {err}");
                    std::process::exit(1);
                }
            };
            let rule_refs: Vec<(u64, &str)> =
                parsed_rules.iter().map(|(d, s)| (*d, s.as_str())).collect();
            for val in fizzbuzz_iter(1, limit, &rule_refs) {
                println!("{val}");
            }
        }
        arg if args.len() == 1 => match arg.parse::<u64>() {
            Ok(limit) => {
                for val in fizzbuzz_iter(1, limit, &[(3, "Fizz"), (5, "Buzz")]) {
                    println!("{val}");
                }
            }
            Err(_) => {
                eprintln!("Error: Invalid limit '{arg}'. Run with --help for usage.");
                std::process::exit(1);
            }
        },
        _ if args.len() == 2 => {
            let start = args[0].parse::<u64>();
            let end = args[1].parse::<u64>();
            match (start, end) {
                (Ok(s), Ok(e)) if s <= e => {
                    for val in fizzbuzz_iter(s, e, &[(3, "Fizz"), (5, "Buzz")]) {
                        println!("{val}");
                    }
                }
                (Ok(s), Ok(e)) => {
                    eprintln!("Error: Start ({s}) cannot be greater than End ({e}).");
                    std::process::exit(1);
                }
                _ => {
                    eprintln!("Error: Arguments must be positive integers.");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error: Unrecognized arguments. Run with --help for usage.");
            std::process::exit(1);
        }
    }
}
