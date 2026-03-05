# MiyuContacts â€” Documentation Fondatrice

## 1. Contexte

**MiyuContacts** est le **kit d'outils (Toolkit)** de liste amis/ennemis et carnet d'adresses de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils d'ajout, de suppression et de liste des amis et ennemis (friend/foe), alignÃ©s sur [Ã‰quivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (amis, ennemis, carnet) appartient Ã  **KindMother**. MiyuContacts expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les dÃ©cisions (ajout autorisÃ©, quota) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuContacts, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'affichage (MiyuWeb) ; l'implÃ©mentation dÃ©taillÃ©e (stockage).

---

## 3. DÃ©finition canonique

> **MiyuContacts est une composition officielle d'outils de contacts (amis/ennemis, carnet d'adresses), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuContacts **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuContacts **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; dÃ©cision (ajout autorisÃ©) = StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (friend add/remove, foe add/remove) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.communication.contacts` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `communication` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuContacts - Reference Outils](./MiyuContacts%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.contacts.friend.add` | Ajoute un ami ; autorisation = StrongFather |
| `tool.contacts.friend.remove` | Supprime un ami |
| `tool.contacts.friend.list` | Liste les amis |
| `tool.contacts.foe.add` | Ajoute un ennemi ; autorisation = StrongFather |
| `tool.contacts.foe.remove` | Supprime un ennemi |
| `tool.contacts.foe.list` | Liste les ennemis |
| `tool.contacts.list` | Liste les contacts (type fourni : friend, foe, ou mixte) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuContacts en contient sept.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision (ajout autorisÃ©) = StrongFather** ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  2** (donnÃ©es personnelles) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : amis, ennemis, carnet. Toute crÃ©ation ou suppression passe par **WriteIntent** vers KindMother.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuContacts - Tool Governance Compliance Contract](./contracts/governance/MiyuContacts%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuContacts sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuContacts devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


