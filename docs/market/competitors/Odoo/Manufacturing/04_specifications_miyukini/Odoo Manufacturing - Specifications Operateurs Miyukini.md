# Odoo Manufacturing — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application Manufacturing (MRP) d'Odoo, en respectant l'architecture COG et la gouvernance Miyukini.

**Références :**
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Odoo Manufacturing - Logique Métier](../00_logique_metier/Odoo%20Manufacturing%20-%20Logique%20Metier%20Complete.md)
- [Odoo Manufacturing - Intégrations Cross-App](../03_integrations/Odoo%20Manufacturing%20-%20Integrations%20Cross%20App.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalent Manufacturing
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores (StrongFather, KindMother, Master Butler, WorrySentinel)

**Hors scope :**
- Implémentation technique détaillée (voir Guide d'Implémentation)
- Spécifications UI/UX (document dédié)

---

## 1. Architecture Opérateurs

### 1.1 Vue d'ensemble

L'équivalent Manufacturing dans Miyukini s'appuie sur un **service COG Miyukini Manufacturing** (ou **Miyukini MRP**), avec des **Opérateurs spécialisés** pour la fabrication, les nomenclatures, les ordres de travail et la planification.

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **ManufacturingBOM** | Nomenclatures (BOM) et lignes composants | Opérateur de Service |
| **ManufacturingRouting** | Gammes et opérations (postes, temps) | Opérateur de Service |
| **ManufacturingWorkCenter** | Postes de travail (capacité, coûts) | Opérateur de Service |
| **ManufacturingOrder** | Ordres de fabrication (OF) et états | Opérateur de Service |
| **ManufacturingWorkOrder** | Ordres de travail (WO) et exécution atelier | Opérateur de Service |
| **ManufacturingPlanning** | Plan directeur (MPS) et propositions OF | Opérateur de Service |
| **ManufacturingReporting** | Rapports (OEE, délais, allocation, coûts) | Opérateur de Service |
| **ManufacturingUI** | Interface bureau (OF, BOM, gammes, MPS) | Opérateur d'Interface |
| **ManufacturingShopFloor** | Interface atelier (tableau de bord poste) | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : ManufacturingService

**Définition :**
> **ManufacturingService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de fabrication (MRP) et de pilotage atelier.**

**Composition :**
- ManufacturingBOM (niveau sécurité 2)
- ManufacturingRouting (niveau sécurité 2)
- ManufacturingWorkCenter (niveau sécurité 2)
- ManufacturingOrder (niveau sécurité 2)
- ManufacturingWorkOrder (niveau sécurité 2)
- ManufacturingPlanning (niveau sécurité 2)
- ManufacturingReporting (niveau sécurité 1–2)
- ManufacturingUI (niveau sécurité 1)
- ManufacturingShopFloor (niveau sécurité 2, périmètre poste)

**Contrat d'Équipe :** Voir section 2.

---

## 2. Contrat d'Équipe ManufacturingService

### 2.1 Opérateurs membres

- ManufacturingBOM, ManufacturingRouting, ManufacturingWorkCenter, ManufacturingOrder, ManufacturingWorkOrder, ManufacturingPlanning, ManufacturingReporting, ManufacturingUI, ManufacturingShopFloor.

### 2.2 Flux autorisés

| De | Vers | Flux |
|----|------|------|
| ManufacturingUI | ManufacturingOrder, ManufacturingBOM, ManufacturingRouting, ManufacturingWorkCenter, ManufacturingPlanning | Requêtes lecture / création / mise à jour (sous Mandat) |
| ManufacturingShopFloor | ManufacturingWorkOrder | Démarrage / fin WO, saisie quantités et temps (sous Mandat poste) |
| ManufacturingOrder | ManufacturingWorkOrder | Création WO à la confirmation OF (interne) |
| ManufacturingOrder | KindMother (Stock) | WriteIntent mouvements matières et finis |
| ManufacturingPlanning | ManufacturingOrder | Proposition / création OF (sous Mandat) |
| ManufacturingReporting | ManufacturingOrder, ManufacturingWorkOrder, ManufacturingWorkCenter | Lecture données pour rapports |
| ManufacturingBOM | ManufacturingOrder | Lecture BOM pour création OF |
| ManufacturingRouting | ManufacturingWorkOrder | Lecture gamme pour création WO |

### 2.3 Direction des flux

- UI et ShopFloor ne communiquent pas directement ; ils passent par les Opérateurs de service (Order, WorkOrder, BOM, etc.).
- Pas de communication directe entre Opérateurs sans Mandat ou Contrat d'équipe.
- Toute modification d'état OF/WO et toute création de move passent par StrongFather (décision) et KindMother (WriteIntent).

### 2.4 Types d'échanges

- Requêtes lecture (BOM, OF, WO, postes, planification).
- Intentions d'écriture : création OF, confirmation OF, clôture OF, backorder, création WO, démarrage/fin WO, mouvements stock.
- Données échangeables : identifiants OF, WO, BOM, produit, quantités, dates, états.

### 2.5 Conditions préalables

- Mandat de Permission valide pour toute action modifiant des données (création OF, confirmation, WO, moves).
- Niveau de sécurité conforme (WorrySentinel).
- BOM et gamme cohérentes (Ever Buddy / cycle de vie) avant confirmation OF.

### 2.6 Niveau de validation requis

- Création / modification BOM ou gamme : validation méthodiste (Master Butler).
- Confirmation OF : décision StrongFather (peut déléguer via Mandat avec seuils).
- Clôture OF, backorder : décision StrongFather.
- Démarrage / fin WO : Mandat poste (ManufacturingShopFloor) ou Mandat standard Manufacturing.

---

## 3. Opérateurs Détaillés

### 3.1 ManufacturingBOM

**Rôle :** Gestion des nomenclatures (BOM) et des lignes de composants.

**Capacités :**
- Création / modification BOM et lignes
- Lecture BOM par produit, variante, type (normal, phantom, kit)
- Configuration consommation (strict / flexible), ready to produce

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance BOM (WriteIntent)
- **Master Butler** : Permissions création/modification BOM
- **Ever Buddy** : Compatibilité versions BOM (révisions si PLM)

**Contrat d'équipe :**
- Consommé par : ManufacturingOrder, ManufacturingPlanning
- Expose : `bom.get`, `bom.create`, `bom.update`, `bom.lines.get`

### 3.2 ManufacturingRouting

**Rôle :** Gestion des gammes (routing) et des opérations (poste, temps, dépendances).

**Capacités :**
- Création / modification gammes et opérations
- Lecture par BOM ou par produit
- Dépendances entre opérations

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :** Idem ManufacturingBOM.

**Contrat d'équipe :**
- Consommé par : ManufacturingOrder (création WO)
- Expose : `routing.get`, `routing.create`, `routing.update`

### 3.3 ManufacturingWorkCenter

**Rôle :** Gestion des postes de travail (capacité, coûts, calendrier).

**Capacités :**
- Création / modification postes
- Lecture pour planification et coûts
- Postes alternatifs

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :** KindMother, Master Butler.

**Contrat d'équipe :**
- Consommé par : ManufacturingOrder, ManufacturingWorkOrder, ManufacturingPlanning, ManufacturingReporting
- Expose : `workcenter.get`, `workcenter.create`, `workcenter.update`

### 3.4 ManufacturingOrder

**Rôle :** Gestion des ordres de fabrication (création, confirmation, mouvements stock, clôture, backorder).

**Capacités :**
- Création OF (manuel, depuis vente, depuis MPS, depuis réapprovisionnement)
- Confirmation OF (réservation, création WO si gamme)
- Clôture, backorder, annulation
- Lecture états et mouvements

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision confirmation, clôture, backorder
- **KindMother** : WriteIntent mouvements stock (raw, finished)
- **Master Butler** : Permissions OF
- **WorrySentinel** : Niveau sécurité, état système

**Contrat d'équipe :**
- Consomme : ManufacturingBOM, ManufacturingRouting, ManufacturingWorkCenter ; KindMother (moves)
- Expose : `mo.create`, `mo.confirm`, `mo.close`, `mo.backorder`, `mo.cancel`, `mo.get`

### 3.5 ManufacturingWorkOrder

**Rôle :** Gestion des ordres de travail (création depuis OF, démarrage, fin, quantités, temps).

**Capacités :**
- Création WO à la confirmation OF (interne)
- Démarrage / fin WO (sous Mandat poste ou standard)
- Saisie quantités produites, temps, alertes qualité/maintenance
- Dépendances (blocked_by) et enchaînement

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision démarrage/fin si seuils (optionnel)
- **KindMother** : WriteIntent états WO, temps, consommation composants par opération
- **Master Butler** : Permissions poste (ManufacturingShopFloor)

**Contrat d'équipe :**
- Consommé par : ManufacturingShopFloor, ManufacturingUI
- Expose : `wo.start`, `wo.finish`, `wo.get`, `wo.list_by_workcenter`

### 3.6 ManufacturingPlanning

**Rôle :** Plan directeur (MPS), propositions d'OF, capacité et besoins.

**Capacités :**
- Calcul des besoins (demande, stock, OF en cours)
- Proposition d'OF (dates, quantités)
- Création d'OF après validation utilisateur (Mandat)
- Vue planification (Gantt, tableau)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de créer un OF depuis MPS
- **KindMother** : Lecture stock, OF ; pas d'écriture directe (passe par ManufacturingOrder)
- **Master Butler** : Permissions MPS

**Contrat d'équipe :**
- Consomme : ManufacturingOrder, ManufacturingBOM, Stock (lecture)
- Expose : `mps.compute`, `mps.propose_mo`, `mps.create_mo` (délégué à ManufacturingOrder)

### 3.7 ManufacturingReporting

**Rôle :** Rapports (OEE, délais, allocation, coûts, analyse production).

**Capacités :**
- Lecture OF, WO, postes, mouvements (agrégations)
- Génération rapports et exports
- Pas de modification des données

**Niveau de sécurité :** 1–2 (Standard à Sensitive selon données)

**Gouvernance :** Master Butler (lecture), WorrySentinel (niveau données).

**Contrat d'équipe :**
- Consomme : ManufacturingOrder, ManufacturingWorkOrder, ManufacturingWorkCenter (lecture)
- Expose : `report.oee`, `report.delays`, `report.allocation`, `report.costs`

### 3.8 ManufacturingUI

**Rôle :** Interface bureau (OF, BOM, gammes, postes, MPS).

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :** Master Butler (permissions par écran).

**Contrat d'équipe :** Appelle les autres Opérateurs du service via BondingBrother (Mandats).

### 3.9 ManufacturingShopFloor

**Rôle :** Interface atelier (tableau de bord poste, WO, temps, quantités).

**Niveau de sécurité :** 2 (Sensitive), périmètre limité au poste assigné.

**Gouvernance :** Master Butler (Mandat poste : uniquement WO de ce poste), WorrySentinel.

**Contrat d'équipe :** Appelle ManufacturingWorkOrder avec Mandat restreint (poste, actions démarrer/terminer, saisie).

---

## 4. Mandats de Permission

### 4.1 Mandat Standard Manufacturing

- **Opérateurs autorisés :** ManufacturingUI, ManufacturingOrder, ManufacturingWorkOrder, ManufacturingPlanning, ManufacturingBOM, ManufacturingRouting, ManufacturingWorkCenter.
- **Flux :** Lecture et création/modification OF, BOM, gammes, postes dans le cadre des permissions utilisateur.
- **Conditions :** Rôle planificateur ou méthodiste ; niveau sécurité 2.
- **Révocation :** Fin de session, changement de rôle, alerte WorrySentinel.

### 4.2 Mandat Poste (Shop Floor)

- **Opérateurs autorisés :** ManufacturingShopFloor, ManufacturingWorkOrder.
- **Flux :** Démarrage/fin WO, saisie quantités et temps, alertes ; uniquement pour les WO du poste assigné.
- **Conditions :** Utilisateur affecté au poste ; pas de modification BOM/OF/gammes.
- **Révocation :** Changement de poste, fin de session.

### 4.3 Mandat Validation (Clôture, backorder)

- **Décision StrongFather** : Clôture OF, création backorder, annulation OF.
- Peut être déléguée via Mandat avec seuils (ex. clôture si quantité produite = quantité demandée).
- Révocation : Alerte qualité, alerte stock, intervention manuelle.

---

## 5. Intégration avec les Cores

- **StrongFather** : Toute décision de confirmer OF, clôturer, backorder, annuler ; optionnellement création OF depuis MPS.
- **KindMother** : Toute persistance OF, WO, BOM, gamme, poste et **tous les mouvements stock** (WriteIntent).
- **Master Butler** : Permissions par Opérateur et par rôle (planificateur, opérateur, méthodiste, rapport).
- **WorrySentinel** : Niveau de sécurité 1–2 selon données ; blocage si état système dégradé.
- **Ever Buddy** : Versions BOM/routing si évolution (révisions) ; pas de modification immédiate des structures en cours d'utilisation sans processus formel.
- **Caring Nanny** : Observation états OF/WO pour tableaux de bord et rapports (lecture seule).

---

## 6. Correspondance Miyukini

**Service proposé :** **Miyukini Manufacturing** (ou **MiyuManufacturing** selon nomenclature) — ManufacturingService.

**Équivalent Odoo :** Manufacturing (MRP) — nomenclatures, ordres de fabrication, ordres de travail, postes, planification, rapports.

---

**Document** : Odoo Manufacturing — Spécifications Opérateurs Miyukini  
**Version** : 1.0  
**Date** : 2026-02-01
