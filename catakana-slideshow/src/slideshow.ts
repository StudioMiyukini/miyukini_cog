// ============================================================
// CATAKANA 2026 — Contrôleur de slideshow
// ============================================================

import { SLIDES } from './slides.js';

export class Slideshow {
  private readonly slideEls: HTMLElement[] = [];
  private readonly dotEls: HTMLElement[] = [];
  private readonly progressFill: HTMLElement;
  private currentIdx = 0;
  private timer: ReturnType<typeof setTimeout> | null = null;
  private readonly total = SLIDES.length;

  constructor(
    private readonly wrapper: HTMLElement,
    private readonly dotsContainer: HTMLElement,
    uiLayer: HTMLElement,
  ) {
    this.progressFill = uiLayer.querySelector<HTMLElement>('#progress-fill')!;
    this.build();
    this.bindKeys();
    this.bindClick();
    this.showSlide(0);
  }

  // ---- Build DOM --------------------------------------------

  private build(): void {
    SLIDES.forEach((cfg, idx) => {
      // Slide element
      const el = document.createElement('div');
      el.className = ['slide', cfg.cssClass].filter(Boolean).join(' ');
      el.dataset['idx'] = String(idx);
      el.innerHTML = cfg.html;
      this.wrapper.appendChild(el);
      this.slideEls.push(el);

      // Dot
      const dot = document.createElement('div');
      dot.className = 'dot';
      dot.setAttribute('aria-label', `Slide ${idx + 1}`);
      dot.addEventListener('click', () => this.goTo(idx));
      this.dotsContainer.appendChild(dot);
      this.dotEls.push(dot);
    });
  }

  // ---- Navigation -------------------------------------------

  goTo(idx: number): void {
    if (idx === this.currentIdx) return;
    this.clearTimer();

    const prev = this.slideEls[this.currentIdx];
    prev.classList.add('exiting');
    prev.classList.remove('animating');

    setTimeout(() => {
      prev.classList.remove('active', 'exiting');
      this.currentIdx = idx;
      this.showSlide(idx);
    }, 420);
  }

  private showSlide(idx: number): void {
    const el = this.slideEls[idx];

    // Reset animation state (handles loop-back)
    el.classList.remove('animating');
    el.classList.add('active');
    void el.offsetWidth; // force reflow
    el.classList.add('animating');

    // Update dots
    this.dotEls.forEach((d, i) => d.classList.toggle('active', i === idx));

    // Start progress bar + timer
    const duration = SLIDES[idx].duration;
    this.startProgress(duration);
    this.timer = setTimeout(() => {
      this.goTo((this.currentIdx + 1) % this.total);
    }, duration);
  }

  // ---- Progress bar -----------------------------------------

  private startProgress(durationMs: number): void {
    this.progressFill.style.transition = 'none';
    this.progressFill.style.width = '0%';
    void this.progressFill.offsetWidth; // reflow
    this.progressFill.style.transition = `width ${durationMs}ms linear`;
    this.progressFill.style.width = '100%';
  }

  // ---- Timer ------------------------------------------------

  private clearTimer(): void {
    if (this.timer !== null) {
      clearTimeout(this.timer);
      this.timer = null;
    }
    // Stop progress bar immediately
    const w = getComputedStyle(this.progressFill).width;
    this.progressFill.style.transition = 'none';
    this.progressFill.style.width = w;
  }

  // ---- Input bindings ---------------------------------------

  private bindKeys(): void {
    document.addEventListener('keydown', (e: KeyboardEvent) => {
      if (e.key === 'ArrowRight' || e.key === ' ') {
        this.goTo((this.currentIdx + 1) % this.total);
      } else if (e.key === 'ArrowLeft') {
        this.goTo((this.currentIdx - 1 + this.total) % this.total);
      } else if (e.key === 'Escape') {
        // Toggle pause: re-trigger current
        this.clearTimer();
        this.showSlide(this.currentIdx);
      } else if (e.key === 't' || e.key === 'T') {
        // Basculer thème clair / sombre
        const app = document.getElementById('app');
        const indicator = document.getElementById('theme-indicator');
        if (app) {
          app.classList.toggle('light');
          const isLight = app.classList.contains('light');
          document.body.style.background = isLight ? '#faf9ff' : '';
          if (indicator) indicator.textContent = isLight ? '☀ Thème clair — T pour basculer' : '☾ Thème sombre — T pour basculer';
        }
      }
    });
  }

  private bindClick(): void {
    document.addEventListener('click', (e: MouseEvent) => {
      const x = e.clientX;
      const half = window.innerWidth / 2;
      if (x >= half) {
        this.goTo((this.currentIdx + 1) % this.total);
      } else {
        this.goTo((this.currentIdx - 1 + this.total) % this.total);
      }
    });
  }
}
