# Odoo Subscriptions — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Subscriptions** (Abonnements) d'Odoo (version 19.0), à partir de la documentation officielle et du comportement décrit. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 (Subscriptions, Renewals, Upselling, Closing, Automatic Payments, eCommerce)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles conceptuels (Commande d'abonnement, Plan récurrent, Produit abonnement)
- Règles métier et contraintes (plans récurrents, facturation, clôture automatique)
- Workflows (création, renouvellement, upsell, résiliation)
- Calculs (prorata, alignement période, prix récurrents)
- Génération automatique de devis et factures
- Paiements récurrents (tokenisation)
- Intégration Sales, Invoicing, eCommerce, Helpdesk, CRM

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Commande de vente abonnement (sale.order avec plan récurrent)

**Rôle :** Une commande de vente avec un **plan récurrent** (recurring plan) défini devient une **abonnement**. Le même modèle `sale.order` est utilisé ; la présence d'un plan récurrent et de lignes produits « abonnement » détermine le statut d'abonnement.

**États / statuts d'abonnement typiques :**
- **Quotation** : Devis non confirmé
- **Sales Order** : Commande confirmée (abonnement actif)
- **In Progress** : Abonnement en cours (facturé, payé)
- **Renewal Quotation** : Devis de renouvellement
- **Churned** : Abonnement résilié (avec motif de clôture)
- **Payment Failure** : Échec de paiement automatique (Contract in exception)
- **Closed** : Fermé (côté portail client)

**Champs conceptuels clés (abonnement) :**
- **Plan récurrent** : Référence au plan (mensuel, 6 mois, annuel, etc.)
- **Date de début / prochaine facture** : Calculées selon le plan et l’alignement période
- **Clôture automatique** : Nombre de jours sans paiement avant clôture automatique
- **Raisons de clôture** : Motif de résiliation (admin ou client)
- **Contract in exception** : Blocage des actions planifiées (ex. après échec paiement)
- **Historique des commandes** : Liens vers renouvellements et upsells (Sales History)

### 1.2 Plan récurrent (Recurring Plan)

**Rôle :** Définit la **fenêtre temporelle** pendant laquelle l’abonnement est actif avant renouvellement, la fréquence de facturation et les options self-service.

**Champs principaux (conceptuels) :**

#### Détails (DETAILS)
- **Billing Period** : Fréquence de facturation (Weeks, Months, Years + valeur). **Restriction :** l’unité « Days » n’est pas autorisée pour les abonnements (réservée à la location).
- **Automatic Closing** : Nombre de jours sans paiement avant clôture automatique de l’abonnement.
- **Align to Period Start** : Si activé, les nouvelles et futures périodes facturent au premier jour de la période suivante ; achat en cours de période → facture au prorata pour la période courante, puis plein tarif à partir du premier jour de la période suivante.
- **Company** : Société (multi-société) pour laquelle le plan est disponible.
- **Invoice Email Template** : Modèle d’email pour l’envoi des factures (ex. « Invoice: Sending »).

#### Self-Service
- **Closable** : Le client peut résilier lui-même depuis le portail.
- **Add Products** : Le client peut ajouter des produits ou modifier les quantités (génération de devis d’upsell).
- **Renew** : Le client peut créer manuellement un devis de renouvellement.
- **Optional Plans** : Plans alternatifs proposés au client (changement de plan → nouveau devis).

#### Pricing (onglet)
- Lignes **Produit / Variante**, **Pricelist**, **Recurring Price** : règles de prix spécifiques au plan (priment sur le prix par défaut du produit).

**Règles métier :**
- Un produit abonnement doit être associé à un plan récurrent sur la commande.
- La période en « Days » est interdite pour les abonnements.

### 1.3 Produit abonnement (product.product / product.template)

**Rôle :** Produit marqué comme **Subscriptions** (et généralement **Sales**) pour être proposé en abonnement.

**Configuration typique :**
- **Product type** : Souvent **Service** (Consumable / Storable possibles selon cas ; politique de facturation impacte les erreurs pour les produits physiques).
- **Invoicing policy** : Détermine quand le client est facturé (ordered/delivered quantities).
- **Unit of Measure** : Souvent **Units**.
- **Sales Price** : Prix récurrent par période.
- **Recurring Prices** (onglet) : Règles de prix par plan (période plus longue = tarif réduit possible).
- **Attributes & Variants** : Optionnel (ex. box, cours eLearning avec variantes).

**Règle importante :** Pour un produit **physique** en abonnement, la politique de facturation doit être **Ordered quantities** sous peine d’erreurs à la facturation.

### 1.4 Ligne de commande (sale.order.line) — abonnement

- **Produit** : Produit abonnement (recurring plan hérité ou défini au niveau commande).
- **Quantité, prix, taxes** : Classiques ; prorata appliqué pour les **services** en cas d’upsell / renouvellement en cours de période (pas pour Consumable/Storable malgré le message éventuel).
- **Recurring plan** : Lié à la commande (une commande = un plan pour l’abonnement).

---

## 2. Règles Métier et Contraintes

### 2.1 Création d’un abonnement

1. **Devis avec produit abonnement + plan récurrent** : En confirmant le devis, la commande devient un abonnement (Sales Order, puis In Progress après facturation/paiement).
2. **eCommerce** : Les produits marqués Subscriptions et vendus sur le site créent et confirment automatiquement les devis d’abonnement en backend.
3. **Contrainte** : Un plan récurrent doit être sélectionné ; la période ne peut pas être en « Days ».

### 2.2 Renouvellement (Renewal)

**Prérequis :**
- Devis avec produit abonnement confirmé.
- Plan récurrent configuré.
- Paiement initial facturé et enregistré.

**Processus :**
- Clic sur **Renew** → nouveau devis « Renewal Quotation » avec dates (start, next invoice) en chatter.
- Confirmation → facturation → enregistrement paiement ; le bouton **Sales History** affiche l’historique des commandes liées à cet abonnement.

### 2.3 Upsell

**Prérequis :**
- Abonnement confirmé et **déjà facturé**.
- Option **Add Products** (self-service) ou action manuelle **Upsell**.

**Processus :**
- Clic sur **Upsell** → nouveau devis avec bandeau « Upsell » ; lignes initiales reprises, produits récurrents affichés avec avertissement prorata.
- Prorata appliqué **uniquement aux produits de type Service**.
- Ajout de produits, envoi au client, confirmation → les produits upsell sont intégrés à l’abonnement ; les prix sont proratés sur le reste de la période en cours.

### 2.4 Résiliation (Closing)

**Configuration :**
- Dans le plan récurrent, option **Closable** (self-service) pour autoriser la clôture par le client.

**Côté administrateur :**
- Bouton **Close** sur la commande abonnement (In Progress) → fenêtre **Close Reason** (saisie ou liste de raisons).
- Après validation : statut **Churned** + motif en chatter.

**Côté client (portail) :**
- Bouton **Close Subscription** → choix d’une raison prédéfinie (Configuration → Close Reasons) → soumission → commande marquée **Closed**.

**Règle :** Le client ne peut pas saisir une raison personnalisée ; uniquement les raisons configurées.

### 2.5 Facturation et paiements automatiques

- **Facturation** : Génération automatique de devis et factures selon le plan (période, alignement).
- **Paiements automatiques** : Nécessitent un **provider avec tokenisation** (Stripe, Adyen, Authorize.net, Razorpay, Flutterwave, Xendit) + portail client ou eCommerce pour enregistrer le moyen de paiement.
- **Contract in exception** : En cas d’échec de paiement automatique, la commande est marquée **Payment Failure** et **Contract in exception** ; les actions planifiées (renouvellements, etc.) ne s’exécutent pas pour éviter double facturation. Résolution manuelle : vérifier si le paiement a eu lieu, créer/facturer si besoin, décocher « Contract in exception » (mode développeur).

### 2.6 Alignement période (Align to Period Start)

- Si activé : facturation au **premier jour** de la période suivante.
- Exemple : achat le 15 juillet, abonnement mensuel → facture 15–31 juillet au prorata ; à partir du 1er août, facture plein tarif chaque 1er du mois.
- Dates personnalisées (ex. 5 de chaque mois) nécessitent du développement spécifique (non fourni par défaut).

### 2.7 Clôture automatique (Automatic Closing)

- Valeur en **jours** sans paiement après la date de renouvellement → clôture automatique de l’abonnement.
- Exemple : renouvellement le 1er du mois, Automatic Closing = 15 → si pas de paiement, clôture le 16.

---

## 3. Workflows

### 3.1 Cycle de vie abonnement (schéma)

```
[Création] → Quotation (devis)
     → Confirm → Sales Order / In Progress (facturé + payé)
     → Renew → Renewal Quotation → Confirm → nouvel ordre lié (Sales History)
     → Upsell → Upsell Quotation → Confirm → lignes ajoutées à l’abonnement
     → Close (admin ou client) → Churned / Closed
     → Payment Failure + Contract in exception → résolution manuelle
```

### 3.2 Actions planifiées (scheduled actions)

- Génération automatique des **devis de renouvellement** et **factures** selon le plan et la date de prochaine facture.
- Bloquées si **Contract in exception** est coché.

### 3.3 Rapports et historique

- **Sales History** : Liste des commandes liées à un même abonnement (initial, renouvellements, upsells) avec statut d’abonnement par ordre.
- **Rapports** : MRR, abonnements actifs, résiliations, etc. (décrits dans la doc Rapports Subscriptions).

---

## 4. Calculs

### 4.1 Prorata

- **Upsell / renouvellement en cours de période** : pour les produits de type **Service**, le montant facturé est proratisé sur le temps restant dans la période.
- **Consumable / Storable** : pas de prorata appliqué (même si un message d’avertissement peut s’afficher).

### 4.2 Prix récurrents

- Prix par défaut sur le produit ; **surcharge** possible dans le plan récurrent (onglet Pricing) et dans l’onglet Recurring Prices du produit (par plan / période).
- Les règles du plan priment sur le prix par défaut du produit.

### 4.3 Date de prochaine facture

- Calculée à partir de la **Billing Period** et de l’**Align to Period Start** (premier jour de la période suivante si alignement activé).

---

## 5. Intégrations métier (résumé)

- **Sales** : Devis / commandes, signature et paiement en ligne, templates, pricelists, conditions de paiement.
- **Invoicing** : Factures, écritures, enregistrement des paiements, email de facture (template).
- **eCommerce** : Produits abonnement publiés → création/confirmation automatique des devis ; tunnel de paiement pour tokenisation.
- **CRM** : Suivi opportunités, pipeline (si utilisé pour les abonnements).
- **Helpdesk** : Triage support pour abonnés (avantages abonnement).
- **Payment providers** : Tokenisation pour paiements récurrents (Stripe, Adyen, etc.).

---

## 6. Synthèse pour Miyukini

**Concepts à traduire en Opérateurs / Kinds :**
- **RecurringPlan** : Plan récurrent (période, clôture auto, self-service, pricing, template email).
- **SubscriptionOrder** : Commande abonnement (sale.order + plan + statut abonnement, next invoice date, close reason, exception).
- **SubscriptionLine** : Ligne récurrente (produit, quantité, prix, prorata si service).
- **Renewal / Upsell / Close** : Intents et workflows gouvernés (StrongFather, KindMother, Ever Buddy pour cycle de vie).
- **Scheduled actions** : Génération renouvellements et factures (Ever Buddy / Jobs).
- **Tokenisation paiement** : Intégration MiyuBilling / prestataires pour paiements récurrents.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
