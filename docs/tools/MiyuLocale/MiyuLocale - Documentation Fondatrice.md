# MiyuLocale — Documentation Fondatrice

## 1. Contexte

**MiyuLocale** est le **kit d'outils (Toolkit)** de locale et internationalisation (i18n) de l'écosystème Miyukini. Il intègre les outils de formatage des dates/heures selon locale, de formatage des nombres selon locale, et de résolution de clés de traduction (translate), sans logique métier — la locale et le catalogue de traductions sont fournis dans le flux gouverné ; la décision sur la langue ou la politique de locale relève de **StrongFather** et des Opérateurs.

L'autorité sur les catalogues de traduction et les préférences de locale appartient à **KindMother**. MiyuLocale expose des capacités d'exécution gouvernée (formater date, formater nombre, traduire) ; les décisions sur la locale active ou le contenu des traductions relèvent de **StrongFather** et des Opérateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuLocale, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (ICU, catalogues i18n) ; la politique de langue et la gestion des traductions métier (StrongFather / Opérateurs).

---

## 3. Définition canonique

> **MiyuLocale est une composition officielle d'outils de locale et internationalisation (format date, format nombre, traduction), déclarée et gouvernée par l'environnement.**

- MiyuLocale **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuLocale **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (formater date, formater nombre, résoudre une clé de traduction) ; locale et catalogue fournis dans le flux ; pas de décision sur la politique de langue.

**Règle fondamentale :** Un Tool MiyuLocale exécute sur des **données et options fournies** (date, nombre, clé, locale, catalogue) ; il ne décide pas de la locale active ni du contenu des traductions.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.locale.miyulocale` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `locale` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuLocale - Reference Outils](./MiyuLocale%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.locale.date.format` | Formate une date/heure selon locale et options fournis |
| `tool.locale.number.format` | Formate un nombre selon locale et options fournis (devise, décimales) |
| `tool.locale.translate` | Résout une clé de traduction (catalogue fourni) ; retourne la chaîne ou clé si absent |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuLocale en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : locale et catalogue fournis dans le flux ; MiyuLocale ne lit pas la base directement (catalogue peut provenir de KindMother via MiyuSQL sous autorité KindMother).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 1** (formatage et traduction ; pas de données sensibles par défaut) |
| **États autorisés** | Tous sauf restriction explicite |
| **États interdits** | Aucun par défaut |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les catalogues de traduction et les préférences de locale. MiyuLocale **ne lit pas** la base directement : locale et catalogue sont **fournis dans le flux** (après lecture via MiyuSQL sous autorité KindMother si besoin). MiyuLocale n'écrit pas la base.

Les obligations de conformité détaillées sont dans [MiyuLocale - Tool Governance Compliance Contract](./contracts/governance/MiyuLocale%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

À l'implémentation : chaque Tool MiyuLocale est une unité logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
