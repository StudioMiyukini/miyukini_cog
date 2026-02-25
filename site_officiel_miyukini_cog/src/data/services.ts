/**
 * Services (Registre d'Opérateurs) — aligné sur le catalogue Miyukini Central.
 * Enrichi avec icônes, couleurs et type pour l'affichage inventaire.
 */
export interface ServiceMeta {
  id: string;
  name: string;
  shortDesc: string;
  category: 'fundamental' | 'jay' | 'tool' | 'game';
  icon: string;
  color: string;
  /** Type de service (1: interne COG, 2: surface web, 3: inter-COG) */
  type: 1 | 2 | 3;
  status: 'production' | 'beta' | 'development' | 'planned';
}

export const SERVICES: ServiceMeta[] = [
  {
    id: 'miyukini-central',
    name: 'Miyukini Central',
    shortDesc: 'Hub de gestion des Services. Vitrine du Registre d\'Opérateurs.',
    category: 'fundamental',
    icon: '◇',
    color: '#2dd4bf',
    type: 1,
    status: 'production',
  },
  {
    id: 'miyukini-web-portal',
    name: 'Web Portal',
    shortDesc: 'Point d\'entrée web pour les utilisateurs externes. Façades publiques gouvernées.',
    category: 'fundamental',
    icon: '🌍',
    color: '#8b5cf6',
    type: 2,
    status: 'production',
  },
  {
    id: 'jayxpose',
    name: 'JayXpose',
    shortDesc: 'Profil exposant, site vitrine, catalogue.',
    category: 'jay',
    icon: '🎯',
    color: '#f59e0b',
    type: 2,
    status: 'production',
  },
  {
    id: 'jayfestival',
    name: 'JayFestival',
    shortDesc: 'Éditions, exposants, événements.',
    category: 'jay',
    icon: '🎪',
    color: '#ec4899',
    type: 2,
    status: 'production',
  },
  {
    id: 'jaykonta',
    name: 'JayKonta',
    shortDesc: 'Budget, facturation (JayBudget / JayKonta).',
    category: 'jay',
    icon: '💼',
    color: '#10b981',
    type: 1,
    status: 'production',
  },
  {
    id: 'jaykoa',
    name: 'JayKoa',
    shortDesc: 'Agenda unifié, intégrateur des dates.',
    category: 'jay',
    icon: '📅',
    color: '#6366f1',
    type: 1,
    status: 'production',
  },
  {
    id: 'jayrdv',
    name: 'JayRDV',
    shortDesc: 'RDV, créneaux, exceptions.',
    category: 'jay',
    icon: '📞',
    color: '#0ea5e9',
    type: 2,
    status: 'beta',
  },
  {
    id: 'jayshop',
    name: 'JayShop',
    shortDesc: 'Boutique en ligne et point de vente.',
    category: 'jay',
    icon: '🛍️',
    color: '#f43f5e',
    type: 2,
    status: 'development',
  },
  {
    id: 'jayfaim',
    name: 'JayFaim',
    shortDesc: 'Restauration, commandes, food trucks.',
    category: 'jay',
    icon: '🍔',
    color: '#eab308',
    type: 2,
    status: 'planned',
  },
  {
    id: 'jaymanga',
    name: 'JayManga',
    shortDesc: 'Lecture et vente de manga en ligne.',
    category: 'jay',
    icon: '📚',
    color: '#e879f9',
    type: 2,
    status: 'development',
  },
  {
    id: 'calculator',
    name: 'Calculatrice',
    shortDesc: 'Outil de calcul intégré.',
    category: 'tool',
    icon: '🔢',
    color: '#64748b',
    type: 1,
    status: 'production',
  },
  {
    id: 'notes',
    name: 'Notes',
    shortDesc: 'Notes rapides.',
    category: 'tool',
    icon: '📝',
    color: '#a3a3a3',
    type: 1,
    status: 'production',
  },
  {
    id: 'text-editor',
    name: 'Éditeur',
    shortDesc: 'Traitement de texte.',
    category: 'tool',
    icon: '✏️',
    color: '#78716c',
    type: 1,
    status: 'production',
  },
  {
    id: 'miyuclicker',
    name: 'Lord of the Click',
    shortDesc: 'Idle / Clicker + carte stratégique.',
    category: 'game',
    icon: '👑',
    color: '#fbbf24',
    type: 1,
    status: 'development',
  },
  {
    id: 'lord-of-the-castle',
    name: 'Lord of the Castle',
    shortDesc: 'Survivor + Tower Defense.',
    category: 'game',
    icon: '🏰',
    color: '#7c3aed',
    type: 1,
    status: 'development',
  },
  {
    id: 'miyulifegame',
    name: 'LifeGame',
    shortDesc: 'Simulation de vie — style WorldBox.',
    category: 'game',
    icon: '🌎',
    color: '#22c55e',
    type: 1,
    status: 'development',
  },
];

export const CATEGORY_LABELS: Record<ServiceMeta['category'], string> = {
  fundamental: 'Fondamentaux',
  jay: 'Services Jay',
  tool: 'Outils',
  game: 'Jeux',
};

/** Couleur de rareté par catégorie (style Genshin) */
export const CATEGORY_COLORS: Record<ServiceMeta['category'], { border: string; bg: string; glow: string }> = {
  fundamental: { border: '#fbbf24', bg: 'rgba(251, 191, 36, 0.15)', glow: 'rgba(251, 191, 36, 0.4)' },
  jay: { border: '#a855f7', bg: 'rgba(168, 85, 247, 0.15)', glow: 'rgba(168, 85, 247, 0.4)' },
  tool: { border: '#3b82f6', bg: 'rgba(59, 130, 246, 0.15)', glow: 'rgba(59, 130, 246, 0.4)' },
  game: { border: '#ec4899', bg: 'rgba(236, 72, 153, 0.15)', glow: 'rgba(236, 72, 153, 0.4)' },
};

/** Icône de catégorie pour les onglets filtre */
export const CATEGORY_ICONS: Record<ServiceMeta['category'], string> = {
  fundamental: '⚙️',
  jay: '🦅',
  tool: '🔧',
  game: '🎮',
};

/** Labels des statuts */
export const STATUS_LABELS: Record<ServiceMeta['status'], string> = {
  production: 'Disponible',
  beta: 'Bêta',
  development: 'En dev',
  planned: 'Planifié',
};

/** Couleurs des statuts */
export const STATUS_COLORS: Record<ServiceMeta['status'], string> = {
  production: '#34d399',
  beta: '#fbbf24',
  development: '#60a5fa',
  planned: '#a78bfa',
};

/** Phrases de Miou pour chaque service (hover) */
export const MIOU_QUOTES: Record<string, string> = {
  'miyukini-central': 'C\'est le cœur de ton COG ! Tout commence ici~ ♪',
  'miyukini-web-portal': 'Le portail ouvre tes services au monde extérieur !',
  'jayxpose': 'Montre qui tu es au monde ! Ta vitrine, ton identité~ ✨',
  'jayfestival': 'Organise des événements incroyables ! J\'adore les festivals~ 🎉',
  'jaykonta': 'Les chiffres, c\'est important ! Garde un œil sur tes finances~ 💰',
  'jaykoa': 'Ton agenda universel ! Plus jamais de rendez-vous oubliés~ 📅',
  'jayrdv': 'Prendre rendez-vous en un clic, c\'est magique non ? ✨',
  'jayshop': 'Vends tes créations au monde entier ! En boutique ou en ligne~ 🛒',
  'jayfaim': 'Mmmh, j\'ai faim rien que d\'y penser... 🍔',
  'jaymanga': 'Des mangas ! Des mangas ! J\'en veux pleeein~ 📖',
  'calculator': 'Un outil simple mais tellement pratique !',
  'notes': 'Note tes idées avant qu\'elles s\'envolent~ 💭',
  'text-editor': 'Pour écrire de beaux textes, c\'est par ici !',
  'miyuclicker': 'Click click click ! C\'est addictif, attention~ 👆',
  'lord-of-the-castle': 'Défends ton château ! Les ennemis approchent... ⚔️',
  'miyulifegame': 'Crée un monde et regarde-le vivre ! C\'est fascinant~ 🌍',
};

/** Phrase par défaut de Miou */
export const MIOU_DEFAULT = 'Bienvenue dans le Registre des Opérateurs ! Survole un service pour en savoir plus~ ♪';
