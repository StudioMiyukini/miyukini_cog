/**
 * Glossaire — termes officiels Miyukini COG (extrait).
 */
export interface GlossaryTerm {
  term: string;
  definition: string;
}

export const GLOSSARY: GlossaryTerm[] = [
  {
    term: 'COG',
    definition: 'Core-Orchestrated Governance Environment. Environnement de gouvernance orchestré par des Cores. Miyukini n\'est pas un OS.',
  },
  {
    term: 'Opérateur',
    definition: 'Entité fonctionnelle gouvernée qui exécute. À ne pas confondre avec "produit" ou "app".',
  },
  {
    term: 'Service',
    definition: 'Capacité perçue par l\'utilisateur. Exposée via Miyukini Central ou le Portail.',
  },
  {
    term: 'Outil',
    definition: 'Capacité exécutable gouvernée, sans autorité. Les Outils font, ne décident jamais.',
  },
  {
    term: 'Kit d\'Outils',
    definition: 'Composition officielle d\'Outils (toolkit).',
  },
  {
    term: 'Registre d\'Opérateurs',
    definition: 'Catalogue des Services disponibles. Équivalent conceptuel du "marketplace".',
  },
  {
    term: 'Mandat de Permission',
    definition: 'Autorisation déléguée, temporaire et encadrée, émise par StrongFather.',
  },
  {
    term: 'Miyukini Central',
    definition: 'Hub de gestion des Services. Point d\'accès pour l\'utilisateur du COG. Central = COG.',
  },
  {
    term: 'Miyukini Web Portal',
    definition: 'Point d\'entrée web pour les utilisateurs externes. Portail = Web.',
  },
  {
    term: 'BondingBrother',
    definition: 'Médiation (Strate 5). Traduit les intentions des Opérateurs en demandes vers les Cores.',
  },
  {
    term: 'Équipe d\'Opérateurs',
    definition: 'Collectif gouverné d\'Opérateurs sous règles explicites. Minimum 2, sous Contrat d\'Équipe.',
  },
  {
    term: 'Façade Publique Gouvernée',
    definition: 'Zone tampon d\'exposition. Strictement unidirectionnelle : le COG sort vers l\'utilisateur.',
  },
];
