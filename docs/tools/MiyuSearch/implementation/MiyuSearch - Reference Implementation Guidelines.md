# MiyuSearch — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuSearch conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuSearch en logique d'implémentation (Tools indexation, requête full-text, suggestions ; pas de décision métier sur pertinence).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuSearch (index.update, query.execute, suggest) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuSearch - Documentation Fondatrice** : ToolkitId `toolkit.search.miyusearch`, liste des Tools (index.update, query.execute, suggest), gouvernance, relation KindMother (index = dérivation des données sources).
- **MiyuSearch - Reference Outils** : Détail de chaque ToolId.
- **MiyuSearch - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spécifiques.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

MiyuSearch est invoqué après décision de la gouvernance. L'implémentation ne ré-évalue pas les permissions. En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

**Règle fondamentale :** MiyuSearch **ne décide pas** du périmètre ni du classement métier. Les données à indexer et les critères de recherche sont fournis dans le flux gouverné ; les décisions sur ce qui est « pertinent » ou affiché relèvent des Opérateurs. Les Tools exécutent sur **données et critères fournis** ; pas de décision sur la pertinence métier.

### 2.3 Index sous autorité KindMother ; pas d'écriture métier directe (BOUND-3)

- **tool.search.index.update** : Met à jour l'index (document/champ fournis) ; WriteIntent ou flux gouverné vers stockage index. L'index est une **dérivation** des données KindMother ; pas d'écriture métier directe sur les données sources.
- **tool.search.query.execute** et **tool.search.suggest** : Lisent l'index pour requêter et suggérer ; critères fournis dans le flux ; retour (identifiants, scores, suggestions) sans décision métier sur le classement final.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant ; uniquement les ToolIds déclarés (index.update, query.execute, suggest).

### 2.7 Niveau de sécurité et états

Niveau **0 à 2** (contenu indexé peut être sensible). États autorisés : tous sauf restriction WorrySentinel. États interdits : selon politique (ex. index en lecture seule en maintenance). Vérifier l'état avant exécution.

### 2.8 Alignement MIP/MSCM

Domaine `search`, layer Strate 6. À l'implémentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution sur mandat uniquement |
| **BOUND-2** | Pas de choix métier | Pas de décision périmètre, pertinence, classement métier |
| **BOUND-3** | Pas d'écriture métier directe sur sources | Index = dérivation ; mise à jour index = flux gouverné / WriteIntent ; pas d'écriture sur données sources |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement index.update, query.execute, suggest |

---

## 4. Patterns recommandés

### 4.1 Structure des Tools

Chaque ToolId = unité atomique : entrée (contexte gouverné, paramètres : document/champ pour index.update ; critères pour query.execute ; préfixe pour suggest), sortie (résultat ou erreur). Pas d'état métier partagé. Format : `tool.search.index.update`, `tool.search.query.execute`, `tool.search.suggest`.

### 4.2 Interface avec KindMother et l'index

- **index.update** : Données fournies dans le flux (ou provenant de KindMother en amont) → mise à jour de l'index via WriteIntent ou mécanisme documenté ; pas d'écriture directe sur les tables métier sources.
- **query.execute / suggest** : Critères fournis dans le flux ; lecture de l'index ; retour d'identifiants/scores/suggestions. Le classement métier et le choix de ce qui est affiché restent du ressort des Opérateurs.

### 4.3 Gestion des erreurs et traçabilité

Erreurs techniques (index indisponible, critères invalides) remontées sans exposer de contenu indexé sensible. Logger du Kernel pour traçabilité (sans contenu des requêtes si sensible).

---

## 5. Alignement MIP / MSCM

- **Domaine** : `search` (toolkit.search.miyusearch).
- **Layer** : Strate 6 dans layers.json.
- **Blocs** : Chaque Tool MiyuSearch = unité logique avec `id`, `do`, `role`, `layer`. Balisage MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 6. Tests

Les tests relèvent des bonnes pratiques projet et du Tool Governance Compliance Contract. Scénarios recommandés : index.update avec données en flux, query.execute et suggest avec critères fournis, vérification qu'aucune décision métier sur pertinence n'est prise dans le kit.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuSearch - Documentation Fondatrice | [MiyuSearch - Documentation Fondatrice](../MiyuSearch%20-%20Documentation%20Fondatrice.md) |
| MiyuSearch - Reference Outils | [MiyuSearch - Reference Outils](../MiyuSearch%20-%20Reference%20Outils.md) |
| MiyuSearch - Tool Governance Compliance Contract | [MiyuSearch - Tool Governance Compliance Contract](../contracts/governance/MiyuSearch%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| MIP v1 | [MIP v1 MSCM Index Protocol](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif
