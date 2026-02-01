# Odoo Rental — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Rental** d'Odoo : personas, scénarios d'usage, onboarding et points de friction, pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation et fonctionnalités Odoo Rental

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles (gestionnaire location, commercial, magasinier, client)
- Parcours d'onboarding (configuration produits, tarification, paramètres)
- Scénarios d'usage principaux (devis, confirmation, enlèvement, retour, facturation)
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Gestionnaire Location (Rental Manager)

**Profil :**
- Responsable du catalogue location et des paramètres
- Définit les produits louables, les grilles tarifaires et les pénalités
- Configure Security Time et règles de disponibilité
- Gère les contrats types (Sign) et les impressions (reçus)

**Besoins :**
- Configuration centralisée : Rental ‣ Products, Rental ‣ Configuration ‣ Settings
- Grilles de prix par produit (heure, jour, semaine, mois) et par durée
- Paramétrage Extra Hour / Extra Day et Security Time
- Choix du modèle de document Sign pour l’accord de location
- Vue d’ensemble des locations en cours et des retours à venir

**Permissions :**
- Accès aux paramètres Rental
- Gestion des produits et de la tarification location
- Accès aux commandes et rapports location

### 1.2 Commercial / Vendeur (Sales User)

**Profil :**
- Crée les devis et commandes de location
- Saisit les dates d’enlèvement et de retour
- Envoie les devis et suit les confirmations
- Déclenche la signature du contrat (Sign) si activé
- Imprime les reçus d’enlèvement et de retour

**Besoins :**
- Création de devis depuis une vue unique (devis, confirmation, planning)
- Saisie simple des dates et calcul automatique du prix
- Envoi du contrat à signer et suivi des signatures
- Impression du reçu (Pickup and Return Receipt) pour le client
- Suivi du statut : enlèvement effectué, retour effectué, facturé

**Permissions :**
- Accès Sales + Rental : création/modification des commandes et lignes location
- Utilisation de Sign (envoi, rappels) si Digital Documents activé

### 1.3 Magasinier / Logistique (Warehouse User)

**Profil :**
- Enregistre les enlèvements et retours physiques
- Gère les mouvements de stock Rental In / Rental Out
- Prépare les produits pour enlèvement et réceptionne au retour

**Besoins :**
- Liste des enlèvements et retours planifiés (tâches ou ordres)
- Validation des livraisons (passage en Rental Out) et des réceptions (Rental In)
- Alerte sur retards et pénalités éventuelles
- Pas de double sortie pour un même produit (cohérence stock)

**Permissions :**
- Accès Stock / Warehouse et aux opérations liées à la location
- Lecture des commandes location pour les dates et quantités

### 1.4 Client (Customer / Portal)

**Profil :**
- Consulte le devis et accepte la location
- Signe le contrat (Sign) si demandé
- Vient enlèvement et retour aux dates convenues
- Reçoit le reçu d’enlèvement/retour et la facture

**Besoins :**
- Devis clair (dates, prix, pénalités éventuelles)
- Signature en ligne du contrat (lien Sign)
- Reçu imprimable avec rappel des dates et des conditions
- Facture après retour (et pénalités si retard)

**Permissions :**
- Portail client : voir ses commandes, signer, télécharger reçu/facture

---

## 2. Parcours d'Onboarding

### 2.1 Configuration initiale (Gestionnaire)

1. **Activer l’app Rental** (si ce n’est pas fait).
2. **Rental ‣ Configuration ‣ Settings**  
   - Activer « Digital Documents » si signature requise (Sign installé).  
   - Choisir le modèle « Rental Agreement » (ou en créer un).
3. **Rental ‣ Products**  
   - Pour chaque produit à louer :  
     - Cocher « Can be Rented ».  
     - Onglet Rental : ajouter des lignes de prix (unité de temps, durée, prix).  
     - Réservations : renseigner Extra Hour, Extra Day, Security Time (heures).
4. **Vérifier les emplacements** Rental In / Rental Out (Stock / Configuration entrepôts).

### 2.2 Premier devis location (Commercial)

1. Créer une commande (Sales) ou une commande type « location ».
2. Ajouter des lignes produit « louables ».
3. Saisir les dates d’enlèvement et de retour (par ligne ou globalement).
4. Vérifier le prix calculé automatiquement.
5. Envoyer le devis au client.
6. Après confirmation : demander la signature (Sign) si activé, puis valider enlèvement/retour et facturation.

---

## 3. Scénarios d'Usage Principaux

### 3.1 Devis → Confirmé → Enlèvement → Retour → Facturé

**Acteurs :** Commercial, Magasinier, Client

**Étapes :**
1. Commercial crée un devis avec produits louables et dates.
2. Client accepte ; commercial confirme la commande.
3. Système crée livraison/réception et tâches enlèvement/retour.
4. Commercial envoie le contrat à signer (Sign) ; client signe.
5. Jour J enlèvement : magasinier enregistre l’enlèvement ; stock passe en Rental Out.
6. Commercial imprime le reçu d’enlèvement/retour pour le client.
7. Jour J retour : magasinier enregistre le retour ; stock repasse en Rental In ; pénalités calculées si retard.
8. Facturation : ligne location + pénalités éventuelles.

### 3.2 Prolongation (extension de location)

**Acteurs :** Commercial, Client

**Étapes :**
1. Client demande une prolongation.
2. Commercial crée une nouvelle ligne ou modifie la date de fin (selon implémentation Odoo).
3. Recalcul du prix et des disponibilités (Security Time, autres commandes).
4. Validation et éventuelle facturation du complément.

### 3.3 Vente du produit loué

**Acteurs :** Commercial

**Comportement (selon versions)** : possibilité de convertir la location en vente (produit déjà chez le client). Workflow dédié si la fonction existe (facturation vente, sortie définitive du stock location).

### 3.4 Consultation planning et disponibilités

**Acteurs :** Gestionnaire, Commercial

**Besoins :**
- Vue planning des locations (dates début/fin par produit ou par commande).
- Vérification des créneaux disponibles en tenant compte du Security Time.
- Liste des retours à venir pour préparer les réceptions.

---

## 4. Points de Friction Identifiés

### 4.1 Tarification

- **Règle « option la moins chère »** : peut surprendre (ex. 8 jours = 3× 3 jours) ; à expliquer clairement en UI et dans les conditions.
- **Multiplicité des lignes de prix** : risque d’erreurs de configuration (doublons, incohérences d’unités). Recommandation : assistant ou contrôles de cohérence.

### 4.2 Disponibilité

- **Security Time** : si mal paramétré (trop long/court), impact direct sur la disponibilité perçue par le client et sur les plannings.
- **Chevauchements** : messages d’erreur explicites si créneau déjà pris ou Security Time non respecté.

### 4.3 Signature (Sign)

- **Dépendance Sign** : sans Sign, pas de signature électronique ; parcours alternatif (PDF à signer manuellement, ou pas de signature).
- **Workflow** : s’assurer que « Sign Documents » est visible et guidé depuis la commande location.

### 4.4 Stock

- **Rental In / Rental Out** : visibilité claire des quantités « à louer » vs « chez le client » pour éviter surréservation.
- **Retards** : gestion des retours tardifs (pénalités, blocage d’autres locations) à clarifier dans les processus.

### 4.5 Multi-entrepôts / Multi-sociétés

- **Emplacements** : chaque entrepôt doit avoir ses Rental In/Out ; configuration et formation nécessaires.
- **Prix et devises** : cohérence des grilles par société/devise si multi-company.

---

## 5. Recommandations pour Miyukini

### 5.1 Parcours

- **Un seul écran « Commande location »** : devis, dates, prix, signature, statut enlèvement/retour/facturation.
- **Calcul de prix transparent** : afficher la règle utilisée (ex. « 8 jours = 3 × 3 jours ») et les pénalités possibles.
- **Planning visuel** : calendrier ou timeline des locations par produit pour éviter les chevauchements et illustrer le Security Time.

### 5.2 Personas

- **Opérateur Location (service)** : agrège commandes, tarification, disponibilité, stock location.
- **Opérateur Interface Location (UI)** : formulaire commande, planning, reçus, suivi signature.
- **Mandats** : création/modification commande, enlèvement, retour, facturation, avec niveaux de sécurité adaptés (données clients, engagements).

### 5.3 Onboarding

- **Checklist de configuration** : produits louables, au moins une grille de prix, Security Time, option Sign, emplacements Rental In/Out.
- **Tours guidés** : premier produit louable, premier devis, première signature, premier reçu.

### 5.4 Réduction des frictions

- **Validation des grilles** : détection de lignes redondantes ou incohérentes.
- **Alerte disponibilité** : avant confirmation, vérifier créneau libre + Security Time.
- **Messages explicites** : « Créneau indisponible à cause du Security Time » ou « Produit déjà loué sur cette période ».

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
