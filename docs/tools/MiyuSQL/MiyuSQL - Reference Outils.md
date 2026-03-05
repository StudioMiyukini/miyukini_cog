# MiyuSQL â€” RÃ©fÃ©rence des outils

## 1. Contexte

Ce document dÃ©crit **chaque outil (Tool)** composant le kit MiyuSQL. Il constitue la rÃ©fÃ©rence technique des capacitÃ©s atomiques de manipulation de donnÃ©es en base (requÃªtes, transactions, cache) sans aucune logique mÃ©tier. Les Tools sont gouvernÃ©s par les Cores (Master Butler, WorrySentinel, Caring Nanny, StrongFather) ; l'autoritÃ© sur les donnÃ©es appartient Ã  KindMother.

**RÃ©fÃ©rence du kit :** [MiyuSQL - Documentation Fondatrice](./MiyuSQL%20-%20Documentation%20Fondatrice.md)

---

## 2. PortÃ©e / Scope

**Ce document fournit :**

- La liste exhaustive des Tools du kit MiyuSQL
- Pour chaque Tool : **ToolId**, **nom lisible**, **action** (phrase courte Â« fait quoi Â»), **niveau de sÃ©curitÃ©** typique, **capability_id** si applicable

**Hors scope :**

- L'implÃ©mentation (driver SQL, connexions, pool)
- Le choix de driver ou de dialecte SQL

---

## 3. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sÃ©curitÃ© | capability_id (ex.) |
|--------|-------------|--------|------------------|----------------------|
| `tool.query.execute` | ExÃ©cution de requÃªte | ExÃ©cute une requÃªte (lecture ou Ã©criture) selon les paramÃ¨tres fournis ; ne dÃ©cide pas du contenu ni de l'autorisation. | 2 | `data.query.execute` |
| `tool.query.prepare` | PrÃ©paration de requÃªte | PrÃ©pare ou valide une requÃªte (syntaxe, paramÃ¨tres) sans l'exÃ©cuter. | 2 | `data.query.prepare` |
| `tool.transaction.begin` | DÃ©but de transaction | DÃ©marre une transaction sur la connexion gouvernÃ©e. | 2 | `data.transaction.begin` |
| `tool.transaction.commit` | Validation de transaction | Valide (commit) la transaction en cours. | 2 | `data.transaction.commit` |
| `tool.transaction.rollback` | Annulation de transaction | Annule (rollback) la transaction en cours. | 2 | `data.transaction.rollback` |
| `tool.cache.get` | Lecture cache | RÃ©cupÃ¨re une entrÃ©e depuis le cache gouvernÃ© (clÃ© fournie) ; ne dÃ©cide pas de la politique de cache. | 2 | `cache.read` |
| `tool.cache.set` | Ã‰criture cache | Enregistre une entrÃ©e dans le cache gouvernÃ© (clÃ©, valeur, TTL optionnel) ; ne dÃ©cide pas de la politique. | 2 | `cache.write` |
| `tool.cache.invalidate` | Invalidation cache | Invalide une ou plusieurs entrÃ©es du cache (par clÃ© ou motif) ; ne dÃ©cide pas de la politique. | 2 | `cache.invalidate` |
| `tool.schema.read` | Lecture schÃ©ma | Lit les mÃ©tadonnÃ©es du schÃ©ma (tables, colonnes, types) sans modifier la base. | 2 | `data.schema.read` |

**Format ToolId :** `tool.<domain>.<action>[.<qualifier>]` â€” conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md).

---

## 4. DÃ©tail par outil (rÃ©sumÃ©)

### 4.1 RequÃªtes

- **tool.query.execute** â€” ExÃ©cute une requÃªte SQL ou Ã©quivalent (paramÃ¨tres fournis). Retourne un rÃ©sultat (lignes, count, etc.) selon le type de requÃªte. Ne contient aucune logique mÃ©tier.
- **tool.query.prepare** â€” PrÃ©pare une requÃªte (parsing, validation des paramÃ¨tres) sans l'exÃ©cuter. Utile pour validation ou planification.

### 4.2 Transactions

- **tool.transaction.begin** â€” DÃ©marre une transaction. Les appels suivants (execute, etc.) s'effectuent dans cette transaction jusqu'Ã  commit ou rollback.
- **tool.transaction.commit** â€” Valide la transaction en cours et rend les modifications permanentes.
- **tool.transaction.rollback** â€” Annule la transaction en cours et restaure l'Ã©tat prÃ©cÃ©dent.

### 4.3 Cache

- **tool.cache.get** â€” RÃ©cupÃ¨re une valeur depuis le cache (clÃ©). Retourne vide ou absent si non trouvÃ©.
- **tool.cache.set** â€” Enregistre une valeur dans le cache (clÃ©, valeur, TTL optionnel).
- **tool.cache.invalidate** â€” Invalide une entrÃ©e ou un ensemble d'entrÃ©es (par clÃ© ou motif).

### 4.4 SchÃ©ma

- **tool.schema.read** â€” Lit les mÃ©tadonnÃ©es du schÃ©ma (tables, colonnes, types). Aucune modification ; lecture seule.

---

## 5. Alignement MIP

Chaque outil listÃ© ci-dessus est conÃ§u pour Ãªtre une **unitÃ© logique** pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation :

- **id** : identifiant du bloc (ex. dÃ©rivÃ© du ToolId)
- **do** : description fonctionnelle courte (ex. Â« exÃ©cute une requÃªte Â»)
- **role** : rÃ´le sÃ©mantique (ex. `data`)
- **layer** : couche (Strate 6 â€” outil / toolkit)

Ã€ l'implÃ©mentation, le code fournissant ces Tools devra Ãªtre balisÃ© MSCM afin d'alimenter **blocks.json**, **domains.json**, **layers.json** selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 6. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Documentation Fondatrice MiyuSQL | [MiyuSQL - Documentation Fondatrice](./MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-29  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence


