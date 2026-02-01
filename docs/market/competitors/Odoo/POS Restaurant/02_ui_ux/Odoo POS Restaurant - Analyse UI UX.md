# Odoo POS Restaurant — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** du module POS Restaurant d'Odoo : vues principales (plan des tables, registre, ordres), boutons et actions, patterns de navigation et recommandations pour Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 — Restaurant features, écrans POS.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales (Floor plan, Registre POS, Liste des ordres)
- Boutons et actions (Tables, Register, Orders, Set Table, Set Tab, Order, Transfer/Merge, Course, Split, Payment, Tip)
- Patterns de navigation (trois boutons principaux, menu Actions, Edit Plan)
- Responsive et usage tactile
- Recommandations pour Miyukini

---

## 1. Vues Principales

### 1.1 Vue Plan des tables (Floor plan)

**Rôle :** Vue d’ensemble des sols et tables ; statut en temps réel (disponible, occupée, réservée, en retard).

**Éléments :**
- **Plan graphique** : sol(s) avec tables positionnées (forme, couleur, numéro, nombre de places).
- **Indicateurs visuels** : couleur ou icône selon état (libre, occupée, réservée à une heure, retard).
- **Boutons de navigation** : par étage (ex. Main Floor, Patio).
- **New Order** : ouvre le registre sans table (ordre direct).
- **Table Selector** : champ numéro de table + bouton Jump pour accès direct.
- **Menu (hamburger)** : Edit Plan, Get QR Codes (si QR menu activé), **Booking** (si activé).

**Comportement :**
- Clic sur une table → ouverture du registre avec cette table sélectionnée (occupation confirmée).
- Édition du plan (Edit Plan) : ajout/suppression de sols et tables, forme, couleur, sièges, fond.

**Recommandations Miyukini :**
- Plan lisible sur écran tactile (taille des zones de clic, contraste).
- Légende claire des états (disponible / occupée / réservée).
- Navigation entre étages sans perdre le contexte (ordre en cours).

### 1.2 Vue Registre (Register)

**Rôle :** Saisie des lignes, gestion table/tab, cours, validation commande, paiement.

**Structure type :**
- **Zone panier** : lignes, quantités, prix, sous-total ; cours (Course 1, 2, …) si utilisés.
- **Zone produits** : grille ou liste de produits par catégorie.
- **Actions contexte** : Set Table, Set Tab, Order, Course, Actions (Transfer/Merge, Cancel Order, Split, etc.).
- **Payment** : ouverture écran paiement (méthodes, client, facture, Tip).
- **Presets** : Dine In / Takeout / Delivery (boutons ou sélecteur).

**Boutons clés :**
- **Set Table** : assigner l’ordre à une table (saisie numéro → Assign).
- **Set Tab** : nommer l’ordre (tab) sans table → Apply.
- **Order** : valider la commande (envoi cuisine si configuré) ; retour possible au plan si écran par défaut = Tables.
- **Course** : ajout d’un cours ; envoi séquentiel (Order = cours 1 ; Fire Course 2, etc.).
- **Release table** (panier vide) : libérer la table.
- **Actions** : Transfer/Merge, Cancel Order, Split, Transfer course.

**Recommandations Miyukini :**
- Hiérarchie visuelle claire : panier toujours visible, actions principales (Order, Payment) mises en avant.
- Feedback immédiat après Order (envoi cuisine, message de confirmation).
- Gestion des cours visible dans le panier (onglets ou blocs par cours).

### 1.3 Vue Ordres (Orders)

**Rôle :** Liste ou cartes des ordres ouverts ; accès rapide pour reprendre une commande, envoyer un cours, payer.

**Éléments :**
- Liste/cartes par ordre (table ou tab, montant, état).
- Clic → ouverture du registre sur cet ordre.
- Permet de retrouver un ordre sans repasser par le plan.

**Recommandations Miyukini :**
- Filtres rapides (par table, par serveur, par état).
- Tri par heure ou table pour faciliter le service.

---

## 2. Composants et Widgets

### 2.1 Plan des tables

- **Tables** : formes (carré, rond), couleurs, numéros, nombre de places.
- **États** : affichage par couleur ou icône (disponible / occupée / réservée / retard).
- **Réservations** : bulles ou badges sur les tables (heure, nom).

### 2.2 Panier et cours

- **Lignes** : produit, quantité, prix, note optionnelle.
- **Cours** : regroupement par « Course 1 », « Course 2 » ; actions Fire Course 2, Transfer course.
- **Presets** : boutons ou dropdown Dine In / Takeout / Delivery.

### 2.3 Paiement et split

- **Écran paiement** : méthodes, montant, client, case Facture, bouton Tip.
- **Split** : sélection de lignes → actions Payment / Split Order / Transfer ; bouton Continue pour enchaîner les parts.

### 2.4 Edit Plan (frontend)

- **Sols** : Add Floor, Rename, Clone, Delete, fond (couleur/image).
- **Tables** : Add Table ; par table : Seats, Square/Round, Color, Rename, Clone, Delete.
- **Save** global pour persister les changements.

---

## 3. Patterns de Navigation

- **Trois entrées principales** : Tables (plan) | Register (registre) | Orders (liste ordres). Permet de basculer sans quitter la session POS.
- **Écran de démarrage** : configurable (Tables ou Register) dans les paramètres.
- **Depuis le plan** : clic table → registre ; New Order → registre sans table.
- **Après Order** : retour automatique au plan si écran par défaut = Tables.
- **Menu Actions** : regroupement Transfer/Merge, Cancel Order, Split, Transfer course pour ne pas surcharger la barre principale.
- **Edit Plan** : accès via menu ; sauvegarde explicite pour éviter les modifications accidentelles.

---

## 4. Responsive et Tactile

- POS souvent utilisé sur écran tactile (borne ou tablette) : zones de clic suffisantes, boutons pas trop petits.
- Plan des tables : zoom/pinch si beaucoup de tables ; scroll si plusieurs étages.
- Clavier virtuel pour Set Tab (nom), numéro de table, montant pourboire.
- Feedback visuel (couleur, court message) après chaque action importante (Order, Payment, Release table).

---

## 5. Accessibilité et Erreurs

- Messages clairs en cas d’erreur (table déjà occupée, champs obligatoires preset, imprimante indisponible).
- Confirmations pour actions destructives (Cancel Order, Delete table/floor).
- Indication du contexte actuel (table/tab courante, ordre en cours) pour éviter les confusions.

---

## 6. Synthèse pour Miyukini

- **Floor plan** : composant réutilisable « plan de tables » avec états et actions (sélection, libération, édition) ; accessible depuis un Opérateur d’interface Restaurant.
- **Registre** : réutilisation du POS générique avec extensions (Set Table, Set Tab, Course, Split) et raccourcis visibles.
- **Navigation** : modèle à 3 vues (Tables | Register | Orders) avec écran de démarrage configurable.
- **Actions groupées** : menu Actions pour Transfer/Merge, Cancel, Split, Transfer course afin de garder une barre principale lisible.
- **Feedback** : notifications courtes après Order, Payment, Release table, et en cas d’erreur (validation, imprimante).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
