# MiyuSocialModeration — Documentation Fondatrice

## 1. Contexte

**MiyuSocialModeration** est le **kit d'outils (Toolkit)** de modération sociale (signalement, blocage, suppression post visibilité) de l'écosystème Miyukini. Il intègre les outils de signalement, de blocage utilisateur et de suppression/visibilité de post, alignés sur [Équivalents Reseaux Sociaux](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Reseaux%20Sociaux.md).

L'autorité sur les données (signalements, blocages, visibilité) appartient à **KindMother**. MiyuSocialModeration expose des capacités d'exécution gouvernée ; **toutes les décisions** (accepter signalement, bloquer, supprimer post) relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuSocialModeration, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** la modération forum (MiyuModerationForum) ; l'implémentation détaillée (politique de modération).

---

## 3. Définition canonique

> **MiyuSocialModeration est une composition officielle d'outils de modération sociale (signalement, blocage, suppression post visibilité), déclarée et gouvernée par l'environnement.**

- MiyuSocialModeration **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuSocialModeration **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; **décision (signalement, blocage, suppression) = StrongFather**.

**Règle fondamentale :** Toute action (signalement, blocage, suppression post) = décision StrongFather + **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.social.moderation` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `social` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuSocialModeration - Reference Outils](./MiyuSocialModeration%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.moderation.report.create` | Crée un signalement ; WriteIntent KindMother |
| `tool.moderation.report.list` | Liste les signalements |
| `tool.social.block.add` | Bloque un utilisateur ; décision = StrongFather |
| `tool.social.block.remove` | Débloque un utilisateur |
| `tool.social.block.list` | Liste les utilisateurs bloqués |
| `tool.social.post.delete` | Supprime un post (visibilité) ; décision = StrongFather |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuSocialModeration en contient six.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **toute décision de modération (signalement, blocage, suppression) = StrongFather** ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **2 à 3** (actions de modération sensibles) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : signalements, blocages, visibilité des posts. Toute action de modération qui modifie des données passe par **WriteIntent** vers KindMother. Décision = StrongFather.

Les obligations de conformité détaillées sont dans [MiyuSocialModeration - Tool Governance Compliance Contract](./contracts/governance/MiyuSocialModeration%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuSocialModeration sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuSocialModeration devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

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
