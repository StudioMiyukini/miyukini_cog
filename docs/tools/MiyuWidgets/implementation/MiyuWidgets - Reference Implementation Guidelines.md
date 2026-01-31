# MiyuWidgets — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuWidgets conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuWidgets en logique d'implémentation (layout, widgets, template ; données dans le flux, pas de lecture base directe).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuWidgets (rendu de blocs et composition de layout : layout.apply, widget.*.render, template.resolve) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuWidgets - Documentation Fondatrice** : ToolkitId `toolkit.web.widgets`, liste des Tools (layout.apply, widget.text/image/button/grid/container.render, template.resolve).
- **MiyuWidgets - Reference Outils** : Détail de chaque ToolId.
- **MiyuWidgets - Tool Governance Compliance Contract** : Obligations spécifiques (données dans le flux uniquement ; pas de lecture base directe ; persistance templates/layouts = KindMother).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

MiyuWidgets est invoqué après passage par la gouvernance. Ne pas ré-évaluer les permissions. Décision de contenu ou de logique métier = Opérateurs et Cores, pas le kit.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools exécutent sur les **données fournies dans le flux** ; aucune décision sur le contenu, la structure ou la visibilité. MiyuWidgets ne lit pas la base directement.

### 2.3 Pas d'accès direct ; données dans le flux (BOUND-3)

**Règle fondamentale MiyuWidgets :** Les Tools opèrent uniquement sur des **données fournies dans le flux**. Aucune lecture directe de la base. Persistance des templates et layouts = KindMother (écrite par d'autres flux). Pas d'écriture métier depuis le kit.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds déclarés (layout.apply, widget.*.render, template.resolve).

### 2.7 Niveau de sécurité et états

Niveau **0 à 2** selon politique d'exposition (page builder éditorial). États autorisés : `HEALTHY`, `DEGRADED`. MiyuWeb fournit les capacités de base ; MiyuWidgets complète pour l'édition visuelle de pages et de thèmes.

### 2.8 Alignement MIP/MSCM

Domaine `web`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat ; pas de décision contenu |
| **BOUND-2** | Pas de choix métier | Pas de décision contenu, structure, visibilité |
| **BOUND-3** | Pas d'accès direct | Données uniquement dans le flux ; pas de lecture base |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement layout.apply, widget.*.render, template.resolve |

---

## 4. Gestion des erreurs et traçabilité

- Erreurs techniques (données manquantes dans le flux, template introuvable) remontées sans exposer de contenu sensible.
- En cas de violation de bornage (tentative de lecture base, modification contexte), refus d'exécution et signal.
- Logger du Kernel pour la traçabilité (sans contenu métier).

---

## 5. Références

| Document | Lien |
|----------|------|
| MiyuWidgets - Documentation Fondatrice | [Documentation Fondatrice](../MiyuWidgets%20-%20Documentation%20Fondatrice.md) |
| MiyuWidgets - Reference Outils | [Reference Outils](../MiyuWidgets%20-%20Reference%20Outils.md) |
| MiyuWidgets - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuWidgets%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif
