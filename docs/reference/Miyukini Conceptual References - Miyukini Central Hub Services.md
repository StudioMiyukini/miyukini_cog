# Miyukini Conceptual References — Miyukini Central (Hub de gestion des Services)

## Contexte

**Miyukini Central** est le **hub de gestion des Services** de l'écosystème Miyukini COG. Il constitue la **vitrine** du Registre d'Opérateurs et le point d'entrée principal pour les utilisateurs : découverte des Services disponibles, accès aux Services déjà activés, lancement et gestion des sessions. Il ne remplace pas MiyukiniAdmin (console souveraine d'administration) ; il s'adresse à l'utilisateur final qui consomme des Services.

**Rôle fondateur :** *Exposer les Services de l'écosystème de façon utilisable — catalogue, « Mes Services », lancement gouverné.*

**Règle canonique (gravée dans le marbre) :** *Tous les Services ont comme point d'accès utilisateur Miyukini Central.* Aucun Service ne doit exposer à l'utilisateur final un point d'entrée UI concurrent du Hub. Voir [Miyukini Conceptual References - Barrieres Techniques COG et Point Acces Central](./Miyukini%20Conceptual%20References%20-%20Barrieres%20Techniques%20COG%20et%20Point%20Acces%20Central.md) (CANON-CENTRAL-1 à CANON-CENTRAL-4).

## Portee / Scope

- **Applicable a :** Conception, développement et exploitation du Hub Miyukini Central (desktop, web, Android).
- **Audience :** Architectes, développeurs, designers UX/UI, responsables sécurité, utilisateurs finaux.
- **Statut :** Document de référence conceptuel — source de vérité pour le Hub.
- **Hors scope :** MiyukiniAdmin (Strate 9, administration système), implémentation technique détaillée (voir Stack UI egui/eframe et contrats d'implémentation).

---

## 1. Definition et position dans la pyramide

### 1.1 Nature du Miyukini Central

| Aspect | Definition |
|--------|------------|
| **Type** | Opérateur d'Interface (Strate 7) |
| **Rôle** | Expose le catalogue des Services (Registre d'Opérateurs) et permet de découvrir, activer et lancer des Services |
| **Question fondamentale** | *« Quels Services sont disponibles, et comment y accéder ? »* |
| **Autorité** | Aucune — le Hub ne décide pas, ne persiste pas, n'émet pas de Mandat ; il traduit les intentions utilisateur vers les Cores via BondingBrother |

**Phrase fondatrice :**

> **Miyukini Central est la vitrine du Registre d'Opérateurs. L'utilisateur voit des Services ; le Hub orchestre l'accès sous gouvernance.**

### 1.2 Ce que Miyukini Central N'EST PAS

- **Pas un Opérateur Souverain** — il n'a pas l'autorité de MiyukiniAdmin.
- **Pas un catalogue de « produits » au sens commercial** — le catalogue est le Registre d'Opérateurs (Services gouvernés).
- **Pas une marketplace ouverte** — les Services sont déclarés dans l'environnement (Master Butler, StrongFather) ; pas d'« installation » libre.
- **Pas une source de vérité** — la vérité (catalogue, permissions, état) est dans les Cores ; le Hub affiche et relaie.

### 1.3 Position dans la pyramide Miyukini

```
STRATE 7 — Opérateurs
├── Miyukini Central (Opérateur d'Interface) ← Hub de Services
├── Autres Opérateurs d'Interface (dashboards, UIs métier)
├── Opérateurs de Service / Domaine (CMS, Caisse, Compta, etc.)
└── …

STRATE 9 — MiyukiniAdmin (exception, hors Hub)
```

Le Hub **consomme** les Cores (Strate 4) et les Outils/Kits (Strate 6) **via** BondingBrother (Strate 5) ; il n'accède jamais directement aux Cores.

---

## 2. Fonctionnement

### 2.1 Architecture logique

```
┌─────────────────────────────────────────────────────────────────┐
│                    Miyukini Central (client)                       │
│  UI (egui/eframe) · Intentions utilisateur · Affichage catalogue  │
└─────────────────────────────────────────────────────────────────┘
                                    │
                                    │ intentions (demande catalogue,
                                    │ « activer Service X », « lancer Service Y »)
                                    ▼
┌─────────────────────────────────────────────────────────────────┐
│                    BondingBrother (Strate 5)                      │
│  Traduction intentions ↔ demandes Cores · Médiation               │
└─────────────────────────────────────────────────────────────────┘
                    │                           │
                    ▼                           ▼
┌──────────────────────────────┐  ┌──────────────────────────────┐
│ Master Butler (Strate 4)     │  │ StrongFather (Strate 4)        │
│ Catalogue Opérateurs/Services│  │ Décision ALLOW/DENY           │
│ Découverte · Permissions     │  │ Émission Mandat de Permission │
└──────────────────────────────┘  └──────────────────────────────┘
                    │                           │
                    └─────────────┬─────────────┘
                                  ▼
┌─────────────────────────────────────────────────────────────────┐
│ WorrySentinel · Caring Nanny · Ever Buddy (état, sécurité, vie)   │
└─────────────────────────────────────────────────────────────────┘
```

- **Le Hub** envoie des **intentions** (ex. « liste des Services disponibles », « activer le Service Compta », « ouvrir le Service Caisse »).
- **BondingBrother** traduit ces intentions en demandes vers Master Butler (catalogue, permissions) et StrongFather (Mandat pour activer/lancer).
- **Master Butler** fournit le **catalogue** (Services, Opérateurs, métadonnées, état de vie ACTIF/DÉPRÉCIÉ/etc.).
- **StrongFather** décide si l'utilisateur peut accéder au Service et émet un **Mandat de Permission** si autorisé.
- **WorrySentinel** impose le niveau de sécurité et l'état de confiance ; **Caring Nanny** peut bloquer l'accès si l'environnement est dégradé ; **Ever Buddy** gère versions et dépréciation.

### 2.2 Flux typiques

| Flux | Acteurs | Description |
|------|---------|-------------|
| **Chargement du catalogue** | Hub → BondingBrother → Master Butler | Le Hub demande la liste des Services disponibles (filtrée par permissions, environnement, état de vie). Master Butler répond avec les métadonnées (nom, description, Opérateur(s), niveau de sécurité, version). |
| **Affichage « Mes Services »** | Hub → BondingBrother → StrongFather / Master Butler | Liste des Services auxquels l'utilisateur a déjà accès (sous Mandat ou éligibilité). Peut s'appuyer sur un cache local de Mandats actifs ou sur une requête « mes accès ». |
| **Activation / Lancement d'un Service** | Hub → BondingBrother → StrongFather | L'utilisateur clique « Ouvrir » sur un Service. Le Hub envoie l'intention « lancer Service X ». StrongFather évalue (permissions, Contrat d'équipe, état) et émet un Mandat de Permission si autorisé. Le Hub reçoit le Mandat (ou un refus) et ouvre l'interface du Service (fenêtre, onglet, URL) ou affiche une erreur. |
| **Vérification état environnement** | Hub → BondingBrother → Caring Nanny / WorrySentinel | Avant d'afficher des actions sensibles ou de lancer un Service, le Hub peut demander l'état de confiance (T0–T4) et le niveau de sécurité actif. Si dégradé, affichage limité ou message explicatif. |

### 2.3 Règles d'or

- **Le Hub ne décide jamais** — toute décision (autoriser, refuser, révoquer) appartient à StrongFather et aux Cores.
- **Le Hub ne persiste pas de données métier** — éventuellement cache local de préférences UI (thème, taille fenêtre) ou de Mandats en cours pour affichage ; pas de base de données Services.
- **Pas de communication directe Hub ↔ Opérateur** — toute interaction passe par BondingBrother et la gouvernance.
- **Un Service = ce que l'utilisateur perçoit** — le Hub affiche des Services ; derrière, un ou plusieurs Opérateurs (Équipe d'Opérateurs) sous Mandat.

---

## 3. Fonctionnalites

### 3.1 Catalogue des Services

- **Contenu :** Liste des Services disponibles dans l'environnement, déclarés auprès de Master Butler.
- **Métadonnées affichées (exemples) :** nom du Service, description courte, icône ou image, Opérateur(s) porteur(s), niveau de sécurité requis, état de vie (ACTIF, DÉPRÉCIÉ, BROUILLON), version, catégorie ou domaine (optionnel).
- **Filtres et recherche :** par nom, catégorie, état de vie ; tri (alphabétique, récent, populaire si métrique disponible).
- **Source de vérité :** Master Butler (catalogue) ; le Hub ne fait qu'afficher et filtrer côté client pour l'UX.

### 3.2 Mes Services

- **Contenu :** Services auxquels l'utilisateur a déjà accès (Mandat actif ou éligibilité confirmée).
- **Actions :** « Ouvrir » (lancement), « Détails », éventuellement « Retirer de Mes Services » (révocation ou désinscription selon politique).
- **État :** Indication si le Service est en cours d'utilisation (session ouverte), à jour ou déprécié (Ever Buddy).

### 3.3 Fiche Service (détail)

- **Contenu :** Description complète, Opérateur(s), prérequis (niveau de sécurité, environnement), version, licence ou conditions d'usage, lien « Ouvrir » ou « Activer ».
- **Actions :** « Ouvrir » / « Activer », « Ajouter à Mes Services » (si processus d'activation distinct), retour au catalogue ou à Mes Services.

### 3.4 Lancement d'un Service

- **Intention :** L'utilisateur demande à ouvrir un Service.
- **Processus :** Hub envoie l'intention à BondingBrother → StrongFather évalue et émet un Mandat (ou refuse) → Hub reçoit le résultat. Si autorisé : ouverture de l'interface du Service (fenêtre native, onglet web, ou redirection selon type d'Opérateur).
- **Échec :** Affichage d'un message gouverné (ex. « Accès refusé », « Environnement dégradé », « Service temporairement indisponible ») sans exposer de détail technique sensible (Maintenance explicable).

### 3.5 Mises à jour et état de vie

- **Affichage :** Indication « À jour », « Mise à jour disponible », « Déprécié » (Ever Buddy).
- **Action « Mise à jour » :** Selon politique environnement — peut déclencher une intention vers Ever Buddy / processus de mise à jour gouverné (pas d'auto-update sauvage par le Hub).

### 3.6 Paramètres et préférences (Hub)

- **Préférences locales uniquement :** thème (clair/sombre), langue, taille de fenêtre, ordre des listes. Pas de paramètres métier ni de permissions — ceux-ci relèvent des Cores.
- **Persistence :** via mécanisme client (eframe persistence, stockage local) sans remonter à KindMother pour ces données purement UI.

### 3.7 Connexion à l'environnement (COG)

- **Identification de l'environnement :** Affichage de l'identité de l'environnement connecté (LSI, VID ou WID selon contexte) et statut (connecté / hors ligne / dégradé).
- **Choix d'environnement :** Si l'utilisateur peut se connecter à plusieurs environnements (ex. COG local + COG distant), sélecteur d'environnement et gestion des sessions (sous gouvernance).

---

## 4. Outils (Tools) et Kits d'Outils (Toolkits)

Le Hub **utilise** des Outils et Kits d'Outils (Strate 6) pour son propre rendu et ses interactions ; il n'implémente pas la logique métier des Services.

### 4.1 Outils susceptibles d'être utilisés par le Hub

| Domaine | Outil / Toolkit (exemples) | Usage dans le Hub |
|---------|----------------------------|-------------------|
| **UI** | Stack egui/eframe (voir [Stack UI egui eframe](../ux_ui/Miyukini%20-%20Stack%20UI%20egui%20eframe.md)) | Rendu des écrans, panels, boutons, listes, navigation. |
| **Réseau / Appels** | Outils d'appel gouverné vers BondingBrother (API ou protocole COG) | Envoi des intentions (catalogue, activer, lancer) et réception des réponses. |
| **Présentation** | Layout, thème, accessibilité (si exposés en Tools) | Cohérence visuelle, thème clair/sombre, taille des polices. |
| **Horloge** | MiyuClock (trace only) | Affichage date/heure locale dans le Hub. |

Les Outils exacts seront définis dans la documentation fondatrice et les contrats du Hub (déclaration Master Butler, liaison Capability → Tool).

### 4.2 Ce que le Hub ne fait PAS avec les Outils

- **N'exécute pas** les Outils des Services (ex. MiyuSQL, MiyuInvoice) — ceux-ci sont exécutés par les Opérateurs qui portent les Services.
- **Ne déclare pas** de nouveaux Services ni de capacités métier — il consomme le catalogue fourni par Master Butler.

---

## 5. Opérateurs

### 5.1 Miyukini Central comme Opérateur

- **Type :** Opérateur d'Interface (Strate 7).
- **Rôle :** Exposer les Services de façon utilisable (catalogue, Mes Services, lancement).
- **Contrat d'équipe :** Le Hub peut faire partie d'une Équipe d'Opérateurs (ex. « Suite utilisateur ») avec d'autres Opérateurs d'Interface ; dans ce cas, lié par un Contrat d'équipe et un Mandat de Permission émis par StrongFather.
- **Déclaration :** Enregistré auprès de Master Butler comme Opérateur ; déclare les capacités qu'il utilise (lecture catalogue, demande Mandat, affichage) — pas d'autorité d'écriture métier.

### 5.2 Relation avec les Opérateurs qui portent les Services

- Les **Services** affichés dans le catalogue sont portés par des **Opérateurs** (ou Équipes d'Opérateurs) distincts (ex. Opérateur Compta, Opérateur Caisse).
- Le Hub **n'est pas** l'Opérateur de ces Services — il permet d'y **accéder**. Une fois le Service lancé, l'utilisateur interagit avec l'interface de l'Opérateur concerné (fenêtre, web, etc.).
- **Pas de dialogue direct** Hub ↔ Opérateur Compta/Caisse : tout passe par BondingBrother et StrongFather (Mandat, puis ouverture de l'UI de l'Opérateur).

---

## 6. Ecrans et vues

### 6.1 Liste exhaustive des écrans

| Ecran | Id / Nom logique | Contenu principal | Actions principales |
|-------|------------------|-------------------|----------------------|
| **Accueil / Dashboard** | `home` | Résumé : Mes Services récents, accès rapide au catalogue, état environnement (connecté, dégradé), actualités ou annonces (optionnel). | Navigation vers Catalogue, Mes Services, Paramètres. |
| **Catalogue** | `catalogue` | Liste des Services disponibles (cartes ou liste) ; filtres, recherche, tri. | Clic sur un Service → Fiche Service ; « Ajouter à Mes Services » si applicable. |
| **Fiche Service** | `service_detail` | Détail d'un Service (nom, description, Opérateur(s), prérequis, version, état de vie). | « Ouvrir », « Activer », « Retour ». |
| **Mes Services** | `my_services` | Liste des Services auxquels l'utilisateur a accès ; état (ouvert, à jour, déprécié). | « Ouvrir », « Détails », éventuellement « Retirer ». |
| **Lancement / Ouverture** | (transition) | Pendant la demande de Mandat : indicateur de chargement ou progression. Puis ouverture de l'UI du Service ou message d'erreur. | Annuler (si possible), Réessayer. |
| **Paramètres** | `settings` | Préférences UI : thème, langue, taille fenêtre ; optionnel : compte, environnement connecté (lecture seule). | Sauvegarder, Réinitialiser. |
| **À propos / Licence** | `about` | Version du Hub, crédits, licences (Miyukini, egui/eframe), lien vers politique de licence. | Fermer. |
| **Erreur / Refus** | `error` | Message gouverné en cas de refus d'accès, environnement dégradé, ou indisponibilité. | Retour, Réessayer, Contacter le support (si TAMR/processus défini). |

### 6.2 Navigation et structure de l'interface

- **Navigation principale :** Barre latérale (sidebar) ou barre haute : Accueil, Catalogue, Mes Services, Paramètres, À propos.
- **Zone centrale :** Contenu de l'écran actif (liste, fiche, formulaire).
- **Barre de statut (optionnelle) :** Environnement connecté, indicateur d'état (T0–T4 ou « OK » / « Dégradé »), horaire (MiyuClock).
- **Fenêtres modales :** Confirmation (ex. « Ouvrir ce Service ? »), erreur, À propos.

---

## 7. Interface et UI/UX

### 7.1 Principes UX

- **Clarté :** L'utilisateur doit comprendre en un coup d'œil quels Services sont disponibles et lesquels il peut ouvrir.
- **Cohérence :** Alignement avec la charte Miyukini (terminologie : Service, pas « app » ; « Ouvrir », « Mes Services »).
- **Feedback immédiat :** Clic « Ouvrir » → chargement visible → succès (ouverture) ou erreur (message gouverné).
- **Accessibilité :** Contraste, taille des cibles, navigation clavier, lecteurs d'écran (egui/eframe et bonnes pratiques).
- **Responsive / adaptatif :** Desktop (fenêtre redimensionnable), web (WASM), éventuellement tablette/Android — layout adapté.

### 7.2 Stack technique UI

- **Rendu :** egui / eframe (voir [Miyukini - Stack UI egui eframe](../ux_ui/Miyukini%20-%20Stack%20UI%20egui%20eframe.md)).
- **Thèmes :** Clair / sombre (egui Visuals) ; cohérence avec les préférences utilisateur.
- **Composants :** Panels (SidePanel, CentralPanel, TopBottomPanel), fenêtres (Window), boutons, listes, cartes, champs de recherche, messages d'erreur.

### 7.3 Parcours utilisateur (User Journey)

#### 7.3.1 Premier lancement

1. Ouverture du Miyukini Central (desktop ou web).
2. Connexion à l'environnement (si nécessaire) : sélection du COG ou authentification locale selon politique.
3. Affichage de l'Accueil : Mes Services (vide ou pré-rempli), lien vers Catalogue.
4. L'utilisateur peut aller au Catalogue pour découvrir les Services.

#### 7.3.2 Découverte et activation d'un Service

1. **Catalogue** : parcourir ou rechercher un Service.
2. **Fiche Service** : clic sur un Service → détail ; lecture des prérequis et description.
3. **Activation** : clic « Ouvrir » ou « Activer ». Le Hub envoie l'intention à StrongFather.
4. **Résultat :** Mandat émis → ouverture de l'interface du Service (nouvelle fenêtre, onglet, ou intégration selon type). Ou refus → message d'erreur gouverné.
5. L'utilisateur travaille dans l'Opérateur du Service ; le Hub reste disponible (Mes Services, Catalogue, Paramètres).

#### 7.3.3 Utilisation récurrente

1. Ouverture du Hub → **Mes Services**.
2. Clic « Ouvrir » sur un Service déjà activé → même flux (Mandat ou cache selon politique) → ouverture du Service.
3. Gestion des sessions : fermeture du Service depuis l'UI du Service ou depuis le Hub (si « Fermer la session » proposé).

#### 7.3.4 Sortie

1. Fermeture des Services ouverts (selon UX).
2. Fermeture du Hub (sauvegarde des préférences si persistence activée).
3. Révocation des Mandats à la déconnexion ou à l'expiration (géré par StrongFather / Cores).

### 7.4 Messages et erreurs

- **Messages gouvernés :** Pas de stacktrace ni de détail technique exposé. Exemples : « Accès refusé », « Ce Service n'est pas disponible dans votre environnement », « L'environnement est temporairement dégradé. Réessayez plus tard. »
- **Maintenance explicable :** Si un incident est tracé, le diagnostic côté Cores reste gouverné (traçabilité pour l'admin, pas pour l'utilisateur final dans le Hub).

---

## 8. Securite

### 8.1 Niveaux de sécurité (WorrySentinel)

- **Affichage :** Les Services peuvent être marqués par niveau de sécurité requis (0–4). Le Hub affiche éventuellement une indication (icône, badge) pour les Services sensibles.
- **Filtrage :** Seuls les Services auxquels l'utilisateur est éligible (permissions, niveau de sécurité) sont proposés ou ouverts — décision StrongFather / Master Butler.
- **Pas d'élévation de niveau par le Hub :** Le Hub ne peut pas contourner WorrySentinel ni accorder un accès non autorisé.

### 8.2 États de confiance (Caring Nanny)

- **T0 (Normal) :** Tous les Services éligibles sont accessibles.
- **T1–T4 (Instable à Bloqué) :** Le Hub peut afficher un bandeau ou un message (« Environnement dégradé ») et limiter les actions (ex. blocage des lancements en T3–T4 selon politique).
- **Source de vérité :** Caring Nanny ; le Hub interroge via BondingBrother et adapte l'affichage.

### 8.3 Mandats de Permission

- **Émission :** Uniquement par StrongFather. Le Hub envoie l'intention « lancer Service X » ; StrongFather évalue (Contrat d'équipe, permissions, état) et émet le Mandat (ou refuse).
- **Révocation :** StrongFather ou WorrySentinel (alerte) ; le Hub peut être notifié (session fermée, accès révoqué) et afficher un message.
- **Pas de stockage de Mandat sensible dans le Hub :** Au plus un identifiant ou un token opaque pour afficher « Session ouverte » ; jamais la clé ou le contenu complet du Mandat côté client non sécurisé.

### 8.4 Authentification et identité

- **Contexte :** L'utilisateur est supposé identifié dans l'environnement (COG). Comment (login, SSO, certificat) relève de la politique d'environnement et de MiyuAuth / Cores.
- **Hub :** N'a pas à gérer les mots de passe ni les jetons bruts ; il consomme une « session » ou un contexte déjà établi (injecté par l'environnement ou par une couche d'authentification en amont).

### 8.5 Audit et traçabilité

- **Actions tracées (côté Cores) :** Demande de catalogue, demande de Mandat, lancement de Service, révocation. Le Hub peut envoyer des événements « vue Catalogue », « clic Ouvrir Service X » pour analytics gouvernés (si politique le prévoit).
- **Pas de log sensible dans le Hub :** Pas de données utilisateur, pas de Mandat en clair dans les logs client.

---

## 9. Synthèse — Tableau de bord conceptuel

| Dimension | Miyukini Central |
|-----------|-------------------|
| **Type** | Opérateur d'Interface (Strate 7) |
| **Rôle** | Vitrine du Registre d'Opérateurs ; catalogue, Mes Services, lancement gouverné |
| **Cores sollicités** | Master Butler (catalogue), StrongFather (Mandat), BondingBrother (médiation), WorrySentinel, Caring Nanny, Ever Buddy (état, sécurité, vie) |
| **Outils UI** | egui / eframe (Stack UI officielle) |
| **Ecrans** | Accueil, Catalogue, Fiche Service, Mes Services, Paramètres, À propos, Erreur |
| **Parcours clé** | Découverte → Fiche Service → Ouvrir → Mandat → Ouverture UI Opérateur |
| **Sécurité** | Niveaux 0–4, états T0–T4, Mandats StrongFather, messages gouvernés, pas d'exposition technique |

---

## 10. References

| Document | Lien |
|----------|------|
| **Glossaire** | [Miyukini Conceptual References - Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| **Opérateurs et terminologie** | [Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md) |
| **Pyramide** | [Pyramide Architecture Complete](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |
| **Stack UI egui/eframe** | [Miyukini - Stack UI egui eframe](../ux_ui/Miyukini%20-%20Stack%20UI%20egui%20eframe.md) |
| **Master Butler** | [docs/core/MasterButler](../../core/MasterButler/) |
| **Mandats et Équipes** | [Mandats et Equipes Operators](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md) |
| **Barrières techniques COG et point d'accès Central** | [Barrieres Techniques COG et Point Acces Central](./Miyukini%20Conceptual%20References%20-%20Barrieres%20Techniques%20COG%20et%20Point%20Acces%20Central.md) |

---

**Date de creation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de reference conceptuel — Miyukini Central (Hub de gestion des Services)
