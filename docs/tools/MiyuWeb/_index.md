# MiyuWeb â€” Index de navigation

## Contexte

**MiyuWeb** est le **kit d'outils (Toolkit)** d'affichage de contenu web de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de rendu HTML, d'exÃ©cution et de compilation de scripts (JS/TypeScript), de service d'assets, de rÃ©solution de thÃ¨me et de layout, de validation de formulaires et de gestion d'Ã©vÃ©nements, alignÃ©s sur KindMother pour la persistance des templates et assets (via MiyuSQL). MiyuWeb opÃ¨re sur des **donnÃ©es fournies dans le flux** â€” il ne lit pas la base directement.

Ce dossier contient la documentation dÃ©diÃ©e au kit MiyuWeb et Ã  ses outils individuels. L'identitÃ© et la composition du kit sont dÃ©finies dans la **Documentation Fondatrice** ; le dÃ©tail de chaque outil (ToolId, action, niveau de sÃ©curitÃ©) est dÃ©crit dans la **Reference Outils**. Les contrats (intÃ©gration KindMother, gouvernance, sÃ©curitÃ©, bornage, tests), les dÃ©pendances et le guide d'implÃ©mentation complÃ¨tent la fondation.

**Strate :** 6 (Tools & Toolkits)  
**ToolkitId :** `toolkit.web.miyuweb`  
**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

Les invariants MiyuWeb utilisent des prÃ©fixes catÃ©goriels (BOUND, DEP, SEC, INV-KM-*, etc.) ; pour le format canonique des Cores, voir [Standardisation NumÃ©ration Invariants](..//..//miyukini-webway-system//reference//_index.md).

---

## Structure de la documentation

### Fondation

| Document | Description |
|----------|-------------|
| [Documentation Fondatrice](./MiyuWeb%20-%20Documentation%20Fondatrice.md) | IdentitÃ©, dÃ©finition canonique, ToolkitId, liste des outils composants, gouvernance, niveau de sÃ©curitÃ©, relation KindMother et MiyuSQL (templates/assets en flux), alignement MIP |
| [Reference Outils](./MiyuWeb%20-%20Reference%20Outils.md) | Liste exhaustive des Tools du kit : ToolId, nom lisible, action, niveau de sÃ©curitÃ©, capability_id |

### Contrats

| Document | Description |
|----------|-------------|
| [KindMother Integration Contract](./contracts/integration/MiyuWeb%20-%20KindMother%20Integration%20Contract.md) | KindMother autoritÃ© sur les donnÃ©es (templates/assets) ; MiyuSQL exÃ©cution persistance ; MiyuWeb reÃ§oit les donnÃ©es dans le flux |
| [Tool Governance Compliance Contract](./contracts/governance/MiyuWeb%20-%20Tool%20Governance%20Compliance%20Contract.md) | ConformitÃ© Master Butler, dÃ©claration ToolkitId et ToolIds, catalogue capabilities |
| [Security and States Contract](./contracts/security/MiyuWeb%20-%20Security%20and%20States%20Contract.md) | Niveau de sÃ©curitÃ© du Toolkit, Ã©tats autorisÃ©s et interdits, alignement WorrySentinel et Caring Nanny |
| [Runtime Boundary Contract](./contracts/boundaries/MiyuWeb%20-%20Runtime%20Boundary%20Contract.md) | Bornage : ce que MiyuWeb ne fait jamais, frontiÃ¨res avec les Cores, invariants de limite |
| [Unit Tests Contract](./contracts/testing/MiyuWeb%20-%20Unit%20Tests%20Contract.md) | Tests unitaires sur les Tools (render, script.execute, script.compile, asset.serve, theme.resolve, etc.), critÃ¨res de succÃ¨s, non-destructivitÃ© |
| [Cycle Tests Contract](./contracts/testing/MiyuWeb%20-%20Cycle%20Tests%20Contract.md) | Tests de cycle MiyuWeb ; chemin complet (rÃ©solution thÃ¨me â†’ chargement template â†’ rendu â†’ formulaire / Ã©vÃ©nement) ; exÃ©cutable par MiyukiniAdmin |

### DÃ©pendances et implÃ©mentation

| Document | Description |
|----------|-------------|
| [Dependencies Contract](./dependencies/MiyuWeb%20-%20Dependencies%20Contract.md) | Liste fermÃ©e des dÃ©pendances (KindMother, Master Butler, BondingBrother, StrongFather, WorrySentinel, Caring Nanny, Ever Buddy, Kernel), relation indirecte MiyuSQL (donnÃ©es en flux), ordre et contraintes |
| [Reference Implementation Guidelines](./implementation/MiyuWeb%20-%20Reference%20Implementation%20Guidelines.md) | Guide informatif d'implÃ©mentation : traduction des contrats, interdictions, patterns (sanitization, CSP), alignement MIP/MSCM |

---

## RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Security Levels | [Miyukini Conceptual References - Security Levels](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| KindMother | [KindMother - Index](..//..//cores//KindMother//_index.md) |
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](..//..//miyukini-webway-system//reference//_index.md) |
| Standardisation NumÃ©ration Invariants | [Miyukini Conceptual References - Standardisation NumÃ©ration Invariants](..//..//miyukini-webway-system//reference//_index.md) |
| MiyukiniAdmin - Cycle Tests Contract | [MiyukiniAdmin - Cycle Tests Contract](..//..//admin//MiyukiniAdmin//contracts//testing//MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md) (test chemin complet MiyuWeb exÃ©cutable par MiyukiniAdmin) |



