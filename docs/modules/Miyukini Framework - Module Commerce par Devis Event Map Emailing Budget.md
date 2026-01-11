# Miyukini Framework - Module Commerce par Devis Event Map Emailing Budget

## Contexte

Le module **Commerce par Devis** s’appuie sur les capabilities **Emailing** et **Budget** pour :

- envoyer des **emails transactionnels** (accusé réception, devis, relances, facture, reçu),
- créer des **écritures budgétaires automatiques** (recette réalisée à confirmation de paiement),

Tout en conservant une orchestration **événementielle** (event-driven), explicite, testable et idempotente.

Ce document explicite la **carte des événements** (event map) et les **templates transactionnels** associés.

## Portée / Scope

- **Inclus** : événements du module `commerce.*`, mapping vers `emailing.*` et `budget.*`, variables de templates, idempotence, règles de déclenchement.
- **Exclus** : implémentation UI, contenu final des emails (copywriting), et paiement en ligne (PSP).

---

## 1) Principes de mécanique (rappel)

- Le module **publie** des événements métier `commerce.*`.
- La capability **Emailing** :
  - rend le template (`notification_settings`) pour un `event_type`,
  - envoie l’email via provider (Gmail/SMTP),
  - journalise via `email_logs`.
- La capability **Budget** :
  - écoute certains événements de paiement,
  - crée/valide une écriture `budget_entries` avec `dedupe_key`.

> Règle : l’envoi d’email et l’écriture budget se font **server-side** (Edge Function / API), jamais depuis le navigateur.

---

## 2) Catalogue des événements du module Commerce par Devis

### 2.1 Events “demande”

- `commerce.quote_request.submitted`
- `commerce.quote_request.assigned`

### 2.2 Events “devis”

- `commerce.quote.created`
- `commerce.quote.sent`
- `commerce.quote.reminder.due` *(recommandé — événement “scheduler” pour les relances)*
- `commerce.quote.viewed` *(optionnel)*
- `commerce.quote.accepted`
- `commerce.quote.rejected`
- `commerce.quote.expired`

### 2.3 Events “facture & paiement”

- `commerce.invoice.issued`
- `commerce.payment.confirmed` *(paiement confirmé manuellement — pas de PSP)*

---

## 3) Mapping Emailing : templates transactionnels

Les templates sont stockés dans `notification_settings` (capability Emailing), avec `event_type` = nom d’événement **ou** alias standard.

### 3.1 Convention `event_type`

Deux approches possibles :

- **A (simple, recommandée)** : `event_type` = même valeur que l’event bus  
  Exemple : `commerce.quote.sent`.
- **B (alias)** : `event_type` “métier stable” indépendant du bus  
  Exemple : `quote_sent` et un mapping interne `quote_sent -> commerce.quote.sent`.

Pour éviter les ambiguïtés, on adopte **A** dans Miyukini (évite les doubles nomenclatures).

### 3.2 Routage & entêtes (from / reply-to / locale)

Objectif : éviter que l’UI “bricole” des emails et centraliser la délivrabilité dans Emailing.

- `from_name` / `from_email` :
  - par défaut : identité du workspace (si domaine validé),
  - fallback : domaine technique (ex: `no-reply@...`) + `reply-to` commercial.
- `reply_to` :
  - recommandé : adresse de l’équipe (ex: `commercial@...`) pour capter les réponses.
- `locale` :
  - valeur “par destinataire” (ex: `fr-FR`) si templates multilingues,
  - sinon `fr-FR` par défaut (clients France).
- Headers techniques (optionnel mais utile) :
  - `X-Miyukini-Event-Type`
  - `X-Miyukini-Entity-Id` (quote_id / invoice_id)
  - `X-Miyukini-Dedupe-Key`

### 3.3 Templates minimum (liste)

| Event (`event_type`) | Email envoyé à | Objectif | Pièces jointes |
| --- | --- | --- | --- |
| `commerce.quote_request.submitted` | demandeur | accusé de réception | optionnel (récap PDF) |
| `commerce.quote_request.assigned` | vendeur/équipe | notification d’attribution | non |
| `commerce.quote.sent` | demandeur | livraison du devis | PDF devis |
| `commerce.quote.reminder.due` | demandeur | relance devis (séquence 1..N) | PDF devis (optionnel) |
| `commerce.quote.accepted` | vendeur + demandeur | confirmation acceptation | PDF devis + CGV (optionnel) |
| `commerce.quote.rejected` | vendeur | notification refus | non |
| `commerce.quote.expired` | demandeur | relance/expiration | PDF devis (optionnel) |
| `commerce.invoice.issued` | demandeur | facture envoyée | PDF facture |
| `commerce.payment.confirmed` | demandeur | reçu / confirmation manuelle | preuve (optionnel) |

### 3.4 Variables de templates (schema minimal)

#### Variables communes

- `requester_name`
- `requester_email`
- `workspace_name`
- `quote_request_title`
- `quote_request_tags`

#### Variables devis

- `quote_number`
- `quote_revision`
- `quote_total_gross`
- `quote_valid_until`
- `quote_pdf_url` *(URL signée / documents)*
- `quote_url` *(page app)*

#### Variables relance

- `reminder_sequence` *(1..N)*
- `reminder_reason` *(ex: `no_reply`, `before_expiry`)*
- `reminder_scheduled_for` *(TIMESTAMPTZ)*

#### Variables facture

- `invoice_number`
- `invoice_total_gross`
- `invoice_due_date`
- `invoice_pdf_url` *(URL signée / documents)*

#### Variables paiement

- `payment_method` *(virement, cheque, etc.)*
- `payment_reference`
- `payment_amount`
- `payment_confirmed_at`

> Best practice : définir `variables_schema` (JSON) dans `notification_settings` pour valider les rendus et éviter les templates cassés.

---

## 4) Event map Emailing (orchestration)

### 4.1 Orchestrations types (pseudo)

#### `commerce.quote.sent`

- Générer PDF devis (Documents)
- `emailing.send(transactional)` avec `notification_settings[event_type=commerce.quote.sent]`
- Écrire `email_logs` (pending -> success/error)

#### `commerce.quote.reminder.due`

- (Optionnel) Régénérer URL signée / vérifier PDF présent
- `emailing.send(transactional)` avec `notification_settings[event_type=commerce.quote.reminder.due]`
- Écrire `email_logs` (pending -> success/error)

#### `commerce.invoice.issued`

- Générer PDF facture (Documents)
- Envoyer email facture
- Écrire log

#### `commerce.payment.confirmed`

- Envoyer reçu/confirmation
- Écrire log

### 4.2 Idempotence emailing (anti-doublons)

Ajouter un `dedupe_key` côté `email_logs.payload` ou colonne dédiée (recommandée) :

- `commerce.quote.sent:<quote_id>:<recipient_email>`
- `commerce.quote.reminder.due:<quote_id>:<sequence>:<recipient_email>`
- `commerce.invoice.issued:<invoice_id>:<recipient_email>`
- `commerce.payment.confirmed:<payment_id>:<recipient_email>`

Règle : si un envoi “success” existe déjà avec la même `dedupe_key`, ne pas renvoyer.

### 4.3 Retry policy

- `pending -> error` : stocker `attempt_count` + `next_retry_at`
- retry exponentiel (ex: 1 min, 5 min, 30 min) avec limite (ex: 5 tentatives)
- surface back-office : bouton “retry” (super_admin) + historique

### 4.4 Scheduling des relances (mécanique recommandée)

Objectif : produire `commerce.quote.reminder.due` de façon déterministe, observable, et idempotente.

Approche “simple Miyukini” :

- une table `commerce_quote_reminders` (ou capability Emailing `email_jobs`) avec :
  - `quote_id`, `sequence`, `run_at`, `status` (`scheduled`,`sent`,`cancelled`),
  - `dedupe_key` UNIQUE = `commerce.quote.reminder.due:<quote_id>:<sequence>`,
  - `recipient_email` (snapshot), `payload` JSONB.
- un worker server-side (Edge Function cron) :
  - sélectionne `run_at <= now()` et `status='scheduled'`,
  - publie `commerce.quote.reminder.due` (ou appelle directement Emailing) en respectant `dedupe_key`,
  - passe `status='sent'` (ou `error` + retry).

Règle : la planification se fait à `commerce.quote.sent` (et éventuellement à chaque révision envoyée).

### 4.5 Statuts de délivrance (optionnel)

Si le provider le permet (webhooks) :

- recevoir `delivered/opened/clicked/bounced/complained`,
- mettre à jour `email_logs.provider_status`,
- (optionnel) publier `emailing.notification.bounced` pour stop relances + action back-office.

---

## 5) Mapping Budget : recettes automatiques

Le budget s’alimente **uniquement** sur le paiement confirmé (pas au moment du devis, ni à l’édition facture).

### 5.1 Événement source

- `commerce.payment.confirmed`

### 5.2 Écriture budgétaire (proposition)

Créer une entrée `budget_entries` :

- `type`: `income`
- `status`: `validated` (ou `draft` si vous voulez une validation comptable manuelle)
- `date`: `payment_confirmed_at::date`
- `amount_*`: `payment_amount` (avec taxes si disponibles)
- `category_id`: ex `Services` / `Prestations` / `Devis` (mapping)
- `module_id`: `commerce-devis`
- `context_id`: `quote_id` / `invoice_id`
- `tags`: inclure tags de la demande (`quote_request.tags`) + `quote_number`
- `dedupe_key`: `commerce.payment.confirmed:<payment_id>` (UNIQUE)
- `invoice_id` (si disponible)

### 5.3 Règles anti-doublons

- `dedupe_key` UNIQUE est la source de vérité.
- si `dedupe_key` déjà présent : ne rien faire.

### 5.4 Cas particuliers

- **Paiement partiel** :
  - créer une écriture par paiement,
  - ou cumuler dans une seule entrée “réconciliée” (niveau 2).
- **Avoir / remboursement** :
  - `type='refund'` ou `income` négatif (selon standard Budget retenu),
  - event futur : `commerce.refund.confirmed`.

---

## 6) Policies / RLS liées aux templates et à l’automatisation

### 6.1 Qui peut éditer les templates ?

Recommandation Miyukini :

- édition `notification_settings` : **admin + super_admin**
- édition provider (gmail/smtp) : **admin + super_admin**
- consultation `email_logs` :
  - **admin + super_admin** (complet),
  - lecture “demandeur” limitée (optionnel) : uniquement ses emails (si besoin UX).

### 6.2 Qui peut déclencher les automations ?

- L’exécution (envoi email + write budget) se fait via **Edge Function** / **Service role**.
- Les utilisateurs n’ont jamais besoin d’INSERT direct sur `email_logs` ni `budget_entries` “auto”.

---

## 7) Checklist de robustesse (vs Catakana non modulaire)

- **1 event = 1 template** (pas de logique “if/else” dans l’UI).
- **PDF server-side** (Documents) avant email.
- **Idempotence** (dedupe_key) sur emailing ET budget.
- **Audit** systématique : `quote_audit` + `email_logs` + `budget_entry_audit` (optionnel).
- **Back-office** : visibilité des retries + erreurs.
