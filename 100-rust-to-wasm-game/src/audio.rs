//! Sound event emitter and queue for Web Audio synthesis.

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEvent {
    None = 0,
    LaserShoot = 1,
    PlasmaShoot = 2,
    SpreadShoot = 3,
    EmpBlast = 4,
    PlayerHit = 5,
    EnemyHit = 6,
    SmallExplosion = 7,
    BigExplosion = 8,
    PowerupPickup = 9,
    ShieldLow = 10,
    BossWarning = 11,
    BossDefeated = 12,
    GameOver = 13,
    WaveComplete = 14,
}

#[derive(Debug, Clone)]
pub struct AudioQueue {
    events: Vec<SoundEvent>,
}

impl Default for AudioQueue {
    fn default() -> Self {
        Self {
            events: Vec::with_capacity(32),
        }
    }
}

impl AudioQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trigger(&mut self, event: SoundEvent) {
        if self.events.len() < 32 {
            self.events.push(event);
        }
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn events(&self) -> &[SoundEvent] {
        &self.events
    }
}
