/**
 * Données des 8 Cores (Strate 4) — alignées sur la doc Miyukini COG.
 */
export interface Core {
  id: string;
  name: string;
  question: string;
  domain: string;
  shortDesc: string;
  decide: boolean;
  govern: boolean;
  observe: boolean;
}

export const CORES: Core[] = [
  {
    id: 'strongfather',
    name: 'StrongFather',
    question: 'Devrait-on faire cette action ?',
    domain: 'Décision stratégique',
    shortDesc: 'Émetteur des Mandats de Permission. Valide les Contrats d\'Équipe.',
    decide: true,
    govern: true,
    observe: false,
  },
  {
    id: 'kindmother',
    name: 'KindMother',
    question: 'Comment les données sont-elles persistées ?',
    domain: 'Données et persistance',
    shortDesc: 'Autorité absolue de la persistance et de la synchronisation.',
    decide: false,
    govern: true,
    observe: false,
  },
  {
    id: 'caringnanny',
    name: 'Caring Nanny',
    question: 'Dans quel état est le système ?',
    domain: 'Observation d\'état',
    shortDesc: 'Détecte, classe et propage les états. Observe sans modifier.',
    decide: false,
    govern: false,
    observe: true,
  },
  {
    id: 'masterbutler',
    name: 'Master Butler',
    question: 'Qu\'est-ce qui est possible ?',
    domain: 'Capacités et permissions',
    shortDesc: 'Registre des capacités. Déclare quels Outils existent.',
    decide: false,
    govern: true,
    observe: false,
  },
  {
    id: 'borderguard',
    name: 'Border Guard',
    question: 'Où sont les frontières ?',
    domain: 'Frontières et confiance',
    shortDesc: 'Définit les frontières et les niveaux de confiance.',
    decide: false,
    govern: true,
    observe: false,
  },
  {
    id: 'everbuddy',
    name: 'Ever Buddy',
    question: 'Comment évoluer sans rompre ?',
    domain: 'Cycle de vie',
    shortDesc: 'Gouverne versions, dépréciation et compatibilité.',
    decide: false,
    govern: true,
    observe: true,
  },
  {
    id: 'worrysentinel',
    name: 'WorrySentinel',
    question: 'Quel niveau de sécurité ?',
    domain: 'Gouvernance de sécurité',
    shortDesc: 'Niveaux de confiance (T0–T4) et de sécurité (0–4).',
    decide: true,
    govern: true,
    observe: true,
  },
  {
    id: 'tamr',
    name: 'TAMR',
    question: 'Quand l\'humain intervient-il ?',
    domain: 'Intervention humaine',
    shortDesc: 'Trust & Authority Mediation Resolver.',
    decide: true,
    govern: false,
    observe: false,
  },
];

export const TRUST_LEVELS = [
  { id: 'T0', name: 'Normal', impact: 'Toutes capacités disponibles' },
  { id: 'T1', name: 'Instable', impact: 'Surveillance accrue' },
  { id: 'T2', name: 'Dégradé', impact: 'Capacités réduites' },
  { id: 'T3', name: 'Restreint', impact: 'Gel des non-essentiels' },
  { id: 'T4', name: 'Bloqué', impact: 'Uniquement diagnostics' },
];

export const SECURITY_LEVELS = [
  { level: 0, name: 'Public' },
  { level: 1, name: 'Standard' },
  { level: 2, name: 'Sensitive' },
  { level: 3, name: 'Critical' },
  { level: 4, name: 'Highest' },
];
