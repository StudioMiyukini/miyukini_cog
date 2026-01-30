# MiyuPosAnalytics — Documentation Fondatrice

## 1. Contexte

**MiyuPosAnalytics** est le **kit d'outils (Toolkit)** d'analytics ventes PoS de l'écosystème Miyukini. Il intègre les outils de tendance ventes, ventes par article, ventes par employé, écart caisse pour un shift, rapport taxes, clôture shift caisse et export tableur, alignés sur le document [Équivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md).

Les données sources (ventes, reçus, shifts) relèvent de **KindMother**. MiyuPosAnalytics expose des capacités d'agrégation et de rapport en lecture (ou exécution gouvernée pour shift.close) ; les Opérateurs (ex. Opérateur Analytics) passent par la gouvernance pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuPosAnalytics, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implémentation détaillée ; la décision de clôture shift (StrongFather).

---

## 3. Définition canonique

> **MiyuPosAnalytics est une composition officielle d'outils d'analytics ventes (tendances, par article, par employé, écart caisse, taxes, clôture shift, export tableur), déclarée et gouvernée par l'environnement.**

- MiyuPosAnalytics **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuPosAnalytics **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (agrégation, rapport, export) ; la décision de clôture shift appartient à StrongFather.

**Règle fondamentale :** Les Tools analytics opèrent sur des données fournies dans le flux ou lues sous autorité KindMother ; toute écriture (shift.close) = WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.pos.miyuposanalytics` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `pos` / `analytics` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuPosAnalytics - Reference Outils](./MiyuPosAnalytics%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.analytics.sales.trend` | Retourne tendance ventes (période, comparaison) |
| `tool.analytics.sales.by_item` | Retourne ventes par article (top N, filtres) |
| `tool.analytics.sales.by_employee` | Retourne les ventes agrégées par employé |
| `tool.analytics.cash.discrepancy` | Retourne l'écart caisse pour un shift |
| `tool.analytics.tax.report` | Retourne rapport taxes (période, filtres) |
| `tool.pos.shift.close` | Clôture un shift caisse (comptage, écart) ; autorisation = StrongFather |
| `tool.data.export.spreadsheet` | Exporte des données en format tableur (données fournies) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPosAnalytics en contient sept.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : toute persistance (shift.close) = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** (détail par outil dans Reference Outils) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données sources (ventes, reçus, shifts). Les Tools analytics (trend, by_item, by_employee, cash.discrepancy, tax.report, export) opèrent en lecture ou sur données fournies ; `tool.pos.shift.close` écrit sous WriteIntent KindMother après autorisation StrongFather.

---

## 9. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents PoS Logiciel Caisse | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
