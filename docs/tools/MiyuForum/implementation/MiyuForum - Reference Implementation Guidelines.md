# MiyuForum â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuForum conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuForum en logique d'implÃ©mentation (Tools structure forum, gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuForum (catÃ©gories, forums, topics, posts, suivi lu, export) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuForum - Documentation Fondatrice** : ToolkitId `toolkit.community.forum`, liste des Tools (category, board, topic, post, readtrack, export), gouvernance, relation KindMother.
- **MiyuForum - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuForum - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spÃ©cifiques.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

MiyuForum est invoquÃ© uniquement aprÃ¨s dÃ©cision StrongFather (crÃ©ation topic/post, sticky, annonce). L'implÃ©mentation ne doit pas rÃ©-Ã©valuer les permissions. En cas d'appel hors gouvernance, refuser l'exÃ©cution et signaler.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Tools (category.*, board.*, topic.*, post.*, readtrack.*, topic.export.*) exÃ©cutent sur les donnÃ©es fournies ; aucune dÃ©cision sur le contenu, la visibilitÃ© ou les rÃ¨gles de modÃ©ration (MiyuModerationForum).

### 2.3 Toute Ã©criture = WriteIntent KindMother (BOUND-3)

**RÃ¨gle fondamentale MiyuForum :** Toute crÃ©ation, mise Ã  jour ou suppression (category, board, topic, post, readtrack) = **WriteIntent** vers KindMother. Aucun accÃ¨s direct Ã  la base ; les Ã©critures passent par le flux gouvernÃ©.

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds dÃ©clarÃ©s (tool.forum.category.*, tool.forum.board.*, tool.forum.topic.*, tool.forum.post.*, tool.forum.readtrack.*, tool.forum.topic.export.*).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **1 Ã  2** (contenu communautaire). Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED`. Ã‰tats interdits : `SECURITY_LOCKDOWN`, `MAINTENANCE`. VÃ©rifier l'Ã©tat (Caring Nanny / WorrySentinel) avant exÃ©cution.

### 2.8 Alignement MIP/MSCM

Domaine `community`, layer Strate 6. Ã€ l'implÃ©mentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision contenu, visibilitÃ©, modÃ©ration |
| **BOUND-3** | Pas d'accÃ¨s direct | Toute Ã©criture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement ToolIds dÃ©clarÃ©s (category, board, topic, post, readtrack, topic.export) |

---

## 4. Patterns recommandÃ©s

### 4.1 Structure des Tools

Chaque ToolId (`tool.forum.category.list|get|create|update`, `tool.forum.board.*`, `tool.forum.topic.*`, `tool.forum.post.*`, `tool.forum.readtrack.*`, `tool.forum.topic.export.*`) = unitÃ© d'exÃ©cution atomique : entrÃ©e (contexte gouvernÃ©, paramÃ¨tres : ids, payload), sortie (rÃ©sultat ou erreur). Pas d'Ã©tat mÃ©tier partagÃ© entre appels.

### 4.2 Interface avec KindMother

Toute Ã©criture (crÃ©ation/mise Ã  jour/suppression de category, board, topic, post, readtrack) produit une **WriteIntent** vers KindMother. Les lectures (list, get) s'appuient sur des donnÃ©es fournies dans le flux (lues en amont par MiyuSQL sous KindMother) ou sur un contrat d'intÃ©gration documentÃ©. MiyuForum n'accÃ¨de pas directement Ã  la base.

### 4.3 Gestion des erreurs et traÃ§abilitÃ©

Erreurs techniques (paramÃ¨tres invalides, WriteIntent refusÃ©e) remontÃ©es sans exposer de donnÃ©es sensibles. En cas de violation de bornage, refus d'exÃ©cution et signal. Utiliser le Logger du Kernel pour la traÃ§abilitÃ© (sans contenu mÃ©tier).

### 4.4 Export topic

`tool.forum.topic.export.*` exÃ©cute la gÃ©nÃ©ration (PDF, texte) sur les donnÃ©es fournies dans le flux ; pas d'Ã©criture mÃ©tier ; rÃ©sultat retournÃ© dans le flux.

---

## 5. Alignement MIP / MSCM

- **Domaine** : `community` (cohÃ©rent avec toolkit.community.forum).
- **Layer** : Strate 6 dans layers.json.
- **Blocs** : Chaque Tool MiyuForum est une unitÃ© logique avec `id`, `do`, `role`, `layer` pour alimenter blocks.json. Balisage MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 6. Tests

Les tests relÃ¨vent des bonnes pratiques projet et du [MiyuForum - Tool Governance Compliance Contract](../contracts/governance/MiyuForum%20-%20Tool%20Governance%20Compliance%20Contract.md) (obligations de non-rÃ©gression, Ã©tats). ScÃ©narios recommandÃ©s : crÃ©ation topic/post via WriteIntent, list/get avec donnÃ©es en flux, readtrack mark read, export sans Ã©criture mÃ©tier.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuForum - Documentation Fondatrice | [MiyuForum - Documentation Fondatrice](../MiyuForum%20-%20Documentation%20Fondatrice.md) |
| MiyuForum - Reference Outils | [MiyuForum - Reference Outils](../MiyuForum%20-%20Reference%20Outils.md) |
| MiyuForum - Tool Governance Compliance Contract | [MiyuForum - Tool Governance Compliance Contract](../contracts/governance/MiyuForum%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//..//miyukini-webway-system//reference//_index.md) |
| MIP v1 | [MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif

