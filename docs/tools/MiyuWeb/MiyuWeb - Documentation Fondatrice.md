# MiyuWeb — Documentation Fondatrice

## 1. Contexte

**MiyuWeb** est le **kit d'outils (Toolkit)** d'affichage de contenu web de l'écosystème Miyukini. Il intègre les outils de rendu HTML, d'exécution et de compilation de scripts (JS/TypeScript), de service d'assets, de résolution de thème et de layout, de validation de formulaires et de gestion d'événements, alignés sur KindMother pour la persistance des templates et assets (via MiyuSQL).

L'autorité sur les données (dont templates et assets) appartient à **KindMother** (Core de données, Strate 4). MiyuWeb expose des capacités d'exécution gouvernée (rendu, résolution thème/layout, script, asset, formulaire, événement) sans remplacer KindMother ; les Opérateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils. MiyuWeb opère sur des **données fournies dans le flux** — il ne lit pas la base directement.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :**

- L'identité et la définition canonique de MiyuWeb
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliqués)
- Le niveau de sécurité et les états système autorisés ou interdits
- La relation avec KindMother et MiyuSQL (templates/assets en base ; MiyuWeb reçoit les données dans le flux)
- L'alignement avec le protocole MIP v1 pour une future implémentation indexable

**Hors scope :**

- L'implémentation détaillée (moteur de rendu, sandbox JS, CSP)
- Toute décision de contenu ou de logique métier — celle-ci reste du ressort des Opérateurs et des Cores

---

## 3. Définition canonique

> **MiyuWeb est une composition officielle d'outils d'affichage de contenu web (rendu HTML, scripts, assets, thème, layout, formulaires, événements), déclarée et gouvernée par l'environnement.**

- MiyuWeb **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuWeb **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (rendre du HTML, résoudre un thème, exécuter/compiler un script, servir un asset, valider un formulaire, dispatcher/capturer des événements) sans décider du contenu ni accéder à la base.

**Règle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacité. Les capacités viennent exclusivement des Tools composants. Les données (templates, assets) sont fournies dans le flux ; la persistance relève de KindMother et MiyuSQL.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.web.miyuweb` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `web` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuWeb est composé des Tools suivants (format canonique `tool.web.<action>` ou `tool.web.<sous-domaine>.<action>`). Le détail de chaque outil (action, niveau de sécurité, capability_id) est décrit dans [MiyuWeb - Reference Outils](./MiyuWeb%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.web.html.render` | Rend du HTML à partir de données et de template fournis ; ne décide pas du contenu |
| `tool.web.layout.render` | Rend un layout (structure de page) à partir de données fournies |
| `tool.web.theme.resolve` | Résout le thème applicable (couleurs, styles) pour un contexte donné |
| `tool.web.script.execute` | Exécute un script (JS/TS) dans un contexte gouverné et sandboxé |
| `tool.web.script.compile` | Compile ou valide un script sans l'exécuter |
| `tool.web.asset.serve` | Sert un asset (image, CSS, etc.) à partir de données fournies dans le flux |
| `tool.web.form.validate` | Valide un formulaire (structure, champs) sans décider des règles métier |
| `tool.web.event.dispatch` | Dispatche un événement dans le flux gouverné |
| `tool.web.input.capture` | Capture une entrée utilisateur (clic, saisie) pour le flux gouverné |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuWeb en contient neuf.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : les templates et assets utilisés sont **fournis dans le flux** (MiyuWeb ne lit pas la base directement) ; persistance = KindMother. Le Toolkit est déclaré dans Master Butler et compatibilisé par Ever Buddy ([Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0, 1 ou 2** selon politique d'exposition (détail dans [MiyuWeb - Security and States Contract](./contracts/security/MiyuWeb%20-%20Security%20and%20States%20Contract.md)) ; cohérent avec WorrySentinel. Le niveau du Toolkit est au moins égal au maximum des niveaux de ses Tools composants. |
| **États autorisés** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon [Toolkit Composition Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)) |

---

## 8. Relation avec KindMother (templates et assets)

- **KindMother** est l'autorité sur toutes les données, y compris les **templates** et **assets** utilisés pour l'affichage web. Toute lecture ou écriture en base (templates, binaires assets) est **sous autorité KindMother** et **exécutée via MiyuSQL** lorsque KindMother mandate l'opération.
- **MiyuWeb** ne lit pas la base. Il reçoit les données (contenu de template, contenu ou métadonnées d'assets) **dans le flux gouverné** — par exemple après qu'elles aient été lues via MiyuSQL sous autorité KindMother, ou construites par un Opérateur. MiyuWeb exécute uniquement : rendu, résolution thème/layout, exécution/compilation de script, service d'asset, validation de formulaire, dispatch et capture d'événements.

**Référence :** [KindMother - Index](../../core/KindMother/_index.md) ou Documentation Fondatrice.

---

## 8bis. Relation avec MiyuSQL — Templates, assets

MiyuWeb et **MiyuSQL** sont deux Toolkits distincts (Strate 6) ; leurs rôles sont complémentaires et ne se recouvrent pas.

### 8bis.1 Qui manipule les templates et assets ?

| Responsabilité | Acteur | Toolkit / Core |
|----------------|--------|-----------------|
| **Autorité sur les données** (dont templates, assets) | KindMother | Core Strate 4 |
| **Persistance : lecture / écriture** (requêtes, transactions, cache) | KindMother mandate, **MiyuSQL** exécute | MiyuSQL (`tool.query.execute`, `tool.transaction.*`, `tool.cache.*`, `tool.schema.read`) |
| **Rendu, résolution thème/layout, script, asset.serve, formulaire, événements** (sans lire la base) | **MiyuWeb** exécute sur des données fournies | MiyuWeb (`tool.web.html.render`, `tool.web.layout.render`, `tool.web.theme.resolve`, etc.) |

- **KindMother** est l'autorité sur toutes les données, y compris les templates et assets. Toute lecture ou écriture en base est **sous autorité KindMother** et **exécutée via MiyuSQL** lorsque KindMother mandate l'opération.
- **MiyuWeb** ne persiste pas et ne lit pas les templates ni les assets en base. Il opère sur des **données (contenu de template, contenu ou métadonnées d'assets) qui lui sont fournies** dans le flux gouverné — par exemple après qu'elles aient été lues via MiyuSQL sous autorité KindMother, ou transmises dans la requête par un Opérateur.

### 8bis.2 Flux typique (lecture puis rendu)

1. Un Opérateur a besoin d'afficher une page web (ex. dashboard, formulaire).
2. **KindMother** (sous gouvernance) mandate une **lecture** en base (ex. récupérer un template par identifiant, ou des assets).
3. **MiyuSQL** exécute la requête (ex. `tool.query.execute` SELECT) sous autorité KindMother et retourne les données au flux.
4. Le flux fournit le template et les données à **MiyuWeb** pour **rendu** (`tool.web.html.render`, `tool.web.layout.render`), **résolution de thème** (`tool.web.theme.resolve`), ou **service d'asset** (`tool.web.asset.serve`).
5. MiyuWeb retourne le résultat (HTML, asset servi, etc.) sans accéder lui-même à la base.

MiyuWeb **ne dépend pas** de MiyuSQL (pas d'appel direct) ; la relation est **indirecte** via KindMother et le flux gouverné : les données persistées ou lues par MiyuSQL (sous KindMother) sont celles sur lesquelles MiyuWeb peut être invoqué ensuite quand elles sont fournies en entrée.

**Référence :** [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md), [MiyuSQL - KindMother Integration Contract](../MiyuSQL/contracts/integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md).

Les obligations de conformité détaillées sont dans [MiyuWeb - Tool Governance Compliance Contract](./contracts/governance/MiyuWeb%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuWeb sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `web` — cohérent avec la projection domains.json (blocs du domaine « web »).
- **Layer** : outil / toolkit (Strate 6) — à refléter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuWeb est une unité logique pouvant devenir un **bloc MSCM** à l'implémentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

À l'implémentation, le code fournissant les Tools MiyuWeb devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md). La documentation ne génère pas les fichiers `mscm_index/*` ; elle définit les concepts pour une indexation future.

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
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](../../reference/Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
