// Cosmic Defender WASM - Frontend Canvas Renderer and Input Controller

(function() {
  const canvas = document.getElementById('game-canvas');
  const ctx = canvas.getContext('2d');

  // HUD Elements
  const scoreDisplay = document.getElementById('score-display');
  const highScoreDisplay = document.getElementById('high-score-display');
  const waveDisplay = document.getElementById('wave-display');
  const shieldBar = document.getElementById('shield-bar');
  const healthBar = document.getElementById('health-bar');
  const weaponName = document.getElementById('weapon-name');
  const weaponLevel = document.getElementById('weapon-level');
  const comboBox = document.getElementById('combo-box');
  const comboDisplay = document.getElementById('combo-display');
  const finalScoreDisplay = document.getElementById('final-score-display');
  const newHighScoreBadge = document.getElementById('new-high-score-badge');

  // Screens & Buttons
  const startScreen = document.getElementById('start-screen');
  const gameOverScreen = document.getElementById('game-over-screen');
  const pauseScreen = document.getElementById('pause-screen');
  const btnStart = document.getElementById('btn-start');
  const btnRestart = document.getElementById('btn-restart');
  const btnResume = document.getElementById('btn-resume');
  const btnSound = document.getElementById('btn-sound');
  const btnCrt = document.getElementById('btn-crt');
  const btnPause = document.getElementById('btn-pause');
  const crtOverlay = document.getElementById('crt-overlay');

  // Game Input State
  const input = {
    up: false,
    down: false,
    left: false,
    right: false,
    fire: false,
    emp: false,
    mouseX: 0,
    mouseY: 0,
    isMouseDown: false,
  };

  let wasmExports = null;
  let wasmMemory = null;
  let isWasmLoaded = false;
  let isPaused = false;
  let lastTime = performance.now();
  let localHighScore = 0;
  try {
    localHighScore = parseInt(localStorage.getItem('cosmic_high_score') || '0', 10);
  } catch (e) {
    console.warn('localStorage is unavailable or restricted:', e);
  }
  highScoreDisplay.textContent = String(localHighScore).padStart(6, '0');

  // Weapon mapping
  const WEAPON_NAMES = ['PLASMA BLASTER', 'PLASMA CANNON', 'SPREAD SHOT', 'LASER BEAM'];

  // Initialize WASM Module
  async function loadWasm() {
    const candidatePaths = [
      './cosmic_defender_wasm.wasm',
      '../target/wasm32-unknown-unknown/release/cosmic_defender_wasm.wasm',
      'cosmic_defender_wasm.wasm'
    ];

    for (const wasmPath of candidatePaths) {
      try {
        const response = await fetch(wasmPath);
        if (response.ok) {
          const buffer = await response.arrayBuffer();
          const module = await WebAssembly.instantiate(buffer, {
            env: {}
          });
          wasmExports = module.instance.exports;
          wasmMemory = wasmExports.memory;
          isWasmLoaded = true;
          console.log(`Cosmic Defender Rust WASM Module loaded from ${wasmPath}!`);
          wasmExports.init_game(canvas.width, canvas.height);
          return;
        }
      } catch (e) {
        // Try next candidate
      }
    }
    console.warn('WASM module could not be loaded via standard candidates.');
    showWasmLoadWarning();
  }

  function showWasmLoadWarning() {
    const warning = document.createElement('div');
    warning.style.cssText = 'position:absolute; bottom:15px; left:50%; transform:translateX(-50%); background:rgba(255,0,85,0.9); color:#fff; padding:10px 18px; border-radius:8px; font-size:13px; font-weight:bold; z-index:100; text-align:center; box-shadow:0 0 15px rgba(255,0,85,0.6);';
    warning.innerHTML = '⚠️ WASM loading note: If running from <code>file://</code>, run <code>python serve.py</code> to play with local server.';
    document.getElementById('game-container').appendChild(warning);
  }

  // Mouse & Touch Drag Controls
  let touchStartX = null;
  let touchStartY = null;

  canvas.addEventListener('touchstart', (e) => {
    if (e.touches.length > 0) {
      touchStartX = e.touches[0].clientX;
      touchStartY = e.touches[0].clientY;
    }
    window.ArcadeAudio.init();
  }, { passive: true });

  canvas.addEventListener('touchmove', (e) => {
    if (e.touches.length > 0 && touchStartX !== null) {
      const dx = e.touches[0].clientX - touchStartX;
      const dy = e.touches[0].clientY - touchStartY;
      input.left = dx < -10;
      input.right = dx > 10;
      input.up = dy < -10;
      input.down = dy > 10;
    }
  }, { passive: true });

  canvas.addEventListener('touchend', () => {
    touchStartX = null;
    touchStartY = null;
    input.left = false;
    input.right = false;
    input.up = false;
    input.down = false;
  }, { passive: true });

  // Input Listeners
  window.addEventListener('keydown', (e) => {
    if (['ArrowUp', 'KeyW'].includes(e.code)) input.up = true;
    if (['ArrowDown', 'KeyS'].includes(e.code)) input.down = true;
    if (['ArrowLeft', 'KeyA'].includes(e.code)) input.left = true;
    if (['ArrowRight', 'KeyD'].includes(e.code)) input.right = true;
    if (e.code === 'Space') input.fire = true;
    if (['KeyE', 'ShiftLeft', 'ShiftRight'].includes(e.code)) {
      if (isWasmLoaded && wasmExports) wasmExports.trigger_emp();
    }
    if (e.code === 'KeyP') togglePause();
  });

  window.addEventListener('keyup', (e) => {
    if (['ArrowUp', 'KeyW'].includes(e.code)) input.up = false;
    if (['ArrowDown', 'KeyS'].includes(e.code)) input.down = false;
    if (['ArrowLeft', 'KeyA'].includes(e.code)) input.left = false;
    if (['ArrowRight', 'KeyD'].includes(e.code)) input.right = false;
    if (e.code === 'Space') input.fire = false;
  });

  canvas.addEventListener('mousedown', (e) => {
    input.isMouseDown = true;
    window.ArcadeAudio.init();
  });
  window.addEventListener('mouseup', () => input.isMouseDown = false);

  // Mobile Touch Controls
  document.getElementById('touch-fire').addEventListener('touchstart', (e) => {
    e.preventDefault();
    input.fire = true;
    window.ArcadeAudio.init();
  });
  document.getElementById('touch-fire').addEventListener('touchend', () => input.fire = false);

  document.getElementById('touch-emp').addEventListener('touchstart', (e) => {
    e.preventDefault();
    if (isWasmLoaded && wasmExports) wasmExports.trigger_emp();
  });

  // Buttons UI
  btnStart.addEventListener('click', () => {
    window.ArcadeAudio.init();
    window.ArcadeAudio.startBgm();
    startScreen.classList.add('hidden');
    if (isWasmLoaded && wasmExports) {
      wasmExports.restart_game();
    }
  });

  btnRestart.addEventListener('click', () => {
    gameOverScreen.classList.add('hidden');
    if (isWasmLoaded && wasmExports) {
      wasmExports.restart_game();
    }
    window.ArcadeAudio.startBgm();
  });

  btnResume.addEventListener('click', () => {
    togglePause();
  });

  btnPause.addEventListener('click', () => togglePause());

  btnSound.addEventListener('click', () => {
    window.ArcadeAudio.init();
    const muted = window.ArcadeAudio.toggleMute();
    btnSound.textContent = muted ? '🔇' : '🔊';
  });

  btnCrt.addEventListener('click', () => {
    crtOverlay.classList.toggle('disabled');
  });

  function togglePause() {
    isPaused = !isPaused;
    if (isPaused) {
      pauseScreen.classList.remove('hidden');
      window.ArcadeAudio.stopBgm();
    } else {
      pauseScreen.classList.add('hidden');
      window.ArcadeAudio.startBgm();
      lastTime = performance.now();
    }
  }

  function readFloat32Array(ptr, length) {
    if (!ptr || !wasmMemory) return [];
    return new Float32Array(wasmMemory.buffer, ptr, length);
  }

  function readUint8Array(ptr, length) {
    if (!ptr || !wasmMemory) return [];
    return new Uint8Array(wasmMemory.buffer, ptr, length);
  }

  // Render Loop
  function gameLoop(time) {
    requestAnimationFrame(gameLoop);
    const dt = Math.min((time - lastTime) / 1000, 0.1);
    lastTime = time;

    if (isPaused) return;

    // Process Gamepad
    pollGamepad();

    // Compute input direction
    let dx = 0;
    let dy = 0;
    if (input.left) dx -= 1;
    if (input.right) dx += 1;
    if (input.up) dy -= 1;
    if (input.down) dy += 1;
    const len = Math.hypot(dx, dy);
    if (len > 0) {
      dx /= len;
      dy /= len;
    }
    const isFiring = input.fire || input.isMouseDown;

    if (isWasmLoaded && wasmExports) {
      // Step WASM simulation
      wasmExports.update_game(dt, dx, dy, isFiring ? 1 : 0);

      // Process sound events triggered by Rust
      const audioPtr = wasmExports.get_audio_events_ptr();
      const audioCount = wasmExports.get_audio_events_count();
      const audioEvents = readUint8Array(audioPtr, audioCount);
      for (let i = 0; i < audioEvents.length; i++) {
        window.ArcadeAudio.playEvent(audioEvents[i]);
      }

      // Read Game State
      const state = wasmExports.get_game_state();
      const score = wasmExports.get_player_score();
      const wave = wasmExports.get_wave();
      const health = wasmExports.get_player_health();
      const shield = wasmExports.get_player_shield();
      const empCharges = wasmExports.get_player_emp_charges();
      const combo = wasmExports.get_combo();
      const wType = wasmExports.get_weapon_type();
      const wLevel = wasmExports.get_weapon_level();
      const screenShake = wasmExports.get_screen_shake();

      // Update HUD
      scoreDisplay.textContent = String(score).padStart(6, '0');
      waveDisplay.textContent = `WAVE ${wave}`;
      shieldBar.style.width = `${Math.max(0, shield)}%`;
      healthBar.style.width = `${Math.max(0, health)}%`;
      weaponName.textContent = WEAPON_NAMES[wType] || 'PLASMA BLASTER';
      weaponLevel.textContent = `POWER LVL ${wLevel}`;

      if (score > localHighScore) {
        localHighScore = score;
        try {
          localStorage.setItem('cosmic_high_score', localHighScore);
        } catch (e) {
          // ignore restricted storage
        }
        highScoreDisplay.textContent = String(localHighScore).padStart(6, '0');
      }

      if (combo > 1) {
        comboBox.style.opacity = '1';
        comboDisplay.textContent = `x${combo}`;
      } else {
        comboBox.style.opacity = '0';
      }

      // Update EMP orbs
      for (let i = 1; i <= 5; i++) {
        const orb = document.getElementById(`emp-orb-${i}`);
        if (orb) {
          if (i <= empCharges) orb.classList.add('active');
          else orb.classList.remove('active');
        }
      }

      if (state === 3) { // GameOver
        if (gameOverScreen.classList.contains('hidden')) {
          gameOverScreen.classList.remove('hidden');
          finalScoreDisplay.textContent = String(score).padStart(6, '0');
          if (score >= localHighScore && score > 0) {
            newHighScoreBadge.style.display = 'block';
          }
          window.ArcadeAudio.stopBgm();
        }
      }

      // Render Canvas
      ctx.save();
      if (screenShake > 0) {
        const shakeX = (Math.random() - 0.5) * screenShake;
        const shakeY = (Math.random() - 0.5) * screenShake;
        ctx.translate(shakeX, shakeY);
      }

      ctx.fillStyle = '#050714';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // 1. Draw Starfield
      const starsPtr = wasmExports.get_stars_buffer_ptr();
      const starsCount = wasmExports.get_stars_count();
      const starsData = readFloat32Array(starsPtr, starsCount * 4);
      for (let i = 0; i < starsCount; i++) {
        const sx = starsData[i * 4];
        const sy = starsData[i * 4 + 1];
        const size = starsData[i * 4 + 2];
        const bright = starsData[i * 4 + 3];
        ctx.fillStyle = `rgba(255, 255, 255, ${bright})`;
        ctx.fillRect(sx, sy, size, size);
      }

      // 2. Draw Particles
      const partPtr = wasmExports.get_particles_buffer_ptr();
      const partCount = wasmExports.get_particles_count();
      const partData = readFloat32Array(partPtr, partCount * 8);
      for (let i = 0; i < partCount; i++) {
        const px = partData[i * 8];
        const py = partData[i * 8 + 1];
        const size = partData[i * 8 + 2];
        const lifeRatio = partData[i * 8 + 3];
        const r = partData[i * 8 + 4];
        const g = partData[i * 8 + 5];
        const b = partData[i * 8 + 6];
        const pType = partData[i * 8 + 7];

        if (pType === 4) { // EMP Wave
          ctx.beginPath();
          ctx.arc(px, py, (1.0 - lifeRatio) * 600, 0, Math.PI * 2);
          ctx.strokeStyle = `rgba(0, 240, 255, ${lifeRatio})`;
          ctx.lineWidth = 6;
          ctx.stroke();
        } else {
          ctx.fillStyle = `rgba(${r}, ${g}, ${b}, ${lifeRatio})`;
          ctx.beginPath();
          ctx.arc(px, py, size * lifeRatio, 0, Math.PI * 2);
          ctx.fill();
        }
      }

      // 3. Draw Powerups
      const powPtr = wasmExports.get_powerups_buffer_ptr();
      const powCount = wasmExports.get_powerups_count();
      const powData = readFloat32Array(powPtr, powCount * 4);
      for (let i = 0; i < powCount; i++) {
        const px = powData[i * 4];
        const py = powData[i * 4 + 1];
        const pr = powData[i * 4 + 2];
        const pType = powData[i * 4 + 3];
        drawPowerup(px, py, pr, pType);
      }

      // 4. Draw Bullets
      const bullPtr = wasmExports.get_bullets_buffer_ptr();
      const bullCount = wasmExports.get_bullets_count();
      const bullData = readFloat32Array(bullPtr, bullCount * 5);
      for (let i = 0; i < bullCount; i++) {
        const bx = bullData[i * 5];
        const by = bullData[i * 5 + 1];
        const br = bullData[i * 5 + 2];
        const bOwner = bullData[i * 5 + 3];
        const bType = bullData[i * 5 + 4];
        drawBullet(bx, by, br, bOwner, bType);
      }

      // 5. Draw Enemies
      const enemyPtr = wasmExports.get_enemies_buffer_ptr();
      const enemyCount = wasmExports.get_enemies_count();
      const enemyData = readFloat32Array(enemyPtr, enemyCount * 7);
      for (let i = 0; i < enemyCount; i++) {
        const ex = enemyData[i * 7];
        const ey = enemyData[i * 7 + 1];
        const er = enemyData[i * 7 + 2];
        const eType = enemyData[i * 7 + 3];
        const hp = enemyData[i * 7 + 4];
        const maxHp = enemyData[i * 7 + 5];
        const bossPhase = enemyData[i * 7 + 6];
        drawEnemy(ex, ey, er, eType, hp, maxHp, bossPhase);
      }

      // 6. Draw Player
      if (health > 0) {
        const px = wasmExports.get_player_x();
        const py = wasmExports.get_player_y();
        const invuln = wasmExports.get_player_invulnerable();
        drawPlayer(px, py, invuln, shield > 0);
      }

      ctx.restore();
    }
  }

  // Graphics Helpers
  function drawPlayer(x, y, invuln, hasShield) {
    if (invuln > 0 && Math.floor(Date.now() / 80) % 2 === 0) {
      return; // Flash effect
    }

    ctx.save();
    ctx.translate(x, y);

    // Ship Hull
    ctx.fillStyle = '#00f0ff';
    ctx.beginPath();
    ctx.moveTo(0, -22);
    ctx.lineTo(16, 16);
    ctx.lineTo(6, 12);
    ctx.lineTo(0, 16);
    ctx.lineTo(-6, 12);
    ctx.lineTo(-16, 16);
    ctx.closePath();
    ctx.fill();

    // Cockpit Canopy
    ctx.fillStyle = '#ffffff';
    ctx.beginPath();
    ctx.moveTo(0, -12);
    ctx.lineTo(4, 2);
    ctx.lineTo(-4, 2);
    ctx.closePath();
    ctx.fill();

    // Wings Accents
    ctx.strokeStyle = '#ff007f';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(-16, 14);
    ctx.lineTo(-6, 10);
    ctx.moveTo(16, 14);
    ctx.lineTo(6, 10);
    ctx.stroke();

    // Shield Aura
    if (hasShield) {
      ctx.beginPath();
      ctx.arc(0, 0, 24, 0, Math.PI * 2);
      ctx.strokeStyle = 'rgba(0, 240, 255, 0.4)';
      ctx.lineWidth = 2;
      ctx.stroke();
    }

    ctx.restore();
  }

  function drawEnemy(x, y, radius, type, hp, maxHp, bossPhase) {
    ctx.save();
    ctx.translate(x, y);

    if (type === 4) { // Boss
      ctx.fillStyle = '#ff0055';
      ctx.beginPath();
      ctx.moveTo(0, 45);
      ctx.lineTo(55, -20);
      ctx.lineTo(35, -45);
      ctx.lineTo(-35, -45);
      ctx.lineTo(-55, -20);
      ctx.closePath();
      ctx.fill();

      // Boss core
      ctx.fillStyle = bossPhase === 3 ? '#ff0000' : '#ffee00';
      ctx.beginPath();
      ctx.arc(0, 0, 18, 0, Math.PI * 2);
      ctx.fill();

      // Boss HP Bar above
      ctx.fillStyle = 'rgba(0, 0, 0, 0.6)';
      ctx.fillRect(-60, -60, 120, 8);
      ctx.fillStyle = '#ff0055';
      ctx.fillRect(-59, -59, 118 * (hp / maxHp), 6);
    } else if (type === 0) { // Scout
      ctx.fillStyle = '#ff007f';
      ctx.beginPath();
      ctx.moveTo(0, 14);
      ctx.lineTo(12, -12);
      ctx.lineTo(-12, -12);
      ctx.closePath();
      ctx.fill();
    } else if (type === 1) { // Corvette
      ctx.fillStyle = '#ff9900';
      ctx.beginPath();
      ctx.moveTo(0, 18);
      ctx.lineTo(20, -10);
      ctx.lineTo(10, -18);
      ctx.lineTo(-10, -18);
      ctx.lineTo(-20, -10);
      ctx.closePath();
      ctx.fill();
    } else if (type === 2) { // Kamikaze
      ctx.fillStyle = '#ff3300';
      ctx.beginPath();
      ctx.moveTo(0, 16);
      ctx.lineTo(10, -12);
      ctx.lineTo(0, -6);
      ctx.lineTo(-10, -12);
      ctx.closePath();
      ctx.fill();
    } else { // Dreadnought
      ctx.fillStyle = '#aa00ff';
      ctx.beginPath();
      ctx.moveTo(0, 26);
      ctx.lineTo(30, -15);
      ctx.lineTo(20, -26);
      ctx.lineTo(-20, -26);
      ctx.lineTo(-30, -15);
      ctx.closePath();
      ctx.fill();
    }

    ctx.restore();
  }

  function drawBullet(x, y, radius, owner, type) {
    ctx.save();
    if (owner === 0) { // Player Bullet
      if (type === 1) { // Plasma
        ctx.fillStyle = '#00ff88';
        ctx.shadowColor = '#00ff88';
        ctx.shadowBlur = 10;
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, Math.PI * 2);
        ctx.fill();
      } else {
        ctx.fillStyle = '#00f0ff';
        ctx.fillRect(x - radius * 0.5, y - radius * 2, radius, radius * 4);
      }
    } else { // Enemy Bullet
      if (type === 5) { // Boss Ring
        ctx.fillStyle = '#ff0055';
        ctx.shadowColor = '#ff0055';
        ctx.shadowBlur = 8;
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, Math.PI * 2);
        ctx.fill();
      } else {
        ctx.fillStyle = '#ff9900';
        ctx.beginPath();
        ctx.arc(x, y, radius, 0, Math.PI * 2);
        ctx.fill();
      }
    }
    ctx.restore();
  }

  function drawPowerup(x, y, radius, type) {
    ctx.save();
    ctx.translate(x, y);
    const colors = ['#0088ff', '#ffee00', '#00f0ff', '#ff007f'];
    const icons = ['🛡️', '⚡', '💣', '⭐'];
    ctx.beginPath();
    ctx.arc(0, 0, radius, 0, Math.PI * 2);
    ctx.fillStyle = colors[type] || '#fff';
    ctx.fill();
    ctx.font = '12px sans-serif';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(icons[type] || '★', 0, 1);
    ctx.restore();
  }

  function pollGamepad() {
    const gamepads = navigator.getGamepads ? navigator.getGamepads() : [];
    if (!gamepads || !gamepads[0]) return;
    const gp = gamepads[0];
    if (Math.abs(gp.axes[0]) > 0.2) {
      input.left = gp.axes[0] < -0.2;
      input.right = gp.axes[0] > 0.2;
    }
    if (Math.abs(gp.axes[1]) > 0.2) {
      input.up = gp.axes[1] < -0.2;
      input.down = gp.axes[1] > 0.2;
    }
    input.fire = gp.buttons[0]?.pressed || gp.buttons[7]?.pressed || false;
    if (gp.buttons[1]?.pressed || gp.buttons[6]?.pressed) {
      if (isWasmLoaded && wasmExports) wasmExports.trigger_emp();
    }
  }

  // Boot
  loadWasm().then(() => {
    requestAnimationFrame(gameLoop);
  });
})();
