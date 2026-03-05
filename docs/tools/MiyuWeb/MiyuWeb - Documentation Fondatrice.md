# MiyuWeb â€” Documentation Fondatrice

## 1. Contexte

**MiyuWeb** est le **kit d'outils (Toolkit)** d'affichage de contenu web de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de rendu HTML, d'exÃ©cution et de compilation de scripts (JS/TypeScript), de service d'assets, de rÃ©solution de thÃ¨me et de layout, de validation de formulaires et de gestion d'Ã©vÃ©nements, alignÃ©s sur KindMother pour la persistance des templates et assets (via MiyuSQL).

L'autoritÃ© sur les donnÃ©es (dont templates et assets) appartient Ã  **KindMother** (Core de donnÃ©es, Strate 4). MiyuWeb expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (rendu, rÃ©solution thÃ¨me/layout, script, asset, formulaire, Ã©vÃ©nement) sans remplacer KindMother ; les OpÃ©rateurs passent par la gouvernance (BondingBrother, Master Butler, StrongFather, WorrySentinel, Caring Nanny) pour utiliser ces outils. MiyuWeb opÃ¨re sur des **donnÃ©es fournies dans le flux** â€” il ne lit pas la base directement.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :**

- L'identitÃ© et la dÃ©finition canonique de MiyuWeb
- Le **ToolkitId** et le catalogue (Master Butler)
- La liste des **outils composants** (ToolIds)
- La gouvernance (flux d'appel, Cores impliquÃ©s)
- Le niveau de sÃ©curitÃ© et les Ã©tats systÃ¨me autorisÃ©s ou interdits
- La relation avec KindMother et MiyuSQL (templates/assets en base ; MiyuWeb reÃ§oit les donnÃ©es dans le flux)
- L'alignement avec le protocole MIP v1 pour une future implÃ©mentation indexable

**Hors scope :**

- L'implÃ©mentation dÃ©taillÃ©e (moteur de rendu, sandbox JS, CSP)
- Toute dÃ©cision de contenu ou de logique mÃ©tier â€” celle-ci reste du ressort des OpÃ©rateurs et des Cores

---

## 3. DÃ©finition canonique

> **MiyuWeb est une composition officielle d'outils d'affichage de contenu web (rendu HTML, scripts, assets, thÃ¨me, layout, formulaires, Ã©vÃ©nements), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuWeb **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuWeb **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (rendre du HTML, rÃ©soudre un thÃ¨me, exÃ©cuter/compiler un script, servir un asset, valider un formulaire, dispatcher/capturer des Ã©vÃ©nements) sans dÃ©cider du contenu ni accÃ©der Ã  la base.

**RÃ¨gle fondamentale :** Un Kit d'Outils orchestre, mais n'ajoute pas de capacitÃ©. Les capacitÃ©s viennent exclusivement des Tools composants. Les donnÃ©es (templates, assets) sont fournies dans le flux ; la persistance relÃ¨ve de KindMother et MiyuSQL.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.web.miyuweb` |
| **Format** | `toolkit.<domain>.<name>` (conforme au [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md)) |
| **Domaine** | `web` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants ; toute utilisation passe par le catalogue et la gouvernance. |

---

## 5. Liste des outils composants

MiyuWeb est composÃ© des Tools suivants (format canonique `tool.web.<action>` ou `tool.web.<sous-domaine>.<action>`). Le dÃ©tail de chaque outil (action, niveau de sÃ©curitÃ©, capability_id) est dÃ©crit dans [MiyuWeb - Reference Outils](./MiyuWeb%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.web.html.render` | Rend du HTML Ã  partir de donnÃ©es et de template fournis ; ne dÃ©cide pas du contenu |
| `tool.web.layout.render` | Rend un layout (structure de page) Ã  partir de donnÃ©es fournies |
| `tool.web.theme.resolve` | RÃ©sout le thÃ¨me applicable (couleurs, styles) pour un contexte donnÃ© |
| `tool.web.script.execute` | ExÃ©cute un script (JS/TS) dans un contexte gouvernÃ© et sandboxÃ© |
| `tool.web.script.compile` | Compile ou valide un script sans l'exÃ©cuter |
| `tool.web.asset.serve` | Sert un asset (image, CSS, etc.) Ã  partir de donnÃ©es fournies dans le flux |
| `tool.web.form.validate` | Valide un formulaire (structure, champs) sans dÃ©cider des rÃ¨gles mÃ©tier |
| `tool.web.event.dispatch` | Dispatche un Ã©vÃ©nement dans le flux gouvernÃ© |
| `tool.web.input.capture` | Capture une entrÃ©e utilisateur (clic, saisie) pour le flux gouvernÃ© |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuWeb en contient neuf.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : les templates et assets utilisÃ©s sont **fournis dans le flux** (MiyuWeb ne lit pas la base directement) ; persistance = KindMother. Le Toolkit est dÃ©clarÃ© dans Master Butler et compatibilisÃ© par Ever Buddy ([Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0, 1 ou 2** selon politique d'exposition (dÃ©tail dans [MiyuWeb - Security and States Contract](./contracts/security/MiyuWeb%20-%20Security%20and%20States%20Contract.md)) ; cohÃ©rent avec WorrySentinel. Le niveau du Toolkit est au moins Ã©gal au maximum des niveaux de ses Tools composants. |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` (selon politique Caring Nanny) |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` (et autres selon [Toolkit Composition Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Toolkit%20Composition%20Contract.md)) |

---

## 8. Relation avec KindMother (templates et assets)

- **KindMother** est l'autoritÃ© sur toutes les donnÃ©es, y compris les **templates** et **assets** utilisÃ©s pour l'affichage web. Toute lecture ou Ã©criture en base (templates, binaires assets) est **sous autoritÃ© KindMother** et **exÃ©cutÃ©e via MiyuSQL** lorsque KindMother mandate l'opÃ©ration.
- **MiyuWeb** ne lit pas la base. Il reÃ§oit les donnÃ©es (contenu de template, contenu ou mÃ©tadonnÃ©es d'assets) **dans le flux gouvernÃ©** â€” par exemple aprÃ¨s qu'elles aient Ã©tÃ© lues via MiyuSQL sous autoritÃ© KindMother, ou construites par un OpÃ©rateur. MiyuWeb exÃ©cute uniquement : rendu, rÃ©solution thÃ¨me/layout, exÃ©cution/compilation de script, service d'asset, validation de formulaire, dispatch et capture d'Ã©vÃ©nements.

**RÃ©fÃ©rence :** [KindMother - Index](..//..//cores//KindMother//_index.md) ou Documentation Fondatrice.

---

## 8bis. Relation avec MiyuSQL â€” Templates, assets

MiyuWeb et **MiyuSQL** sont deux Toolkits distincts (Strate 6) ; leurs rÃ´les sont complÃ©mentaires et ne se recouvrent pas.

### 8bis.1 Qui manipule les templates et assets ?

| ResponsabilitÃ© | Acteur | Toolkit / Core |
|----------------|--------|-----------------|
| **AutoritÃ© sur les donnÃ©es** (dont templates, assets) | KindMother | Core Strate 4 |
| **Persistance : lecture / Ã©criture** (requÃªtes, transactions, cache) | KindMother mandate, **MiyuSQL** exÃ©cute | MiyuSQL (`tool.query.execute`, `tool.transaction.*`, `tool.cache.*`, `tool.schema.read`) |
| **Rendu, rÃ©solution thÃ¨me/layout, script, asset.serve, formulaire, Ã©vÃ©nements** (sans lire la base) | **MiyuWeb** exÃ©cute sur des donnÃ©es fournies | MiyuWeb (`tool.web.html.render`, `tool.web.layout.render`, `tool.web.theme.resolve`, etc.) |

- **KindMother** est l'autoritÃ© sur toutes les donnÃ©es, y compris les templates et assets. Toute lecture ou Ã©criture en base est **sous autoritÃ© KindMother** et **exÃ©cutÃ©e via MiyuSQL** lorsque KindMother mandate l'opÃ©ration.
- **MiyuWeb** ne persiste pas et ne lit pas les templates ni les assets en base. Il opÃ¨re sur des **donnÃ©es (contenu de template, contenu ou mÃ©tadonnÃ©es d'assets) qui lui sont fournies** dans le flux gouvernÃ© â€” par exemple aprÃ¨s qu'elles aient Ã©tÃ© lues via MiyuSQL sous autoritÃ© KindMother, ou transmises dans la requÃªte par un OpÃ©rateur.

### 8bis.2 Flux typique (lecture puis rendu)

1. Un OpÃ©rateur a besoin d'afficher une page web (ex. dashboard, formulaire).
2. **KindMother** (sous gouvernance) mandate une **lecture** en base (ex. rÃ©cupÃ©rer un template par identifiant, ou des assets).
3. **MiyuSQL** exÃ©cute la requÃªte (ex. `tool.query.execute` SELECT) sous autoritÃ© KindMother et retourne les donnÃ©es au flux.
4. Le flux fournit le template et les donnÃ©es Ã  **MiyuWeb** pour **rendu** (`tool.web.html.render`, `tool.web.layout.render`), **rÃ©solution de thÃ¨me** (`tool.web.theme.resolve`), ou **service d'asset** (`tool.web.asset.serve`).
5. MiyuWeb retourne le rÃ©sultat (HTML, asset servi, etc.) sans accÃ©der lui-mÃªme Ã  la base.

MiyuWeb **ne dÃ©pend pas** de MiyuSQL (pas d'appel direct) ; la relation est **indirecte** via KindMother et le flux gouvernÃ© : les donnÃ©es persistÃ©es ou lues par MiyuSQL (sous KindMother) sont celles sur lesquelles MiyuWeb peut Ãªtre invoquÃ© ensuite quand elles sont fournies en entrÃ©e.

**RÃ©fÃ©rence :** [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md), [MiyuSQL - KindMother Integration Contract](../MiyuSQL/contracts/integration/MiyuSQL%20-%20KindMother%20Integration%20Contract.md).

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuWeb - Tool Governance Compliance Contract](./contracts/governance/MiyuWeb%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuWeb sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol) :

- **Domaine** : `web` â€” cohÃ©rent avec la projection domains.json (blocs du domaine Â« web Â»).
- **Layer** : outil / toolkit (Strate 6) â€” Ã  reflÃ©ter dans layers.json lorsque le code existera.
- **Blocs** : chaque Tool MiyuWeb est une unitÃ© logique pouvant devenir un **bloc MSCM** Ã  l'implÃ©mentation : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuWeb devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md). La documentation ne gÃ©nÃ¨re pas les fichiers `mscm_index/*` ; elle dÃ©finit les concepts pour une indexation future.

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
| MiyuSQL - Documentation Fondatrice | [MiyuSQL - Documentation Fondatrice](../MiyuSQL/MiyuSQL%20-%20Documentation%20Fondatrice.md) |
| Pyramide Architecture | [Miyukini Conceptual References - Pyramide Architecture Complete](..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


