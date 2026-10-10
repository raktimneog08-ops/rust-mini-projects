//! Player spaceship logic, movement physics, weapon modes, and defense systems.

use crate::audio::{AudioQueue, SoundEvent};
use crate::bullet::{Bullet, BulletOwner, BulletType};
use crate::collision::Circle;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    StandardBlaster = 0,
    PlasmaCannon = 1,
    SpreadShot = 2,
    LaserBeam = 3,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub speed: f32,
    pub radius: f32,
    pub health: f32,
    pub max_health: f32,
    pub shield: f32,
    pub max_shield: f32,
    pub shield_regen_timer: f32,
    pub weapon_type: WeaponType,
    pub weapon_level: u8,
    pub fire_cooldown: f32,
    pub emp_charges: u8,
    pub invulnerable_timer: f32,
    pub score: u32,
    pub combo: u32,
    pub combo_timer: f32,
}

impl Player {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            x: screen_width * 0.5,
            y: screen_height * 0.85,
            vx: 0.0,
            vy: 0.0,
            speed: 340.0,
            radius: 18.0,
            health: 100.0,
            max_health: 100.0,
            shield: 100.0,
            max_shield: 100.0,
            shield_regen_timer: 0.0,
            weapon_type: WeaponType::StandardBlaster,
            weapon_level: 1,
            fire_cooldown: 0.0,
            emp_charges: 2,
            invulnerable_timer: 2.0, // Grace period on spawn
            score: 0,
            combo: 0,
            combo_timer: 0.0,
        }
    }

    #[inline]
    pub fn hitbox(&self) -> Circle {
        Circle::new(self.x, self.y, self.radius)
    }

    pub fn update(&mut self, dt: f32, input_dx: f32, input_dy: f32, screen_w: f32, screen_h: f32) {
        // Smooth acceleration and damping
        let target_vx = input_dx * self.speed;
        let target_vy = input_dy * self.speed;
        let lerp_factor = (15.0 * dt).min(1.0);

        self.vx += (target_vx - self.vx) * lerp_factor;
        self.vy += (target_vy - self.vy) * lerp_factor;

        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Clamp to screen bounds
        let margin = self.radius + 4.0;
        self.x = self.x.clamp(margin, screen_w - margin);
        self.y = self.y.clamp(margin, screen_h - margin);

        // Cooldowns and timers
        if self.fire_cooldown > 0.0 {
            self.fire_cooldown -= dt;
        }
        if self.invulnerable_timer > 0.0 {
            self.invulnerable_timer -= dt;
        }

        // Shield regeneration
        self.shield_regen_timer += dt;
        if self.shield_regen_timer >= 3.0 && self.shield < self.max_shield {
            self.shield = (self.shield + 20.0 * dt).min(self.max_shield);
        }

        // Combo timeout decay
        if self.combo_timer > 0.0 {
            self.combo_timer -= dt;
            if self.combo_timer <= 0.0 {
                self.combo = 0;
            }
        }
    }

    pub fn try_fire(&mut self, audio: &mut AudioQueue) -> Vec<Bullet> {
        if self.fire_cooldown > 0.0 {
            return Vec::new();
        }

        let mut bullets = Vec::new();

        match self.weapon_type {
            WeaponType::StandardBlaster => {
                self.fire_cooldown = 0.14 - (self.weapon_level as f32 * 0.015);
                audio.trigger(SoundEvent::LaserShoot);

                if self.weapon_level == 1 {
                    bullets.push(Bullet::new(
                        self.x,
                        self.y - 15.0,
                        0.0,
                        -650.0,
                        4.0,
                        25.0,
                        BulletOwner::Player,
                        BulletType::StandardLaser,
                        2.0,
                    ));
                } else {
                    bullets.push(Bullet::new(
                        self.x - 10.0,
                        self.y - 12.0,
                        0.0,
                        -650.0,
                        4.0,
                        22.0,
                        BulletOwner::Player,
                        BulletType::StandardLaser,
                        2.0,
                    ));
                    bullets.push(Bullet::new(
                        self.x + 10.0,
                        self.y - 12.0,
                        0.0,
                        -650.0,
                        4.0,
                        22.0,
                        BulletOwner::Player,
                        BulletType::StandardLaser,
                        2.0,
                    ));
                }
            }
            WeaponType::PlasmaCannon => {
                self.fire_cooldown = 0.28;
                audio.trigger(SoundEvent::PlasmaShoot);
                bullets.push(Bullet::new(
                    self.x,
                    self.y - 18.0,
                    0.0,
                    -500.0,
                    8.0,
                    65.0 + (self.weapon_level as f32 * 15.0),
                    BulletOwner::Player,
                    BulletType::PlasmaOrb,
                    2.5,
                ));
            }
            WeaponType::SpreadShot => {
                self.fire_cooldown = 0.20;
                audio.trigger(SoundEvent::SpreadShoot);
                let count = 3 + self.weapon_level as usize;
                let spread_angle = 0.35;
                let angle_step = spread_angle / (count - 1) as f32;
                let start_angle = -std::f32::consts::PI * 0.5 - spread_angle * 0.5;

                for i in 0..count {
                    let angle = start_angle + (i as f32 * angle_step);
                    let speed = 580.0;
                    bullets.push(Bullet::new(
                        self.x,
                        self.y - 12.0,
                        angle.cos() * speed,
                        angle.sin() * speed,
                        3.5,
                        18.0,
                        BulletOwner::Player,
                        BulletType::SpreadPellet,
                        1.8,
                    ));
                }
            }
            WeaponType::LaserBeam => {
                self.fire_cooldown = 0.08;
                audio.trigger(SoundEvent::LaserShoot);
                bullets.push(Bullet::new(
                    self.x,
                    self.y - 15.0,
                    0.0,
                    -900.0,
                    5.0,
                    30.0,
                    BulletOwner::Player,
                    BulletType::StandardLaser,
                    1.0,
                ));
            }
        }

        bullets
    }

    pub fn take_damage(&mut self, amount: f32, audio: &mut AudioQueue) -> bool {
        if self.invulnerable_timer > 0.0 {
            return false;
        }

        self.shield_regen_timer = 0.0;
        self.combo = 0; // Break combo on hit
        audio.trigger(SoundEvent::PlayerHit);

        if self.shield >= amount {
            self.shield -= amount;
        } else {
            let overflow = amount - self.shield;
            self.shield = 0.0;
            self.health -= overflow;
            audio.trigger(SoundEvent::ShieldLow);
        }

        self.invulnerable_timer = 0.6; // Brief invulnerability
        self.health <= 0.0
    }

    pub fn add_score(&mut self, base_points: u32) {
        self.combo += 1;
        self.combo_timer = 3.5;
        let multiplier = 1.0 + ((self.combo.min(50) as f32) * 0.1);
        let added = (base_points as f32 * multiplier) as u32;
        self.score += added;
    }

    pub fn use_emp(&mut self, audio: &mut AudioQueue) -> bool {
        if self.emp_charges > 0 {
            self.emp_charges -= 1;
            audio.trigger(SoundEvent::EmpBlast);
            true
        } else {
            false
        }
    }
}
