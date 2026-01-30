# MiyuTreasury — Documentation Fondatrice

## 1. Contexte

**MiyuTreasury** est le **kit d'outils (Toolkit)** de trésorerie et prévisionnel (tableau de bord, prévisionnel, alertes) de l'écosystème Miyukini. Il intègre les outils d'agrégation des indicateurs pour le tableau de bord, de calcul du prévisionnel et de vérification des seuils et échéances, alignés sur [Équivalents Comptabilité Indépendants](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Comptabilite%20Independants.md).

Les données sous-jacentes (écritures, factures, échéances) appartiennent à **KindMother**. MiyuTreasury expose des capacités de **lecture agrégée** et de **vérification** (alertes) ; les règles d'alerte relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuTreasury, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (modèles prévisionnels) ; la tenue des livres (voir MiyuComptaLedger).

---

## 3. Définition canonique

> **MiyuTreasury est une composition officielle d'outils de trésorerie et prévisionnel (tableau de bord, prévisionnel, alertes), déclarée et gouvernée par l'environnement.**

- MiyuTreasury **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuTreasury **n'ajoute aucune logique métier** : il orchestre des capacités atomiques d'agrégation et de vérification ; règles alertes = StrongFather.

**Règle fondamentale :** Les Tools **lisent** les données KindMother (écritures, factures, échéances) ; pas d'écriture métier (sauf paramètres alertes si définis).

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.treasury.forecast` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `treasury` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuTreasury - Reference Outils](./MiyuTreasury%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.treasury.dashboard.aggregate` | Agrège les indicateurs pour le tableau de bord trésorerie |
| `tool.treasury.forecast.compute` | Calcule un prévisionnel à partir de données fournies |
| `tool.treasury.alert.check` | Vérifie les seuils et échéances (exécution ; règles = StrongFather) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuTreasury en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : règles alertes = StrongFather ; les Tools lisent les données KindMother (pas d'écriture métier sauf paramètres alertes).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 2** (données trésorerie sensibles) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : écritures, factures, échéances. Les Tools MiyuTreasury **lisent** ces données pour agrégation et prévisionnel ; ils n'écrivent pas (sauf paramètres alertes si définis). Règles alertes = StrongFather.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents Comptabilité Indépendants | [Miyukini Conceptual References - Equivalents Comptabilite Independants](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Comptabilite%20Independants.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
