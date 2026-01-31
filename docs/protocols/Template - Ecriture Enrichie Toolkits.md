# Template — Écriture enrichie des Toolkits

**Version :** 1.0  
**Statut :** Template — à adapter par Toolkit  
**Date :** 2026-01-30  

---

## Contexte

Ce document est la **template maître** pour l'écriture enrichie d'un Kit d'Outils (Toolkit) dans l'écosystème Miyukini. Il regroupe les structures types de chaque livrable : Documentation Fondatrice, Reference Outils, contrats (gouvernance, intégration, sécurité, bornage, dépendances), sécurité, dépendances/relations, et guide d'implémentation.

**Usage :** Copier les sections concernées dans les fichiers du dossier `docs/tools/<MiyuXXX>/` et remplacer les placeholders `<MiyuXXX>`, `<ToolkitId>`, `<domain>`, etc. par les valeurs du kit.

**Référence :** [Miyukini Protocol - Ecriture Enrichie Toolkits](./Miyukini%20Protocol%20-%20Ecriture%20Enrichie%20Toolkits.md)

---

## 1. Checklist des livrables

| # | Livrable | Obligatoire | Fichier cible |
|---|----------|-------------|---------------|
| 1 | Documentation Fondatrice | Oui | `<MiyuXXX> - Documentation Fondatrice.md` |
| 2 | Reference Outils | Oui | `<MiyuXXX> - Reference Outils.md` |
| 3 | Tool Governance Compliance Contract | Oui | `contracts/governance/<MiyuXXX> - Tool Governance Compliance Contract.md` |
| 4 | KindMother Integration Contract | Si écriture/lecture données métier | `contracts/integration/<MiyuXXX> - KindMother Integration Contract.md` |
| 5 | Security and States Contract | Si niveau 3–4 ou règles spécifiques | `contracts/security/<MiyuXXX> - Security and States Contract.md` |
| 6 | Runtime Boundary Contract | Si bornage explicite (BOUND-*) | `contracts/boundaries/<MiyuXXX> - Runtime Boundary Contract.md` |
| 7 | Dependencies Contract | Si liste fermée à formaliser | `contracts/dependencies/<MiyuXXX> - Dependencies Contract.md` |
| 8 | Reference Implementation Guidelines | Recommandé | `implementation/<MiyuXXX> - Reference Implementation Guidelines.md` |

---

## 2. Template — Documentation Fondatrice

**Fichier :** `docs/tools/<MiyuXXX>/<MiyuXXX> - Documentation Fondatrice.md`

```markdown
# <MiyuXXX> — Documentation Fondatrice

## 1. Contexte

**<MiyuXXX>** est le **kit d'outils (Toolkit)** de [domaine / description courte] de l'écosystème Miyukini. [Une phrase sur les capacités exposées et l'alignement éventuel avec un document de référence, ex. Équivalents Moteur Forum.]

L'autorité sur [données / périmètre] appartient à **KindMother** [ou N/A]. <MiyuXXX> expose des capacités d'exécution gouvernée ; les décisions [ex. modification autorisée, règles métier] relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de <MiyuXXX>, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother [si applicable].

**Hors scope :** [ex. identité de base MiyuAuth ; affichage MiyuWeb ; implémentation détaillée.]

---

## 3. Définition canonique

> **<MiyuXXX> est une composition officielle d'outils de [domaine / description], déclarée et gouvernée par l'environnement.**

- <MiyuXXX> **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- <MiyuXXX> **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; décision [ex. modification, règles] = StrongFather.

**Règle fondamentale :** [ex. Toute écriture = **WriteIntent** vers KindMother. Règles métier = StrongFather.]

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.<domain>.<name>` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `<domain>` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [<MiyuXXX> - Reference Outils](./<MiyuXXX> - Reference Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.<domain>.<action1>` | [Description] |
| `tool.<domain>.<action2>` | [Description] ; autorisation = StrongFather / WriteIntent KindMother |
| … | … |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini Conceptual References - Tools et Toolkits.md)). Spécificité : [ex. décision = StrongFather ; toute écriture = WriteIntent KindMother.]

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 4** (justifier : public, personnel, sensible, critique) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

[Si niveau 3–4 : renvoi au contrat Security ou doctrine sécurité.]

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur [données]. Toute création ou mise à jour passe par **WriteIntent** vers KindMother. [Schéma / périmètre = KindMother.]

Les obligations de conformité détaillées sont dans [<MiyuXXX> - Tool Governance Compliance Contract](./contracts/governance/<MiyuXXX> - Tool Governance Compliance Contract.md).

[Si pas de KindMother : « Ce kit ne persiste pas de données métier ; pas de relation KindMother. »]

---

## 9. Dépendances et relations

| Type | Dépendance | Rôle |
|------|------------|------|
| Cores | Master Butler, StrongFather, [KindMother / WorrySentinel / Caring Nanny / Ever Buddy] | Catalogue, décision, [données / sécurité / état / cycle de vie] |
| Kernel | Id, Logger, Clock, Config, Lifecycle | Technique |
| Autres Toolkits | [MiyuYYY si consommé] | [Capacité consommée] |

**Règle :** Pas de dépendance non déclarée ; pas d'Opérateur comme dépendance directe (flux via BondingBrother).

[Si liste fermée formalisée : renvoyer au Dependencies Contract.]

---

## 10. Alignement MIP

La documentation et la future implémentation de <MiyuXXX> sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools <MiyuXXX> devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

- **Domaine** : cohérent avec le ToolkitId (ex. `<domain>`).
- **Layer** : Strate 6 (outil / toolkit) dans layers.json.
- **Blocs** : Chaque Tool = unité logique avec `id`, `do`, `role`, `layer` pour blocks.json.

---

## 11. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| [Document référence métier si applicable] | [Lien] |
| Master Butler - Tool Governance Contract | [Lien] |

---

**Date de création :** YYYY-MM-DD  
**Version :** 1.0  
**Statut :** Document de référence fondateur
```

---

## 3. Template — Reference Outils

**Fichier :** `docs/tools/<MiyuXXX>/<MiyuXXX> - Reference Outils.md`

```markdown
# <MiyuXXX> — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **<MiyuXXX>** (`toolkit.<domain>.<name>`). Chaque outil est une capacité atomique gouvernée ; [décision / persistance] relève de [StrongFather / KindMother].

**Référence :** [<MiyuXXX> - Documentation Fondatrice](./<MiyuXXX> - Documentation Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.<domain>.<action1>` | [Action courte] | 0–4 | Lecture / WriteIntent KindMother / Autorisation StrongFather |
| `tool.<domain>.<action2>` | [Action courte] | 0–4 | … |
| … | … | … | … |

---

**Invariant :** [Règle clé du kit, ex. Toute écriture = WriteIntent KindMother.]
```

---

## 4. Template — Tool Governance Compliance Contract (obligatoire)

**Fichier :** `docs/tools/<MiyuXXX>/contracts/governance/<MiyuXXX> - Tool Governance Compliance Contract.md`

```markdown
# <MiyuXXX> — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master Butler - Tool Governance Compliance Template.md).

**ToolkitId :** `toolkit.<domain>.<name>`

---

## Obligations spécifiques <MiyuXXX>

- [Obligation 1 : ex. Décision (modification autorisée) = StrongFather.]
- [Obligation 2 : ex. Toute écriture = **WriteIntent** vers KindMother.]
- [Obligation 3 : ex. Schéma / périmètre = KindMother. Règles métier = StrongFather.]
- [Autres obligations spécifiques au kit.]

---

**Date de création :** YYYY-MM-DD  
**Version :** 1.0  
**Statut :** Contrat de conformité
```

---

## 5. Template — KindMother Integration Contract (optionnel)

**Fichier :** `docs/tools/<MiyuXXX>/contracts/integration/<MiyuXXX> - KindMother Integration Contract.md`

**À utiliser si :** le kit produit des WriteIntent ou lit des données métier via KindMother.

```markdown
# <MiyuXXX> — KindMother Integration Contract

## Contexte

Ce contrat définit les **règles d'intégration** entre le Toolkit <MiyuXXX> et **KindMother** (Core de données). Il complète le [Tool Governance Compliance Contract](../governance/<MiyuXXX> - Tool Governance Compliance Contract.md).

**ToolkitId :** `toolkit.<domain>.<name>`

---

## Périmètre des données

| Donnée / entité | Autorité | Lecture | Écriture |
|-----------------|----------|---------|----------|
| [Entité 1] | KindMother | [ToolId(s)] | WriteIntent via [ToolId(s)] |
| [Entité 2] | KindMother | … | … |

---

## Règles d'accès

- Toute **lecture** de données métier : via flux gouverné (données déjà présentes dans le flux ou requête via KindMother selon contrat environnement).
- Toute **écriture** : **WriteIntent** vers KindMother ; aucun accès direct à MiyuSQL ou à la base depuis le kit.
- Schéma / structure des données : définis par KindMother ; le kit ne modifie pas le schéma.

---

## WriteIntent (si applicable)

- **Types d'écriture** : [ex. création profil, mise à jour champ, suppression douce.]
- **Validation** : StrongFather décide de l'autorisation ; KindMother valide et persiste.
- **Traçabilité** : conformément au contrat KindMother et au Logger Kernel.

---

**Date de création :** YYYY-MM-DD  
**Version :** 1.0  
**Statut :** Contrat d'intégration
```

---

## 6. Template — Security and States Contract (optionnel)

**Fichier :** `docs/tools/<MiyuXXX>/contracts/security/<MiyuXXX> - Security and States Contract.md`

**À utiliser si :** niveau de sécurité 3–4 ou règles spécifiques (sanitization, CSP, audit, états dégradés).

```markdown
# <MiyuXXX> — Security and States Contract

## Contexte

Ce contrat définit les **règles de sécurité et d'états** applicables au Toolkit <MiyuXXX>. Il complète le [Tool Governance Compliance Contract](../governance/<MiyuXXX> - Tool Governance Compliance Contract.md) et s'aligne sur [WorrySentinel](../../../../reference/…) et [Caring Nanny](../../../../reference/…).

**ToolkitId :** `toolkit.<domain>.<name>`

---

## Niveau de sécurité

| Élément | Valeur |
|---------|--------|
| **Niveau du kit** | 0 | 1 | 2 | 3 | 4 |
| **Justification** | [Public / Standard / Sensitive / Critical / Highest] |
| **Données concernées** | [Type de données] |

---

## États autorisés / interdits

| État système | Autorisé pour <MiyuXXX> |
|--------------|--------------------------|
| HEALTHY | Oui |
| DEGRADED | Oui / Non |
| SECURITY_LOCKDOWN | Non |
| MAINTENANCE | Non |

---

## Règles spécifiques (si applicable)

- **Sanitization** : [ex. sortie HTML, CSP, validation entrées.]
- **Audit** : [ex. traçabilité des appels, pas d'exposition de données sensibles.]
- **Dégradation** : [ex. en SECURITY_LOCKDOWN, refus d'exécution et signal.]

---

**Date de création :** YYYY-MM-DD  
**Version :** 1.0  
**Statut :** Contrat de sécurité
```

---

## 7. Template — Runtime Boundary Contract (optionnel)

**Fichier :** `docs/tools/<MiyuXXX>/contracts/boundaries/<MiyuXXX> - Runtime Boundary Contract.md`

**À utiliser si :** bornage explicite (BOUND-*), interdictions d'accès ou de périmètre.

```markdown
# <MiyuXXX> — Runtime Boundary Contract

## Contexte

Ce contrat définit les **bornes d'exécution** du Toolkit <MiyuXXX>. Il formalise les interdictions (BOUND-*) et le périmètre autorisé.

**ToolkitId :** `toolkit.<domain>.<name>`

---

## Interdictions (BOUND-*)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| BOUND-1 | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat |
| BOUND-2 | Pas de choix métier | Exécution sur données/paramètres fournis |
| BOUND-3 | Pas d'accès direct non gouverné | WriteIntent KindMother ou pas de persistance métier |
| BOUND-4 | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| BOUND-5 | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé (niveau, permissions) |
| BOUND-6 | Pas de capacité nouvelle | Uniquement ToolIds déclarés |

[Ajouter bornes spécifiques au kit si besoin.]

---

## Périmètre autorisé

- **Entrées** : Contexte gouverné, paramètres fournis par le flux.
- **Sorties** : Résultat ou erreur contractuelle ; pas d'effet de bord non documenté.
- **Accès réseau / IO** : [Interdit / limité à …]

---

**Date de création :** YYYY-MM-DD  
**Version :** 1.0  
**Statut :** Contrat de bornage
```

---

## 8. Template — Dependencies Contract (optionnel)

**Fichier :** `docs/tools/<MiyuXXX>/contracts/dependencies/<MiyuXXX> - Dependencies Contract.md`

**À utiliser si :** liste fermée de dépendances (Cores, Kernel, autres Toolkits) à formaliser.

```markdown
# <MiyuXXX> — Dependencies Contract

## Contexte

Ce contrat définit la **liste fermée des dépendances** du Toolkit <MiyuXXX>. Aucune dépendance non listée n'est autorisée.

**ToolkitId :** `toolkit.<domain>.<name>`

---

## Dépendances autorisées

| Type | Composant | Usage |
|------|-----------|--------|
| Cores | Master Butler | Catalogue, permissions |
| Cores | StrongFather | Décision |
| Cores | KindMother | WriteIntent, lecture données [si applicable] |
| Cores | WorrySentinel | Niveau sécurité |
| Cores | Caring Nanny | État système |
| Cores | Ever Buddy | Cycle de vie [si applicable] |
| Kernel | Id, Logger, Clock, Config, Lifecycle | Technique |
| Toolkit | [MiyuYYY] | [Capacité consommée] |

---

## Règles

- Aucune dépendance vers un **Opérateur** (flux via BondingBrother).
- Aucune librairie externe non déclarée dans l'environnement.
- Modification de la liste = révision du contrat et de la Doc Fondatrice.

---

**Date de création :** YYYY-MM-DD  
**Version :** 1.0  
**Statut :** Contrat de dépendances
```

---

## 9. Guide d'implémentation (Reference Implementation Guidelines)

**Fichier :** `docs/tools/<MiyuXXX>/implementation/<MiyuXXX> - Reference Implementation Guidelines.md`

**Source :** Utiliser le [docs_tools - Reference Implementation Guidelines Template](../tools/docs_tools - Reference Implementation Guidelines Template.md) et l'adapter au kit en remplaçant :

- MiyuXXX, ToolkitId, domaine
- Liste des contrats sources du kit (Doc Fondatrice, Reference Outils, Governance, KindMother, Security, Boundary, Dependencies)
- Principes spécifiques (sanitization, niveau sécurité, WriteIntent, etc.)
- Patterns et spécificités du kit

Le guide est **informatif, non normatif** ; les contrats priment toujours.

---

## 10. Checklist finale avant publication

- [ ] Documentation Fondatrice : Contexte, Portée, Définition canonique, Identifiant, Liste outils, Gouvernance, Sécurité/états, KindMother, Dépendances/relations, Alignement MIP, Références, lien vers contrat.
- [ ] Reference Outils : Liste complète des ToolIds avec action, niveau sécurité, note.
- [ ] Tool Governance Compliance Contract : Référence template Master Butler + obligations spécifiques.
- [ ] Contrats optionnels : Rédigés si applicable (KindMother, Security, Boundary, Dependencies).
- [ ] Nomenclature : Fichiers sans accents, format `MiyuXXX - Sujet.md`.
- [ ] Arborescence : `docs/tools/<MiyuXXX>/` avec contracts/, implementation/ si besoin.
- [ ] Liens : Tous les liens relatifs valides ; lien MIP v1 vers `protocols/Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol.md`.
- [ ] Terminologie : Glossaire respecté (Outil, Kit d'Outils, WriteIntent, KindMother, StrongFather, etc.).
- [ ] ToolkitId : Format `toolkit.<domain>.<name>`.
- [ ] Au moins deux Tools dans le Toolkit.

---

## Références croisées

| Document | Lien |
|----------|------|
| Protocole Ecriture Enrichie Toolkits | [Miyukini Protocol - Ecriture Enrichie Toolkits](./Miyukini%20Protocol%20-%20Ecriture%20Enrichie%20Toolkits.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Master Butler - Tool Governance Compliance Template | [Master Butler - Tool Governance Compliance Template](../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md) |
| Reference Implementation Guidelines Template | [docs_tools - Reference Implementation Guidelines Template](../tools/docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |

---

**Date du template :** 2026-01-30  
**Version :** 1.0  
**Statut :** Template — à adapter par Toolkit dans docs/tools/<MiyuXXX>/
