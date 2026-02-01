# Odoo Manufacturing — Index de Documentation

## Contexte

Ce dossier contient l'**analyse complète** de l'application **Manufacturing** (MRP) d'Odoo, réalisée selon la méthodologie standardisée. L'analyse couvre la logique métier, les parcours utilisateur, l'UI/UX, les intégrations, les spécifications Opérateurs Miyukini, l'intégration COG et les guides d'implémentation.

**Date d'analyse :** 2026-02-01  
**Source :** Documentation Odoo 19.0, patterns Supply Chain / MRP

---

## Structure de Documentation

### 00_logique_metier/
- **[Odoo Manufacturing - Logique Métier Complète](./00_logique_metier/Odoo%20Manufacturing%20-%20Logique%20Metier%20Complete.md)**
  - Modèles de données (BOM, OF, WO, postes, gammes)
  - Règles métier et contraintes
  - Workflows et transitions d'état
  - Consommation, scrap, sous-traitance, unbuild, lots/séries
  - Coûts et reporting
  - Points d'attention pour Miyukini

### 01_parcours_utilisateur/
- **[Odoo Manufacturing - Parcours Utilisateur Détaillés](./01_parcours_utilisateur/Odoo%20Manufacturing%20-%20Parcours%20Utilisateur%20Detailles.md)**
  - Personas (Planificateur, Opérateur atelier, Méthodiste, Stock, Direction)
  - Parcours d'onboarding
  - Scénarios d'usage (MTO, MTS, sous-traitance, retards, unbuild)
  - Points de friction et recommandations

### 02_ui_ux/
- **[Odoo Manufacturing - Analyse UI/UX](./02_ui_ux/Odoo%20Manufacturing%20-%20Analyse%20UI%20UX.md)**
  - Vues OF, WO, BOM, postes, gammes (List, Kanban, Form, Gantt)
  - Tableau de bord poste (Shop Floor)
  - MPS et rapports (OEE, délais, allocation)
  - Patterns navigation et design atelier

### 03_integrations/
- **[Odoo Manufacturing - Intégrations Cross-App](./03_integrations/Odoo%20Manufacturing%20-%20Integrations%20Cross%20App.md)**
  - Intégration Stock (moves, réservations)
  - Intégration Purchase (sous-traitance)
  - Intégration Sales (création OF depuis commande)
  - Réapprovisionnement (orderpoint, règles Manufacture)
  - Quality, Maintenance, HR (optionnel)

### 04_specifications_miyukini/
- **[Odoo Manufacturing - Spécifications Opérateurs Miyukini](./04_specifications_miyukini/Odoo%20Manufacturing%20-%20Specifications%20Operateurs%20Miyukini.md)**
  - Architecture Opérateurs (9 Opérateurs)
  - Équipe ManufacturingService
  - Contrat d'Équipe et Mandats (Standard, Poste, Validation)
  - Niveaux de sécurité (1–2)
  - Intégration avec les Cores

### 05_integration_cog/
- **[Odoo Manufacturing - Guide Intégration COG](./05_integration_cog/Odoo%20Manufacturing%20-%20Guide%20Integration%20COG.md)**
  - Architecture d'intégration COG
  - Patterns (WriteIntent mouvements, Mandat poste, création OF depuis MPS)
  - Exemples pseudo-code Rust
  - Gestion erreurs et rollback
  - Intégration Stock et Planification

### 06_guides_implementation/
- **[Odoo Manufacturing - Guide Implémentation](./06_guides_implementation/Odoo%20Manufacturing%20-%20Guide%20Implementation.md)**
  - Architecture technique et crates Rust
  - Schémas de données (BOM, OF, WO, poste, gamme)
  - API et contrats
  - Plan de développement par phases (MVP → Complet)
  - Bornage fonctionnel et critères d'acceptation

---

## Résumé Exécutif

### Fonctionnalités principales identifiées

1. **Nomenclatures (BOM)** : Composants, quantités, types (normal/phantom/kit), consommation, ready to produce.
2. **Ordres de fabrication (OF)** : Création, confirmation, mouvements stock (raw/finished), clôture, backorder.
3. **Ordres de travail (WO)** : Création depuis gamme, dépendances, démarrage/fin, quantités et temps.
4. **Postes et gammes** : Postes de travail (capacité, coûts), gammes (routing) et opérations.
5. **Planification (MPS)** : Besoins, propositions OF, création OF validée.
6. **Rapports** : Délais, allocation, OEE, coûts OF.
7. **Atelier** : Tableau de bord poste (Shop Floor), Mandat poste.

### Architecture Miyukini proposée

**9 Opérateurs :**
- ManufacturingBOM, ManufacturingRouting, ManufacturingWorkCenter
- ManufacturingOrder, ManufacturingWorkOrder, ManufacturingPlanning
- ManufacturingReporting, ManufacturingUI, ManufacturingShopFloor

**1 Équipe d'Opérateurs :** ManufacturingService

**Niveaux de sécurité :** 1–2 (Standard à Sensitive)

**Correspondance Miyukini :** Miyukini Manufacturing (MiyuManufacturing) — ManufacturingService

---

## Statut de l'analyse

| Document | Statut | Version |
|----------|--------|---------|
| Logique Métier | ✅ Complété | 1.0 |
| Parcours Utilisateur | ✅ Complété | 1.0 |
| UI/UX | ✅ Complété | 1.0 |
| Intégrations Cross-App | ✅ Complété | 1.0 |
| Spécifications Opérateurs Miyukini | ✅ Complété | 1.0 |
| Guide Intégration COG | ✅ Complété | 1.0 |
| Guide Implémentation | ✅ Complété | 1.0 |

---

**Document** : Odoo Manufacturing — Index de Documentation  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : ✅ Analyse complète à 100% — référence pour implémentation Miyukini
