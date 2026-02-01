# Odoo POS Restaurant — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** du module POS Restaurant d'Odoo : personnas, scénarios d'usage (service en salle, prise de commande, transfert de table, cours, addition partagée, réservations), onboarding et points de friction, pour guider l'implémentation d'un équivalent dans Miyukini.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles (serveur, maître d’hôtel, cuisine, client)
- Parcours d'onboarding (activation Bar/Restaurant, création sols/tables)
- Scénarios d'usage (prise de commande à la table, transfert, cours, split addition, réservation)
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles

### 1.1 Serveur / Bar (POS User)

**Profil :**
- Prend les commandes à la table ou au bar
- Assigne les commandes aux tables, gère les tabs
- Valide les commandes (Order) et envoie en cuisine
- Encaisse et gère les additions (split, pourboires)
- Libère les tables en fin de service

**Permissions :**
- Utilisation du POS en mode Restaurant
- Accès au plan des tables et au registre
- Création / modification d’ordres, transfert, split
- Paiement et impression reçu

### 1.2 Maître d’hôtel / Responsable salle

**Profil :**
- Vue d’ensemble du plan des tables (occupation, réservations, retards)
- Gestion des réservations (Booking) et du placement des clients
- Édition du plan (création/ modification de tables, sols) depuis le frontend si droits ouverts
- Gestion des transferts de table et des fusions de commandes

**Permissions :**
- Accès au plan des tables et à la liste des ordres
- Gestion des réservations (création, déplacement d’étapes)
- Optionnel : édition du plan (Edit Plan)

### 1.3 Cuisine / Bar (préparation)

**Profil :**
- Reçoit les ordres via imprimantes de préparation ou écran préparation
- Ne utilise pas directement le POS Restaurant pour saisie ; consulte les tickets ou l’affichage préparation

**Permissions :**
- Pas d’accès au POS ; accès aux imprimantes / écran préparation selon configuration

### 1.4 Gestionnaire / Admin

**Profil :**
- Configure le POS en mode Bar/Restaurant
- Crée les sols et tables (backend ou paramètres POS)
- Configure les imprimantes de préparation (catégories → imprimantes)
- Configure presets (Take out / Delivery / Members), booking, pourboires, early receipt

**Permissions :**
- Configuration Point of Sale, Floor Plans, Preparation Printers, Settings

### 1.5 Client

**Profil :**
- Consommateur en salle, au bar, takeout ou livraison
- Peut voir l’addition avant paiement (Bill) si activé
- Peut laisser un pourboire (selon configuration et région)

**Permissions :**
- Aucun accès direct au POS ; interaction via le personnel

---

## 2. Parcours d'Onboarding

### 2.1 Activation du mode Restaurant

**Acteur :** Gestionnaire

**Étapes :**
1. Aller dans Point of Sale → Configuration → Settings.
2. Dans la section Point of Sale, activer **Is a Bar/Restaurant**.
3. Enregistrer.
4. Définir l’écran de démarrage par défaut (Tables ou Register) dans PoS Interface.
5. Optionnel : activer Take out / Delivery / Members, Booking, Tips, Allow Bill Splitting, Early Receipt Printing.

**Durée estimée :** 5–10 minutes

### 2.2 Création des sols et tables

**Depuis le backend :**
1. Point of Sale → Configuration → Floor Plans → New.
2. Saisir le nom du sol, lier au Point of Sale.
3. Ajouter une image de fond (optionnel).
4. Add a line : pour chaque table, saisir numéro, nombre de places, forme, dimensions, couleur, actif.
5. Save.

**Depuis le frontend POS :**
1. Ouvrir une session POS.
2. Menu (hamburger) → Edit Plan.
3. Add Floor → nom → Apply.
4. Table → ajouter des tables ; configurer sièges, forme, couleur, numéro (icônes dédiées).
5. Save.

**Durée estimée :** 15–30 min selon nombre de tables

### 2.3 Configuration imprimantes cuisine (optionnel)

1. Connecter l’imprimante (IoT ou Epson).
2. POS Settings → Preparation → Preparation Printers : activer.
3. Créer une imprimante, choisir le type (IoT ou Epson IP).
4. Printed Product Categories : associer catégories de produits à cette imprimante.
5. Save.

**Durée estimée :** 10–20 min

---

## 3. Scénarios d'Usage Principaux

### 3.1 Scénario : Prise de commande à la table

**Acteur :** Serveur

**Étapes :**
1. Sur le plan des tables, cliquer sur la table (occupation confirmée).
2. Ajouter les produits au panier.
3. Optionnel : utiliser Course pour séparer entrées / plats / desserts.
4. Cliquer **Order** → envoi en cuisine (si imprimantes configurées).
5. Retour au plan ou passage à une autre table.
6. Plus tard : rouvrir l’ordre (plan ou Orders), ajouter produits si besoin, **Fire Course 2** pour le cours suivant, etc.
7. **Payment** → choix du mode de paiement, client / facture si besoin → Validate.
8. Si panier vide, **Release table** pour libérer la table.

**Points d’attention :** Envoi séquentiel des cours ; libération de table pour mise à jour du plan.

### 3.2 Scénario : Transfert de table

**Acteur :** Serveur

**Étapes :**
1. Ouvrir l’ordre de la table source (plan ou Orders).
2. Actions → **Transfer/Merge**.
3. Choisir la table cible sur le plan (libre = transfert ; occupée = fusion).
4. Valider → l’ordre (et l’occupation) est déplacé ou fusionné.

**Points d’attention :** Fusion = deux commandes sur la même table ; les deux restent distinctes pour paiement sauf traitement manuel.

### 3.3 Scénario : Addition partagée (split)

**Acteur :** Serveur

**Étapes :**
1. Ouvrir l’ordre, cliquer **Payment**.
2. Actions → **Split**.
3. Sélectionner un ou plusieurs produits :
   - **Payment** : régler directement cette sélection.
   - **Split Order** : créer une sous-commande (à régler séparément).
   - **Transfer** : transférer vers une autre table.
4. Continue pour chaque « part » ; régler chaque sous-commande avant de revenir à l’ordre principal.

**Points de friction :** Workflow multi-étapes ; sous-commande obligatoirement réglée avant de continuer.

### 3.4 Scénario : Réservation (Booking)

**Acteur :** Maître d’hôtel ou réception

**Étapes :**
1. Activer Booking dans POS Settings ; configurer le type de rendez-vous (ressources = tables, capacités).
2. Dans le POS, cliquer **Booking**.
3. New → nom, date/heure, nombre de convives, téléphone, durée, ressources (tables).
4. Save ; déplacer la carte entre étapes (Booked, Checked-In, No Show).
5. Depuis le plan : clic sur la notification de réservation sur une table pour éditer rapidement.

**Points d’attention :** Dépendance à l’app Appointments ; cohérence ressources / capacités.

### 3.5 Scénario : Takeout / Delivery (presets)

**Acteur :** Serveur

**Étapes :**
1. Dans le registre, choisir le preset **Takeout** ou **Delivery** (si activé).
2. **Takeout** : Set Tab → nom de l’ordre, Apply ; date/heure de retrait.
3. **Delivery** : sélectionner ou créer un client ; choisir un créneau.
4. Saisir les produits, **Order** puis **Payment** comme d’habitude.

**Points d’attention :** Champs obligatoires selon preset (nom, client, créneau) et limites de capacité/heures si configurés.

---

## 4. Points de Friction Identifiés

- **Bill (addition avant paiement) :** selon configuration et matériel, le bouton Bill peut ne rien imprimer (PDF ou imprimante non liée) ; clarification nécessaire pour l’utilisateur.
- **Split :** workflow riche mais exigeant ; risque d’erreur si plusieurs sous-commandes et paiements partiels.
- **Booking :** double configuration (POS + Appointments) ; courbe d’apprentissage pour lier tables et ressources.
- **Édition du plan en frontend :** suppression de tables/sols définitive ; pas de corbeille.
- **Multi-ordres sur une table :** distinction entre les ordres (bouton +) à expliquer aux nouveaux utilisateurs.
- **Cours et impression cuisine :** dépendance aux catégories produits et à la configuration imprimantes ; mauvaise config = ordres non routés.

---

## 5. Recommandations pour Miyukini

- **Opérateur Floor/Table** : parcours dédié « premier démarrage » (création d’un sol + quelques tables) avec valeurs par défaut.
- **Parcours Serveur** : scénarios guidés (commande → cours → paiement → libération) et raccourcis clairs (Set Table, Order, Payment, Release table).
- **Split d’addition** : modélisation explicite (sous-commande, part par convive) avec états clairs et validation avant retour à l’ordre principal.
- **Réservations** : contrat d’équipe avec un service Appointments/RDV pour réutilisation des concepts (ressources, créneaux) sans dupliquer la logique.
- **Impression cuisine** : abstraction « canal de préparation » (imprimante, écran, autre) avec routage par catégorie ou tag, indépendant du matériel Odoo.
- **Documentation utilisateur** : fiches « Mode Restaurant », « Plan des tables », « Cours », « Split », « Réservations » pour réduire la friction.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
