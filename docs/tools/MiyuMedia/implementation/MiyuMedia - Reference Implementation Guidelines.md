# MiyuMedia — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuMedia conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuMedia en logique d'implémentation (upload, service, transformation des médias ; WriteIntent KindMother).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuMedia (gestion des médias : upload, service, transformation) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuMedia - Documentation Fondatrice** : ToolkitId `toolkit.content.media`, liste des Tools (tool.media.upload, tool.media.serve, tool.media.transform).
- **MiyuMedia - Reference Outils** : Détail de chaque ToolId.
- **MiyuMedia - Tool Governance Compliance Contract** : Obligations spécifiques (WriteIntent KindMother pour upload ; pas de décision politique stockage/quota).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

MiyuMedia est invoqué après passage par la gouvernance. Ne pas ré-évaluer les permissions. Politique de stockage et quotas = StrongFather / Cores ; le kit exécute uniquement upload, serve, transform.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools exécutent sur les données fournies ; aucune décision sur la politique de stockage, les quotas ou le classement. Règles fournies dans le flux.

### 2.3 Toute écriture = WriteIntent KindMother (BOUND-3)

`tool.media.upload` produit une **WriteIntent** vers KindMother. Aucun accès direct à la base. `tool.media.serve` et `tool.media.transform` opèrent sur données fournies dans le flux (pas de persistance directe).

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds déclarés (upload, serve, transform).

### 2.7 Niveau de sécurité et états

Niveau **0 à 2** selon politique d'exposition (médias publics à sensibles). États autorisés : `HEALTHY`, `DEGRADED`. MiyuCMS peut agréger MiyuMedia pour le Service CMS complet ; MiyuMedia peut être utilisé seul.

### 2.8 Alignement MIP/MSCM

Domaine `content`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat ; politique stockage = Cores |
| **BOUND-2** | Pas de choix métier | Pas de décision politique stockage, quotas |
| **BOUND-3** | Pas d'accès direct | Upload = WriteIntent KindMother ; serve/transform sur flux |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement upload, serve, transform |

---

## 4. Gestion des erreurs et traçabilité

- Erreurs techniques (fichier invalide, WriteIntent refusée) remontées sans exposer de chemins ou métadonnées sensibles.
- En cas de violation de bornage, refus d'exécution et signal.
- Logger du Kernel pour la traçabilité (sans contenu métier).

---

## 5. Références

| Document | Lien |
|----------|------|
| MiyuMedia - Documentation Fondatrice | [Documentation Fondatrice](../MiyuMedia%20-%20Documentation%20Fondatrice.md) |
| MiyuMedia - Reference Outils | [Reference Outils](../MiyuMedia%20-%20Reference%20Outils.md) |
| MiyuMedia - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuMedia%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif
