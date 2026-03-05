# MiyuBookmarks â€” Documentation Fondatrice

## 1. Contexte

**MiyuBookmarks** est le **kit d'outils (Toolkit)** de signets (topics ou entitÃ©s gÃ©nÃ©riques) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils d'ajout, de suppression et de liste des signets, alignÃ©s sur [Ã‰quivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (signets, cibles, utilisateur) appartient Ã  **KindMother**. MiyuBookmarks expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les dÃ©cisions (ajout autorisÃ©, quota) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuBookmarks, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'affichage des signets (MiyuWeb) ; l'implÃ©mentation dÃ©taillÃ©e (types de cibles).

---

## 3. DÃ©finition canonique

> **MiyuBookmarks est une composition officielle d'outils de signets (ajout, suppression, liste), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuBookmarks **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuBookmarks **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; dÃ©cision (ajout autorisÃ©) = StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (add, remove) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.content.bookmarks` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `content` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuBookmarks - Reference Outils](./MiyuBookmarks%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.bookmark.add` | Ajoute un signet (cible = topic ou entitÃ© fournie) ; autorisation = StrongFather |
| `tool.bookmark.remove` | Supprime un signet |
| `tool.bookmark.list` | Liste les signets (utilisateur, filtres fournis) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuBookmarks en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision (ajout autorisÃ©, quota) = StrongFather** ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1** (donnÃ©es utilisateur) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : signets, cibles, utilisateur. Toute crÃ©ation ou suppression passe par **WriteIntent** vers KindMother.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuBookmarks - Tool Governance Compliance Contract](./contracts/governance/MiyuBookmarks%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuBookmarks sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuBookmarks devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Moteur Forum | [Miyukini Conceptual References - Equivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


