# MiyuSocialMessaging — Documentation Fondatrice

## 1. Contexte

**MiyuSocialMessaging** est le **kit d'outils (Toolkit)** de messagerie sociale (DMs, conversations, réactions, marques de lecture) de l'écosystème Miyukini. Il intègre les outils d'envoi de DM, de liste des conversations, de réactions et de marques de lecture, alignés sur [Équivalents Reseaux Sociaux](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Reseaux%20Sociaux.md).

L'autorité sur les données (messages, conversations, réactions, readmarks) appartient à **KindMother**. MiyuSocialMessaging expose des capacités d'exécution gouvernée ; les décisions (envoi autorisé, destinataires) relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuSocialMessaging, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'affichage (MiyuWeb) ; l'implémentation détaillée (stockage, temps réel).

---

## 3. Définition canonique

> **MiyuSocialMessaging est une composition officielle d'outils de messagerie sociale (DMs, conversations, réactions, marques de lecture), déclarée et gouvernée par l'environnement.**

- MiyuSocialMessaging **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuSocialMessaging **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; décision d'envoi = StrongFather.

**Règle fondamentale :** Toute écriture (message, réaction, readmark) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.social.messaging` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `social` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuSocialMessaging - Reference Outils](./MiyuSocialMessaging%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.social.dm.send` | Envoie un message direct ; autorisation = StrongFather |
| `tool.social.dm.list` | Liste les messages d'une conversation |
| `tool.social.dm.get` | Récupère un message |
| `tool.social.conversation.list` | Liste les conversations |
| `tool.social.conversation.get` | Récupère une conversation (fil de messages) |
| `tool.social.dm.reaction.add` | Ajoute une réaction à un DM |
| `tool.social.dm.reaction.remove` | Supprime une réaction |
| `tool.social.dm.readmark.set` | Marque comme lu ; WriteIntent KindMother |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuSocialMessaging en contient huit.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décision d'envoi = StrongFather** ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **2** (données personnelles, messagerie) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : messages, conversations, réactions, readmarks. Toute création ou mise à jour passe par **WriteIntent** vers KindMother.

Les obligations de conformité détaillées sont dans [MiyuSocialMessaging - Tool Governance Compliance Contract](./contracts/governance/MiyuSocialMessaging%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuSocialMessaging sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuSocialMessaging devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents Reseaux Sociaux | [Miyukini Conceptual References - Equivalents Reseaux Sociaux](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Reseaux%20Sociaux.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
