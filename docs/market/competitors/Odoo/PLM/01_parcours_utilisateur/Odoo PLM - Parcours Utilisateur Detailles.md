# Odoo PLM — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **PLM (Product Lifecycle Management)** d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 PLM

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Ingénieur Produit / R&D (Responsable ECO)

**Profil :**
- Rôle stratégique : Conception et évolution des produits et nomenclatures
- Responsabilités :
  - Créer des ECO (types adaptés : nouveau produit, mise à jour gamme, conformité)
  - Démarrer les révisions et modifier composants / opérations
  - Attacher les fichiers de conception (CAD, PDF)
  - Suivre l'avancement des ECO jusqu'à vérification
  - Collaborer via Chatter avec les approbateurs

**Besoins :**
- Vue par type d'ECO et par stage
- Accès rapide à la révision BoM depuis l'ECO
- Comparaison BoM actuelle vs révision (BoM Changes, Operation Changes)
- Gestion des documents dans l'ECO
- Notifications sur approbations et blocages

**Permissions :**
- Création et modification d'ECO
- Accès aux types d'ECO de son périmètre
- Accès Manufacturing (BoM) pour consulter et modifier les révisions

### 1.2 Approbateur (Stakeholder)

**Profil :**
- Rôle de validation : Vérifier les changements avant mise en production
- Responsabilités :
  - Consulter les ECO en stage de vérification
  - Examiner les différences (composants, opérations, documents)
  - Approuver ou refuser les changements
  - Débloquer le bouton Apply Changes par approbation

**Besoins :**
- Liste des ECO en attente d'approbation
- Résumé des changements (BoM Changes, Operation Changes) sans aller dans la révision
- Historique des approbations et commentaires
- Notification à l'arrivée d'un ECO en vérification

**Permissions :**
- Lecture ECO et révisions ; approbation selon configuration des stages (ECO Type)

### 1.3 Opérateur Fabrication (Proposition de changement)

**Profil :**
- Rôle terrain : Proposer des améliorations depuis le poste de travail
- Responsabilités :
  - Suggérer des modifications (ex. nouvelle étape de contrôle qualité)
  - Créer ou alimenter un ECO (ex. via alias email du type ECO)
  - Être assigné comme **Responsible** pour clarification
  - Collaborer avec l'ingénieur pour préciser la demande

**Besoins :**
- Création d'ECO simplifiée (ex. par email)
  - Alias email sur le type d'ECO : envoi d'un email crée un ECO
- Vue des ECO où il est responsable
- Instructions détaillées (Quality Control Points) dans les opérations BoM

**Permissions :**
- Création d'ECO (via formulaire ou email selon config)
- Accès limité aux ECO dont il est responsable ou participant

### 1.4 Responsable PLM / Chef de projet produit

**Profil :**
- Rôle coordination : Priorisation et suivi global des changements
- Responsabilités :
  - Configurer les types d'ECO et les stages
  - Définir les approbateurs par stage
  - Suivre le volume d'ECO par type et stage
  - Gérer les conflits (rebase) quand plusieurs ECO touchent la même BoM
  - Consulter l'historique des versions BoM (recalls, réclamations)

**Besoins :**
- Vue d'ensemble (PLM Overview) : Kanban par type d'ECO, compteurs
  - Ex. BOM Updates : nombre d'Engineering Changes
- Accès à la liste des ECO (filtres Done pour historique)
- Gestion des types d'ECO et des stages
- Apply Rebase pour résoudre les conflits

**Permissions :**
- Configuration des types d'ECO et stages
- Accès à tous les ECO et à l'historique des versions
- Droits Apply Changes et Apply Rebase

---

## 2. Parcours d'Onboarding

### 2.1 Premier accès à l'app PLM

**Étapes typiques :**
1. Accès à l'app **PLM** depuis le menu Supply Chain
2. **Overview** : Kanban des types d'ECO (ex. BOM Updates, New Product Introduction)
3. Clic sur un type d'ECO → liste des ECO de ce type (Kanban ou liste)
4. **New** : création d'un nouvel ECO (Type, Produit, BoM, Apply on, Responsible, Effective, Tags)
5. **Start Revision** : création de la révision BoM, apparition des onglets Documents et Revisions

**Points d'attention :**
- Choix du **Type** pour organiser les ECO par processus (équipes, responsabilités)
- **Apply on** : Bill of Materials nécessaire pour modifier composants/opérations
- **Product** obligatoire avant de sélectionner une BoM

### 2.2 Configuration (administrateur / responsable PLM)

**Étapes :**
1. Création des **ECO Types** (ex. BOM Updates, New Product Introduction, Regulatory)
2. Configuration des **stages** par type (Nouveau, En cours, Vérification, Clôture)
3. Définition des stages **vérification** (approbation requise)
4. Attribution des **approbateurs** par stage
5. Option : **alias email** sur un type d'ECO pour création d'ECO par email

**Intégration Quality (optionnel) :**
- Activer l'app Quality pour les Quality Control Points dans les opérations BoM
- Configurer les points de contrôle (Instructions, Register Production, etc.) pour les utiliser dans les révisions

---

## 3. Scénarios d'Usage Principaux

### 3.1 Créer et appliquer un ECO (changement BoM)

**Acteur :** Ingénieur Produit

**Parcours :**
1. PLM → choisir le type d'ECO (ex. BOM Updates) → **New**
2. Renseigner : Description, Type, Apply on = Bill of Materials, Product, BoM (auto si une seule), Responsible, Effective (As soon as possible ou At Date), Tags
3. **Start Revision** → révision BoM créée, smart button **Revisions** visible
4. Clic **Revisions** → ouverture de la révision BoM (marquée Archived)
5. Modifier **Components** : quantités, ajout/suppression de lignes
6. Modifier **Operations** (si Work Orders activé) : durée, poste, instructions (Quality)
7. Retour à l'ECO → onglet **BoM Changes** pour comparer révision vs production
8. Option : smart button **Documents** → ajouter/modifier/supprimer fichiers
9. Déplacer l'ECO vers le stage **Vérification**
10. Les approbateurs approuvent → bouton **Apply Changes** disponible
11. Clic **Apply Changes** → ECO en stage de clôture, révision devient BoM de production, ancienne BoM archivée, version BoM incrémentée

**Résultat :** BoM de production mise à jour avec traçabilité (version, ECO, effective date).

### 3.2 Approuver un ECO

**Acteur :** Approbateur

**Parcours :**
1. Notification ou liste des ECO en stage Vérification
2. Ouverture de l'ECO → onglets **BoM Changes** et **Operation Changes** pour revue
3. Option : ouvrir la **Révision** (smart button Revisions) pour détail
4. Chatter : commentaires, demande de précision
5. Action d'approbation (selon config : bouton Approve, validation stage)
6. Une fois toutes les approbations obtenues → **Apply Changes** visible pour le responsable ECO

**Résultat :** Changements débloqués pour application.

### 3.3 Consulter l'historique des versions BoM

**Acteur :** Responsable PLM ou Qualité (recalls, réclamations)

**Parcours :**
1. Manufacturing → Products → Bills of Materials → sélectionner la BoM
2. Smart button **ECO** → liste des ECO liés au produit
3. Filtre **Done** → historique des ECO appliqués (révision, responsable, effective date)
4. Clic sur un ECO → détail des composants, opérations, documents de cette version
5. BoM → onglet **Miscellaneous** → champ **Version** = version courante en production

**Résultat :** Traçabilité de quelle version était en vigueur à quelle date.

### 3.4 Résoudre un conflit (Rebase)

**Acteur :** Ingénieur ou Responsable PLM

**Contexte :** Plusieurs ECO ouverts sur la même BoM ; un ECO a déjà été appliqué (ex. ECO0011 → BoM v6), un autre (ECO0012) travaille encore sur v5.

**Parcours :**
1. Ouvrir ECO0012
2. Onglet **Previous Eco Bom Changes** : affiche les différences entre la BoM de production actuelle (v6) et la base de l'ECO0012 (v5)
3. Clic **Apply Rebase** : intégration des changements déjà appliqués (ex. ECO0011) dans la base de l'ECO0012, sans écraser les modifications propres à ECO0012
4. Poursuivre les modifications et appliquer ECO0012 quand prêt

**Résultat :** Conflit résolu ; les deux jeux de changements sont conservés.

### 3.5 Créer un ECO depuis l'email (alias)

**Acteur :** Opérateur ou toute personne autorisée

**Parcours :**
1. Configuration : type d'ECO avec **alias email** défini
2. Envoi d'un email à l'adresse de l'alias
3. Odoo crée un ECO dans ce type (stage initial)
4. Les responsables peuvent compléter l'ECO (Produit, BoM, révision, etc.)

**Résultat :** Remontée d'idées terrain sans passer par le formulaire complet.

---

## 4. Points de Friction Identifiés

### 4.1 Complexité du premier ECO

- **Problème :** Plusieurs champs obligatoires (Type, Product, BoM, Apply on) ; Start Revision non réversible immédiatement
- **Recommandation Miyukini :** Assistant guidé (étapes claires), validation progressive, annulation possible avant Apply Changes

### 4.2 Compréhension Revisions vs Production

- **Problème :** Utilisateur modifie la révision en pensant modifier la production ; confusion Archived / Production
- **Recommandation Miyukini :** Libellés explicites (Révision / Production), indicateur visuel fort (bannière « Révision — pas encore en production »)

### 4.3 Rebase et conflits

- **Problème :** Apply Rebase peu visible ; utilisateur peut continuer sur une base obsolète
- **Recommandation Miyukini :** Détection automatique de base obsolète, alerte + proposition Rebase, explication courte dans l'UI

### 4.4 Historique des versions

- **Problème :** Effective Date vide si "As soon as possible" ; contournement = regarder le Chatter (heure de passage en stage clôture)
- **Recommandation Miyukini :** Toujours enregistrer une date effective (ex. date d'Apply Changes) pour traçabilité

### 4.5 Documents et pièces jointes

- **Problème :** Fichiers sur BoM (Chatter) vs Documents dans l'ECO ; synchronisation après Apply Changes peut prêter à confusion
- **Recommandation Miyukini :** Un seul concept « Documents de l’ECO » ; après application, lien explicite « Documents désormais sur la BoM de production » avec audit

---

## 5. Recommandations pour Miyukini

- **Personas :** Différencier clairement Ingénieur, Approbateur, Opérateur, Responsable PLM avec Mandats de Permission et Contrats d'équipe adaptés.
- **Onboarding :** Tutoriel ciblé (création ECO, révision, approbation, application) et types d'ECO prédéfinis.
- **Workflow :** Modéliser les stages et approbations via StrongFather + TAMR (intervention humaine) ; Apply Changes = décision + WriteIntent KindMother.
- **Traçabilité :** Version et effective date systématiques ; historique consultable sans contournement.
- **Rebase :** Détection de base obsolète + flux guidé Apply Rebase avec explication.
- **Documents :** Un seul flux « documents ECO » → après application, traçabilité sur la BoM (MiyuMedia + gouvernance).

---

**Document rédigé selon la méthodologie d'analyse Odoo.**
