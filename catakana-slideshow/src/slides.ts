// ============================================================
// CATAKANA 2026 — Données et templates HTML de chaque slide
// ============================================================

export interface SlideConfig {
  id: string;
  cssClass: string;
  duration: number; // millisecondes avant avance auto
  html: string;
}

/** Raccourci : attribut style pour le délai d'animation */
function d(sec: number): string {
  return `style="--d:${sec}s"`;
}

// ---- Slide 1 : HERO ----------------------------------------

function heroSlide(): SlideConfig {
  return {
    id: 'hero',
    cssClass: 'slide-hero',
    duration: 8500,
    html: `
      <div class="slide-inner">
        <div class="a hero-edition-badge" ${d(0.1)}>✦ 3ème Édition · Association Catakana ✦</div>

        <h1 class="a hero-main-title" ${d(0.35)}>
          CATA<span class="kata">KANA</span>
        </h1>

        <div class="a hero-year-line" ${d(0.6)}>2 &thinsp; 0 &thinsp; 2 &thinsp; 6</div>

        <div class="a hero-divider" ${d(0.8)}></div>

        <p class="a hero-tagline" ${d(1.0)}>Le festival dont vous êtes les héros</p>

        <div class="a hero-badges" ${d(1.25)}>
          <div class="hero-badge">
            <span class="hero-badge-icon">📅</span>
            <span>2 &amp; 3 mai 2026</span>
          </div>
          <div class="hero-badge">
            <span class="hero-badge-icon">📍</span>
            <span>Château de Catala · Saint-Orens-de-Gameville</span>
          </div>
          <div class="hero-badge">
            <span class="hero-badge-icon">✨</span>
            <span>Entrée libre</span>
          </div>
        </div>
      </div>
    `,
  };
}

// ---- Slide 2 : THÉMATIQUE ----------------------------------

function themeSlide(): SlideConfig {
  return {
    id: 'theme',
    cssClass: 'slide-theme',
    duration: 8000,
    html: `
      <div class="slide-inner">
        <div class="a theme-edition-pill" ${d(0.1)}>Thématique 2026</div>

        <h2 class="a theme-main-title" ${d(0.35)}>
          La fantaisie<br>dans la pop culture
        </h2>

        <p class="a theme-sub" ${d(0.65)}>
          Plongez dans les univers qui ont forgé notre imaginaire collectif
        </p>

        <div class="a-fade theme-chips" ${d(0.9)}>
          <span class="theme-chip">⚔️ Fantasy médiévale</span>
          <span class="theme-chip">🐉 Dragons &amp; magie</span>
          <span class="theme-chip">🧙 Sorciers &amp; guildes</span>
          <span class="theme-chip">🗺️ Mondes ouverts</span>
          <span class="theme-chip">📖 Isekai &amp; aventure</span>
          <span class="theme-chip">🏰 Empires fictifs</span>
        </div>
      </div>
    `,
  };
}

// ---- Slide 3 : UNIVERS -------------------------------------

function universeSlide(): SlideConfig {
  return {
    id: 'universe',
    cssClass: '',
    duration: 9500,
    html: `
      <div class="slide-inner">
        <span class="a eyebrow" ${d(0.1)}>Les univers à l'honneur</span>
        <h2 class="a section-title" ${d(0.3)}>
          Une plongée dans <span class="accent-gold">la fantaisie</span>
        </h2>

        <div class="universe-grid">
          <div class="a universe-card" ${d(0.55)}>
            <span class="universe-icon">🎬</span>
            <h3 class="universe-card-title">Films &amp; Séries</h3>
            <ul class="universe-list">
              <li>Le Seigneur des Anneaux</li>
              <li>Game of Thrones</li>
              <li>Harry Potter</li>
            </ul>
          </div>

          <div class="a universe-card" ${d(0.75)}>
            <span class="universe-icon">⚔️</span>
            <h3 class="universe-card-title">Anime &amp; Manga</h3>
            <ul class="universe-list">
              <li>Dungeon Meshi</li>
              <li>Berserk</li>
              <li>Re:Zero</li>
              <li>Overlord</li>
              <li>Goblin Slayer</li>
            </ul>
          </div>

          <div class="a universe-card" ${d(0.95)}>
            <span class="universe-icon">🎮</span>
            <h3 class="universe-card-title">Jeux Vidéo</h3>
            <ul class="universe-list">
              <li>World of Warcraft</li>
              <li>Final Fantasy</li>
              <li>League of Legends</li>
              <li>Diablo</li>
            </ul>
          </div>
        </div>
      </div>
    `,
  };
}

// ---- Slide 4 : GUILDE D'AVENTURIER -------------------------

function guildSlide(): SlideConfig {
  return {
    id: 'guild',
    cssClass: 'slide-guild',
    duration: 9000,
    html: `
      <div class="slide-inner">
        <span class="a guild-icon" ${d(0.1)}>⚔️</span>
        <span class="a guild-label" ${d(0.3)}>Animation fil rouge</span>
        <h2 class="a guild-title" ${d(0.5)}>La Guilde d'Aventurier</h2>
        <p class="a guild-tagline" ${d(0.7)}>"Vous êtes les héros de ce festival"</p>

        <p class="a guild-desc" ${d(0.9)}>
          En participant aux animations, en découvrant les exposants
          et en relevant des défis tout au long du week-end,
          <strong style="color:var(--accent-gold)">gagnez des pièces d'or et des trésors</strong>
          pour remplir le coffre de la guilde !
        </p>

        <div class="a guild-steps" ${d(1.1)}>
          <div class="guild-step">
            <span class="guild-step-icon">🗺️</span>
            <span class="guild-step-verb">Acceptez</span>
            <span class="guild-step-noun">des Quêtes</span>
          </div>
          <div class="guild-step">
            <span class="guild-step-icon">⚡</span>
            <span class="guild-step-verb">Participez</span>
            <span class="guild-step-noun">aux Animations</span>
          </div>
          <div class="guild-step">
            <span class="guild-step-icon">🪙</span>
            <span class="guild-step-verb">Gagnez</span>
            <span class="guild-step-noun">des Pièces d'Or</span>
          </div>
          <div class="guild-step">
            <span class="guild-step-icon">💎</span>
            <span class="guild-step-verb">Débloquez</span>
            <span class="guild-step-noun">des Trésors</span>
          </div>
        </div>
      </div>
    `,
  };
}

// ---- Slide 5 : ACTIVITÉS -----------------------------------

function activitiesSlide(): SlideConfig {
  return {
    id: 'activities',
    cssClass: '',
    duration: 8500,
    html: `
      <div class="slide-inner">
        <span class="a eyebrow" ${d(0.1)}>Au programme du week-end</span>
        <h2 class="a section-title" ${d(0.3)}>
          Des activités pour <span class="accent-orange">tous</span>
        </h2>

        <div class="activities-grid">
          <div class="a activity-card" ${d(0.5)}>
            <span class="activity-icon">🎭</span>
            <div>
              <div class="activity-name">Concours Cosplay</div>
              <div class="activity-desc">Solo &amp; groupe, photobooth, vestiaires</div>
            </div>
          </div>

          <div class="a activity-card" ${d(0.65)}>
            <span class="activity-icon">🎲</span>
            <div>
              <div class="activity-name">Jeux de Rôle</div>
              <div class="activity-desc">Tournois, jeux de société, figurines</div>
            </div>
          </div>

          <div class="a activity-card" ${d(0.8)}>
            <span class="activity-icon">🎤</span>
            <div>
              <div class="activity-name">Karaoké &amp; Quiz</div>
              <div class="activity-desc">Blind test, quiz thématiques, scène libre</div>
            </div>
          </div>

          <div class="a activity-card" ${d(0.95)}>
            <span class="activity-icon">🎵</span>
            <div>
              <div class="activity-name">Concert</div>
              <div class="activity-desc">École de musique, prestations scéniques</div>
            </div>
          </div>

          <div class="a activity-card" ${d(1.1)}>
            <span class="activity-icon">🎨</span>
            <div>
              <div class="activity-name">Ateliers Créatifs</div>
              <div class="activity-desc">Dessin, accessoires, création numérique</div>
            </div>
          </div>

          <div class="a activity-card" ${d(1.25)}>
            <span class="activity-icon">🍔</span>
            <div>
              <div class="activity-name">Food Trucks</div>
              <div class="activity-desc">Restauration variée, espaces détente</div>
            </div>
          </div>
        </div>
      </div>
    `,
  };
}

// ---- Slide 6 : EXPOSANTS -----------------------------------

function exhibitorsSlide(): SlideConfig {
  return {
    id: 'exhibitors',
    cssClass: 'slide-exhibitors',
    duration: 7500,
    html: `
      <div class="slide-inner">
        <span class="a eyebrow" ${d(0.1)}>Artist Alley &amp; Stands</span>

        <div class="a-scale exhibitors-big-number" ${d(0.3)}>
          30<span style="color:var(--accent-orange)">+</span>
        </div>

        <p class="a exhibitors-subtitle" ${d(0.65)}>Exposants &amp; Créateurs</p>

        <div class="a exhibitors-cats" ${d(0.9)}>
          <span class="exhibitor-cat">
            <span class="exhibitor-cat-icon">✏️</span> Illustrateurs &amp; Auteurs
          </span>
          <span class="exhibitor-cat">
            <span class="exhibitor-cat-icon">🧵</span> Artisans &amp; Makers
          </span>
          <span class="exhibitor-cat">
            <span class="exhibitor-cat-icon">📚</span> Écrivains
          </span>
          <span class="exhibitor-cat">
            <span class="exhibitor-cat-icon">🎮</span> Créateurs indépendants
          </span>
          <span class="exhibitor-cat">
            <span class="exhibitor-cat-icon">🤝</span> Associations partenaires
          </span>
        </div>
      </div>
    `,
  };
}

// ---- Slide 7 : TIMELINE ------------------------------------

function timelineSlide(): SlideConfig {
  return {
    id: 'timeline',
    cssClass: '',
    duration: 9500,
    html: `
      <div class="slide-inner">
        <span class="a eyebrow" ${d(0.1)}>Notre histoire</span>
        <h2 class="a section-title" ${d(0.3)}>
          3 éditions, <span class="accent-orange">une passion</span>
        </h2>

        <div class="timeline-cards">
          <div class="a timeline-card" ${d(0.5)}>
            <div class="timeline-year">2024</div>
            <div class="timeline-edition-label">1ère Édition</div>
            <div class="timeline-theme-name">Le Manga</div>
            <ul class="timeline-facts">
              <li>~2 000 visiteurs</li>
              <li>30 exposants</li>
              <li>Lancement du concept</li>
              <li>27 &amp; 28 avril</li>
            </ul>
          </div>

          <div class="a timeline-card" ${d(0.72)}>
            <div class="timeline-year">2025</div>
            <div class="timeline-edition-label">2ème Édition</div>
            <div class="timeline-theme-name">40 ans de culture geek</div>
            <ul class="timeline-facts">
              <li>Objectif 1 200 visiteurs</li>
              <li>30+ exposants</li>
              <li>Magazine Catakana</li>
              <li>26 &amp; 27 avril</li>
            </ul>
          </div>

          <div class="a timeline-card is-current" ${d(0.94)}>
            <div class="timeline-year">2026</div>
            <div class="timeline-edition-label">3ème Édition ✦</div>
            <div class="timeline-theme-name">La fantaisie dans la pop culture</div>
            <ul class="timeline-facts">
              <li>Objectif 2 000 visiteurs</li>
              <li>30+ exposants</li>
              <li>Guilde d'aventurier</li>
              <li>2 &amp; 3 mai</li>
            </ul>
          </div>
        </div>
      </div>
    `,
  };
}

// ---- Slide 8 : VALEURS -------------------------------------

function valuesSlide(): SlideConfig {
  return {
    id: 'values',
    cssClass: '',
    duration: 8000,
    html: `
      <div class="slide-inner">
        <span class="a eyebrow" ${d(0.1)}>Ce qui nous guide</span>
        <h2 class="a section-title" ${d(0.3)}>
          Nos <span class="accent-gold">valeurs</span>
        </h2>

        <div class="values-grid">
          <div class="a value-card" ${d(0.5)}>
            <span class="value-icon">♿</span>
            <div class="value-name">Accessibilité</div>
            <div class="value-desc">Un événement familial, gratuit ou à faible coût, ouvert à tous les publics</div>
          </div>

          <div class="a value-card" ${d(0.7)}>
            <span class="value-icon">🎨</span>
            <div class="value-name">Créativité</div>
            <div class="value-desc">Un lieu d'expression et d'expérimentation artistique libre et bienveillant</div>
          </div>

          <div class="a value-card" ${d(0.9)}>
            <span class="value-icon">🌱</span>
            <div class="value-name">Écologie</div>
            <div class="value-desc">Réemploi et recyclage de matériaux pour la décoration et les stands</div>
          </div>

          <div class="a value-card" ${d(1.1)}>
            <span class="value-icon">📚</span>
            <div class="value-name">Transmission</div>
            <div class="value-desc">Collaboration active avec les écoles et structures culturelles locales</div>
          </div>
        </div>
      </div>
    `,
  };
}

// ---- Slide 9 : INFOS PRATIQUES / CTA -----------------------

function ctaSlide(): SlideConfig {
  return {
    id: 'cta',
    cssClass: 'slide-cta',
    duration: 9000,
    html: `
      <div class="slide-inner">
        <div class="a cta-super-label" ${d(0.1)}>✦ Rejoignez l'aventure ✦</div>

        <h2 class="a cta-main-title" ${d(0.35)}>
          Festival <span class="cat">Catakana</span> 2026
        </h2>

        <p class="a cta-main-tagline" ${d(0.6)}>
          Le festival dont vous êtes les héros
        </p>

        <div class="a cta-info-row" ${d(0.8)}>
          <div class="cta-info-card">
            <span class="cta-card-icon">📅</span>
            <span class="cta-card-label">Dates</span>
            <span class="cta-card-value">Samedi 2 &amp; Dimanche 3 mai 2026</span>
          </div>
          <div class="cta-info-card">
            <span class="cta-card-icon">📍</span>
            <span class="cta-card-label">Lieu</span>
            <span class="cta-card-value">Château de Catala<br>Saint-Orens-de-Gameville (31)</span>
          </div>
          <div class="cta-info-card">
            <span class="cta-card-icon">🕙</span>
            <span class="cta-card-label">Horaires</span>
            <span class="cta-card-value">Sam. 10h – 20h<br>Dim. 10h – 18h</span>
          </div>
        </div>

        <p class="a cta-website" ${d(1.1)}>catakana.fr</p>
      </div>
    `,
  };
}

// ---- Export -------------------------------------------------

export const SLIDES: SlideConfig[] = [
  heroSlide(),
  themeSlide(),
  universeSlide(),
  guildSlide(),
  activitiesSlide(),
  exhibitorsSlide(),
  timelineSlide(),
  valuesSlide(),
  ctaSlide(),
];
