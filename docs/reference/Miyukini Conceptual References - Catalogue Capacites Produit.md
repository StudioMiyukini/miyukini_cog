# Catalogue des capacités produit génériques

> Document de référence produit.
> Niveau : **produit fonctionnel** (hors kernel, hors infra, hors implémentation).
> Objectif : servir de base durable (5–10 ans) pour la conception de SaaS, sites web et applications.

---

## Contexte

Ce catalogue s'inscrit dans un écosystème produit construit **au-dessus d'un kernel technique minimal** (infra uniquement). Le kernel reste gelé, sans métier, sans auth, sans base de données, sans framework applicatif. Les produits sont construits sous forme de **modules fonctionnels** qui consomment le kernel.

## Portée

- **Capacités indépendantes de toute techno** : formulées au niveau produit, pas d'implémentation.
- **Génériques et réutilisables** : applicables à la majorité des SaaS, sites web dynamiques, applications métier, plateformes interactives.
- **Usages réels** : pas de features théoriques ; chaque capacité correspond à des besoins répétés sur le terrain.

## Objectif

Servir de **catalogue de référence** pour :

- la conception de plusieurs produits (SaaS, sites, apps) ;
- la définition de modules produits ;
- les roadmaps et les choix de scope ;
- sans jamais polluer le kernel.

---

## 1. Gestion des utilisateurs

Capacité à représenter et gérer les comptes utilisateurs : données d'identité, préférences, états du compte et cycles de vie (inscription, activation, suspension, clôture). Distincte de l'accès et des rôles, qui concernent les permissions.

- Profils utilisateur (données publiques / privées)
- Avatar et photo de profil
- Préférences utilisateur
- États du compte (actif, suspendu, archivé)
- Invitations et onboarding
- Désinscription et suppression de compte
- Gestion multi-utilisateurs (équipes, organisations)
- Historique d'activité utilisateur
- Profils publics vs profils privés (champ et périmètre)

---

## 2. Accès et rôles

Capacité à définir qui peut faire quoi, dans quel périmètre. Couvre les rôles, les permissions, l'héritage et les restrictions par contexte (organisation, projet, espace, dossier).

- Rôles fonctionnels
- Rôles prédéfinis vs rôles personnalisés
- Permissions par action ou ressource
- Héritage de rôles
- Accès par contexte (projet, espace, équipe, dossier)
- Périmètre d'accès (organisation, projet, dossier)
- Délégation temporaire
- Restrictions fonctionnelles

---

## 3. Contenu et données métier

Capacité à créer, lire, modifier et supprimer des objets métier ; à gérer leurs états, relations, versions et métadonnées. Inclut les fichiers, pièces jointes, médias et le catalogue ou listage.

- Création / lecture / modification / suppression
- Statuts (brouillon, publié, archivé)
- Relations entre objets (références, hiérarchies, many-to-many)
- Versioning et historique
- Duplication et templates
- Métadonnées
- Fichiers, pièces jointes et médias
- Catalogue et listage

---

## 4. Formulaires et saisie utilisateur

Capacité à capturer des données via des formulaires configurables : champs, validations, logique conditionnelle, formulaires multi-étapes, sauvegarde partielle et réutilisation de modèles.

- Champs configurables
- Champs calculés
- Validation fonctionnelle
- Logique conditionnelle et règles de visibilité
- Formulaires multi-étapes
- Sauvegarde partielle
- Réutilisation de modèles (formulaires types)
- Pièces jointes et médias dans les formulaires

---

## 5. Navigation et organisation

Capacité à structurer l'accès aux contenus et aux fonctionnalités : espaces, hiérarchies, recherche, filtres, tris, vues et raccourcis. L'utilisateur peut s'orienter et retrouver l'information.

- Espaces et sections
- Hiérarchies (dossiers, catégories)
- Recherche fonctionnelle, full-text et par facettes
- Filtres et tris
- Vues personnalisées
- Favoris et raccourcis
- Breadcrumbs
- Raccourcis et tableau de bord personnalisable

---

## 6. Collaboration et partage

Capacité à partager des ressources, à collaborer (édition, commentaires, mentions) et à suivre l'activité commune. Le partage peut être par lien, avec échéance et droits différenciés.

- Partage de ressources
- Partage par lien avec échéance
- Droits différenciés (lecture, écriture, admin)
- Accès collaboratif
- Commentaires
- Mentions
- Suivi d'activité et activité récente partagée
- Notifications liées à la collaboration

---

## 7. Communication et notifications

Capacité à notifier l'utilisateur (in-app, messages transactionnels, alertes, rappels) et à centraliser les notifications. L'utilisateur peut régler canaux, fréquence et état de lecture.

- Notifications in-app
- Messages transactionnels
- Alertes système
- Rappels
- Centre de notifications
- Préférences de canal et de fréquence
- Notifications groupées
- Marquer comme lu / non lu

---

## 8. Paramétrage et personnalisation

Capacité à configurer le produit : paramètres globaux, par utilisateur, par espace ou projet ; préférences d'affichage ; options fonctionnelles activables (feature toggles côté produit). Inclut thèmes et raccourcis clavier.

- Paramètres globaux
- Paramètres par utilisateur
- Paramètres par espace ou projet
- Préférences d'affichage
- Options fonctionnelles activables
- Thèmes visuels (clair, sombre)
- Raccourcis clavier

---

## 9. Administration produit

Capacité à gérer le produit côté administrateur : utilisateurs, contenus, modération, tableaux de bord, actions globales et de masse, délégation d'administration, journal des actions.

- Gestion des utilisateurs
- Gestion des contenus
- Modération
- Tableaux de bord administratifs
- Actions globales (suspension, purge, export)
- Sélection multiple et actions de masse
- Délégation d'administration
- Journal des actions admin

---

## 10. Observabilité produit (fonctionnelle)

Capacité à rendre visible l'activité utilisateur, les journaux fonctionnels, les statistiques d'usage et les états système du point de vue produit. **Ne couvre pas** les métriques infra, APM ou tracing technique.

- Activité utilisateur
- Journaux fonctionnels
- Statistiques d'usage
- États système visibles
- Indicateurs métier simples

---

## 11. Sécurité fonctionnelle

Capacité à auditer les actions sensibles, à conserver un historique des changements, à appliquer des restrictions fonctionnelles et à gérer signalement et modération. Inclut rétention, export pour conformité.

- Audit des actions sensibles
- Historique des changements
- Restrictions fonctionnelles
- Signalement d'abus et modération
- Durées de rétention des historiques
- Export des données pour conformité

---

## 12. Internationalisation et accessibilité

Capacité à proposer plusieurs langues, formats locaux (dates, nombres) et à rendre le produit accessible (contraste, navigation, alternatives). Inclut ordre et direction des textes, sous-titrage, conformité niveau produit (sans imposer de norme technique).

- Langues multiples
- Formats locaux (dates, nombres)
- Accessibilité fonctionnelle
- Ordre et direction des textes
- Sous-titrage et alternatives
- Textes personnalisables
- Adaptation culturelle
- Conformité niveau produit (ex. WCAG-like, sans imposer de norme technique)

---

## 13. Facturation et monétisation (conceptuelle)

Capacité à gérer les plans, offres, quotas, essais, périodes gratuites et l'historique de facturation **du point de vue fonctionnel**. Inclut gestion des moyens de paiement (concept), prorata, remboursements, factures et attestations. Aucune implémentation de passerelle imposée.

- Plans et offres
- Quotas et limites
- Accès conditionné au plan
- Essais et périodes gratuites
- Historique de facturation (fonctionnel)
- Gestion des moyens de paiement (concept)
- Prorata et remboursements
- Factures et attestations

---

## 14. Automatisation et règles

Capacité à exécuter des règles conditionnelles, des actions automatiques, des workflows simples et des circuits de validation ou d'approbation. Couvre les déclencheurs (temps réel ou différés) et les garde-fous.

- Règles conditionnelles
- Actions automatiques
- Workflows simples
- Déclencheurs (temps réel vs différés)
- Scénarios utilisateur
- Circuits de validation et d'approbation
- Limites et garde-fous

---

## 15. Import / export et interopérabilité

Capacité à importer et exporter des données, à définir des mappings fonctionnels et à tracer les opérations. Peut inclure synchronisation conceptuelle, formats courants et planification d'exports.

- Import de données
- Export de données
- Formats courants (CSV, etc.) et mapping fonctionnel
- Mapping fonctionnel des données
- Synchronisation conceptuelle
- Historique des imports / exports
- Planification d'exports
- Traçabilité des opérations

---

## 16. Rapports et analyse métier

Capacité à produire des tableaux de bord, KPIs, rapports prédéfinis ou ad hoc et des exports analytiques. **Distincte de l'observabilité produit** : ici, il s'agit de données métier et d'usage pour le pilotage, pas de métriques système.

- Tableaux de bord configurables
- Rapports standard et personnalisés
- Indicateurs et seuils
- Exports (graphiques, données)
- Périodes et comparaisons
- Filtres métier

---

## 17. Aide, support et guidance

Capacité à guider l'utilisateur et à réduire la charge du support : FAQ, aide in-product, tutoriels, parcours guidés, recherche d'aide et contact support (côté produit, canal et formulaire).

- FAQ et base de connaissances
- Aide contextualisée et tooltips
- Parcours guidés et wizards
- Tutoriels et démos
- Recherche d'aide
- Contact support (canal et formulaire côté produit)

---

## 18. Calendrier, planification et réservation

Capacité à gérer des agendas, des créneaux, des disponibilités et des réservations. Fréquente dans les SaaS et applications de services (rendez-vous, réservations, planification).

- Calendriers et vues (jour, semaine, mois)
- Créneaux et disponibilités
- Réservation et annulation
- Rappels et rappels de délai
- Récurrence et exceptions
- Conflits et débordements

---

## Capacités quasi-universelles

Présentes dans la très grande majorité des SaaS, sites web dynamiques et applications métier. À envisager par défaut en conception.

- Gestion des utilisateurs
- Accès et rôles
- Contenu et données métier
- Formulaires et saisie
- Navigation et organisation
- Paramétrage et personnalisation
- Administration produit
- Rapports et analyse métier

---

## Capacités fréquentes mais optionnelles

Très courantes selon le type de produit, mais pas indispensables à tous les cas. À inclure en fonction du positionnement, du métier et du scope.

- Collaboration et partage
- Communication et notifications
- Observabilité produit (fonctionnelle)
- Internationalisation et accessibilité
- Automatisation et règles (niveau simple)
- Aide, support et guidance
- Calendrier, planification et réservation
- Import / export et interopérabilité (niveau de base)

---

## Capacités avancées / à fort coût

Utiles dans certains contextes (enterprise, conformité, monétisation, personnalisation poussée) mais coûteuses à concevoir, à implémenter et à maintenir. À traiter en modules dédiés ou en phases ultérieures.

- Facturation et monétisation
- Automatisation avancée (règles complexes, circuits d'approbation lourds)
- Sécurité fonctionnelle poussée (audit détaillé, rétention, conformité)
- Interopérabilité étendue (synchronisation, connecteurs, APIs métier)
- Personnalisation profonde (white-label, chaînes de personnalisation)
- Rapports et analyse avancés (données volumineuses, prédictif, etc.) lorsqu'on les distingue du rapport « standard »

---

## Capacités spécifiques fréquemment rencontrées (hors liste précédente)

Capacités récurrentes dans certains types de produits (CMS, planification, événementiel, jeux, e‑commerce, caisse). Plus ciblées que les capacités génériques 1–18 ; utiles pour le scope et la roadmap de produits dans ces domaines.

### CMS / type WordPress

- Gestion de pages et articles
- Hiérarchie de contenus
- Taxonomies (catégories, tags)
- Blocs de contenu modulaires
- Gestion des médias (images, vidéos, fichiers)
- Brouillons, prévisualisation, publication programmée
- Thèmes fonctionnels (structure, pas design)

### Prise de rendez-vous / planification (type Planity)

- Gestion des agendas
- Créneaux et disponibilités
- Réservations
- Annulations et reports
- Gestion des ressources (personnes, salles, équipements)
- Notifications liées aux rendez-vous

### Événementiel / festival (type Catakana)

- Gestion d'événements
- Programmation (scènes, horaires, activités)
- Gestion des intervenants / exposants
- Inscriptions et participations
- Billetterie conceptuelle
- Gestion des bénévoles
- Communication événementielle

### Jeux web / MMO / idle / tower defense

- Comptes joueurs (profil de jeu)
- Progression et niveaux
- Ressources (gain, consommation)
- Systèmes de récompenses
- États persistants de jeu
- Boucles de gameplay (idle / temps réel)
- Classements (leaderboards)
- Événements temporaires
- Gestion de parties / sessions

### E‑commerce / e‑shop

- Catalogue produits
- Gestion des prix
- Panier
- Commandes
- Gestion des stocks
- Promotions et coupons
- Livraison et retrait
- Gestion des retours

### Logiciel de caisse / point of sale (type SumUp)

- Encaissement
- Gestion des moyens de paiement (conceptuelle)
- Tickets et reçus
- Gestion des articles
- Gestion des taxes
- Clôture de caisse
- Historique des ventes
- Gestion multi-caisses
