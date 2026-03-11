// ============================================================
// CATAKANA 2026 — Point d'entrée
// ============================================================

import './style.css';
import { ParticleSystem } from './particles.js';
import { Slideshow } from './slideshow.js';

function init(): void {
  const canvas = document.getElementById('particles-canvas') as HTMLCanvasElement | null;
  const wrapper = document.getElementById('slides-wrapper') as HTMLElement | null;
  const dotsContainer = document.getElementById('slide-dots') as HTMLElement | null;
  const progressBar = document.getElementById('ui-layer') as HTMLElement | null;
  const app = document.getElementById('app');

  if (!canvas || !wrapper || !dotsContainer || !progressBar || !app) {
    console.error('[Catakana] DOM elements missing — check index.html');
    return;
  }

  // Ajouter l'indicateur de thème dans le DOM
  const indicator = document.createElement('div');
  indicator.id = 'theme-indicator';
  indicator.textContent = '☾ Thème sombre — T pour basculer';
  document.body.appendChild(indicator);

  // Appliquer le thème clair si ?light dans l'URL
  if (window.location.search.includes('light') || window.location.hash.includes('light')) {
    app.classList.add('light');
    document.body.style.background = '#faf9ff';
    indicator.textContent = '☀ Thème clair — T pour basculer';
  }

  new ParticleSystem(canvas);
  new Slideshow(wrapper, dotsContainer, progressBar);
}

// Les scripts module sont différés → DOM toujours prêt à l'exécution
// Fallback si DOMContentLoaded déjà passé (inline module dans viteSingleFile)
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
