//! Project 05: Fibonacci Generator Engine
//!
//! Provides high-performance Fibonacci sequence computation with zero-dependency
//! arbitrary-precision arithmetic (`BigUint`), multiple algorithmic strategies
//! (Iterative, Fast Doubling, Matrix Exponentiation, Memoized, Binet Closed-Form,
//! and Naive Recursive), sequence iterators, and mathematical analytics (Pisano
//! periods, Golden Ratio convergence, and Fibonacci primality/membership checks).

use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

/// Constant base for BigUint limb representation ($10^9$).
const BASE: u64 = 1_000_000_000;

/// Zero-dependency arbitrary-precision unsigned integer represented
/// as little-endian base-$10^9$ limbs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigUint {
    limbs: Vec<u32>,
}

impl BigUint {
    /// Creates a BigUint representing zero.
    #[must_use]
    pub fn zero() -> Self {
        Self { limbs: vec![0] }
    }

    /// Creates a BigUint representing one.
    #[must_use]
    pub fn one() -> Self {
        Self { limbs: vec![1] }
    }

    /// Creates a BigUint from a single `u64` value.
    #[must_use]
    pub fn from_u64(mut val: u64) -> Self {
        if val == 0 {
            return Self::zero();
        }
        let mut limbs = Vec::new();
        while val > 0 {
            limbs.push((val % BASE) as u32);
            val /= BASE;
        }
        Self { limbs }
    }

    /// Creates a BigUint from a single `u128` value.
    #[must_use]
    pub fn from_u128(mut val: u128) -> Self {
        if val == 0 {
            return Self::zero();
        }
        let base_128 = BASE as u128;
        let mut limbs = Vec::new();
        while val > 0 {
            limbs.push((val % base_128) as u32);
            val /= base_128;
        }
        Self { limbs }
    }

    /// Returns `true` if the value is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty() || (self.limbs.len() == 1 && self.limbs[0] == 0)
    }

    /// Returns the exact count of decimal digits.
    #[must_use]
    pub fn digit_count(&self) -> usize {
        if self.is_zero() {
            return 1;
        }
        let last = *self.limbs.last().unwrap_or(&0);
        let last_digits = if last == 0 {
            1
        } else {
            (last as f64).log10().floor() as usize + 1
        };
        (self.limbs.len() - 1) * 9 + last_digits
    }

    /// Normalizes limbs by trimming leading zeros.
    fn normalize(&mut self) {
        while self.limbs.len() > 1 && self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
        if self.limbs.is_empty() {
            self.limbs.push(0);
        }
    }

    /// Subtracts `other` from `self`, requiring `self >= other`.
    #[must_use]
    pub fn sub(&self, other: &Self) -> Self {
        assert!(self >= other, "BigUint subtraction underflow");
        let mut limbs = Vec::with_capacity(self.limbs.len());
        let mut borrow = 0i64;

        for i in 0..self.limbs.len() {
            let a_val = self.limbs[i] as i64;
            let b_val = other.limbs.get(i).copied().unwrap_or(0) as i64;
            let mut diff = a_val - b_val - borrow;
            if diff < 0 {
                diff += BASE as i64;
                borrow = 1;
            } else {
                borrow = 0;
            }
            limbs.push(diff as u32);
        }

        let mut result = Self { limbs };
        result.normalize();
        result
    }

    /// Multiplies the BigUint by a small `u32` scalar.
    #[must_use]
    pub fn mul_scalar(&self, scalar: u32) -> Self {
        if scalar == 0 || self.is_zero() {
            return Self::zero();
        }
        if scalar == 1 {
            return self.clone();
        }

        let scalar_u64 = scalar as u64;
        let mut limbs = Vec::with_capacity(self.limbs.len() + 1);
        let mut carry = 0u64;

        for &digit in &self.limbs {
            let cur = (digit as u64) * scalar_u64 + carry;
            limbs.push((cur % BASE) as u32);
            carry = cur / BASE;
        }

        if carry > 0 {
            limbs.push(carry as u32);
        }

        let mut res = Self { limbs };
        res.normalize();
        res
    }
}

impl Default for BigUint {
    fn default() -> Self {
        Self::zero()
    }
}

impl From<u64> for BigUint {
    fn from(val: u64) -> Self {
        Self::from_u64(val)
    }
}

impl From<u128> for BigUint {
    fn from(val: u128) -> Self {
        Self::from_u128(val)
    }
}

impl From<usize> for BigUint {
    fn from(val: usize) -> Self {
        Self::from_u64(val as u64)
    }
}

impl std::ops::Add for &BigUint {
    type Output = BigUint;

    fn add(self, rhs: Self) -> BigUint {
        let max_len = self.limbs.len().max(rhs.limbs.len());
        let mut limbs = Vec::with_capacity(max_len + 1);
        let mut carry = 0u64;

        for i in 0..max_len {
            let a = self.limbs.get(i).copied().unwrap_or(0) as u64;
            let b = rhs.limbs.get(i).copied().unwrap_or(0) as u64;
            let sum = a + b + carry;
            limbs.push((sum % BASE) as u32);
            carry = sum / BASE;
        }

        if carry > 0 {
            limbs.push(carry as u32);
        }

        let mut res = BigUint { limbs };
        res.normalize();
        res
    }
}

impl std::ops::Add for BigUint {
    type Output = BigUint;
    fn add(self, rhs: Self) -> BigUint {
        &self + &rhs
    }
}

impl std::ops::Add<&BigUint> for BigUint {
    type Output = BigUint;
    fn add(self, rhs: &BigUint) -> BigUint {
        &self + rhs
    }
}

impl std::ops::Add<BigUint> for &BigUint {
    type Output = BigUint;
    fn add(self, rhs: BigUint) -> BigUint {
        self + &rhs
    }
}

impl std::ops::AddAssign<&BigUint> for BigUint {
    fn add_assign(&mut self, rhs: &BigUint) {
        *self = &*self + rhs;
    }
}

impl std::ops::AddAssign<BigUint> for BigUint {
    fn add_assign(&mut self, rhs: BigUint) {
        *self = &*self + &rhs;
    }
}

impl std::ops::Mul for &BigUint {
    type Output = BigUint;

    fn mul(self, rhs: Self) -> BigUint {
        if self.is_zero() || rhs.is_zero() {
            return BigUint::zero();
        }

        let mut result = vec![0u64; self.limbs.len() + rhs.limbs.len()];

        for (i, &x) in self.limbs.iter().enumerate() {
            let mut carry = 0u64;
            let x_128 = x as u128;
            for (j, &y) in rhs.limbs.iter().enumerate() {
                let cur = result[i + j] as u128 + x_128 * (y as u128) + carry as u128;
                result[i + j] = (cur % (BASE as u128)) as u64;
                carry = (cur / (BASE as u128)) as u64;
            }
            if carry > 0 {
                result[i + rhs.limbs.len()] += carry;
            }
        }

        let mut limbs = Vec::with_capacity(result.len());
        let mut carry = 0u64;
        for val in result {
            let cur = val + carry;
            limbs.push((cur % BASE) as u32);
            carry = cur / BASE;
        }
        while carry > 0 {
            limbs.push((carry % BASE) as u32);
            carry /= BASE;
        }

        let mut res = BigUint { limbs };
        res.normalize();
        res
    }
}

impl std::ops::Mul for BigUint {
    type Output = BigUint;
    fn mul(self, rhs: Self) -> BigUint {
        &self * &rhs
    }
}

impl std::ops::Mul<&BigUint> for BigUint {
    type Output = BigUint;
    fn mul(self, rhs: &BigUint) -> BigUint {
        &self * rhs
    }
}

impl std::ops::Mul<BigUint> for &BigUint {
    type Output = BigUint;
    fn mul(self, rhs: BigUint) -> BigUint {
        self * &rhs
    }
}

impl std::ops::MulAssign<&BigUint> for BigUint {
    fn mul_assign(&mut self, rhs: &BigUint) {
        *self = &*self * rhs;
    }
}

impl std::ops::MulAssign<BigUint> for BigUint {
    fn mul_assign(&mut self, rhs: BigUint) {
        *self = &*self * &rhs;
    }
}

impl Ord for BigUint {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.limbs.len() != other.limbs.len() {
            return self.limbs.len().cmp(&other.limbs.len());
        }
        for (a, b) in self.limbs.iter().rev().zip(other.limbs.iter().rev()) {
            match a.cmp(b) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for BigUint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "0");
        }
        let last_idx = self.limbs.len() - 1;
        write!(f, "{}", self.limbs[last_idx])?;
        for i in (0..last_idx).rev() {
            write!(f, "{:09}", self.limbs[i])?;
        }
        Ok(())
    }
}

impl FromStr for BigUint {
    type Err = FibonacciError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(FibonacciError::InvalidInput("Empty string".to_string()));
        }
        if !trimmed.chars().all(|c| c.is_ascii_digit()) {
            return Err(FibonacciError::InvalidInput(format!(
                "Invalid digits in '{trimmed}'"
            )));
        }

        let mut result = BigUint::zero();
        let chunk_size = 9;
        let mut end = trimmed.len();

        let mut multiplier = BigUint::one();
        while end > 0 {
            let start = end.saturating_sub(chunk_size);
            let chunk_str = &trimmed[start..end];
            let chunk_val: u32 = chunk_str
                .parse()
                .map_err(|e| FibonacciError::InvalidInput(format!("Parse error: {e}")))?;

            let term = BigUint::from_u64(chunk_val as u64) * &multiplier;
            result += &term;

            multiplier = &multiplier * &BigUint::from_u64(BASE);
            end = start;
        }

        result.normalize();
        Ok(result)
    }
}

/// Algorithmic strategy for calculating Fibonacci numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Algorithm {
    /// Iterative two-register evaluation ($O(n)$ time, $O(1)$ space).
    Iterative,
    /// Fast Doubling logarithmic recursion ($O(\log n)$ time).
    FastDoubling,
    /// $2 \times 2$ Matrix binary exponentiation ($O(\log n)$ time).
    Matrix,
    /// Top-down memoized evaluation ($O(n)$ time, $O(n)$ space).
    Memoized,
    /// Binet's closed-form analytic formula using golden ratio $\phi$.
    Binet,
    /// Naive recursive evaluation ($O(2^n)$ time; restricted to $n \le 40$).
    Recursive,
}

impl Algorithm {
    /// Returns the canonical name of the algorithm.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Algorithm::Iterative => "Iterative (Linear Accumulator)",
            Algorithm::FastDoubling => "Fast Doubling (Logarithmic)",
            Algorithm::Matrix => "Matrix Exponentiation (Logarithmic)",
            Algorithm::Memoized => "Memoized (Dynamic Programming)",
            Algorithm::Binet => "Binet's Analytic Closed-Form",
            Algorithm::Recursive => "Naive Recursive (Exponential)",
        }
    }

    /// Returns time complexity in Big-O notation.
    #[must_use]
    pub const fn time_complexity(&self) -> &'static str {
        match self {
            Algorithm::Iterative => "O(n)",
            Algorithm::FastDoubling => "O(log n)",
            Algorithm::Matrix => "O(log n)",
            Algorithm::Memoized => "O(n)",
            Algorithm::Binet => "O(1) [approx]",
            Algorithm::Recursive => "O(2^n)",
        }
    }

    /// Returns space complexity in Big-O notation.
    #[must_use]
    pub const fn space_complexity(&self) -> &'static str {
        match self {
            Algorithm::Iterative => "O(1)",
            Algorithm::FastDoubling => "O(log n)",
            Algorithm::Matrix => "O(1)",
            Algorithm::Memoized => "O(n)",
            Algorithm::Binet => "O(1)",
            Algorithm::Recursive => "O(n) [call stack]",
        }
    }

    /// All algorithms in a slice.
    pub const ALL: [Algorithm; 6] = [
        Algorithm::Iterative,
        Algorithm::FastDoubling,
        Algorithm::Matrix,
        Algorithm::Memoized,
        Algorithm::Binet,
        Algorithm::Recursive,
    ];
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Algorithm {
    type Err = FibonacciError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned = s.trim().to_lowercase().replace(['-', '_', ' '], "");
        match cleaned.as_str() {
            "iterative" | "iter" | "linear" => Ok(Algorithm::Iterative),
            "fastdoubling" | "doubling" | "fd" => Ok(Algorithm::FastDoubling),
            "matrix" | "mat" | "exponentiation" => Ok(Algorithm::Matrix),
            "memoized" | "memo" | "dp" => Ok(Algorithm::Memoized),
            "binet" | "analytic" | "closedform" | "formula" => Ok(Algorithm::Binet),
            "recursive" | "naive" | "rec" => Ok(Algorithm::Recursive),
            _ => Err(FibonacciError::InvalidAlgorithm(s.to_string())),
        }
    }
}

/// Errors occurring during Fibonacci computation or parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FibonacciError {
    /// Primitive integer overflow when $n$ exceeds native representation capacity.
    PrimitiveOverflow {
        n: usize,
        type_name: &'static str,
        max_n: usize,
    },
    /// Recursion depth ceiling reached for exponential naive algorithm.
    RecursionLimitExceeded { n: usize, max_allowed: usize },
    /// Unknown or unsupported algorithm name specified.
    InvalidAlgorithm(String),
    /// Malformed or invalid input string or parameter.
    InvalidInput(String),
}

impl fmt::Display for FibonacciError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FibonacciError::PrimitiveOverflow {
                n,
                type_name,
                max_n,
            } => {
                write!(
                    f,
                    "Fibonacci index n={n} overflows {type_name} (maximum supported n is {max_n}; use BigUint or --big)"
                )
            }
            FibonacciError::RecursionLimitExceeded { n, max_allowed } => {
                write!(
                    f,
                    "Index n={n} exceeds naive recursion safety ceiling of n<={max_allowed} (O(2^{n}) would freeze CPU)"
                )
            }
            FibonacciError::InvalidAlgorithm(name) => {
                write!(
                    f,
                    "Unknown algorithm '{name}'. Available: iterative, fast-doubling, matrix, memoized, binet, recursive"
                )
            }
            FibonacciError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
        }
    }
}

impl std::error::Error for FibonacciError {}

/// Maximum Fibonacci index representable in `u64` ($F(93) = 12,200,160,415,121,876,738$).
pub const MAX_U64_FIBONACCI_INDEX: usize = 93;

/// Maximum Fibonacci index representable in `u128` ($F(186) = 332,825,110,087,067,562,321,196,029,789,634,457,848$).
pub const MAX_U128_FIBONACCI_INDEX: usize = 186;

/// Maximum safe index for naive recursive evaluation to avoid freezing execution.
pub const MAX_NAIVE_RECURSIVE_INDEX: usize = 40;

/// Calculates the $n$-th Fibonacci number using native `u64` arithmetic.
///
/// # Errors
/// Returns `FibonacciError::PrimitiveOverflow` if $n > 93$.
pub fn fibonacci_u64(n: usize) -> Result<u64, FibonacciError> {
    if n > MAX_U64_FIBONACCI_INDEX {
        return Err(FibonacciError::PrimitiveOverflow {
            n,
            type_name: "u64",
            max_n: MAX_U64_FIBONACCI_INDEX,
        });
    }

    if n == 0 {
        return Ok(0);
    }
    if n == 1 {
        return Ok(1);
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    Ok(b)
}

/// Calculates the $n$-th Fibonacci number using native `u128` arithmetic.
///
/// # Errors
/// Returns `FibonacciError::PrimitiveOverflow` if $n > 186$.
pub fn fibonacci_u128(n: usize) -> Result<u128, FibonacciError> {
    if n > MAX_U128_FIBONACCI_INDEX {
        return Err(FibonacciError::PrimitiveOverflow {
            n,
            type_name: "u128",
            max_n: MAX_U128_FIBONACCI_INDEX,
        });
    }

    if n == 0 {
        return Ok(0);
    }
    if n == 1 {
        return Ok(1);
    }

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    Ok(b)
}

/// Calculates the $n$-th Fibonacci number with arbitrary precision using the specified algorithm.
///
/// # Errors
/// Returns `FibonacciError::RecursionLimitExceeded` if naive recursion is selected with $n > 40$.
pub fn fibonacci_big(n: usize, algo: Algorithm) -> Result<BigUint, FibonacciError> {
    match algo {
        Algorithm::Iterative => Ok(fibonacci_big_iterative(n)),
        Algorithm::FastDoubling => Ok(fibonacci_big_fast_doubling(n)),
        Algorithm::Matrix => Ok(fibonacci_big_matrix(n)),
        Algorithm::Memoized => Ok(fibonacci_big_memoized(n)),
        Algorithm::Binet => fibonacci_binet(n),
        Algorithm::Recursive => fibonacci_big_recursive(n),
    }
}

/// Iterative linear evaluation ($O(n)$ time, $O(1)$ space).
fn fibonacci_big_iterative(n: usize) -> BigUint {
    if n == 0 {
        return BigUint::zero();
    }
    if n == 1 {
        return BigUint::one();
    }

    let mut a = BigUint::zero();
    let mut b = BigUint::one();

    for _ in 2..=n {
        let next = &a + &b;
        a = b;
        b = next;
    }
    b
}

/// Helper for Fast Doubling algorithm: computes $(F(k), F(k+1))$.
///
/// Identities:
/// $F(2k) = F(k) \times (2F(k+1) - F(k))$
/// $F(2k+1) = F(k+1)^2 + F(k)^2$
fn fast_doubling_helper(n: usize) -> (BigUint, BigUint) {
    if n == 0 {
        return (BigUint::zero(), BigUint::one());
    }

    let (fk, fk1) = fast_doubling_helper(n / 2);

    // 2 * F(k+1) - F(k)
    let two_fk1 = fk1.mul_scalar(2);
    let diff = two_fk1.sub(&fk);

    // F(2k) = F(k) * (2*F(k+1) - F(k))
    let f2k = &fk * &diff;

    // F(2k+1) = F(k+1)^2 + F(k)^2
    let fk1_sq = &fk1 * &fk1;
    let fk_sq = &fk * &fk;
    let f2k1 = &fk1_sq + &fk_sq;

    if n.is_multiple_of(2) {
        (f2k, f2k1)
    } else {
        let f2k2 = &f2k + &f2k1;
        (f2k1, f2k2)
    }
}

/// Fast Doubling logarithmic evaluation ($O(\log n)$ time).
fn fibonacci_big_fast_doubling(n: usize) -> BigUint {
    fast_doubling_helper(n).0
}

/// $2 \times 2$ matrix structure for matrix exponentiation.
#[derive(Clone)]
struct Matrix2x2 {
    a: BigUint,
    b: BigUint,
    c: BigUint,
    d: BigUint,
}

impl Matrix2x2 {
    fn identity() -> Self {
        Self {
            a: BigUint::one(),
            b: BigUint::zero(),
            c: BigUint::zero(),
            d: BigUint::one(),
        }
    }

    fn fib_base() -> Self {
        Self {
            a: BigUint::one(),
            b: BigUint::one(),
            c: BigUint::one(),
            d: BigUint::zero(),
        }
    }

    fn mul(&self, rhs: &Self) -> Self {
        // [a b] * [x y] = [a*x + b*z, a*y + b*w]
        // [c d]   [z w]   [c*x + d*z, c*y + d*w]
        let a = &(&self.a * &rhs.a) + &(&self.b * &rhs.c);
        let b = &(&self.a * &rhs.b) + &(&self.b * &rhs.d);
        let c = &(&self.c * &rhs.a) + &(&self.d * &rhs.c);
        let d = &(&self.c * &rhs.b) + &(&self.d * &rhs.d);
        Self { a, b, c, d }
    }

    fn pow(&self, mut exp: usize) -> Self {
        let mut base = self.clone();
        let mut result = Self::identity();
        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            exp /= 2;
        }
        result
    }
}

/// Matrix exponentiation evaluation ($O(\log n)$ time).
fn fibonacci_big_matrix(n: usize) -> BigUint {
    if n == 0 {
        return BigUint::zero();
    }
    let m = Matrix2x2::fib_base().pow(n);
    // [F(n+1) F(n)  ]
    // [F(n)   F(n-1)]
    m.b
}

/// Top-down memoized evaluation ($O(n)$ time, $O(n)$ space).
fn fibonacci_big_memoized(n: usize) -> BigUint {
    if n == 0 {
        return BigUint::zero();
    }
    if n == 1 {
        return BigUint::one();
    }

    let mut memo = Vec::with_capacity(n + 1);
    memo.push(BigUint::zero());
    memo.push(BigUint::one());

    for i in 2..=n {
        let next = &memo[i - 1] + &memo[i - 2];
        memo.push(next);
    }

    memo.pop().unwrap_or_else(BigUint::zero)
}

/// Binet's closed-form analytic formula using double-precision floating point.
///
/// Note: Accurately represents Fibonacci numbers up to $n \approx 78$ due to 53-bit IEEE 754 mantissa.
fn fibonacci_binet(n: usize) -> Result<BigUint, FibonacciError> {
    if n > 78 {
        return Err(FibonacciError::InvalidInput(format!(
            "Binet's floating-point formula loses IEEE-754 precision past n=78 (requested n={n}); use FastDoubling or Iterative"
        )));
    }

    let sqrt_5 = 5.0_f64.sqrt();
    let phi = (1.0 + sqrt_5) / 2.0;
    let psi = (1.0 - sqrt_5) / 2.0;

    let fn_float = (phi.powi(n as i32) - psi.powi(n as i32)) / sqrt_5;
    let rounded = fn_float.round() as u64;

    Ok(BigUint::from_u64(rounded))
}

/// Naive recursive evaluation ($O(2^n)$ time; guarded for $n \le 40$).
fn fibonacci_big_recursive(n: usize) -> Result<BigUint, FibonacciError> {
    if n > MAX_NAIVE_RECURSIVE_INDEX {
        return Err(FibonacciError::RecursionLimitExceeded {
            n,
            max_allowed: MAX_NAIVE_RECURSIVE_INDEX,
        });
    }

    fn rec(k: usize) -> BigUint {
        if k == 0 {
            BigUint::zero()
        } else if k == 1 {
            BigUint::one()
        } else {
            &rec(k - 1) + &rec(k - 2)
        }
    }

    Ok(rec(n))
}

/// Infinite sequence iterator generating Fibonacci numbers as `BigUint`.
#[derive(Debug, Clone)]
pub struct BigFibonacciIter {
    current: BigUint,
    next: BigUint,
    index: usize,
}

impl BigFibonacciIter {
    /// Creates a new `BigFibonacciIter` starting at index 0.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current: BigUint::zero(),
            next: BigUint::one(),
            index: 0,
        }
    }

    /// Current index reached in the sequence.
    #[must_use]
    pub fn current_index(&self) -> usize {
        self.index
    }
}

impl Default for BigFibonacciIter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for BigFibonacciIter {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.current.clone();
        let new_next = &self.current + &self.next;
        self.current = std::mem::replace(&mut self.next, new_next);
        self.index += 1;
        Some(val)
    }
}

/// Native `u128` sequence iterator that stops when reaching `u128::MAX`.
#[derive(Debug, Clone)]
pub struct FibonacciIter {
    current: u128,
    next: u128,
    index: usize,
}

impl FibonacciIter {
    /// Creates a new `FibonacciIter` starting at index 0.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current: 0,
            next: 1,
            index: 0,
        }
    }

    /// Current index reached in the sequence.
    #[must_use]
    pub fn current_index(&self) -> usize {
        self.index
    }
}

impl Default for FibonacciIter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for FibonacciIter {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > MAX_U128_FIBONACCI_INDEX {
            return None;
        }

        let val = self.current;
        if let Some(new_next) = self.current.checked_add(self.next) {
            self.current = self.next;
            self.next = new_next;
            self.index += 1;
            Some(val)
        } else {
            self.index += 1;
            Some(val)
        }
    }
}

/// Generates a sequence of $N$ Fibonacci numbers as `BigUint`.
#[must_use]
pub fn fibonacci_sequence(count: usize) -> Vec<BigUint> {
    BigFibonacciIter::new().take(count).collect()
}

/// Generates a sequence of $N$ Fibonacci numbers as native `u128`.
///
/// # Errors
/// Returns `FibonacciError::PrimitiveOverflow` if `count > 187`.
pub fn fibonacci_sequence_u128(count: usize) -> Result<Vec<u128>, FibonacciError> {
    if count > MAX_U128_FIBONACCI_INDEX + 1 {
        return Err(FibonacciError::PrimitiveOverflow {
            n: count - 1,
            type_name: "u128",
            max_n: MAX_U128_FIBONACCI_INDEX,
        });
    }
    Ok(FibonacciIter::new().take(count).collect())
}

/// Computes the integer square root $\lfloor \sqrt{n} \rfloor$ for `u128`.
#[must_use]
pub fn integer_sqrt(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    let mut x0 = 1u128 << ((128 - n.leading_zeros()).div_ceil(2));
    loop {
        let x1 = (x0 + n / x0) >> 1;
        if x1 >= x0 {
            return x0;
        }
        x0 = x1;
    }
}

/// Checks whether `n` is a perfect square.
#[must_use]
pub fn is_perfect_square(n: u128) -> bool {
    let s = integer_sqrt(n);
    s * s == n
}

/// Determines whether a given number `x` belongs to the Fibonacci sequence.
///
/// Based on the mathematical theorem: A positive integer $x$ is Fibonacci if and only if
/// $5x^2 + 4$ or $5x^2 - 4$ is a perfect square.
/// For large `u128` values where $5x^2$ could overflow, binary search on the `u128` sequence is used.
#[must_use]
pub fn is_fibonacci(x: u128) -> bool {
    // Up to sqrt(u128::MAX / 5) ~ 8.24e18, 5*x*x + 4 never overflows u128
    if x == 0 {
        return true;
    }

    const OVERFLOW_THRESHOLD: u128 = 8_000_000_000_000_000_000;

    if x <= OVERFLOW_THRESHOLD {
        let x2 = x * x;
        let c1 = 5 * x2 + 4;
        let c2_is_square = (5 * x2).checked_sub(4).is_some_and(is_perfect_square);
        is_perfect_square(c1) || c2_is_square
    } else {
        // Binary search across the 187 pre-computed u128 terms
        let terms: Vec<u128> = FibonacciIter::new().collect();
        terms.binary_search(&x).is_ok()
    }
}

/// Finds the exact index $n$ such that $F(n) = x$, or `None` if `x` is not a Fibonacci number.
#[must_use]
pub fn find_fibonacci_index(x: u128) -> Option<usize> {
    let terms: Vec<u128> = FibonacciIter::new().collect();
    terms.binary_search(&x).ok()
}

/// Golden ratio convergence entry comparing $F(n+1)/F(n)$ to $\phi$.
#[derive(Debug, Clone, PartialEq)]
pub struct GoldenRatioEntry {
    pub n: usize,
    pub fn_val: u128,
    pub fn1_val: u128,
    pub ratio: f64,
    pub delta: f64,
}

/// Constant value of the Golden Ratio $\phi = \frac{1+\sqrt{5}}{2}$.
pub const GOLDEN_RATIO_PHI: f64 = 1.618_033_988_749_895;

/// Generates a table of consecutive ratios $F(n+1)/F(n)$ converging towards $\phi$.
#[must_use]
pub fn golden_ratio_approximations(count: usize) -> Vec<GoldenRatioEntry> {
    let safe_count = count.min(MAX_U128_FIBONACCI_INDEX);
    let terms: Vec<u128> = FibonacciIter::new().take(safe_count + 2).collect();
    let mut entries = Vec::new();

    for n in 1..=safe_count {
        if n >= terms.len() - 1 {
            break;
        }
        let fn_val = terms[n];
        let fn1_val = terms[n + 1];
        let ratio = fn1_val as f64 / fn_val as f64;
        let delta = (ratio - GOLDEN_RATIO_PHI).abs();

        entries.push(GoldenRatioEntry {
            n,
            fn_val,
            fn1_val,
            ratio,
            delta,
        });
    }

    entries
}

/// Computes the Pisano Period for a given integer modulo $m \ge 2$.
///
/// The Pisano period $\pi(m)$ is the period with which the sequence of
/// Fibonacci numbers modulo $m$ repeats.
#[must_use]
pub fn pisano_period(modulo: u64) -> u64 {
    if modulo <= 1 {
        return 1;
    }

    let mut previous: u64 = 0;
    let mut current: u64 = 1;

    // The period always starts with 0, 1 and cannot exceed 6 * m
    for i in 0..(modulo * 6) {
        let temp = (previous + current) % modulo;
        previous = current;
        current = temp;

        if previous == 0 && current == 1 {
            return i + 1;
        }
    }

    modulo * 6
}

/// Simple deterministic primality test for checking prime Fibonacci numbers.
#[must_use]
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n.is_multiple_of(2) || n.is_multiple_of(3) {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n.is_multiple_of(i) || n.is_multiple_of(i + 2) {
            return false;
        }
        i += 6;
    }
    true
}

/// Benchmark result for comparing multiple Fibonacci algorithms.
#[derive(Debug, Clone)]
pub struct AlgorithmBenchmark {
    pub algorithm: Algorithm,
    pub duration: Duration,
    pub digits: usize,
    pub status: Result<(), String>,
}

/// Benchmarks all supported algorithms for a given index $n$.
#[must_use]
pub fn benchmark_algorithms(n: usize) -> Vec<AlgorithmBenchmark> {
    use std::time::Instant;

    let mut results = Vec::new();

    for algo in Algorithm::ALL {
        let start = Instant::now();
        let res = fibonacci_big(n, algo);
        let duration = start.elapsed();

        match res {
            Ok(val) => {
                results.push(AlgorithmBenchmark {
                    algorithm: algo,
                    duration,
                    digits: val.digit_count(),
                    status: Ok(()),
                });
            }
            Err(e) => {
                results.push(AlgorithmBenchmark {
                    algorithm: algo,
                    duration,
                    digits: 0,
                    status: Err(e.to_string()),
                });
            }
        }
    }

    results
}
