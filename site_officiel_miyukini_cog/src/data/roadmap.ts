/**
 * Roadmap du projet Miyukini COG
 */

export type PhaseStatus = 'completed' | 'in-progress' | 'planned';

export interface RoadmapMilestone {
  id: string;
  title: string;
  description: string;
  status: PhaseStatus;
}

export interface RoadmapPhase {
  id: string;
  name: string;
  description: string;
  status: PhaseStatus;
  milestones: RoadmapMilestone[];
}

export const ROADMAP_PHASES: RoadmapPhase[] = [
  {
    id: 'phase-1',
    name: 'Phase 1 — Fondations',
    description: 'Architecture de base, Kernel, et premiers Cores',
    status: 'completed',
    milestones: [
      {
        id: 'm1-1',
        title: 'Kernel Miyukini',
        description: 'Substrat technique neutre : Id, Logger, Clock, Config, Lifecycle',
        status: 'completed',
      },
      {
        id: 'm1-2',
        title: 'Architecture pyramidale',
        description: 'Définition des 10 strates et des règles de dépendance',
        status: 'completed',
      },
      {
        id: 'm1-3',
        title: 'Premiers Cores',
        description: 'StrongFather, KindMother, Master Butler opérationnels',
        status: 'completed',
      },
      {
        id: 'm1-4',
        title: 'BondingBrother',
        description: 'Couche de médiation entre Opérateurs et Cores',
        status: 'completed',
      },
    ],
  },
  {
    id: 'phase-2',
    name: 'Phase 2 — Gouvernance',
    description: 'Tous les Cores et système de gouvernance complet',
    status: 'completed',
    milestones: [
      {
        id: 'm2-1',
        title: '8 Cores complets',
        description: 'Tous les Cores de gouvernance implémentés et documentés',
        status: 'completed',
      },
      {
        id: 'm2-2',
        title: 'Lois d\'autonomie',
        description: 'LOI-1 à LOI-8 définies et appliquées',
        status: 'completed',
      },
      {
        id: 'm2-3',
        title: 'WorrySentinel',
        description: 'Niveaux de confiance (T0-T4) et sécurité (0-4)',
        status: 'completed',
      },
      {
        id: 'm2-4',
        title: 'TAMR',
        description: 'Trust & Authority Mediation Resolver pour intervention humaine',
        status: 'completed',
      },
    ],
  },
  {
    id: 'phase-3',
    name: 'Phase 3 — Toolkits',
    description: 'Outils et Kits d\'Outils de la strate 6',
    status: 'in-progress',
    milestones: [
      {
        id: 'm3-1',
        title: 'MiyuAuth',
        description: 'Authentification et gestion des identités',
        status: 'completed',
      },
      {
        id: 'm3-2',
        title: 'MiyuContent',
        description: 'Gestion de contenu et médias',
        status: 'in-progress',
      },
      {
        id: 'm3-3',
        title: 'MiyuBilling',
        description: 'Facturation et paiements',
        status: 'in-progress',
      },
      {
        id: 'm3-4',
        title: '49 Toolkits',
        description: 'Ensemble complet des Toolkits de base',
        status: 'in-progress',
      },
    ],
  },
  {
    id: 'phase-4',
    name: 'Phase 4 — Services',
    description: 'Opérateurs et Services utilisateur',
    status: 'in-progress',
    milestones: [
      {
        id: 'm4-1',
        title: 'Miyukini Central',
        description: 'Hub de gestion des Services — opérationnel',
        status: 'completed',
      },
      {
        id: 'm4-2',
        title: 'Services Jay',
        description: 'JayKoa, JayKonta, JayRDV',
        status: 'in-progress',
      },
      {
        id: 'm4-3',
        title: 'Miyukini Web Portal',
        description: 'Façades publiques pour utilisateurs externes',
        status: 'in-progress',
      },
      {
        id: 'm4-4',
        title: 'JayShop & JayFaim',
        description: 'Commerce et restauration',
        status: 'planned',
      },
    ],
  },
  {
    id: 'phase-5',
    name: 'Phase 5 — Jeux',
    description: 'Services de divertissement intégrés',
    status: 'in-progress',
    milestones: [
      {
        id: 'm5-1',
        title: 'Lord of the Click',
        description: 'Idle / Clicker avec carte stratégique',
        status: 'in-progress',
      },
      {
        id: 'm5-2',
        title: 'Lord of the Castle',
        description: 'Survivor + Tower Defense',
        status: 'in-progress',
      },
      {
        id: 'm5-3',
        title: 'MiyukiniLifeGame',
        description: 'Simulateur de monde vivant style WorldBox',
        status: 'in-progress',
      },
    ],
  },
  {
    id: 'phase-6',
    name: 'Phase 6 — Fédération',
    description: 'Interactions Inter-COG et fédération',
    status: 'planned',
    milestones: [
      {
        id: 'm6-1',
        title: 'Protocole Inter-COG',
        description: 'Communication sécurisée entre COGs',
        status: 'planned',
      },
      {
        id: 'm6-2',
        title: 'Migration diplomate',
        description: 'Transfert contrôlé entre environnements',
        status: 'planned',
      },
      {
        id: 'm6-3',
        title: 'Fédération explicite',
        description: 'Réseau de COGs interconnectés',
        status: 'planned',
      },
    ],
  },
];

export const PROJECT_STATS = [
  { label: 'Crates Rust', value: '70+', icon: '📦' },
  { label: 'Cores système', value: '8', icon: '⚙️' },
  { label: 'Toolkits', value: '49', icon: '🔧' },
  { label: 'Services', value: '10', icon: '🎯' },
  { label: 'Fichiers docs', value: '1045+', icon: '📄' },
  { label: 'Analyses marché', value: '244', icon: '📊' },
];

export const STATUS_LABELS: Record<PhaseStatus, string> = {
  completed: 'Terminé',
  'in-progress': 'En cours',
  planned: 'Planifié',
};

export const STATUS_COLORS: Record<PhaseStatus, string> = {
  completed: '#34d399',
  'in-progress': '#fbbf24',
  planned: '#a78bfa',
};
