# MiyuStory â€” Documentation Fondatrice

## 1. Contexte

**MiyuStory** est le **kit d'outils (Toolkit)** de contenu Ã©phÃ©mÃ¨re (stories 24h) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de crÃ©ation, de liste et de rÃ©action aux stories, alignÃ©s sur [Ã‰quivalents Reseaux Sociaux](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (stories, rÃ©actions, expiration) appartient Ã  **KindMother**. MiyuStory expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les dÃ©cisions (crÃ©ation autorisÃ©e, visibilitÃ©, durÃ©e) relÃ¨vent de **StrongFather**. Ever Buddy gouverne le cycle de vie (expiration 24h).

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuStory, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'affichage (MiyuWeb) ; l'implÃ©mentation dÃ©taillÃ©e (expiration, purge).

---

## 3. DÃ©finition canonique

> **MiyuStory est une composition officielle d'outils de contenu Ã©phÃ©mÃ¨re (stories 24h), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuStory **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuStory **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; dÃ©cision (crÃ©ation, visibilitÃ©) = StrongFather ; cycle de vie (expiration) = Ever Buddy.

**RÃ¨gle fondamentale :** Toute Ã©criture (story, rÃ©action) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.social.story` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `social` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuStory - Reference Outils](./MiyuStory%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.social.story.create` | CrÃ©e une story ; autorisation = StrongFather |
| `tool.social.story.list` | Liste les stories (filtres, pagination fournis) |
| `tool.social.story.get` | RÃ©cupÃ¨re une story |
| `tool.social.story.reaction.add` | Ajoute une rÃ©action Ã  une story |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuStory en contient quatre.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision (crÃ©ation, visibilitÃ©) = StrongFather** ; **cycle de vie (expiration 24h) = Ever Buddy** ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  2** (contenu Ã©phÃ©mÃ¨re) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : stories, rÃ©actions. Toute crÃ©ation passe par **WriteIntent** vers KindMother. Expiration et purge relÃ¨vent du cycle de vie (Ever Buddy) et de l'implÃ©mentation.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuStory - Tool Governance Compliance Contract](./contracts/governance/MiyuStory%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuStory sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuStory devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Reseaux Sociaux | [Miyukini Conceptual References - Equivalents Reseaux Sociaux](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


