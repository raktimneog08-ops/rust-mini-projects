//! Enemy entities, AI motion behaviors, weapon firing, and boss phases.

use crate::audio::{AudioQueue, SoundEvent};
use crate::bullet::{Bullet, BulletOwner, BulletType};
use crate::collision::Circle;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Scout = 0,       // Fast zigzag
    Corvette = 1,    // Heavy armored, fires pulses
    Kamikaze = 2,    // High-speed rammer
    Dreadnought = 3, // Mini-boss with dual cannons
    Boss = 4,        // Mothership boss with rotating bullet curtains & phases
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerupType {
    ShieldBoost = 0,
    WeaponUpgrade = 1,
    EmpCharge = 2,
    ScoreBonus = 3,
}

#[derive(Debug, Clone, Copy)]
pub struct Powerup {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub powerup_type: PowerupType,
    pub radius: f32,
    pub life: f32,
}

impl Powerup {
    pub fn new(x: f32, y: f32, powerup_type: PowerupType) -> Self {
        Self {
            x,
            y,
            vy: 90.0,
            powerup_type,
            radius: 12.0,
            life: 10.0,
        }
    }

    #[inline]
    pub fn hitbox(&self) -> Circle {
        Circle::new(self.x, self.y, self.radius)
    }

    pub fn update(&mut self, dt: f32) {
        self.y += self.vy * dt;
        self.life -= dt;
    }
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub enemy_type: EnemyType,
    pub health: f32,
    pub max_health: f32,
    pub radius: f32,
    pub shoot_cooldown: f32,
    pub ai_timer: f32,
    pub boss_phase: u8,
    pub score_value: u32,
}

impl Enemy {
    pub fn new_scout(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 120.0,
            vy: 80.0,
            enemy_type: EnemyType::Scout,
            health: 25.0,
            max_health: 25.0,
            radius: 14.0,
            shoot_cooldown: 1.5,
            ai_timer: 0.0,
            boss_phase: 0,
            score_value: 100,
        }
    }

    pub fn new_corvette(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 40.0,
            vy: 45.0,
            enemy_type: EnemyType::Corvette,
            health: 90.0,
            max_health: 90.0,
            radius: 22.0,
            shoot_cooldown: 2.2,
            ai_timer: 0.0,
            boss_phase: 0,
            score_value: 300,
        }
    }

    pub fn new_kamikaze(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 180.0,
            enemy_type: EnemyType::Kamikaze,
            health: 20.0,
            max_health: 20.0,
            radius: 12.0,
            shoot_cooldown: 99.0, // doesn't shoot, only dives
            ai_timer: 0.0,
            boss_phase: 0,
            score_value: 150,
        }
    }

    pub fn new_dreadnought(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 50.0,
            vy: 20.0,
            enemy_type: EnemyType::Dreadnought,
            health: 320.0,
            max_health: 320.0,
            radius: 34.0,
            shoot_cooldown: 1.2,
            ai_timer: 0.0,
            boss_phase: 0,
            score_value: 800,
        }
    }

    pub fn new_boss(screen_w: f32) -> Self {
        Self {
            x: screen_w * 0.5,
            y: 80.0,
            vx: 70.0,
            vy: 0.0,
            enemy_type: EnemyType::Boss,
            health: 2000.0,
            max_health: 2000.0,
            radius: 55.0,
            shoot_cooldown: 0.6,
            ai_timer: 0.0,
            boss_phase: 1,
            score_value: 10000,
        }
    }

    #[inline]
    pub fn hitbox(&self) -> Circle {
        Circle::new(self.x, self.y, self.radius)
    }

    pub fn update(&mut self, dt: f32, screen_w: f32, target_player_x: f32) {
        self.ai_timer += dt;
        if self.shoot_cooldown > 0.0 {
            self.shoot_cooldown -= dt;
        }

        match self.enemy_type {
            EnemyType::Scout => {
                // Sine wave zig-zag
                self.x += (self.ai_timer * 4.0).sin() * self.vx * dt * 2.0;
                self.y += self.vy * dt;
            }
            EnemyType::Corvette => {
                self.x += self.vx * dt;
                if self.x < 50.0 || self.x > screen_w - 50.0 {
                    self.vx = -self.vx;
                }
                self.y += self.vy * dt;
            }
            EnemyType::Kamikaze => {
                // Steer aggressively towards player X
                let dx = target_player_x - self.x;
                self.vx += dx.clamp(-120.0, 120.0) * dt * 2.0;
                self.vx = self.vx.clamp(-200.0, 200.0);
                self.x += self.vx * dt;
                self.y += self.vy * dt;
            }
            EnemyType::Dreadnought => {
                self.x += self.vx * dt;
                if self.x < 80.0 || self.x > screen_w - 80.0 {
                    self.vx = -self.vx;
                }
                if self.y < 120.0 {
                    self.y += self.vy * dt;
                }
            }
            EnemyType::Boss => {
                // Boss movement pattern based on phase
                self.x += self.vx * dt;
                if self.x < 100.0 || self.x > screen_w - 100.0 {
                    self.vx = -self.vx;
                }

                // Phase transitions based on HP
                let hp_ratio = self.health / self.max_health;
                if hp_ratio <= 0.3 {
                    self.boss_phase = 3;
                } else if hp_ratio <= 0.65 {
                    self.boss_phase = 2;
                } else {
                    self.boss_phase = 1;
                }
            }
        }
    }

    pub fn try_shoot(&mut self, audio: &mut AudioQueue) -> Vec<Bullet> {
        if self.shoot_cooldown > 0.0 {
            return Vec::new();
        }

        let mut bullets = Vec::new();

        match self.enemy_type {
            EnemyType::Scout => {
                self.shoot_cooldown = 1.6;
                bullets.push(Bullet::new(
                    self.x,
                    self.y + 12.0,
                    0.0,
                    260.0,
                    3.5,
                    15.0,
                    BulletOwner::Enemy,
                    BulletType::EnemyPulse,
                    3.5,
                ));
            }
            EnemyType::Corvette => {
                self.shoot_cooldown = 1.8;
                bullets.push(Bullet::new(
                    self.x - 12.0,
                    self.y + 16.0,
                    -40.0,
                    240.0,
                    4.5,
                    20.0,
                    BulletOwner::Enemy,
                    BulletType::EnemyPulse,
                    3.5,
                ));
                bullets.push(Bullet::new(
                    self.x + 12.0,
                    self.y + 16.0,
                    40.0,
                    240.0,
                    4.5,
                    20.0,
                    BulletOwner::Enemy,
                    BulletType::EnemyPulse,
                    3.5,
                ));
            }
            EnemyType::Kamikaze => {
                // No projectiles
            }
            EnemyType::Dreadnought => {
                self.shoot_cooldown = 1.0;
                let angles = [-0.25, 0.0, 0.25];
                for a in angles {
                    let speed = 250.0;
                    let angle = std::f32::consts::PI * 0.5 + a;
                    bullets.push(Bullet::new(
                        self.x,
                        self.y + 20.0,
                        angle.cos() * speed,
                        angle.sin() * speed,
                        4.5,
                        18.0,
                        BulletOwner::Enemy,
                        BulletType::EnemyPulse,
                        4.0,
                    ));
                }
            }
            EnemyType::Boss => {
                match self.boss_phase {
                    1 => {
                        // 5-way spread
                        self.shoot_cooldown = 0.8;
                        audio.trigger(SoundEvent::EnemyHit);
                        for i in 0..5 {
                            let angle = std::f32::consts::PI * 0.5 + ((i as f32 - 2.0) * 0.22);
                            let speed = 260.0;
                            bullets.push(Bullet::new(
                                self.x,
                                self.y + 35.0,
                                angle.cos() * speed,
                                angle.sin() * speed,
                                5.0,
                                20.0,
                                BulletOwner::Enemy,
                                BulletType::BossRing,
                                4.0,
                            ));
                        }
                    }
                    2 => {
                        // Spiral bullet ring
                        self.shoot_cooldown = 0.45;
                        let ring_count = 8;
                        let offset = self.ai_timer * 3.0;
                        for i in 0..ring_count {
                            let angle = offset
                                + (i as f32 * (std::f32::consts::PI * 2.0 / ring_count as f32));
                            let speed = 220.0;
                            bullets.push(Bullet::new(
                                self.x,
                                self.y + 30.0,
                                angle.cos() * speed,
                                angle.sin() * speed,
                                4.5,
                                18.0,
                                BulletOwner::Enemy,
                                BulletType::BossRing,
                                4.0,
                            ));
                        }
                    }
                    _ => {
                        // Phase 3: Frenzy bullet storm + high speed pulses
                        self.shoot_cooldown = 0.25;
                        let ring_count = 12;
                        let offset = self.ai_timer * 5.0;
                        for i in 0..ring_count {
                            let angle = offset
                                + (i as f32 * (std::f32::consts::PI * 2.0 / ring_count as f32));
                            let speed = 250.0;
                            bullets.push(Bullet::new(
                                self.x,
                                self.y + 30.0,
                                angle.cos() * speed,
                                angle.sin() * speed,
                                5.0,
                                22.0,
                                BulletOwner::Enemy,
                                BulletType::BossRing,
                                4.0,
                            ));
                        }
                    }
                }
            }
        }

        bullets
    }

    pub fn take_damage(&mut self, damage: f32, audio: &mut AudioQueue) -> bool {
        self.health -= damage;
        audio.trigger(SoundEvent::EnemyHit);
        self.health <= 0.0
    }
}
