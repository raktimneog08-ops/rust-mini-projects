//! Cosmic Defender WASM - Rust Core Game Engine & WASM Exports.
//!
//! Provides a zero-overhead C-ABI / WASM interface for high performance 60 FPS
//! browser execution and canvas rendering.

#![allow(static_mut_refs)]

pub mod audio;
pub mod bullet;
pub mod collision;
pub mod enemy;
pub mod game;
pub mod particle;
pub mod player;

use game::Game;

// Global game instance container for direct WASM access
static mut GAME_INSTANCE: Option<Game> = None;

// Reusable render buffers to avoid allocations per frame
static mut STARS_BUFFER: Vec<f32> = Vec::new();
static mut ENEMIES_BUFFER: Vec<f32> = Vec::new();
static mut BULLETS_BUFFER: Vec<f32> = Vec::new();
static mut PARTICLES_BUFFER: Vec<f32> = Vec::new();
static mut POWERUPS_BUFFER: Vec<f32> = Vec::new();
static mut AUDIO_EVENTS_BUFFER: Vec<u8> = Vec::new();

#[no_mangle]
pub extern "C" fn init_game(width: f32, height: f32) {
    unsafe {
        GAME_INSTANCE = Some(Game::new(width, height));
        STARS_BUFFER = Vec::with_capacity(600);
        ENEMIES_BUFFER = Vec::with_capacity(1000);
        BULLETS_BUFFER = Vec::with_capacity(2000);
        PARTICLES_BUFFER = Vec::with_capacity(4000);
        POWERUPS_BUFFER = Vec::with_capacity(200);
        AUDIO_EVENTS_BUFFER = Vec::with_capacity(64);
    }
}

#[no_mangle]
pub extern "C" fn update_game(dt: f32, input_dx: f32, input_dy: f32, is_firing: i32) {
    unsafe {
        if let Some(ref mut game) = GAME_INSTANCE {
            game.update(dt, input_dx, input_dy, is_firing != 0);
        }
    }
}

#[no_mangle]
pub extern "C" fn trigger_emp() {
    unsafe {
        if let Some(ref mut game) = GAME_INSTANCE {
            game.trigger_emp();
        }
    }
}

#[no_mangle]
pub extern "C" fn restart_game() {
    unsafe {
        if let Some(ref mut game) = GAME_INSTANCE {
            game.restart();
        }
    }
}

#[no_mangle]
pub extern "C" fn get_game_state() -> i32 {
    unsafe {
        if let Some(ref game) = GAME_INSTANCE {
            game.state as i32
        } else {
            0
        }
    }
}

#[no_mangle]
pub extern "C" fn get_player_x() -> f32 {
    unsafe { GAME_INSTANCE.as_ref().map(|g| g.player.x).unwrap_or(0.0) }
}

#[no_mangle]
pub extern "C" fn get_player_y() -> f32 {
    unsafe { GAME_INSTANCE.as_ref().map(|g| g.player.y).unwrap_or(0.0) }
}

#[no_mangle]
pub extern "C" fn get_player_health() -> f32 {
    unsafe {
        GAME_INSTANCE
            .as_ref()
            .map(|g| g.player.health)
            .unwrap_or(0.0)
    }
}

#[no_mangle]
pub extern "C" fn get_player_shield() -> f32 {
    unsafe {
        GAME_INSTANCE
            .as_ref()
            .map(|g| g.player.shield)
            .unwrap_or(0.0)
    }
}

#[no_mangle]
pub extern "C" fn get_player_emp_charges() -> i32 {
    unsafe {
        GAME_INSTANCE
            .as_ref()
            .map(|g| g.player.emp_charges as i32)
            .unwrap_or(0)
    }
}

#[no_mangle]
pub extern "C" fn get_player_score() -> u32 {
    unsafe { GAME_INSTANCE.as_ref().map(|g| g.player.score).unwrap_or(0) }
}

#[no_mangle]
pub extern "C" fn get_high_score() -> u32 {
    unsafe { GAME_INSTANCE.as_ref().map(|g| g.high_score).unwrap_or(0) }
}

#[no_mangle]
pub extern "C" fn get_wave() -> u32 {
    unsafe { GAME_INSTANCE.as_ref().map(|g| g.wave).unwrap_or(1) }
}

#[no_mangle]
pub extern "C" fn get_combo() -> u32 {
    unsafe { GAME_INSTANCE.as_ref().map(|g| g.player.combo).unwrap_or(0) }
}

#[no_mangle]
pub extern "C" fn get_weapon_type() -> i32 {
    unsafe {
        GAME_INSTANCE
            .as_ref()
            .map(|g| g.player.weapon_type as i32)
            .unwrap_or(0)
    }
}

#[no_mangle]
pub extern "C" fn get_weapon_level() -> i32 {
    unsafe {
        GAME_INSTANCE
            .as_ref()
            .map(|g| g.player.weapon_level as i32)
            .unwrap_or(1)
    }
}

#[no_mangle]
pub extern "C" fn get_screen_shake() -> f32 {
    unsafe {
        GAME_INSTANCE
            .as_ref()
            .map(|g| g.screen_shake)
            .unwrap_or(0.0)
    }
}

#[no_mangle]
pub extern "C" fn get_player_invulnerable() -> f32 {
    unsafe {
        GAME_INSTANCE
            .as_ref()
            .map(|g| g.player.invulnerable_timer)
            .unwrap_or(0.0)
    }
}

// =========================================================================
// Fast Multi-Entity Data Buffers (Float32 Array Pointer + Length)
// =========================================================================

#[no_mangle]
pub extern "C" fn get_stars_buffer_ptr() -> *const f32 {
    unsafe {
        STARS_BUFFER.clear();
        if let Some(ref game) = GAME_INSTANCE {
            for star in &game.particles.stars {
                // Layout: [x, y, size, brightness]
                STARS_BUFFER.push(star.x);
                STARS_BUFFER.push(star.y);
                STARS_BUFFER.push(star.size);
                STARS_BUFFER.push(star.brightness);
            }
        }
        STARS_BUFFER.as_ptr()
    }
}

#[no_mangle]
pub extern "C" fn get_stars_count() -> usize {
    unsafe { STARS_BUFFER.len() / 4 }
}

#[no_mangle]
pub extern "C" fn get_enemies_buffer_ptr() -> *const f32 {
    unsafe {
        ENEMIES_BUFFER.clear();
        if let Some(ref game) = GAME_INSTANCE {
            for e in &game.enemies {
                // Layout: [x, y, radius, type, health, max_health, boss_phase]
                ENEMIES_BUFFER.push(e.x);
                ENEMIES_BUFFER.push(e.y);
                ENEMIES_BUFFER.push(e.radius);
                ENEMIES_BUFFER.push(e.enemy_type as u8 as f32);
                ENEMIES_BUFFER.push(e.health);
                ENEMIES_BUFFER.push(e.max_health);
                ENEMIES_BUFFER.push(e.boss_phase as f32);
            }
        }
        ENEMIES_BUFFER.as_ptr()
    }
}

#[no_mangle]
pub extern "C" fn get_enemies_count() -> usize {
    unsafe { ENEMIES_BUFFER.len() / 7 }
}

#[no_mangle]
pub extern "C" fn get_bullets_buffer_ptr() -> *const f32 {
    unsafe {
        BULLETS_BUFFER.clear();
        if let Some(ref game) = GAME_INSTANCE {
            for b in &game.bullets {
                // Layout: [x, y, radius, owner, bullet_type]
                BULLETS_BUFFER.push(b.x);
                BULLETS_BUFFER.push(b.y);
                BULLETS_BUFFER.push(b.radius);
                BULLETS_BUFFER.push(b.owner as u8 as f32);
                BULLETS_BUFFER.push(b.bullet_type as u8 as f32);
            }
        }
        BULLETS_BUFFER.as_ptr()
    }
}

#[no_mangle]
pub extern "C" fn get_bullets_count() -> usize {
    unsafe { BULLETS_BUFFER.len() / 5 }
}

#[no_mangle]
pub extern "C" fn get_particles_buffer_ptr() -> *const f32 {
    unsafe {
        PARTICLES_BUFFER.clear();
        if let Some(ref game) = GAME_INSTANCE {
            for p in &game.particles.particles {
                // Layout: [x, y, size, life/max_life, r, g, b, type]
                PARTICLES_BUFFER.push(p.x);
                PARTICLES_BUFFER.push(p.y);
                PARTICLES_BUFFER.push(p.size);
                PARTICLES_BUFFER.push(p.life / p.max_life.max(0.001));
                PARTICLES_BUFFER.push(p.r as f32);
                PARTICLES_BUFFER.push(p.g as f32);
                PARTICLES_BUFFER.push(p.b as f32);
                PARTICLES_BUFFER.push(p.particle_type as u8 as f32);
            }
        }
        PARTICLES_BUFFER.as_ptr()
    }
}

#[no_mangle]
pub extern "C" fn get_particles_count() -> usize {
    unsafe { PARTICLES_BUFFER.len() / 8 }
}

#[no_mangle]
pub extern "C" fn get_powerups_buffer_ptr() -> *const f32 {
    unsafe {
        POWERUPS_BUFFER.clear();
        if let Some(ref game) = GAME_INSTANCE {
            for p in &game.powerups {
                // Layout: [x, y, radius, type]
                POWERUPS_BUFFER.push(p.x);
                POWERUPS_BUFFER.push(p.y);
                POWERUPS_BUFFER.push(p.radius);
                POWERUPS_BUFFER.push(p.powerup_type as u8 as f32);
            }
        }
        POWERUPS_BUFFER.as_ptr()
    }
}

#[no_mangle]
pub extern "C" fn get_powerups_count() -> usize {
    unsafe { POWERUPS_BUFFER.len() / 4 }
}

#[no_mangle]
pub extern "C" fn get_audio_events_ptr() -> *const u8 {
    unsafe {
        AUDIO_EVENTS_BUFFER.clear();
        if let Some(ref mut game) = GAME_INSTANCE {
            for &ev in game.audio_queue.events() {
                AUDIO_EVENTS_BUFFER.push(ev as u8);
            }
            game.audio_queue.clear();
        }
        AUDIO_EVENTS_BUFFER.as_ptr()
    }
}

#[no_mangle]
pub extern "C" fn get_audio_events_count() -> usize {
    unsafe { AUDIO_EVENTS_BUFFER.len() }
}
