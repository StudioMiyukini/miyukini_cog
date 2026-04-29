/**
 * Services détaillés — données complètes pour les pages de détail.
 * Aligné sur la documentation Miyukini COG.
 */

export interface ServiceFeature {
  title: string;
  description: string;
  icon: string;
}

export interface ServiceIntegration {
  serviceId: string;
  serviceName: string;
  description: string;
}

export interface ServiceUseCase {
  title: string;
  description: string;
  persona: string;
}

export interface ServiceDetailed {
  id: string;
  name: string;
  tagline: string;
  shortDesc: string;
  longDesc: string;
  category: 'fundamental' | 'jay' | 'tool' | 'game';
  type: 1 | 2 | 3; // Type de service (1: interne COG, 2: surface web, 3: inter-COG)
  features: ServiceFeature[];
  integrations: ServiceIntegration[];
  useCases: ServiceUseCase[];
  status: 'production' | 'beta' | 'development' | 'planned';
  icon: string;
  color: string;
}

export const SERVICES_DETAILED: ServiceDetailed[] = [
  // ============ SERVICES FONDAMENTAUX ============
  {
    id: 'miyukini-central',
    name: 'Miyukini Central',
    tagline: 'Le hub de votre COG',
    shortDesc: 'Hub de gestion des Services. Vitrine du Registre d\'Opérateurs.',
    longDesc: 'Miyukini Central est le point d\'entrée unique pour l\'utilisateur du COG. Il expose le Registre d\'Opérateurs, permet de naviguer entre les Services, et offre une vue unifiée de l\'écosystème. Central = COG.',
    category: 'fundamental',
    type: 1,
    features: [
      { title: 'Registre d\'Opérateurs', description: 'Catalogue complet des Services disponibles', icon: '📚' },
      { title: 'Navigation unifiée', description: 'Accès fluide entre tous les Services', icon: '🧭' },
      { title: 'Tableau de bord', description: 'Vue synthétique de l\'état du COG', icon: '📊' },
      { title: 'Profil utilisateur', description: 'Gestion du compte et préférences', icon: '👤' },
    ],
    integrations: [
      { serviceId: 'all', serviceName: 'Tous les Services', description: 'Point d\'accès central à tous les Services du COG' },
    ],
    useCases: [
      { title: 'Accès quotidien', description: 'Ouvrir Central pour accéder à ses Services', persona: 'Utilisateur COG' },
      { title: 'Découverte', description: 'Explorer les Services disponibles', persona: 'Nouvel utilisateur' },
    ],
    status: 'production',
    icon: '◇',
    color: '#2dd4bf',
  },
  {
    id: 'miyukini-web-portal',
    name: 'Miyukini Web Portal',
    tagline: 'La façade web de votre COG',
    shortDesc: 'Point d\'entrée web pour les utilisateurs externes. Façades publiques gouvernées.',
    longDesc: 'Le Portail expose les surfaces web des Services de type 2 aux utilisateurs externes (non-COG). Il gère les Façades Publiques Gouvernées : zones tampon unidirectionnelles où le COG sort vers l\'utilisateur. Portail = Web.',
    category: 'fundamental',
    type: 2,
    features: [
      { title: 'Façades Publiques', description: 'Exposition contrôlée des Services web', icon: '🌐' },
      { title: 'Authentification externe', description: 'Gestion des accès visiteurs', icon: '🔐' },
      { title: 'Multi-domaines', description: 'Support de plusieurs domaines personnalisés', icon: '🏷️' },
      { title: 'SEO optimisé', description: 'Référencement des pages publiques', icon: '🔍' },
    ],
    integrations: [
      { serviceId: 'jayrdv', serviceName: 'JayRDV', description: 'Réservation en ligne' },
    ],
    useCases: [
      { title: 'Site vitrine', description: 'Exposer son activité au public', persona: 'Entreprise' },
      { title: 'Billetterie', description: 'Vendre des billets en ligne', persona: 'Organisateur' },
    ],
    status: 'production',
    icon: '🌍',
    color: '#8b5cf6',
  },

  // ============ SERVICES JAY ============
  {
    id: 'jaykonta',
    name: 'JayKonta',
    tagline: 'Comptabilité multi-échelle',
    shortDesc: 'Budget, facturation (marques JayBudget / JayKonta).',
    longDesc: 'JayKonta est le Service de comptabilité multi-échelle. Il inclut JayBudget (Purse) pour les finances personnelles et JayKonta (Account) pour l\'entreprise. Journal comptable, bilans, déclarations fiscales et intégration des flux financiers.',
    category: 'jay',
    type: 1,
    features: [
      { title: 'JayBudget (Purse)', description: 'Budget personnel, suivi des dépenses quotidiennes', icon: '👛' },
      { title: 'JayKonta (Account)', description: 'Comptabilité entreprise, journal et bilan', icon: '📒' },
      { title: 'Mouvements', description: 'Saisie et catégorisation des opérations', icon: '💸' },
      { title: 'Récurrences', description: 'Gestion des dépenses/revenus récurrents', icon: '🔁' },
      { title: 'Prévisionnel', description: 'Projection des finances futures', icon: '📈' },
      { title: 'Déclarations', description: 'Préparation des déclarations fiscales', icon: '📄' },
    ],
    integrations: [
      { serviceId: 'jayrdv', serviceName: 'JayRDV', description: 'Paiements des réservations' },
      { serviceId: 'jayfaim', serviceName: 'JayFaim', description: 'Revenus restauration' },
      { serviceId: 'jayshop', serviceName: 'JayShop', description: 'Ventes et encaissements' },
    ],
    useCases: [
      { title: 'Budget familial', description: 'Suivre ses dépenses mensuelles', persona: 'Particulier' },
      { title: 'Micro-entreprise', description: 'Gérer sa comptabilité simplifiée', persona: 'Auto-entrepreneur' },
      { title: 'Association', description: 'Tenir les comptes d\'une association', persona: 'Trésorier' },
    ],
    status: 'production',
    icon: '💼',
    color: '#10b981',
  },
  {
    id: 'jaykoa',
    name: 'JayKoa',
    tagline: 'L\'agenda universel',
    shortDesc: 'Agenda unifié, intégrateur des dates.',
    longDesc: 'JayKoa est le calendrier universel de l\'écosystème Miyukini. Il agrège et orchestre les informations temporelles de tous les autres Services : RDV, éditions, participations. Détection de conflits, vues calendrier multiples et export iCal/PDF.',
    category: 'jay',
    type: 1,
    features: [
      { title: 'Vue unifiée', description: 'Tous vos événements en un seul endroit', icon: '📆' },
      { title: 'Multi-sources', description: 'Agrégation automatique des autres Services', icon: '🔗' },
      { title: 'Détection conflits', description: 'Alertes sur les chevauchements', icon: '⚠️' },
      { title: 'Vues multiples', description: 'Jour, semaine, mois, agenda', icon: '👁️' },
      { title: 'Export', description: 'iCal, PDF, partage de calendrier', icon: '📤' },
      { title: 'Rappels', description: 'Notifications configurables', icon: '🔔' },
    ],
    integrations: [
      { serviceId: 'jayrdv', serviceName: 'JayRDV', description: 'Synchronisation des rendez-vous' },
      { serviceId: 'jayfaim', serviceName: 'JayFaim', description: 'Réservations restaurant' },
    ],
    useCases: [
      { title: 'Planning pro', description: 'Voir tous ses RDV et événements', persona: 'Professionnel' },
      { title: 'Organisateur', description: 'Planifier ses éditions annuelles', persona: 'Organisateur' },
      { title: 'Famille', description: 'Coordonner l\'agenda familial', persona: 'Parent' },
    ],
    status: 'production',
    icon: '📅',
    color: '#6366f1',
  },
  {
    id: 'jayrdv',
    name: 'JayRDV',
    tagline: 'Prise de rendez-vous simplifiée',
    shortDesc: 'RDV, créneaux, exceptions.',
    longDesc: 'JayRDV est le Service de prise de rendez-vous et réservation en ligne. Pour les professionnels : gestion des créneaux et paramètres. Pour les clients : espace "Mes RDV". Pour les visiteurs : réservation sans compte.',
    category: 'jay',
    type: 2,
    features: [
      { title: 'Gestion créneaux', description: 'Définir ses disponibilités', icon: '⏰' },
      { title: 'Réservation en ligne', description: 'Page publique de prise de RDV', icon: '🖱️' },
      { title: 'Exceptions', description: 'Congés, indisponibilités ponctuelles', icon: '🚫' },
      { title: 'Confirmations', description: 'Emails et rappels automatiques', icon: '✉️' },
      { title: 'Paiement', description: 'Acompte ou paiement à la réservation', icon: '💳' },
      { title: 'Multi-services', description: 'Différents types de RDV avec durées variables', icon: '📑' },
    ],
    integrations: [
      { serviceId: 'jaykoa', serviceName: 'JayKoa', description: 'Synchronisation agenda' },
      { serviceId: 'jaykonta', serviceName: 'JayKonta', description: 'Paiements et facturation' },
      { serviceId: 'miyukini-web-portal', serviceName: 'Portail', description: 'Page de réservation publique' },
    ],
    useCases: [
      { title: 'Salon de coiffure', description: 'Réservations clients en ligne', persona: 'Coiffeur' },
      { title: 'Consultant', description: 'Planifier ses rendez-vous clients', persona: 'Freelance' },
      { title: 'Cabinet médical', description: 'Gérer les prises de RDV patients', persona: 'Praticien' },
    ],
    status: 'beta',
    icon: '📞',
    color: '#0ea5e9',
  },
  {
    id: 'jayshop',
    name: 'JayShop',
    tagline: 'Commerce et vente unifiés',
    shortDesc: 'Boutique en ligne et point de vente.',
    longDesc: 'JayShop est le Service commerce et vente. Onboarding vendeur, boutique web, point de vente (PoS), tickets et historique des ventes. Inclut la gestion d\'événements avec fiches, coûts et stocks temporaires.',
    category: 'jay',
    type: 2,
    features: [
      { title: 'Boutique en ligne', description: 'Vente sur le web avec panier et paiement', icon: '🛒' },
      { title: 'Point de vente (PoS)', description: 'Caisse pour vente physique', icon: '🏪' },
      { title: 'Gestion stocks', description: 'Inventaire et alertes de réapprovisionnement', icon: '📦' },
      { title: 'Tickets', description: 'Émission de tickets de caisse', icon: '🧾' },
      { title: 'Historique', description: 'Journal complet des ventes', icon: '📜' },
      { title: 'Événements', description: 'Stocks temporaires pour marchés/salons', icon: '🎪' },
    ],
    integrations: [
      { serviceId: 'jaykonta', serviceName: 'JayKonta', description: 'Comptabilité des ventes' },
    ],
    useCases: [
      { title: 'Artisan créateur', description: 'Vendre ses créations en ligne et sur marchés', persona: 'Artisan' },
      { title: 'Commerçant', description: 'Gérer sa boutique physique et web', persona: 'Commerçant' },
      { title: 'Exposant', description: 'Tenir un stand sur un salon', persona: 'Exposant' },
    ],
    status: 'development',
    icon: '🛍️',
    color: '#f43f5e',
  },
  {
    id: 'jayfaim',
    name: 'JayFaim',
    tagline: 'Restauration et commandes',
    shortDesc: 'Restauration, commandes, food trucks.',
    longDesc: 'JayFaim est le Service de réservation de tables et commande en ligne pour restaurants, traiteurs et food trucks. Gestion des créneaux, menus et commandes.',
    category: 'jay',
    type: 2,
    features: [
      { title: 'Réservation tables', description: 'Booking en ligne avec gestion du plan de salle', icon: '🍽️' },
      { title: 'Click & Collect', description: 'Commande en ligne, retrait sur place', icon: '📱' },
      { title: 'Menus', description: 'Gestion de la carte et des menus', icon: '📋' },
      { title: 'Food truck mode', description: 'Gestion itinérante avec géolocalisation', icon: '🚚' },
      { title: 'Commandes', description: 'Suivi des commandes en cuisine', icon: '👨‍🍳' },
      { title: 'Événements', description: 'Restauration sur festivals et marchés', icon: '🎪' },
    ],
    integrations: [
      { serviceId: 'jayrdv', serviceName: 'JayRDV', description: 'Réservations de tables' },
      { serviceId: 'jaykonta', serviceName: 'JayKonta', description: 'Comptabilité restauration' },
      { serviceId: 'jaykoa', serviceName: 'JayKoa', description: 'Planning des services' },
    ],
    useCases: [
      { title: 'Restaurant', description: 'Gérer réservations et commandes', persona: 'Restaurateur' },
      { title: 'Food truck', description: 'Vendre sur les marchés et festivals', persona: 'Food trucker' },
      { title: 'Traiteur', description: 'Prendre les commandes événementielles', persona: 'Traiteur' },
    ],
    status: 'planned',
    icon: '🍔',
    color: '#eab308',
  },

  // ============ OUTILS ============
  {
    id: 'calculator',
    name: 'Calculatrice',
    tagline: 'Calculs rapides',
    shortDesc: 'Outil de calcul intégré.',
    longDesc: 'Une calculatrice simple et efficace intégrée au COG pour les calculs quotidiens.',
    category: 'tool',
    type: 1,
    features: [
      { title: 'Opérations de base', description: 'Addition, soustraction, multiplication, division', icon: '➕' },
      { title: 'Historique', description: 'Retrouver ses calculs récents', icon: '📜' },
    ],
    integrations: [],
    useCases: [
      { title: 'Calcul rapide', description: 'Effectuer un calcul pendant une saisie', persona: 'Utilisateur' },
    ],
    status: 'production',
    icon: '🔢',
    color: '#64748b',
  },
  {
    id: 'notes',
    name: 'Notes',
    tagline: 'Vos idées à portée de main',
    shortDesc: 'Notes rapides.',
    longDesc: 'Un bloc-notes simple pour capturer rapidement des idées, listes et mémos.',
    category: 'tool',
    type: 1,
    features: [
      { title: 'Notes texte', description: 'Saisie libre de notes', icon: '📝' },
      { title: 'Listes', description: 'Listes à puces et cases à cocher', icon: '☑️' },
      { title: 'Recherche', description: 'Retrouver ses notes par mots-clés', icon: '🔍' },
    ],
    integrations: [],
    useCases: [
      { title: 'Prise de notes', description: 'Noter une idée rapidement', persona: 'Utilisateur' },
    ],
    status: 'production',
    icon: '📝',
    color: '#a3a3a3',
  },

  // ============ JEUX ============
  {
    id: 'miyuclicker',
    name: 'Lord of the Click',
    tagline: 'Cliquez, construisez, dominez',
    shortDesc: 'Idle / Clicker + carte stratégique.',
    longDesc: 'Lord of the Click est un jeu idle/clicker avec une dimension stratégique sur carte. Récoltez des ressources, construisez votre empire et affrontez d\'autres joueurs.',
    category: 'game',
    type: 1,
    features: [
      { title: 'Clicker', description: 'Génération de ressources par clics', icon: '👆' },
      { title: 'Idle', description: 'Production passive même hors-ligne', icon: '💤' },
      { title: 'Carte stratégique', description: 'Expansion territoriale sur la carte', icon: '🗺️' },
      { title: 'Améliorations', description: 'Arbres de compétences et upgrades', icon: '⬆️' },
    ],
    integrations: [],
    useCases: [
      { title: 'Détente', description: 'Jouer pendant les pauses', persona: 'Joueur casual' },
    ],
    status: 'development',
    icon: '👑',
    color: '#fbbf24',
  },
  {
    id: 'lord-of-the-castle',
    name: 'Lord of the Castle',
    tagline: 'Survivez et défendez',
    shortDesc: 'Miyukini Survivor — Survivor + Tower Defense.',
    longDesc: 'Lord of the Castle combine le genre survivor (vagues d\'ennemis, power-ups) avec des éléments de tower defense. Construisez vos défenses et survivez aux vagues.',
    category: 'game',
    type: 1,
    features: [
      { title: 'Survivor', description: 'Affrontez des vagues d\'ennemis', icon: '⚔️' },
      { title: 'Tower Defense', description: 'Construisez des tours défensives', icon: '🏰' },
      { title: 'Power-ups', description: 'Collectez des améliorations temporaires', icon: '✨' },
      { title: 'Progression', description: 'Débloquez de nouveaux personnages et tours', icon: '📈' },
    ],
    integrations: [],
    useCases: [
      { title: 'Session courte', description: 'Une partie de 10-20 minutes', persona: 'Joueur' },
    ],
    status: 'development',
    icon: '🏰',
    color: '#7c3aed',
  },
  {
    id: 'miyulifegame',
    name: 'MiyukiniLifeGame',
    tagline: 'Simulez un monde vivant',
    shortDesc: 'Simulation de vie — style WorldBox.',
    longDesc: 'MiyukiniLifeGame est un simulateur de monde vivant inspiré de WorldBox. Créez des terrains, placez des civilisations et observez-les évoluer, se développer et interagir.',
    category: 'game',
    type: 1,
    features: [
      { title: 'Création de monde', description: 'Sculptez le terrain avec des outils divins', icon: '🌍' },
      { title: 'Civilisations', description: 'Placez des peuples et observez leur évolution', icon: '🏘️' },
      { title: 'Pouvoirs divins', description: 'Déclenchez catastrophes et bénédictions', icon: '⚡' },
      { title: 'Simulation', description: 'IA autonome des unités et nations', icon: '🤖' },
    ],
    integrations: [],
    useCases: [
      { title: 'Création', description: 'Construire un monde unique', persona: 'Créateur' },
      { title: 'Observation', description: 'Regarder les civilisations évoluer', persona: 'Observateur' },
    ],
    status: 'development',
    icon: '🌎',
    color: '#22c55e',
  },
];

/** Obtenir un Service par son ID */
export function getServiceById(id: string): ServiceDetailed | undefined {
  return SERVICES_DETAILED.find((s) => s.id === id);
}

/** Obtenir le Service suivant dans sa catégorie */
export function getNextService(currentId: string): ServiceDetailed | undefined {
  const current = getServiceById(currentId);
  if (!current) return undefined;
  const sameCategory = SERVICES_DETAILED.filter((s) => s.category === current.category);
  const index = sameCategory.findIndex((s) => s.id === currentId);
  return index >= 0 && index < sameCategory.length - 1 ? sameCategory[index + 1] : undefined;
}

/** Obtenir le Service précédent dans sa catégorie */
export function getPrevService(currentId: string): ServiceDetailed | undefined {
  const current = getServiceById(currentId);
  if (!current) return undefined;
  const sameCategory = SERVICES_DETAILED.filter((s) => s.category === current.category);
  const index = sameCategory.findIndex((s) => s.id === currentId);
  return index > 0 ? sameCategory[index - 1] : undefined;
}

/** Labels des catégories */
export const CATEGORY_LABELS: Record<ServiceDetailed['category'], string> = {
  fundamental: 'Fondamentaux',
  jay: 'Services Jay',
  tool: 'Outils',
  game: 'Jeux',
};

/** Labels des statuts */
export const STATUS_LABELS: Record<ServiceDetailed['status'], string> = {
  production: 'En production',
  beta: 'Bêta',
  development: 'En développement',
  planned: 'Planifié',
};

/** Couleurs des statuts */
export const STATUS_COLORS: Record<ServiceDetailed['status'], string> = {
  production: '#34d399',
  beta: '#fbbf24',
  development: '#60a5fa',
  planned: '#a78bfa',
};
