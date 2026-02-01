# Odoo POS Shop — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application Point of Sale (POS) Shop d'Odoo, identifiant les personas, scénarios d'usage, étapes d'onboarding et points de friction pour guider l'implémentation d'un équivalent dans Miyukini.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles utilisateurs (caissier, responsable, client)
- Parcours d'onboarding
- Scénarios d'usage principaux (ouverture session, vente, paiement, clôture, retours)
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles

### 1.1 Caissier / Vendeur (Cashier)

**Profil :**
- Ouvre et clôture sa session de caisse
- Enregistre les ventes (produits, quantités, remises)
- Encaisse (espèces, carte, plusieurs moyens)
- Imprime les tickets
- Gère les retours et remboursements
- Effectue des entrées/sorties de caisse (Cash In/Out)

**Permissions :**
- Ouvrir/clôturer une session sur son POS
- Créer et valider des commandes
- Enregistrer les paiements
- Créer des clients depuis le POS (selon paramétrage)
- Traiter les remboursements

### 1.2 Responsable de Magasin / Manager

**Profil :**
- Configure les points de vente (pricelist, paiements, imprimantes)
- Consulte les sessions et rapports (CA, écarts, commandes)
- Gère les écarts de caisse (autorisation de clôture malgré écart)
- Supervise les caissiers (multi-employee)

**Permissions :**
- Accès configuration POS
- Consultation de toutes les sessions et commandes
- Clôture de session avec écart (si paramétré)
- Rapports et analytics

### 1.3 Client (Customer)

**Profil :**
- Acheteur en magasin
- Peut être identifié (pour fidélité, pricelist, facture)
- Bénéficie du ticket et éventuellement de la facture

**Interactions :**
- Pas d'interface directe (sauf self-ordering si activé)
- Identification par le caissier (recherche client)
- Reçoit ticket et/ou facture

---

## 2. Parcours d'Onboarding

### 2.1 Première Utilisation (Caissier)

**Étapes :**

1. **Accès au POS**
   - Menu "Point of Sale" → Sélection du point de vente (carte)
   - Clic sur la carte pour ouvrir l'interface POS

2. **Ouverture de session**
   - Clic "New Session"
   - Saisie du fonds de caisse (ou 0)
   - Clic "Open Session"

3. **Première vente**
   - Clic sur des produits pour les ajouter au panier
   - Ajustement des quantités (Qty) si besoin
   - Option : remise (% Disc) ou modification du prix (Price)
   - Clic "Payment" → choix du moyen de paiement → saisie du montant → "Validate"
   - Clic "New Order" pour le client suivant

4. **Clôture de session**
   - Menu (icône) → "Close Session"
   - Comptage du cash (calculatrice) → saisie du montant compté
   - Vérification des totaux par moyen de paiement
   - "Close Session" pour valider

**Durée estimée :** 15–30 minutes (avec accompagnement)

---

## 3. Scénarios d'Usage Principaux

### 3.1 Scénario : Vente Simple (Sans Client)

**Acteur :** Caissier

**Étapes :**
1. Session ouverte
2. Clic sur les produits (ajout au panier)
3. Ajuster quantité si besoin (Qty + clavier)
4. Clic "Payment"
5. Choisir "Cash" (ou autre), saisir le montant reçu, "Validate"
6. Ticket imprimé (si imprimante configurée)
7. "New Order"

**Durée estimée :** 1–2 minutes par client

### 3.2 Scénario : Vente avec Client et Facture

**Acteur :** Caissier

**Étapes :**
1. Clic "Customer" → recherche ou création du client → sélection
2. Ajout des produits au panier
3. Paiement comme ci-dessus
4. Avant ou après paiement : cocher "To invoice" ou action "Create Invoice"
5. Génération de la facture, impression éventuelle
6. "New Order"

**Durée estimée :** 2–4 minutes

### 3.3 Scénario : Remise et Prix Modifié

**Acteur :** Caissier

**Étapes :**
1. Après ajout d’un produit, clic sur la ligne
2. "% Disc" → saisie du pourcentage de remise
3. Ou "Price" → saisie du nouveau prix unitaire
4. Validation ; les totaux se mettent à jour
5. Poursuite du paiement

**Durée estimée :** < 1 minute

### 3.4 Scénario : Remboursement (Retour)

**Acteur :** Caissier

**Étapes :**
1. Menu "Actions" → "Refund"
2. Recherche et sélection de la commande d’origine
3. Sélection des lignes (produits) à rembourser
4. Saisie des quantités à rembourser (keypad)
5. Clic "Refund"
6. Écran paiement : choix de la méthode de remboursement, "Validate"
7. Impression du ticket d’avoir si besoin
8. "New Order"

**Durée estimée :** 2–5 minutes

### 3.5 Scénario : Cash In / Cash Out

**Acteur :** Caissier ou responsable

**Étapes :**
1. Menu (icône) → "Cash In/Out"
2. Choix "Cash In" ou "Cash Out"
3. Saisie du montant et du motif
4. "Confirm"
5. La caisse est mise à jour sans créer de vente

**Durée estimée :** < 1 minute

### 3.6 Scénario : Clôture de Session avec Contrôle

**Acteur :** Caissier

**Étapes :**
1. Menu → "Close Session"
2. Consultation du récapitulatif (nombre de commandes, totaux par moyen de paiement)
3. Clic sur l’icône calculatrice pour "Counted" (espèces)
4. Saisie des billets/pièces dans la popup → "Confirm" ou "Discard"
5. Vérification de la colonne "Counted" vs "Expected"
6. Si écart : message "Payments Difference" → "Ok" pour clôturer malgré tout (si droits)
7. "Close Session" → retour au tableau de bord POS

**Durée estimée :** 5–10 minutes

---

## 4. Points de Friction Identifiés

### 4.1 Chargement des Produits

**Problème :** Seul un nombre limité de produits est chargé pour des raisons de performance ; le produit recherché peut être absent.

**Mitigation Odoo :** Bouton "Search more" pour charger davantage de produits.

**Recommandations pour Miyukini :**
- Recherche full-text ou par code-barres sans tout charger
- Pagination ou chargement progressif du catalogue

### 4.2 Saisie Décimale (Virgule / Point)

**Problème :** Selon le clavier, la décimale peut être "," ou ".".

**Mitigation Odoo :** Les deux sont acceptés.

**Recommandations pour Miyukini :** Accepter les deux séparateurs décimaux selon locale.

### 4.3 Écart de Caisse à la Clôture

**Problème :** Un écart (manquant ou surplus) peut bloquer ou inquiéter le caissier.

**Recommandations pour Miyukini :**
- Règles claires (autorisation de clôture avec écart selon rôle)
- Commentaire obligatoire en cas d’écart pour traçabilité
- Rapport d’écarts pour le responsable

### 4.4 Mode Hors Ligne

**Problème :** En cas de coupure réseau, le POS doit rester utilisable puis se resynchroniser.

**Recommandations pour Miyukini :**
- Stratégie offline-first (stock et prix en cache, file de commandes à envoyer)
- Indicateur de connexion et de file en attente

---

## 5. Recommandations pour Miyukini

### 5.1 Workflow Guidé

- Assistant de première ouverture de session (fond de caisse, vérification imprimante)
- Rappels en fin de journée pour clôturer la session
- Checklist de clôture (comptage, écarts, commentaires)

### 5.2 Intégrations Fluides

- Intégration native MiyuStore (produits, prix, code-barres)
- Intégration MiyuInvoice (facturation depuis le ticket)
- Intégration MiyuContacts (client rapide, fidélité si applicable)
- Trésorerie / paiements (MiyuTreasury ou MiyuBilling) pour rapprochement caisse

### 5.3 Expérience Caissier

- Interface tactile et clavier numérique optimisés
- Raccourcis (quantité, remise, paiement)
- Feedback immédiat (son, visuel) sur ajout produit et paiement validé
- Gestion des files (plusieurs paniers en attente si besoin)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
