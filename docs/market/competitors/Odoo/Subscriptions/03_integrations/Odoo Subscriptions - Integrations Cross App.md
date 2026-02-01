# Odoo Subscriptions — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Subscriptions** (Abonnements) d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 19.0 (Subscriptions, Sales, Invoicing, eCommerce, Payment, CRM, Helpdesk)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres modules Odoo (sale, account, website_sale, payment, crm, helpdesk)
- Flux de données inter-apps
- Mécanismes d'intégration (modèle sale.order étendu, facturation, paiements, tokenisation)
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules requis (conceptuels)

**Dépendances explicites typiques :**
- **base** : Modèles de base, res.company, res.users
- **sale** : sale.order, sale.order.line (devis / commandes ; abonnement = commande avec plan récurrent)
- **account** : account.move, account.payment, facturation, enregistrement des paiements
- **product** : product.product, product.template (produits abonnement, recurring prices)
- **mail** : Chatter, notifications, activités sur les commandes
- **payment** : Prestataires de paiement, tokenisation (Stripe, Adyen, Authorize.net, Razorpay, Flutterwave, Xendit)

**Optionnel selon déploiement :**
- **website_sale** : eCommerce ; création / confirmation automatique des devis abonnement depuis le site
- **portal** : Portail client (signature, paiement, gestion self-service)
- **crm** : Suivi opportunités, pipeline (si abonnements gérés en opportunités)
- **helpdesk** : Triage support pour abonnés

### 1.2 Flux de données

```
Product (recurring plan, recurring prices) ──► sale.order (plan, lignes abonnement)
Sale (quotation, confirmation)              ──► Subscriptions (abonnement = order + plan)
Account (invoice, payment)                  ◄── Subscriptions (factures, renouvellements, prorata)
Payment (tokenization)                      ◄── Subscriptions (paiements récurrents)
Website_sale                                ──► Subscriptions (devis auto créés/confirmés)
Portal                                       ──► Subscriptions (Renew, Upsell, Close self-service)
CRM / Helpdesk                               ◄── Subscriptions (lien client / abonnement)
```

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Sales (sale)

**Flux :**
- **Modèle commun** : L’abonnement repose sur `sale.order` ; un champ (ou relation) « plan récurrent » et des lignes produits « abonnement » font qu’une commande est traitée comme abonnement.
- **Création** : Depuis Sales ou Subscriptions, création d’un devis avec Recurring Plan + lignes produits abonnement.
- **Workflow** : Même cycle Draft → Sent → Sale (Confirm) ; spécificité : après confirmation, statuts d’abonnement (In Progress, Renewal Quotation, Churned, Payment Failure, Closed).
- **Actions** : Renew (nouveau devis lié), Upsell (devis upsell lié), Close (wizard Close Reason).
- **Templates, Pricelist, Payment Terms** : Réutilisés depuis Sales (Quotation Template, Expiration, Pricelist, Payment Terms).
- **Signature / Paiement en ligne** : Options « Online signature » et « Online payment » (Sales) utilisées pour la confirmation des devis abonnement.

**Modèles consommés / étendus :**
- sale.order (champs / états abonnement : plan récurrent, next invoice date, subscription status, close reason, contract in exception)
- sale.order.line (produits, quantités, prix, prorata pour services)

**Recommandations Miyukini :**
- Opérateur Subscription réutilise ou s’appuie sur l’Opérateur Sales (Miyukini Sales / MiyuStore) pour devis et commandes ; différenciation par « type » abonnement et plan récurrent.
- Contrat d’équipe entre SubscriptionService et SalesService pour les flux création / confirmation / renouvellement.

### 2.2 Intégration avec Invoicing (account)

**Flux :**
- **Facturation** : Génération automatique (planned actions) ou manuelle des factures selon le plan récurrent et la date de prochaine facture.
- **Prorata** : Pour les services, montants proratisés sur la période en cours (upsell, renouvellement en milieu de période).
- **Paiements** : Enregistrement des paiements sur les factures (manuels ou automatiques si tokenisation).
- **Template email** : Envoi des factures via le modèle configuré dans le plan (ex. Invoice: Sending).
- **Contract in exception** : En cas d’échec de paiement automatique, les actions planifiées (génération factures / renouvellements) sont bloquées jusqu’à résolution manuelle.

**Modèles consommés / créés :**
- account.move (factures client liées aux commandes abonnement)
- account.payment (paiements enregistrés)
- Modèle d’email (mail.template) pour factures abonnement

**Recommandations Miyukini :**
- KindMother (WriteIntent) pour les factures et paiements ; intégration MiyuInvoice / MiyuBilling pour facturation récurrente et prorata.
- Ever Buddy pour la planification (prochaine date de facture, génération des factures) ; blocage des actions planifiées en cas d’« exception » (équivalent Contract in exception).

### 2.3 Intégration avec Payment (paiements récurrents)

**Flux :**
- **Tokenisation** : Prestataires supportant la tokenisation (Stripe, Adyen, Authorize.net, Razorpay, Flutterwave, Xendit) permettent d’enregistrer le moyen de paiement du client (checkout ou portail).
- **Paiements automatiques** : À chaque échéance (renouvellement), tentative de prélèvement sur le moyen enregistré.
- **Échec** : En cas d’échec, la commande est marquée Payment Failure et Contract in exception ; pas de nouvelle tentative automatique tant que l’exception n’est pas levée (vérification manuelle, création facture si besoin, décocher Contract in exception).

**Modèles / APIs :**
- payment.provider, payment.method, tokenisation (sauvegarde moyen de paiement)
- Appels aux APIs des prestataires pour prélèvement récurrent

**Recommandations Miyukini :**
- Intégration MiyuBilling / prestataires avec support tokenisation ; Mandat de Permission pour lier « renouvellement » à « prélèvement autorisé ».
- WorrySentinel pour le niveau de sécurité des données de paiement ; traçabilité des échecs (Contract in exception) sans exposer les détails techniques en UI standard.

### 2.4 Intégration avec Website / eCommerce (website_sale)

**Flux :**
- **Produits abonnement publiés** : Sur le site, les produits avec option Subscriptions (et Sales) sont achetables comme abonnements.
- **Création automatique** : À l’achat d’un produit abonnement en ligne, création et confirmation automatiques du devis abonnement en backend.
- **Checkout** : Même tunnel que les ventes classiques ; si tokenisation activée, option de sauvegarde du moyen de paiement pour les renouvellements.

**Modèles / mécanismes :**
- product.template (published, subscription flag)
- sale.order créé depuis le tunnel eCommerce avec plan récurrent et lignes abonnement
- Lien website → sale (ordre, transaction)

**Recommandations Miyukini :**
- Équivalent « MiyuStore + Subscription » : produits abonnement exposés sur le canal web avec création automatique de l’entité abonnement (WriteIntent) et gouvernance (StrongFather) pour confirmation.

### 2.5 Intégration avec Portal

**Flux :**
- **Accès** : Le client voit ses devis et commandes abonnement dans le portail.
- **Signature / Paiement** : Signature en ligne et paiement en ligne pour confirmer un devis (si options activées).
- **Self-service** : Si le plan l’autorise : Renew (créer un devis de renouvellement), Add Products (upsell), Close Subscription (avec liste de raisons prédéfinies).
- **Tokenisation** : Saisie ou confirmation du moyen de paiement dans le portail pour les renouvellements automatiques.

**Recommandations Miyukini :**
- Opérateur d’interface « Subscription Portal » ou extension du portail existant ; actions Renew, Upsell, Close exposées via BondingBrother avec Mandats limités (self-service = Mandat avec périmètre défini par le plan).

### 2.6 Intégration avec CRM et Helpdesk

**Flux :**
- **CRM** : Opportunités pouvant mener à un abonnement ; conversion en devis / commande abonnement.
- **Helpdesk** : Lien client / abonnement pour le triage (avantage abonné, statut, prochaine facture) ; cas « Payment Failure » ou « Contract in exception » pour le support.

**Recommandations Miyukini :**
- Lien explicite entre abonnement (SubscriptionOrder) et contact / opportunité (MiyuContacts, CRM si existant) ; exposition des champs utiles (statut, next invoice date, exception) pour le support (MiyuForum / Helpdesk).

---

## 3. Synthèse des Dépendances

| Module / App   | Rôle dans Subscriptions |
|----------------|--------------------------|
| **sale**       | Modèle de commande, devis, confirmation, signature / paiement en ligne |
| **account**    | Factures, paiements, template email facture, prorata |
| **product**    | Produits abonnement, recurring prices, variantes |
| **payment**    | Tokenisation, paiements récurrents |
| **website_sale** | eCommerce, création auto des devis abonnement |
| **portal**     | Portail client, self-service Renew / Upsell / Close |
| **mail**      | Chatter, notifications, activités |
| **crm**       | Opportunités, pipeline (optionnel) |
| **helpdesk**  | Triage support abonnés (optionnel) |

---

## 4. Recommandations Miyukini (récapitulatif)

- **SubscriptionService** en Équipe d’Opérateurs consommant : Miyukini Sales (ou MiyuStore), MiyuInvoice / MiyuBilling, MiyuNotify, MiyuContacts ; exposant Renew, Upsell, Close, génération factures planifiées.
- **Contrats d’équipe** : Avec SalesService (création / confirmation commande), InvoiceService (facturation, prorata), BillingService (tokenisation, prélèvement).
- **Mandats** : Self-service (Renew, Upsell, Close) limités par le plan récurrent (Closable, Add Products, Renew, Optional Plans) et par le périmètre client (ses propres abonnements).
- **Ever Buddy** : Planification des renouvellements et factures ; gestion de l’état « exception » (équivalent Contract in exception) pour bloquer les actions planifiées jusqu’à résolution.
- **WorrySentinel** : Niveau de sécurité sur les données abonnement et paiement ; audit des échecs de paiement sans exposition technique inutile.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
