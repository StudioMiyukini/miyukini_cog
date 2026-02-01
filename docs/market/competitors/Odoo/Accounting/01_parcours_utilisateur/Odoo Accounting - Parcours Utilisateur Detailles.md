# Odoo Accounting — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application Accounting d'Odoo, identifiant les personas, scénarios d'usage, étapes d'onboarding et points de friction pour guider l'implémentation d'un équivalent dans Miyukini.

**Source d'analyse :** Interface utilisateur Odoo, workflows identifiés dans le code, documentation utilisateur.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux
- Points de friction identifiés
- Recommandations pour Miyukini

**Hors scope :**
- Détails techniques d'implémentation
- Spécifications UI/UX détaillées (document dédié)

---

## 1. Personas et Rôles

### 1.1 Comptable (Accountant)

**Profil :**
- Utilisateur principal de l'application
- Gère le plan comptable, les journaux, les écritures
- Valide les factures et écritures
- Effectue les rapprochements bancaires

**Permissions :**
- Accès complet aux écritures
- Validation des écritures
- Modification du plan comptable
- Rapprochements bancaires

### 1.2 Responsable Financier (Financial Manager)

**Profil :**
- Supervise la comptabilité
- Valide les écritures importantes
- Consulte les rapports
- Gère les budgets

**Permissions :**
- Lecture de toutes les écritures
- Validation des écritures (selon seuils)
- Accès aux rapports
- Configuration des journaux

### 1.3 Utilisateur Standard (Standard User)

**Profil :**
- Saisit des factures ou écritures simples
- Consulte ses propres écritures
- Pas d'accès à la configuration

**Permissions :**
- Création de factures clients/fournisseurs
- Consultation de ses propres écritures
- Pas de validation ni de modification du plan comptable

---

## 2. Parcours d'Onboarding

### 2.1 Première Installation

**Étapes :**

1. **Configuration de l'entreprise**
   - Nom, adresse, SIRET/SIREN
   - Devise principale
   - Pays fiscal
   - Date de début d'exercice

2. **Installation du plan comptable**
   - Choix du plan comptable (PCG France, etc.)
   - Import automatique ou manuel
   - Vérification des comptes

3. **Configuration des journaux**
   - Journal des ventes
   - Journal des achats
   - Journal bancaire
   - Journal de caisse
   - Journal divers

4. **Configuration des taxes**
   - Taux de TVA (20%, 10%, 5.5%, etc.)
   - Comptes de taxes
   - Règles de répartition

5. **Configuration des conditions de paiement**
   - Conditions standards (30 jours, 60 jours, etc.)
   - Conditions avec escompte

6. **Saisie des soldes d'ouverture**
   - Balance d'ouverture des comptes
   - Vérification de l'équilibre

**Durée estimée :** 2-4 heures pour une première installation complète

**Points de friction identifiés :**
- Complexité de la configuration initiale
- Nécessité de connaissances comptables
- Risque d'erreur dans les soldes d'ouverture

### 2.2 Première Utilisation (Utilisateur Standard)

**Étapes :**

1. **Accès à l'application**
   - Connexion
   - Navigation vers "Factures" ou "Écritures"

2. **Création d'une première facture**
   - Sélection du type (Facture client)
   - Choix du client
   - Ajout de lignes produits
   - Validation

3. **Envoi de la facture**
   - Génération du PDF
   - Envoi par email

**Durée estimée :** 15-30 minutes

**Points de friction identifiés :**
- Interface complexe pour débutants
- Nombreux champs à remplir
- Nécessité de comprendre les comptes comptables

---

## 3. Scénarios d'Usage Principaux

### 3.1 Scénario : Émission d'une Facture Client

**Acteur :** Comptable ou Vendeur

**Étapes :**

1. **Création de la facture**
   - Menu : Factures → Factures clients → Créer
   - Sélection du client (recherche ou création)
   - Date de facture
   - Date d'échéance ou conditions de paiement

2. **Ajout des lignes**
   - Ajout de produits/services
   - Quantité, prix unitaire
   - Taxes applicables
   - Remises éventuelles

3. **Vérification**
   - Montant HT, TTC
   - Comptes comptables automatiques
   - Conditions de paiement

4. **Validation**
   - Bouton "Valider"
   - Génération automatique du numéro
   - Création des lignes comptables

5. **Envoi**
   - Génération PDF
   - Envoi par email au client

**Durée estimée :** 5-10 minutes par facture

**Points de friction :**
- Recherche de produits parfois lente
- Calculs automatiques pas toujours évidents
- Gestion des taxes complexes

### 3.2 Scénario : Saisie d'une Facture Fournisseur

**Acteur :** Comptable

**Étapes :**

1. **Création de la facture**
   - Menu : Factures → Factures fournisseurs → Créer
   - Sélection du fournisseur
   - Référence fournisseur
   - Date de facture

2. **Saisie des lignes**
   - Ajout manuel ou import OCR
   - Comptes comptables
   - Taxes
   - Montants

3. **Validation**
   - Vérification des montants
   - Validation
   - Génération des écritures comptables

**Durée estimée :** 10-15 minutes par facture

**Points de friction :**
- Saisie manuelle fastidieuse
- OCR parfois imprécis
- Gestion des acomptes

### 3.3 Scénario : Rapprochement Bancaire

**Acteur :** Comptable

**Étapes :**

1. **Import des relevés**
   - Import fichier bancaire (OFX, CSV)
   - Ou saisie manuelle

2. **Correspondance automatique**
   - Algorithme de correspondance
   - Suggestions de rapprochement

3. **Validation manuelle**
   - Vérification des correspondances
   - Correction si nécessaire
   - Rapprochement des lignes

4. **Validation**
   - Validation du rapprochement
   - Création des écritures de rapprochement

**Durée estimée :** 30-60 minutes par relevé

**Points de friction :**
- Correspondances automatiques pas toujours fiables
- Interface de rapprochement complexe
- Gestion des écarts de change

### 3.4 Scénario : Enregistrement d'un Paiement

**Acteur :** Comptable

**Étapes :**

1. **Sélection de la facture**
   - Menu : Factures → Factures clients
   - Sélection de la facture à payer

2. **Enregistrement du paiement**
   - Bouton "Enregistrer un paiement"
   - Montant payé
   - Date de paiement
   - Moyen de paiement
   - Journal bancaire/caisse

3. **Rapprochement automatique**
   - Rapprochement avec la facture
   - Mise à jour de l'état de paiement

**Durée estimée :** 2-5 minutes par paiement

**Points de friction :**
- Gestion des paiements partiels
- Gestion des paiements multiples factures

### 3.5 Scénario : Consultation des Rapports

**Acteur :** Responsable Financier

**Étapes :**

1. **Accès aux rapports**
   - Menu : Rapports → Comptabilité
   - Sélection du rapport (Grand livre, Balance, etc.)

2. **Filtrage**
   - Période
   - Comptes
   - Journaux
   - Partenaires

3. **Export**
   - Export PDF ou Excel
   - Impression

**Durée estimée :** 5-10 minutes par rapport

**Points de friction :**
- Temps de génération parfois long
- Options de filtrage complexes

---

## 4. Points de Friction Identifiés

### 4.1 Complexité Initiale

**Problème :**
- Configuration initiale longue et complexe
- Nécessite des connaissances comptables
- Risque d'erreur élevé

**Impact :** Découragement des nouveaux utilisateurs

**Recommandation pour Miyukini :**
- Assistant d'onboarding guidé
- Configuration par défaut intelligente
- Validation automatique des configurations

### 4.2 Interface Dense

**Problème :**
- Nombreux champs visibles simultanément
- Informations techniques (comptes comptables) visibles même pour utilisateurs non-comptables
- Navigation complexe

**Impact :** Courbe d'apprentissage élevée

**Recommandation pour Miyukini :**
- Interface progressive (affichage selon rôle)
- Masquage des détails techniques par défaut
- Navigation simplifiée

### 4.3 Gestion des Erreurs

**Problème :**
- Messages d'erreur techniques
- Pas toujours clairs sur la cause
- Difficile de corriger

**Impact :** Frustration utilisateur

**Recommandation pour Miyukini :**
- Messages d'erreur explicites
- Suggestions de correction
- Maintenance explicable (Kernel Maintenance Observability)

### 4.4 Performance

**Problème :**
- Lenteur sur grandes quantités de données
- Temps de chargement des listes
- Génération de rapports lente

**Impact :** Productivité réduite

**Recommandation pour Miyukini :**
- Optimisation des requêtes
- Pagination efficace
- Génération asynchrone des rapports

---

## 5. Recommandations pour Miyukini

### 5.1 Onboarding Simplifié

**Objectif :** Réduire le temps d'onboarding de 2-4h à 30-60 minutes

**Stratégie :**
1. **Assistant guidé** : Étapes progressives avec validation
2. **Configuration par défaut** : Plan comptable pré-configuré selon pays
3. **Validation automatique** : Vérification des configurations avant activation
4. **Aide contextuelle** : Tooltips et explications à chaque étape

### 5.2 Interface Adaptative

**Objectif :** Adapter l'interface au rôle utilisateur

**Stratégie :**
1. **Vues simplifiées** : Masquage des détails techniques pour utilisateurs non-comptables
2. **Vues expertes** : Accès complet pour comptables
3. **Personnalisation** : Préférences utilisateur pour l'affichage

### 5.3 Workflows Guidés

**Objectif :** Guider l'utilisateur dans les tâches complexes

**Stratégie :**
1. **Wizards contextuels** : Assistants pour tâches complexes (rapprochement, validation)
2. **Suggestions intelligentes** : Propositions basées sur l'historique
3. **Validation progressive** : Vérifications à chaque étape

### 5.4 Feedback Utilisateur

**Objectif :** Améliorer la compréhension des actions

**Stratégie :**
1. **Messages clairs** : Explications des actions effectuées
2. **Prévisualisation** : Aperçu avant validation
3. **Historique** : Traçabilité des actions (audit trail)

---

## 6. Parcours Spécifiques par Persona

### 6.1 Comptable : Journée Type

**Matin :**
1. Consultation des écritures de la veille (10 min)
2. Validation des factures en attente (30 min)
3. Rapprochement bancaire (60 min)

**Après-midi :**
1. Saisie des factures fournisseurs (90 min)
2. Enregistrement des paiements (30 min)
3. Génération des rapports (30 min)

**Total :** ~4 heures de travail comptable

### 6.2 Responsable Financier : Consultation Hebdomadaire

**Tâches :**
1. Consultation du grand livre (15 min)
2. Vérification des soldes (15 min)
3. Validation des écritures importantes (30 min)
4. Consultation des rapports financiers (30 min)

**Total :** ~1h30 par semaine

### 6.3 Utilisateur Standard : Création Factures

**Tâches :**
1. Création de factures clients (5-10 min/facture)
2. Envoi des factures (2 min/facture)

**Fréquence :** Selon besoin (quotidien à hebdomadaire)

---

## 7. Métriques d'Usage Identifiées

### 7.1 Temps Moyen par Tâche

| Tâche | Temps moyen | Temps optimal (objectif) |
|-------|-------------|--------------------------|
| Création facture client | 10 min | 5 min |
| Saisie facture fournisseur | 15 min | 8 min |
| Rapprochement bancaire | 45 min | 20 min |
| Enregistrement paiement | 5 min | 2 min |
| Consultation rapport | 10 min | 5 min |

### 7.2 Fréquence d'Usage

| Tâche | Fréquence | Persona |
|-------|-----------|---------|
| Création factures | Quotidienne | Comptable, Vendeur |
| Rapprochement bancaire | Hebdomadaire/Mensuelle | Comptable |
| Consultation rapports | Hebdomadaire/Mensuelle | Responsable Financier |
| Validation écritures | Quotidienne | Comptable |

---

## 8. Conclusion

L'application Accounting d'Odoo présente des **parcours utilisateur complets mais complexes**, avec :

- **Onboarding long** : 2-4 heures de configuration initiale
- **Interface dense** : Nombreux champs et options visibles
- **Workflows puissants** : Couverture complète des besoins comptables
- **Points de friction** : Complexité, performance, gestion d'erreurs

Pour Miyukini, l'implémentation devra :
1. **Simplifier l'onboarding** : Assistant guidé, configuration par défaut
2. **Adapter l'interface** : Vues selon rôle, masquage des détails techniques
3. **Guider les workflows** : Wizards contextuels, suggestions intelligentes
4. **Améliorer le feedback** : Messages clairs, prévisualisation, traçabilité

**Prochaines étapes :** Voir [Analyse UI/UX](./02_ui_ux/Odoo%20Accounting%20-%20Analyse%20UI%20UX.md) pour les spécifications d'interface détaillées.

---

**Document** : Odoo Accounting — Parcours Utilisateur Détaillés  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : Analyse complète — référence pour UX Miyukini
