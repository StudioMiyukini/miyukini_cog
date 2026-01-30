# MiyuAuth — Index de navigation

## Contexte

**MiyuAuth** est le **kit d'outils (Toolkit)** d'identité utilisateur de l'écosystème Miyukini. Il intègre les outils de résolution de rôle (citoyen / visiteur / externe), d'attestation, de vérification Passeport Utilisateur et Visa de Connexion, alignés sur la Connexion Inter-COG et sur KindMother Identity & Cross-Domain Trust.

Ce dossier contient la documentation dédiée au kit MiyuAuth et à ses outils individuels. L'identité et la composition du kit sont définies dans la **Documentation Fondatrice** ; le détail de chaque outil (ToolId, action, niveau de sécurité) est décrit dans la **Reference Outils**. Les contrats (intégration KindMother, gouvernance, sécurité, bornage, tests), les dépendances et le guide d'implémentation complètent la fondation.

**Strate :** 6 (Tools & Toolkits)  
**ToolkitId :** `toolkit.identity.miyauth`  
**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

Les invariants MiyuAuth utilisent des préfixes catégoriels (BOUND, DEP, SEC, INV-KM-*, etc.) ; pour le format canonique des Cores, voir [Standardisation Numération Invariants](../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md).

---

## Structure de la documentation

### Fondation

| Document | Description |
|----------|-------------|
| [Documentation Fondatrice](./MiyuAuth%20-%20Documentation%20Fondatrice.md) | Identité, définition canonique, ToolkitId, liste des outils composants, gouvernance, niveau de sécurité, relation KindMother et Connexion Inter-COG, relation MiyuSQL (données identité, Passeport, Visa), alignement MIP |
| [Reference Outils](./MiyuAuth%20-%20Reference%20Outils.md) | Liste exhaustive des Tools du kit : ToolId, nom lisible, action, niveau de sécurité, capability_id |

### Contrats

| Document | Description |
|----------|-------------|
| [KindMother Integration Contract](./contracts/integration/MiyuAuth%20-%20KindMother%20Integration%20Contract.md) | KindMother validateur unique de la confiance ; MiyuAuth exécute des capacités sans décider de la confiance |
| [Tool Governance Compliance Contract](./contracts/governance/MiyuAuth%20-%20Tool%20Governance%20Compliance%20Contract.md) | Conformité Master Butler, déclaration ToolkitId et ToolIds, catalogue capabilities |
| [Security and States Contract](./contracts/security/MiyuAuth%20-%20Security%20and%20States%20Contract.md) | Niveau de sécurité (2 ou 3), états autorisés et interdits, alignement WorrySentinel et Caring Nanny |
| [Runtime Boundary Contract](./contracts/boundaries/MiyuAuth%20-%20Runtime%20Boundary%20Contract.md) | Bornage : ce que MiyuAuth ne fait jamais, frontières avec les Cores, invariants de limite |
| [Unit Tests Contract](./contracts/testing/MiyuAuth%20-%20Unit%20Tests%20Contract.md) | Tests unitaires sur les Tools (resolve, attest, verify, role), critères de succès, non-destructivité |
| [Cycle Tests Contract](./contracts/testing/MiyuAuth%20-%20Cycle%20Tests%20Contract.md) | Tests de cycle MiyuAuth ; chemin complet (résolution identité → rôle → vérification Passeport/Visa) ; exécutable par MiyukiniAdmin |

### Dépendances et implémentation

| Document | Description |
|----------|-------------|
| [Dependencies Contract](./dependencies/MiyuAuth%20-%20Dependencies%20Contract.md) | Liste fermée des dépendances (KindMother, Master Butler, BondingBrother, StrongFather, WorrySentinel, Caring Nanny, Kernel), ordre et contraintes |
| [Reference Implementation Guidelines](./implementation/MiyuAuth%20-%20Reference%20Implementation%20Guidelines.md) | Guide informatif d'implémentation : traduction des contrats, interdictions, patterns, alignement MIP/MSCM |

### Audit

| Document | Description |
|----------|-------------|
| [Audit Documentation](./MiyuAuth%20-%20Audit%20Documentation.md) | Audit de la documentation MiyuAuth au regard des références (docs/reference) et du modèle MiyuSQL ; conformité et améliorations possibles |

---

## Références

| Document | Lien |
|----------|------|
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Connexion Inter-COG | [Miyukini Conceptual References - Connexion Inter-COG](../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) |
| Security Levels | [Miyukini Conceptual References - Security Levels](../../reference/Miyukini%20Conceptual%20References%20-%20Security%20Levels.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| KindMother - Identity & Cross-Domain Trust | [KindMother - Identity & Cross-Domain Trust Contract](../../core/KindMother/contracts/authority/KindMother%20-%20Identity%20%26%20Cross-Domain%20Trust%20Contract.md) |
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| Souveraineté Environnement | [Miyukini Conceptual References - Souveraineté Environnement](../../reference/Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](../../reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |
| Standardisation Numération Invariants | [Miyukini Conceptual References - Standardisation Numération Invariants](../../reference/Miyukini%20Conceptual%20References%20-%20Standardisation%20Numeration%20Invariants.md) |
| MiyukiniAdmin - Cycle Tests Contract | [MiyukiniAdmin - Cycle Tests Contract](../../core/MiyukiniAdmin/contracts/testing/MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md) (test chemin complet MiyuAuth exécutable par MiyukiniAdmin) |
