use cosmic_defender_wasm::audio::AudioQueue;
use cosmic_defender_wasm::collision::{Circle, Rect};
use cosmic_defender_wasm::enemy::Enemy;
use cosmic_defender_wasm::game::{Game, GameState};
use cosmic_defender_wasm::player::Player;

#[test]
fn test_collision_detection() {
    let c1 = Circle::new(100.0, 100.0, 15.0);
    let c2 = Circle::new(120.0, 100.0, 10.0);
    let c3 = Circle::new(200.0, 200.0, 10.0);

    assert!(c1.intersects(&c2), "c1 and c2 should intersect");
    assert!(!c1.intersects(&c3), "c1 and c3 should not intersect");

    let rect = Rect::new(90.0, 90.0, 20.0, 20.0);
    assert!(rect.intersects_circle(&c1), "rect should intersect c1");
}

#[test]
fn test_player_damage_and_shields() {
    let mut player = Player::new(800.0, 600.0);
    let mut audio = AudioQueue::new();

    assert_eq!(player.shield, 100.0);
    assert_eq!(player.health, 100.0);

    // Take 40 damage: Shield absorbs all
    player.invulnerable_timer = 0.0;
    let is_dead = player.take_damage(40.0, &mut audio);
    assert!(!is_dead);
    assert_eq!(player.shield, 60.0);
    assert_eq!(player.health, 100.0);

    // Take 80 damage: Shield absorbs 60, health takes 20
    player.invulnerable_timer = 0.0;
    let is_dead = player.take_damage(80.0, &mut audio);
    assert!(!is_dead);
    assert_eq!(player.shield, 0.0);
    assert_eq!(player.health, 80.0);
}

#[test]
fn test_player_emp_blast() {
    let mut player = Player::new(800.0, 600.0);
    let mut audio = AudioQueue::new();

    assert_eq!(player.emp_charges, 2);
    assert!(player.use_emp(&mut audio));
    assert_eq!(player.emp_charges, 1);
    assert!(player.use_emp(&mut audio));
    assert_eq!(player.emp_charges, 0);
    assert!(
        !player.use_emp(&mut audio),
        "Cannot use EMP when charges = 0"
    );
}

#[test]
fn test_player_score_and_combo() {
    let mut player = Player::new(800.0, 600.0);

    player.add_score(100);
    assert_eq!(player.combo, 1);
    assert_eq!(player.score, 110); // 100 * (1.0 + 1 * 0.1) = 110

    player.add_score(100);
    assert_eq!(player.combo, 2);
    assert_eq!(player.score, 110 + 120); // +120 = 230
}

#[test]
fn test_enemy_boss_phases() {
    let mut boss = Enemy::new_boss(800.0);
    assert_eq!(boss.boss_phase, 1);

    // Phase 1 -> 2 at 65% HP
    boss.health = 1200.0;
    boss.update(0.1, 800.0, 400.0);
    assert_eq!(boss.boss_phase, 2);

    // Phase 2 -> 3 at 30% HP
    boss.health = 500.0;
    boss.update(0.1, 800.0, 400.0);
    assert_eq!(boss.boss_phase, 3);
}

#[test]
fn test_game_state_and_loop() {
    let mut game = Game::new(800.0, 600.0);
    assert_eq!(game.state, GameState::Playing);
    assert_eq!(game.wave, 1);

    // Simulate 60 frames
    for _ in 0..60 {
        game.update(1.0 / 60.0, 1.0, 0.0, true);
    }

    assert!(
        game.bullets.len() > 0,
        "Player bullets should have been fired"
    );
    assert!(
        game.player.x > 400.0,
        "Player should have moved to the right"
    );
}
