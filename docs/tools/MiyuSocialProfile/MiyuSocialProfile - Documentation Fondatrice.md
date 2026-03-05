# MiyuSocialProfile â€” Documentation Fondatrice

## 1. Contexte

**MiyuSocialProfile** est le **kit d'outils (Toolkit)** de profil social (profil, abonnÃ©s, abonnements) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de lecture/mise Ã  jour du profil social, de follow/unfollow et de liste des abonnÃ©s/abonnements, alignÃ©s sur [Ã‰quivalents Reseaux Sociaux](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (profil social, follow, abonnÃ©s, abonnements) appartient Ã  **KindMother**. MiyuSocialProfile expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les dÃ©cisions (modification autorisÃ©e, follow autorisÃ©) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuSocialProfile, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'identitÃ© de base (MiyuAuth) ; le profil Ã©tendu forum (MiyuProfile) ; l'affichage (MiyuWeb).

---

## 3. DÃ©finition canonique

> **MiyuSocialProfile est une composition officielle d'outils de profil social (profil, abonnÃ©s, abonnements), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuSocialProfile **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuSocialProfile **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; dÃ©cision (modification, follow) = StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (profil, follow) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.social.profile` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `social` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuSocialProfile - Reference Outils](./MiyuSocialProfile%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.social.profile.get` | RÃ©cupÃ¨re le profil social |
| `tool.social.profile.update` | Met Ã  jour le profil social ; autorisation = StrongFather |
| `tool.social.follow.add` | Ajoute un abonnement (follow) ; WriteIntent KindMother |
| `tool.social.follow.remove` | Supprime un abonnement |
| `tool.social.followers.list` | Liste les abonnÃ©s |
| `tool.social.following.list` | Liste les abonnements |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuSocialProfile en contient six.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision (modification, follow autorisÃ©) = StrongFather** ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  2** (donnÃ©es personnelles) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : profil social, follow, abonnÃ©s, abonnements. Toute crÃ©ation ou mise Ã  jour passe par **WriteIntent** vers KindMother.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuSocialProfile - Tool Governance Compliance Contract](./contracts/governance/MiyuSocialProfile%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuSocialProfile sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuSocialProfile devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


