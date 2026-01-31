# MiyuForum — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuForum conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuForum en logique d'implémentation (Tools structure forum, gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuForum (catégories, forums, topics, posts, suivi lu, export) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuForum - Documentation Fondatrice** : ToolkitId `toolkit.community.forum`, liste des Tools (category, board, topic, post, readtrack, export), gouvernance, relation KindMother.
- **MiyuForum - Reference Outils** : Détail de chaque ToolId.
- **MiyuForum - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spécifiques.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

MiyuForum est invoqué uniquement après décision StrongFather (création topic/post, sticky, annonce). L'implémentation ne doit pas ré-évaluer les permissions. En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools (category.*, board.*, topic.*, post.*, readtrack.*, topic.export.*) exécutent sur les données fournies ; aucune décision sur le contenu, la visibilité ou les règles de modération (MiyuModerationForum).

### 2.3 Toute écriture = WriteIntent KindMother (BOUND-3)

**Règle fondamentale MiyuForum :** Toute création, mise à jour ou suppression (category, board, topic, post, readtrack) = **WriteIntent** vers KindMother. Aucun accès direct à la base ; les écritures passent par le flux gouverné.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds déclarés (tool.forum.category.*, tool.forum.board.*, tool.forum.topic.*, tool.forum.post.*, tool.forum.readtrack.*, tool.forum.topic.export.*).

### 2.7 Niveau de sécurité et états

Niveau **1 à 2** (contenu communautaire). États autorisés : `HEALTHY`, `DEGRADED`. États interdits : `SECURITY_LOCKDOWN`, `MAINTENANCE`. Vérifier l'état (Caring Nanny / WorrySentinel) avant exécution.

### 2.8 Alignement MIP/MSCM

Domaine `community`, layer Strate 6. À l'implémentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix métier | Pas de décision contenu, visibilité, modération |
| **BOUND-3** | Pas d'accès direct | Toute écriture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement ToolIds déclarés (category, board, topic, post, readtrack, topic.export) |

---

## 4. Patterns recommandés

### 4.1 Structure des Tools

Chaque ToolId (`tool.forum.category.list|get|create|update`, `tool.forum.board.*`, `tool.forum.topic.*`, `tool.forum.post.*`, `tool.forum.readtrack.*`, `tool.forum.topic.export.*`) = unité d'exécution atomique : entrée (contexte gouverné, paramètres : ids, payload), sortie (résultat ou erreur). Pas d'état métier partagé entre appels.

### 4.2 Interface avec KindMother

Toute écriture (création/mise à jour/suppression de category, board, topic, post, readtrack) produit une **WriteIntent** vers KindMother. Les lectures (list, get) s'appuient sur des données fournies dans le flux (lues en amont par MiyuSQL sous KindMother) ou sur un contrat d'intégration documenté. MiyuForum n'accède pas directement à la base.

### 4.3 Gestion des erreurs et traçabilité

Erreurs techniques (paramètres invalides, WriteIntent refusée) remontées sans exposer de données sensibles. En cas de violation de bornage, refus d'exécution et signal. Utiliser le Logger du Kernel pour la traçabilité (sans contenu métier).

### 4.4 Export topic

`tool.forum.topic.export.*` exécute la génération (PDF, texte) sur les données fournies dans le flux ; pas d'écriture métier ; résultat retourné dans le flux.

---

## 5. Alignement MIP / MSCM

- **Domaine** : `community` (cohérent avec toolkit.community.forum).
- **Layer** : Strate 6 dans layers.json.
- **Blocs** : Chaque Tool MiyuForum est une unité logique avec `id`, `do`, `role`, `layer` pour alimenter blocks.json. Balisage MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 6. Tests

Les tests relèvent des bonnes pratiques projet et du [MiyuForum - Tool Governance Compliance Contract](../contracts/governance/MiyuForum%20-%20Tool%20Governance%20Compliance%20Contract.md) (obligations de non-régression, états). Scénarios recommandés : création topic/post via WriteIntent, list/get avec données en flux, readtrack mark read, export sans écriture métier.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuForum - Documentation Fondatrice | [MiyuForum - Documentation Fondatrice](../MiyuForum%20-%20Documentation%20Fondatrice.md) |
| MiyuForum - Reference Outils | [MiyuForum - Reference Outils](../MiyuForum%20-%20Reference%20Outils.md) |
| MiyuForum - Tool Governance Compliance Contract | [MiyuForum - Tool Governance Compliance Contract](../contracts/governance/MiyuForum%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| MIP v1 | [MIP v1 MSCM Index Protocol](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif
