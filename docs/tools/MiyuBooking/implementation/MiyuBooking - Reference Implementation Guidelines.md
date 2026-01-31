# MiyuBooking — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuBooking conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuBooking en logique d'implémentation (créneaux, réservations, ressources, tarification ; gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle. Les contrats fondateurs priment toujours.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuBooking (réservation en ligne : créneaux, réservations, ressources, prix, participants) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Sources contractuelles

- **MiyuBooking - Documentation Fondatrice** : ToolkitId `toolkit.booking.reservations`, liste des Tools (slots.*, create, update, cancel, resource.*, price.compute, participants.compute).
- **MiyuBooking - Reference Outils** : Détail de chaque ToolId.
- **MiyuBooking - Tool Governance Compliance Contract** : Obligations spécifiques (décision StrongFather, WriteIntent KindMother).
- **Master Butler - Tool Governance Compliance Template** : Obligations communes.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

MiyuBooking est invoqué uniquement après décision StrongFather (création réservation, annulation). Ne pas ré-évaluer les permissions. En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools exécutent sur les données fournies ; aucune décision sur la politique de réservation, les règles de créneaux ou les tarifs. Règles fournies par KindMother ou dans le flux.

### 2.3 Toute écriture = WriteIntent KindMother (BOUND-3)

Toute création/mise à jour/annulation de réservation = **WriteIntent** vers KindMother. Aucun accès direct à la base. Lectures (slots.list, resource.availability, price.compute, participants.compute) sur données fournies dans le flux ou gouvernées.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds déclarés (slots.list/resolve, create, update, cancel, resource.resolve/availability, price.compute, participants.compute).

### 2.7 Niveau de sécurité et états

Niveau **0 à 2** (lecture créneaux 0–1, création/annulation réservation 1–2). États autorisés : `HEALTHY`, `DEGRADED`. Vérifier WorrySentinel / Caring Nanny avant exécution.

### 2.8 Alignement MIP/MSCM

Domaine `booking`, layer Strate 6. Baliser le code MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix métier | Pas de décision politique réservation, créneaux, tarifs |
| **BOUND-3** | Pas d'accès direct | Toute écriture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement ToolIds déclarés (slots, create, update, cancel, resource.*, price.compute, participants.compute) |

---

## 4. Gestion des erreurs et traçabilité

- Erreurs techniques (paramètres invalides, WriteIntent refusée) remontées sans exposer de données sensibles.
- En cas de violation de bornage, refus d'exécution et signal.
- Logger du Kernel pour la traçabilité (sans contenu métier).

---

## 5. Références

| Document | Lien |
|----------|------|
| MiyuBooking - Documentation Fondatrice | [Documentation Fondatrice](../MiyuBooking%20-%20Documentation%20Fondatrice.md) |
| MiyuBooking - Reference Outils | [Reference Outils](../MiyuBooking%20-%20Reference%20Outils.md) |
| MiyuBooking - Tool Governance Compliance Contract | [Contrat Governance](../contracts/governance/MiyuBooking%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Template Reference Implementation Guidelines | [docs_tools - Template](../../docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| MIP v1 | [Miyukini Prompt Protocol - MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Guide informatif
