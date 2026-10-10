//! Project 01: Hello World CLI
//!
//! Demonstrates fundamental Rust concepts:
//! - Standard I/O and command line argument inspection
//! - String formatting and memory management
//! - Unit testing and idiomatic project structure

use std::env;

/// Generates a personalized greeting message.
#[must_use]
pub fn generate_greeting(name: Option<&str>) -> String {
    match name {
        Some(n) if !n.trim().is_empty() => {
            format!("Hello, {}! Welcome to Rust Mini Projects.", n.trim())
        }
        _ => "Hello, World! Welcome to Rust Mini Projects.".to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let name = args.first().map(String::as_str);
    let greeting = generate_greeting(name);
    println!("{greeting}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_greeting() {
        assert_eq!(
            generate_greeting(None),
            "Hello, World! Welcome to Rust Mini Projects."
        );
    }

    #[test]
    fn test_empty_greeting() {
        assert_eq!(
            generate_greeting(Some("   ")),
            "Hello, World! Welcome to Rust Mini Projects."
        );
    }

    #[test]
    fn test_custom_greeting() {
        assert_eq!(
            generate_greeting(Some("Ferris")),
            "Hello, Ferris! Welcome to Rust Mini Projects."
        );
    }
}
