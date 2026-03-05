# MiyuLocale â€” Documentation Fondatrice

## 1. Contexte

**MiyuLocale** est le **kit d'outils (Toolkit)** de locale et internationalisation (i18n) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de formatage des dates/heures selon locale, de formatage des nombres selon locale, et de rÃ©solution de clÃ©s de traduction (translate), sans logique mÃ©tier â€” la locale et le catalogue de traductions sont fournis dans le flux gouvernÃ© ; la dÃ©cision sur la langue ou la politique de locale relÃ¨ve de **StrongFather** et des OpÃ©rateurs.

L'autoritÃ© sur les catalogues de traduction et les prÃ©fÃ©rences de locale appartient Ã  **KindMother**. MiyuLocale expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (formater date, formater nombre, traduire) ; les dÃ©cisions sur la locale active ou le contenu des traductions relÃ¨vent de **StrongFather** et des OpÃ©rateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuLocale, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (ICU, catalogues i18n) ; la politique de langue et la gestion des traductions mÃ©tier (StrongFather / OpÃ©rateurs).

---

## 3. DÃ©finition canonique

> **MiyuLocale est une composition officielle d'outils de locale et internationalisation (format date, format nombre, traduction), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuLocale **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuLocale **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (formater date, formater nombre, rÃ©soudre une clÃ© de traduction) ; locale et catalogue fournis dans le flux ; pas de dÃ©cision sur la politique de langue.

**RÃ¨gle fondamentale :** Un Tool MiyuLocale exÃ©cute sur des **donnÃ©es et options fournies** (date, nombre, clÃ©, locale, catalogue) ; il ne dÃ©cide pas de la locale active ni du contenu des traductions.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.locale.miyulocale` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `locale` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuLocale - Reference Outils](./MiyuLocale%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.locale.date.format` | Formate une date/heure selon locale et options fournis |
| `tool.locale.number.format` | Formate un nombre selon locale et options fournis (devise, dÃ©cimales) |
| `tool.locale.translate` | RÃ©sout une clÃ© de traduction (catalogue fourni) ; retourne la chaÃ®ne ou clÃ© si absent |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuLocale en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : locale et catalogue fournis dans le flux ; MiyuLocale ne lit pas la base directement (catalogue peut provenir de KindMother via MiyuSQL sous autoritÃ© KindMother).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  1** (formatage et traduction ; pas de donnÃ©es sensibles par dÃ©faut) |
| **Ã‰tats autorisÃ©s** | Tous sauf restriction explicite |
| **Ã‰tats interdits** | Aucun par dÃ©faut |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les catalogues de traduction et les prÃ©fÃ©rences de locale. MiyuLocale **ne lit pas** la base directement : locale et catalogue sont **fournis dans le flux** (aprÃ¨s lecture via MiyuSQL sous autoritÃ© KindMother si besoin). MiyuLocale n'Ã©crit pas la base.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuLocale - Tool Governance Compliance Contract](./contracts/governance/MiyuLocale%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

Ã€ l'implÃ©mentation : chaque Tool MiyuLocale est une unitÃ© logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

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


