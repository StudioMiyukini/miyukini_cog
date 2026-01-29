# MiyuSQL — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuSQL. Il constitue la référence technique des capacités atomiques de manipulation de données en base (requêtes, transactions, cache) sans aucune logique métier. Les Tools sont gouvernés par les Cores (Master Butler, WorrySentinel, Caring Nanny, StrongFather) ; l'autorité sur les données appartient à KindMother.

**Référence du kit :** [MiyuSQL - Documentation Fondatrice](./MiyuSQL%20-%20Documentation%20Fondatrice.md)

---

## 2. Portée / Scope

**Ce document fournit :**

- La liste exhaustive des Tools du kit MiyuSQL
- Pour chaque Tool : **ToolId**, **nom lisible**, **action** (phrase courte « fait quoi »), **niveau de sécurité** typique, **capability_id** si applicable

**Hors scope :**

- L'implémentation (driver SQL, connexions, pool)
- Le choix de driver ou de dialecte SQL

---

## 3. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité | capability_id (ex.) |
|--------|-------------|--------|------------------|----------------------|
| `tool.query.execute` | Exécution de requête | Exécute une requête (lecture ou écriture) selon les paramètres fournis ; ne décide pas du contenu ni de l'autorisation. | 2 | `data.query.execute` |
| `tool.query.prepare` | Préparation de requête | Prépare ou valide une requête (syntaxe, paramètres) sans l'exécuter. | 2 | `data.query.prepare` |
| `tool.transaction.begin` | Début de transaction | Démarre une transaction sur la connexion gouvernée. | 2 | `data.transaction.begin` |
| `tool.transaction.commit` | Validation de transaction | Valide (commit) la transaction en cours. | 2 | `data.transaction.commit` |
| `tool.transaction.rollback` | Annulation de transaction | Annule (rollback) la transaction en cours. | 2 | `data.transaction.rollback` |
| `tool.cache.get` | Lecture cache | Récupère une entrée depuis le cache gouverné (clé fournie) ; ne décide pas de la politique de cache. | 2 | `cache.read` |
| `tool.cache.set` | Écriture cache | Enregistre une entrée dans le cache gouverné (clé, valeur, TTL optionnel) ; ne décide pas de la politique. | 2 | `cache.write` |
| `tool.cache.invalidate` | Invalidation cache | Invalide une ou plusieurs entrées du cache (par clé ou motif) ; ne décide pas de la politique. | 2 | `cache.invalidate` |
| `tool.schema.read` | Lecture schéma | Lit les métadonnées du schéma (tables, colonnes, types) sans modifier la base. | 2 | `data.schema.read` |

**Format ToolId :** `tool.<domain>.<action>[.<qualifier>]` — conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md).

---

## 4. Détail par outil (résumé)

### 4.1 Requêtes

- **tool.query.execute** — Exécute une requête SQL ou équivalent (paramètres fournis). Retourne un résultat (lignes, count, etc.) selon le type de requête. Ne contient aucune logique métier.
- **tool.query.prepare** — Prépare une requête (parsing, validation des paramètres) sans l'exécuter. Utile pour validation ou planification.

### 4.2 Transactions

- **tool.transaction.begin** — Démarre une transaction. Les appels suivants (execute, etc.) s'effectuent dans cette transaction jusqu'à commit ou rollback.
- **tool.transaction.commit** — Valide la transaction en cours et rend les modifications permanentes.
- **tool.transaction.rollback** — Annule la transaction en cours et restaure l'état précédent.

### 4.3 Cache

- **tool.cache.get** — Récupère une valeur depuis le cache (clé). Retourne vide ou absent si non trouvé.
- **tool.cache.set** — Enregistre une valeur dans le cache (clé, valeur, TTL optionnel).
- **tool.cache.invalidate** — Invalide une entrée ou un ensemble d'entrées (par clé ou motif).

### 4.4 Schéma

- **tool.schema.read** — Lit les métadonnées du schéma (tables, colonnes, types). Aucune modification ; lecture seule.

---

## 5. Alignement MIP

Chaque outil listé ci-dessus est conçu pour être une **unité logique** pouvant devenir un **bloc MSCM** à l'implémentation :

- **id** : identifiant du bloc (ex. dérivé du ToolId)
- **do** : description fonctionnelle courte (ex. « exécute une requête »)
- **role** : rôle sémantique (ex. `data`)
- **layer** : couche (Strate 6 — outil / toolkit)

À l'implémentation, le code fournissant ces Tools devra être balisé MSCM afin d'alimenter **blocks.json**, **domains.json**, **layers.json** selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 6. Références croisées

| Document | Lien |
|----------|------|
| Documentation Fondatrice MiyuSQL | [MiyuSQL - Documentation Fondatrice](./MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |

---

**Date de création :** 2026-01-29  
**Version :** 1.0  
**Statut :** Document de référence
