use guessing_game_cli::{Difficulty, GuessOutcome, GuessingGame};

#[test]
fn test_game_full_simulation_win() {
    let mut game = GuessingGame::new(77, 1, 100, 7);

    // Binary search approach simulation
    assert_eq!(
        game.guess(50),
        GuessOutcome::TooLow {
            remaining_attempts: 6
        }
    );
    assert_eq!(
        game.guess(75),
        GuessOutcome::TooLow {
            remaining_attempts: 5
        }
    );
    assert_eq!(
        game.guess(88),
        GuessOutcome::TooHigh {
            remaining_attempts: 4
        }
    );
    assert_eq!(
        game.guess(81),
        GuessOutcome::TooHigh {
            remaining_attempts: 3
        }
    );
    assert_eq!(
        game.guess(78),
        GuessOutcome::TooHigh {
            remaining_attempts: 2
        }
    );
    assert_eq!(game.guess(77), GuessOutcome::Correct { attempts_used: 6 });
    assert!(game.is_completed());
}

#[test]
fn test_game_full_simulation_loss() {
    let mut game = GuessingGame::new(10, 1, 20, 3);
    assert_eq!(
        game.guess(1),
        GuessOutcome::TooLow {
            remaining_attempts: 2
        }
    );
    assert_eq!(
        game.guess(2),
        GuessOutcome::TooLow {
            remaining_attempts: 1
        }
    );
    assert_eq!(game.guess(3), GuessOutcome::GameOver { secret: 10 });
    assert!(game.is_completed());
}

#[test]
fn test_custom_difficulty() {
    let custom = Difficulty::Custom {
        min: 100,
        max: 200,
        max_attempts: 4,
    };
    let mut game = GuessingGame::with_difficulty(150, custom);
    assert_eq!(
        game.guess(50),
        GuessOutcome::OutOfBounds { min: 100, max: 200 }
    );
    assert_eq!(game.guess(150), GuessOutcome::Correct { attempts_used: 1 });
}
