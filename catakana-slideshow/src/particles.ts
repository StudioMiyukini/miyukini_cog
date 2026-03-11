// ============================================================
// CATAKANA 2026 — Système de particules (canvas)
// ============================================================

interface Particle {
  x: number;
  y: number;
  vx: number;
  vy: number;
  radius: number;
  baseOpacity: number;
  opacity: number;
  opacityDir: number;
  opacitySpeed: number;
  color: string;
  isStar: boolean; // true = circle, false = 4-point sparkle
}

const COLORS = [
  '#ffffff', '#ffffff', '#ffffff', // majorité blanches
  '#ffd700', '#ffd700',            // or (fantasy)
  '#ff6b2b',                       // orange Catakana
  '#a855f7',                       // violet magie
  '#60a5fa',                       // bleu clair
];

export class ParticleSystem {
  private readonly canvas: HTMLCanvasElement;
  private readonly ctx: CanvasRenderingContext2D;
  private particles: Particle[] = [];
  private rafId = 0;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    const ctx = canvas.getContext('2d');
    if (!ctx) throw new Error('Canvas 2D context unavailable');
    this.ctx = ctx;
    this.handleResize = this.handleResize.bind(this);
    this.tick = this.tick.bind(this);
    window.addEventListener('resize', this.handleResize);
    this.handleResize();
    this.spawn();
    this.tick();
  }

  private handleResize(): void {
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }

  private spawn(): void {
    const area = this.canvas.width * this.canvas.height;
    const count = Math.min(Math.max(Math.floor(area / 11000), 55), 180);
    this.particles = Array.from({ length: count }, () => this.createParticle(true));
  }

  private createParticle(randomY = false): Particle {
    const isStar = Math.random() > 0.22;
    const baseOpacity = isStar
      ? Math.random() * 0.45 + 0.08
      : Math.random() * 0.6 + 0.15;
    return {
      x: Math.random() * this.canvas.width,
      y: randomY ? Math.random() * this.canvas.height : this.canvas.height + 8,
      vx: (Math.random() - 0.5) * 0.18,
      vy: -(Math.random() * 0.35 + 0.06),
      radius: isStar ? Math.random() * 1.4 + 0.3 : Math.random() * 2.2 + 0.8,
      baseOpacity,
      opacity: Math.random() * baseOpacity,
      opacityDir: 1,
      opacitySpeed: Math.random() * 0.007 + 0.002,
      color: COLORS[Math.floor(Math.random() * COLORS.length)],
      isStar,
    };
  }

  private tick(): void {
    const { ctx, canvas, particles } = this;
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    for (let i = 0; i < particles.length; i++) {
      const p = particles[i];

      // Twinkle
      p.opacity += p.opacityDir * p.opacitySpeed;
      if (p.opacity >= p.baseOpacity) p.opacityDir = -1;
      if (p.opacity <= 0) { p.opacityDir = 1; }

      // Move
      p.x += p.vx;
      p.y += p.vy;

      // Wrap / respawn
      if (p.y < -10) {
        particles[i] = this.createParticle(false);
        continue;
      }
      if (p.x < -10) p.x = canvas.width + 8;
      if (p.x > canvas.width + 10) p.x = -8;

      ctx.save();
      ctx.globalAlpha = Math.max(0, p.opacity);
      ctx.fillStyle = p.color;

      if (p.isStar) {
        ctx.beginPath();
        ctx.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
        ctx.fill();
      } else {
        // 4-point sparkle
        const s = p.radius * 2.8;
        ctx.beginPath();
        ctx.moveTo(p.x, p.y - s);
        ctx.lineTo(p.x + s * 0.28, p.y - s * 0.28);
        ctx.lineTo(p.x + s, p.y);
        ctx.lineTo(p.x + s * 0.28, p.y + s * 0.28);
        ctx.lineTo(p.x, p.y + s);
        ctx.lineTo(p.x - s * 0.28, p.y + s * 0.28);
        ctx.lineTo(p.x - s, p.y);
        ctx.lineTo(p.x - s * 0.28, p.y - s * 0.28);
        ctx.closePath();
        ctx.fill();
      }

      ctx.restore();
    }

    this.rafId = requestAnimationFrame(this.tick);
  }

  destroy(): void {
    cancelAnimationFrame(this.rafId);
    window.removeEventListener('resize', this.handleResize);
  }
}
