# MiyuSocialModeration â€” Documentation Fondatrice

## 1. Contexte

**MiyuSocialModeration** est le **kit d'outils (Toolkit)** de modÃ©ration sociale (signalement, blocage, suppression post visibilitÃ©) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de signalement, de blocage utilisateur et de suppression/visibilitÃ© de post, alignÃ©s sur [Ã‰quivalents Reseaux Sociaux](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (signalements, blocages, visibilitÃ©) appartient Ã  **KindMother**. MiyuSocialModeration expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; **toutes les dÃ©cisions** (accepter signalement, bloquer, supprimer post) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuSocialModeration, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** la modÃ©ration forum (MiyuModerationForum) ; l'implÃ©mentation dÃ©taillÃ©e (politique de modÃ©ration).

---

## 3. DÃ©finition canonique

> **MiyuSocialModeration est une composition officielle d'outils de modÃ©ration sociale (signalement, blocage, suppression post visibilitÃ©), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuSocialModeration **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuSocialModeration **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; **dÃ©cision (signalement, blocage, suppression) = StrongFather**.

**RÃ¨gle fondamentale :** Toute action (signalement, blocage, suppression post) = dÃ©cision StrongFather + **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.social.moderation` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `social` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuSocialModeration - Reference Outils](./MiyuSocialModeration%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.moderation.report.create` | CrÃ©e un signalement ; WriteIntent KindMother |
| `tool.moderation.report.list` | Liste les signalements |
| `tool.social.block.add` | Bloque un utilisateur ; dÃ©cision = StrongFather |
| `tool.social.block.remove` | DÃ©bloque un utilisateur |
| `tool.social.block.list` | Liste les utilisateurs bloquÃ©s |
| `tool.social.post.delete` | Supprime un post (visibilitÃ©) ; dÃ©cision = StrongFather |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuSocialModeration en contient six.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **toute dÃ©cision de modÃ©ration (signalement, blocage, suppression) = StrongFather** ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **2 Ã  3** (actions de modÃ©ration sensibles) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : signalements, blocages, visibilitÃ© des posts. Toute action de modÃ©ration qui modifie des donnÃ©es passe par **WriteIntent** vers KindMother. DÃ©cision = StrongFather.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuSocialModeration - Tool Governance Compliance Contract](./contracts/governance/MiyuSocialModeration%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuSocialModeration sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuSocialModeration devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


