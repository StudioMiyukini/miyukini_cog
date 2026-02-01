# Odoo Sales — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application Sales d'Odoo, identifiant les personas, scénarios d'usage, étapes d'onboarding et points de friction pour guider l'implémentation d'un équivalent dans Miyukini.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs
- Parcours d'onboarding
- Scénarios d'usage principaux (création devis, confirmation, facturation)
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles

### 1.1 Commercial (Salesperson)

**Profil :**
- Crée des devis depuis les opportunities CRM
- Gère ses propres devis et commandes
- Suit les confirmations clients
- Génère des factures

**Permissions :**
- Création/modification de devis
- Confirmation de commandes
- Génération de factures (selon permissions)
- Consultation de ses propres devis/commandes

### 1.2 Responsable Commercial (Sales Manager)

**Profil :**
- Supervise les devis de l'équipe
- Valide les commandes importantes
- Analyse les performances
- Configure les pricelists

**Permissions :**
- Accès à tous les devis/commandes de son équipe
- Validation de commandes
- Configuration des pricelists
- Consultation des rapports

### 1.3 Client (Portal User)

**Profil :**
- Consulte ses devis/commandes
- Signe les devis en ligne
- Paiement en ligne
- Suit l'état de ses commandes

**Permissions :**
- Consultation de ses propres devis/commandes
- Signature en ligne
- Paiement en ligne

---

## 2. Parcours d'Onboarding

### 2.1 Première Utilisation (Commercial)

**Étapes :**

1. **Création d'un premier devis**
   - Navigation vers "Ventes" → "Devis"
   - Clic sur "Nouveau"
   - Sélection du client
   - Ajout de lignes produits
   - Sauvegarde

2. **Envoi du devis**
   - Clic sur "Envoyer par email"
   - Sélection du template
   - Envoi

3. **Confirmation de commande**
   - Réception confirmation client
   - Clic sur "Confirmer"
   - Commande confirmée

**Durée estimée :** 10-20 minutes

---

## 3. Scénarios d'Usage Principaux

### 3.1 Scénario : Création Devis depuis Opportunity

**Acteur :** Commercial

**Étapes :**

1. **Depuis CRM Opportunity**
   - Clic sur "Créer un devis"
   - Pré-remplissage : client, équipe, commercial
   - Ouverture formulaire devis

2. **Ajout de lignes**
   - Sélection produits depuis catalogue
   - Quantités, prix (depuis pricelist)
   - Remises si nécessaire

3. **Configuration**
   - Conditions de paiement
   - Date de validité
   - Notes

4. **Envoi**
   - Génération PDF
   - Envoi par email

**Durée estimée :** 5-10 minutes

### 3.2 Scénario : Confirmation Commande avec Paiement

**Acteur :** Client (Portal)

**Étapes :**

1. **Réception email devis**
   - Lien vers portail
   - Consultation devis

2. **Signature et paiement**
   - Signature en ligne (si requis)
   - Paiement en ligne (acompte ou total)
   - Confirmation automatique

3. **Suivi**
   - Consultation commande confirmée
   - Suivi facturation

**Durée estimée :** 3-5 minutes

### 3.3 Scénario : Génération Facture depuis Commande

**Acteur :** Commercial ou Comptable

**Étapes :**

1. **Depuis commande confirmée**
   - Clic sur "Créer une facture"
   - Sélection lignes à facturer
   - Groupement (si plusieurs commandes)

2. **Génération**
   - Création facture automatique
   - Lien bidirectionnel créé
   - Statut facturation mis à jour

3. **Suivi**
   - Consultation factures générées
   - Suivi encaissements

**Durée estimée :** 2-3 minutes

---

## 4. Points de Friction Identifiés

### 4.1 Complexité Pricelist

**Problème :** Configuration des pricelists peut être complexe.

**Recommandations pour Miyukini :**
- Interface simplifiée pour pricelists standards
- Templates de pricelist prédéfinis

### 4.2 Gestion des Taxes

**Problème :** Calcul des taxes peut être complexe selon position fiscale.

**Recommandations pour Miyukini :**
- Calcul automatique transparent
- Explication des taxes appliquées

---

## 5. Recommandations pour Miyukini

### 5.1 Workflow Simplifié

**Actions :**
- Assistant de création de devis guidé
- Pré-remplissage intelligent depuis CRM
- Templates de devis réutilisables

### 5.2 Intégrations Fluides

**Actions :**
- Intégration native avec Miyukini CRM (conversion)
- Intégration native avec MiyuInvoice (facturation)
- Intégration native avec MiyuStore (produits)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
