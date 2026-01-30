# MiyuWeb — Index de navigation

## Contexte

**MiyuWeb** est le **kit d'outils (Toolkit)** d'affichage de contenu web de l'écosystème Miyukini. Il intègre les outils de rendu HTML, d'exécution et de compilation de scripts (JS/TypeScript), de service d'assets, de résolution de thème et de layout, de validation de formulaires et de gestion d'événements, alignés sur KindMother pour la persistance des templates et assets (via MiyuSQL). MiyuWeb opère sur des **données fournies dans le flux** — il ne lit pas la base directement.

Ce dossier contient la documentation dédiée au kit MiyuWeb et à ses outils individuels. L'identité et la composition du kit sont définies dans la **Documentation Fondatrice** ; le détail de chaque outil (ToolId, action, niveau de sécurité) est décrit dans la **Reference Outils**. Les contrats (intégration KindMother, gouvernance, sécurité, bornage, tests), les dépendances et le guide d'implémentation complètent la fondation.

**Strate :** 6 (Tools & Toolkits)  
**ToolkitId :** `toolkit.web.miyuweb`  
**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

Les invariants MiyuWeb utilisent des préfixes catégoriels (BOUND, DEP, SEC, INV-KM-*, etc.) ; pour le format canonique des Cores, voir [Standardisation Numération Invariants](../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md).

---

## Structure de la documentation

### Fondation

| Document | Description |
|----------|-------------|
| [Documentation Fondatrice](./MiyuWeb%20-%20Documentation%20Fondatrice.md) | Identité, définition canonique, ToolkitId, liste des outils composants, gouvernance, niveau de sécurité, relation KindMother et MiyuSQL (templates/assets en flux), alignement MIP |
| [Reference Outils](./MiyuWeb%20-%20Reference%20Outils.md) | Liste exhaustive des Tools du kit : ToolId, nom lisible, action, niveau de sécurité, capability_id |

### Contrats

| Document | Description |
|----------|-------------|
| [KindMother Integration Contract](./contracts/integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) | KindMother autorité sur les données (templates/assets) ; MiyuSQL exécution persistance ; MiyuWeb reçoit les données dans le flux |
| [Tool Governance Compliance Contract](./contracts/governance/MiyuWeb%20-%20Tool%20Governance%20Compliance%20Contract.md) | Conformité Master Butler, déclaration ToolkitId et ToolIds, catalogue capabilities |
| [Security and States Contract](./contracts/security/MiyuWeb%20-%20Security%20and%20States%20Contract.md) | Niveau de sécurité du Toolkit, états autorisés et interdits, alignement WorrySentinel et Caring Nanny |
| [Runtime Boundary Contract](./contracts/boundaries/MiyuWeb%20-%20Runtime%20Boundary%20Contract.md) | Bornage : ce que MiyuWeb ne fait jamais, frontières avec les Cores, invariants de limite |
| [Unit Tests Contract](./contracts/testing/MiyuWeb%20-%20Unit%20Tests%20Contract.md) | Tests unitaires sur les Tools (render, script.execute, script.compile, asset.serve, theme.resolve, etc.), critères de succès, non-destructivité |
| [Cycle Tests Contract](./contracts/testing/MiyuWeb%20-%20Cycle%20Tests%20Contract.md) | Tests de cycle MiyuWeb ; chemin complet (résolution thème → chargement template → rendu → formulaire / événement) ; exécutable par MiyukiniAdmin |

### Dépendances et implémentation

| Document | Description |
|----------|-------------|
| [Dependencies Contract](./dependencies/MiyuWeb%20-%20Dependencies%20Contract.md) | Liste fermée des dépendances (KindMother, Master Butler, BondingBrother, StrongFather, WorrySentinel, Caring Nanny, Ever Buddy, Kernel), relation indirecte MiyuSQL (données en flux), ordre et contraintes |
| [Reference Implementation Guidelines](./implementation/MiyuWeb%20-%20Reference%20Implementation%20Guidelines.md) | Guide informatif d'implémentation : traduction des contrats, interdictions, patterns (sanitization, CSP), alignement MIP/MSCM |

---

## Références

| Document | Lien |
|----------|------|
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Security Levels | [Miyukini Conceptual References - Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| KindMother | [KindMother - Index](../../core/KindMother/_index.md) |
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](../../reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |
| Standardisation Numération Invariants | [Miyukini Conceptual References - Standardisation Numération Invariants](../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md) |
| MiyukiniAdmin - Cycle Tests Contract | [MiyukiniAdmin - Cycle Tests Contract](../../core/MiyukiniAdmin/contracts/testing/MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md) (test chemin complet MiyuWeb exécutable par MiyukiniAdmin) |
