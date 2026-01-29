# MiyuSQL — Documentation Fondatrice

## 1. Contexte

**MiyuSQL** est le **kit d'outils (Toolkit)** de gestion de données en base de données (DB) de l'écosystème Miyukini. Il est primordial et très souvent utilisé : il intègre tous les outils de manipulation de données en base nécessaires aux Opérateurs pour exécuter des requêtes, gérer des transactions et utiliser un cache gouverné.

L'autorité sur les données et la persistance appartient à **KindMother** (Core de données, Strate 4). MiyuSQL expose des capacités d'exécution gouvernée (requête, transaction, cache) sans remplacer KindMother ; les Opérateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :**

- L'identité et la définition canonique de MiyuSQL
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliqués)
- Le niveau de sécurité et les états système autorisés ou interdits
- La relation avec KindMother
- L'alignement avec le protocole MIP v1 pour une future implémentation indexable

**Hors scope :**

- L'implémentation détaillée (driver SQL, connexions, pool)
- Toute logique métier (choix de schéma, règles métier) — celle-ci reste du ressort des Opérateurs et des Cores

---

## 3. Définition canonique

> **MiyuSQL est une composition officielle d'outils de manipulation de données en base de données, déclarée et gouvernée par l'environnement.**

- MiyuSQL **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuSQL **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (exécuter une requête, démarrer une transaction, lire/écrire le cache) sans décider quoi que ce soit.

**Règle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacité. Les capacités viennent exclusivement des Tools composants.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.data.miyusql` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `data` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuSQL est composé des Tools suivants (format canonique `tool.<domain>.<action>`). Le détail de chaque outil (action, niveau de sécurité, capability_id) est décrit dans [MiyuSQL - Reference Outils](./MiyuSQL%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.query.execute` | Exécute une requête (lecture ou écriture selon intention gouvernée) |
| `tool.query.prepare` | Prépare ou valide une requête sans l'exécuter |
| `tool.transaction.begin` | Démarre une transaction |
| `tool.transaction.commit` | Valide une transaction |
| `tool.transaction.rollback` | Annule une transaction |
| `tool.cache.get` | Récupère une entrée depuis le cache |
| `tool.cache.set` | Enregistre une entrée dans le cache |
| `tool.cache.invalidate` | Invalide une ou plusieurs entrées du cache |
| `tool.schema.read` | Lit les métadonnées du schéma (tables, colonnes) sans modifier |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuSQL en contient neuf.

---

## 6. Gouvernance

Tout appel à un outil du kit MiyuSQL (ou au kit lui-même) suit le flux de gouvernance suivant :

1. **Opérateur** (Strate 7) — demande d'utilisation d'un Tool ou du Toolkit
2. **BondingBrother** — médiation, traduction de l'intention, préparation du contexte
3. **Master Butler** — vérification de l'existence du Tool/Toolkit, permissions requises, niveau de sécurité
4. **WorrySentinel** — vérification que le niveau de sécurité actuel permet cet appel
5. **Caring Nanny** — vérification que l'état système (HEALTHY, DEGRADED, etc.) permet cet appel
6. **StrongFather** — décision finale ALLOW ou DENY
7. **Exécution** — si autorisé, le Tool (ou les Tools du Toolkit) exécute l'action ; les opérations de données sont réalisées sous autorité **KindMother** (persistance).

**Référence :** [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) (schéma de flux complet).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **2** (données utilisateur), cohérent avec WorrySentinel (Data Tools = 2). Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon [Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)) |

---

## 8. Relation avec KindMother

- **KindMother** est l'autorité absolue sur les données et la persistance (Core de données, Strate 4).
- Les opérations DB (lecture, écriture, transaction) sont **sous autorité KindMother** : les Tools MiyuSQL exécutent des capacités gouvernées (ex. exécuter une requête) mais ne décident pas des données à modifier ; le métier passe par **WriteIntent** et les décisions stratégiques via StrongFather.
- MiyuSQL **expose** les capacités d'exécution gouvernée (requête, transaction, cache) sans remplacer KindMother. Il ne contient aucune logique métier.

**Référence :** [Miyukini Conceptual References - Acces DB et Droits Agents IA](../../reference/Miyukini%20Conceptual%20References%20-%20Acces%20DB%20et%20Droits%20Agents%20IA.md) (distinction outillage vs WriteIntent, si pertinent).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuSQL sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `data` — cohérent avec la projection domains.json (blocs du domaine « data »).
- **Layer** : outil / toolkit (Strate 6) — à refléter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuSQL est une unité logique pouvant devenir un **bloc MSCM** à l'implémentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

À l'implémentation, le code fournissant les Tools MiyuSQL devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md). La documentation ne génère pas les fichiers `mscm_index/*` ; elle définit les concepts pour une indexation future.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| KindMother | [KindMother - Index](../../core/KindMother/_index.md) ou Documentation Fondatrice |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-29  
**Version :** 1.0  
**Statut :** Document de référence fondateur
