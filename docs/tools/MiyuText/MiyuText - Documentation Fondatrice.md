# MiyuText â€” Documentation Fondatrice

## 1. Contexte

**MiyuText** est le **kit d'outils (Toolkit)** de traitement de texte de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de rendu markdown vers HTML, de recherche/remplacement dans des chaÃ®nes, de substitution de templates texte (placeholders), et de sanitization pour affichage sÃ©curisÃ© (XSS, Ã©chappement), sans logique mÃ©tier â€” le contenu et les options sont fournis dans le flux gouvernÃ©.

L'autoritÃ© sur le contenu mÃ©tier appartient Ã  **KindMother** et aux OpÃ©rateurs. MiyuText expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (rendre, remplacer, substituer, sanitiser) ; les dÃ©cisions sur le contenu Ã  afficher ou modifier relÃ¨vent de **StrongFather** et des OpÃ©rateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuText, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (moteur markdown, politique CSP) ; l'Ã©dition de fichiers (MiyuText opÃ¨re sur chaÃ®nes fournies).

---

## 3. DÃ©finition canonique

> **MiyuText est une composition officielle d'outils de traitement de texte (markdown, recherche/remplacement, templates, sanitization), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuText **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuText **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (rendre du markdown, remplacer, substituer des placeholders, sanitiser) ; le contenu et les options sont fournis dans le flux.

**RÃ¨gle fondamentale :** Un Tool MiyuText exÃ©cute sur des **chaÃ®nes et options fournies** ; il ne lit pas la base ni ne dÃ©cide du contenu Ã  publier.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.text.miyutext` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `text` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuText - Reference Outils](./MiyuText%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.text.markdown.render` | Rend du markdown fourni en HTML (options fournies) ; ne dÃ©cide pas du contenu |
| `tool.text.replace` | Recherche et remplacement dans une chaÃ®ne (littÃ©ral ou regex fournis) |
| `tool.text.template.apply` | Substitue des placeholders dans un template (donnÃ©es fournies) |
| `tool.text.sanitize` | Sanitise une chaÃ®ne pour affichage sÃ©curisÃ© (XSS, Ã©chappement) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuText en contient quatre.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : contenu et options fournis dans le flux ; aucune lecture base ; toute Ã©criture mÃ©tier = WriteIntent KindMother (MiyuText n'Ã©crit pas la base).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** (sanitization = sensible selon contenu) |
| **Ã‰tats autorisÃ©s** | Tous sauf restriction explicite |
| **Ã‰tats interdits** | Aucun par dÃ©faut |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur le contenu mÃ©tier. MiyuText **n'Ã©crit pas** et **ne lit pas** la base : il opÃ¨re sur des **chaÃ®nes fournies dans le flux**. Les OpÃ©rateurs (ex. MiyuCMS, MiyuWeb) rÃ©cupÃ¨rent le contenu via MiyuSQL sous autoritÃ© KindMother, puis appellent MiyuText pour rendre, remplacer ou sanitiser.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuText - Tool Governance Compliance Contract](./contracts/governance/MiyuText%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

Ã€ l'implÃ©mentation : chaque Tool MiyuText est une unitÃ© logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


