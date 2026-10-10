//! Particle effects and starfield simulation for visual effects.

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticleType {
    Spark = 0,
    ExplosionSmoke = 1,
    LaserTrail = 2,
    ShieldRing = 3,
    EmpWave = 4,
    Debris = 5,
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
    pub max_life: f32,
    pub size: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub particle_type: ParticleType,
}

#[derive(Debug, Clone, Copy)]
pub struct Star {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub size: f32,
    pub brightness: f32,
}

#[inline]
pub fn next_xorshift(seed: &mut u32) -> f32 {
    let mut x = *seed;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *seed = x;
    (x as f32) / (u32::MAX as f32)
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub stars: Vec<Star>,
    pub rng_seed: u32,
}

impl ParticleSystem {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let mut seed = 123456789;
        let mut stars = Vec::with_capacity(120);

        // Initialize parallax starfield
        for _ in 0..120 {
            let x = next_xorshift(&mut seed) * screen_width;
            let y = next_xorshift(&mut seed) * screen_height;
            let layer = next_xorshift(&mut seed);
            let (speed, size, brightness) = if layer < 0.6 {
                (
                    20.0 + next_xorshift(&mut seed) * 20.0,
                    1.0,
                    0.4 + next_xorshift(&mut seed) * 0.3,
                )
            } else if layer < 0.9 {
                (
                    50.0 + next_xorshift(&mut seed) * 30.0,
                    1.5,
                    0.6 + next_xorshift(&mut seed) * 0.3,
                )
            } else {
                (
                    90.0 + next_xorshift(&mut seed) * 40.0,
                    2.2,
                    0.9 + next_xorshift(&mut seed) * 0.1,
                )
            };
            stars.push(Star {
                x,
                y,
                speed,
                size,
                brightness,
            });
        }

        Self {
            particles: Vec::with_capacity(512),
            stars,
            rng_seed: seed,
        }
    }

    pub fn update(&mut self, dt: f32, screen_width: f32, screen_height: f32) {
        // Update starfield
        let seed = &mut self.rng_seed;
        for star in &mut self.stars {
            star.y += star.speed * dt;
            if star.y > screen_height {
                star.y = 0.0;
                star.x = next_xorshift(seed) * screen_width;
            }
        }

        // Update particles
        let mut i = 0;
        while i < self.particles.len() {
            let p = &mut self.particles[i];
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vx *= 0.96; // drag
            p.vy *= 0.96;
            p.life -= dt;

            if p.life <= 0.0 {
                self.particles.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn emit_thruster_spark(&mut self, x: f32, y: f32) {
        if self.particles.len() >= 512 {
            return;
        }
        let r1 = next_xorshift(&mut self.rng_seed);
        let r2 = next_xorshift(&mut self.rng_seed);
        let r3 = next_xorshift(&mut self.rng_seed);
        let r4 = next_xorshift(&mut self.rng_seed);
        let r5 = next_xorshift(&mut self.rng_seed);

        let angle = std::f32::consts::PI * 0.5 + (r1 - 0.5) * 0.6;
        let speed = 60.0 + r2 * 80.0;
        let px = x + (r3 - 0.5) * 6.0;
        let life = 0.15 + r4 * 0.15;
        let size = 2.5 + r5 * 2.0;

        self.particles.push(Particle {
            x: px,
            y,
            vx: angle.cos() * speed,
            vy: angle.sin() * speed,
            life,
            max_life: 0.3,
            size,
            r: 0,
            g: 220 + (r1 * 35.0) as u8,
            b: 255,
            particle_type: ParticleType::Spark,
        });
    }

    pub fn emit_explosion(&mut self, x: f32, y: f32, count: usize, is_boss: bool) {
        let max_particles = if is_boss {
            count.min(100)
        } else {
            count.min(40)
        };
        for _ in 0..max_particles {
            if self.particles.len() >= 512 {
                break;
            }
            let r_angle = next_xorshift(&mut self.rng_seed);
            let r_speed = next_xorshift(&mut self.rng_seed);
            let r_life = next_xorshift(&mut self.rng_seed);
            let r_color = next_xorshift(&mut self.rng_seed);
            let r_size = next_xorshift(&mut self.rng_seed);

            let angle = r_angle * std::f32::consts::PI * 2.0;
            let speed = (30.0 + r_speed * 220.0) * if is_boss { 1.5 } else { 1.0 };
            let max_life = 0.4 + r_life * 0.5;

            // Palette: fiery neon cyan/orange/magenta
            let (r, g, b) = if r_color < 0.4 {
                (255, 120 + (r_color * 100.0) as u8, 30)
            } else if r_color < 0.7 {
                (255, 40, 140)
            } else {
                (0, 240, 255)
            };

            let size = (3.0 + r_size * 5.0) * if is_boss { 1.8 } else { 1.0 };

            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                life: max_life,
                max_life,
                size,
                r,
                g,
                b,
                particle_type: ParticleType::Debris,
            });
        }
    }

    pub fn emit_emp_wave(&mut self, x: f32, y: f32) {
        if self.particles.len() < 512 {
            self.particles.push(Particle {
                x,
                y,
                vx: 0.0,
                vy: 0.0,
                life: 0.6,
                max_life: 0.6,
                size: 10.0,
                r: 50,
                g: 220,
                b: 255,
                particle_type: ParticleType::EmpWave,
            });
        }
    }
}
