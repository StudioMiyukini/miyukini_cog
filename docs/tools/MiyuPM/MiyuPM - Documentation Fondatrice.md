# MiyuPM — Documentation Fondatrice

## 1. Contexte

**MiyuPM** est le **kit d'outils (Toolkit)** de messagerie privée (envoi, dossiers, brouillons, conversation, export) de l'écosystème Miyukini. Il intègre les outils d'envoi, de liste, de gestion des dossiers et brouillons, et d'export des MP, alignés sur [Équivalents Moteur Forum](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md).

L'autorité sur les données (messages, dossiers, brouillons, conversations) appartient à **KindMother**. MiyuPM expose des capacités d'exécution gouvernée ; les décisions (envoi autorisé, destinataires, quotas) relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuPM, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** les pièces jointes (MiyuMedia) ; l'anti-spam / flood (MiyuAntiSpam) ; l'implémentation détaillée (stockage).

---

## 3. Définition canonique

> **MiyuPM est une composition officielle d'outils de messagerie privée (envoi, dossiers, brouillons, conversation, export), déclarée et gouvernée par l'environnement.**

- MiyuPM **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuPM **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; décision d'envoi = StrongFather.

**Règle fondamentale :** Toute écriture (message, dossier, brouillon) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.communication.pm` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `communication` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuPM - Reference Outils](./MiyuPM%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.pm.send` | Envoie un message privé (destinataire, contenu fournis) ; autorisation = StrongFather |
| `tool.pm.list` | Liste les messages (dossier, filtres fournis) |
| `tool.pm.get` | Récupère un message |
| `tool.pm.folder.list` | Liste les dossiers (inbox, sent, etc.) |
| `tool.pm.folder.create` | Crée un dossier personnalisé |
| `tool.pm.folder.update` | Met à jour un dossier |
| `tool.pm.draft.create` | Crée un brouillon |
| `tool.pm.draft.update` | Met à jour un brouillon |
| `tool.pm.draft.list` | Liste les brouillons |
| `tool.pm.conversation.list` | Liste les conversations |
| `tool.pm.conversation.get` | Récupère une conversation (fil de messages) |
| `tool.pm.export` | Exporte les messages (format fourni) ; exécution seule |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuPM en contient douze.

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

**KindMother** est l'autorité sur les données : messages, dossiers, brouillons, conversations. Toute création, mise à jour ou suppression passe par **WriteIntent** vers KindMother.

Les obligations de conformité détaillées sont dans [MiyuPM - Tool Governance Compliance Contract](./contracts/governance/MiyuPM%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuPM sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuPM devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

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
