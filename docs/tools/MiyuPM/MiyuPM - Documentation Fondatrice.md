# MiyuPM â€” Documentation Fondatrice

## 1. Contexte

**MiyuPM** est le **kit d'outils (Toolkit)** de messagerie privÃ©e (envoi, dossiers, brouillons, conversation, export) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils d'envoi, de liste, de gestion des dossiers et brouillons, et d'export des MP, alignÃ©s sur [Ã‰quivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (messages, dossiers, brouillons, conversations) appartient Ã  **KindMother**. MiyuPM expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les dÃ©cisions (envoi autorisÃ©, destinataires, quotas) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuPM, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** les piÃ¨ces jointes (MiyuMedia) ; l'anti-spam / flood (MiyuAntiSpam) ; l'implÃ©mentation dÃ©taillÃ©e (stockage).

---

## 3. DÃ©finition canonique

> **MiyuPM est une composition officielle d'outils de messagerie privÃ©e (envoi, dossiers, brouillons, conversation, export), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuPM **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuPM **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; dÃ©cision d'envoi = StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (message, dossier, brouillon) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.communication.pm` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `communication` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuPM - Reference Outils](./MiyuPM%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.pm.send` | Envoie un message privÃ© (destinataire, contenu fournis) ; autorisation = StrongFather |
| `tool.pm.list` | Liste les messages (dossier, filtres fournis) |
| `tool.pm.get` | RÃ©cupÃ¨re un message |
| `tool.pm.folder.list` | Liste les dossiers (inbox, sent, etc.) |
| `tool.pm.folder.create` | CrÃ©e un dossier personnalisÃ© |
| `tool.pm.folder.update` | Met Ã  jour un dossier |
| `tool.pm.draft.create` | CrÃ©e un brouillon |
| `tool.pm.draft.update` | Met Ã  jour un brouillon |
| `tool.pm.draft.list` | Liste les brouillons |
| `tool.pm.conversation.list` | Liste les conversations |
| `tool.pm.conversation.get` | RÃ©cupÃ¨re une conversation (fil de messages) |
| `tool.pm.export` | Exporte les messages (format fourni) ; exÃ©cution seule |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPM en contient douze.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision d'envoi = StrongFather** ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **2** (donnÃ©es personnelles, messagerie) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : messages, dossiers, brouillons, conversations. Toute crÃ©ation, mise Ã  jour ou suppression passe par **WriteIntent** vers KindMother.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuPM - Tool Governance Compliance Contract](./contracts/governance/MiyuPM%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuPM sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuPM devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


