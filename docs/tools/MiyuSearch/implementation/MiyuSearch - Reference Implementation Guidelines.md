# MiyuSearch â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuSearch conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuSearch en logique d'implÃ©mentation (Tools indexation, requÃªte full-text, suggestions ; pas de dÃ©cision mÃ©tier sur pertinence).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuSearch (index.update, query.execute, suggest) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuSearch - Documentation Fondatrice** : ToolkitId `toolkit.search.miyusearch`, liste des Tools (index.update, query.execute, suggest), gouvernance, relation KindMother (index = dÃ©rivation des donnÃ©es sources).
- **MiyuSearch - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuSearch - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spÃ©cifiques.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

MiyuSearch est invoquÃ© aprÃ¨s dÃ©cision de la gouvernance. L'implÃ©mentation ne rÃ©-Ã©value pas les permissions. En cas d'appel hors gouvernance, refuser l'exÃ©cution et signaler.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

**RÃ¨gle fondamentale :** MiyuSearch **ne dÃ©cide pas** du pÃ©rimÃ¨tre ni du classement mÃ©tier. Les donnÃ©es Ã  indexer et les critÃ¨res de recherche sont fournis dans le flux gouvernÃ© ; les dÃ©cisions sur ce qui est Â« pertinent Â» ou affichÃ© relÃ¨vent des OpÃ©rateurs. Les Tools exÃ©cutent sur **donnÃ©es et critÃ¨res fournis** ; pas de dÃ©cision sur la pertinence mÃ©tier.

### 2.3 Index sous autoritÃ© KindMother ; pas d'Ã©criture mÃ©tier directe (BOUND-3)

- **tool.search.index.update** : Met Ã  jour l'index (document/champ fournis) ; WriteIntent ou flux gouvernÃ© vers stockage index. L'index est une **dÃ©rivation** des donnÃ©es KindMother ; pas d'Ã©criture mÃ©tier directe sur les donnÃ©es sources.
- **tool.search.query.execute** et **tool.search.suggest** : Lisent l'index pour requÃªter et suggÃ©rer ; critÃ¨res fournis dans le flux ; retour (identifiants, scores, suggestions) sans dÃ©cision mÃ©tier sur le classement final.

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant ; uniquement les ToolIds dÃ©clarÃ©s (index.update, query.execute, suggest).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **0 Ã  2** (contenu indexÃ© peut Ãªtre sensible). Ã‰tats autorisÃ©s : tous sauf restriction WorrySentinel. Ã‰tats interdits : selon politique (ex. index en lecture seule en maintenance). VÃ©rifier l'Ã©tat avant exÃ©cution.

### 2.8 Alignement MIP/MSCM

Domaine `search`, layer Strate 6. Ã€ l'implÃ©mentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution sur mandat uniquement |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision pÃ©rimÃ¨tre, pertinence, classement mÃ©tier |
| **BOUND-3** | Pas d'Ã©criture mÃ©tier directe sur sources | Index = dÃ©rivation ; mise Ã  jour index = flux gouvernÃ© / WriteIntent ; pas d'Ã©criture sur donnÃ©es sources |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement index.update, query.execute, suggest |

---

## 4. Patterns recommandÃ©s

### 4.1 Structure des Tools

Chaque ToolId = unitÃ© atomique : entrÃ©e (contexte gouvernÃ©, paramÃ¨tres : document/champ pour index.update ; critÃ¨res pour query.execute ; prÃ©fixe pour suggest), sortie (rÃ©sultat ou erreur). Pas d'Ã©tat mÃ©tier partagÃ©. Format : `tool.search.index.update`, `tool.search.query.execute`, `tool.search.suggest`.

### 4.2 Interface avec KindMother et l'index

- **index.update** : DonnÃ©es fournies dans le flux (ou provenant de KindMother en amont) â†’ mise Ã  jour de l'index via WriteIntent ou mÃ©canisme documentÃ© ; pas d'Ã©criture directe sur les tables mÃ©tier sources.
- **query.execute / suggest** : CritÃ¨res fournis dans le flux ; lecture de l'index ; retour d'identifiants/scores/suggestions. Le classement mÃ©tier et le choix de ce qui est affichÃ© restent du ressort des OpÃ©rateurs.

### 4.3 Gestion des erreurs et traÃ§abilitÃ©

Erreurs techniques (index indisponible, critÃ¨res invalides) remontÃ©es sans exposer de contenu indexÃ© sensible. Logger du Kernel pour traÃ§abilitÃ© (sans contenu des requÃªtes si sensible).

---

## 5. Alignement MIP / MSCM

- **Domaine** : `search` (toolkit.search.miyusearch).
- **Layer** : Strate 6 dans layers.json.
- **Blocs** : Chaque Tool MiyuSearch = unitÃ© logique avec `id`, `do`, `role`, `layer`. Balisage MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 6. Tests

Les tests relÃ¨vent des bonnes pratiques projet et du Tool Governance Compliance Contract. ScÃ©narios recommandÃ©s : index.update avec donnÃ©es en flux, query.execute et suggest avec critÃ¨res fournis, vÃ©rification qu'aucune dÃ©cision mÃ©tier sur pertinence n'est prise dans le kit.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuSearch - Documentation Fondatrice | [MiyuSearch - Documentation Fondatrice](../MiyuSearch%20-%20Documentation%20Fondatrice.md) |
| MiyuSearch - Reference Outils | [MiyuSearch - Reference Outils](../MiyuSearch%20-%20Reference%20Outils.md) |
| MiyuSearch - Tool Governance Compliance Contract | [MiyuSearch - Tool Governance Compliance Contract](../contracts/governance/MiyuSearch%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//..//miyukini-webway-system//reference//_index.md) |
| MIP v1 | [MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif

