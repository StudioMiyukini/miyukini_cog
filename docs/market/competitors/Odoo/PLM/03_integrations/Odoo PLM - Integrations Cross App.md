# Odoo PLM — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **PLM (Product Lifecycle Management)** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 19.0 PLM, dépendances Supply Chain

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo
- Flux de données inter-apps
- Mécanismes d'intégration
- APIs et hooks utilisés
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules Requis (conceptuels)

**Dépendances explicites (app PLM, typiquement `mrp_plm`) :**
- **mrp** (Manufacturing) : Nomenclatures (BoM), composants, opérations ; modèle central partagé
- **stock** (Inventory) : Produits, données d'approvisionnement (fournisseur, délais) sur les composants
- **mail** : Chatter, notifications, collaboration sur les ECO
- **web** : Framework web, vues, formulaire

**Rôle :**
- **mrp** : PLM étend les BoM avec versioning, ECO et révisions ; pas de BoM sans Manufacturing
- **stock** : lecture des données produit/composant ; pas de modification directe des stocks par PLM
- **mail** : suivi des approbations, commentaires, activités sur les ECO

### 1.2 Modules Optionnels

- **quality** (Quality) : Quality Control Points dans les opérations BoM ; instructions détaillées (Steps) ; comparaison dans ECO (Operation Changes)
- **WhatsApp** (intégration externe) : communication d'équipe (documentation Odoo)

---

## 2. Flux de Données Inter-Apps

### 2.1 PLM ↔ Manufacturing (mrp)

**Flux :**
```
PLM ECO → Start Revision → Copie BoM (mrp.bom)
         → Modifications sur la révision (composants, opérations)
         → Apply Changes → BoM de production mise à jour (mrp.bom)
                          → Ancienne BoM archivée
```

**Données partagées :**
- **mrp.bom** : modèle BoM (composants, opérations, version)
- **mrp.bom.line** : lignes de composants
- **mrp.routing.workcenter** (ou équivalent) : opérations
- Champ **version** sur la BoM (affiché onglet Miscellaneous)
- Lien ECO ↔ BoM : liste des ECO par produit/BoM (smart button ECO sur la BoM)

**Sens des flux :**
- **PLM → mrp** : Apply Changes écrit la révision comme nouvelle BoM de production (archive l’ancienne, met à jour la version)
- **mrp → PLM** : Lecture de la BoM courante pour créer la révision ; Manufacturing reste la source des nomenclatures utilisées en production (ordres de fabrication)

**Règle :** Les ordres de fabrication (Manufacturing Orders) utilisent toujours la **BoM de production courante** ; les révisions PLM ne les affectent qu’après Apply Changes.

### 2.2 PLM ↔ Inventory (stock)

**Flux :**
- **Lecture** : produits (`product.product`, `product.template`), composants ; informations d’approvisionnement (fournisseur, délai, prix) pour affichage et achats
- **Pas d’écriture** : PLM ne crée pas de mouvements de stock ; il gère la définition produit/BoM, pas les quantités en entrepôt

### 2.3 PLM ↔ Quality (quality)

**Flux :**
- **Quality Control Points** : définis dans l’app Quality ; attachés aux opérations BoM (Steps / Instructions)
- Dans une **révision BoM** : les opérations peuvent inclure / modifier ces steps ; ajout de nouvelles instructions (ex. « New Step Suggestion »)
- **Operation Changes** (ECO) : affichage des différences d’opérations, incluant Step, Step Type, Work Center, Manual Duration Change
- **Réordonnancement** : dans la révision BoM, glisser-déposer des lignes d’instructions (Quality)

**Données partagées (conceptuelles) :**
- Modèle des points de contrôle qualité (instructions par opération / produit / quantité)
- Types : Instructions, Register Production, Take a Picture, etc.

### 2.4 PLM ↔ Mail (mail)

**Flux :**
- **Chatter** sur l’ECO : commentaires, pièces jointes, activités
- **Notifications** : assignation responsable, passage en stage vérification, approbations
- **Activités** : tâches, délais, rappels
- **Followers** : abonnés à l’ECO pour suivi des changements

**Usage :** Collaboration, traçabilité des décisions, contournement pour « date effective » quand Effective = As soon as possible (heure de passage en stage clôture).

---

## 3. Mécanismes d'Intégration

### 3.1 Extension du modèle BoM (mrp)

- **Champs ajoutés par PLM** (conceptuels) : version, lien vers ECO courante / révisions
- **Comportements** : 
  - Création d’une copie BoM (révision) à Start Revision
  - Archive / désarchive et remplacement de la BoM de production à Apply Changes
- **Smart button ECO** sur la BoM : liste des ECO liés au produit (visible si app PLM installée)

### 3.2 Workflow et étapes (ECO Type)

- **Stages** : définis par type d’ECO ; certains stages « vérification » déclenchent la nécessité d’approbations
- **Approvals** : mécanisme (interne ou intégré) pour débloquer Apply Changes
- Pas de détail d’API dans la doc ; en Miyukini : modéliser via StrongFather (décision) + TAMR (intervention humaine)

### 3.3 Alias email (création ECO)

- **Type d’ECO** peut avoir un **alias email**
- Email reçu sur cet alias → création automatique d’un ECO dans ce type (stage initial)
- Intégration mail / messagerie Odoo

### 3.4 Pièces jointes et Documents

- **BoM** : pièces jointes via Chatter (modèle mail / ir.attachment)
- **ECO** : smart button Documents → gestion des attachments dans le contexte ECO ; après Apply Changes, liaison des nouveaux fichiers à la BoM de production, archivage des retirés
- Stockage : modèle standard Odoo des pièces jointes (ir.attachment) avec res_id / res_model ECO ou BoM

---

## 4. APIs et Hooks (conceptuels)

### 4.1 Création révision BoM

- **Start Revision** : 
  - Création d’une copie de la BoM de production
  - Affectation du numéro de version suivant (V2, V3, …)
  - Affichage des stages de l’ECO Type
  - Rendu visible des smart buttons Revisions et Documents

### 4.2 Application des changements

- **Apply Changes** :
  - Validation des approbations (stage vérification)
  - Archive de l’ancienne BoM de production
  - « Promotion » de la révision en BoM de production (désarchivage, mise à jour version)
  - Synchronisation des documents ECO → BoM de production
  - Passage de l’ECO en stage de clôture

### 4.3 Rebase

- **Apply Rebase** :
  - Détection : base de l’ECO obsolète (autre ECO déjà appliqué)
  - Fusion : intégration des changements de la BoM de production actuelle dans la base de l’ECO courant, sans écraser les modifications locales de l’ECO
  - Onglet **Previous Eco Bom Changes** pour visualiser les écarts avant rebase

### 4.4 Comparaison BoM (BoM Changes / Operation Changes)

- Calcul des différences entre révision et BoM de production (composants et opérations)
- Affichage avec type (Add / Remove / Update) et code couleur (bleu / noir / rouge)

---

## 5. Tableau Récapitulatif des Intégrations

| App cible   | Type de flux     | Données / Actions                                      |
|------------|------------------|--------------------------------------------------------|
| **Manufacturing** | Lecture / Écriture | BoM (révision, production), version, composants, opérations ; Apply Changes met à jour la BoM de production |
| **Inventory**     | Lecture           | Produits, composants, infos approvisionnement          |
| **Quality**       | Lecture / Écriture | Quality Control Points (Steps) dans opérations BoM ; révision peut les modifier |
| **Mail**          | Lecture / Écriture | Chatter, notifications, activités, alias email → ECO  |

---

## 6. Recommandations pour Miyukini

- **Manufacturing / BoM** : Un seul modèle « nomenclature » (KindMother) avec versioning ; PLM = couche de gouvernance (ECO, révisions, Apply Changes) sans dupliquer le modèle métier BoM.
- **Flux Apply Changes** : StrongFather (décision d’appliquer) + KindMother (WriteIntent : bascule de version, archive ancienne BoM).
- **Approvals** : TAMR (intervention humaine) + StrongFather (validation) ; pas d’approbation = pas d’Apply Changes.
- **Documents** : MiyuMedia (ou équivalent) avec lien explicite ECO → BoM après application ; traçabilité et audit.
- **Quality** : Contrat d’équipe entre Opérateur PLM et Opérateur Quality (lecture/écriture des steps dans les opérations) sous Mandat.
- **Mail / Notifications** : MiyuNotify pour alertes (stage vérification, approbation, Apply Changes) et collaboration (commentaires liés à l’ECO).
- **Rebase** : Ever Buddy (compatibilité versions) + logique de fusion explicite ; détection de base obsolète et proposition Apply Rebase dans l’UI.

---

**Document rédigé selon la méthodologie d'analyse Odoo.**
