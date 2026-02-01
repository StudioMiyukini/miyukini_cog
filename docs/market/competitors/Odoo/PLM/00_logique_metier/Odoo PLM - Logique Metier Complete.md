# Odoo PLM — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **PLM (Product Lifecycle Management)** d'Odoo (version 19.0). Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 PLM, app `mrp_plm`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données (ECO, ECO Type, révisions BoM, documents)
- Règles métier et contraintes (gestion des changements, approbations)
- Workflows (création ECO → révision → approbation → application)
- Contrôle de version des nomenclatures (BoM)
- Gestion des fichiers de conception (CAD, PDF)
- Rebase et résolution de conflits ECO concurrents
- Intégrations Manufacturing, Inventory, Quality

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Engineering Change Order (ECO)

**Rôle :** Représente un **ordre de modification technique** — encapsule les changements sur un produit ou une nomenclature (BoM) sans affecter la production en cours.

**Champs clés (conceptuels) :**

#### Identification
- `name` : Nom / référence ECO (ex. ECO005)
- `description` : Résumé de l'amélioration
- `type_id` : Type d'ECO (catégorisation, étapes projet)
- `company_id` : Entreprise (multi-company, optionnel)

#### Périmètre
- `apply_on` : Périmètre — **Bill of Materials** ou **Product Only**
- `product_id` : Produit concerné
- `bom_id` : Nomenclature (BoM) modifiée — auto-rempli si le produit a une BoM

#### Responsabilité et planification
- `user_id` / `responsible_id` : Responsable de l'ECO
- `effective` : Quand l'ECO devient actif — **As soon as possible** ou **At Date** (date précise)
- `tag_ids` : Tags pour priorisation et organisation

#### Révision et états
- Révision BoM : copie de la BoM de production stockée dans l'ECO (bouton **Revisions**)
- Numéro de version attribué (V2, V3, …)
- Stages de l'ECO Type affichés (Nouveau, En cours, Vérification, Clôture)
- **Documents** : fichiers de conception attachés à l'ECO

**Règles métier :**
- Un **Produit** doit être sélectionné avant de pouvoir choisir une **BoM**
- **Start Revision** : crée la copie de la BoM de production, affiche l’onglet Documents, assigne la version suivante, affiche les stages
- Le bouton **Revisions** n’apparaît que si **Apply on** = Bill of Materials et que **Start Revision** a été cliqué
- Les modifications dans l’ECO n’affectent **pas** la BoM de production tant que **Apply Changes** n’est pas exécuté
- **Apply Changes** : déplace l’ECO en stage de clôture, archive l’ancienne BoM de production, la révision devient la nouvelle BoM de production

**Workflow :**
```
Nouveau → Start Revision → Modifications (composants, opérations, documents)
  → Stage vérification (approbations) → Apply Changes → Clôture
```

---

### 1.2 ECO Type (Type d'ordre de modification)

**Rôle :** Catégorise et organise les ECO par type de changement (nouveau produit, mise à jour gamme, conformité réglementaire). Chaque type a ses **stages** (jalons) configurables.

**Champs clés (conceptuels) :**
- `name` : Nom du type (ex. BOM Updates, New Product Introduction)
- `stage_ids` : Étapes / stages du workflow ECO
- Filtrage : les employés ne voient que les ECO du type relevant de leurs responsabilités

**Règles métier :**
- Les stages incluent des étapes de **vérification** qui exigent une **approbation** avant que **Apply Changes** soit disponible
- Configuration des approbateurs par stage

---

### 1.3 Révision BoM (Bill of Materials — Nomenclature)

**Rôle :** Copie de la BoM de production créée dans le cadre d’un ECO. Elle est marquée **Archived** (test / révision) tant que l’ECO n’est pas appliqué.

**Champs clés (conceptuels) :**
- Même structure qu’une BoM Manufacturing : **Components**, **Operations**
- Lien vers l’ECO parent
- Version (V2, V3, …)
- **Components** : lignes de composants (quantité, produit) — ajout / suppression / modification
- **Operations** : opérations de fabrication — durée, poste de travail, instructions (Quality Control Points)

**Règles métier :**
- Modifications autorisées uniquement sur la révision (pas sur la BoM de production)
- Après **Apply Changes** : la révision perd le statut Archived et devient la BoM de production ; l’ancienne BoM est archivée
- **BoM Changes** (onglet ECO) : comparaison révision vs BoM de production (bleu = ajout, noir = commun, rouge = supprimé)
- **Operation Changes** (onglet ECO) : comparaison des opérations (Add, Remove, Update)

---

### 1.4 Version et historique BoM

**Rôle :** Chaque BoM a un numéro de **Version** (onglet Miscellaneous). L’historique des versions est porté par la liste des ECO associés à la BoM / au produit.

**Champs clés (conceptuels) :**
- **Version** (BoM) : version courante en production
- **ECO** (smart button sur BoM) : liste des ECO du produit — filtre **Done** pour l’historique des révisions
- Pour chaque ECO : responsable, **Effective Date** (si "At Date" est choisi)
- Si **Effective** = "As soon as possible", aucune date n’est enregistrée dans l’historique de révision

**Règles métier :**
- Traçabilité : quelle version de BoM était en vigueur à une date donnée (recalls, réclamations)
- Possibilité de **revenir** à une version précédente (revert) via un nouvel ECO

---

### 1.5 Documents / Fichiers de conception

**Rôle :** Fichiers CAD, PDF, images attachés à une BoM ou à un ECO.

**Comportement :**
- **Sur la BoM** : pièce jointe via Chatter (icône trombone)
- **Dans l’ECO** : smart button **Documents** — ajout, modification, suppression de fichiers
- Fichiers archivés dans l’ECO : retirés de la BoM mais restent accessibles dans l’ECO
- À l’**Apply Changes** : les nouveaux fichiers de l’ECO sont liés à la BoM de production

**Règles métier :**
- Les changements de fichiers dans l’ECO n’affectent la BoM de production qu’après application des changements
- Fichiers supprimés dans l’ECO : archivés, pas supprimés définitivement

---

### 1.6 Rebase (conflits ECO concurrents)

**Rôle :** Résolution de conflits lorsque plusieurs ECO modifient la même BoM alors que la production a déjà été mise à jour par un autre ECO.

**Contexte :**
- BoM en version 5
- ECO0011 et ECO0012 créés sur la version 5
- ECO0011 appliqué → BoM passe en version 6
- ECO0012 travaille donc sur une **ancienne** base (version 5)

**Mécanisme :**
- Onglet **Previous Eco Bom Changes** : affiche les différences entre la BoM de production actuelle et la base de l’ECO en cours
- Bouton **Apply Rebase** : met à jour la base de l’ECO avec les changements déjà appliqués (ex. ECO0011), sans écraser les changements propres à l’ECO (ECO0012)
- Permet de garder les modifications de l’ECO courant tout en intégrant les changements déjà en production

**Règles métier :**
- Rebase = fusion des changements, pas remplacement
- Traçabilité et réversibilité conservées

---

## 2. Workflows et Transitions d'État

### 2.1 Workflow ECO

```
Création ECO (Type, Produit, BoM, Apply on)
  → Start Revision
       → [Révision BoM créée, Documents disponible, Stages affichés]
  → Modifications (Composants, Opérations, Documents)
  → Déplacement vers stage de Vérification
       → Approbations requises
  → Apply Changes (disponible après approbation)
       → ECO en stage de clôture
       → Ancienne BoM archivée, Révision = nouvelle BoM de production
       → Version BoM incrémentée
```

### 2.2 Workflow Approbations

- Stages de type **vérification** : un ou plusieurs approbateurs doivent accepter
- Une fois les approbations obtenues : bouton **Apply Changes** activé
- Pas d’application des changements sans passage par le workflow d’approbation (configurable par ECO Type)

### 2.3 Workflow Documents

- Ajout / modification / suppression de fichiers dans l’ECO (smart button Documents)
- **Apply Changes** : synchronisation des fichiers vers la BoM de production (nouveaux liés, archivés retirés)

---

## 3. Règles Métier et Contraintes

### 3.1 Périmètre ECO

- **Apply on = Bill of Materials** : modifications sur la BoM (composants, opérations) ; révision BoM créée
- **Apply on = Product Only** : pas de révision BoM ; usage pour changements produit uniquement (ex. attributs, pas de BoM)

### 3.2 Contraintes Produit et BoM

- **Product** obligatoire pour accéder au choix de **Bill of Materials**
- Si le produit a une seule BoM, elle est auto-sélectionnée ; sinon choix explicite
- La BoM modifiée doit être la BoM de production (active) au moment du Start Revision

### 3.3 Contrôle de version

- Version BoM : entier (1, 2, 3, …) dans l’onglet Miscellaneous de la BoM
- Chaque **Apply Changes** incrémente la version
- Historique complet via liste des ECO (filtrés Done) sur la BoM

### 3.4 Effective Date

- **As soon as possible** : application dès **Apply Changes** ; pas de date enregistrée dans l’historique
- **At Date** : application à une date donnée ; traçabilité « quelle version à quelle date » pour rappels et conformité

### 3.5 Quality Control Points (Quality app)

- Les **opérations** BoM peuvent inclure des **instructions** (Quality Control Points)
- Configuration dans l’app **Quality** : titre, Control per (Product / Operation / Quantity), Type (Instructions, Take a Picture, etc.)
- Dans l’ECO, onglet **Operation Changes** peut afficher Step, Step Type, Work Center, Manual Duration Change
- Réordonnancement des instructions par glisser-déposer dans la révision BoM

### 3.6 Intégrité et traçabilité

- Aucune modification directe de la BoM de production depuis l’ECO
- Toute mise en production passe par **Apply Changes**
- Revert possible en créant un nouvel ECO qui réapplique une version antérieure (conceptuellement)

---

## 4. Intégrations avec Autres Modules

### 4.1 Manufacturing (`mrp`)

- **BoM** : modèle central partagé (nomenclature, composants, opérations)
- **PLM** étend la BoM avec versioning et ECO
- Accès BoM : Manufacturing → Products → Bills of Materials ; ou PLM → Master Data → Bill of Materials
- Work Orders (Manufacturing) utilisent la BoM de production courante ; les révisions ECO ne les affectent qu’après Apply Changes

### 4.2 Inventory (`stock`)

- Données d’approvisionnement (fournisseur, prix, délai) sur les composants — utilisées en amont (procurement)
- Pas de modification directe des stocks par le PLM ; le PLM agit sur la définition produit/BoM

### 4.3 Quality (`quality`)

- **Quality Control Points** : instructions détaillées dans les opérations BoM
- Configuration des points de contrôle (Instructions, Register Production, Take a Picture, etc.)
- Les étapes (Steps) apparaissent dans l’ECO (Operation Changes) pour comparaison et ajout dans une révision

### 4.4 Mail / Chatter

- Collaboration : invitations, commentaires, activités sur l’ECO
- Log des approbations et discussions liées à l’ECO

### 4.5 WhatsApp (optionnel)

- Communication d’équipe (documentation Odoo)

---

## 5. Synthèse des Entités Métier

| Entité | Rôle principal |
|--------|-----------------|
| **ECO** | Ordre de modification ; conteneur des changements (révision BoM, documents, stages) |
| **ECO Type** | Catégorie d’ECO ; workflow avec stages et approbations |
| **BoM (révision)** | Copie de la BoM dans l’ECO ; modifiable jusqu’à Apply Changes |
| **BoM (production)** | Nomenclature courante ; mise à jour uniquement via Apply Changes |
| **Version** | Numéro de version de la BoM (traçabilité) |
| **Documents** | Fichiers de conception (CAD, PDF) attachés à l’ECO / BoM |
| **Approvals** | Étapes de vérification ; déblocage de Apply Changes |

---

## 6. Correspondance Miyukini

**Service proposé :** `MiyukiniPLM` ou `MiyuPLM`

**Concepts clés à mapper :**
- **ECO** → Intention de changement produit/BoM (WriteIntent + workflow gouverné)
- **Révision BoM** → Branche de modification (KindMother : versioning, pas d’écrasement production)
- **Apply Changes** → Décision StrongFather + WriteIntent KindMother (bascule version)
- **Approvals** → TAMR (intervention humaine) + StrongFather (validation)
- **Documents** → MiyuMedia / stockage gouverné avec traçabilité
- **Rebase** → Ever Buddy (compatibilité versions) + résolution conflits explicite

---

**Document rédigé selon la méthodologie d'analyse Odoo — Référence :** Documentation Odoo 19.0 PLM, Engineering Change Orders, Version Control.
