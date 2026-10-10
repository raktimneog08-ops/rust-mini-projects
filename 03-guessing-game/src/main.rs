//! Project 03: Guessing Game CLI
//!
//! Interactive terminal game with difficulty levels, hints, and input validation.

use guessing_game_cli::{Difficulty, GuessOutcome, GuessingGame};
use std::env;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

fn generate_random_number(min: u32, max: u32) -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(123_456_789);

    // 64-bit xorshift PRNG
    let mut x = (nanos as u64) ^ 0x5DEECE66D;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;

    let range = (max - min + 1) as u64;
    min + (x % range) as u32
}

fn print_banner() {
    println!("╔════════════════════════════════════════════╗");
    println!("║       🎯 RUST NUMBER GUESSING GAME         ║");
    println!("╚════════════════════════════════════════════╝");
    println!("Commands: Enter a number to guess, 'hint' for a clue, 'quit' to exit.");
    println!();
}

fn print_usage() {
    println!("Usage: guessing_game_cli [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --easy               Range 1-50 (10 attempts)");
    println!("  --medium             Range 1-100 (7 attempts) [Default]");
    println!("  --hard               Range 1-500 (9 attempts)");
    println!("  --secret <N>         Set a specific secret number (for demos/testing)");
    println!("  --help, -h           Show this help message");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut difficulty = Difficulty::Medium;
    let mut manual_secret: Option<u32> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_usage();
                return;
            }
            "--easy" => difficulty = Difficulty::Easy,
            "--medium" => difficulty = Difficulty::Medium,
            "--hard" => difficulty = Difficulty::Hard,
            "--secret" if i + 1 < args.len() => {
                manual_secret = args[i + 1].parse().ok();
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }

    let (min, max, max_attempts) = difficulty.config();
    let secret = manual_secret.unwrap_or_else(|| generate_random_number(min, max));

    let mut game = GuessingGame::new(secret, min, max, max_attempts);

    print_banner();
    println!("I'm thinking of a number between {min} and {max}. You have {max_attempts} attempts!");
    println!();

    let stdin = io::stdin();
    let mut input = String::new();

    while !game.is_completed() {
        print!("Guess > ");
        io::stdout().flush().unwrap_or(());

        input.clear();
        if stdin.read_line(&mut input).is_err() {
            eprintln!("Failed to read line.");
            break;
        }

        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("quit") || trimmed.eq_ignore_ascii_case("exit") {
            println!("Thanks for playing! The secret was {}.", game.secret());
            break;
        }

        if trimmed.eq_ignore_ascii_case("hint") {
            println!("{}", game.hint());
            continue;
        }

        let guess: u32 = match trimmed.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️  Please enter a valid positive number or 'hint'/'quit'.");
                continue;
            }
        };

        match game.guess(guess) {
            GuessOutcome::Correct { attempts_used } => {
                println!(
                    "🎉 CONGRATULATIONS! You found {} in {} attempt(s)!",
                    game.secret(),
                    attempts_used
                );
            }
            GuessOutcome::TooLow { remaining_attempts } => {
                println!("📉 Too LOW! ({remaining_attempts} attempts remaining)");
            }
            GuessOutcome::TooHigh { remaining_attempts } => {
                println!("📈 Too HIGH! ({remaining_attempts} attempts remaining)");
            }
            GuessOutcome::OutOfBounds { min, max } => {
                println!("🚫 Out of range! Guess must be between {min} and {max}.");
            }
            GuessOutcome::GameOver { secret } => {
                println!("💀 GAME OVER! You ran out of attempts.");
                println!("The secret number was: {secret}");
            }
            GuessOutcome::AlreadyFinished => {
                break;
            }
        }
    }
}
