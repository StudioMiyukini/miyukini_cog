# MiyuAuth â€” Index de navigation

## Contexte

**MiyuAuth** est le **kit d'outils (Toolkit)** d'identitÃ© utilisateur de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de rÃ©solution de rÃ´le (citoyen / visiteur / externe), d'attestation, de vÃ©rification Passeport Utilisateur et Visa de Connexion, alignÃ©s sur la Connexion Inter-COG et sur KindMother Identity & Cross-Domain Trust.

Ce dossier contient la documentation dÃ©diÃ©e au kit MiyuAuth et Ã  ses outils individuels. L'identitÃ© et la composition du kit sont dÃ©finies dans la **Documentation Fondatrice** ; le dÃ©tail de chaque outil (ToolId, action, niveau de sÃ©curitÃ©) est dÃ©crit dans la **Reference Outils**. Les contrats (intÃ©gration KindMother, gouvernance, sÃ©curitÃ©, bornage, tests), les dÃ©pendances et le guide d'implÃ©mentation complÃ¨tent la fondation.

**Strate :** 6 (Tools & Toolkits)  
**ToolkitId :** `toolkit.identity.miyauth`  
**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

Les invariants MiyuAuth utilisent des prÃ©fixes catÃ©goriels (BOUND, DEP, SEC, INV-KM-*, etc.) ; pour le format canonique des Cores, voir [Standardisation NumÃ©ration Invariants](..//..//miyukini-webway-system//reference//_index.md).

---

## Structure de la documentation

### Fondation

| Document | Description |
|----------|-------------|
| [Documentation Fondatrice](./MiyuAuth%20-%20Documentation%20Fondatrice.md) | IdentitÃ©, dÃ©finition canonique, ToolkitId, liste des outils composants, gouvernance, niveau de sÃ©curitÃ©, relation KindMother et Connexion Inter-COG, relation MiyuSQL (donnÃ©es identitÃ©, Passeport, Visa), alignement MIP |
| [Reference Outils](./MiyuAuth%20-%20Reference%20Outils.md) | Liste exhaustive des Tools du kit : ToolId, nom lisible, action, niveau de sÃ©curitÃ©, capability_id |

### Contrats

| Document | Description |
|----------|-------------|
| [KindMother Integration Contract](./contracts/integration/MiyuAuth%20-%20KindMother%20Integration%20Contract.md) | KindMother validateur unique de la confiance ; MiyuAuth exÃ©cute des capacitÃ©s sans dÃ©cider de la confiance |
| [Tool Governance Compliance Contract](./contracts/governance/MiyuAuth%20-%20Tool%20Governance%20Compliance%20Contract.md) | ConformitÃ© Master Butler, dÃ©claration ToolkitId et ToolIds, catalogue capabilities |
| [Security and States Contract](./contracts/security/MiyuAuth%20-%20Security%20and%20States%20Contract.md) | Niveau de sÃ©curitÃ© (2 ou 3), Ã©tats autorisÃ©s et interdits, alignement WorrySentinel et Caring Nanny |
| [Runtime Boundary Contract](./contracts/boundaries/MiyuAuth%20-%20Runtime%20Boundary%20Contract.md) | Bornage : ce que MiyuAuth ne fait jamais, frontiÃ¨res avec les Cores, invariants de limite |
| [Unit Tests Contract](./contracts/testing/MiyuAuth%20-%20Unit%20Tests%20Contract.md) | Tests unitaires sur les Tools (resolve, attest, verify, role), critÃ¨res de succÃ¨s, non-destructivitÃ© |
| [Cycle Tests Contract](./contracts/testing/MiyuAuth%20-%20Cycle%20Tests%20Contract.md) | Tests de cycle MiyuAuth ; chemin complet (rÃ©solution identitÃ© â†’ rÃ´le â†’ vÃ©rification Passeport/Visa) ; exÃ©cutable par MiyukiniAdmin |

### DÃ©pendances et implÃ©mentation

| Document | Description |
|----------|-------------|
| [Dependencies Contract](./dependencies/MiyuAuth%20-%20Dependencies%20Contract.md) | Liste fermÃ©e des dÃ©pendances (KindMother, Master Butler, BondingBrother, StrongFather, WorrySentinel, Caring Nanny, Kernel), ordre et contraintes |
| [Reference Implementation Guidelines](./implementation/MiyuAuth%20-%20Reference%20Implementation%20Guidelines.md) | Guide informatif d'implÃ©mentation : traduction des contrats, interdictions, patterns, alignement MIP/MSCM |

### Audit

| Document | Description |
|----------|-------------|
| [Audit Documentation](./MiyuAuth%20-%20Audit%20Documentation.md) | Audit de la documentation MiyuAuth au regard des rÃ©fÃ©rences (docs/reference) et du modÃ¨le MiyuSQL ; conformitÃ© et amÃ©liorations possibles |

---

## RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Connexion Inter-COG | [Miyukini Conceptual References - Connexion Inter-COG](..//..//miyukini-webway-system//reference//_index.md) |
| Security Levels | [Miyukini Conceptual References - Security Levels](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| KindMother - Identity & Cross-Domain Trust | [KindMother - Identity & Cross-Domain Trust Contract](..//..//cores//KindMother//contracts//authority//KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md) |
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| SouverainetÃ© Environnement | [Miyukini Conceptual References - SouverainetÃ© Environnement](..//..//miyukini-webway-system//reference//_index.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](..//..//miyukini-webway-system//reference//_index.md) |
| Standardisation NumÃ©ration Invariants | [Miyukini Conceptual References - Standardisation NumÃ©ration Invariants](..//..//miyukini-webway-system//reference//_index.md) |
| MiyukiniAdmin - Cycle Tests Contract | [MiyukiniAdmin - Cycle Tests Contract](..//..//admin//MiyukiniAdmin//contracts//testing//MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md) (test chemin complet MiyuAuth exÃ©cutable par MiyukiniAdmin) |



