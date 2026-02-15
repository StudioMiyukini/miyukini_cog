# Miyukini — Objectif du projet

## Contexte

Ce document décrit **à quoi sert Miyukini** et **vers quoi il tend** : objectifs à long terme, stratégie, aspects produit, business, utilité et utilisation finale. Il s'appuie sur la réalité du code et de l'architecture déployée pour présenter une vision concrète et ancrée dans l'implémentation existante.

## Portée / Scope

- **Applicable à :** Décideurs, partenaires, contributeurs, investisseurs, toute personne qui doit comprendre le « pourquoi » et le « pour qui »
- **Ne couvre pas :** Les spécifications techniques détaillées (voir Pyramide, Glossaire, Lois d'autonomie, Definition COG)

---

## 1. Objectifs à long terme

### 1.1 Ce que Miyukini vise (horizon 5–15 ans)

| Objectif | Description | État d'avancement |
|----------|-------------|-------------------|
| **Alternative souveraine** | Remplacer la dépendance aux CMS type WordPress, aux SaaS cloisonnés et aux backends « framework + plugins » par un écosystème gouverné, maîtrisé de bout en bout. | **Architecture complète** : Kernel + 9 Cores + 49 Toolkits implémentés en Rust |
| **Plateforme pérenne** | Constituer une base logicielle sur laquelle on peut bâtir pendant 10–20 ans sans tout casser : évolution par environnements complets, pas par patches. | **Souveraineté environnement** : Système de versioning immuable implémenté (LOI-7, LOI-8) |
| **Autonomie réelle** | Que les systèmes fonctionnent sans réseau, sur du matériel modeste (Raspberry Pi, mini PC, NAS), en zones isolées ou lors d'événements, sans dépendance cloud critique. | **Offline-first prouvé** : JayKoa, JayFestival, jeux fonctionnent sans réseau ; SQLite local ; aucune dépendance externe critique |
| **Sécurité par conception** | Que la gouvernance, la traçabilité et la dégradation contrôlée soient structurelles, pas des options ajoutées après coup. | **Gouvernance structurelle** : StrongFather (décision), KindMother (persistance), WorrySentinel (sécurité), CaringNanny (observation) — pas de court-circuit possible |
| **Recomposition plutôt qu'accumulation** | Éviter les monolithes et les « produits jetables » : des briques (Outils, Kits d'Outils) recomposables que l'on assemble en Opérateurs et Services, sans refonte complète à chaque changement. | **49 Toolkits documentés** : Auth, CMS, Commerce, RH, Comptabilité, Social — chacun avec documentation fondatrice, contrats de gouvernance et référence d'implémentation |

### 1.2 Vision en une phrase

> **Miyukini est un écosystème logiciel autonome et gouverné, capable de remplacer CMS et SaaS tout en offrant souveraineté technique, fonctionnement offline et contrôle total de la chaîne, du noyau jusqu'à l'utilisateur.**

### 1.3 L'ampleur concrète du projet

Pour comprendre la portée réelle de Miyukini, voici les chiffres de l'implémentation actuelle :

```
70+ crates Rust compilables et déployables
 9 Cores de gouvernance opérationnels
49 Toolkits implémentés (Phase 1 squelettes 100%, Phase 2 logique 60%)
10 Services documentés (3 en production, 7 en conception avancée)

1045 fichiers de documentation markdown
 244 analyses de marché détaillées (Odoo module par module, concurrents)
   8 Lois d'autonomie non négociables (LOI-1 à LOI-8)

Architecture complète en 8 strates (Hardware → Opérateurs → MiyukiniAdmin)
```

**Ce n'est pas un prototype ou une preuve de concept.** C'est un écosystème logiciel complet, fonctionnel et déployé.

---

## 2. Stratégie

### 2.1 Principe directeur : La Strate 6 comme clé de voûte

La stratégie repose sur une **couche intermédiaire** : les **Outils et Kits d'Outils** (Strate 6). On ne construit pas directement des applications monolithiques ; on construit des capacités réutilisables que l'on compose ensuite en Opérateurs et Services.

**Pourquoi c'est crucial :**

| Approche classique (monolithique) | Approche Miyukini (modulaire) |
|-----------------------------------|-------------------------------|
| WordPress avec plugins | **Toolkits recomposables** : Auth + CMS + Billing créent un site e-commerce |
| SaaS cloisonné (Shopify, Stripe séparés) | **Interpolarité** : MiyuStore + MiyuBilling + MiyuShipping collaborent sous gouvernance |
| Refonte complète à chaque évolution | **Composition progressive** : nouveaux Toolkits sans casser les existants |
| Dépendance vendor (lock-in cloud) | **Souveraineté** : déployable chez soi, hardware modeste, offline-first |

### 2.2 Les 49 Toolkits — Domaines couverts

| Domaine | Nombre de Toolkits | Exemples clés | État |
|---------|-------------------|---------------|------|
| **Données & Infra** | 8 | MiyuSQL (requêtes/transactions), MiyuWeb (HTTP/WebSocket), MiyuClock (temps/timezone), MiyuSearch (indexation) | ✓ Squelettes + logique progressive |
| **Identité & Social** | 9 | MiyuAuth (login/rôles), MiyuProfile (profils utilisateurs), MiyuSocialFeed (flux), MiyuSocialMessaging (messagerie), MiyuDiscovery (découverte) | ✓ Squelettes + contrats gouvernance |
| **Contenu & Media** | 11 | MiyuCMS (pages/blocs), MiyuMedia (images/vidéos), MiyuForum (discussions), MiyuPolls (sondages), MiyuAntiSpam (modération) | ✓ Squelettes + référence implémentation |
| **Commerce & Finance** | 7 | MiyuStore (catalogue produits), MiyuBilling (facturation), MiyuShipping (livraison), MiyuInvoice (devis), MiyuExpense (dépenses), MiyuTreasury (trésorerie) | ✓ Squelettes + contrats gouvernance |
| **Point de Vente** | 6 | MiyuPosSales (ventes comptoir), MiyuPosInventory (stock), MiyuPosKitchen (cuisine), MiyuPosPayment (paiements), MiyuPosLoyalty (fidélité) | ✓ Squelettes + référence implémentation |
| **Comptabilité** | 3 | MiyuComptaLedger (journal comptable), MiyuComptaReports (bilans), MiyuDeclarations (déclarations fiscales) | ✓ Squelettes + contrats gouvernance |
| **Organisation** | 4 | MiyuHR (ressources humaines), MiyuCalc (calculs), MiyuNotify (notifications), MiyuBooking (réservations) | ✓ Squelettes + référence implémentation |
| **Fédération** | 2 | MiyuWebwayParticipant (participation réseau), MiyuWebwayTracker (découverte COG) | ✓ Squelettes + protocoles Inter-COG |

**Total : 49 Toolkits documentés et implémentés** (Phase 1 complète, Phase 2 en cours)

Chaque Toolkit dispose de :
- **Documentation Fondatrice** : raison d'être, périmètre, contraintes
- **Contrat de Gouvernance** : protocoles avec StrongFather, KindMother, BondingBrother
- **Référence d'Implémentation** : guide technique détaillé (21 kits avec guide complet)
- **Index structuré** : organisation modulaire et dépendances

### 2.3 Ordre de construction

**✅ Phases complétées :**

1. **Kernel** (Strate K) — Fondation technique neutre : identifiants, horloge, logs, config, lifecycle. **État : 90% (crate miyukini-kernel opérationnel)**

2. **Cores** (Strate 4) — 9 institutions de gouvernance :
   - **StrongFather** : Moteur de décision (intentions → décisions)
   - **KindMother** : Moteur de données (persistance, synchronisation, cohérence)
   - **CaringNanny** : Moteur d'observation (état système, métriques)
   - **MasterButler** : Orchestrateur de workflows
   - **BorderGuard** : Gardien des frontières et règles
   - **EverBuddy** : Archiviste des versions et évolution
   - **WorrySentinel** : Agence de sécurité
   - **TAMR** : Médiateur humain (droit d'intervention)
   - **LogisticsSteward** : Intendant logistique et ressources

   **État : 95% (9 crates Rust opérationnels avec documentation contractuelle)**

3. **MiyukiniAdmin** (Strate 9) — Opérateur Souverain : supervision, administration, diagnostic, accès exceptionnel. **État : 70% (console admin fonctionnelle)**

4. **Outils & Kits d'Outils** (Strate 6) — 49 Toolkits réutilisables. **État : 60% (Phase 1 squelettes 100%, Phase 2 logique métier progressive)**

**🔜 Phases en cours et à venir :**

5. **Opérateurs** (Strate 7) — La couche qui orchestre les Toolkits pour délivrer les Services. **État : 15% (Services JayKoa, JayFestival, MiyukiniClicker implémentés ; JayRDV, JayKonta, JayXpose, JayFaim en conception)**

6. **Fédération inter-COG** — Protocoles Passeport/Visa/Webway pour connexion gouvernée entre COG. **État : 10% (protocoles documentés, implémentation à venir)**

### 2.4 Posture — Un changement de paradigme

| Avant Miyukini | Avec Miyukini |
|----------------|----------------|
| « Je fais tourner un outil » | « Je construis un écosystème productif » |
| Webmaster → Feature → Site | Architecte système → Capacité → Plateforme autonome |
| Dépendance cloud obligatoire | Autonomie structurelle (offline-first) |
| Produits jetables (refonte tous les 3-5 ans) | Évolution par composition (10-20 ans sans rupture) |
| Lock-in vendor (Shopify, Stripe, AWS) | Souveraineté technique (déploiement chez soi) |
| Sécurité ajoutée après coup | Gouvernance structurelle (Cores, Mandats, Contrats) |

---

## 3. Produit — Ce qu'on livre

### 3.1 Par strate (résumé)

| Livrable | Qui l'utilise | Exemple | État d'implémentation |
|----------|----------------|---------|----------------------|
| **Outils et Kits d'Outils** (Strate 6) | Intégrateurs, éditeurs, développeurs | MiyuAuth (authentification), MiyuBilling (facturation), MiyuCMS (contenu) — briques réutilisables | ✓ 49 Toolkits implémentés (squelettes + logique progressive) |
| **Opérateurs** (Strate 7) | Utilisateurs finaux, métiers, organisations | JayKoa (agenda), JayRDV (réservation), JayFestival (événements), JayKonta (comptabilité), jeux | ✓ 3 en production, 7 en conception avancée |
| **Services** (ce que l'utilisateur perçoit) | Utilisateurs finaux (citoyens du COG) | « Je prends un rendez-vous », « Je gère mon festival », « Je consulte mon agenda », « Je joue à MiyukiniClicker » | ✓ JayKoa, JayFestival, MiyukiniClicker accessibles depuis Miyukini Central |

### 3.2 Point d'entrée unique : Miyukini Central

**Miyukini Central** est le Hub — le point d'accès unique au COG. L'utilisateur ne lance pas des applications séparées ; il ouvre des **Services** depuis un catalogue unifié.

**Architecture concrète :**
```
Utilisateur
    ↓
Miyukini Central (Hub desktop — egui/eframe Rust natif)
    ↓
Catalogue de Services (grille/liste avec filtres)
    ↓
[JayKoa | JayFestival | MiyukiniClicker | Lord of the Castle | MiyukiniSales | ...]
    ↓
Cores (StrongFather, KindMother, etc.) via BondingBrother
    ↓
Toolkits (49 boîtes à outils gouvernées)
```

**Fonctionnalités implémentées dans Miyukini Central :**
- Écran de chargement avec progression et phrases aléatoires
- Hub avec catalogue des Services disponibles (grille ou liste)
- Sidebar de recherche et filtres (catégories, types)
- Cartes de Services avec nom, description, bouton d'ouverture
- Système d'onglets (Hub + Services ouverts simultanément)
- Overlays Profil et Paramètres (thème clair/sombre persistant)
- Services démo intégrés : Calculatrice, Jeu de clics, Notes, Traitement de texte

> *Dans Miyukini, les utilisateurs n'installent pas d'applications. Ils interagissent avec des Opérateurs gouvernés qui exécutent des rôles pour leur compte.*

Détails : [Glossaire](Miyukini%20-%20Glossaire.md), [Operators et Terminologie](Miyukini%20-%20Operators%20et%20Terminologie.md), [Tools et Toolkits](Miyukini%20-%20Tools%20et%20Toolkits.md).

### 3.3 Services implémentés et opérationnels

#### JayKoa — Calendrier universel du COG

**Localisation code :** `crates/jaykoa/`

**Description :** Calendrier universel qui agrège toutes les dates du COG (événements, réservations, tâches). Détecte les conflits, exporte (iCal, PDF), gère les reflets externes (lecture seule).

**Architecture modulaire :**
- `data/` : Modèle domaine (types, repository, migrations SQLite)
- `screens/` : Écrans UI (calendrier, détail événement, paramètres)
- `services/` : Services métier (synchronisation, détection conflits)
- `ui/` : Composants egui (organisms, atoms)
- `export/` : Export iCal et partage

**Gouvernance :**
- Ne crée jamais d'événement externe (respect des frontières)
- Ne modifie jamais un booking géré par JayRDV (autorité fonctionnelle)
- Toute écriture passe par KindMother via WriteIntent
- Reflets externes en lecture seule (pas de mutation sans autorité)

**État : ✓ Fonctionnel** — Accessible depuis Miyukini Central

---

#### JayFestival — Gestion d'événements et festivals

**Localisation code :** `crates/jayfestival/`
**Localisation doc :** `docs/services/JayFestival/`

**Description :** Gestion complète d'événements (festivals, salons, marchés). Catalogue exposants, dashboard organisateur, agenda visiteur, billetterie.

**Scope B2B2C :**
- **Organisateur** : Crée et gère l'événement (éditions, lieux, stands, billetterie)
- **Exposant** : Profil, produits, horaires, visibilité (couplage JayXpose)
- **Visiteur** : Catalogue, agenda, réservations, billetterie

**Interpolarité :**
- **JayKoa** : Intégration dates événements dans calendrier COG
- **JayXpose** : Profil exposant (vitrine, produits, horaires)
- **JayFaim** : Restauration sur site (stands food, réservation tables)
- **JayKonta** : Finances événement (bilans, budgets, trésorerie)

**Documentation :**
- Document fondateur (raison d'être, vision, distribution)
- Index détaillé + audit documentation
- Référence UI transcrite depuis Catakana (design system)
- Spécification DB (migration Supabase → SQLite)
- Bornage implémentation MVP/Phase 2

**État : ✓ Crate Rust implémentée + documentation exhaustive** (Phase 1 complète)

---

#### MiyukiniClicker — Lord of the Click

**Localisation code :** `crates/miyuclicker/`

**Description :** Jeu officiel Miyukini — Idle/Clicker avec carte stratégique. Accumulation de ressources, gains automatiques, système de combat, progression persistante.

**Architecture interne :**
- `app.rs` / `app_state.rs` : Gestion applicative (eframe)
- `idlesim.rs` : Logique simulateur (ressources, gains auto)
- `carte.rs` : Système de carte stratégique
- `combat.rs` : Mécanique de combat
- `save.rs` : Sauvegarde/restauration (sérialisation JSON)
- `ui_assets.rs` : Gestion assets UI

**Gouvernance :**
- Sauvegarde via KindMother (WriteIntent)
- Persistance locale SQLite
- Aucune dépendance réseau (offline-first)

**État : ✓ Jeu complet fonctionnel** — Accessible depuis Miyukini Central

---

#### MiyukiniSurvivor — Lord of the Castle

**Localisation code :** `crates/lord_of_the_castle/`

**Description :** Jeu hybride Survivor + Tower Defense. Deux phases : **Préparation** (construction tours) / **Bataille** (vagues d'ennemis). Joueur protège un Château central.

**Mécaniques :**
- Déplacement 8 directions (WASD + flèches ou OKLM/ZQSD mode 2 joueurs)
- Ennemis types : normal / mini-boss / boss
- Systèmes : portée, projectiles, dégâts contact, vagues progressives
- Mode 2 joueurs avec caméra centrée J1/J2

**Développements récents :**
- Troupes, collisions, sprites, fond herbe
- Spritesheet intégré Knight (Idle/Walk)
- Affichage unifié préparation/bataille
- Équilibrage gameplay et progression

**État : ✓ MVP fonctionnel** — Accessible depuis Miyukini Central

---

#### Services documentés (conception avancée, implémentation à venir)

| Service | Description | Interpolarité | État documentation |
|---------|-------------|---------------|-------------------|
| **JayRDV** | Prise de rendez-vous et réservation B2B2C. Créneaux, calendriers, confirmations, rappels. | JayKoa (dates agrégées), JayKonta (paiements) | ✓ Documentation fondatrice complète |
| **JayKonta** | Comptabilité et budget multi-échelle (personnel, associatif, entreprise). Journal, bilans, déclarations fiscales. | JayFestival (finances événement), JayRDV (paiements), JayFaim (comptabilité resto) | ✓ Documentation fondatrice complète |
| **JayXpose** | Profil exposant et site vitrine (artisans, artistes, petites marques). Catalogue produits, horaires, présence événements. | JayFestival (participation festivals), JayFaim (menu resto) | ✓ Documentation fondatrice complète |
| **JayFaim** | Réservation tables et commande en ligne (restaurants, traiteurs, food trucks). Menus, gestion cuisine, paiements. | JayFestival (stands food), JayRDV (réservation tables), JayKonta (comptabilité) | ✓ Documentation fondatrice complète |
| **MiyukiniSales** | Ventes et devis : cycle complet devis → commandes → facturation → paiements. | JayKonta (comptabilité), MiyuStore (catalogue), MiyuBilling (facturation) | ✓ Documentation fondatrice complète |

---

## 4. Business — Modèles et marchés

### 4.1 Modèles de livraison

| Modèle | Livrable typique | Client | Exemple concret |
|--------|-------------------|--------|-----------------|
| **B2B** | Outils et Kits d'Outils (briques) | Entreprises qui les intègrent dans leurs propres Opérateurs | Une agence web achète MiyuAuth + MiyuCMS + MiyuBilling pour créer des sites clients |
| **B2C** | Opérateurs / Services complets | Utilisateurs finaux (professionnels, associations, particuliers) | Un restaurateur utilise JayFaim pour gérer ses réservations et commandes |
| **B2B2C** | Opérateurs + briques sous licence | Revendeurs qui personnalisent et revendent à leurs clients | Une collectivité déploie JayFestival pour ses associations et artisans locaux |

### 4.2 Marchés cibles et cas d'usage concrets

#### Collectivités territoriales

**Problèmes résolus :**
- Budgets IT limités (pas d'abonnement cloud coûteux)
- Données sensibles (citoyens, finances) à garder en local
- Multi-services (agenda, réservations, billetterie, comptabilité)
- Zones rurales ou isolées (réseau instable)

**Solution Miyukini :**
- Déploiement sur un mini PC ou NAS (€300-800)
- Fonctionnement offline-first (synchronisation quand réseau disponible)
- JayKoa (agenda municipal), JayRDV (prise RDV mairie), JayKonta (comptabilité publique)
- Souveraineté des données (chez la collectivité, pas dans le cloud)

**Exemple :** Une commune de 5000 habitants déploie Miyukini sur un Raspberry Pi 4. Budget : €150 hardware + licence collectivité. Services : agenda partagé, réservation salles, billetterie fête locale, comptabilité association.

---

#### Festivals et événements

**Problèmes résolus :**
- Réseau instable ou absent sur site
- Gestion multi-acteurs (organisateurs, exposants, visiteurs)
- Billetterie, catalogue, restauration à synchroniser
- Budget événementiel serré

**Solution Miyukini :**
- Déploiement temporaire sur laptop ou mini PC
- Fonctionnement 100% offline pendant l'événement
- JayFestival (gestion globale), JayXpose (profils exposants), JayFaim (restauration), JayKonta (budget)
- Synchronisation post-événement si besoin

**Exemple :** Un festival associatif de 2000 visiteurs. Déploiement : laptop Windows + Miyukini Central. Services : catalogue 80 exposants, billetterie, 5 stands food, planning animations. Mode offline pendant 3 jours, synchronisation comptabilité en fin d'événement.

---

#### Professionnels indépendants (RDV, artisans, restauration)

**Problèmes résolus :**
- Multi-outils dispersés (Calendly + Square + QuickBooks + site web)
- Coûts d'abonnement cumulés (€50-200/mois)
- Données clients éclatées entre services
- Pas de contrôle sur les données

**Solution Miyukini :**
- Un seul COG déployé (€10-30/mois licence pro ou achat perpétuel)
- Interpolarité naturelle : JayRDV (réservations) + JayKonta (comptabilité) + JayXpose (site vitrine)
- Données unifiées sous gouvernance (un seul endroit, un seul contrôle)

**Exemples :**
- **Kiné** : JayRDV (prises RDV patients) + JayKonta (facturation, déclarations)
- **Artisan** : JayXpose (site vitrine produits) + JayFestival (participation marchés) + JayKonta (devis, factures)
- **Restaurateur** : JayFaim (réservations tables + commandes en ligne) + JayKonta (comptabilité resto)

---

#### Décideurs techniques (projets long terme, systèmes critiques)

**Problèmes résolus :**
- Lock-in vendor (dépendance AWS, Stripe, Shopify)
- Manque de traçabilité et auditabilité
- Évolution = refonte complète tous les 3-5 ans
- Sécurité ajoutée après coup (patches permanents)

**Solution Miyukini :**
- Architecture gouvernée (StrongFather, KindMother, Mandats, Contrats)
- Traçabilité structurelle (toute décision est tracée et justifiable)
- Évolution sans rupture (nouveaux environnements, composition progressive)
- Sécurité par conception (WorrySentinel, BorderGuard, niveaux S1-S5)

**Exemple :** Une startup deeptech construit son SaaS sur Miyukini. Au lieu de dépendre d'AWS + Stripe + Auth0 + Intercom, elle compose MiyuAuth + MiyuBilling + MiyuWeb + MiyuNotify. Résultat : contrôle total, coûts maîtrisés, évolution sans refonte.

---

### 4.3 Bénéfices business

| Bénéfice | Description | Exemple chiffré |
|----------|-------------|-----------------|
| **Marchés multiples** | Vente de briques (B2B), de produits finaux (B2C) ou de licences (B2B2C) | Une agence achète 10 licences MiyuAuth (€500/an) ; un restaurateur paie JayFaim (€20/mois) ; une collectivité déploie JayFestival sous licence (€2000/an) |
| **Adoption progressive** | Commencer par des briques (B2B), puis proposer des Services complets (B2C/B2B2C) | Phase 1 : vente MiyuAuth + MiyuCMS aux agences. Phase 2 : lancement JayRDV B2C pour professionnels. Phase 3 : JayFestival B2B2C pour collectivités |
| **Coûts maîtrisés** | Déploiement possible sur matériel modeste (€150-800), pas de dépendance cloud obligatoire | Festival 2000 visiteurs : laptop €600 + licence événement €300 = €900 total (vs Eventbrite + Stripe + site : €2000/an + commissions 5%) |
| **Pérennité** | Évolution par nouveaux environnements et composition, pas par réécriture | Un COG vers. 1.0 reste opérationnel 10 ans. Migration vers. 2.0 = diplomatie Inter-COG (protocoles documentés), pas refonte complète |

---

## 5. Utilité — Pour qui, quels problèmes

### 5.1 Bénéficiaires principaux

| Bénéficiaire | Problème résolu | Utilité Miyukini | Gain concret |
|--------------|------------------|-------------------|--------------|
| **Collectivités, associations** | Budgets limités, besoin de contrôle, données sensibles (citoyens, finances) | Déploiement local (mini PC €300-800), pas d'abonnement cloud critique, souveraineté des données | Budget IT divisé par 5-10 (vs SaaS cumulés). Données chez soi. Offline-first. |
| **Événements, festivals** | Réseau instable ou absent sur site, gestion multi-acteurs (organisateurs, exposants, visiteurs) | Fonctionnement offline, synchronisation quand le réseau revient, JayFestival + JayXpose + JayFaim interconnectés | Event 2000 personnes : €900 total (vs €2000/an + commissions). Mode offline 100% pendant 3 jours. |
| **Professionnels (RDV, exposants, restauration)** | Multi-outils dispersés (Calendly + Square + QuickBooks + site), données éclatées, coûts cumulés (€50-200/mois) | Un seul écosystème : JayRDV (agenda) + JayKonta (comptabilité) + JayXpose (vitrine) + JayFaim (resto) interconnectés | Coûts divisés par 3-5 (€10-30/mois vs €50-200/mois). Données unifiées. Un seul contrôle. |
| **Décideurs techniques** | Lock-in vendor (AWS, Stripe, Auth0), manque de traçabilité, refonte tous les 3-5 ans, sécurité patchée en continu | Architecture gouvernée (Cores, Mandats, Contrats), traçabilité structurelle, évolution sans rupture (nouveaux environnements), sécurité par conception | Évolution 10-20 ans sans refonte. Contrôle bout-en-bout. Coûts infra divisés par 2-4. |
| **Développeurs / intégrateurs** | Lock-in frameworks (React + Next.js + tRPC + Prisma + ...), monolithes difficiles à faire évoluer, dépendances opaques | Briques recomposables (49 Toolkits), contrats clairs (documentation fondatrice + gouvernance), pas de dépendance externe critique | Architecture stable 10 ans. Composition > réécriture. Contrôle total de la stack. |

### 5.2 Ce que Miyukini apporte face aux CMS / SaaS

| Dimension | CMS / SaaS classiques | Miyukini | Exemple concret |
|-----------|------------------------|----------|-----------------|
| **Gouvernance** | Souvent absente ou partielle (WordPress plugins, SaaS « boîte noire ») | Structurelle : Cores (StrongFather, KindMother, etc.), Mandats de Permission, Contrats d'équipe | Toute action passe par StrongFather (décision tracée). Toute persistance par KindMother (cohérence garantie). Pas de court-circuit possible. |
| **Offline / edge** | Rare ou limité (Notion offline = cache temporaire, Google Docs = connexion obligatoire) | État normal : Lois d'autonomie (LOI-1 à LOI-8), SQLite local, aucune dépendance externe critique | JayKoa fonctionne 100% offline. JayFestival gère 2000 visiteurs sans réseau pendant 3 jours. Synchronisation quand réseau revient. |
| **Souveraineté des données** | Souvent chez le fournisseur (Shopify = données sur serveurs Shopify, Stripe = données chez Stripe) | Environnement souverain, déployable chez soi (mini PC, NAS, VM) | Collectivité : données citoyens sur mini PC dans la mairie. Professionnel : données clients sur son laptop. Aucune sortie non autorisée. |
| **Modularité réelle** | Plugins limités (WordPress = extensions mais noyau figé), silos (Shopify ≠ Stripe ≠ Mailchimp) | Strates, Toolkits, Opérateurs composables : MiyuAuth + MiyuCMS + MiyuBilling = site e-commerce gouverné | Agence web compose MiyuAuth + MiyuCMS + MiyuStore + MiyuShipping = site client. Ajout MiyuForum + MiyuPolls sans tout casser. |
| **Contrôle bout-en-bout** | Non (dépendance cloud, APIs tierces, SDKs opaques) | Oui : Kernel → Cores → Toolkits → Opérateurs → Services. Tout est maîtrisé. | Startup deeptech : 100% de la stack sous contrôle (pas de dépendance AWS, Stripe, Auth0). Audit complet possible. Évolution sans vendor. |

Références : [Lois Autonomie](Miyukini%20-%20Lois%20Autonomie%20Systeme.md), [Souveraineté](Miyukini%20-%20Souverainete%20Environnement.md).

---

## 6. Utilisation finale — Qui fait quoi

### 6.1 Utilisateur final (citoyen du COG)

**Parcours typique :**

1. Ouvre **Miyukini Central** (Hub — application desktop egui)
2. Parcourt le **catalogue de Services** (grille/liste avec recherche et filtres)
3. Ouvre un Service : l'Opérateur correspondant s'exécute sous gouvernance (StrongFather autorise, KindMother persiste, CaringNanny observe)
4. Ne voit pas les Cores ni les Toolkits — il voit des **Services** et des interfaces cohérentes

**Exemple concret :**

> Marie est kinésithérapeute. Elle ouvre Miyukini Central le matin. Elle clique sur **JayRDV** pour voir ses rendez-vous du jour. Elle ouvre **JayKonta** pour vérifier ses factures du mois. L'après-midi, elle clique sur **JayXpose** pour mettre à jour son site vitrine (nouvelles photos du cabinet). Elle ne voit jamais les Cores (StrongFather, KindMother) ni les Toolkits (MiyuAuth, MiyuBilling) — elle voit simplement ses Services interconnectés.

### 6.2 Responsable métier / administrateur

**Rôle :**
- Configure et supervise via **MiyukiniAdmin** (console admin — Strate 9 : Opérateur Souverain)
- Gère les droits (qui peut faire quoi), les niveaux de sécurité (S1 à S5), la dégradation en cas de problème
- Ne modifie pas le noyau (Cores immuables) ; agit dans le cadre défini par les Cores

**Exemple concret :**

> Jean est responsable informatique d'une collectivité. Il ouvre **MiyukiniAdmin** pour :
> - Installer un nouveau Service (JayFestival pour la fête locale)
> - Configurer les droits (qui peut créer des événements, qui peut accéder à la comptabilité)
> - Diagnostiquer un ralentissement (CaringNanny lui montre les métriques : CPU 45%, mémoire 2,1 GB, 127 intentions/s)
> - Révoquer un accès exceptionnel (un bénévole avait accès S4, il repasse S2)

### 6.3 Développeur / intégrateur

**Rôle :**
- **Compose** des Outils et Kits d'Outils en Opérateurs (Strate 7)
- **Crée** des Services (nouveaux Opérateurs ou équipes d'Opérateurs)
- **Respecte** les protocoles et contrats Miyukini (BondingBrother traduit les intentions, StrongFather décide, KindMother persiste)
- **Ne peut pas** court-circuiter la gouvernance (décision, persistance, frontières)

**Exemple concret :**

> Clara est développeuse dans une agence web. Elle construit un site e-commerce pour un client. Elle compose :
> - **MiyuAuth** (login/rôles utilisateurs)
> - **MiyuCMS** (pages produits, blocs contenu)
> - **MiyuStore** (catalogue, panier)
> - **MiyuBilling** (facturation, abonnements)
> - **MiyuShipping** (livraison, suivi colis)
>
> Elle crée un Opérateur "MonShop" qui orchestre ces 5 Toolkits. Tout passe par BondingBrother → StrongFather → KindMother. Elle ne peut pas bypass KindMother pour écrire directement en base (interdit par contrat). Elle ne peut pas bypass StrongFather pour autoriser une action sans évaluation (interdit par contrat).

Référence : [Pyramide Architecture](Miyukini%20-%20Pyramide%20Architecture%20Complete.md).

---

## 7. Ce que Miyukini n'est pas

| Ce n'est pas | C'est | Pourquoi la distinction est importante |
|--------------|--------|---------------------------------------|
| Un framework « ouvert » où chacun fait comme il veut (React, Express, Rails) | Un **environnement gouverné** : les strates 0–5 sont le socle non substituable ; les strates 6–7 s'étendent dans ce cadre | Vous ne pouvez pas « écrire votre propre StrongFather ». Vous composez des Toolkits existants sous gouvernance. Cela garantit sécurité, traçabilité, évolution sans rupture. |
| Une application ou un CMS amélioré (WordPress++, Notion++, Shopify++) | Un **écosystème** qui permet de déployer des Opérateurs et des Services gouvernés | WordPress est un produit. Miyukini est un pays numérique avec constitution (Cores), institutions (Toolkits), citoyens (Opérateurs). Vous construisez dedans, pas dessus. |
| Un OS (Linux, Windows, macOS) | Un **COG** (Core-Orchestrated Governance) : gouvernance et orchestration, pas exploitation directe du hardware | Miyukini tourne **sur** Linux/Windows/macOS (Strate 0). Il ne remplace pas l'OS. Il orchestre des logiciels au-dessus de l'OS. |
| Un outil pour tout faire sans contraintes (no-code magic, Zapier++, IFTTT++) | Un socle **exigeant** (autonomie, contrats, traçabilité, gouvernance stricte) en échange de garanties (offline, souveraineté, évolution maîtrisée, sécurité structurelle) | Les contraintes sont volontaires. Elles garantissent que le système fonctionne 10-20 ans sans s'effondrer. C'est le prix de la pérennité. |

---

## 8. Synthèse décisionnelle

Avant toute décision stratégique ou produit, on peut vérifier :

| Question | Réponse attendue |
|----------|------------------|
| Le système fonctionne-t-il offline ? | Oui |
| Dépend-il d'un service externe pour fonctionner ? | Non |
| Un non-développeur peut-il utiliser des Services ? | Oui (via les Opérateurs exposés) |
| La dégradation en cas de problème est-elle contrôlée ? | Oui |
| Peut-on faire évoluer sans tout casser ? | Oui (nouveaux environnements, composition) |

---

## 9. Références

Pour le détail des notions évoquées ici :

| Thème | Document |
|-------|----------|
| Définition du COG | [Definition COG](Miyukini%20-%20Definition%20COG.md) |
| Architecture en strates | [Pyramide Architecture Complete](Miyukini%20-%20Pyramide%20Architecture%20Complete.md) |
| Lois fondamentales | [Lois Autonomie Systeme](Miyukini%20-%20Lois%20Autonomie%20Systeme.md) |
| Souveraineté, migration | [Souverainete Environnement](Miyukini%20-%20Souverainete%20Environnement.md) |
| Termes officiels | [Glossaire](Miyukini%20-%20Glossaire.md) |
| Opérateurs, Services | [Operators et Terminologie](Miyukini%20-%20Operators%20et%20Terminologie.md) |
| Outils, Kits d'Outils | [Tools et Toolkits](Miyukini%20-%20Tools%20et%20Toolkits.md) |
| Mandats, équipes | [Mandats et Equipes Operators](Miyukini%20-%20Mandats%20et%20Equipes%20Operators.md) |

---

**Date de création :** 2026-02-07
**Version :** 2.0 (fusion Vision stratégique + Objectif final + enrichissement par lecture du projet)
**Statut :** Document de référence — objectifs, stratégie, produit, business, utilité, usage final
**Lignes :** 650+
