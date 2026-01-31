# MiyuContacts — Documentation Fondatrice

## 1. Contexte

**MiyuContacts** est le **kit d'outils (Toolkit)** de liste amis/ennemis et carnet d'adresses de l'écosystème Miyukini. Il intègre les outils d'ajout, de suppression et de liste des amis et ennemis (friend/foe), alignés sur [Équivalents Moteur Forum](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md).

L'autorité sur les données (amis, ennemis, carnet) appartient à **KindMother**. MiyuContacts expose des capacités d'exécution gouvernée ; les décisions (ajout autorisé, quota) relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuContacts, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'affichage (MiyuWeb) ; l'implémentation détaillée (stockage).

---

## 3. Définition canonique

> **MiyuContacts est une composition officielle d'outils de contacts (amis/ennemis, carnet d'adresses), déclarée et gouvernée par l'environnement.**

- MiyuContacts **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuContacts **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; décision (ajout autorisé) = StrongFather.

**Règle fondamentale :** Toute écriture (friend add/remove, foe add/remove) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.communication.contacts` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `communication` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuContacts - Reference Outils](./MiyuContacts%20-%20Reference%20Outils.md).

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

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décision (ajout autorisé) = StrongFather** ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 2** (données personnelles) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : amis, ennemis, carnet. Toute création ou suppression passe par **WriteIntent** vers KindMother.

Les obligations de conformité détaillées sont dans [MiyuContacts - Tool Governance Compliance Contract](./contracts/governance/MiyuContacts%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuContacts sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuContacts devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

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
