//! Project 05: Fibonacci Generator & Mathematical Analytics CLI
//!
//! Interactive shell and command-line tool for computing arbitrary-precision
//! Fibonacci numbers, analyzing sequence properties, testing primality and
//! membership, benchmarking evaluation algorithms, and exploring golden ratio convergence.

use std::env;
use std::io::{self, Write};
use std::time::Instant;

use fibonacci_generator_cli::{
    benchmark_algorithms, fibonacci_big, fibonacci_sequence, find_fibonacci_index,
    golden_ratio_approximations, is_fibonacci, pisano_period, Algorithm, BigUint, GOLDEN_RATIO_PHI,
    MAX_U128_FIBONACCI_INDEX, MAX_U64_FIBONACCI_INDEX,
};

fn print_banner() {
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("║             🌀 FIBONACCI GENERATOR & ANALYTICS CLI                 ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");
    println!("High-performance arbitrary precision, 6 algorithms & sequence analytics");
    println!();
}

fn print_usage() {
    println!("Usage:");
    println!("  fibonacci_generator_cli <N>                     Calculate N-th Fibonacci number");
    println!("  fibonacci_generator_cli <N> --algo <NAME>       Use specific algorithm (fast-doubling, matrix, etc.)");
    println!(
        "  fibonacci_generator_cli --seq <COUNT>           Generate first COUNT Fibonacci numbers"
    );
    println!("  fibonacci_generator_cli --is-fib <NUMBER>       Check if a number is in the Fibonacci sequence");
    println!(
        "  fibonacci_generator_cli --ratio <COUNT>         Show Golden Ratio (φ) convergence table"
    );
    println!("  fibonacci_generator_cli --pisano <MODULO>       Compute Pisano period modulo M");
    println!(
        "  fibonacci_generator_cli --benchmark <N>         Benchmark all algorithms at index N"
    );
    println!("  fibonacci_generator_cli --help, -h              Display this help guide");
    println!();
    println!("Available Algorithms:");
    println!("  fast-doubling (default for large N)  O(log n) time");
    println!("  iterative                            O(n) time, O(1) space");
    println!("  matrix                               O(log n) time via 2x2 binary exponentiation");
    println!("  memoized                             O(n) time, O(n) space dynamic programming");
    println!(
        "  binet                                O(1) analytic floating-point closed form (n <= 78)"
    );
    println!("  recursive                            O(2^n) naive recursion (n <= 40)");
    println!();
    println!("Examples:");
    println!("  cargo run -p fibonacci_generator_cli -- 50");
    println!("  cargo run -p fibonacci_generator_cli -- 1000 --algo fast-doubling");
    println!("  cargo run -p fibonacci_generator_cli -- --seq 15");
    println!("  cargo run -p fibonacci_generator_cli -- --is-fib 6765");
    println!("  cargo run -p fibonacci_generator_cli -- --ratio 10");
    println!("  cargo run -p fibonacci_generator_cli -- --benchmark 35");
}

fn compute_and_print_n(n: usize, algo: Algorithm) {
    let start = Instant::now();
    match fibonacci_big(n, algo) {
        Ok(result) => {
            let elapsed = start.elapsed();
            let digits = result.digit_count();
            println!();
            println!("┌────────────────────────────────────────────────────────────────────┐");
            println!("│ Fibonacci Term F({n:<5})                                            │");
            println!("├────────────────────────────────────────────────────────────────────┤");
            println!("│ Algorithm:     {:<51} │", algo.name());
            println!("│ Time Elapsed:  {:<51} │", format!("{elapsed:?}"));
            println!("│ Decimal Digits:{:<51} │", digits);

            let type_fit = if n <= MAX_U64_FIBONACCI_INDEX {
                "Fits in u64, u128, BigUint"
            } else if n <= MAX_U128_FIBONACCI_INDEX {
                "Fits in u128, BigUint (overflows u64)"
            } else {
                "Requires BigUint (overflows u64, u128)"
            };
            println!("│ Storage Fit:   {:<51} │", type_fit);
            println!("└────────────────────────────────────────────────────────────────────┘");
            println!();

            let result_str = result.to_string();
            if digits > 120 {
                println!(
                    "Value (truncated preview): {}...{} (total {} digits)",
                    &result_str[..30],
                    &result_str[result_str.len() - 30..],
                    digits
                );
                println!();
                println!("Full Value:");
                println!("{result_str}");
            } else {
                println!("Value: {result_str}");
            }
        }
        Err(e) => {
            eprintln!("Error computing F({n}): {e}");
        }
    }
}

fn print_sequence_table(count: usize) {
    let count = count.min(10_000);
    println!();
    println!("┌───────┬────────────────────────────────────────┬────────┐");
    println!("│ Index │ Value                                  │ Digits │");
    println!("├───────┼────────────────────────────────────────┼────────┤");

    let sequence = fibonacci_sequence(count);
    for (i, val) in sequence.into_iter().enumerate() {
        let val_str = val.to_string();
        let display_val = if val_str.len() > 38 {
            format!("{}...{}", &val_str[..18], &val_str[val_str.len() - 17..])
        } else {
            val_str
        };
        println!(
            "│ {:<5} │ {:>38} │ {:>6} │",
            i,
            display_val,
            val.digit_count()
        );
    }
    println!("└───────┴────────────────────────────────────────┴────────┘");
}

fn print_ratio_table(count: usize) {
    let count = count.clamp(2, 50);
    println!();
    println!("Golden Ratio φ = {:.15}", GOLDEN_RATIO_PHI);
    println!("┌───────┬────────────────────────┬────────────────────────┬───────────────────┬───────────────┐");
    println!("│   n   │ F(n)                   │ F(n+1)                 │ Ratio F(n+1)/F(n) │ |Ratio - φ|   │");
    println!("├───────┼────────────────────────┼────────────────────────┼───────────────────┼───────────────┤");

    let entries = golden_ratio_approximations(count);
    for e in entries {
        println!(
            "│ {:<5} │ {:>22} │ {:>22} │ {:>17.14} │ {:>13.2e} │",
            e.n, e.fn_val, e.fn1_val, e.ratio, e.delta
        );
    }
    println!("└───────┴────────────────────────┴────────────────────────┴───────────────────┴───────────────┘");
}

fn check_membership(num_str: &str) {
    let trimmed = num_str.trim();
    if let Ok(val) = trimmed.parse::<u128>() {
        let is_fib = is_fibonacci(val);
        if is_fib {
            if let Some(idx) = find_fibonacci_index(val) {
                println!("✓ {val} is a Fibonacci number! (Index: F({idx}))");
            } else {
                println!("✓ {val} is a Fibonacci number!");
            }
        } else {
            println!("✗ {val} is NOT a Fibonacci number.");
        }
    } else {
        // Fallback for huge numbers using BigUint
        match trimmed.parse::<BigUint>() {
            Ok(big_val) => {
                // Generate and search
                let mut iter = fibonacci_generator_cli::BigFibonacciIter::new();
                let mut found = false;
                let mut current_idx = 0;
                while current_idx <= 2000 {
                    let term = iter.next().unwrap();
                    if term == big_val {
                        println!("✓ {big_val} is a Fibonacci number! (Index: F({current_idx}))");
                        found = true;
                        break;
                    }
                    if term > big_val {
                        break;
                    }
                    current_idx += 1;
                }
                if !found {
                    println!("✗ {big_val} is NOT a Fibonacci number (within scanned range).");
                }
            }
            Err(e) => eprintln!("Failed to parse number: {e}"),
        }
    }
}

fn run_benchmark_suite(n: usize) {
    println!();
    println!("Benchmarking all algorithms for N = {n}:");
    println!(
        "┌────────────────────────────────────┬───────────┬──────────────┬──────────────────┐"
    );
    println!(
        "│ Algorithm                          │ Complexity│ Time Elapsed │ Status           │"
    );
    println!(
        "├────────────────────────────────────┼───────────┼──────────────┼──────────────────┤"
    );

    let results = benchmark_algorithms(n);
    for res in results {
        let status_str = match &res.status {
            Ok(_) => format!("OK ({} digits)", res.digits),
            Err(e) => {
                if e.contains("safety ceiling") {
                    "Skipped (>40)".to_string()
                } else if e.contains("IEEE-754") {
                    "Loss of precision".to_string()
                } else {
                    "Error".to_string()
                }
            }
        };

        println!(
            "│ {:<34} │ {:<9} │ {:>12?} │ {:<16} │",
            res.algorithm.name(),
            res.algorithm.time_complexity(),
            res.duration,
            status_str
        );
    }
    println!(
        "└────────────────────────────────────┴───────────┴──────────────┴──────────────────┘"
    );
}

fn run_interactive_shell() {
    print_banner();
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        println!("Available Actions:");
        println!("  1) Compute N-th Fibonacci number");
        println!("  2) Generate Fibonacci sequence table");
        println!("  3) Verify Fibonacci membership");
        println!("  4) Golden ratio (φ) convergence analysis");
        println!("  5) Pisano period calculator");
        println!("  6) Run algorithmic benchmark");
        println!("  7) Exit");
        print!("\nSelect an option [1-7]: ");
        io::stdout().flush().unwrap_or(());

        line.clear();
        if stdin.read_line(&mut line).is_err() {
            break;
        }

        match line.trim() {
            "1" => {
                print!("Enter N: ");
                io::stdout().flush().unwrap_or(());
                line.clear();
                if stdin.read_line(&mut line).is_ok() {
                    if let Ok(n) = line.trim().parse::<usize>() {
                        println!("Algorithms: 1) Fast Doubling [default], 2) Iterative, 3) Matrix, 4) Memoized, 5) Binet, 6) Recursive");
                        print!("Select algorithm [1-6, or Enter for default]: ");
                        io::stdout().flush().unwrap_or(());
                        line.clear();
                        let _ = stdin.read_line(&mut line);
                        let algo = match line.trim() {
                            "2" => Algorithm::Iterative,
                            "3" => Algorithm::Matrix,
                            "4" => Algorithm::Memoized,
                            "5" => Algorithm::Binet,
                            "6" => Algorithm::Recursive,
                            _ => Algorithm::FastDoubling,
                        };
                        compute_and_print_n(n, algo);
                    } else {
                        println!("Invalid non-negative integer.");
                    }
                }
            }
            "2" => {
                print!("Enter count of terms to generate (e.g. 20): ");
                io::stdout().flush().unwrap_or(());
                line.clear();
                if stdin.read_line(&mut line).is_ok() {
                    if let Ok(count) = line.trim().parse::<usize>() {
                        print_sequence_table(count);
                    } else {
                        println!("Invalid count.");
                    }
                }
            }
            "3" => {
                print!("Enter number to test: ");
                io::stdout().flush().unwrap_or(());
                line.clear();
                if stdin.read_line(&mut line).is_ok() {
                    check_membership(line.trim());
                }
            }
            "4" => {
                print!("Enter number of convergence steps (e.g. 15): ");
                io::stdout().flush().unwrap_or(());
                line.clear();
                if stdin.read_line(&mut line).is_ok() {
                    if let Ok(count) = line.trim().parse::<usize>() {
                        print_ratio_table(count);
                    } else {
                        println!("Invalid count.");
                    }
                }
            }
            "5" => {
                print!("Enter modulo M (>= 2): ");
                io::stdout().flush().unwrap_or(());
                line.clear();
                if stdin.read_line(&mut line).is_ok() {
                    if let Ok(m) = line.trim().parse::<u64>() {
                        let period = pisano_period(m);
                        println!("Pisano period π({m}) = {period}");
                    } else {
                        println!("Invalid integer modulo.");
                    }
                }
            }
            "6" => {
                print!("Enter N for benchmark (recommended 20 - 40): ");
                io::stdout().flush().unwrap_or(());
                line.clear();
                if stdin.read_line(&mut line).is_ok() {
                    if let Ok(n) = line.trim().parse::<usize>() {
                        run_benchmark_suite(n);
                    } else {
                        println!("Invalid integer.");
                    }
                }
            }
            "7" | "exit" | "quit" | "q" => {
                println!("Exiting Fibonacci CLI. Goodbye!");
                break;
            }
            _ => {
                println!("Please select a valid option from 1 to 7.");
            }
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        run_interactive_shell();
        return;
    }

    let mut i = 1;
    let mut n_arg: Option<usize> = None;
    let mut algo = Algorithm::FastDoubling;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "--help" | "-h" => {
                print_banner();
                print_usage();
                return;
            }
            "--seq" | "-s" => {
                if i + 1 < args.len() {
                    if let Ok(count) = args[i + 1].parse::<usize>() {
                        print_sequence_table(count);
                        return;
                    }
                }
                eprintln!("Error: --seq requires a positive integer count");
                return;
            }
            "--is-fib" | "-c" => {
                if i + 1 < args.len() {
                    check_membership(&args[i + 1]);
                    return;
                }
                eprintln!("Error: --is-fib requires a number argument");
                return;
            }
            "--ratio" | "-r" => {
                if i + 1 < args.len() {
                    if let Ok(count) = args[i + 1].parse::<usize>() {
                        print_ratio_table(count);
                        return;
                    }
                }
                eprintln!("Error: --ratio requires a positive count");
                return;
            }
            "--pisano" | "-p" => {
                if i + 1 < args.len() {
                    if let Ok(m) = args[i + 1].parse::<u64>() {
                        let period = pisano_period(m);
                        println!("Pisano period π({m}) = {period}");
                        return;
                    }
                }
                eprintln!("Error: --pisano requires an integer modulo >= 2");
                return;
            }
            "--benchmark" | "-b" => {
                if i + 1 < args.len() {
                    if let Ok(n) = args[i + 1].parse::<usize>() {
                        run_benchmark_suite(n);
                        return;
                    }
                }
                eprintln!("Error: --benchmark requires an integer index N");
                return;
            }
            "--algo" | "-a" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<Algorithm>() {
                        Ok(parsed_algo) => {
                            algo = parsed_algo;
                            i += 1;
                        }
                        Err(e) => {
                            eprintln!("Error: {e}");
                            return;
                        }
                    }
                } else {
                    eprintln!("Error: --algo requires an algorithm name");
                    return;
                }
            }
            other => {
                if let Ok(val) = other.parse::<usize>() {
                    n_arg = Some(val);
                } else {
                    eprintln!("Unknown argument '{other}'. Run with --help for options.");
                    return;
                }
            }
        }
        i += 1;
    }

    if let Some(n) = n_arg {
        compute_and_print_n(n, algo);
    } else {
        print_usage();
    }
}
