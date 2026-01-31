# MiyuSocialFeed — Documentation Fondatrice

## 1. Contexte

**MiyuSocialFeed** est le **kit d'outils (Toolkit)** de fil d'actualité social (publication, flux, réactions, partages, commentaires) de l'écosystème Miyukini. Il intègre les outils de publication, de liste du flux, de réactions, de partages et de commentaires, alignés sur [Équivalents Reseaux Sociaux](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Reseaux%20Sociaux.md).

L'autorité sur les données (posts, flux, réactions, partages, commentaires) appartient à **KindMother**. MiyuSocialFeed expose des capacités d'exécution gouvernée ; les décisions (publication autorisée, visibilité, modération) relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuSocialFeed, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** les stories (MiyuStory) ; la messagerie (MiyuSocialMessaging) ; l'implémentation détaillée (algorithme de flux).

---

## 3. Définition canonique

> **MiyuSocialFeed est une composition officielle d'outils de fil d'actualité social (publication, flux, réactions, partages, commentaires), déclarée et gouvernée par l'environnement.**

- MiyuSocialFeed **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuSocialFeed **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; décision (publication, visibilité) = StrongFather.

**Règle fondamentale :** Toute écriture (post, réaction, partage, commentaire) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.social.feed` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `social` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuSocialFeed - Reference Outils](./MiyuSocialFeed%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.social.post.create` | Crée une publication ; autorisation = StrongFather |
| `tool.social.post.update` | Met à jour une publication |
| `tool.social.post.delete` | Supprime une publication ; décision = StrongFather |
| `tool.social.post.get` | Récupère une publication |
| `tool.social.feed.list` | Liste le flux (filtres, pagination fournis) |
| `tool.social.reaction.add` | Ajoute une réaction (like, etc.) |
| `tool.social.reaction.remove` | Supprime une réaction |
| `tool.social.reaction.list` | Liste les réactions d'un post |
| `tool.social.share.create` | Crée un partage ; autorisation = StrongFather |
| `tool.social.share.list` | Liste les partages |
| `tool.social.comment.create` | Crée un commentaire |
| `tool.social.comment.list` | Liste les commentaires d'un post |
| `tool.social.comment.delete` | Supprime un commentaire ; décision = StrongFather |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuSocialFeed en contient treize.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décision (publication, visibilité, suppression) = StrongFather** ; toute écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 3** (contenu social, données personnelles) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : posts, flux, réactions, partages, commentaires. Toute création, mise à jour ou suppression passe par **WriteIntent** vers KindMother.

Les obligations de conformité détaillées sont dans [MiyuSocialFeed - Tool Governance Compliance Contract](./contracts/governance/MiyuSocialFeed%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuSocialFeed sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuSocialFeed devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

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
