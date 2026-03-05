# MiyuSocialMessaging â€” Documentation Fondatrice

## 1. Contexte

**MiyuSocialMessaging** est le **kit d'outils (Toolkit)** de messagerie sociale (DMs, conversations, rÃ©actions, marques de lecture) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils d'envoi de DM, de liste des conversations, de rÃ©actions et de marques de lecture, alignÃ©s sur [Ã‰quivalents Reseaux Sociaux](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (messages, conversations, rÃ©actions, readmarks) appartient Ã  **KindMother**. MiyuSocialMessaging expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les dÃ©cisions (envoi autorisÃ©, destinataires) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuSocialMessaging, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'affichage (MiyuWeb) ; l'implÃ©mentation dÃ©taillÃ©e (stockage, temps rÃ©el).

---

## 3. DÃ©finition canonique

> **MiyuSocialMessaging est une composition officielle d'outils de messagerie sociale (DMs, conversations, rÃ©actions, marques de lecture), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuSocialMessaging **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuSocialMessaging **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; dÃ©cision d'envoi = StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (message, rÃ©action, readmark) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.social.messaging` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `social` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuSocialMessaging - Reference Outils](./MiyuSocialMessaging%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.social.dm.send` | Envoie un message direct ; autorisation = StrongFather |
| `tool.social.dm.list` | Liste les messages d'une conversation |
| `tool.social.dm.get` | RÃ©cupÃ¨re un message |
| `tool.social.conversation.list` | Liste les conversations |
| `tool.social.conversation.get` | RÃ©cupÃ¨re une conversation (fil de messages) |
| `tool.social.dm.reaction.add` | Ajoute une rÃ©action Ã  un DM |
| `tool.social.dm.reaction.remove` | Supprime une rÃ©action |
| `tool.social.dm.readmark.set` | Marque comme lu ; WriteIntent KindMother |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuSocialMessaging en contient huit.

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

**KindMother** est l'autoritÃ© sur les donnÃ©es : messages, conversations, rÃ©actions, readmarks. Toute crÃ©ation ou mise Ã  jour passe par **WriteIntent** vers KindMother.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuSocialMessaging - Tool Governance Compliance Contract](./contracts/governance/MiyuSocialMessaging%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuSocialMessaging sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuSocialMessaging devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Reseaux Sociaux | [Miyukini Conceptual References - Equivalents Reseaux Sociaux](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


