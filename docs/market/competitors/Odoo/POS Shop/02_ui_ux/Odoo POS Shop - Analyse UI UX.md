# Odoo POS Shop — Analyse UI/UX Complète

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application Point of Sale (POS) Shop d'Odoo, basée sur la documentation officielle et les conventions du module.

**Source d'analyse :** Documentation Odoo 18/19, module `point_of_sale`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Interface de session POS (écran de vente, panier, paiement)
- Tableau de bord POS (sessions, points de vente)
- Écrans de contrôle (ouverture, clôture)
- Patterns de navigation et raccourcis
- Compatibilité multi-appareils (desktop, tablette, mobile)
- Recommandations pour Miyukini

---

## 1. Vues et Écrans Principaux

### 1.1 Tableau de Bord POS (POS Dashboard)

**Rôle :** Point d’entrée : choix du point de vente et gestion des sessions.

**Caractéristiques :**
- Cartes par point de vente (pos.config)
- Sur chaque carte : nom du POS, statut (session ouverte ou non)
- Actions : ouvrir l’interface POS (clic sur la carte), ou menu (⋮) : Sessions, Orders, etc.
- Bouton "New Session" pour ouvrir une nouvelle session sur le POS sélectionné

**Navigation :**
- "Sessions" : liste des sessions (ouvertes et passées), colonnes "Opened By", "Orders", totaux
- "Orders" : liste de toutes les commandes (tous points de vente ou filtré)
- Accès "Backend" depuis le menu sans fermer la session

**Recommandations pour Miyukini :**
- Tableau de bord clair avec statut en temps réel (session ouverte / fermée)
- Accès rapide aux rapports (sessions, commandes) et à la configuration

### 1.2 Écran de Contrôle d’Ouverture (Opening Control)

**Rôle :** Saisie du fonds de caisse avant d’ouvrir la session.

**Caractéristiques :**
- Popup ou écran dédié après "New Session"
- Champ montant (ouverture de caisse)
- Boutons : "Open Session" (valider), éventuellement "Cancel"
- Message d’information sur le fait qu’une seule session peut être ouverte par navigateur

**Recommandations pour Miyukini :**
- Formulaire minimal (un montant, un bouton)
- Possibilité d’ouvrir à 0
- Rappel des règles (une session ouverte à la fois)

### 1.3 Interface de Vente (POS Session Interface)

**Rôle :** Écran principal de vente : catalogue, panier, paiement.

**Structure type :**

#### Zone gauche : Produits (Product List)
- Grille ou liste de produits (image, nom, prix)
- Recherche (texte ou code-barres)
- "Search more" si produits limités en chargement
- Catégories / filtres possibles selon config
- Clic sur un produit = ajout d’une ligne au panier (qty 1, incrémentable)

#### Zone centrale / droite : Panier (Order / Cart)
- Lignes : produit, quantité, prix unitaire, remise, sous-total
- Sur une ligne : actions rapides (Qty, % Disc, Price, Customer Note)
- Affichage des notes client sur les lignes
- Total HT/TTC, taxes
- Boutons d’action : "Payment", "New Order", "Customer", "Actions" (ex. Refund)

#### Zone basse ou latérale : Numpad / Actions
- Clavier numérique (quantités, montants)
- Touches +/- pour quantité
- Séparateur décimal ("," et "." acceptés)
- Boutons : Qty, % Disc, Price, Customer Note, etc.

**Recommandations pour Miyukini :**
- Layout adaptatif : grille produits + panier visible en même temps (desktop/tablette)
- Panier toujours visible avec totaux mis à jour en temps réel
- Zones tactiles larges pour usage sur écran tactile

### 1.4 Écran de Paiement (Payment Screen)

**Rôle :** Saisie des paiements pour la commande en cours.

**Caractéristiques :**
- Liste des moyens de paiement (Cash, Card, etc.) configurés pour le POS
- Clic sur une méthode → saisie du montant (numpad)
- "Validate" pour enregistrer le paiement
- Si montant saisi sans choisir de méthode : "Cash" par défaut
- Affichage du total à payer, du déjà payé, du reste à payer / à rendre
- Possibilité de répartir sur plusieurs moyens (espèces + carte)
- Bouton "Customer" accessible depuis cet écran
- Après paiement total : validation de la commande, impression ticket, retour au panier vide (New Order)

**Recommandations pour Miyukini :**
- Affichage clair : "Total", "Paid", "To pay" / "Change"
- Feedback immédiat à chaque paiement partiel
- Gestion explicite du "à rendre" en espèces

### 1.5 Écran de Clôture (Closing Control)

**Rôle :** Contrôle des montants avant de clôturer la session.

**Caractéristiques :**
- Récapitulatif : nombre de commandes, total de la session
- Tableau par moyen de paiement : "Expected" (attendu), "Counted" (compté)
- Pour les espèces : bouton calculatrice pour ouvrir la popup de comptage
- Popup de comptage : saisie du nombre de billets/pièces par type (ou montant total), total calculé, "Confirm" / "Discard"
- Le montant "Counted" est reporté dans la colonne
- Si écart : écran "Payments Difference" avec "Ok" pour clôturer malgré tout (si autorisé)
- Boutons : "Close Session" (valider la clôture), "Discard" (annuler et rester en session)

**Recommandations pour Miyukini :**
- Vue synthétique par méthode de paiement
- Outil de comptage espèces intégré (billets/pièces ou total)
- Traçabilité des écarts (commentaire, validation responsable)

### 1.6 Menu Session (Dropdown)

**Rôle :** Accès aux actions sans quitter l’écran de vente.

**Contenu type :**
- "Backend" : retour à l’interface Odoo principale (session reste ouverte)
- "Cash In/Out" : entrée ou sortie de caisse
- "Close Session" : lance le contrôle de clôture
- Autres selon modules (ex. Orders, Reports)

**Recommandations pour Miyukini :**
- Menu compact mais visible (icône en haut à droite)
- Actions critiques (Close Session, Cash In/Out) clairement libellées

---

## 2. Composants d’Interface

### 2.1 Grille Produits

- Tuiles ou lignes : image, nom, prix (TTC ou HT selon config)
- Code-barres pour recherche rapide (scan)
- Catégories en onglets ou sidebar
- Chargement progressif ("Search more")

### 2.2 Panier (Order Lines)

- Liste des lignes avec produit, qty, prix, remise, sous-total
- Clic sur une ligne : affichage des actions (Qty, % Disc, Price, Note)
- Notes client affichées sous la ligne ou en tooltip
- Suppression de ligne (icône poubelle ou action)

### 2.3 Numpad

- Chiffres 0–9, séparateur décimal, +/- pour quantité
- Utilisé pour : quantité, remise %, montant prix, montant paiement, comptage caisse
- Contexte affiché (ex. "Enter quantity", "Enter amount")

### 2.4 Choix Client (Customer)

- Popup ou sidebar : recherche par nom, email, etc.
- Création rapide : "Create" → formulaire simplifié (nom, email, téléphone, etc.)
- Sélection → le client est attaché à la commande (pricelist, facturation, fidélité)

### 2.5 Notes Client (Customer Note)

- Popup après clic "Customer Note" sur une ligne
- Zone de texte libre
- Affichée sur le ticket et sur la facture

### 2.6 Remboursement (Refund)

- Action "Refund" → liste des commandes (filtres : date, numéro, client)
- Sélection de la commande → liste des lignes avec quantités remboursables
- Saisie des quantités à rembourser (numpad)
- Validation → panier avec lignes en négatif → écran paiement (méthode remboursement)

---

## 3. Patterns de Navigation

### 3.1 Flux Principal

```
Dashboard → (New Session) → Opening Control → Session UI
    → [Order → Payment → Validate] × N
    → Menu → Close Session → Closing Control → Close → Dashboard
```

### 3.2 Raccourcis et Usages

- **Décimales :** "," et "." acceptés au clavier
- **Cash par défaut :** Si on saisit un montant sans choisir de méthode, Cash est sélectionné
- **New Order :** Après validation du paiement, passage au client suivant sans fermer la session

### 3.3 Multi-Employés

- Connexion employé (PIN ou login) sur la même session
- La commande peut être associée à l’employé connecté pour rapports et commissions

---

## 4. Responsive et Multi-Appareils

### 4.1 Navigateurs et Appareils

- Odoo POS est une webapp : Chrome, Firefox, Safari (Windows, macOS, Linux, Android, iOS)
- Une session ouverte par navigateur (pas plusieurs onglets pour la même session)
- Interface adaptée tactile (boutons suffisamment grands, peu de hover)

### 4.2 Recommandations pour Miyukini

- Design responsive : desktop (grande grille produits + panier), tablette (même logique, tailles adaptées), mobile (priorité panier + recherche produit)
- Mode hors ligne pris en charge (cache catalogue, file de synchronisation)
- Accessibilité : contraste, focus clavier, labels pour lecteurs d’écran

---

## 5. Accessibilité et Feedback

### 5.1 Feedback Utilisateur

- Mise à jour immédiate des totaux à chaque modification de ligne ou paiement
- Message de confirmation après paiement validé et après clôture de session
- En cas d’erreur (ex. paiement insuffisant) : message clair sans bloquer toute l’interface

### 5.2 Confirmations

- Cash In/Out : confirmation du montant et du motif
- Close Session avec écart : avertissement + confirmation
- Refund : rappel de la commande et des quantités remboursées

**Recommandations pour Miyukini :**
- Feedback visuel (et optionnel sonore) sur ajout produit et paiement
- Messages d’erreur explicites (ex. "Total remaining: 5.00 €")
- Indicateur de connexion et de file de synchronisation en mode offline

---

## 6. Synthèse des Recommandations pour Miyukini

### 6.1 Interface

- Écran de vente en deux zones (produits | panier) avec numpad dédié
- Tableau de bord POS avec statut des sessions et accès aux rapports
- Contrôles d’ouverture et de clôture simples et guidés

### 6.2 Expérience

- Workflow linéaire : ouverture → ventes → clôture, avec rappels en fin de journée
- Recherche produit performante (full-text / code-barres) sans tout charger
- Gestion des retours et des écarts de caisse tracée et autorisée selon le rôle

### 6.3 Technique

- Webapp compatible desktop, tablette, mobile
- Support offline et resynchronisation
- Raccourcis clavier et tactiles pour les actions répétitives (quantité, remise, paiement)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
