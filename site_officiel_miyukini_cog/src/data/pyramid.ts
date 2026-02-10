/**
 * Pyramide des strates Miyukini — référence architecture.
 */
export interface Stratum {
  id: string;
  level: string;
  name: string;
  description: string;
  examples?: string[];
}

export const STRATA: Stratum[] = [
  {
    id: '0',
    level: '0',
    name: 'Hardware & OS',
    description: 'Réalité physique. Hors contrôle Miyukini.',
  },
  {
    id: 'k',
    level: 'K',
    name: 'Kernel',
    description: 'Substrat technique neutre. Id, Logger, Clock, Config, Lifecycle.',
    examples: ['miyukini-kernel'],
  },
  {
    id: '3',
    level: '3',
    name: 'Invariants & Contrats',
    description: 'Principes architecturaux. Définis dans le code des Cores.',
  },
  {
    id: '4',
    level: '4',
    name: 'Cores Système',
    description: '8 Cores de gouvernance. Décident ou gouvernent, n\'exécutent jamais.',
    examples: ['StrongFather', 'KindMother', 'Caring Nanny', 'Master Butler', 'Border Guard', 'Ever Buddy', 'WorrySentinel', 'TAMR'],
  },
  {
    id: '5',
    level: '5',
    name: 'Interfaces & Adaptation',
    description: 'BondingBrother — médiation entre Opérateurs et Cores.',
    examples: ['bondingbrother'],
  },
  {
    id: '6',
    level: '6',
    name: 'Outils & Kits d\'Outils',
    description: 'Capacités exécutables, sans autorité. Gouvernés par les Cores.',
    examples: ['Toolkits miyu*'],
  },
  {
    id: '7',
    level: '7',
    name: 'Opérateurs',
    description: 'Entités fonctionnelles gouvernées. Services et UIs.',
    examples: ['Miyukini Central', 'JayXpose', 'JayFestival', 'JayKonta', 'JayKoa', 'JayRDV'],
  },
  {
    id: '9',
    level: '9',
    name: 'MiyukiniAdmin',
    description: 'Opérateur Souverain. Console d\'administration (exception).',
    examples: ['miyukini-admin'],
  },
];
