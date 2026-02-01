# Odoo Quality — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Quality** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 19.0, module Quality

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo (Stock, MRP, Mail)
- Flux de données inter-apps
- Mécanismes d'intégration (QCP → contrôles sur MO, picking, work order)
- Données partagées (produits, ordres, équipes)
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules Requis

**Dépendances explicites (typiques) :**
- **stock** : Opérations d'inventaire (pickings : réception, livraison, retour) ; création de contrôles sur les mouvements de stock
- **mail** : Chatter, followers, activités sur contrôles et alertes (optionnel mais courant)

Sans **stock**, l'app Quality ne peut pas créer de contrôles sur les pickings. Sans **mrp** (Manufacturing), pas de contrôles sur les ordres de fabrication ni sur les work orders.

### 1.2 Modules Optionnels

- **MRP (Manufacturing)** :
  - QCP sur opération **Manufacturing** : création de contrôles à la confirmation des MO
  - **Work Order Operation** : QCP ciblant une opération de travail (work order) ; contrôles traités dans le **Shop Floor**
  - Boutons Quality Checks et Quality Alert sur les MO
- **Mail** : Chatter et notifications sur quality checks et quality alerts

---

## 2. Intégrations Détaillées

### 2.1 Stock (Inventory)

**Flux :**
```
QCP (operation = Receipt/Delivery/Return/…) → picking créé/validé
  → quality.check créés (liés au picking)
Picking → bouton Quality Checks → pop-up liste des contrôles
Picking → bouton Quality Alert (ou ⚙️ ‣ Quality Alert)
```

**Mécanismes :**
- QCP définit les opérations stock concernées ; à la création ou validation du picking, les contrôles sont créés selon Control Per et Control Frequency
- Champ **Picking** sur quality.check et quality.alert pour lier au bon de mouvement
- « # To Process » sur les cartes d'opération (Réceptions, Livraisons, etc.) permet de filtrer et ouvrir les ordres ; les contrôles s'affichent sur le picking

**Recommandations pour Miyukini :**
- Service Quality découplé ; intégration Inventory via Contrat d'équipe (MiyuInventory ou équivalent) : création de contrôles sur événements « picking créé / validé », lecture picking pour affichage contexte
- WriteIntent pour lier un contrôle à un picking sans couplage fort

### 2.2 MRP (Manufacturing)

**Flux :**
```
QCP (operation = Manufacturing, optional Work Order Operation)
  → MO confirmé → quality.check créés (liés au MO ou au work order)
MO → bouton Quality Checks → pop-up liste des contrôles
MO → bouton Quality Alert (si au moins un contrôle pour ce MO)
Work order (Shop Floor) → étape contrôle (créée par QCP) → pop-up Pass/Fail ou Validate
Shop Floor ‣ carte WO ‣ ⋮ → Create a Quality Alert (Product, Work Center pré-remplis)
```

**Mécanismes :**
- QCP avec opération **Manufacturing** : déclenchement à la confirmation du MO (ou selon fréquence)
- **Work Order Operation** : QCP cible une opération de travail précise ; les contrôles sont des étapes du work order et sont traités dans le module Shop Floor
- Champs **Production Order** (MO), **Work Center**, **Work Order** sur quality.check et quality.alert
- Contrôles pour work order : création **uniquement** par QCP (pas de création manuelle pour un WO)

**Recommandations pour Miyukini :**
- Intégration Manufacturing via Contrat d'équipe (MiyuManufacturing ou équivalent) : événements « MO confirmé », « WO démarré » pour créer les contrôles ; exposition MO/WO en lecture pour contexte
- WriteIntent pour lier contrôle à MO / work order ; pas d’autorité Quality sur la planification MRP

### 2.3 Mail

**Flux :**
```
quality.check / quality.alert → mail.thread → Chatter, Followers, Activities
```

**Mécanismes :**
- Héritage mail.thread sur les modèles quality (checks, alerts) pour commentaires, pièces jointes, followers, activités planifiées

**Recommandations pour Miyukini :**
- Intégration MiyuNotify pour notifications et suivi sur contrôles et alertes

### 2.4 Produits et Catégories

**Flux :**
```
product.product / product.category → QCP (Products, Product Categories)
QCP → quality.check (produit(s) concernés déduits de l’opération)
```

**Mécanismes :**
- QCP filtre par produit(s) ou catégorie(s) ; si vide, tous les produits de l’opération sont concernés
- Contrôles créés avec référence au(x) produit(s) de l’opération (MO, picking)

**Recommandations pour Miyukini :**
- Référence produits/catégories via catalogue partagé (KindMother ou Opérateur Catalogue) ; pas de duplication des données produit dans le service Quality

---

## 3. Flux de Données Résumés

| Source | Cible | Données / Usage |
|--------|--------|------------------|
| stock.picking | quality.check | Création contrôles (QCP), lien picking_id |
| stock.picking | quality.alert | Contexte alerte (picking_id) |
| mrp.production | quality.check | Création contrôles (QCP), lien production_id |
| mrp.production | quality.alert | Contexte alerte (MO) |
| mrp.workorder | quality.check | Contrôles sur WO (créés par QCP), traitement Shop Floor |
| mrp.workcenter | quality.alert | Work Center concerné |
| product.product | QCP, check, alert | Produits concernés |
| product.category | QCP | Catégories concernées |
| quality.team | QCP, check, alert | Équipe responsable |
| res.users | quality.team | Membres équipe, Responsible |
| res.company | QCP, check, alert, team | company_id (multi-société) |
| mail | check, alert | Chatter, followers, activités |

---

## 4. APIs et Hooks (conceptuels)

**Typiques dans un module Odoo Quality :**
- **Création automatique des contrôles** : à la confirmation MO, à la validation/création picking, selon règles QCP (Control Per, Control Frequency)
- **Vérification des conditions QCP** : opération, produit/catégorie, fréquence (All / Randomly / Periodically)
- **Traitement du contrôle** : mise à jour état (passed/failed), déclenchement éventuel « Message If Failure » (création alerte, notification)
- **Contraintes d'accès** : ir.model.access, ir.rule (quality.user, quality.manager)
- **Boutons sur MO / picking** : visibilité conditionnelle (au moins un contrôle pour l’ordre pour Quality Alert)

**Recommandations pour Miyukini :**
- Exposer la création de contrôles (automatique et manuelle) comme flux gouvernés : StrongFather (décision), KindMother (WriteIntent)
- « Message If Failure » : règle configurée (alerte, notification) gérée par le service Quality en collaboration avec MiyuNotify, sans logique métier dans l’UI seule

---

## 5. Synthèse pour Miyukini

- **Stock** : Intégration obligatoire pour contrôles sur pickings ; Contrat d’équipe avec MiyuInventory (ou équivalent), événements picking → création contrôles, lecture picking pour contexte
- **MRP** : Intégration optionnelle pour contrôles sur MO et work orders ; Contrat d’équipe avec MiyuManufacturing (ou équivalent), événements MO/WO → création contrôles, Shop Floor pour traitement WO
- **Mail** : Intégration MiyuNotify pour chatter, followers, activités sur contrôles et alertes
- **Produits** : Référence catalogue partagé (pas de duplication)
- **Sécurité** : Mandats et permissions par équipe qualité et par type d’opération (création QCP, création contrôle manuel, création alerte, traitement contrôle)

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
