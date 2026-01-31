# MiyuBookmarks — Documentation Fondatrice

## 1. Contexte

**MiyuBookmarks** est le **kit d'outils (Toolkit)** de signets (topics ou entités génériques) de l'écosystème Miyukini. Il intègre les outils d'ajout, de suppression et de liste des signets, alignés sur [Équivalents Moteur Forum](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md).

L'autorité sur les données (signets, cibles, utilisateur) appartient à **KindMother**. MiyuBookmarks expose des capacités d'exécution gouvernée ; les décisions (ajout autorisé, quota) relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuBookmarks, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'affichage des signets (MiyuWeb) ; l'implémentation détaillée (types de cibles).

---

## 3. Définition canonique

> **MiyuBookmarks est une composition officielle d'outils de signets (ajout, suppression, liste), déclarée et gouvernée par l'environnement.**

- MiyuBookmarks **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuBookmarks **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; décision (ajout autorisé) = StrongFather.

**Règle fondamentale :** Toute écriture (add, remove) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.content.bookmarks` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `content` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuBookmarks - Reference Outils](./MiyuBookmarks%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.bookmark.add` | Ajoute un signet (cible = topic ou entité fournie) ; autorisation = StrongFather |
| `tool.bookmark.remove` | Supprime un signet |
| `tool.bookmark.list` | Liste les signets (utilisateur, filtres fournis) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuBookmarks en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décision (ajout autorisé, quota) = StrongFather** ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1** (données utilisateur) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : signets, cibles, utilisateur. Toute création ou suppression passe par **WriteIntent** vers KindMother.

Les obligations de conformité détaillées sont dans [MiyuBookmarks - Tool Governance Compliance Contract](./contracts/governance/MiyuBookmarks%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuBookmarks sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuBookmarks devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents Moteur Forum | [Miyukini Conceptual References - Equivalents Moteur Forum](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
