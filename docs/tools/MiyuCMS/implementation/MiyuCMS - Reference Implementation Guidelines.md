# MiyuCMS â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuCMS conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuCMS en logique d'implÃ©mentation (contenus, rÃ©visions, commentaires, mÃ©dias ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuCMS (gestion de contenu Ã©ditorial : crÃ©ation, mise Ã  jour, publication, planification, rÃ©visions, commentaires, mÃ©dias) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuCMS - Documentation Fondatrice** : ToolkitId `toolkit.content.cms`, liste des Tools (content.*, revision.*, comment.*, media.*).
- **MiyuCMS - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuCMS - Tool Governance Compliance Contract** : Obligations spÃ©cifiques (dÃ©cision StrongFather pour publication/modÃ©ration, WriteIntent KindMother).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

MiyuCMS est invoquÃ© uniquement aprÃ¨s dÃ©cision StrongFather (publication, modÃ©ration, restauration rÃ©vision). Ne pas rÃ©-Ã©valuer les permissions. En cas d'appel hors gouvernance, refuser l'exÃ©cution et signaler.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Tools exÃ©cutent sur les donnÃ©es fournies ; aucune dÃ©cision sur la politique Ã©ditoriale, la modÃ©ration ou la visibilitÃ©. RÃ¨gles fournies par StrongFather / KindMother ou dans le flux.

### 2.3 Toute Ã©criture = WriteIntent KindMother (BOUND-3)

Toute crÃ©ation/mise Ã  jour (contenu, rÃ©vision, commentaire, mÃ©dia) = **WriteIntent** vers KindMother. Aucun accÃ¨s direct Ã  la base. Lectures (revision.list, revision.compare, comment.list, media.serve) sur donnÃ©es fournies dans le flux.

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds dÃ©clarÃ©s (content.*, revision.*, comment.*, media.upload/serve/transform).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **0 Ã  2** selon politique d'exposition. Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED`. VÃ©rifier WorrySentinel / Caring Nanny avant exÃ©cution. L'affichage des contenus est du ressort de MiyuWeb (donnÃ©es fournies dans le flux).

### 2.8 Alignement MIP/MSCM

Domaine `content`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision publication, modÃ©ration, politique Ã©ditoriale |
| **BOUND-3** | Pas d'accÃ¨s direct | Toute Ã©criture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement content.*, revision.*, comment.*, media.* dÃ©clarÃ©s |

---

## 4. Gestion des erreurs et traÃ§abilitÃ©

- Erreurs techniques (paramÃ¨tres invalides, WriteIntent refusÃ©e) remontÃ©es sans exposer de contenu ou donnÃ©es sensibles.
- En cas de violation de bornage, refus d'exÃ©cution et signal.
- Logger du Kernel pour la traÃ§abilitÃ© (sans contenu mÃ©tier).

---

## 5. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| MiyuCMS - Documentation Fondatrice | [Documentation Fondatrice](../MiyuCMS%20-%20Documentation%20Fondatrice.md) |
| MiyuCMS - Reference Outils | [Reference Outils](../MiyuCMS%20-%20Reference%20Outils.md) |
| MiyuCMS - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuCMS%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif

