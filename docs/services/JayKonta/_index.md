# JayKonta - Service comptabilite multi-echelle

## Contexte

JayKonta est le service COG unifie du domaine comptabilite et budget.
Il couvre deux points d'entree :
- JayBudget (Purse) pour le personnel
- JayKonta (Account) pour l'entreprise

## Documentation principale

| Document | Role |
|----------|------|
| [JayKonta - Document Fondateur](./JayKonta%20-%20Document%20Fondateur.md) | Vision, positionnement, securite, integration. |
| [JayKonta - Documentation Enrichie](./JayKonta%20-%20Documentation%20Enrichie.md) | Architecture de service enrichie, capacites, gouvernance, UX cible. |
| [JayKonta - Contrats Service Operateurs et Toolkits](./JayKonta%20-%20Contrats%20Service%20Operateurs%20et%20Toolkits.md) | Contrats normatifs CK-SVC, CK-OP, CK-TK, CK-INT, CK-SEC, CK-AUD. |
| [JayKonta - Bornage Implementation](./JayKonta%20-%20Bornage%20Implementation.md) | Perimetre phase 1/2/3, in scope, hors scope, criteres fin de phase. |
| [JayKonta - Plan Implementation](./JayKonta%20-%20Plan%20Implementation.md) | Plan de delivery par phases et taches. |
| [JayKonta - Analyse PR Concurrence Web](./JayKonta%20-%20Analyse%20PR%20Concurrence%20Web.md) | Benchmark concurrentiel web et adaptations COG. |
| [**JayKonta - Mocks UI et Guide Implementation**](./JayKonta%20-%20Mocks%20UI%20et%20Guide%20Implementation.md) | **[NOUVEAU]** Mocks ASCII tous ecrans (Purse P1-P6, Account A1-A7), schema SQL KindMother complet, types Rust, composants Dioxus, guide implementation 6 phases (42.5j). Debloque Phase 2.2 du Parcours Developpement. |
| [JayKonta - Interfaces Inter-Services Futures](./reference/JayKonta%20-%20Interfaces%20Inter-Services%20Futures.md) | Guide d'implementation des interfaces futures: enveloppe, payloads, erreurs, idempotence, audit et versioning. |
| [Reference](./reference/_index.md) | Niveaux securite, points d'entree, integration. |
| [Publics](./publics/_index.md) | Purse et Account : analyses, parcours, operateurs/toolkits. |

## Voir aussi

- [Miyukini Festival Service](../JayFestival/JayFestival%20-%20Document%20Fondateur.md)
- [JayRDV](../JayRDV/JayRDV%20-%20Document%20Fondateur.md)
- [Politique de residence des donnees sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md)
