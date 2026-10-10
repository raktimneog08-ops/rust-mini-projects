// Web Audio API Retro 8-bit Procedural Sound Synthesizer for Cosmic Defender WASM

class ArcadeAudioEngine {
  constructor() {
    this.ctx = null;
    this.muted = false;
    this.volume = 0.6;
    this.bgmPlaying = false;
    this.bgmStep = 0;
    this.bgmTimer = null;
  }

  init() {
    if (!this.ctx) {
      const AudioContext = window.AudioContext || window.webkitAudioContext;
      if (AudioContext) {
        this.ctx = new AudioContext();
      }
    }
    if (this.ctx && this.ctx.state === 'suspended') {
      this.ctx.resume();
    }
  }

  toggleMute() {
    this.muted = !this.muted;
    return this.muted;
  }

  playEvent(eventId) {
    if (this.muted || !this.ctx) return;

    switch (eventId) {
      case 1: // LaserShoot
        this.playLaser();
        break;
      case 2: // PlasmaShoot
        this.playPlasma();
        break;
      case 3: // SpreadShoot
        this.playSpread();
        break;
      case 4: // EmpBlast
        this.playEmp();
        break;
      case 5: // PlayerHit
        this.playHit(true);
        break;
      case 6: // EnemyHit
        this.playHit(false);
        break;
      case 7: // SmallExplosion
        this.playExplosion(0.2, 300);
        break;
      case 8: // BigExplosion
        this.playExplosion(0.6, 120);
        break;
      case 9: // PowerupPickup
        this.playPowerup();
        break;
      case 10: // ShieldLow
        this.playTone(220, 0.1, 'sawtooth');
        break;
      case 11: // BossWarning
        this.playBossAlarm();
        break;
      case 12: // BossDefeated
        this.playBossDefeated();
        break;
      case 13: // GameOver
        this.playGameOver();
        break;
      case 14: // WaveComplete
        this.playWaveComplete();
        break;
    }
  }

  playLaser() {
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sawtooth';
    osc.frequency.setValueAtTime(880, t);
    osc.frequency.exponentialRampToValueAtTime(110, t + 0.12);
    gain.gain.setValueAtTime(0.18 * this.volume, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.12);
    osc.connect(gain);
    gain.connect(this.ctx.destination);
    osc.start(t);
    osc.stop(t + 0.12);
  }

  playPlasma() {
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(320, t);
    osc.frequency.exponentialRampToValueAtTime(60, t + 0.22);
    gain.gain.setValueAtTime(0.35 * this.volume, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.22);
    osc.connect(gain);
    gain.connect(this.ctx.destination);
    osc.start(t);
    osc.stop(t + 0.22);
  }

  playSpread() {
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'square';
    osc.frequency.setValueAtTime(600, t);
    osc.frequency.exponentialRampToValueAtTime(200, t + 0.08);
    gain.gain.setValueAtTime(0.12 * this.volume, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.08);
    osc.connect(gain);
    gain.connect(this.ctx.destination);
    osc.start(t);
    osc.stop(t + 0.08);
  }

  playEmp() {
    const t = this.ctx.currentTime;
    // Sweep tone
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(1200, t);
    osc.frequency.exponentialRampToValueAtTime(40, t + 0.6);
    gain.gain.setValueAtTime(0.4 * this.volume, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.6);
    osc.connect(gain);
    gain.connect(this.ctx.destination);
    osc.start(t);
    osc.stop(t + 0.6);
    this.playExplosion(0.7, 80);
  }

  playHit(isPlayer) {
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = isPlayer ? 'sawtooth' : 'triangle';
    osc.frequency.setValueAtTime(isPlayer ? 180 : 450, t);
    osc.frequency.exponentialRampToValueAtTime(40, t + 0.08);
    gain.gain.setValueAtTime((isPlayer ? 0.25 : 0.1) * this.volume, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + 0.08);
    osc.connect(gain);
    gain.connect(this.ctx.destination);
    osc.start(t);
    osc.stop(t + 0.08);
  }

  playExplosion(duration, filterFreq) {
    const t = this.ctx.currentTime;
    const bufferSize = this.ctx.sampleRate * duration;
    const buffer = this.ctx.createBuffer(1, bufferSize, this.ctx.sampleRate);
    const output = buffer.getChannelData(0);
    for (let i = 0; i < bufferSize; i++) {
      output[i] = Math.random() * 2 - 1;
    }

    const whiteNoise = this.ctx.createBufferSource();
    whiteNoise.buffer = buffer;

    const filter = this.ctx.createBiquadFilter();
    filter.type = 'lowpass';
    filter.frequency.setValueAtTime(filterFreq, t);
    filter.frequency.exponentialRampToValueAtTime(30, t + duration);

    const gain = this.ctx.createGain();
    gain.gain.setValueAtTime(0.35 * this.volume, t);
    gain.gain.exponentialRampToValueAtTime(0.01, t + duration);

    whiteNoise.connect(filter);
    filter.connect(gain);
    gain.connect(this.ctx.destination);

    whiteNoise.start(t);
  }

  playPowerup() {
    const notes = [440, 554.37, 659.25, 880];
    notes.forEach((freq, idx) => {
      setTimeout(() => {
        this.playTone(freq, 0.08, 'square', 0.15);
      }, idx * 60);
    });
  }

  playBossAlarm() {
    for (let i = 0; i < 3; i++) {
      setTimeout(() => {
        this.playTone(600, 0.15, 'sawtooth', 0.25);
      }, i * 200);
    }
  }

  playBossDefeated() {
    for (let i = 0; i < 6; i++) {
      setTimeout(() => {
        this.playExplosion(0.5, 200 - i * 20);
      }, i * 150);
    }
  }

  playGameOver() {
    const notes = [392.0, 349.23, 311.13, 261.63];
    notes.forEach((freq, idx) => {
      setTimeout(() => {
        this.playTone(freq, 0.25, 'triangle', 0.25);
      }, idx * 200);
    });
  }

  playWaveComplete() {
    const notes = [523.25, 659.25, 783.99, 1046.5];
    notes.forEach((freq, idx) => {
      setTimeout(() => {
        this.playTone(freq, 0.12, 'square', 0.2);
      }, idx * 80);
    });
  }

  playTone(freq, duration, type = 'sine', vol = 0.2) {
    if (this.muted || !this.ctx) return;
    const t = this.ctx.currentTime;
    const osc = this.ctx.createOscillator();
    const gain = this.ctx.createGain();
    osc.type = type;
    osc.frequency.setValueAtTime(freq, t);
    gain.gain.setValueAtTime(vol * this.volume, t);
    gain.gain.exponentialRampToValueAtTime(0.001, t + duration);
    osc.connect(gain);
    gain.connect(this.ctx.destination);
    osc.start(t);
    osc.stop(t + duration);
  }

  startBgm() {
    if (this.bgmPlaying || !this.ctx) return;
    this.bgmPlaying = true;
    const bassline = [110, 110, 130.81, 146.83, 110, 110, 164.81, 146.83];
    this.bgmTimer = setInterval(() => {
      if (!this.muted && this.bgmPlaying) {
        const note = bassline[this.bgmStep % bassline.length];
        this.playTone(note, 0.1, 'triangle', 0.08);
        this.bgmStep++;
      }
    }, 180);
  }

  stopBgm() {
    this.bgmPlaying = false;
    if (this.bgmTimer) {
      clearInterval(this.bgmTimer);
      this.bgmTimer = null;
    }
  }
}

window.ArcadeAudio = new ArcadeAudioEngine();
