# Miyukini Core System

> *"Miyukini is not an OS. It's the cog that makes digital systems work together."*

**Miyukini est un COG — Core-Orchestrated Governance Environment.** Un environnement de gouvernance orchestré par des cores, qui coordonne, sécurise et fait fonctionner des systèmes logiciels complets, du noyau jusqu'à l'utilisateur final.

---

## Table des matières

1. [Introduction](#1-introduction)
2. [Vision et philosophie](#2-vision-et-philosophie)
3. [Mécaniques générales](#3-mécaniques-générales)
4. [Environnements, fédération et sécurité](#4-environnements-fédération-et-sécurité)
5. [Méthodologie de développement](#5-méthodologie-de-développement)
6. [État actuel et roadmap](#6-état-actuel-et-roadmap)
7. [Gouvernance et sécurité](#7-gouvernance-et-sécurité)
8. [À qui s'adresse Miyukini](#8-à-qui-sadresse-miyukini)
9. [Documentation de référence](#9-documentation-de-référence)
10. [Licence](#10-licence)
11. [Conclusion](#11-conclusion)
12. [Log de rédaction](#12-log-de-rédaction)

---

## 1. Introduction

### Qu'est-ce que Miyukini

Miyukini Core System (MCS) est un **écosystème logiciel gouverné** conçu pour produire des applications autonomes, sécurisées structurellement, et capables de fonctionner dans des conditions de contrainte extrême (offline, ressources limitées, environnements isolés).

Miyukini n'est pas un framework. Ce n'est pas une bibliothèque. C'est un **environnement gouverné dans lequel des Opérateurs opèrent**.

La distinction est fondamentale : un framework fournit des outils que le développeur utilise comme bon lui semble. Miyukini fournit un **cadre non négociable** dans lequel les Opérateurs opèrent selon des règles strictes, des invariants vérifiables, et une gouvernance centralisée.

### Pourquoi Miyukini existe

Les architectures logicielles modernes reposent sur des hypothèses implicites : connectivité permanente, ressources cloud élastiques, services tiers toujours accessibles. Ces hypothèses excluent une part significative des cas d'usage réels.

Miyukini adopte la posture inverse : **la déconnexion est un état normal du système, pas une erreur à corriger**.

Résultat : systèmes qui démarrent sans réseau, fonctionnent sans cloud, dégradent proprement en isolation, restent administrables localement, et évoluent quand le réseau revient (sans reconstruction).

---

## 2. Vision et philosophie

### Systèmes autonomes

Un système Miyukini est **autonome** au sens strict : démarrable sans réseau, fonctionnel sans cloud, dégradé proprement en isolation, prévisible sans synchronisation, administrable localement, évolutif à la reconnexion. Cette autonomie est structurelle et vérifiable.

### Séparation stricte des responsabilités

- **Décision** (StrongFather) ≠ **Exécution** (Opérateurs, Outils) ≠ **Persistance** (KindMother). Aucun core n'empiète sur les autres.
- *« Un Outil fait, mais ne décide jamais. »*
- *« Un Mandat de Permission n'est pas une optimisation. C'est un acte de gouvernance délégué. »*

### Complexité par collaboration

> **Dans Miyukini, la complexité est gérée par la collaboration, pas par l'accumulation.**

Les Services sont portés par des Opérateurs seuls ou par des Équipes d'Opérateurs ; les Opérateurs orchestrent des Outils et des Kits d'Outils. La complexité ne s'empile pas — elle se répartit sous gouvernance explicite.

### Lois d'autonomie (LOI-1 à LOI-8)

Les **8 lois d'autonomie** sont des invariants architecturaux non négociables :

| Loi | Énoncé |
|-----|--------|
| **LOI-1** | Aucune dépendance externe critique à l'exécution |
| **LOI-2** | Le système accepte l'isolement comme état normal |
| **LOI-3** | L'état local est souverain |
| **LOI-4** | Pas de temps global requis |
| **LOI-5** | Le coût doit être proportionnel au hardware |
| **LOI-6** | L'autonomie n'empêche pas la fédération |
| **LOI-7** | La strate Cores est immuable — évolution par environnement |
| **LOI-8** | Migration = diplomatie entre environnements |

Question de conception : *« Est-ce que ça fonctionne encore si le système est seul, lent, et isolé ? »*

### Pyramide et Cores

L'écosystème est organisé en **strates** hiérarchiques ; la dépendance est strictement unidirectionnelle (de haut en bas).

```
┌─────────────────────────────────────────────────────────────┐
│ 🔧 STRATE 9 — MiyukiniAdmin (EXCEPTION)                      │
│    Opérateur Souverain d'administration                      │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟧 STRATE 8 — SERVICES                                       │
│    Capacités perçues par l'utilisateur (facturation, caisse, │
│    réservation…) — délivrées par les Opérateurs (Strate 7)   │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟩 STRATE 7 — OPÉRATEURS                                     │
│    Entités fonctionnelles gouvernées (exécutent des rôles)   │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟦 STRATE 6 — TOOLS & TOOLKITS                               │
│    Capacités exécutables gouvernées (Outils, Kits d'Outils)  │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟨 STRATE 5 — INTERFACES & ADAPTATION                        │
│    BondingBrother (médiation intentions ↔ Cores)             │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟥 STRATE 4 — CORES SYSTÈME                                  │
│    StrongFather · KindMother · Caring Nanny · Master Butler  │
│    Border Guard · Ever Buddy · WorrySentinel · TAMR · …       │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟪 STRATE 3 — INVARIANTS & CONTRATS                          │
│    Principes architecturaux                                   │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ ⚙️  KERNEL — Id · Logger · Clock · Config · Lifecycle         │
│    Substrat technique neutre (aucune logique métier)          │
└─────────────────────────────────────────────────────────────┘
                              ▲
┌─────────────────────────────────────────────────────────────┐
│ 🟫 STRATE 0 — HARDWARE & OS                                  │
│    Réalité physique                                           │
└─────────────────────────────────────────────────────────────┘
```

Les **Cores** (Strate 4) gouvernent ; ils ne décident pas à la place de l'exécution, ils n'exécutent jamais. BondingBrother (Strate 5) assure la médiation entre les intentions et les Cores.

---

## 3. Mécaniques générales

### Du besoin utilisateur au résultat

1. **Service** — Ce que l'utilisateur perçoit et consomme (ex. « facturation », « réservation », « caisse »).
2. **Opérateur(s)** — Entité(s) fonctionnelle(s) gouvernée(s) qui portent le service. Un service peut être délivré par un seul Opérateur ou par une **Équipe d'Opérateurs** (collaboration mandatée sous Contrat d'équipe et Mandat de Permission).
3. **Outils et Kits d'Outils** — Capacités exécutables que les Opérateurs appellent. Les Outils sont atomiques et sans autorité ; les Kits d'Outils sont des compositions officielles d'Outils, optimisées pour l'efficience.
4. **Gouvernance** — Toute action passe par BondingBrother ; les décisions (autoriser/refuser) par StrongFather ; les écritures par KindMother (WriteIntent). Master Butler tient le registre des capacités et permissions ; WorrySentinel gouverne les niveaux de sécurité et les états de confiance.

### Règle fondamentale

> **Dans Miyukini, les utilisateurs n'installent pas d'applications. Ils interagissent avec des Opérateurs gouvernés qui exécutent des rôles pour leur compte.**

Les Opérateurs, grâce à leur savoir-faire et leurs compétences, **orchestrent les Outils** — seuls ou en équipe — en fonction des besoins des services consommés par les utilisateurs.

---

## 4. Environnements, fédération et sécurité

### Environnements (COG) : identification et souveraineté

Un **environnement Miyukini (COG)** est une **entité souveraine, versionnée, isolée et identifiée de manière unique**. Ce n'est pas un simple runtime : c'est une instance de gouvernance.

| Propriété | Description |
|-----------|-------------|
| **Version complète des Cores** | Ensemble cohérent et figé de tous les Cores (Strate 4) |
| **ID d'environnement unique** | Identifiant généré à la création par le kernel |
| **Frontières strictes** | Limites claires entre l'environnement et l'extérieur |
| **Opérateurs assujettis** | Chaque Opérateur est lié à un environnement unique |

**Règle fondatrice (LOI-7)** : *« La strate Cores est immuable. Toute évolution se fait par la création d'un nouvel environnement complet. »* Pas de micro-patch, pas de hotfix sauvage — uniquement des environnements complets, auditable et versionnés.

**Identification des environnements** : selon le contexte, un COG peut être identifié par une **LSI** (Local Sovereign ID, générée localement), une **VID** (Verified ID, vérifiée par un registre global en contexte fédéré) ou une **WID** (Witnessed ID, attestée par échange indirect — clé USB, QR, signature). L'identité d'environnement est la base de toute relation inter-COG.

### Échanges entre environnements fédérés

L'autonomie n'empêche pas la fédération (LOI-6). Les échanges entre COG ne sont **jamais** une fusion de gouvernance : ils passent par une **visite gouvernée**.

- **COG Origine** — Atteste l'identité de l'utilisateur (émetteur du **Passeport Utilisateur**). Ne participe pas à l'exécution distante.
- **COG Hébergeur** — Souverain exécutif de la session. Vérifie le visiteur, accorde ou refuse l'accès, émet un **Visa de Connexion**, surveille et peut révoquer à tout moment.
- **Utilisateur Visiteur** — Citoyen dans son COG d'origine, visiteur gouverné dans le COG hôte. Agit uniquement via le Visa ; ne transporte aucun core, aucune logique, aucun état.
- **Bridge inter-COG** — Canal diplomatique (BondingBrother étendu). Transporte identités, intentions et autorisations. **Aucun pouvoir décisionnel, aucun état métier.**

> **Le bridge ne fait jamais confiance, il transporte.**

**Migration** (LOI-8) = diplomatie entre environnements : processus formel, contrat explicite, frontière contrôlée, traduction (pas copie brute). Acteurs : Border Guard (règles), BondingBrother (traduction), StrongFather (décision), KindMother (persistance), Ever Buddy (compatibilité).

### Logique de sécurité sous-jacente

- **Identité ≠ autorité** — Le Passeport prouve qui tu es et d'où tu viens ; il ne donne aucun droit. L'autorité reste au COG Hébergeur (Visa).
- **Un seul souverain par session** — Le COG Hébergeur est l'unique source de vérité de l'état pendant la visite. Aucun core n'est partagé ; aucun état n'est migré en direct.
- **Sécurité avant fluidité** — *« Un COG n'accueille jamais une gouvernance étrangère. Il n'accueille que des visiteurs, sous visa, dans un cadre qu'il définit seul. »*
- **Zero-trust** — Aucun appelant (environnement, utilisateur, Opérateur) n'est présumé valide ; toute intention est évaluée selon les politiques (WorrySentinel, StrongFather).

Cette logique s'applique aussi en interne : chaque environnement est souverain sur son périmètre ; la fédération repose sur des protocoles explicites (Passeport, Demande de Visite, Visa, Bridge), jamais sur une confiance implicite.

---

## 5. Méthodologie de développement

Le projet suit un **cycle strict**, de l'idée jusqu'à l'audit, sans court-circuit.

### Enchaînement des phases

1. **Idée** — Besoin ou fonctionnalité cible (ex. compta indépendants, caisse PoS, réservation, CMS).
2. **Analyse d'équivalents (PR)** — Étude des produits / logiciels existants du marché (Indy, Pennylane, logiciels caisse, moteurs forum, CMS boutique + réservation SaaS, etc.) pour en extraire les périmètres fonctionnels et les cas d'usage.
3. **Transcription conceptuelle dans la référence** — Rédaction ou mise à jour de documents **Miyukini Conceptual References — Équivalents …** (ex. Équivalents Comptabilité Indépendants, Équivalents PoS Logiciel Caisse, Équivalents Moteur Forum). La sémantique Miyukini (Outil, Kit d'Outils, Opérateur, Service, KindMother, StrongFather, etc.) est appliquée ; aucun code à ce stade.
4. **Documentation enrichie** — Rédaction de la **documentation fondatrice** des Tools/Toolkits ou Opérateurs : définition canonique, identifiants, liste d'outils, **contrats** (gouvernance, sécurité, intégration KindMother, tests), implémentation, alignement MIP. Planification et **bornage** de l'implémentation (périmètre, limites, livrables).
5. **Implémentation** — Développement selon le [protocole d'implémentation générale](docs/protocols/Miyukini%20Prompt%20Protocol%20-%20Implémentation%20générale.md) : planification → distribution des tâches → vérification, corrections et tests → gel et versionnement. Conformité MSCM/MIP.
6. **Test et audit** — Vérification des contrats, qualité, sécurité ; audit documentation si nécessaire.

### Principes

- **La référence conceptuelle précède le code.** On ne code pas un « équivalent Pennylane » sans avoir transposé le périmètre en termes Miyukini (Tools, Toolkits, Opérateurs, Services).
- **La documentation est normative.** Contrats, gouvernance et sécurité sont écrits et gelés avant ou en parallèle de l'implémentation.
- **Planification et bornage** évitent la dérive de périmètre et permettent un suivi explicite (y compris avec des agents IA en mode PLAN / Auto).

---

## 6. État actuel et roadmap

### Travail en cours : Kits d'Outils (Toolkits)

L'effort actuel se concentre sur les **Toolkits** pour couvrir le spectre des besoins le plus large possible. Chaque Toolkit est documenté (documentation fondatrice, contrats de gouvernance, référence outils, intégration KindMother, sécurité) et positionné dans la pyramide (Strate 6).

**Domaines couverts (extrait)** : données (MiyuSQL), identité (MiyuAuth), web (MiyuWeb), horloge (MiyuClock), contenu (MiyuCMS, MiyuMedia), widgets, boutique (MiyuStore), livraison (MiyuShipping), réservation (MiyuBooking), facturation SaaS (MiyuBilling), caisse et PoS (MiyuPosSales, MiyuPosInventory, MiyuPosAnalytics, MiyuPosLoyalty, MiyuPosKitchen, MiyuPosPayment), RH (MiyuHR), facturation métier (MiyuInvoice), comptabilité (MiyuComptaLedger, MiyuComptaReports, MiyuDeclarations), notes de frais (MiyuExpense), trésorerie (MiyuTreasury), etc.

L'objectif est d'avoir un **catalogue de capacités gouvernées** (Tools & Toolkits) stable et documenté, sur lequel s'appuyer pour la phase suivante.

### Phase suivante : Opérateurs

Une fois le spectre des Toolkits suffisamment couvert, le travail se déplacera vers les **Opérateurs** (Strate 7). Les Opérateurs, grâce à leur savoir-faire et leurs compétences, **orchestreront les Outils** — seuls ou en équipe — en fonction des besoins des **services** consommés par les utilisateurs. Les Opérateurs ne décident pas ; ils exécutent des rôles sous gouvernance (StrongFather, Mandats de Permission, Contrats d'équipe).

### Maturité conceptuelle

La pyramide, les Cores, les Lois d'autonomie, les contrats de sécurité et de gouvernance d'écosystème sont **stabilisés et documentés**. Le Kernel et les Cores du workspace sont en place ; l'outillage MIP (index structurel MSCM) est disponible.

---

## 7. Gouvernance et sécurité

- **Zero-trust** : aucun appelant présumé valide ; toute intention évaluée selon les politiques.
- **Niveaux de sécurité** (0–4) et **états de confiance** (T0–T4) : gouvernés par WorrySentinel ; dégradation progressive, pas de blocage brutal.
- **Offline-first** : WriteIntent acceptés localement ; réconciliation explicite à la reconnexion.
- **MiyukiniAdmin** : Opérateur Souverain (Strate 9), exception — installation, diagnostic, arbitrage, accès exceptionnel ; strictement encadré.

---

## 8. À qui s'adresse Miyukini

| Acteur | Besoin |
|--------|--------|
| **Architectes système** | Autonomie structurelle, sécurité par conception, traçabilité auditable, fonctionnement déterministe en isolation |
| **Développeurs d'Opérateurs** | Collectivités, événements sans réseau fiable, IoT/edge, contextes réglementés |
| **Décideurs techniques** | Projets long terme (5–10 ans), systèmes critiques, contrôle total non négociable |

**Miyukini n'est pas destiné** aux projets exigeant une mise en production immédiate sans comprendre l'architecture, aux applications temps réel critique (&lt;100 ms), aux équipes refusant les contraintes de gouvernance, ou aux contextes où la connectivité permanente et la dépendance cloud sont assumées sans exigence d'autonomie.

---

## 9. Documentation de référence

| Thème | Document principal |
|-------|--------------------|
| **Terminologie** | [Glossaire officiel](docs/reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| **Architecture** | [Pyramide Architecture Complete](docs/reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |
| **Autonomie** | [Lois Autonomie Système](docs/reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) |
| **COG** | [Definition COG](docs/reference/Miyukini%20Conceptual%20References%20-%20Definition%20COG.md) |
| **Souveraineté environnement** | [Souverainete Environnement](docs/reference/Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md) |
| **Connexion inter-COG** | [Connexion Inter-COG](docs/reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) |
| **Opérateurs** | [Operators et Terminologie](docs/reference/Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md) |
| **Tools et Toolkits** | [Tools et Toolkits](docs/reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| **Objectif final** | [Objectif Final](docs/reference/Miyukini%20Conceptual%20References%20-%20Objectif%20Final.md) |

**Kits d'Outils** : [docs/tools/_index.md](docs/tools/_index.md) — index des Toolkits documentés.  
**Protocoles** : [docs/protocols/](docs/protocols/) — MIP, écriture documentation conceptuelle, implémentation générale.  
**Cores** : [docs/core/](docs/core/) — par Core (StrongFather, KindMother, Master Butler, MiyukiniAdmin, etc.).  
**Stack UI (egui / eframe)** : [Miyukini - Stack UI egui eframe](docs/ux_ui/Miyukini%20-%20Stack%20UI%20egui%20eframe.md) — applications desktop et web en pur Rust (Hub, clients natifs).  
**Miyukini Central (Hub Services)** : [Miyukini Conceptual References - Miyukini Central Hub Services](docs/reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md) — hub de gestion des Services (catalogue, Mes Services, lancement, UI/UX, sécurité).

---

## 10. Licence

Miyukini est distribué sous une **politique de licence duale** :

- **Usage domestique / personnel** (personne physique, à des fins non commerciales) : **gratuit** — voir le fichier [LICENSE](LICENSE) à la racine du dépôt.
- **Usage par une société ou une collectivité** (entreprise, association, administration, établissement public, etc.) : **payant** — une **licence commerciale** est requise.

Détails et conditions : [Miyukini — Politique de licence](docs/legal/Miyukini%20-%20Politique%20de%20Licence.md).

---

## 11. Conclusion

Miyukini ne vise pas à être le plus rapide ni le plus flexible, mais **prévisible, traçable, autonome et structurellement sécurisé**. Il demande un investissement initial (architecture en strates, contraintes de gouvernance, invariants) en échange de garanties : fonctionnement déterministe en isolation, sécurité par conception, évolution sans rupture, traçabilité complète.

*« Miyukini n'est pas une bibliothèque. C'est un environnement gouverné dans lequel des Opérateurs opèrent. »*

---

## 12. Log de rédaction

**2026-01-30 — Refonte README racine**

- Ajout section **Licence** : politique duale (usage domestique gratuit, sociétés et collectivités payant) ; lien vers LICENSE et docs/legal/Miyukini - Politique de Licence.md.
- Suppression de l'arborescence du projet et de la cartographie des crates (aspects purement techniques).
- Ajout section **Environnements, fédération et sécurité** : identification des environnements (COG, LSI/VID/WID), échanges entre environnements fédérés (visite gouvernée, Passeport, Visa, Bridge), logique de sécurité sous-jacente (identité ≠ autorité, un seul souverain, zero-trust).
- Recentrage sur la **vision**, la **philosophie**, les **mécaniques générales** (Service → Opérateur(s) → Tools/Toolkits → Gouvernance).
- Ajout de la **méthodologie de développement** : Idée → Analyse PR équivalent → Transcription conceptuelle (référence) → Documentation enrichie (contrats, implémentation, sécurité, planification et bornage) → Implémentation → Test et audit.
- Ajout de l'**état actuel** : travail concentré sur les Toolkits pour couvrir le spectre des besoins ; **phase suivante** : Opérateurs qui orchestrent les Outils seuls ou en équipe selon les services consommés par les utilisateurs.
- Table des matières simplifiée ; documentation de référence conservée sous forme de liens, sans arborescence détaillée.

**2026-01-29 — Réorganisation README racine (version précédente)**

- Structure en 6 parties ; arborescence, cartographie crates, documentation avec mapping dossiers.

---

**Document** : README racine officiel  
**Dernière mise à jour** : 2026-01-30  
**Références** : Glossaire officiel, Pyramide Architecture Complete, Lois d'autonomie, Objectif Final, Tools et Toolkits
