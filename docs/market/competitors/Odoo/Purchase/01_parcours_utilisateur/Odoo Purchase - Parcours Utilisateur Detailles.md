# Odoo Purchase — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Purchase** d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Code source GitHub Odoo 19.0

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

### 1.1 Acheteur (Buyer)

**Profil :**
- Rôle quotidien : Création et gestion des commandes d'achat
- Responsabilités :
  - Créer des RFQ (Demandes de Devis)
  - Envoyer des RFQ aux fournisseurs
  - Confirmer des commandes d'achat
  - Suivre les réceptions
  - Gérer les factures fournisseur

**Besoins :**
- Interface simple et rapide pour créer des commandes
- Accès au catalogue produits
- Comparaison de prix fournisseurs
- Suivi des commandes en cours
- Gestion des dates de réception

**Permissions :**
- `group_purchase_user` : Utilisateur Purchase standard
- Peut créer/modifier RFQ et commandes
- Peut confirmer commandes (selon règles approbation)

### 1.2 Responsable Achats (Purchase Manager)

**Profil :**
- Rôle stratégique : Validation et approbation des commandes
- Responsabilités :
  - Approuver les commandes nécessitant validation
  - Gérer les fournisseurs
  - Analyser les performances d'achat
  - Configurer les règles d'approbation
  - Gérer les budgets

**Besoins :**
- Tableau de bord avec KPIs
- Vue d'ensemble des commandes en attente d'approbation
- Comparaison de prix entre fournisseurs
- Rapports d'analyse
- Configuration des règles métier

**Permissions :**
- `group_purchase_manager` : Manager Purchase
- Peut approuver toutes les commandes
- Peut configurer les règles d'approbation
- Accès aux rapports et analyses

### 1.3 Comptable (Accountant)

**Profil :**
- Rôle financier : Gestion des factures fournisseur
- Responsabilités :
  - Générer des factures depuis commandes
  - Faire correspondre factures avec commandes (Bill Matching)
  - Valider les factures
  - Gérer les paiements

**Besoins :**
- Lien commande ↔ facture
- Outil de matching factures
- Vue des montants à facturer
- Suivi des paiements

**Permissions :**
- `group_account_invoice` : Gestion factures
- Accès aux factures fournisseur
- Bill Matching

### 1.4 Fournisseur (Vendor / Portal User)

**Profil :**
- Rôle externe : Consultation et reconnaissance des commandes
- Responsabilités :
  - Consulter les commandes reçues
  - Reconnaître les commandes (acknowledge)
  - Mettre à jour les dates de réception prévues

**Besoins :**
- Accès portail simple
- Vue claire des commandes
- Possibilité de mettre à jour les dates
- Reconnaissance facile

**Permissions :**
- Accès portail fournisseur
- Consultation commandes
- Mise à jour dates (si autorisé)
- Reconnaissance commandes

---

## 2. Parcours d'Onboarding

### 2.1 Premier Accès — Acheteur

**Étapes :**
1. **Accès menu Purchase**
   - Menu principal → Purchase → Orders → Requests for Quotation
   - Vue liste vide avec message d'aide

2. **Création première RFQ**
   - Clic "Create" → Formulaire RFQ
   - Sélection fournisseur (`partner_id`)
   - Ajout produits via catalogue ou saisie manuelle
   - Validation → RFQ créée en `draft`

3. **Envoi RFQ**
   - Clic "Send RFQ" → Composeur email
   - Template email pré-rempli
   - Envoi → RFQ passe en `sent`

4. **Confirmation Commande**
   - Réception réponse fournisseur
   - Clic "Confirm Order" → Commande confirmée
   - Si approbation requise → `to approve`
   - Sinon → `purchase`

**Points d'aide :**
- Messages d'aide contextuels dans vues vides
- Tooltips sur champs importants
- Wizards d'aide pour premières actions

### 2.2 Configuration Initiale — Manager

**Étapes :**
1. **Configuration règles d'approbation**
   - Settings → Purchase → Approval
   - Configuration double validation (`po_double_validation`)
   - Définition seuil montant (`po_double_validation_amount`)

2. **Configuration fournisseurs**
   - Ajout fournisseurs (Vendors)
   - Configuration devises fournisseurs
   - Configuration conditions de paiement

3. **Configuration produits**
   - Vérification produits `purchase_ok=True`
   - Configuration sellers (`product.supplierinfo`)
   - Configuration prix fournisseurs

**Points d'aide :**
- Wizards de configuration guidée
- Documentation intégrée
- Exemples de configuration

---

## 3. Scénarios d'Usage Principaux

### 3.1 Scénario 1 : Création et Envoi RFQ

**Acteur :** Acheteur

**Objectif :** Demander un devis à un fournisseur

**Étapes :**
1. Accès menu → Requests for Quotation → Create
2. Sélection fournisseur (`partner_id`)
3. Ajout produits :
   - Option A : Catalogue → Sélection produits → Quantités
   - Option B : Saisie manuelle → Produit → Quantité → Prix
4. Vérification montants (HT, TTC)
5. Ajout conditions générales si nécessaire (`note`)
6. Sauvegarde → RFQ en `draft`
7. Clic "Send RFQ" → Composeur email
8. Vérification email → Envoi
9. RFQ passe en `sent`

**Résultat attendu :**
- RFQ envoyée au fournisseur
- Email reçu par fournisseur
- RFQ visible dans liste "Sent"

**Points de friction possibles :**
- Recherche fournisseur si nombreux
- Sélection produits si catalogue volumineux
- Calcul prix si seller non configuré

### 3.2 Scénario 2 : Confirmation Commande d'Achat

**Acteur :** Acheteur

**Objectif :** Confirmer une commande après réception devis

**Étapes :**
1. Accès RFQ en `sent`
2. Vérification devis fournisseur
3. Ajustement quantités/prix si nécessaire
4. Clic "Confirm Order"
5. Si approbation requise :
   - Commande passe en `to approve`
   - Notification manager
6. Si approbation non requise :
   - Commande passe en `purchase`
   - `date_approve` = maintenant
   - Si Inventory installé → Génération picking réception

**Résultat attendu :**
- Commande confirmée (`purchase`)
- Date approbation enregistrée
- Si Inventory → Réception créée

**Points de friction possibles :**
- Attente approbation si double validation
- Erreurs si produits manquants
- Erreurs si validation analytique échoue

### 3.3 Scénario 3 : Approbation Commande

**Acteur :** Purchase Manager

**Objectif :** Approuver une commande en attente

**Étapes :**
1. Accès menu → Requests for Quotation
2. Filtre "To Approve"
3. Ouverture commande en `to approve`
4. Vérification :
   - Montant total
   - Fournisseur
   - Produits
   - Dates prévues
5. Clic "Approve Order"
6. Commande passe en `purchase`
7. `date_approve` = maintenant

**Résultat attendu :**
- Commande approuvée (`purchase`)
- Notification acheteur
- Si Inventory → Réception créée

**Points de friction possibles :**
- Nombreuses commandes à approuver
- Manque d'informations pour décision
- Pas de comparaison prix automatique

### 3.4 Scénario 4 : Génération Facture Fournisseur

**Acteur :** Comptable

**Objectif :** Créer facture fournisseur depuis commande

**Étapes :**
1. Accès commande en `purchase`
2. Vérification quantités reçues (`qty_received`)
3. Clic "Create Bills" (ou depuis liste avec sélection multiple)
4. Vérification facture générée :
   - Lignes facture depuis lignes commande
   - Montants (HT, TTC)
   - Taxes
5. Ajustements si nécessaire
6. Validation facture (`action_post`)

**Résultat attendu :**
- Facture créée (`account.move` type `in_invoice`)
- Lien bidirectionnel commande ↔ facture
- Statut facturation mis à jour (`invoice_status`)

**Points de friction possibles :**
- Quantités reçues non synchronisées
- Taxes incorrectes
- Devises différentes

### 3.5 Scénario 5 : Bill Matching

**Acteur :** Comptable

**Objectif :** Faire correspondre facture reçue avec commande

**Étapes :**
1. Accès commande en `purchase`
2. Clic "Bill Matching"
3. Vue liste factures fournisseur non matchées
4. Sélection facture correspondante
5. Matching automatique ou manuel :
   - Par référence (`partner_ref`)
   - Par montant
   - Par lignes produits
6. Validation matching
7. Lien facture ↔ commande créé

**Résultat attendu :**
- Facture liée à commande
- Lignes facture liées à lignes commande
- Statut facturation mis à jour

**Points de friction possibles :**
- Factures multiples pour une commande
- Références différentes
- Montants différents (remises, frais)

### 3.6 Scénario 6 : Réception Produits (si Inventory)

**Acteur :** Acheteur / Réceptionniste

**Objectif :** Enregistrer réception produits

**Étapes :**
1. Accès commande en `purchase`
2. Vérification picking réception créé (si Inventory)
3. Ouverture picking
4. Validation quantités reçues
5. Confirmation picking → `done`
6. Quantités reçues mises à jour sur lignes (`qty_received`)

**Résultat attendu :**
- Quantités reçues enregistrées
- Stock mis à jour (si Inventory)
- Commande prête pour facturation

**Points de friction possibles :**
- Quantités partielles
- Produits manquants
- Qualité non conforme

### 3.7 Scénario 7 : Reconnaissance Fournisseur (Portal)

**Acteur :** Fournisseur

**Objectif :** Reconnaître réception commande

**Étapes :**
1. Accès portail fournisseur
2. Connexion avec identifiants
3. Menu "Purchase Orders"
4. Consultation commandes reçues
5. Ouverture commande
6. Clic "Acknowledge" → Reconnaissance
7. Optionnel : Mise à jour dates prévues (`date_planned`)

**Résultat attendu :**
- Commande reconnue (`acknowledged=True`)
- Dates prévues mises à jour si modifiées
- Notification acheteur

**Points de friction possibles :**
- Accès portail complexe
- Interface peu intuitive
- Pas de possibilité de négocier

---

## 4. Points de Friction Identifiés

### 4.1 Friction 1 : Recherche Fournisseur

**Problème :**
- Si nombreux fournisseurs, recherche peut être lente
- Pas de filtres avancés par défaut

**Impact :** Perte de temps lors création RFQ

**Recommandation Miyukini :**
- Recherche intelligente avec autocomplétion
- Filtres par catégorie, pays, devise
- Fournisseurs fréquents en priorité

### 4.2 Friction 2 : Sélection Produits

**Problème :**
- Catalogue volumineux → sélection difficile
- Pas de vue catalogue optimisée pour achats

**Impact :** Temps de création RFQ élevé

**Recommandation Miyukini :**
- Vue catalogue spécialisée achats
- Filtres par fournisseur, catégorie
- Recherche rapide produits
- Historique produits achetés

### 4.3 Friction 3 : Calcul Prix

**Problème :**
- Si seller non configuré, prix manuel
- Pas de suggestion prix automatique

**Impact :** Erreurs de saisie, temps perdu

**Recommandation Miyukini :**
- Calcul prix automatique depuis seller
- Suggestion prix si seller existe
- Avertissement si prix anormal

### 4.4 Friction 4 : Attente Approbation

**Problème :**
- Double validation → attente manager
- Pas de notification automatique
- Pas de vue centralisée approbations

**Impact :** Délais de traitement, manque de visibilité

**Recommandation Miyukini :**
- Notifications automatiques approbation
- Vue centralisée "À approuver"
- Workflow guidé approbation
- Délégation temporaire

### 4.5 Friction 5 : Synchronisation Quantités

**Problème :**
- Quantités reçues vs commandées non synchronisées
- Facturation avant réception complète

**Impact :** Erreurs facturation, gestion complexe

**Recommandation Miyukini :**
- Synchronisation automatique Inventory ↔ Purchase
- Avertissement si facturation avant réception
- Gestion quantités partielles

### 4.6 Friction 6 : Bill Matching

**Problème :**
- Matching manuel fastidieux
- Pas de matching automatique intelligent

**Impact :** Temps perdu, erreurs possibles

**Recommandation Miyukini :**
- Matching automatique par référence/montant
- Suggestion matching avec score de confiance
- Validation rapide matching

### 4.7 Friction 7 : Portail Fournisseur

**Problème :**
- Interface peu intuitive
- Fonctionnalités limitées
- Pas de négociation

**Impact :** Adoption faible, communication difficile

**Recommandation Miyukini :**
- Interface moderne et intuitive
- Fonctionnalités étendues (négociation, contre-propositions)
- Notifications temps réel

---

## 5. Recommandations pour Miyukini

### 5.1 UX Améliorée

**Création RFQ :**
- Assistant guidé étape par étape
- Recherche intelligente fournisseurs/produits
- Calcul prix automatique
- Prévisualisation avant envoi

**Gestion Approbations :**
- Vue centralisée "À approuver"
- Notifications automatiques
- Workflow guidé
- Comparaison prix automatique

**Bill Matching :**
- Matching automatique intelligent
- Suggestion avec score confiance
- Validation rapide
- Historique matching

### 5.2 Intégrations Fluides

**Inventory :**
- Synchronisation automatique quantités
- Génération réceptions automatique
- Suivi réceptions en temps réel

**Accounting :**
- Génération factures fluide
- Lien bidirectionnel automatique
- Synchronisation montants

**Contacts :**
- Gestion fournisseurs centralisée
- Historique commandes par fournisseur
- Performance fournisseurs

### 5.3 Portail Fournisseur Moderne

**Interface :**
- Design moderne et responsive
- Navigation intuitive
- Recherche rapide

**Fonctionnalités :**
- Reconnaissance commandes
- Mise à jour dates prévues
- Consultation historique
- Négociation (si activé)
- Notifications temps réel

### 5.4 Analytics et Reporting

**Tableaux de bord :**
- KPIs achats (montants, commandes, fournisseurs)
- Vue d'ensemble approbations
- Suivi réceptions
- Analyse performance fournisseurs

**Rapports :**
- Comparaison prix fournisseurs
- Historique achats
- Analyse tendances
- Rapports personnalisables

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
