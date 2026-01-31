# MiyuCMS — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuCMS conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuCMS en logique d'implémentation (contenus, révisions, commentaires, médias ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuCMS (gestion de contenu éditorial : création, mise à jour, publication, planification, révisions, commentaires, médias) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuCMS - Documentation Fondatrice** : ToolkitId `toolkit.content.cms`, liste des Tools (content.*, revision.*, comment.*, media.*).
- **MiyuCMS - Reference Outils** : Détail de chaque ToolId.
- **MiyuCMS - Tool Governance Compliance Contract** : Obligations spécifiques (décision StrongFather pour publication/modération, WriteIntent KindMother).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

MiyuCMS est invoqué uniquement après décision StrongFather (publication, modération, restauration révision). Ne pas ré-évaluer les permissions. En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools exécutent sur les données fournies ; aucune décision sur la politique éditoriale, la modération ou la visibilité. Règles fournies par StrongFather / KindMother ou dans le flux.

### 2.3 Toute écriture = WriteIntent KindMother (BOUND-3)

Toute création/mise à jour (contenu, révision, commentaire, média) = **WriteIntent** vers KindMother. Aucun accès direct à la base. Lectures (revision.list, revision.compare, comment.list, media.serve) sur données fournies dans le flux.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds déclarés (content.*, revision.*, comment.*, media.upload/serve/transform).

### 2.7 Niveau de sécurité et états

Niveau **0 à 2** selon politique d'exposition. États autorisés : `HEALTHY`, `DEGRADED`. Vérifier WorrySentinel / Caring Nanny avant exécution. L'affichage des contenus est du ressort de MiyuWeb (données fournies dans le flux).

### 2.8 Alignement MIP/MSCM

Domaine `content`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix métier | Pas de décision publication, modération, politique éditoriale |
| **BOUND-3** | Pas d'accès direct | Toute écriture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement content.*, revision.*, comment.*, media.* déclarés |

---

## 4. Gestion des erreurs et traçabilité

- Erreurs techniques (paramètres invalides, WriteIntent refusée) remontées sans exposer de contenu ou données sensibles.
- En cas de violation de bornage, refus d'exécution et signal.
- Logger du Kernel pour la traçabilité (sans contenu métier).

---

## 5. Références

| Document | Lien |
|----------|------|
| MiyuCMS - Documentation Fondatrice | [Documentation Fondatrice](../MiyuCMS%20-%20Documentation%20Fondatrice.md) |
| MiyuCMS - Reference Outils | [Reference Outils](../MiyuCMS%20-%20Reference%20Outils.md) |
| MiyuCMS - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuCMS%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif
