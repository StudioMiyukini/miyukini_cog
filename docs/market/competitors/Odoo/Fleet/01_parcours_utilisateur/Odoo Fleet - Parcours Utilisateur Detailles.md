# Odoo Fleet — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Fleet** (Flotte véhicules) d'Odoo, identifiant les personas, scénarios d'usage, processus de demande et d'attribution de véhicules, et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 (Fleet, Models, Vehicles, Services, Accidents, Cost analysis)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles (Fleet Manager, HR / Admin, Conducteur, Employé demandeur)
- Parcours : configuration (fabricants, modèles, catégories), ajout véhicule, contrats, services, sinistres
- Scénarios : demande de véhicule (Belgique), alertes fin de contrat, analyse des coûts
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Administrateur Fleet (Fleet Administrator)

**Profil :**
- Rôle : Configuration globale du module Fleet et droits complets sur véhicules, modèles, contrats, services
- Responsabilités : Paramètres (End Date Contract Alert, New Vehicle Request), référentiels (fabricants, modèles, catégories, types de service), vue sur tout le parc et les coûts

**Besoins :**
- Configuration : alertes contrat, limites demande véhicule (Belgique)
- Gestion des fabricants, modèles, catégories, types de service
- Vue tableau de bord : véhicules, contrats à échéance, coûts totaux, par véhicule, par conducteur
- Export et rapports (pivot, comparaison détaillée)

**Permissions :** Droits complets Fleet ; accès Configuration.

### 1.2 Fleet Manager (Gestionnaire de flotte)

**Profil :**
- Rôle : Gestion courante du parc (véhicules, conducteurs, contrats, entretiens, sinistres)
- Responsabilités : Ajout/sortie de véhicules, assignation conducteurs, création contrats, enregistrement services et sinistres, suivi des coûts

**Besoins :**
- Liste / Kanban / Formulaire véhicules
- Création véhicule (modèle, immatriculation, VIN, conducteur, contrat)
- Gestion des contrats (création, responsable, alertes)
- Enregistrement des services (type, coût, fournisseur, odomètre, description)
- Gestion des sinistres (service type Accident, description, notes)
- Relevés odomètre
- Consultation des rapports de coûts (par véhicule, par conducteur, période)

**Permissions :** Gestion des véhicules, contrats, services ; pas nécessairement la configuration des référentiels (modèles, fabricants).

### 1.3 Conducteur (Driver)

**Profil :**
- Rôle : Utilisateur assigné à un véhicule ; peut être un employé (fiche HR avec Fleet Mobility Card) ou un contact
- Responsabilités : Utilisation du véhicule ; saisie éventuelle des relevés odomètre ; déclaration sinistre (selon processus interne)

**Besoins :**
- Vue sur le(s) véhicule(s) qui lui sont assignés
- Saisie des relevés kilométriques (si droits accordés)
- Information sur prochains entretiens / échéances contrat (si exposé dans l’UI)

**Permissions :** Limité à ses véhicules ; saisie odomètre et lecture selon droits.

### 1.4 Employé demandeur (Belgique — New Vehicle Request)

**Profil :**
- Rôle : Employé pouvant demander un véhicule de fonction (avantage en nature)
- Responsabilités : Soumettre une demande pour un modèle éligible ; respect des limites (parc, politique entreprise)

**Besoins :**
- Formulaire ou flux « Demande de véhicule » : choix du modèle (liste des modèles avec « Can be requested »)
- Suivi du statut de la demande (en attente, acceptée, refusée)
- Information sur les modèles éligibles et les limites

**Permissions :** Accès au flux demande véhicule ; pas d’accès à la gestion complète du parc.

### 1.5 Responsable contrat (Contract Responsible)

**Profil :**
- Rôle : Personne désignée comme responsable d’un contrat (assurance, leasing) sur un véhicule
- Responsabilités : Recevoir les alertes « fin de contrat » (email) et initier le renouvellement ou la clôture

**Besoins :**
- Réception des emails d’alerte (X jours avant échéance)
- Accès à la fiche contrat / véhicule pour renouveler ou clôturer (selon droits)

**Permissions :** Lecture contrat(s) dont il est responsable ; édition selon droits Fleet.

---

## 2. Parcours d’Onboarding / Configuration

### 2.1 Mise en place initiale Fleet

1. **Activation** : Installer l’app Fleet (Human Resources).
2. **Configuration** : Fleet → Configuration → Settings  
   - **End Date Contract Alert** : nombre de jours avant échéance pour envoi de l’email (ex. 30).  
   - **New Vehicle Request** (Belgique) : activer et définir les limites selon disponibilité du parc.
3. **Référentiels** :  
   - **Fabricants** : vérifier les 67 préchargés ; ajouter les manquants (nom, logo).  
   - **Catégories** : créer les catégories de modèles (ex. Compacte, Berline, Utilitaire).  
   - **Modèles** : créer les modèles utilisés (fabricant, type car/bike, catégorie, informations moteur, fiscalité si Belgique, fournisseurs).  
   - **Types de service** : créer les types (Vidange, Révision, Pneus, Accident - Faute conducteur, Accident - Sans faute, etc.).
4. **Véhicules** : ajouter les véhicules existants (modèle, immatriculation, VIN, conducteur, contrat éventuel).
5. **Contrats** : renseigner les contrats en cours (assurance, leasing) avec responsable et dates.
6. **Historique** : saisir les derniers relevés odomètre et les derniers services si besoin de continuité.

**Points d’attention :**
- Sans modèle, aucun véhicule ne peut être créé.
- Type véhicule (car/bike) fixe : nécessaire pour intégration Paie (avantage en nature).

### 2.2 Premier véhicule

1. Fleet → Configuration → Modèles → New : nom, fabricant, type, catégorie, onglets Information / Engine / Vendors (et Salary si Belgique).
2. Fleet → Véhicules → New : sélectionner le modèle, immatriculation, VIN, conducteur (créer un contact si besoin), onglets Contract, Tax Info, Note.
3. Si contrat : créer le contrat (type, dates, montant, responsable).
4. Saisir un premier relevé odomètre et, si besoin, un premier service (ex. mise en service).

---

## 3. Scénarios d’Usage Principaux

### 3.1 Ajout d’un nouveau véhicule au parc

- **Acteur** : Fleet Manager (ou Admin).
- **Parcours** : Fleet → Véhicules → New.
- **Saisie** : Modèle (obligatoire), immatriculation, VIN, société, conducteur (optionnel), onglets Fiscality, Contract, Model, Note.
- **Résultat** : Véhicule actif dans le parc ; prêt pour relevés odomètre et services.
- **Friction possible** : Si le modèle n’existe pas, retour en Configuration → Modèles (et éventuellement Fabricant).

### 3.2 Assignation / changement de conducteur

- **Acteur** : Fleet Manager.
- **Parcours** : Ouvrir la fiche véhicule → champ Driver → sélectionner un contact (ou créer). Si Employees : lien possible avec employé (Fleet Mobility Card sur fiche employé).
- **Résultat** : Le conducteur est associé au véhicule ; les prochains services/odomètre peuvent être enregistrés avec ce conducteur.

### 3.3 Création et suivi d’un contrat (assurance / leasing)

- **Acteur** : Fleet Manager (ou Admin).
- **Parcours** : Depuis la fiche véhicule (onglet Contract) ou Fleet → Contrats → New : type, véhicule, dates, montant, responsable.
- **Résultat** : Contrat enregistré ; alertes envoyées au responsable selon paramètre « End Date Contract Alert ».
- **Suivi** : Liste / filtre « Contrats à échéance » ; email automatique au responsable.

### 3.4 Enregistrement d’un service (entretien / réparation)

- **Acteur** : Fleet Manager (ou conducteur si droits).
- **Parcours** : Fleet → Services → New (ou depuis la fiche véhicule) : véhicule, type de service, date, coût, fournisseur, odomètre, description, notes.
- **Résultat** : Service enregistré ; pris en compte dans les coûts par véhicule et par conducteur.
- **Kanban** : Si workflow par stades (planifié / en cours / terminé), déplacement de la carte.

### 3.5 Déclaration d’un sinistre

- **Acteur** : Fleet Manager ou conducteur (selon processus).
- **Parcours** :  
  1. Choisir le type de service « Accident - Faute conducteur » ou « Accident - Sans faute ».  
  2. Créer une fiche service : véhicule, conducteur, date, coût, fournisseur, description des travaux, **notes** (détails sinistre : lieu, circonstances, tiers).  
  3. Si plusieurs réparations (plusieurs fournisseurs) : dupliquer ou créer plusieurs services avec les **mêmes notes** pour lier au même sinistre.
- **Résultat** : Sinistre tracé ; filtrage possible par véhicule, conducteur, faute, coût (rapports et tableaux de bord).

### 3.6 Relevé odomètre

- **Acteur** : Fleet Manager ou conducteur (si droits).
- **Parcours** : Depuis la fiche véhicule ou menu dédié : saisir valeur odomètre et date (et conducteur si pertinent).
- **Résultat** : Historique d’usage ; base pour planification des prochains entretiens et analyse des km parcourus.

### 3.7 Demande de véhicule (Belgique — New Vehicle Request)

- **Acteur** : Employé demandeur.
- **Parcours** : Accéder au flux « Demande de véhicule » (menu selon implémentation) → choisir un modèle dans la liste des modèles éligibles (« Can be requested ») → soumettre.
- **Validation** : RH / Fleet Manager accepte ou refuse (selon limites parc et politique).
- **Résultat** : Si accepté : attribution d’un véhicule (ou commande) et liaison à l’employé (conducteur).

### 3.8 Analyse des coûts

- **Acteur** : Fleet Manager ou Admin.
- **Parcours** : Fleet → Cost analysis (ou rapports) : sélection période, filtres (véhicule, conducteur, société). Consultations : Total costs, Cost by vehicle, Cost by driver, Detailed comparison.
- **Résultat** : Export possible (pivot, CSV) pour reporting et comparaison.

---

## 4. Points de Friction Identifiés

### 4.1 Référentiels

- **Modèles non préchargés** : Chaque modèle doit être créé manuellement ; risque d’incohérence (doublons, libellés différents).
- **Types car/bike fixes** : Pas d’extension (ex. deux-roues motorisé, utilitaire léger) sans évolution Odoo ; peut limiter certains parcs.

### 4.2 Conducteur

- **Un seul conducteur par véhicule** : Pas de gestion native de véhicules partagés (tour de rôle) ; contournement par changement manuel du conducteur ou processus hors outil.
- **Lien employé** : Dépend de l’app Employees et du champ Fleet Mobility Card ; configuration à maintenir.

### 4.3 Contrats et alertes

- **Un seul paramètre global** « End Date Contract Alert » : Même nombre de jours pour tous les contrats ; pas de règle par type de contrat (ex. assurance 30 j, leasing 60 j) sans personnalisation.
- **Responsable** : Doit être un utilisateur Odoo pour recevoir l’email ; pas de simple contact email externe standard.

### 4.4 Services et sinistres

- **Sinistres = services** : Pas de modèle « Accident » dédié avec champs structurés (lieu, circonstances, tiers, constat) ; tout passe par description et notes. Plusieurs lignes pour plusieurs fournisseurs avec mêmes notes pour garder le lien.
- **Workflow services** : Étapes (planifié → terminé) selon configuration ; pas toujours visible ou configurable finement selon version.

### 4.5 Coûts et Accounting

- **Intégration comptable** : Selon version et modules, le lien Fleet ↔ Accounting peut être partiel (export manuel, pas de compte analytique dédié automatique) ; à vérifier par déploiement.

### 4.6 Demande de véhicule (Belgique)

- **Disponible uniquement pour localisation Belgique** : Autres pays : pas de flux standard « demande de véhicule » dans Fleet ; processus manuel ou module spécifique.

---

## 5. Recommandations pour Miyukini

### 5.1 Parcours

- **Référentiels** : Prévoir des modèles et fabricants cohérents (unicité, libellés normalisés) ; import possible depuis catalogue ou API constructeurs si pertinent.
- **Conducteur** : Conserver « un conducteur principal » par véhicule ; compléter par historique d’assignation (dates, conducteur) pour traçabilité et coût par conducteur.
- **Alertes contrat** : Paramètre par type de contrat (nombre de jours par type) et responsable par contrat ; notifications (email + in-app) avec lien direct vers la fiche contrat.
- **Sinistres** : Modèle dédié « Sinistre » (Accident) avec champs structurés (lieu, circonstances, tiers, constat, coût total) et liaison à une ou plusieurs « réparations » (lignes de coût par fournisseur) pour garder une vue unifiée.
- **Demande de véhicule** : Proposer un flux générique « Demande de véhicule » (éligibilité des modèles, limites parc, validation RH/Fleet) indépendant de la localisation Belgique, paramétrable par pays / avantage en nature.

### 5.2 Personas

- **Fleet Manager** : Tableau de bord (véhicules, contrats à échéance, coûts, prochains entretiens) et actions rapides (nouveau véhicule, nouveau service, alerte).
- **Conducteur** : Vue limitée « Mes véhicules », saisie odomètre, déclaration sinistre simplifiée (formulaire guidé).
- **Responsable contrat** : Notifications claires avec action « Renouveler » / « Clôturer » et accès direct au contrat.

### 5.3 Friction

- Réduire la friction « modèle manquant » : suggestion de création de modèle depuis la fiche véhicule (création en une étape avec fabricant/catégorie par défaut).
- Historique d’assignation conducteur : évolution sans casser la règle « un conducteur actif » pour les rapports et alertes.
- Rapports coûts : export pivot + liaison native avec module Comptabilité / Analytique (Miyukini) pour ventilation automatique par véhicule, conducteur, compte.

---

**Document** : Odoo Fleet — Parcours Utilisateur Détaillés  
**Version** : 1.0  
**Date** : 2026-02-01
