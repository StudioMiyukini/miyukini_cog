/**
 * Services (Registre d'Opérateurs) — aligné sur le catalogue Miyukini Central.
 */
export interface ServiceMeta {
  id: string;
  name: string;
  shortDesc: string;
  category: 'fundamental' | 'jay' | 'tool' | 'game';
}

export const SERVICES: ServiceMeta[] = [
  {
    id: 'miyukini-central',
    name: 'Miyukini Central',
    shortDesc: 'Hub de gestion des Services. Vitrine du Registre d\'Opérateurs.',
    category: 'fundamental',
  },
  {
    id: 'miyukini-web-portal',
    name: 'Miyukini Web Portal',
    shortDesc: 'Point d\'entrée web pour les utilisateurs externes. Façades publiques gouvernées.',
    category: 'fundamental',
  },
  {
    id: 'jayxpose',
    name: 'JayXpose',
    shortDesc: 'Profil exposant, site vitrine, catalogue.',
    category: 'jay',
  },
  {
    id: 'jayfestival',
    name: 'JayFestival',
    shortDesc: 'Éditions, exposants, événements.',
    category: 'jay',
  },
  {
    id: 'jaykonta',
    name: 'JayKonta',
    shortDesc: 'Budget, facturation (marques JayBudget / JayKonta).',
    category: 'jay',
  },
  {
    id: 'jaykoa',
    name: 'JayKoa',
    shortDesc: 'Agenda unifié, intégrateur des dates.',
    category: 'jay',
  },
  {
    id: 'jayrdv',
    name: 'JayRDV',
    shortDesc: 'RDV, créneaux, exceptions.',
    category: 'jay',
  },
  {
    id: 'jayfaim',
    name: 'JayFaim',
    shortDesc: 'Restauration, commandes, food trucks.',
    category: 'jay',
  },
  {
    id: 'calculator',
    name: 'Calculatrice',
    shortDesc: 'Outil de calcul intégré.',
    category: 'tool',
  },
  {
    id: 'notes',
    name: 'Notes',
    shortDesc: 'Notes rapides.',
    category: 'tool',
  },
  {
    id: 'text-editor',
    name: 'Éditeur de texte',
    shortDesc: 'Traitement de texte.',
    category: 'tool',
  },
  {
    id: 'miyuclicker',
    name: 'Lord of the Click',
    shortDesc: 'Idle / Clicker + carte stratégique.',
    category: 'game',
  },
  {
    id: 'lord-of-the-castle',
    name: 'Lord of the Castle',
    shortDesc: 'Miyukini Survivor — Survivor + Tower Defense.',
    category: 'game',
  },
];

export const CATEGORY_LABELS: Record<ServiceMeta['category'], string> = {
  fundamental: 'Fondamentaux',
  jay: 'Services Jay',
  tool: 'Outils',
  game: 'Jeux',
};
