//! Projectile entities for player and enemy attacks.

use crate::collision::Circle;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulletOwner {
    Player = 0,
    Enemy = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulletType {
    StandardLaser = 0,
    PlasmaOrb = 1,
    SpreadPellet = 2,
    EnemyPulse = 3,
    EnemyHoming = 4,
    BossRing = 5,
    BossBeamPart = 6,
}

#[derive(Debug, Clone, Copy)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub damage: f32,
    pub owner: BulletOwner,
    pub bullet_type: BulletType,
    pub life: f32,
}

impl Bullet {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        x: f32,
        y: f32,
        vx: f32,
        vy: f32,
        radius: f32,
        damage: f32,
        owner: BulletOwner,
        bullet_type: BulletType,
        life: f32,
    ) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            radius,
            damage,
            owner,
            bullet_type,
            life,
        }
    }

    #[inline]
    pub fn hitbox(&self) -> Circle {
        Circle::new(self.x, self.y, self.radius)
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
        self.life -= dt;
    }

    #[inline]
    pub fn is_alive(&self, screen_w: f32, screen_h: f32) -> bool {
        self.life > 0.0
            && self.x >= -30.0
            && self.x <= screen_w + 30.0
            && self.y >= -30.0
            && self.y <= screen_h + 30.0
    }
}
