# MiyuSQL — Index de navigation

## Contexte

**MiyuSQL** est le **kit d'outils (Toolkit)** de gestion de donnees en base de donnees (DB) de l'ecosysteme Miyukini. Il integre tous les outils de manipulation de donnees en base (requetes, transactions, cache) necessaires aux Operateurs pour executer des operations gouvernees sous autorite KindMother.

Ce dossier contient la documentation dediee au kit MiyuSQL et a ses outils individuels. L'identite et la composition du kit sont definies dans la **Documentation Fondatrice** ; le detail de chaque outil (ToolId, action, niveau de securite) est decrit dans la **Reference Outils**. Les contrats (integration, gouvernance, securite, bornage, tests), les dependances et le guide d'implementation completent la fondation.

**Strate :** 6 (Tools & Toolkits)  
**ToolkitId :** `toolkit.data.miyusql`  
**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## Structure de la documentation

### Fondation

| Document | Description |
|----------|-------------|
| [Documentation Fondatrice](./MiyuSQL%20-%20Documentation%20Fondatrice.md) | Identite, definition canonique, ToolkitId, liste des outils composants, gouvernance, niveau de securite, relation KindMother, alignement MIP |
| [Reference Outils](./MiyuSQL%20-%20Reference%20Outils.md) | Liste exhaustive des Tools du kit : ToolId, nom lisible, action, niveau de securite, capability_id |

### Contrats

| Document | Description |
|----------|-------------|
| [KindMother Integration Contract](./contracts/integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md) | Autorite KindMother, passage obligatoire par WriteIntent pour ecritures, execution sous autorite KindMother |
| [Tool Governance Compliance Contract](./contracts/governance/MiyuSQL%20-%20Tool%20Governance%20Compliance%20Contract.md) | Conformite Master Butler, declaration ToolkitId et ToolIds, catalogue capabilities |
| [Security and States Contract](./contracts/security/MiyuSQL%20-%20Security%20and%20States%20Contract.md) | Niveau de securite 2, etats autorises et interdits, alignement WorrySentinel et Caring Nanny |
| [Runtime Boundary Contract](./contracts/boundaries/MiyuSQL%20-%20Runtime%20Boundary%20Contract.md) | Bornage : ce que MiyuSQL ne fait jamais, frontieres avec les Cores, invariants de limite |
| [Unit Tests Contract](./contracts/testing/MiyuSQL%20-%20Unit%20Tests%20Contract.md) | Tests unitaires sur les Tools (requete, transaction, cache, schema), criteres de succes, non-destructivite |
| [Cycle Tests Contract](./contracts/testing/MiyuSQL%20-%20Cycle%20Tests%20Contract.md) | Tests de cycle MiyuSQL ; test chemin complet MiyukiniSQLtest (E2E) executable par MiyukiniAdmin |

### Dependances et implementation

| Document | Description |
|----------|-------------|
| [Dependencies Contract](./dependencies/MiyuSQL%20-%20Dependencies%20Contract.md) | Liste fermee des dependances (KindMother, Master Butler, BondingBrother, StrongFather, WorrySentinel, Caring Nanny, Kernel), ordre et contraintes |
| [Reference Implementation Guidelines](./implementation/MiyuSQL%20-%20Reference%20Implementation%20Guidelines.md) | Guide informatif d'implementation : traduction des contrats, interdictions, patterns, alignement MIP/MSCM |

---

## References

| Document | Lien |
|----------|------|
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| KindMother | [KindMother - Index](../../core/KindMother/_index.md) |
| MiyukiniAdmin - Cycle Tests Contract | [MiyukiniAdmin - Cycle Tests Contract](../../core/MiyukiniAdmin/contracts/testing/MiyukiniAdmin%20-%20Cycle%20Tests%20Contract.md) (test MiyuSQL Full Path / MiyukiniSQLtest) |
