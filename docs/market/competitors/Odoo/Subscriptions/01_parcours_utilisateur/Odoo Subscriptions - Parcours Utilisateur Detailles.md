# Odoo Subscriptions — Parcours Utilisateur Détaillés

## Contexte

Ce document analyse les **parcours utilisateur** de l'application **Subscriptions** (Abonnements) d'Odoo, identifiant les personas, scénarios d'usage, processus d'onboarding et points de friction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 (Subscriptions, Renewals, Upselling, Closing, eCommerce, Automatic Payments)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Personas et rôles (Administrateur, Commercial, Client abonné, Support)
- Parcours d'onboarding (configuration plans, produits, eCommerce)
- Scénarios d'usage (création abonnement, renouvellement, upsell, résiliation, paiement auto)
- Points de friction identifiés
- Recommandations pour Miyukini

---

## 1. Personas et Rôles Utilisateurs

### 1.1 Administrateur / Configurateur

**Profil :**
- Rôle : Configuration des plans récurrents, produits abonnement, raisons de clôture, email facture, options self-service.
- Responsabilités :
  - Créer et modifier les plans récurrents (Billing Period, Automatic Closing, Align to Period Start, Invoice Email Template).
  - Configurer les options self-service (Closable, Add Products, Renew, Optional Plans).
  - Définir les prix par plan (onglet Pricing des plans, Recurring Prices des produits).
  - Créer les raisons de clôture (Configuration → Close Reasons).
  - Activer les remises (Sales → Configuration → Settings → Discounts) pour upsell avec réduction.
  - Configurer les prestataires de paiement avec tokenisation (Stripe, Adyen, etc.) pour paiements automatiques.

**Besoins :**
- Interface claire par plan (DETAILS, SELF-SERVICE, Pricing).
- Smart buttons (Subscriptions, Subscription Items) pour suivre l’usage des plans.
- Contrôle multi-société (Company sur le plan).

**Permissions :**
- Accès Configuration Subscriptions, Sales, Invoicing, Payment providers.

### 1.2 Commercial / Vendeur

**Profil :**
- Rôle : Création de devis abonnement, confirmation, renouvellement manuel, upsell, résiliation côté back-office.
- Responsabilités :
  - Créer des devis depuis Sales ou Subscriptions (client, plan récurrent, lignes produits abonnement).
  - Envoyer le devis par email ou faire signer / payer en ligne (signature, paiement).
  - Confirmer le devis → abonnement In Progress.
  - Lancer un renouvellement (Renew) et traiter le devis de renouvellement.
  - Proposer un upsell (Upsell) et faire confirmer le devis d’upsell.
  - Clôturer un abonnement (Close) avec motif (Close Reason).
  - Consulter Sales History pour l’historique des commandes liées.

**Besoins :**
- Boutons d’action visibles (Renew, Upsell, Close) selon l’état de l’abonnement.
  - Renew : visible quand abonnement confirmé, plan configuré, première facture payée.
  - Upsell : après première facturation.
  - Close : quand abonnement In Progress.
- Filtres (Quotations, In Progress, Churned, Payment Failure).
- Gestion des exceptions : décocher « Contract in exception » (mode développeur) après résolution d’un échec de paiement.

**Permissions :**
- Accès Subscriptions, Sales, lecture/écriture sur commandes et factures selon droits.

### 1.3 Client abonné (Portail client)

**Profil :**
- Rôle : Consulter ses abonnements, payer, renouveler, ajouter des produits, résilier (si Closable).
- Responsabilités :
  - Voir les devis et commandes abonnement depuis le portail.
  - Signer et payer en ligne (si exigé).
  - Saisir ou confirmer un moyen de paiement pour les paiements récurrents (tokenisation).
  - Si self-service activé : Renew (créer un devis de renouvellement), Add Products (upsell), Close Subscription (avec choix de raison prédéfinie).

**Besoins :**
- Vue claire des abonnements (actifs, fermés, en attente de paiement).
  - Bouton « Close Subscription » visible si le plan est Closable.
- Liste de raisons de clôture imposée (pas de texte libre).
- Retour visuel après clôture (statut Closed).

**Permissions :**
- Accès portail limité à ses propres commandes / abonnements.

### 1.4 Support / Helpdesk

**Profil :**
- Rôle : Triage et traitement des demandes d’abonnés (avantage abonnement).
- Responsabilités :
  - Identifier le client comme abonné (lien CRM / Helpdesk avec abonnement).
  - Suivre les cas « Payment Failure » ou « Contract in exception » (vérification paiement, instructions pour décocher l’exception).

**Besoins :**
- Lien entre ticket / contact et abonnement (statut, prochaine facture, historique).

---

## 2. Parcours d'Onboarding

### 2.1 Premier déploiement (Administrateur)

1. **Activation** : Installer l’app Subscriptions (dépendances Sales, Invoicing).
2. **Plans récurrents** : Subscriptions → Configuration → Recurring Plans → créer au moins un plan (ex. Monthly, Yearly) avec Billing Period, Automatic Closing, Align to Period Start, Invoice Email Template, options self-service (Closable, Add Products, Renew, Optional Plans), optionnel Pricing.
3. **Produits** : Subscriptions → Products → créer ou éditer des produits ; cocher Subscriptions (et Sales) ; type souvent Service ; politique de facturation, UoM, Sales Price ; onglet Recurring Prices par plan.
4. **Raisons de clôture** : Subscriptions → Configuration → Close Reasons → créer les motifs (côté client et admin).
5. **Remises** (pour upsell avec discount) : Sales → Configuration → Settings → cocher Discounts → Save.
6. **Email facture** : Vérifier le template (ex. Invoice: Sending) dans le plan ou globalement.
7. **Optionnel – eCommerce** : Publier les produits abonnement sur le site (Go to Website → Published).
8. **Optionnel – Paiements automatiques** : Configurer un provider avec tokenisation ; configurer les méthodes de paiement et la tokenisation ; les clients pourront enregistrer un moyen de paiement au checkout ou dans le portail.

### 2.2 Premier abonnement (Commercial)

1. Depuis Sales ou Subscriptions → New.
2. Renseigner Customer, Recurring Plan, Order Lines (produits abonnement).
3. Optionnel : Quotation Template, Expiration, Pricelist, Payment Terms.
4. Send by Email ou Confirm ; si Online signature / Online payment, le client signe ou paie depuis le portail.
5. Après confirmation : facturer (selon politique) et enregistrer le paiement → abonnement In Progress ; bouton Renew puis Upsell disponibles selon prérequis.

### 2.3 Premier achat eCommerce (Client)

1. Parcourir le site, ajouter un produit abonnement au panier.
2. Checkout : adresse, livraison si applicable, paiement.
3. Si tokenisation activée : option « sauvegarder ma carte » (ou équivalent) pour les renouvellements automatiques.
4. En backend : devis abonnement créé et confirmé automatiquement.

---

## 3. Scénarios d'Usage Principaux

### 3.1 Création manuelle d’un abonnement

- **Acteur** : Commercial.
- **Étapes** : Nouveau devis → Client + Plan récurrent + Lignes produits abonnement → Envoi ou confirmation → Signature / paiement en ligne si requis → Confirmation → Facturation → Enregistrement paiement.
- **Résultat** : Commande en statut In Progress ; prochaine date de facture calculée ; actions planifiées actives (sauf Contract in exception).

### 3.2 Renouvellement manuel

- **Acteur** : Commercial ou Client (si Renew en self-service).
- **Prérequis** : Abonnement confirmé, plan configuré, première facture payée.
- **Étapes** : Ouvrir la commande abonnement → Renew → Nouveau devis « Renewal Quotation » → Confirmer → Facturer → Enregistrer paiement.
- **Résultat** : Nouvelle commande liée dans Sales History ; abonnement prolongé.

### 3.3 Upsell

- **Acteur** : Commercial ou Client (si Add Products en self-service).
- **Prérequis** : Abonnement déjà facturé au moins une fois.
- **Étapes** : Ouvrir l’abonnement → Upsell → Ajouter des produits (et éventuellement remise) → Envoyer par email → Client approuve → Confirmer.
- **Résultat** : Lignes ajoutées à l’abonnement ; prorata sur les services pour la période en cours.

### 3.4 Résiliation par l’admin

- **Acteur** : Commercial / Admin.
- **Étapes** : Ouvrir l’abonnement In Progress → Close → Saisir ou choisir Close Reason → Submit.
- **Résultat** : Statut Churned, motif dans le chatter.

### 3.5 Résiliation par le client (self-service)

- **Acteur** : Client (portail).
- **Prérequis** : Plan avec Closable activé.
- **Étapes** : Portail → Ouvrir l’abonnement → Close Subscription → Choisir une raison dans la liste → Submit.
- **Résultat** : Commande marquée Closed ; raison visible en backend.

### 3.6 Paiement automatique et échec

- **Acteur** : Système (cron) + Client (moyen de paiement enregistré).
- **Étapes** : À la date de renouvellement, génération facture + tentative prélèvement ; si échec → Payment Failure + Contract in exception.
- **Résolution** : Admin/Support vérifie (chatter, compte bancaire) ; si paiement reçu → créer/facturer manuellement si besoin, décocher Contract in exception (mode dev) ; si pas payé → relance client, puis éventuellement Close.

---

## 4. Points de Friction Identifiés

- **Prorata** : Appliqué uniquement aux services ; message d’avertissement possible pour Consumable/Storable sans prorata effectif → risque de confusion.
- **Contract in exception** : Décocher nécessite le **mode développeur** → peu accessible pour un utilisateur métier.
- **Raisons de clôture client** : Uniquement liste prédéfinie ; pas de champ libre → moins flexible pour feedback.
- **Alignement période** : Date de début personnalisée (ex. 5 de chaque mois) non disponible par défaut (développement sur mesure).
- **Période en jours** : Interdite pour les abonnements → pas de facturation hebdo courte « type jour » sans contournement.
- **Multi-société** : Plan lié à une société ; à bien configurer pour chaque entité.

---

## 5. Recommandations pour Miyukini

- **Opérateur Subscription** : Unifier création, renouvellement, upsell, clôture avec des intents explicites (RenewIntent, UpsellIntent, CloseIntent) et gouvernance (StrongFather, KindMother).
- **Portail** : Exposer Renew, Add Products, Close avec les mêmes règles (Mandats, raisons prédéfinies) ; éviter le mode développeur pour « exception » en proposant un flux métier dédié (ex. « Résoudre échec paiement » avec checklist).
- **Prorata** : Documenter clairement par type de produit (Service vs autres) et l’afficher dans l’UI (tooltip, message selon type).
- **Plans récurrents** : Modèle RecurringPlan riche (période, clôture auto, self-service, pricing, template email) avec validation des contraintes (pas de Days pour abonnement).
- **Rapports** : Prévoir équivalents MRR, historiques, résiliations et Payment Failure pour le pilotage et le support.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
