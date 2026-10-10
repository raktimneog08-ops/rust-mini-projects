//! Project 02: FizzBuzz Engine
//!
//! Provides modular, generic FizzBuzz evaluation, configurable replacement rules,
//! and streaming iterators for range computations.

/// Evaluates a single number under standard FizzBuzz rules:
/// - Multiples of 15 yield "FizzBuzz"
/// - Multiples of 3 yield "Fizz"
/// - Multiples of 5 yield "Buzz"
/// - All other numbers yield their decimal string representation.
#[must_use]
pub fn fizzbuzz(n: u64) -> String {
    fizzbuzz_custom(n, &[(3, "Fizz"), (5, "Buzz")])
}

/// Evaluates a single number under custom divisor-to-label rules.
///
/// Divisor rules are evaluated in sequential order. When a number is divisible
/// by multiple rules, their labels are concatenated. If no divisors match,
/// the decimal string representation of the number is returned.
#[must_use]
pub fn fizzbuzz_custom(n: u64, rules: &[(u64, &str)]) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    for &(divisor, label) in rules {
        if divisor > 0 && n.is_multiple_of(divisor) {
            result.push_str(label);
        }
    }

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}

/// Returns an iterator yielding FizzBuzz results from `start` through `end` (inclusive).
pub fn fizzbuzz_iter<'a>(
    start: u64,
    end: u64,
    rules: &'a [(u64, &'a str)],
) -> impl Iterator<Item = String> + 'a {
    (start..=end).map(move |n| fizzbuzz_custom(n, rules))
}

/// Computes FizzBuzz results as a vector for a range from `start` to `end` (inclusive).
#[must_use]
pub fn fizzbuzz_range(start: u64, end: u64) -> Vec<String> {
    fizzbuzz_iter(start, end, &[(3, "Fizz"), (5, "Buzz")]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_fizzbuzz_numbers() {
        assert_eq!(fizzbuzz(1), "1");
        assert_eq!(fizzbuzz(2), "2");
        assert_eq!(fizzbuzz(4), "4");
        assert_eq!(fizzbuzz(7), "7");
    }

    #[test]
    fn test_standard_fizz() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(6), "Fizz");
        assert_eq!(fizzbuzz(9), "Fizz");
        assert_eq!(fizzbuzz(12), "Fizz");
    }

    #[test]
    fn test_standard_buzz() {
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(20), "Buzz");
    }

    #[test]
    fn test_standard_fizzbuzz() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(30), "FizzBuzz");
        assert_eq!(fizzbuzz(45), "FizzBuzz");
    }

    #[test]
    fn test_zero_handling() {
        assert_eq!(fizzbuzz(0), "0");
    }

    #[test]
    fn test_custom_rules() {
        let rules = [(2, "Foo"), (3, "Bar"), (5, "Baz")];
        assert_eq!(fizzbuzz_custom(2, &rules), "Foo");
        assert_eq!(fizzbuzz_custom(3, &rules), "Bar");
        assert_eq!(fizzbuzz_custom(6, &rules), "FooBar");
        assert_eq!(fizzbuzz_custom(30, &rules), "FooBarBaz");
        assert_eq!(fizzbuzz_custom(7, &rules), "7");
    }

    #[test]
    fn test_range_computation() {
        let results = fizzbuzz_range(1, 15);
        assert_eq!(
            results,
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}
