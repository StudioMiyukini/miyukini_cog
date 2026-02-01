# Odoo Subscriptions — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Subscriptions** (Abonnements) d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l’équivalent Subscriptions
- Contrats d’équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **SubscriptionOperator** | Gestion des abonnements (création, renouvellement, upsell, clôture) | Opérateur de Service |
| **RecurringPlanOperator** | Plans récurrents (configuration, pricing, self-service) | Opérateur de Domaine |
| **SubscriptionBillingOperator** | Facturation récurrente et prorata (génération factures, planification) | Opérateur de Service |
| **SubscriptionPaymentOperator** | Paiements récurrents (tokenisation, prélèvement, exception) | Opérateur de Service |
| **SubscriptionUI** | Interface utilisateur Subscriptions (back-office et portail) | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : SubscriptionService

**Définition :**
> **SubscriptionService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service d'abonnements (plans, création, renouvellement, upsell, résiliation, facturation et paiements récurrents).**

**Composition :**
- SubscriptionOperator (niveau sécurité 2)
- RecurringPlanOperator (niveau sécurité 1–2)
- SubscriptionBillingOperator (niveau sécurité 3)
- SubscriptionPaymentOperator (niveau sécurité 3)
- SubscriptionUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 SubscriptionOperator

**Rôle :** Gestion des abonnements (création à partir de devis, renouvellement, upsell, clôture).

**Capacités :**
- Création d’abonnement à partir d’un devis (Confirm → abonnement In Progress)
- Renouvellement (Renew) : création d’un devis de renouvellement lié
- Upsell (Upsell) : création d’un devis d’upsell lié ; intégration des lignes à l’abonnement après confirmation
- Clôture (Close) : résiliation avec motif (admin ou client) ; passage Churned / Closed
- Gestion du statut d’abonnement (Quotation, In Progress, Renewal Quotation, Churned, Payment Failure, Closed)
- Exposition de Sales History (historique des commandes liées)
- Gestion de l’état « exception » (Contract in exception) : blocage des actions planifiées jusqu’à résolution

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de confirmer, renouveler, upsell, clôturer
- **KindMother** : Persistance des commandes abonnement et des liens (Renewal, Upsell)
- **Master Butler** : Permissions création / renouvellement / upsell / clôture (admin vs client self-service)
- **WorrySentinel** : Niveau sécurité données abonnement (client, montants)
- **Ever Buddy** : Cycle de vie abonnement (états, planification renouvellements)

**Contrat d'équipe :**
- Consomme : RecurringPlanOperator (plans), Miyukini Sales / MiyuStore (devis / commandes), MiyuInvoice (factures), MiyuNotify (notifications), MiyuContacts (client)
- Expose : `subscription.create`, `subscription.renew`, `subscription.upsell`, `subscription.close`, `subscription.resolve_exception`

**Mandat de Permission requis :**
- Création / confirmation : Mandat avec StrongFather (décision confirm) + KindMother (WriteIntent order)
- Renouvellement : Mandat avec StrongFather (renew) + KindMother (WriteIntent renewal quotation)
- Upsell : Mandat avec StrongFather (upsell) + KindMother (WriteIntent upsell quotation)
- Clôture : Mandat avec StrongFather (close) + KindMother (WriteIntent status + close reason)
- Résolution exception : Mandat avec StrongFather (resolve_exception) + KindMother (WriteIntent contract_in_exception)

### 2.2 RecurringPlanOperator

**Rôle :** Plans récurrents (période, clôture automatique, self-service, pricing, template email).

**Capacités :**
- Création / modification de plans récurrents (Billing Period, Automatic Closing, Align to Period Start)
- Configuration self-service (Closable, Add Products, Renew, Optional Plans)
- Pricing par plan (lignes Produit / Variante, Pricelist, Recurring Price)
- Template email facture (Invoice Email Template)
- Raisons de clôture (Close Reasons) pour admin et client
- Contrainte : Billing Period ne peut pas être en « Days » pour les abonnements
- Exposition des smart buttons (nombre d’abonnements, Subscription Items)

**Niveau de sécurité :** 1–2 (Standard à Sensitive selon données pricing)

**Gouvernance :**
- **StrongFather** : Décision de créer / modifier un plan (droits configuration)
- **KindMother** : Persistance des plans et des règles de prix
- **Master Butler** : Permissions configuration plans
- **Ever Buddy** : Compatibilité des plans avec les abonnements existants (dépréciation / évolution)

**Contrat d'équipe :**
- Consomme : MiyuContacts (multi-société si Company sur le plan), MiyuNotify (template email)
- Expose : `recurring_plan.create`, `recurring_plan.update`, `recurring_plan.read`, `close_reasons.list`

**Mandat de Permission requis :**
- Création / modification plan : Mandat avec StrongFather (config) + KindMother (WriteIntent plan)
- Lecture plans / raisons : Mandat avec Master Butler (recurring_plan.read, close_reasons.list)

### 2.3 SubscriptionBillingOperator

**Rôle :** Facturation récurrente et prorata (génération des factures selon le plan, prorata pour les services).

**Capacités :**
- Génération automatique (planifiée) ou manuelle des factures selon la date de prochaine facture et le plan récurrent
- Prorata pour les produits de type Service (upsell / renouvellement en cours de période)
- Envoi des factures par email (template configuré dans le plan)
- Mise à jour de la date de prochaine facture après facturation
- Blocage des actions planifiées si « Contract in exception » (équivalent)

**Niveau de sécurité :** 3 (Critical)

**Gouvernance :**
- **StrongFather** : Décision de facturer (planification ou manuel)
- **KindMother** : Persistance des factures (WriteIntent account.move)
- **Master Butler** : Permissions facturation récurrente
- **Ever Buddy** : Planification des échéances (next invoice date) et compatibilité des périodes
- **WorrySentinel** : Niveau sécurité données factures

**Contrat d'équipe :**
- Consomme : SubscriptionOperator (abonnements, next invoice date), RecurringPlanOperator (plan, template), MiyuInvoice (factures, écritures)
- Expose : `subscription_billing.generate_invoice`, `subscription_billing.schedule`, `subscription_billing.prorata_compute`

**Mandat de Permission requis :**
- Génération facture : Mandat avec StrongFather (bill) + KindMother (WriteIntent invoice) + MiyuInvoice (création move)
- Planification : Mandat avec Ever Buddy (schedule) + SubscriptionBillingOperator (generate_invoice)

### 2.4 SubscriptionPaymentOperator

**Rôle :** Paiements récurrents (tokenisation, prélèvement automatique, gestion des échecs et « exception »).

**Capacités :**
- Enregistrement du moyen de paiement (tokenisation) au checkout ou dans le portail
- Prélèvement automatique à l’échéance (renouvellement)
- Gestion des échecs : marquage Payment Failure + Contract in exception ; blocage des actions planifiées
- Résolution manuelle : vérification paiement, création facture si besoin, levée de l’exception (équivalent décocher Contract in exception)
- Intégration avec prestataires supportant la tokenisation (Stripe, Adyen, etc.)

**Niveau de sécurité :** 3 (Critical)

**Gouvernance :**
- **StrongFather** : Décision d’autoriser le prélèvement récurrent
- **KindMother** : Persistance des tokens et des paiements (sans stocker les données sensibles brutes)
- **Master Butler** : Permissions tokenisation et prélèvement
- **WorrySentinel** : Niveau sécurité maximal pour les données de paiement ; audit des échecs
- **TAMR** : Point d’intervention humaine pour la résolution des échecs (résolution exception)

**Contrat d'équipe :**
- Consomme : SubscriptionOperator (abonnements, exception), MiyuBilling / prestataires (tokenisation, prélèvement), MiyuInvoice (paiements enregistrés)
- Expose : `subscription_payment.tokenize`, `subscription_payment.charge`, `subscription_payment.resolve_failure`

**Mandat de Permission requis :**
- Tokenisation : Mandat avec StrongFather (save_payment) + WorrySentinel (niveau sécurité) + MiyuBilling (token)
- Prélèvement : Mandat avec StrongFather (charge) + KindMother (WriteIntent payment) + MiyuBilling (charge)
- Résolution échec : Mandat avec StrongFather (resolve_failure) + KindMother (WriteIntent exception) + TAMR (intervention humaine)

### 2.5 SubscriptionUI

**Rôle :** Interface utilisateur Subscriptions (back-office et portail client).

**Capacités :**
- Back-office : listes et formulaires (Plans récurrents, Produits abonnement, Devis / Commandes abonnement) ; boutons Renew, Upsell, Close ; Sales History ; wizards Close Reason ; gestion Contract in exception (résolution métier sans mode développeur)
- Portail : liste des abonnements du client ; Renew, Add Products, Close Subscription (self-service) ; signature et paiement en ligne ; saisie moyen de paiement pour tokenisation
- eCommerce : produits abonnement publiés ; tunnel checkout avec option tokenisation ; création automatique des devis abonnement en backend

**Niveau de sécurité :** 1 (Standard pour l’affichage ; élévation selon action)

**Gouvernance :**
- **BondingBrother** : Médiation entre l’utilisateur et les Opérateurs (SubscriptionOperator, RecurringPlanOperator, SubscriptionBillingOperator, SubscriptionPaymentOperator)
- **Master Butler** : Permissions d’affichage et d’action (admin vs client vs self-service selon plan)
- **WorrySentinel** : Niveau sécurité des écrans (données sensibles masquées ou restreintes)

**Contrat d'équipe :**
- Consomme : Tous les Opérateurs SubscriptionService ; Miyukini Sales / MiyuStore (devis / commandes) ; MiyuInvoice (factures) ; MiyuNotify (notifications)
- Expose : écrans et actions UI (pas d’API directe ; passage par BondingBrother)

**Mandat de Permission requis :**
- Toute action UI : Mandat couvrant l’action sous-jacente (create, renew, upsell, close, resolve_exception, tokenize, etc.) selon le rôle et le plan (self-service).

---

## 3. Contrat d'Équipe SubscriptionService

**Opérateurs membres :**
- SubscriptionOperator, RecurringPlanOperator, SubscriptionBillingOperator, SubscriptionPaymentOperator, SubscriptionUI

**Flux autorisés (résumé) :**
- SubscriptionUI → BondingBrother → SubscriptionOperator (create, renew, upsell, close, resolve_exception)
- SubscriptionUI → BondingBrother → RecurringPlanOperator (config plans, close reasons)
- SubscriptionUI → BondingBrother → SubscriptionBillingOperator (generate invoice, schedule)
- SubscriptionUI → BondingBrother → SubscriptionPaymentOperator (tokenize, charge, resolve_failure)
- SubscriptionOperator → RecurringPlanOperator (read plan, close reasons)
- SubscriptionOperator → Miyukini Sales / MiyuStore (devis / commandes)
- SubscriptionBillingOperator → SubscriptionOperator (read subscription, next invoice date)
- SubscriptionBillingOperator → MiyuInvoice (factures)
- SubscriptionPaymentOperator → SubscriptionOperator (exception, status)
- SubscriptionPaymentOperator → MiyuBilling (token, charge)

**Direction des flux :** Unidirectionnelle selon les capacités exposées (Consomme / Expose).

**Types d'échanges :** WriteIntent (création / mise à jour abonnement, facture, paiement, exception), lectures (plans, raisons, historique).

**Niveau de validation requis :** StrongFather pour toute décision métier (confirm, renew, upsell, close, resolve_exception, bill, charge) ; KindMother pour toute persistance.

---

## 4. Mandats de Permission (résumé)

| Action | Mandat (résumé) |
|--------|------------------|
| Création / confirmation abonnement | StrongFather + KindMother (order) + Master Butler (create) |
| Renouvellement | StrongFather (renew) + KindMother (renewal quotation) |
| Upsell | StrongFather (upsell) + KindMother (upsell quotation) |
| Clôture | StrongFather (close) + KindMother (status + close reason) |
| Résolution exception | StrongFather (resolve) + KindMother (exception) |
| Configuration plan | StrongFather (config) + KindMother (plan) |
| Génération facture | StrongFather (bill) + KindMother (invoice) + MiyuInvoice |
| Tokenisation | StrongFather (save_payment) + WorrySentinel + MiyuBilling |
| Prélèvement | StrongFather (charge) + KindMother (payment) + MiyuBilling |
| Résolution échec paiement | StrongFather (resolve_failure) + KindMother (exception) + TAMR |

---

## 5. Correspondance Miyukini

**Service Miyukini cible :**
- **MiyuPM** (Miyu Subscriptions / Abonnements) ou **Miyukini Subscriptions** — à aligner avec la nomenclature existante (Miyu* pour modules, Miyukini* pour services stratégiques).
- Document Fondateur et structure de référence à créer dans `docs/services/MiyukiniSubscriptions/` ou `docs/modules/miyupm/` selon le choix d’architecture.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
