//! Project 03: Guessing Game Engine
//!
//! Provides deterministic game state evaluation, difficulty presets,
//! hint generation, and attempt tracking.

use std::cmp::Ordering;

/// Difficulty presets for the Guessing Game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Custom {
        min: u32,
        max: u32,
        max_attempts: u32,
    },
}

impl Difficulty {
    #[must_use]
    pub fn config(&self) -> (u32, u32, u32) {
        match *self {
            Difficulty::Easy => (1, 50, 10),
            Difficulty::Medium => (1, 100, 7),
            Difficulty::Hard => (1, 500, 9),
            Difficulty::Custom {
                min,
                max,
                max_attempts,
            } => (min, max, max_attempts),
        }
    }
}

/// The outcome of an attempted guess.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuessOutcome {
    Correct { attempts_used: u32 },
    TooLow { remaining_attempts: u32 },
    TooHigh { remaining_attempts: u32 },
    OutOfBounds { min: u32, max: u32 },
    GameOver { secret: u32 },
    AlreadyFinished,
}

/// Core game state machine.
#[derive(Debug, Clone)]
pub struct GuessingGame {
    secret: u32,
    min: u32,
    max: u32,
    max_attempts: u32,
    attempts_used: u32,
    is_completed: bool,
}

impl GuessingGame {
    /// Creates a new game instance with a known secret (ideal for deterministic testing or RNG injection).
    #[must_use]
    pub fn new(secret: u32, min: u32, max: u32, max_attempts: u32) -> Self {
        Self {
            secret,
            min,
            max,
            max_attempts,
            attempts_used: 0,
            is_completed: false,
        }
    }

    /// Creates a new game using a difficulty preset and a provided secret.
    #[must_use]
    pub fn with_difficulty(secret: u32, difficulty: Difficulty) -> Self {
        let (min, max, max_attempts) = difficulty.config();
        Self::new(secret, min, max, max_attempts)
    }

    /// Evaluates a user guess against the secret.
    pub fn guess(&mut self, number: u32) -> GuessOutcome {
        if self.is_completed {
            return GuessOutcome::AlreadyFinished;
        }

        if number < self.min || number > self.max {
            return GuessOutcome::OutOfBounds {
                min: self.min,
                max: self.max,
            };
        }

        self.attempts_used += 1;

        match number.cmp(&self.secret) {
            Ordering::Equal => {
                self.is_completed = true;
                GuessOutcome::Correct {
                    attempts_used: self.attempts_used,
                }
            }
            Ordering::Less => {
                if self.attempts_used >= self.max_attempts {
                    self.is_completed = true;
                    GuessOutcome::GameOver {
                        secret: self.secret,
                    }
                } else {
                    GuessOutcome::TooLow {
                        remaining_attempts: self.max_attempts - self.attempts_used,
                    }
                }
            }
            Ordering::Greater => {
                if self.attempts_used >= self.max_attempts {
                    self.is_completed = true;
                    GuessOutcome::GameOver {
                        secret: self.secret,
                    }
                } else {
                    GuessOutcome::TooHigh {
                        remaining_attempts: self.max_attempts - self.attempts_used,
                    }
                }
            }
        }
    }

    /// Provides a hint based on the secret number properties.
    #[must_use]
    pub fn hint(&self) -> String {
        let is_even = self.secret.is_multiple_of(2);
        let parity = if is_even { "even" } else { "odd" };
        let midpoint = (self.min + self.max) / 2;
        let half = if self.secret > midpoint {
            "upper half"
        } else {
            "lower half"
        };
        format!(
            "Hint: The secret number is {parity} and in the {half} of [{}-{}]",
            self.min, self.max
        )
    }

    #[must_use]
    pub fn attempts_used(&self) -> u32 {
        self.attempts_used
    }

    #[must_use]
    pub fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    #[must_use]
    pub fn is_completed(&self) -> bool {
        self.is_completed
    }

    #[must_use]
    pub fn secret(&self) -> u32 {
        self.secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_guess_on_first_try() {
        let mut game = GuessingGame::new(42, 1, 100, 5);
        let outcome = game.guess(42);
        assert_eq!(outcome, GuessOutcome::Correct { attempts_used: 1 });
        assert!(game.is_completed());
    }

    #[test]
    fn test_guess_too_low_and_too_high() {
        let mut game = GuessingGame::new(50, 1, 100, 3);
        assert_eq!(
            game.guess(25),
            GuessOutcome::TooLow {
                remaining_attempts: 2
            }
        );
        assert_eq!(
            game.guess(75),
            GuessOutcome::TooHigh {
                remaining_attempts: 1
            }
        );
        assert_eq!(game.guess(50), GuessOutcome::Correct { attempts_used: 3 });
    }

    #[test]
    fn test_game_over_when_attempts_exhausted() {
        let mut game = GuessingGame::new(50, 1, 100, 2);
        assert_eq!(
            game.guess(10),
            GuessOutcome::TooLow {
                remaining_attempts: 1
            }
        );
        assert_eq!(game.guess(20), GuessOutcome::GameOver { secret: 50 });
        assert!(game.is_completed());
        assert_eq!(game.guess(50), GuessOutcome::AlreadyFinished);
    }

    #[test]
    fn test_out_of_bounds() {
        let mut game = GuessingGame::new(50, 1, 100, 3);
        assert_eq!(
            game.guess(0),
            GuessOutcome::OutOfBounds { min: 1, max: 100 }
        );
        assert_eq!(
            game.guess(101),
            GuessOutcome::OutOfBounds { min: 1, max: 100 }
        );
        // Out of bounds guesses do not consume attempts
        assert_eq!(game.attempts_used(), 0);
    }

    #[test]
    fn test_difficulty_presets() {
        let game = GuessingGame::with_difficulty(25, Difficulty::Easy);
        assert_eq!(game.max_attempts(), 10);

        let game_med = GuessingGame::with_difficulty(50, Difficulty::Medium);
        assert_eq!(game_med.max_attempts(), 7);

        let game_hard = GuessingGame::with_difficulty(250, Difficulty::Hard);
        assert_eq!(game_hard.max_attempts(), 9);
    }

    #[test]
    fn test_hint_generation() {
        let game = GuessingGame::new(42, 1, 100, 5);
        let hint = game.hint();
        assert!(hint.contains("even"));
        assert!(hint.contains("lower half"));
    }
}
