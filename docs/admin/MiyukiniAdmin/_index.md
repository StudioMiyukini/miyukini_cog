# MiyukiniAdmin â€” Index de Navigation

## Contexte

MiyukiniAdmin est l'**Operateur Souverain** (Strate 9) de l'ecosysteme Miyukini Core System. Il constitue une **exception volontaire** a la logique Operateur standard : une console root autonome, non reutilisable, qui observe, installe, arbitre, mais ne vit pas dans le flux normal.

MiyukiniAdmin represente la **console d'administration** du systeme : il connait l'etat de la maison, il peut intervenir en cas de crise, il supervise les metriques â€” sans jamais etre un produit metier ni une API publique. Il dispose de **capacites internes propres** et d'un **systeme d'auth dedie** (compte admin, MFA, RBAC) ; il ne consomme pas d'Outil ni de Kit d'Outils (Strate 6). Il est l'**unique Operateur Souverain** (Strate 9).

**Strate :** 9 (Operateur Souverain - Exception)  
**Role :** Console root d'administration  
**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## Question fondamentale

> **"Comment superviser, installer et arbitrer l'ecosysteme Miyukini de maniere souveraine et tracable ?"**

Cette question se decline en :
- Comment installer et bootstrapper un environnement Miyukini ?
- Comment observer l'etat du systeme sans le modifier implicitement ?
- Comment intervenir en cas de crise avec traÃ§abilite complete ?
- Comment gerer les niveaux de securite de maniere explicite ?

---

## Structure de la documentation

### Foundation

Documents fondateurs definissant l'identite et le role de MiyukiniAdmin.

| Document | Description |
|----------|-------------|
| [Documentation Fondatrice](./foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md) | Definition conceptuelle, role, positionnement, invariants fondamentaux |
| [Auth and Permissions Overview](./foundation/MiyukiniAdmin%20-%20Auth%20and%20Permissions%20Overview.md) | Vue d'ensemble auth, permissions, premier demarrage, roles et capacites |
| [Installation & Bootstrap Guide](./foundation/MiyukiniAdmin%20-%20Installation%20&%20Bootstrap%20Guide.md) | Guide d'installation et bootstrap (dependance circulaire, EIP) |

---

### Architecture

Documentation architecturale.

| Document | Description |
|----------|-------------|
| [Architecture & Flows](./architecture/MiyukiniAdmin%20-%20Architecture%20&%20Flows.md) | Architecture conceptuelle, composants, flux d'administration |
| [Core Interaction Contract](./architecture/MiyukiniAdmin%20-%20Core%20Interaction%20Contract.md) | Modele d'interaction avec les autres cores |

---

### Contracts

Contrats normatifs et non negociables.

#### Monitoring

| Document | Description |
|----------|-------------|
| [Consumption Metrics Contract](./contracts/monitoring/MiyukiniAdmin%20-%20Consumption%20Metrics%20Contract.md) | Metriques de consommation (CPU, RAM, reseau, disque) |
| [DB Metrics Contract](./contracts/monitoring/MiyukiniAdmin%20-%20DB%20Metrics%20Contract.md) | Metriques DB (requetes, latence, pool, sante SQL engine) |

#### Testing

| Document | Description |
|----------|-------------|
| [Cycle Tests Contract](./contracts/testing/MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md) | Tests de cycle (performance, latence, montee en charge) |
| [Unit Tests Contract](./contracts/testing/MiyukiniAdmin%20-%20Unit%20Tests%20Contract.md) | Tests unitaires (coherence DB, conformite contractuelle) |
| [Module Testing and Lifecycle Contract](./contracts/testing/MiyukiniAdmin%20-%20Module%20Testing%20and%20Lifecycle%20Contract.md) | Tests des modules (Kits d'outils, Operateurs, Equipes, Services), cellule Admin, integrite TAMR, cycle de vie (add, lock/unlock, delete) |

#### Database

| Document | Description |
|----------|-------------|
| [DB Operations Contract](./contracts/database/MiyukiniAdmin%20-%20DB%20Operations%20Contract.md) | Manipulation DB (via KindMother), scripts de migration |
| [Emergency DB Access Contract](./contracts/database/MiyukiniAdmin%20-%20Emergency%20DB%20Access%20Contract.md) | Acces DB direct (mode recovery) |
| [Backup Restore Contract](./contracts/database/MiyukiniAdmin%20-%20Backup%20Restore%20Contract.md) | Sauvegarde et restauration de la base |

#### Security

| Document | Description |
|----------|-------------|
| [Auth and First-Boot Contract](./contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md) | Auth MiyukiniAdmin, premier demarrage, environnement vierge, verrou StrongFather, Futur Admin, compte admin |
| [Authentication Contract](./contracts/security/MiyukiniAdmin%20-%20Authentication%20Contract.md) | Login, MFA, session, mot de passe, rate limiting, stockage secrets, audit auth |
| [Permission Contract](./contracts/security/MiyukiniAdmin%20-%20Permission%20Contract.md) | RBAC MiyukiniAdmin : roles (Admin, Recovery, Audit), capacites, matrice role â†’ capacites |
| [Security Level Management Contract](./contracts/security/MiyukiniAdmin%20-%20Security%20Level%20Management%20Contract.md) | Gestion niveaux securite 0-4 |
| [Threat Model Contract](./contracts/security/MiyukiniAdmin%20-%20Threat%20Model%20Contract.md) | Modele de menaces |

#### Governance

| Document | Description |
|----------|-------------|
| [Invariants & Guarantees](./contracts/governance/MiyukiniAdmin%20-%20Invariants%20&%20Guarantees.md) | Catalogue consolide des invariants INV-MA-1 a INV-MA-10 |
| [Violations & Anti-Patterns](./contracts/governance/MiyukiniAdmin%20-%20Violations%20&%20Anti-Patterns.md) | Violations cataloguees, anti-patterns |

#### Integration

| Document | Description |
|----------|-------------|
| [BondingBrother Integration Contract](./contracts/integration/MiyukiniAdmin%20-%20BondingBrother%20Integration%20Contract.md) | Point d'acces exclusif vers les cores |
| [StrongFather Integration Contract](./contracts/integration/MiyukiniAdmin%20-%20StrongFather%20Integration%20Contract.md) | Validation des actions administratives |
| [KindMother Integration Contract](./contracts/integration/MiyukiniAdmin%20-%20KindMother%20Integration%20Contract.md) | Acces controle aux donnees |
| [CaringNanny Integration Contract](./contracts/integration/MiyukiniAdmin%20-%20CaringNanny%20Integration%20Contract.md) | Observation de l'etat systeme |
| [LogisticsSteward Integration Contract](./contracts/integration/MiyukiniAdmin%20-%20LogisticsSteward%20Integration%20Contract.md) | Gouvernance des ressources et priorites |

---

### UI

Documentation de l'interface utilisateur (inspiration PHPMyAdmin).

| Document | Description |
|----------|-------------|
| [UI Design Philosophy](./ui/MiyukiniAdmin%20-%20UI%20Design%20Philosophy.md) | Philosophie UI (console root, non B2C) |
| [Dashboard & Metrics Display](./ui/MiyukiniAdmin%20-%20Dashboard%20&%20Metrics%20Display.md) | Dashboard metriques (inspiration PHPMyAdmin) |
| [Dashboard HyperUI Implementation](./ui/MiyukiniAdmin%20-%20Dashboard%20HyperUI%20Implementation.md) | Composants HyperUI copies, structure dashboard, bornage |
| [Organisation Pages et UX DB](./ui/MiyukiniAdmin%20-%20Organisation%20Pages%20et%20UX%20DB.md) | Besoins, arborescence, routes, parcours utilisateur DB |
| [DB Management Interface](./ui/MiyukiniAdmin%20-%20DB%20Management%20Interface.md) | Interface manipulation DB |
| [Affichage Dynamique et Metriques](./ui/MiyukiniAdmin%20-%20Affichage%20Dynamique%20et%20Metriques.md) | Strategie technique affichage dynamique (polling, SSE, metriques, logs) |
| [Security Control Panel](./ui/MiyukiniAdmin%20-%20Security%20Control%20Panel.md) | Panneau de controle securite |

---

### Implementation

Guides d'implementation.

| Document | Description |
|----------|-------------|
| [Reference Implementation Guidelines](./implementation/MiyukiniAdmin%20-%20Reference%20Implementation%20Guidelines.md) | Guidelines d'implementation de reference |
| [Implementation Security and Controls](./implementation/MiyukiniAdmin%20-%20Implementation%20Security%20and%20Controls.md) | Implementation controles et securite (etat environnement, verrou, reponse securitaire, recovery auto, auth, RBAC) |

---

### Operations

Documentation operationnelle (serveur, deploiement).

| Document | Description |
|----------|-------------|
| [Recovery Procedures](./operations/MiyukiniAdmin%20-%20Recovery%20Procedures.md) | Procedures de recuperation |
| [Serveur HTTP HTTPS](./operations/MiyukiniAdmin%20-%20Serveur%20HTTP%20HTTPS.md) | Configuration du serveur HTTP et option HTTPS pour affichage securise |

---

### Reference

Documentation de reference et exemples. Les capacites de l'interface sont explicitees et liees aux documents de `docs/reference`.

| Document | Description |
|----------|-------------|
| [Capacites et Reference](./reference/MiyukiniAdmin%20-%20Capacites%20et%20Reference.md) | Capacites livrees et liens vers docs/reference (MiyukiniAdmin Status, Glossaire, Security Levels, etc.) |
| [Pages et Outils Reference Supabase](./reference/MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md) | Correspondance Supabase / MiyukiniAdmin (pages, outils, sujets SQL/DB) |
| [Reference SQL et DB](./reference/MiyukiniAdmin%20-%20Reference%20SQL%20et%20DB.md) | Reference technique SQL et DB (schemas, tables, types, requetes, Realtime, roles) ; service hors-bord, pas de RLS |
| [Gestion DB type Supabase](./reference/MiyukiniAdmin%20-%20Gestion%20DB%20type%20Supabase.md) | Capacites Supabase transposees en COG, migrations, backups, implÃ©mentation |
| [Vocabulary & Glossary](./reference/MiyukiniAdmin%20-%20Vocabulary%20&%20Glossary.md) | Vocabulaire canonique de MiyukiniAdmin |
| [FAQ & Common Questions](_index.md) | Questions frequentes |
| [Examples & Use Cases](_index.md) | Exemples et cas d'usage |

---

## Invariants cles

| Invariant | Description |
|-----------|-------------|
| **INV-MA-1** | Aucune dependance vers MiyukiniAdmin par un autre Operateur |
| **INV-MA-2** | Aucune consommation d'Outil ou Kit d'Outils |
| **INV-MA-3** | Aucune API publique exposee |
| **INV-MA-4** | Toujours via BondingBrother pour acceder aux cores |
| **INV-MA-5** | Toute action est tracable, horodatee, justifiee, auditable |
| **INV-MA-6** | Ecriture DB directe uniquement en mode recovery (conditions cumulatives) |
| **INV-MA-7** | UI propre, isolee, non reutilisable |
| **INV-MA-8** | Logique metier administrative uniquement (pas de logique Operateur metier) |
| **INV-MA-9** | Backend et frontend internes complets |
| **INV-MA-10** | Jamais silencieux, jamais implicite |

---

## Interdictions

| Code | Interdiction |
|------|--------------|
| **INTERD-MA-1** | MiyukiniAdmin ne peut pas etre importe par un autre Operateur |
| **INTERD-MA-2** | MiyukiniAdmin ne peut pas consommer d'Outils ou Kits d'Outils |
| **INTERD-MA-3** | MiyukiniAdmin ne peut pas exposer d'API publique |
| **INTERD-MA-4** | MiyukiniAdmin ne peut pas etre embarque dans un Operateur client |
| **INTERD-MA-5** | MiyukiniAdmin ne peut pas contenir de logique metier applicative |
| **INTERD-MA-6** | MiyukiniAdmin ne peut pas partager ses composants UI |
| **INTERD-MA-7** | MiyukiniAdmin ne peut pas bypasser BondingBrother |
| **INTERD-MA-8** | MiyukiniAdmin ne peut pas effectuer d'action implicite ou silencieuse |

---

## Perimetre fonctionnel

| Domaine | Fonctions |
|---------|-----------|
| **Auth & Premier demarrage** | Detection environnement vierge/initialise, verrou StrongFather bootstrap, Futur Admin, parcours installation, creation compte admin, systeme d'auth propre (voir [Auth and First-Boot Contract](./contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)) |
| **Installation & Bootstrap** | Installation complete, verification hardware/OS, initialisation kernel, generation identites (EIP) |
| **Monitoring & Metriques** | Lecture passive metriques, traces kernel, statistiques decisions, etats Operateurs, sante DB |
| **Tests Techniques** | Tests performance, latence, montee en charge, coherence DB, conformite contractuelle ; page Tests avec flux de test des cores |
| **Tests des modules et cycle de vie** | Tests Kits d'outils, Operateurs, Equipes, Services via manifeste embarquÃ© ; cellule Admin ; integrite TAMR ; ajout, verrouillage/deverrouillage, suppression de modules (voir [Module Testing and Lifecycle Contract](./contracts/testing/MiyukiniAdmin%20-%20Module%20Testing%20and%20Lifecycle%20Contract.md)) |
| **Securite & Arbitrage** | Lecture etat WorrySentinel, changement niveau securite, modes de degradation, isolation modules |
| **Acces aux Donnees** | Via KindMother sous autorite StrongFather ; CRUD tables et manipulation des donnees (liste, lecture, creation, mise a jour, suppression) |
| **Recovery Exceptionnel** | Ecriture DB directe (conditions cumulatives strictes) |
| **Serveur** | Interface servie en HTTP ; option HTTPS pour affichage securise (voir [Serveur HTTP HTTPS](./operations/MiyukiniAdmin%20-%20Serveur%20HTTP%20HTTPS.md)) |

---

## Relations avec les Cores

| Core | Relation |
|------|----------|
| **BondingBrother** | Point d'acces exclusif â€” toute interaction passe par BondingBrother |
| **StrongFather** | Validation des actions administratives, decisions sur interventions |
| **KindMother** | Acces controle aux donnees, validation operations maintenance |
| **CaringNanny** | Exposition metriques systeme, etats Operateurs, sante globale |
| **WorrySentinel** | Lecture etat securite, changement niveaux, modes degradation |
| **Master Butler** | Decouverte des modules (Kits d'outils, Operateurs, Equipes, Services) ; enregistrement/retrait lors du cycle de vie des modules |
| **TAMR** | Champ d'action integrite â€” verification d'integrite des modules en collaboration avec MiyukiniAdmin |
| **LogisticsSteward** | Gouvernance des ressources â€” MiyukiniAdmin peut demander priorites maximales, soumis a gouvernance globale sauf protocole d'exception |

### Diagramme de relations

```mermaid
graph TB
    subgraph Strate9[Strate 9 - Operateur Souverain]
        MA[MiyukiniAdmin<br/>Console Root]
    end

    subgraph Strate4[Strate 4 - Cores Systeme]
        SF[StrongFather<br/>Decision]
        KM[KindMother<br/>Persistance]
        CN[CaringNanny<br/>Etat]
        WS[WorrySentinel<br/>Securite]
        BB[BondingBrother*<br/>Mediation]
    end

> **Note :** *BondingBrother est classÃ© avec les Cores mais conserve sa fonction de liaison

    subgraph Strate3[Strate 3 - Gouvernance Ressources]
        LS[LogisticsSteward<br/>Ressources]
    end

    MA -->|"acces exclusif"| BB
    BB -->|"validation actions"| SF
    BB -->|"acces donnees"| KM
    BB -->|"metriques systeme"| CN
    BB -->|"etat securite"| WS
    BB -->|"demande priorites"| LS
    LS -->|"gouvernance ressources"| MA
```

---

## Conformite aux Lois d'Autonomie Systeme

MiyukiniAdmin est **conforme** aux [Lois d'Autonomie Systeme](..//..//miyukini-webway-system//reference//_index.md) :

| Loi | Conformite | Note |
|-----|------------|------|
| **LOI-1** | âœ… | Peut fonctionner offline pour monitoring local |
| **LOI-2** | âœ… | Reconnait l'isolement comme etat normal |
| **LOI-3** | âœ… | Auto-suffisant fonctionnellement |
| **LOI-5** | âœ… | Compatible hardware faible (UI legere) |
| **LOI-6** | âœ… | Controlle les echanges federes |

---

## Position dans la Pyramide Miyukini

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ STRATE 9 â€” MiyukiniAdmin (EXCEPTION)     â”‚ â† Vous etes ici
â”‚ Operateur Souverain d'administration     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ STRATE 7 â€” Operateurs                    â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ STRATE 6 â€” Outils & Kits d'Outils        â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ STRATE 5 â€” Interfaces & Adaptation       â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ STRATE 4 â€” Cores Systeme                 â”‚
â”‚         (incluant BondingBrother*)       â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜

> **Note :** *BondingBrother est classÃ© avec les Cores (strate 4) en raison de son importance critique, mais conserve sa fonction de passerelle (strate 5). Tous les Cores dÃ©pendent de lui.
```

**MiyukiniAdmin est au-dessus de la pyramide, pas dedans.**

---

## Phrase fondatrice

> **MiyukiniAdmin est la console root de l'ecosysteme Miyukini : un Operateur Souverain autonome qui observe, installe et arbitre le systeme de maniere souveraine, tracable et explicite, sans jamais etre un produit metier ni une API publique.**

---

## Documents de reference

- [MiyukiniAdmin - Auth and First-Boot Contract](./contracts/security/MiyukiniAdmin%20-%20Auth%20and%20First-Boot%20Contract.md)
- [MiyukiniAdmin - Environment Identity Protocol EIP](..//..//contrats//MiyukiniAdmin%20-%20Environment%20Identity%20Protocol%20EIP.md)
- [Miyukini Conceptual References - MiyukiniAdmin Status](..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Security Levels](..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Pyramide Architecture Complete](..//..//miyukini-webway-system//reference//_index.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//miyukini-webway-system//reference//_index.md)
- [BondingBrother - Documentation Fondatrice](..//..//cores//BondingBrother//foundation//BondingBrother%20-%20Documentation%20Fondatrice.md)
- [StrongFather - Documentation Fondatrice](..//..//cores//StrongFather//foundation//StrongFather%20-%20Documentation%20Fondatrice.md)
- [KindMother - Documentation Fondatrice](..//..//cores//KindMother//foundation//KindMother%20-%20Documentation%20Fondatrice.md)
- [CaringNanny - Documentation Fondatrice](..//..//cores//CaringNanny//foundation//Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- [WorrySentinel - Documentation Fondatrice](..//..//cores//WorrySentinel//foundation//WorrySentinel%20-%20Documentation%20Fondatrice.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Document de reference




