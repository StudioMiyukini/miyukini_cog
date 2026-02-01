# Odoo Subscriptions — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Subscriptions** (Abonnements) d'Odoo (version 19.0), à partir de la documentation officielle. Il identifie les vues, composants, patterns de navigation et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 (Subscriptions, Renewals, Upselling, Closing, eCommerce, Automatic Payments)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Structure de navigation et menus
- Vues principales (Plans récurrents, Produits, Devis / Commandes abonnement)
- Formulaires (plan, produit, devis abonnement)
- Boutons d’action (Renew, Upsell, Close, Sales History)
- Wizards (Close Reason, portail Close Subscription)
- Portail client et eCommerce
- Recommandations pour Miyukini

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Structure de Navigation

### 1.1 Menu principal Subscriptions

- **Subscriptions** (app racine)
  - **Dashboard** (vue d’ensemble)
  - **Quotations** : liste des devis / commandes abonnement (filtres par statut)
  - **Products** : produits abonnement
  - **Configuration**
    - **Recurring Plans** : plans récurrents
    - **Close Reasons** : raisons de clôture (admin et client)

### 1.2 Accès depuis Sales

- Depuis l’app **Sales** : même modèle de commande ; création d’un devis avec **Recurring Plan** et lignes produits abonnement fait apparaître l’abonnement dans Subscriptions et donne accès aux actions Renew, Upsell, Close.

### 1.3 Entrées secondaires

- **Sales History** : accessible via smart button sur la commande abonnement (après confirmation / renouvellements).
- **Preview** : aperçu du portail client depuis la commande (bouton en en-tête).

---

## 2. Vues Principales

### 2.1 Recurring Plans (Plans récurrents)

**Vue liste / dashboard :**
- Liste des plans avec colonnes typiques : Name, Billing Period, Automatic Closing, Closable, etc.
- Action **New** → formulaire vierge.

**Formulaire plan :**
- **En-tête** : Name.
- **Sections** :
  - **DETAILS** : Billing Period (unité + valeur), Automatic Closing (jours), Align to Period Start, Company, Invoice Email Template.
  - **SELF-SERVICE** : Closable, Add Products, Renew, Optional Plans.
  - **Pricing** (onglet) : lignes Produit / Variante, Pricelist, Recurring Price.
- **Smart buttons** (en haut du formulaire) :
  - **Subscriptions** : nombre d’abonnements actifs sous ce plan → liste des commandes.
  - **Subscription Items** : liste des lignes récurrentes actives (itemisées, avec Subscription et Customer).

**Comportement :**
- Restriction affichée : Billing Period ne peut pas être en « Days » pour les abonnements.
- Lien interne vers le template d’email (Invoice: Sending) au survol du champ Invoice Email Template.

### 2.2 Produits abonnement (Products)

**Navigation :** Subscriptions → Products → New ou éditer un produit existant.

**Formulaire produit :**
- **General Information** :
  - Product type (souvent Service), Invoicing policy, Unit of Measure, Sales Price.
  - Options **Subscriptions** et **Sales** activées par défaut (à conserver).
- **Attributes & Variants** : optionnel (variantes).
- **Recurring Prices** (onglet) : lignes « Add a price rule » (plan / période, prix récurrent).
- **Smart button** : **Go to Website** pour publier sur eCommerce (slider Unpublished → Published).

**Message important** : Pour produit **physique**, politique de facturation = **Ordered quantities** (sinon erreurs à la facturation).

### 2.3 Devis / Commandes abonnement (Quotations)

**Vue liste :**
- Colonnes typiques : Référence, Client, Plan récurrent, Date, Montant, Statut (Quotation, Sales Order, In Progress, Renewal Quotation, Churned, Payment Failure, Closed), etc.
- Filtres : par statut (Quotations, In Progress, Churned, Payment Failure), par plan, par client.
- **Renew** visible sur la ligne (boutons au-dessus du formulaire une fois la commande ouverte) si prérequis remplis.

**Formulaire commande abonnement :**
- Structure type **sale.order** : Customer, Recurring Plan, Order Lines, Payment Terms, Pricelist, Expiration, etc.
- **Onglet Other Info**, section **SALES** : Online signature, Online payment (checkboxes).
- **Onglet Other Info**, section **SUBSCRIPTION** : **Contract in exception** (checkbox) — coché automatiquement en cas d’échec de paiement.
- **Boutons d’action** (au-dessus du formulaire, selon état) :
  - **Renew** : crée un devis « Renewal Quotation ».
  - **Upsell** : crée un devis « Upsell » (bandeau Upsell en haut à droite).
  - **Close** : ouvre le pop-up **Close Reason** (saisie ou liste de raisons).
- **Smart button** : **Sales History** (après confirmation) → page listant les commandes liées avec leur Subscription Status.
- **Preview** : ouvre le portail client pour voir la commande comme le client.

**Tags / états visuels :**
- **In Progress**, **Renewal Quotation**, **Churned**, **Payment Failure**, **Closed** (badges ou libellés selon version).

### 2.4 Devis Renouvellement (Renewal Quotation)

- Nouveau devis créé au clic sur **Renew**.
- Tag **Renewal Quotation**.
- Chatter : dates de début et prochaine facture renseignées automatiquement.
- Workflow standard : Confirmer → Facturer → Enregistrer paiement.

### 2.5 Devis Upsell

- Nouveau devis créé au clic sur **Upsell**.
- Bannière **Upsell** en haut à droite.
- Lignes initiales reprises ; avertissement sous les lignes récurrentes (prorata, uniquement pour les services).
- Ajout de produits dans Order Lines, envoi par email, confirmation → prorata appliqué aux services.

---

## 3. Wizards et Pop-ups

### 3.1 Close Reason (administration)

- **Déclencheur** : bouton **Close** sur la commande abonnement (In Progress).
- **Contenu** : pop-up avec champ **Reason** (saisie libre ou liste déroulante) et bouton **Submit**.
- **Effet** : statut **Churned** + motif enregistré dans le chatter.

### 3.2 Close Subscription (portail client)

- **Déclencheur** : bouton **Close Subscription** (côté gauche de la commande dans le portail).
- **Contenu** : pop-up avec **liste de raisons prédéfinies** (Configuration → Close Reasons) et bouton **Submit**.
- **Effet** : commande marquée **Closed** ; raison visible en backend.

---

## 4. Portail Client et eCommerce

### 4.1 Portail

- **Accès** : Preview depuis la commande (admin) ou connexion client.
- **Contenu** : liste des devis / commandes ; détail commande avec montants, lignes, statut.
- **Actions** (si self-service activé sur le plan) :
  - **Renew** : créer un devis de renouvellement.
  - **Add products** : déclencher un upsell.
  - **Close Subscription** : choix d’une raison prédéfinie → soumission.
- **Paiement** : signature et paiement en ligne si exigés ; saisie moyen de paiement pour tokenisation (si provider configuré).

### 4.2 eCommerce

- **Fiche produit** : produit abonnement publié (slider Published).
- **Panier / Checkout** : même tunnel que les ventes classiques ; option « sauvegarder ma carte » (ou équivalent) si tokenisation activée.
- **Backend** : création et confirmation automatiques des devis abonnement après achat en ligne.

---

## 5. Patterns et Feedback

- **Smart buttons** : Subscriptions / Subscription Items (plan), Sales History (commande) — navigation directe vers les listes associées.
- **Tags de statut** : In Progress, Churned, Payment Failure, Renewal Quotation, Closed — repérage visuel rapide.
- **Boutons conditionnels** : Renew, Upsell, Close affichés selon l’état et les prérequis (facturation, paiement).
- **Avertissements** : prorata (services uniquement), politique de facturation pour produits physiques, restriction « Days » pour les plans.
- **Contract in exception** : indiqué visuellement (Payment Failure + checkbox) ; résolution manuelle (mode développeur pour décocher).

---

## 6. Recommandations pour Miyukini

- **Plans récurrents** : formulaire structuré en sections (DETAILS, SELF-SERVICE, Pricing) avec smart buttons d’impact (nombre d’abonnements, lignes actives).
- **Commande abonnement** : regrouper les champs et actions spécifiques (Recurring Plan, next invoice date, Renew, Upsell, Close, Sales History, Contract in exception) dans un bloc dédié « Abonnement » pour éviter la dispersion.
- **Résiliation** : wizard unique côté admin (raison libre ou liste) ; portail avec liste fermée de raisons (Close Reasons) et message de confirmation après clôture.
- **Exception paiement** : flux métier dédié « Résoudre échec paiement » (checklist : paiement reçu ou non, création facture si besoin, déblocage des actions planifiées) sans exiger le mode développeur.
- **Prorata** : afficher clairement dans l’UI que le prorata s’applique aux services uniquement (tooltip ou message contextuel sur les lignes).
- **Portail** : mêmes libellés et comportements que la doc (Renew, Add Products, Close Subscription) pour faciliter l’adoption par les utilisateurs venant d’Odoo.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
