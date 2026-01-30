# MiyuHR — Documentation Fondatrice

## 1. Contexte

**MiyuHR** est le **kit d'outils (Toolkit)** RH (pointeuse, plannings) de l'écosystème Miyukini. Il intègre les outils d'enregistrement entrée/sortie (time clock) et de lecture du planning (shifts) pour un employé/période, alignés sur le document [Équivalents PoS Logiciel Caisse](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md).

L'autorité sur les données (heures, plannings) appartient à **KindMother** (Core de données, Strate 4). Les permissions (qui peut pointeuse, qui peut lire les plannings) relèvent de **Master Butler** et **StrongFather**. MiyuHR expose des capacités d'exécution gouvernée ; les Opérateurs (ex. Opérateur RH) passent par la gouvernance pour utiliser ces outils.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuHR, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother, l'alignement MIP.

**Hors scope :** l'implémentation détaillée ; la gestion des droits employés (Master Butler, StrongFather).

---

## 3. Définition canonique

> **MiyuHR est une composition officielle d'outils RH (pointeuse entrée/sortie, lecture planning), déclarée et gouvernée par l'environnement.**

- MiyuHR **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuHR **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (enregistrement entrée/sortie, lecture planning) ; les permissions = Master Butler + StrongFather.

**Règle fondamentale :** Toute écriture (time_clock.in, time_clock.out) = WriteIntent vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.hr.miyuhr` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `hr` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuHR - Reference Outils](./MiyuHR%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.hr.time_clock.in` | Enregistre une entrée (début de shift) |
| `tool.hr.time_clock.out` | Enregistre une sortie (fin de shift) |
| `tool.hr.schedule.get` | Retourne le planning (shifts) pour un employé/période |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuHR en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : permissions employé = Master Butler + StrongFather ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 2** (détail par outil dans Reference Outils) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : heures (entrées/sorties), plannings. Toute écriture (time_clock.in, time_clock.out) passe par **WriteIntent** sous autorité KindMother. MiyuHR exécute des capacités atomiques ; les permissions (qui peut pointeuse) = Master Butler + StrongFather.

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
