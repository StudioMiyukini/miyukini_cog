# Miyukini Protocol — Écriture enrichie des Toolkits

**Version :** 1.0  
**Statut :** Normatif — Processus contrôlé  
**Date :** 2026-01-30  

---

## 1. Contexte

Ce protocole définit les **règles et le cycle d'écriture enrichie** de la documentation des Kits d'Outils (Toolkits) dans l'écosystème Miyukini. Il s'applique à tout nouveau Toolkit ou à toute mise à jour majeure d'un Toolkit existant.

**Références fondatrices :**

- [Miyukini Conceptual References - Tools et Toolkits](../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) : définitions canoniques Outil, Kit d'Outils
- [Miyukini Conceptual References - Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) : terminologie officielle
- [Template - Ecriture Enrichie Toolkits](./Template%20-%20Ecriture%20Enrichie%20Toolkits.md) : structure et contenus types

### 1.1 Portée / Scope

| Inclus | Exclus |
|--------|--------|
| Documentation fondatrice, contrats, sécurité, dépendances, relations, implémentation | Code source, tests techniques, déploiement |
| Tous les Toolkits sous `docs/tools/<MiyuXXX>/` | Documentation des Cores (docs/core) |
| Conformité Glossaire, Tools et Toolkits, MIP v1 | Documentation client ou marketing |

### 1.2 Objectif

Garantir une **documentation complète, cohérente et gouvernée** pour chaque Toolkit : identité, catalogue d'outils, contrats de conformité, sécurité, dépendances, relations avec les Cores et autres Toolkits, et guide d'implémentation.

---

## 2. Principes fondamentaux

| Principe | Description |
|----------|-------------|
| **Un Toolkit = une composition officielle** | Agrégation d'Outils existants ; pas de capacité nouvelle (réf. Tools et Toolkits). |
| **Documentation = source de vérité** | La Doc Fondatrice et les contrats priment sur tout guide d'implémentation. |
| **Contrats allégés** | Chaque contrat renvoie au template Master Butler (ou équivalent) et ne décrit que les obligations spécifiques au kit. |
| **Alignement MIP obligatoire** | Tout Toolkit documenté doit inclure une section ou référence à l'alignement MIP v1 (MSCM, index). |
| **Nomenclature et arborescence** | Respect de la règle projet : `<PREFIX> - <SUJET> <DETAIL>.md`, pas d'accents, arborescence standard `docs/tools/<MiyuXXX>/`. |

---

## 3. Cycle d'écriture enrichie (obligatoire)

Le cycle est **ordonné** ; les livrables peuvent être rédigés en parallèle dès que les dépendances sont satisfaites.

```
1. Documentation Fondatrice
2. Reference Outils
3. Contrats (Governance obligatoire ; KindMother, Security, Boundary, Dependencies si applicable)
4. Sécurité (niveau, états — intégrés à la Doc Fondatrice ou contrat dédié)
5. Dépendances et relations (Doc Fondatrice + contrat Dependencies si besoin)
6. Guide d'implémentation (Reference Implementation Guidelines)
7. Vérification et audit
```

Aucune publication officielle d'un Toolkit sans **au minimum** : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

---

## 4. Livrables obligatoires et optionnels

### 4.1 Obligatoires

| Livrable | Emplacement | Description |
|----------|-------------|-------------|
| **Documentation Fondatrice** | `docs/tools/<MiyuXXX>/<MiyuXXX> - Documentation Fondatrice.md` | Identité, ToolkitId, définition canonique, liste des Tools, gouvernance, niveau de sécurité et états, relation KindMother, alignement MIP. |
| **Reference Outils** | `docs/tools/<MiyuXXX>/<MiyuXXX> - Reference Outils.md` | Liste détaillée des ToolIds (action, niveau sécurité, note). |
| **Tool Governance Compliance Contract** | `docs/tools/<MiyuXXX>/contracts/governance/<MiyuXXX> - Tool Governance Compliance Contract.md` | Conformité au [Master Butler - Tool Governance Compliance Template](../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md) ; obligations spécifiques du kit. |

### 4.2 Optionnels (selon besoin du kit)

| Livrable | Emplacement | Quand l'utiliser |
|----------|-------------|------------------|
| **KindMother Integration Contract** | `docs/tools/<MiyuXXX>/contracts/integration/<MiyuXXX> - KindMother Integration Contract.md` | Si le kit produit des WriteIntent ou lit des données métier via KindMother. |
| **Security and States Contract** | `docs/tools/<MiyuXXX>/contracts/security/<MiyuXXX> - Security and States Contract.md` | Si niveau 3–4 ou règles spécifiques (sanitization, CSP, audit). |
| **Runtime Boundary Contract** | `docs/tools/<MiyuXXX>/contracts/boundaries/<MiyuXXX> - Runtime Boundary Contract.md` | Si bornage explicite (BOUND-*), interdictions d'accès ou de périmètre. |
| **Dependencies Contract** | `docs/tools/<MiyuXXX>/contracts/dependencies/<MiyuXXX> - Dependencies Contract.md` | Si liste fermée de dépendances (Cores, Kernel, autres Toolkits) à formaliser. |
| **Reference Implementation Guidelines** | `docs/tools/<MiyuXXX>/implementation/<MiyuXXX> - Reference Implementation Guidelines.md` | Recommandé pour tout kit prioritaire ; dérivé du [Template Reference Implementation Guidelines](../tools/docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md). |

### 4.3 Arborescence type

```
docs/tools/<MiyuXXX>/
├── _index.md
├── <MiyuXXX> - Documentation Fondatrice.md
├── <MiyuXXX> - Reference Outils.md
├── contracts/
│   ├── governance/
│   │   └── <MiyuXXX> - Tool Governance Compliance Contract.md
│   ├── integration/   (si applicable)
│   │   └── <MiyuXXX> - KindMother Integration Contract.md
│   ├── security/       (si applicable)
│   │   └── <MiyuXXX> - Security and States Contract.md
│   ├── boundaries/     (si applicable)
│   │   └── <MiyuXXX> - Runtime Boundary Contract.md
│   └── dependencies/  (si applicable)
│       └── <MiyuXXX> - Dependencies Contract.md
└── implementation/     (recommandé)
    └── <MiyuXXX> - Reference Implementation Guidelines.md
```

---

## 5. Règles de contenu (résumé)

### 5.1 Documentation Fondatrice

- **Contexte** : rôle du kit, domaine, autorité (KindMother, StrongFather).
- **Portée / Scope** : ce qui est inclus / exclu.
- **Définition canonique** : phrase type « MiyuXXX est une composition officielle d'outils de… ».
- **Identifiant et catalogue** : ToolkitId `toolkit.<domain>.<name>`, format conforme Master Butler.
- **Liste des outils composants** : tableau ou renvoi à Reference Outils.
- **Gouvernance** : flux standard (Tools et Toolkits) + spécificités.
- **Niveau de sécurité et états** : niveau 0–4, états autorisés/interdits (HEALTHY, DEGRADED, SECURITY_LOCKDOWN, MAINTENANCE).
- **Relation avec KindMother** : si écriture/lecture données métier ; WriteIntent.
- **Alignement MIP** : référence au [MIP v1](./Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) ; domaine, layer, blocs MSCM.
- **Références croisées** : Glossaire, Tools et Toolkits, contrats du kit.
- **Lien vers contrat** : phrase explicite « Les obligations de conformité détaillées sont dans [MiyuXXX - Tool Governance Compliance Contract](…). »

### 5.2 Contrats

- Référencer le template ou document parent (Master Butler).
- Ne rédiger que les **obligations spécifiques** au kit (WriteIntent, niveau sécurité, interdictions BOUND-*, dépendances fermées).
- Statut : « Contrat de conformité » ou « Contrat d'intégration » selon le type.

### 5.3 Sécurité

- **Niveaux (0–4)** : Public, Standard, Sensitive, Critical, Highest (Glossaire).
- **États** : HEALTHY, DEGRADED, SECURITY_LOCKDOWN, MAINTENANCE (opérationnel WorrySentinel / Caring Nanny).
- Pour niveau 3–4 : renvoi explicite au contrat Security ou à la doctrine sécurité du projet.

### 5.4 Dépendances et relations

- **Cores** : Master Butler, StrongFather, KindMother, WorrySentinel, Caring Nanny, Ever Buddy (selon besoin).
- **Kernel** : Id, Logger, Clock, Config, Lifecycle.
- **Autres Toolkits** : si le kit consomme des capacités d'un autre Toolkit, le documenter (Doc Fondatrice ou Dependencies Contract).
- Règle : pas de dépendance non déclarée ; pas d'Opérateur comme dépendance directe du Toolkit (flux via BondingBrother).

### 5.5 Implémentation

- Guide **informatif, non normatif** : il ne crée pas de nouvelles règles.
- Sources : Doc Fondatrice, Reference Outils, tous les contrats du kit.
- Contenu type : principes BOUND-*, interdictions, patterns recommandés, gestion d'erreurs, traçabilité, alignement MIP/MSCM, références.

---

## 6. Vérification et conformité

Avant gel ou publication :

| Vérification | Critère |
|--------------|---------|
| **Terminologie** | Glossaire respecté (Outil, Kit d'Outils, WriteIntent, KindMother, etc.). |
| **ToolkitId** | Format `toolkit.<domain>.<name>`. |
| **Composition** | Au moins deux Tools par Toolkit (Toolkit Composition Contract). |
| **Contrat Governance** | Présent et référencé depuis la Doc Fondatrice. |
| **Alignement MIP** | Section ou phrase explicite avec lien vers MIP v1. |
| **Numérotation** | Sections cohérentes (ex. § 9 Alignement MIP, § 10 Références). |
| **Liens** | Tous les liens relatifs valides (Doc Fondatrice → contrats, → référence). |

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| Template Ecriture Enrichie Toolkits | [Template - Ecriture Enrichie Toolkits](./Template%20-%20Ecriture%20Enrichie%20Toolkits.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Master Butler - Tool Governance Compliance Template | [Master Butler - Tool Governance Compliance Template](../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](./Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |
| Reference Implementation Guidelines Template | [docs_tools - Reference Implementation Guidelines Template](../tools/docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| Audit Qualité Conformité Sécurité | [docs_tools - Audit Qualite Conformite Securite Implementation](../tools/docs_tools%20-%20Audit%20Qualite%20Conformite%20Securite%20Implementation.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Protocole normatif — Écriture enrichie Toolkits
