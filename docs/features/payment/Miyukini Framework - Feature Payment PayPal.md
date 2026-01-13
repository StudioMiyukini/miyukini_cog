# Miyukini Framework - Feature Payment PayPal

## Contexte

Le Miyukini Framework doit permettre d'**intégrer PayPal** pour les paiements en ligne, supportant à la fois les **achats uniques** (Orders API) et les **abonnements** (Subscriptions API). Cette fonctionnalité doit être **modulaire**, **configurable par module**, et **sécurisée** (server-side uniquement).

Cette spécification s'inspire :
- Des bonnes pratiques PayPal (OAuth2, webhooks, idempotence)
- Des besoins des modules Commerce et Booking (paiement de factures, réservations)
- De l'architecture existante (configuration provider + module, outbox pattern)

## Portée / Scope

- **Capability** : Payment PayPal (achats uniques + abonnements)
- **Niveaux** :
  - **Niveau 1** : Orders API (achats uniques, capture, remboursements)
  - **Niveau 2** : Subscriptions API (abonnements récurrents, plans, facturation)
  - **Niveau 3** : Webhooks, disputes, payouts
- **Back-office** : configuration SuperAdmin (provider global) + configuration par module
- **Sécurité** : RLS + appels server-side uniquement (Edge Functions)
- **Hors scope** : autres providers (Stripe, etc.) — mais l'architecture le permet

---

## 1) Concepts (glossaire)

- **Provider** : configuration PayPal globale (sandbox/live, client_id/secret, webhook)
- **Module Config** : configuration par module (mode orders/subscriptions, intent, URLs, toggles)
- **Order** : paiement unique PayPal (création → capture → completion)
- **Subscription** : abonnement PayPal (plan → subscription → billing cycles)
- **Webhook** : notifications PayPal (payment completed, subscription updated, etc.)
- **Outbox Pattern** : table `payment_outbox` pour événements asynchrones (idempotence)
- **Edge Function** : exécution server-side pour appels PayPal (recommandé)

---

## 2) Architecture cible (capability)

### 2.1 Règle d'or (sécurité)

**Tous les appels PayPal doivent être server-side** (Edge Functions / API) :
- éviter d'exposer des secrets côté navigateur,
- centraliser le retry, la rate-limit, l'audit,
- uniformiser le comportement entre modules,
- gérer les webhooks de manière fiable.

### 2.2 Ports (interfaces) recommandés

- **`PayPalProviderPort`**
  - `createOrder(request): Promise<Order>`
  - `captureOrder(orderId): Promise<Capture>`
  - `refundPayment(captureId, amount?): Promise<Refund>`
  - `createSubscription(planId, customer): Promise<Subscription>`
  - `cancelSubscription(subscriptionId): Promise<void>`
  - `testConnection(): Promise<TestResult>`
- **`PayPalWebhookPort`**
  - `verifyWebhook(signature, payload): Promise<boolean>`
  - `processWebhook(event): Promise<void>`
- **`PaymentOutboxPort`**
  - `enqueueEvent(event): Promise<void>`
  - `processPendingEvents(): Promise<void>`

---

## 3) Data contract (tables) — état actuel

### 3.1 Configuration provider (globale)

#### `public.paypal_provider_configs` ✅ (existant)

Champs :
- `id` UUID
- `workspace_id` UUID
- `is_active` BOOLEAN
- `environment` ENUM (`sandbox`, `live`)
- `client_id` TEXT
- `client_secret` TEXT (secret, à chiffrer à terme)
- `merchant_id` TEXT (optionnel)
- `webhook_id` TEXT (ID webhook PayPal)
- `brand_name` TEXT (optionnel)
- `return_url` TEXT (optionnel, global)
- `cancel_url` TEXT (optionnel, global)
- `last_test_at`, `last_test_status`, `last_test_error`
- `updated_by` UUID
- `created_at`, `updated_at`

**RLS** : super_admin uniquement (lecture/écriture)

### 3.2 Configuration module

#### `public.module_paypal_configs` ✅ (existant)

Champs :
- `id` UUID
- `workspace_id` UUID
- `module_id` TEXT (ex: `commerce-devis`, `booking`)
- `is_enabled` BOOLEAN
- `mode` ENUM (`orders`, `subscriptions`)
- `intent` ENUM (`CAPTURE`, `AUTHORIZE`)
- `currency` TEXT (ex: `EUR`)
- `success_url` TEXT (optionnel, override global)
- `cancel_url` TEXT (optionnel, override global)
- `enable_refunds` BOOLEAN
- `enable_webhooks` BOOLEAN
- `enable_disputes` BOOLEAN
- `enable_payouts` BOOLEAN
- `settings_json` JSONB (extensible)
- `updated_by` UUID
- `created_at`, `updated_at`

**RLS** : super_admin uniquement (lecture/écriture)

### 3.3 Outbox (événements asynchrones)

#### `public.payment_outbox` ✅ (existant)

Champs :
- `id` UUID
- `provider` TEXT (ex: `paypal`)
- `event_type` TEXT (ex: `payment.paypal.webhook.received`)
- `dedupe_key` TEXT UNIQUE (idempotence)
- `module_id` TEXT (optionnel)
- `payload` JSONB
- `status` TEXT (`pending`, `processing`, `success`, `error`)
- `attempt_count` INT
- `next_retry_at` TIMESTAMPTZ
- `last_error` TEXT
- `created_at`, `updated_at`

**RLS** : admin/super_admin (écriture worker via service role)

### 3.4 Tables à créer (paiements)

#### `public.paypal_orders` (à créer)

Champs :
- `id` UUID PRIMARY KEY
- `workspace_id` UUID
- `module_id` TEXT
- `paypal_order_id` TEXT UNIQUE (ID PayPal)
- `status` TEXT (`CREATED`, `SAVED`, `APPROVED`, `VOIDED`, `COMPLETED`)
- `intent` TEXT (`CAPTURE`, `AUTHORIZE`)
- `amount` NUMERIC
- `currency` TEXT
- `payer_email` TEXT (optionnel)
- `payer_name` TEXT (optionnel)
- `application_context` JSONB (return_url, cancel_url, etc.)
- `links` JSONB (approve_url, capture_url, etc.)
- `created_at`, `updated_at`

**RLS** : user voit ses propres orders, admin voit tous

#### `public.paypal_captures` (à créer)

Champs :
- `id` UUID PRIMARY KEY
- `order_id` UUID REFERENCES `paypal_orders(id)`
- `paypal_capture_id` TEXT UNIQUE (ID PayPal)
- `status` TEXT (`COMPLETED`, `DECLINED`, `PARTIALLY_REFUNDED`, `PENDING`, `REFUNDED`)
- `amount` NUMERIC
- `currency` TEXT
- `final_capture` BOOLEAN
- `seller_protection` JSONB
- `disbursement_mode` TEXT
- `created_at`, `updated_at`

**RLS** : user voit ses propres captures, admin voit tous

#### `public.paypal_refunds` (à créer)

Champs :
- `id` UUID PRIMARY KEY
- `capture_id` UUID REFERENCES `paypal_captures(id)`
- `paypal_refund_id` TEXT UNIQUE (ID PayPal)
- `status` TEXT (`CANCELLED`, `FAILED`, `PENDING`, `COMPLETED`)
- `amount` NUMERIC
- `currency` TEXT
- `reason` TEXT (optionnel)
- `created_at`, `updated_at`

**RLS** : admin uniquement

#### `public.paypal_subscriptions` (à créer)

Champs :
- `id` UUID PRIMARY KEY
- `workspace_id` UUID
- `module_id` TEXT
- `paypal_subscription_id` TEXT UNIQUE (ID PayPal)
- `plan_id` TEXT (ID plan PayPal)
- `status` TEXT (`APPROVAL_PENDING`, `APPROVED`, `ACTIVE`, `SUSPENDED`, `CANCELLED`, `EXPIRED`)
- `subscriber_id` UUID REFERENCES `profiles(id)`
- `start_time` TIMESTAMPTZ
- `billing_cycles` JSONB
- `payment_preferences` JSONB
- `application_context` JSONB
- `links` JSONB (approve_url, etc.)
- `created_at`, `updated_at`

**RLS** : user voit ses propres subscriptions, admin voit tous

#### `public.paypal_subscription_transactions` (à créer)

Champs :
- `id` UUID PRIMARY KEY
- `subscription_id` UUID REFERENCES `paypal_subscriptions(id)`
- `paypal_transaction_id` TEXT UNIQUE (ID PayPal)
- `billing_cycle` INT
- `status` TEXT (`COMPLETED`, `DECLINED`, `PENDING`)
- `amount` NUMERIC
- `currency` TEXT
- `transaction_time` TIMESTAMPTZ
- `created_at`, `updated_at`

**RLS** : user voit ses propres transactions, admin voit tous

---

## 4) PayPal APIs utilisées

### 4.1 Orders API (achats uniques)

**Endpoints** :
- `POST /v2/checkout/orders` : créer un order
- `GET /v2/checkout/orders/{id}` : récupérer un order
- `POST /v2/checkout/orders/{id}/capture` : capturer un order
- `POST /v2/payments/captures/{id}/refund` : rembourser une capture

**Workflow** :
1. Client initie paiement → Edge Function crée order PayPal
2. PayPal retourne `approve_url` → redirection client
3. Client approuve → PayPal redirige vers `return_url`
4. Edge Function capture l'order → paiement complété
5. Webhook confirme → mise à jour statut

### 4.2 Subscriptions API (abonnements)

**Endpoints** :
- `POST /v1/billing/plans` : créer un plan
- `GET /v1/billing/plans/{id}` : récupérer un plan
- `POST /v1/billing/subscriptions` : créer une subscription
- `GET /v1/billing/subscriptions/{id}` : récupérer une subscription
- `POST /v1/billing/subscriptions/{id}/cancel` : annuler une subscription
- `GET /v1/billing/subscriptions/{id}/transactions` : lister les transactions

**Workflow** :
1. Admin crée un plan PayPal (via Edge Function ou manuellement)
2. Client initie abonnement → Edge Function crée subscription
3. PayPal retourne `approve_url` → redirection client
4. Client approuve → subscription active
5. PayPal facture automatiquement selon plan
6. Webhooks notifient chaque transaction

### 4.3 Webhooks

**Endpoints** :
- `POST /v1/notifications/verify-webhook-signature` : vérifier signature
- Webhook endpoint : Edge Function dédiée

**Événements à écouter** :
- `PAYMENT.CAPTURE.COMPLETED` : capture complétée
- `PAYMENT.CAPTURE.DENIED` : capture refusée
- `PAYMENT.CAPTURE.REFUNDED` : remboursement effectué
- `BILLING.SUBSCRIPTION.CREATED` : subscription créée
- `BILLING.SUBSCRIPTION.ACTIVATED` : subscription activée
- `BILLING.SUBSCRIPTION.CANCELLED` : subscription annulée
- `BILLING.SUBSCRIPTION.PAYMENT.FAILED` : paiement échoué
- `BILLING.SUBSCRIPTION.UPDATED` : subscription mise à jour

---

## 5) Edge Functions à développer

### 5.1 `paypal-create-order` ✅ (à créer)

**Objectif** : créer un order PayPal pour un achat unique

**Input** :
```typescript
{
  module_id: string
  amount: number
  currency: string
  description?: string
  return_url?: string
  cancel_url?: string
  metadata?: Record<string, any>
}
```

**Output** :
```typescript
{
  order_id: string // PayPal order ID
  approve_url: string // URL de redirection client
  status: string
}
```

**Logique** :
1. Vérifier auth (user authentifié)
2. Charger config module (`module_paypal_configs`)
3. Charger config provider (`paypal_provider_configs`)
4. Obtenir access token PayPal (OAuth2)
5. Créer order via PayPal API
6. Enregistrer dans `paypal_orders`
7. Retourner `approve_url` pour redirection

### 5.2 `paypal-capture-order` ✅ (à créer)

**Objectif** : capturer un order PayPal après approbation client

**Input** :
```typescript
{
  order_id: string // PayPal order ID
}
```

**Output** :
```typescript
{
  capture_id: string
  status: string
  amount: number
}
```

**Logique** :
1. Vérifier auth
2. Charger order depuis `paypal_orders`
3. Vérifier que l'order est `APPROVED`
4. Obtenir access token PayPal
5. Capturer order via PayPal API
6. Enregistrer capture dans `paypal_captures`
7. Mettre à jour order (`status = COMPLETED`)
8. Émettre événement `payment.paypal.order.completed`
9. Enqueue dans `payment_outbox` pour traitement asynchrone

### 5.3 `paypal-refund` ✅ (à créer)

**Objectif** : rembourser une capture PayPal

**Input** :
```typescript
{
  capture_id: string // PayPal capture ID
  amount?: number // partiel si fourni, total sinon
  reason?: string
}
```

**Output** :
```typescript
{
  refund_id: string
  status: string
  amount: number
}
```

**Logique** :
1. Vérifier auth (admin uniquement)
2. Charger capture depuis `paypal_captures`
3. Vérifier que la capture est `COMPLETED`
4. Obtenir access token PayPal
5. Créer refund via PayPal API
6. Enregistrer refund dans `paypal_refunds`
7. Mettre à jour capture (`status = REFUNDED` ou `PARTIALLY_REFUNDED`)
8. Émettre événement `payment.paypal.refund.completed`

### 5.4 `paypal-create-subscription` ✅ (à créer)

**Objectif** : créer une subscription PayPal

**Input** :
```typescript
{
  module_id: string
  plan_id: string // PayPal plan ID
  subscriber_id: string // profiles.id
  return_url?: string
  cancel_url?: string
}
```

**Output** :
```typescript
{
  subscription_id: string // PayPal subscription ID
  approve_url: string
  status: string
}
```

**Logique** :
1. Vérifier auth
2. Charger config module
3. Charger config provider
4. Obtenir access token PayPal
5. Créer subscription via PayPal API
6. Enregistrer dans `paypal_subscriptions`
7. Retourner `approve_url`

### 5.5 `paypal-cancel-subscription` ✅ (à créer)

**Objectif** : annuler une subscription PayPal

**Input** :
```typescript
{
  subscription_id: string // PayPal subscription ID
  reason?: string
}
```

**Output** :
```typescript
{
  status: string
}
```

**Logique** :
1. Vérifier auth (user propriétaire ou admin)
2. Charger subscription depuis `paypal_subscriptions`
3. Obtenir access token PayPal
4. Annuler subscription via PayPal API
5. Mettre à jour subscription (`status = CANCELLED`)
6. Émettre événement `payment.paypal.subscription.cancelled`

### 5.6 `paypal-webhook` ✅ (à créer)

**Objectif** : recevoir et traiter les webhooks PayPal

**Input** : Webhook PayPal (headers + body)

**Logique** :
1. Vérifier signature webhook (via PayPal API)
2. Parser l'événement
3. Enqueue dans `payment_outbox` avec `dedupe_key` unique
4. Retourner 200 OK immédiatement
5. Worker asynchrone traite l'événement

**Événements traités** :
- `PAYMENT.CAPTURE.COMPLETED` → mettre à jour capture, émettre `payment.paypal.capture.completed`
- `PAYMENT.CAPTURE.DENIED` → mettre à jour capture, émettre `payment.paypal.capture.denied`
- `PAYMENT.CAPTURE.REFUNDED` → mettre à jour refund, émettre `payment.paypal.refund.completed`
- `BILLING.SUBSCRIPTION.ACTIVATED` → mettre à jour subscription, émettre `payment.paypal.subscription.activated`
- `BILLING.SUBSCRIPTION.CANCELLED` → mettre à jour subscription, émettre `payment.paypal.subscription.cancelled`
- `BILLING.SUBSCRIPTION.PAYMENT.FAILED` → émettre `payment.paypal.subscription.payment.failed`
- `BILLING.SUBSCRIPTION.UPDATED` → mettre à jour subscription

### 5.7 `paypal-webhook-worker` ✅ (à créer)

**Objectif** : traiter les événements en attente dans `payment_outbox`

**Trigger** : Cron (toutes les minutes) ou manuel

**Logique** :
1. Charger événements `status = pending` et `next_retry_at <= now()`
2. Pour chaque événement :
   - Mettre `status = processing`
   - Traiter selon `event_type`
   - Mettre `status = success` ou `error`
   - Si erreur : incrémenter `attempt_count`, calculer `next_retry_at` (backoff exponentiel)
3. Retry max : 5 tentatives

### 5.8 `paypal-test` ✅ (existant)

**Objectif** : tester la connexion PayPal (OAuth2)

**Déjà implémenté** : teste l'obtention d'un access token

---

## 6) Intégration avec modules

### 6.1 Module Commerce par Devis

**Cas d'usage** : paiement d'une facture

**Workflow** :
1. Client accepte devis → facture générée
2. Client clique "Payer" → redirection vers Edge Function `paypal-create-order`
3. Order créé avec montant facture
4. Client approuve sur PayPal
5. Capture automatique → événement `payment.paypal.order.completed`
6. Webhook → mise à jour facture (`status = paid_confirmed`)
7. Écriture budgétaire automatique (via événement `commerce.payment.confirmed`)

**Configuration module** :
- `module_id = 'commerce-devis'`
- `mode = 'orders'`
- `intent = 'CAPTURE'`
- `success_url = '/quotes/{quote_id}/payment/success'`
- `cancel_url = '/quotes/{quote_id}/payment/cancel'`

### 6.2 Module Booking

**Cas d'usage** : paiement d'une réservation

**Workflow** :
1. Client réserve un créneau
2. Si prestation payante → redirection vers `paypal-create-order`
3. Order créé avec montant prestation
4. Client approuve → capture
5. Réservation confirmée (`status = confirmed`)
6. Slot mis à jour (si paiement requis)

**Configuration module** :
- `module_id = 'booking'`
- `mode = 'orders'`
- `intent = 'CAPTURE'`
- `success_url = '/booking/{booking_id}/payment/success'`
- `cancel_url = '/booking/{booking_id}/payment/cancel'`

### 6.3 Module Abonnements (futur)

**Cas d'usage** : abonnement mensuel/annuel

**Workflow** :
1. Client choisit un plan d'abonnement
2. Redirection vers `paypal-create-subscription`
3. Subscription créée avec plan PayPal
4. Client approuve → subscription active
5. PayPal facture automatiquement selon plan
6. Webhooks notifient chaque transaction
7. Mise à jour statut abonnement utilisateur

**Configuration module** :
- `module_id = 'subscriptions'`
- `mode = 'subscriptions'`
- Plans PayPal créés manuellement ou via API

---

## 7) Event map (capability Payment)

### 7.1 Events émis (domaine payment)

- `payment.paypal.order.created`
- `payment.paypal.order.approved`
- `payment.paypal.order.completed`
- `payment.paypal.order.cancelled`
- `payment.paypal.capture.completed`
- `payment.paypal.capture.denied`
- `payment.paypal.refund.completed`
- `payment.paypal.subscription.created`
- `payment.paypal.subscription.activated`
- `payment.paypal.subscription.cancelled`
- `payment.paypal.subscription.payment.completed`
- `payment.paypal.subscription.payment.failed`
- `payment.paypal.webhook.received`

### 7.2 Events consommés (cross-modules)

- `commerce.invoice.issued` → déclenche possibilité de paiement
- `booking.booking.requested` → déclenche paiement si requis

### 7.3 Side-effects typiques

- `payment.paypal.order.completed` → Commerce : facture `paid_confirmed` + Budget : écriture recette
- `payment.paypal.subscription.activated` → Abonnements : activation plan utilisateur
- `payment.paypal.subscription.payment.failed` → Abonnements : suspension plan

---

## 8) Policies / RLS (modèle recommandé)

### 8.1 Principes

- **User** : voit uniquement ses propres orders/subscriptions
- **Admin** : voit tous les paiements (modération)
- **Super Admin** : configuration uniquement
- **Service Role** : écriture dans outbox (worker)

### 8.2 Règles (pseudo)

- `paypal_orders`
  - `SELECT`: `auth.uid() = user_id` OR `is_admin_user()`
  - `INSERT`: authenticated (via Edge Function)
  - `UPDATE`: service role (via webhook/worker)
- `paypal_captures`
  - `SELECT`: user propriétaire de l'order OU admin
  - `INSERT/UPDATE`: service role
- `paypal_subscriptions`
  - `SELECT`: `auth.uid() = subscriber_id` OR `is_admin_user()`
  - `INSERT`: authenticated (via Edge Function)
  - `UPDATE`: service role
- `paypal_provider_configs`, `module_paypal_configs`
  - `SELECT/UPDATE`: `is_super_admin()`

---

## 9) UX / Écrans

### 9.1 Front (client)

**Paiement unique** :
- Bouton "Payer avec PayPal" → redirection Edge Function
- Redirection vers PayPal (approbation)
- Retour sur `success_url` → confirmation
- Retour sur `cancel_url` → annulation

**Abonnement** :
- Sélection plan → redirection Edge Function
- Redirection vers PayPal (approbation)
- Retour sur `success_url` → confirmation
- Page "Mes abonnements" → gestion (annulation)

### 9.2 Back-office (admin)

**Gestion paiements** :
- Liste des orders (filtres : statut, module, date)
- Détails order (montant, statut, capture, refunds)
- Action : rembourser (si enable_refunds)

**Gestion abonnements** :
- Liste des subscriptions (filtres : statut, module, subscriber)
- Détails subscription (plan, cycles, transactions)
- Action : annuler (si admin)

### 9.3 Super Admin

**Configuration** :
- Configuration provider (sandbox/live, credentials)
- Configuration par module (mode, intent, URLs, toggles)
- Test de connexion
- Logs webhooks

---

## 10) Sécurité & bonnes pratiques

### 10.1 Secrets

- `client_secret` stocké dans `paypal_provider_configs` (à chiffrer à terme via Vault)
- Jamais exposé côté client
- Rotation programmée

### 10.2 Webhooks

- Vérification signature obligatoire
- Idempotence via `dedupe_key`
- Retry avec backoff exponentiel
- Logs complets

### 10.3 Idempotence

- Tous les appels PayPal avec `idempotency_key` si supporté
- `dedupe_key` dans `payment_outbox` pour éviter doublons
- Vérification avant création order/subscription

### 10.4 Audit

- Tous les appels PayPal loggés
- Changements de statut tracés
- Erreurs enregistrées avec contexte

---

## 11) Plan de développement

### Phase 1 : Infrastructure de base ✅ (partiellement fait)

- [x] Migration `paypal_provider_configs`
- [x] Migration `module_paypal_configs`
- [x] Migration `payment_outbox`
- [x] Edge Function `paypal-test`
- [ ] Migrations tables `paypal_orders`, `paypal_captures`, `paypal_refunds`

### Phase 2 : Orders API (achats uniques)

- [ ] Edge Function `paypal-create-order`
- [ ] Edge Function `paypal-capture-order`
- [ ] Edge Function `paypal-refund`
- [ ] UI client : bouton "Payer avec PayPal"
- [ ] UI admin : liste orders, détails, remboursement
- [ ] Intégration Commerce : paiement factures
- [ ] Intégration Booking : paiement réservations

### Phase 3 : Webhooks

- [ ] Edge Function `paypal-webhook` (réception)
- [ ] Edge Function `paypal-webhook-worker` (traitement)
- [ ] Configuration webhook dans PayPal Dashboard
- [ ] Tests webhooks (sandbox)

### Phase 4 : Subscriptions API (abonnements)

- [ ] Migration `paypal_subscriptions`
- [ ] Migration `paypal_subscription_transactions`
- [ ] Edge Function `paypal-create-subscription`
- [ ] Edge Function `paypal-cancel-subscription`
- [ ] UI client : sélection plan, gestion abonnements
- [ ] UI admin : liste subscriptions, détails, transactions

### Phase 5 : Améliorations

- [ ] Gestion disputes (si enable_disputes)
- [ ] Payouts (si enable_payouts)
- [ ] Reporting (revenus, remboursements, abonnements actifs)
- [ ] Notifications email (paiement complété, échec, remboursement)

---

## 12) Tests & validation

### 12.1 Tests unitaires

- Helpers PayPal (OAuth2, signature webhook)
- Parsing événements webhook
- Calcul idempotency keys

### 12.2 Tests d'intégration

- Création order (sandbox)
- Capture order (sandbox)
- Webhook reception (sandbox)
- Refund (sandbox)

### 12.3 Tests e2e

- Parcours paiement facture (Commerce)
- Parcours paiement réservation (Booking)
- Parcours abonnement (futur)

---

## 13) Références

### Documentation PayPal

- [Orders API v2](https://developer.paypal.com/docs/api/orders/v2/)
- [Subscriptions API v1](https://developer.paypal.com/docs/api/subscriptions/v1/)
- [Webhooks](https://developer.paypal.com/docs/api-basics/notifications/webhooks/)
- [OAuth2](https://developer.paypal.com/docs/api/overview/#get-an-access-token)

### Documentation Framework

- `docs/framework/migration/Miyukini Framework - Migration Paiement PayPal Provider.sql`
- `docs/framework/migration/Miyukini Framework - Migration Paiement PayPal Module Config.sql`
- `docs/framework/migration/Miyukini Framework - Migration Paiement Outbox.sql`

---

## Prochaines étapes

1. **Valider l'architecture** : Orders API vs Subscriptions API, intégration modules
2. **Créer les migrations** : tables `paypal_orders`, `paypal_captures`, `paypal_refunds`
3. **Développer Edge Functions** : `paypal-create-order`, `paypal-capture-order`, `paypal-refund`
4. **Implémenter webhooks** : `paypal-webhook`, `paypal-webhook-worker`
5. **Intégrer Commerce** : bouton paiement factures
6. **Intégrer Booking** : paiement réservations (si requis)
7. **Tests complets** : sandbox puis live
