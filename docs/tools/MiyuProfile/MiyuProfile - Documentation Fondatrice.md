# MiyuProfile â€” Documentation Fondatrice

## 1. Contexte

**MiyuProfile** est le **kit d'outils (Toolkit)** de profil Ã©tendu (champs, signature, avatar, rangs, prÃ©fÃ©rences) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de lecture/mise Ã  jour du profil, des champs personnalisÃ©s, de la signature, de l'avatar, des rangs et des prÃ©fÃ©rences, alignÃ©s sur [Ã‰quivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (profil, champs, signature, avatar, rangs, prÃ©fÃ©rences) appartient Ã  **KindMother**. MiyuProfile expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les dÃ©cisions (modification autorisÃ©e, rÃ¨gles d'attribution des rangs) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuProfile, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'identitÃ© de base (MiyuAuth) ; l'affichage du profil (MiyuWeb) ; l'implÃ©mentation dÃ©taillÃ©e (schÃ©ma champs).

---

## 3. DÃ©finition canonique

> **MiyuProfile est une composition officielle d'outils de profil Ã©tendu (champs, signature, avatar, rangs, prÃ©fÃ©rences), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuProfile **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuProfile **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; dÃ©cision (modification autorisÃ©e, rangs) = StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (profil, champ, signature, avatar, prÃ©fÃ©rences) = **WriteIntent** vers KindMother. RÃ¨gles d'attribution des rangs = StrongFather.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.identity.profile` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `identity` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuProfile - Reference Outils](./MiyuProfile%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.profile.get` | RÃ©cupÃ¨re le profil (utilisateur fourni) |
| `tool.profile.update` | Met Ã  jour le profil (donnÃ©es fournies) ; autorisation = StrongFather |
| `tool.profile.field.list` | Liste les champs personnalisÃ©s (schÃ©ma) |
| `tool.profile.field.get` | RÃ©cupÃ¨re la valeur d'un champ |
| `tool.profile.field.set` | Met Ã  jour un champ ; WriteIntent KindMother |
| `tool.profile.avatar.get` | RÃ©cupÃ¨re l'avatar |
| `tool.profile.avatar.set` | Met Ã  jour l'avatar ; stockage KindMother ou MiyuMedia |
| `tool.profile.avatar.resolve` | RÃ©sout l'avatar (ex. Gravatar) |
| `tool.profile.signature.get` | RÃ©cupÃ¨re la signature |
| `tool.profile.signature.set` | Met Ã  jour la signature ; WriteIntent KindMother |
| `tool.profile.rank.list` | Liste les rangs disponibles |
| `tool.profile.rank.resolve` | RÃ©sout le rang d'un utilisateur ; rÃ¨gles = StrongFather |
| `tool.profile.preferences.get` | RÃ©cupÃ¨re les prÃ©fÃ©rences |
| `tool.profile.preferences.set` | Met Ã  jour les prÃ©fÃ©rences ; WriteIntent KindMother |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuProfile en contient quatorze.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision (modification autorisÃ©e, rangs) = StrongFather** ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  2** (donnÃ©es personnelles) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : profil, champs, signature, avatar (ou MiyuMedia), rangs, prÃ©fÃ©rences. Toute crÃ©ation ou mise Ã  jour passe par **WriteIntent** vers KindMother. SchÃ©ma des champs = KindMother.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuProfile - Tool Governance Compliance Contract](./contracts/governance/MiyuProfile%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuProfile sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuProfile devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


