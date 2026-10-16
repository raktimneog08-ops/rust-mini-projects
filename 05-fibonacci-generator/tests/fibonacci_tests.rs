use fibonacci_generator_cli::{
    fibonacci_big, fibonacci_sequence, fibonacci_u128, fibonacci_u64, find_fibonacci_index,
    golden_ratio_approximations, is_fibonacci, is_perfect_square, is_prime, pisano_period,
    Algorithm, BigFibonacciIter, BigUint, FibonacciError, FibonacciIter, GOLDEN_RATIO_PHI,
    MAX_U128_FIBONACCI_INDEX, MAX_U64_FIBONACCI_INDEX,
};

const KNOWN_FIBONACCI: [u64; 21] = [
    0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
];

#[test]
fn test_known_fibonacci_u64() {
    for (n, &expected) in KNOWN_FIBONACCI.iter().enumerate() {
        let val = fibonacci_u64(n).expect("u64 computation failed");
        assert_eq!(val, expected, "F({n}) expected {expected}, got {val}");
    }
}

#[test]
fn test_u64_boundary_and_overflow() {
    let f93 = fibonacci_u64(93).expect("F(93) should succeed in u64");
    assert_eq!(f93, 12_200_160_415_121_876_738);

    let err = fibonacci_u64(94).unwrap_err();
    assert_eq!(
        err,
        FibonacciError::PrimitiveOverflow {
            n: 94,
            type_name: "u64",
            max_n: MAX_U64_FIBONACCI_INDEX,
        }
    );
}

#[test]
fn test_u128_boundary_and_overflow() {
    let f186 = fibonacci_u128(186).expect("F(186) should succeed in u128");
    assert_eq!(f186, 332_825_110_087_067_562_321_196_029_789_634_457_848);

    let err = fibonacci_u128(187).unwrap_err();
    assert_eq!(
        err,
        FibonacciError::PrimitiveOverflow {
            n: 187,
            type_name: "u128",
            max_n: MAX_U128_FIBONACCI_INDEX,
        }
    );
}

#[test]
fn test_all_algorithms_consistency_small_n() {
    for n in 0..=35 {
        let expected = fibonacci_big(n, Algorithm::Iterative).unwrap();
        let fast_doubling = fibonacci_big(n, Algorithm::FastDoubling).unwrap();
        let matrix = fibonacci_big(n, Algorithm::Matrix).unwrap();
        let memoized = fibonacci_big(n, Algorithm::Memoized).unwrap();
        let binet = fibonacci_big(n, Algorithm::Binet).unwrap();
        let recursive = fibonacci_big(n, Algorithm::Recursive).unwrap();

        assert_eq!(expected, fast_doubling, "FastDoubling mismatch at n={n}");
        assert_eq!(expected, matrix, "Matrix mismatch at n={n}");
        assert_eq!(expected, memoized, "Memoized mismatch at n={n}");
        assert_eq!(expected, binet, "Binet mismatch at n={n}");
        assert_eq!(expected, recursive, "Recursive mismatch at n={n}");
    }
}

#[test]
fn test_f100_known_value() {
    let f100 = fibonacci_big(100, Algorithm::FastDoubling).unwrap();
    assert_eq!(f100.to_string(), "354224848179261915075");
    assert_eq!(f100.digit_count(), 21);
}

#[test]
fn test_recurrence_relation_large_n() {
    let n = 250;
    let fn_val = fibonacci_big(n, Algorithm::FastDoubling).unwrap();
    let fn1_val = fibonacci_big(n + 1, Algorithm::FastDoubling).unwrap();
    let fn2_val = fibonacci_big(n + 2, Algorithm::FastDoubling).unwrap();

    let sum = &fn_val + &fn1_val;
    assert_eq!(sum, fn2_val, "F({n}) + F({}) != F({})", n + 1, n + 2);
}

#[test]
fn test_biguint_string_roundtrip() {
    let sample = "123456789012345678901234567890";
    let big: BigUint = sample.parse().expect("Parse failed");
    assert_eq!(big.to_string(), sample);
    assert_eq!(big.digit_count(), 30);
}

#[test]
fn test_is_fibonacci_membership() {
    for &num in &KNOWN_FIBONACCI {
        assert!(is_fibonacci(num as u128), "Failed for known fib {num}");
    }

    let non_fibs = [4u128, 6, 7, 9, 10, 11, 12, 14, 15, 100, 500, 1000];
    for &num in &non_fibs {
        assert!(!is_fibonacci(num), "Falsely identified non-fib {num}");
    }
}

#[test]
fn test_find_fibonacci_index() {
    assert_eq!(find_fibonacci_index(0), Some(0));
    assert_eq!(find_fibonacci_index(2), Some(3));
    assert_eq!(find_fibonacci_index(3), Some(4));
    assert_eq!(find_fibonacci_index(5), Some(5));
    assert_eq!(find_fibonacci_index(8), Some(6));
    assert_eq!(find_fibonacci_index(13), Some(7));
    assert_eq!(find_fibonacci_index(6765), Some(20));
    assert_eq!(find_fibonacci_index(14), None);
}

#[test]
fn test_pisano_periods() {
    assert_eq!(pisano_period(2), 3);
    assert_eq!(pisano_period(3), 8);
    assert_eq!(pisano_period(4), 6);
    assert_eq!(pisano_period(5), 20);
    assert_eq!(pisano_period(10), 60);
}

#[test]
fn test_golden_ratio_convergence() {
    let entries = golden_ratio_approximations(20);
    assert_eq!(entries.len(), 20);

    // Delta should monotonically decrease after initial oscillations
    let delta_early = entries[4].delta;
    let delta_late = entries[18].delta;
    assert!(
        delta_late < delta_early,
        "Convergence failed: early={delta_early}, late={delta_late}"
    );
    assert!(delta_late < 1e-7);
    assert!((entries[18].ratio - GOLDEN_RATIO_PHI).abs() < 1e-7);
}

#[test]
fn test_iterators() {
    let u128_terms: Vec<u128> = FibonacciIter::new().take(10).collect();
    let big_terms: Vec<BigUint> = BigFibonacciIter::new().take(10).collect();

    for i in 0..10 {
        assert_eq!(u128_terms[i] as u64, KNOWN_FIBONACCI[i]);
        assert_eq!(big_terms[i].to_string(), KNOWN_FIBONACCI[i].to_string());
    }
}

#[test]
fn test_sequence_generator() {
    let seq = fibonacci_sequence(15);
    assert_eq!(seq.len(), 15);
    for (i, item) in seq.iter().enumerate() {
        assert_eq!(item.to_string(), KNOWN_FIBONACCI[i].to_string());
    }
}

#[test]
fn test_perfect_square() {
    assert!(is_perfect_square(0));
    assert!(is_perfect_square(1));
    assert!(is_perfect_square(4));
    assert!(is_perfect_square(9));
    assert!(is_perfect_square(16));
    assert!(is_perfect_square(1_000_000));
    assert!(!is_perfect_square(2));
    assert!(!is_perfect_square(3));
    assert!(!is_perfect_square(5));
    assert!(!is_perfect_square(99));
}

#[test]
fn test_primality() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
    assert!(is_prime(5));
    assert!(is_prime(13));
    assert!(is_prime(89));
}
