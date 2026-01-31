# MiyuProfile — Documentation Fondatrice

## 1. Contexte

**MiyuProfile** est le **kit d'outils (Toolkit)** de profil étendu (champs, signature, avatar, rangs, préférences) de l'écosystème Miyukini. Il intègre les outils de lecture/mise à jour du profil, des champs personnalisés, de la signature, de l'avatar, des rangs et des préférences, alignés sur [Équivalents Moteur Forum](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md).

L'autorité sur les données (profil, champs, signature, avatar, rangs, préférences) appartient à **KindMother**. MiyuProfile expose des capacités d'exécution gouvernée ; les décisions (modification autorisée, règles d'attribution des rangs) relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuProfile, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'identité de base (MiyuAuth) ; l'affichage du profil (MiyuWeb) ; l'implémentation détaillée (schéma champs).

---

## 3. Définition canonique

> **MiyuProfile est une composition officielle d'outils de profil étendu (champs, signature, avatar, rangs, préférences), déclarée et gouvernée par l'environnement.**

- MiyuProfile **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuProfile **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; décision (modification autorisée, rangs) = StrongFather.

**Règle fondamentale :** Toute écriture (profil, champ, signature, avatar, préférences) = **WriteIntent** vers KindMother. Règles d'attribution des rangs = StrongFather.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.identity.profile` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `identity` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuProfile - Reference Outils](./MiyuProfile%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.profile.get` | Récupère le profil (utilisateur fourni) |
| `tool.profile.update` | Met à jour le profil (données fournies) ; autorisation = StrongFather |
| `tool.profile.field.list` | Liste les champs personnalisés (schéma) |
| `tool.profile.field.get` | Récupère la valeur d'un champ |
| `tool.profile.field.set` | Met à jour un champ ; WriteIntent KindMother |
| `tool.profile.avatar.get` | Récupère l'avatar |
| `tool.profile.avatar.set` | Met à jour l'avatar ; stockage KindMother ou MiyuMedia |
| `tool.profile.avatar.resolve` | Résout l'avatar (ex. Gravatar) |
| `tool.profile.signature.get` | Récupère la signature |
| `tool.profile.signature.set` | Met à jour la signature ; WriteIntent KindMother |
| `tool.profile.rank.list` | Liste les rangs disponibles |
| `tool.profile.rank.resolve` | Résout le rang d'un utilisateur ; règles = StrongFather |
| `tool.profile.preferences.get` | Récupère les préférences |
| `tool.profile.preferences.set` | Met à jour les préférences ; WriteIntent KindMother |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuProfile en contient quatorze.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décision (modification autorisée, rangs) = StrongFather** ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 2** (données personnelles) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : profil, champs, signature, avatar (ou MiyuMedia), rangs, préférences. Toute création ou mise à jour passe par **WriteIntent** vers KindMother. Schéma des champs = KindMother.

Les obligations de conformité détaillées sont dans [MiyuProfile - Tool Governance Compliance Contract](./contracts/governance/MiyuProfile%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuProfile sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuProfile devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

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
