# Odoo Rental — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Rental** d'Odoo : vues, formulaires, widgets et patterns de navigation pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation et fonctionnalités Odoo Rental (14.0–18.0)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales (commandes, lignes, produits, configuration)
- Formulaires et champs spécifiques location
- Widgets (dates, tarification, statuts)
- Patterns de navigation et actions
- Documents imprimables (reçus)
- Intégration Sign (signature)

**Hors scope :**
- Implémentation technique détaillée
- Logique métier (document dédié)

---

## 1. Vues Principales

### 1.1 Commandes de location (sale.order)

**Contexte :** Les commandes de vente avec lignes de location réutilisent les vues Sales, enrichies par Rental.

**Vue Liste (List View) :**
- Colonnes classiques vente : partenaire, date, montant, statut
- Colonnes additionnelles possibles : date enlèvement, date retour, statut location (draft, confirmed, pickup, return, invoiced)
- Filtres : « Mes commandes », « À enlever », « À retourner », « Locations en cours »
- Tri par date de début ou de fin de location

**Vue Formulaire (Form View) :**
- Bloc **Rental** ou onglet dédié :
  - Dates d’enlèvement et de retour (globales ou par ligne)
  - Statut location (calculé ou sélection)
  - Boutons : **Sign Documents**, **Print ‣ Pickup and Return Receipt**
- Lignes de commande : pour chaque ligne location, affichage des dates, durée, prix calculé, pénalités éventuelles
- Chatter (messages, activités) pour suivi client et rappels

**Actions principales :**
- **Confirm** : confirmer la commande (génère livraison/réception et tâches)
- **Sign Documents** : ouvrir le wizard Sign (modèle Rental Agreement, sélection client)
- **Print ‣ Pickup and Return Receipt** : génération PDF reçu enlèvement/retour

### 1.2 Lignes de commande (sale.order.line)

**Dans le formulaire commande :**
- Colonnes / champs en ligne : produit, dates début/fin, durée, prix unitaire, pénalités (si retard)
- Calcul automatique du prix selon la grille location (option la moins chère)
- Saisie des dates : widgets date/heure pour **Start Date** et **End Date**
- Indication visuelle si créneau indisponible ou en conflit (Security Time, autre location)

**Widgets typiques :**
- **Date range** ou **datetime** pour période de location
- **Monetary** pour prix et pénalités
- **Badge** ou **Label** pour statut de la ligne (planned, picked up, returned, invoiced)

### 1.3 Produits (product.template / product.product)

**Menu :** Rental ‣ Products (ou Sales ‣ Products avec filtre « Louables »)

**Vue Liste :**
- Colonnes : nom, catégorie, « Can be Rented », prix location (résumé), stock Rental In / Rental Out
- Filtre : « Can be Rented » = true

**Vue Formulaire — Onglet Rental :**
- **Can be Rented** : case à cocher
- **Rental Pricing** : tableau « Add a price »
  - Colonnes : Unit of time (Hours, Days, Weeks, Months), Duration (nombre), Price (monétaire)
  - Plusieurs lignes pour décotes longue durée
- **Reservations** :
  - **Extra Hour** : montant
  - **Extra Day** : montant
  - **Security Time** : nombre d’heures (indisponibilité entre deux locations)
- Aide contextuelle sur la règle de calcul (une ligne, option la moins chère)

### 1.4 Configuration (Settings)

**Menu :** Rental ‣ Configuration ‣ Settings

**Contenu :**
- **Digital Documents** : activer/désactiver
- **Default Rental Agreement** : liste déroulante des modèles Sign (ou lien « Upload Template » vers Sign)
- Bouton **Save** pour enregistrer

**UX :**
- Page paramètres standard Odoo (sections, toggles, liens vers Sign)
- Message si Sign non installé lors de l’activation de Digital Documents (proposition d’installation)

---

## 2. Widgets et Composants Spécifiques

### 2.1 Dates de location

- **Start Date** / **End Date** : champs datetime pour planifier la période
- **Duration** : calculé (affiché en lecture seule) ou éditable selon implémentation
- **Unité** : heure, jour, semaine, mois (cohérent avec la grille produit)
- Contrôles de cohérence : fin > début ; alerte si créneau indisponible

### 2.2 Tarification et pénalités

- **Prix calculé** : affiché sur la ligne (monétaire), avec possibilité d’afficher la règle utilisée (ex. « 3 × 3 days »)
- **Extra Hour / Extra Day** : montants en configuration produit ; sur la facture ou le reçu, montant calculé selon retard
- **Security Time** : champ numérique (heures) en configuration produit

### 2.3 Statuts location

- **Commande** : Draft, Confirmed, Pickup (partiel/total), Return (partiel/total), Invoiced
- **Ligne** : Planifié, Enlevé, Retourné, Facturé
- Représentation : badges colorés ou libellés courts dans les vues liste et formulaire

### 2.4 Stock (Rental In / Rental Out)

- Affichage possible dans la fiche produit ou dans un rapport : quantités **Rental In** (disponibles à la location) et **Rental Out** (actuellement louées)
- Tableau de bord ou vue dédiée : évolution des quantités louées dans le temps

---

## 3. Documents et Impressions

### 3.1 Reçu d’enlèvement et de retour (Pickup and Return Receipt)

**Déclenchement :** Depuis la commande, **Print ‣ Pickup and Return Receipt**

**Contenu typique (PDF) :**
- En-tête : société, client, numéro de commande
- Tableau des articles : désignation, dates prévues enlèvement/retour, statut (picked up / to return / returned)
- Rappel des coûts de retard (Extra Hour / Extra Day)
- Zone pour signature client (optionnel) ou simple reçu informatif

**Usage :** Remis au client à l’enlèvement et/ou au retour pour traçabilité.

### 3.2 Contrat de location (Sign)

**Déclenchement :** **Sign Documents** sur la commande → choix du modèle (ex. Rental Agreement) → **Sign Documents** → sélection du client → **Sign Now**

**Parcours Sign :**
- Envoi du document au client (email)
- Client signe dans l’interface Sign
- **Validate & Send Completed Document** côté back-office
- Document final attaché à la commande (ou au partenaire)

**UX :**
- Bouton visible uniquement si Digital Documents activé et commande confirmée (ou en brouillon selon implémentation)
- Rappel si signature requise avant enlèvement (message ou blocage)

---

## 4. Patterns de Navigation

### 4.1 Depuis Sales

- **Création commande** : Sales ‣ Orders ‣ New ; ajout de lignes produit « louables » avec dates → la commande se comporte en location
- **Suivi** : même liste/formulaire commandes, avec filtres et colonnes Rental
- **Reporting** : possibilité de vues/rapports « Locations » (revenus, taux d’occupation, retards)

### 4.2 Depuis Rental

- **Rental ‣ Orders** (ou équivalent) : liste des commandes de location
- **Rental ‣ Products** : catalogue des produits louables et configuration tarification
- **Rental ‣ Configuration ‣ Settings** : paramètres généraux et Sign
- **Rental ‣ Reporting** (si existant) : planning, disponibilités, revenus

### 4.3 Tâches enlèvement / retour

- Accès depuis la commande (lien vers les tâches ou activités) ou depuis Project / Activités
- Liste des enlèvements et retours à la date du jour ou à venir
- Coche ou bouton « Enlèvement effectué » / « Retour effectué » pour mise à jour statut et stock

---

## 5. Design et Accessibilité

### 5.1 Cohérence avec Sales

- Même charte que le module Sales (couleurs, boutons, tableaux)
- Terminologie claire : « Rental », « Pickup », « Return », « Security Time », « Extra Day/Hour »

### 5.2 Responsive

- Formulaire commande et lignes utilisables sur tablette pour enregistrement enlèvement/retour sur site
- Sign et reçu PDF accessibles sur mobile pour le client

### 5.3 Messages et retours utilisateur

- **Succès** : « Order confirmed », « Pickup registered », « Return registered », « Document signed »
- **Erreur** : « This period is not available (Security Time or overlapping rental) », « Product is not rentable »
- **Avertissement** : « Return is late — delay costs will be applied »

---

## 6. Recommandations pour Miyukini

### 6.1 Écrans

- **Page « Commande location »** : tout-en-un (lignes, dates, prix, statuts, signature, reçu) pour limiter les allers-retours
- **Page « Produit louable »** : onglet Location avec grille de prix, pénalités et Security Time regroupés
- **Vue planning** : calendrier ou timeline des locations par produit pour visualiser disponibilités et Security Time

### 6.2 Composants réutilisables

- **Bloc période location** : Start / End + durée calculée + prix calculé + alerte conflit
- **Bloc signature** : statut (non envoyé / en attente / signé) + bouton « Demander signature »
- **Bloc reçu** : bouton « Imprimer reçu enlèvement/retour » + aperçu éventuel

### 6.3 Accessibilité

- Labels explicites pour tous les champs (tarification, pénalités, Security Time)
- Résumé du calcul de prix (« X jours = N × durée Y ») pour utilisateurs et clients
- Contraste et focus pour boutons Sign et Print

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
