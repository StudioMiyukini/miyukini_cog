# MiyuSQL â€” Documentation Fondatrice

## 1. Contexte

**MiyuSQL** est le **kit d'outils (Toolkit)** de gestion de donnÃ©es en base de donnÃ©es (DB) de l'Ã©cosystÃ¨me Miyukini. Il est primordial et trÃ¨s souvent utilisÃ© : il intÃ¨gre tous les outils de manipulation de donnÃ©es en base nÃ©cessaires aux OpÃ©rateurs pour exÃ©cuter des requÃªtes, gÃ©rer des transactions et utiliser un cache gouvernÃ©.

L'autoritÃ© sur les donnÃ©es et la persistance appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuSQL expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (requÃªte, transaction, cache) sans remplacer KindMother ; les OpÃ©rateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :**

- L'identitÃ© et la dÃ©finition canonique de MiyuSQL
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliquÃ©s)
- Le niveau de sÃ©curitÃ© et les Ã©tats systÃ¨me autorisÃ©s ou interdits
- La relation avec KindMother
- L'alignement avec le protocole MIP v1 pour une future implÃ©mentation indexable

**Hors scope :**

- L'implÃ©mentation dÃ©taillÃ©e (driver SQL, connexions, pool)
- Toute logique mÃ©tier (choix de schÃ©ma, rÃ¨gles mÃ©tier) â€” celle-ci reste du ressort des OpÃ©rateurs et des Cores

---

## 3. DÃ©finition canonique

> **MiyuSQL est une composition officielle d'outils de manipulation de donnÃ©es en base de donnÃ©es, dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuSQL **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuSQL **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (exÃ©cuter une requÃªte, dÃ©marrer une transaction, lire/Ã©crire le cache) sans dÃ©cider quoi que ce soit.

**RÃ¨gle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacitÃ©. Les capacitÃ©s viennent exclusivement des Tools composants.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.data.miyusql` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `data` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuSQL est composÃ© des Tools suivants (format canonique `tool.<domain>.<action>`). Le dÃ©tail de chaque outil (action, niveau de sÃ©curitÃ©, capability_id) est dÃ©crit dans [MiyuSQL - Reference Outils](./MiyuSQL%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.query.execute` | ExÃ©cute une requÃªte (lecture ou Ã©criture selon intention gouvernÃ©e) |
| `tool.query.prepare` | PrÃ©pare ou valide une requÃªte sans l'exÃ©cuter |
| `tool.transaction.begin` | DÃ©marre une transaction |
| `tool.transaction.commit` | Valide une transaction |
| `tool.transaction.rollback` | Annule une transaction |
| `tool.cache.get` | RÃ©cupÃ¨re une entrÃ©e depuis le cache |
| `tool.cache.set` | Enregistre une entrÃ©e dans le cache |
| `tool.cache.invalidate` | Invalide une ou plusieurs entrÃ©es du cache |
| `tool.schema.read` | Lit les mÃ©tadonnÃ©es du schÃ©ma (tables, colonnes) sans modifier |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuSQL en contient neuf.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : les opÃ©rations de donnÃ©es (requÃªtes, transactions, cache) sont rÃ©alisÃ©es sous autoritÃ© **KindMother** (persistance).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **2** (donnÃ©es utilisateur), cohÃ©rent avec WorrySentinel (Data Tools = 2). Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon [Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)) |

---

## 8. Relation avec KindMother

- **KindMother** est l'autoritÃ© absolue sur les donnÃ©es et la persistance (Core de donnÃ©es, Strate 4).
- Les opÃ©rations DB (lecture, Ã©criture, transaction) sont **sous autoritÃ© KindMother** : les Tools MiyuSQL exÃ©cutent des capacitÃ©s gouvernÃ©es (ex. exÃ©cuter une requÃªte) mais ne dÃ©cident pas des donnÃ©es Ã  modifier ; le mÃ©tier passe par **WriteIntent** et les dÃ©cisions stratÃ©giques via StrongFather.
- MiyuSQL **expose** les capacitÃ©s d'exÃ©cution gouvernÃ©e (requÃªte, transaction, cache) sans remplacer KindMother. Il ne contient aucune logique mÃ©tier.

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Acces DB et Droits Agents IA](..//..//miyukini-webway-system//reference//_index.md) (distinction outillage vs WriteIntent, si pertinent).

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuSQL - Tool Governance Compliance Contract](./contracts/governance/MiyuSQL%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuSQL sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `data` â€” cohÃ©rent avec la projection domains.json (blocs du domaine Â« data Â»).
- **Layer** : outil / toolkit (Strate 6) â€” Ã  reflÃ©ter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuSQL est une unitÃ© logique pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuSQL devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md). La documentation ne gÃ©nÃ¨re pas les fichiers `mscm_index/*` ; elle dÃ©finit les concepts pour une indexation future.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |
| Toolkit Composition Contract | [Master Butler - Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md) |
| KindMother | [KindMother - Index](..//..//cores//KindMother//_index.md) ou Documentation Fondatrice |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-29  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


