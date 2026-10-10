//! Main Game Engine loop, state machine, wave progression, and physics update.

use crate::audio::{AudioQueue, SoundEvent};
use crate::bullet::{Bullet, BulletOwner};
use crate::enemy::{Enemy, EnemyType, Powerup, PowerupType};
use crate::particle::ParticleSystem;
use crate::player::{Player, WeaponType};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Menu = 0,
    Playing = 1,
    Paused = 2,
    GameOver = 3,
    Victory = 4,
}

pub struct Game {
    pub screen_width: f32,
    pub screen_height: f32,
    pub state: GameState,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
    pub powerups: Vec<Powerup>,
    pub particles: ParticleSystem,
    pub audio_queue: AudioQueue,
    pub wave: u32,
    pub wave_timer: f32,
    pub wave_enemies_remaining: u32,
    pub boss_spawned: bool,
    pub screen_shake: f32,
    pub high_score: u32,
    rng_seed: u32,
}

impl Game {
    pub fn new(width: f32, height: f32) -> Self {
        let mut g = Self {
            screen_width: width,
            screen_height: height,
            state: GameState::Menu,
            player: Player::new(width, height),
            enemies: Vec::with_capacity(64),
            bullets: Vec::with_capacity(256),
            powerups: Vec::with_capacity(16),
            particles: ParticleSystem::new(width, height),
            audio_queue: AudioQueue::new(),
            wave: 1,
            wave_timer: 2.0,
            wave_enemies_remaining: 0,
            boss_spawned: false,
            screen_shake: 0.0,
            high_score: 0,
            rng_seed: 987654321,
        };
        g.start_game();
        g
    }

    fn next_rand_u32(&mut self) -> u32 {
        let mut x = self.rng_seed;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng_seed = x;
        x
    }

    pub fn next_rand_f32(&mut self) -> f32 {
        (self.next_rand_u32() as f32) / (u32::MAX as f32)
    }

    pub fn start_game(&mut self) {
        self.state = GameState::Playing;
        self.player = Player::new(self.screen_width, self.screen_height);
        self.enemies.clear();
        self.bullets.clear();
        self.powerups.clear();
        self.wave = 1;
        self.wave_timer = 1.0;
        self.boss_spawned = false;
        self.init_wave();
    }

    pub fn restart(&mut self) {
        if self.player.score > self.high_score {
            self.high_score = self.player.score;
        }
        self.start_game();
    }

    pub fn init_wave(&mut self) {
        if self.wave.is_multiple_of(5) {
            // Boss Wave
            self.boss_spawned = true;
            self.wave_enemies_remaining = 1;
            self.audio_queue.trigger(SoundEvent::BossWarning);
        } else {
            self.boss_spawned = false;
            self.wave_enemies_remaining = 8 + (self.wave * 4);
        }
    }

    pub fn trigger_emp(&mut self) {
        if self.player.use_emp(&mut self.audio_queue) {
            self.screen_shake = 12.0;
            self.particles.emit_emp_wave(self.player.x, self.player.y);

            // Destroy all enemy bullets
            self.bullets.retain(|b| b.owner == BulletOwner::Player);

            // Damage all enemies on screen
            for enemy in &mut self.enemies {
                enemy.health -= 250.0;
                self.particles.emit_explosion(enemy.x, enemy.y, 15, false);
            }
        }
    }

    pub fn update(&mut self, dt: f32, input_dx: f32, input_dy: f32, is_firing: bool) {
        if self.state != GameState::Playing {
            return;
        }

        // Screen shake decay
        if self.screen_shake > 0.0 {
            self.screen_shake = (self.screen_shake - dt * 20.0).max(0.0);
        }

        // Update player
        self.player.update(
            dt,
            input_dx,
            input_dy,
            self.screen_width,
            self.screen_height,
        );

        // Player thruster particle emission
        if input_dy.abs() > 0.05 || input_dx.abs() > 0.05 || true {
            self.particles
                .emit_thruster_spark(self.player.x, self.player.y + 16.0);
        }

        // Player firing
        if is_firing {
            let mut new_bullets = self.player.try_fire(&mut self.audio_queue);
            self.bullets.append(&mut new_bullets);
        }

        // Wave Spawning Director
        self.update_wave_spawning(dt);

        // Update Enemies & their shooting
        let mut enemy_bullets = Vec::new();
        let target_x = self.player.x;
        for enemy in &mut self.enemies {
            enemy.update(dt, self.screen_width, target_x);
            let mut b = enemy.try_shoot(&mut self.audio_queue);
            enemy_bullets.append(&mut b);
        }
        self.bullets.append(&mut enemy_bullets);

        // Remove offscreen enemies (non-boss)
        let screen_h = self.screen_height;
        self.enemies.retain(|e| {
            e.y < screen_h + 40.0 && (e.health > 0.0 || e.enemy_type == EnemyType::Boss)
        });

        // Update Bullets
        for bullet in &mut self.bullets {
            bullet.update(dt);
        }
        let sw = self.screen_width;
        let sh = self.screen_height;
        self.bullets.retain(|b| b.is_alive(sw, sh));

        // Update Powerups
        for p in &mut self.powerups {
            p.update(dt);
        }
        self.powerups.retain(|p| p.life > 0.0 && p.y < sh + 20.0);

        // Collisions: Player Bullets vs Enemies
        self.resolve_player_bullet_collisions();

        // Collisions: Enemy Bullets vs Player
        self.resolve_enemy_bullet_collisions();

        // Collisions: Enemies vs Player Ship (Ramming)
        self.resolve_ship_to_ship_collisions();

        // Collisions: Player vs Powerups
        self.resolve_powerup_collisions();

        // Update Particles & Stars
        self.particles
            .update(dt, self.screen_width, self.screen_height);

        // Check for wave completion
        if self.enemies.is_empty() && self.wave_enemies_remaining == 0 {
            self.wave += 1;
            self.audio_queue.trigger(SoundEvent::WaveComplete);
            self.init_wave();
        }

        // Check game over
        if self.player.health <= 0.0 {
            self.state = GameState::GameOver;
            self.screen_shake = 18.0;
            self.audio_queue.trigger(SoundEvent::GameOver);
            self.particles
                .emit_explosion(self.player.x, self.player.y, 60, false);
            if self.player.score > self.high_score {
                self.high_score = self.player.score;
            }
        }
    }

    fn update_wave_spawning(&mut self, dt: f32) {
        if self.wave_enemies_remaining > 0 {
            self.wave_timer -= dt;
            if self.wave_timer <= 0.0 {
                if self.boss_spawned {
                    self.enemies.push(Enemy::new_boss(self.screen_width));
                    self.wave_enemies_remaining = 0;
                } else {
                    let spawn_count = (1 + (self.wave / 3)).min(self.wave_enemies_remaining);
                    for _ in 0..spawn_count {
                        let rand_x = 40.0 + self.next_rand_f32() * (self.screen_width - 80.0);
                        let roll = self.next_rand_f32();

                        let enemy = if roll < 0.45 {
                            Enemy::new_scout(rand_x, -20.0)
                        } else if roll < 0.75 {
                            Enemy::new_corvette(rand_x, -20.0)
                        } else if roll < 0.90 {
                            Enemy::new_kamikaze(rand_x, -20.0)
                        } else {
                            Enemy::new_dreadnought(rand_x, -30.0)
                        };

                        self.enemies.push(enemy);
                    }
                    self.wave_enemies_remaining -= spawn_count;
                    self.wave_timer = 1.8 - (self.wave as f32 * 0.08).min(1.2);
                }
            }
        }
    }

    fn resolve_player_bullet_collisions(&mut self) {
        let mut defeated_enemies = Vec::new();

        for bullet in self
            .bullets
            .iter_mut()
            .filter(|b| b.owner == BulletOwner::Player && b.life > 0.0)
        {
            let b_hitbox = bullet.hitbox();
            for (idx, enemy) in self.enemies.iter_mut().enumerate() {
                if enemy.health > 0.0 && enemy.hitbox().intersects(&b_hitbox) {
                    bullet.life = 0.0; // Consume bullet
                    let is_dead = enemy.take_damage(bullet.damage, &mut self.audio_queue);
                    self.particles.emit_explosion(bullet.x, bullet.y, 4, false);

                    if is_dead {
                        defeated_enemies.push((
                            idx,
                            enemy.x,
                            enemy.y,
                            enemy.enemy_type,
                            enemy.score_value,
                        ));
                    }
                    break;
                }
            }
        }

        // Process destroyed enemies in reverse order
        defeated_enemies.sort_by_key(|b| std::cmp::Reverse(b.0));
        for (idx, x, y, etype, score_val) in defeated_enemies {
            if idx < self.enemies.len() {
                self.enemies.swap_remove(idx);
                self.player.add_score(score_val);
                let is_boss = etype == EnemyType::Boss;
                self.screen_shake = if is_boss { 20.0 } else { 4.0 };
                self.audio_queue.trigger(if is_boss {
                    SoundEvent::BossDefeated
                } else {
                    SoundEvent::SmallExplosion
                });
                self.particles
                    .emit_explosion(x, y, if is_boss { 80 } else { 20 }, is_boss);

                // Chance to drop powerups
                let drop_chance = if is_boss { 1.0 } else { 0.18 };
                if self.next_rand_f32() <= drop_chance {
                    let p_type = match (self.next_rand_f32() * 4.0) as u8 {
                        0 => PowerupType::ShieldBoost,
                        1 => PowerupType::WeaponUpgrade,
                        2 => PowerupType::EmpCharge,
                        _ => PowerupType::ScoreBonus,
                    };
                    self.powerups.push(Powerup::new(x, y, p_type));
                }
            }
        }
    }

    fn resolve_enemy_bullet_collisions(&mut self) {
        let player_hitbox = self.player.hitbox();
        for bullet in self
            .bullets
            .iter_mut()
            .filter(|b| b.owner == BulletOwner::Enemy && b.life > 0.0)
        {
            if bullet.hitbox().intersects(&player_hitbox) {
                bullet.life = 0.0;
                self.player
                    .take_damage(bullet.damage, &mut self.audio_queue);
                self.screen_shake = 6.0;
                self.particles.emit_explosion(bullet.x, bullet.y, 6, false);
            }
        }
    }

    fn resolve_ship_to_ship_collisions(&mut self) {
        let player_hitbox = self.player.hitbox();
        for enemy in &mut self.enemies {
            if enemy.health > 0.0 && enemy.hitbox().intersects(&player_hitbox) {
                self.player.take_damage(40.0, &mut self.audio_queue);
                enemy.take_damage(60.0, &mut self.audio_queue);
                self.screen_shake = 8.0;
                self.particles
                    .emit_explosion(self.player.x, self.player.y, 12, false);
            }
        }
    }

    fn resolve_powerup_collisions(&mut self) {
        let player_hitbox = self.player.hitbox();
        let mut collected = Vec::new();

        for (idx, p) in self.powerups.iter_mut().enumerate() {
            if p.hitbox().intersects(&player_hitbox) {
                collected.push((idx, p.powerup_type));
            }
        }

        collected.sort_by_key(|b| std::cmp::Reverse(b.0));
        for (idx, ptype) in collected {
            if idx < self.powerups.len() {
                self.powerups.swap_remove(idx);
                self.audio_queue.trigger(SoundEvent::PowerupPickup);
                match ptype {
                    PowerupType::ShieldBoost => {
                        self.player.shield =
                            (self.player.shield + 40.0).min(self.player.max_shield);
                    }
                    PowerupType::WeaponUpgrade => {
                        if self.player.weapon_level < 3 {
                            self.player.weapon_level += 1;
                        } else {
                            // Cycle next weapon type on max level
                            self.player.weapon_type = match self.player.weapon_type {
                                WeaponType::StandardBlaster => WeaponType::SpreadShot,
                                WeaponType::SpreadShot => WeaponType::PlasmaCannon,
                                WeaponType::PlasmaCannon => WeaponType::LaserBeam,
                                WeaponType::LaserBeam => WeaponType::StandardBlaster,
                            };
                            self.player.weapon_level = 2;
                        }
                    }
                    PowerupType::EmpCharge => {
                        if self.player.emp_charges < 5 {
                            self.player.emp_charges += 1;
                        }
                    }
                    PowerupType::ScoreBonus => {
                        self.player.add_score(1500);
                    }
                }
            }
        }
    }
}
