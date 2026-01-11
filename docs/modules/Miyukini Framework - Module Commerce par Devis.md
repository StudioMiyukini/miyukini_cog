# Miyukini Framework - Module Commerce par Devis

## Contexte

Le module **Commerce par Devis** permet de vendre une prestation ou un produit via un **cycle devis → décision humaine → facture → confirmation de paiement**.

Contraintes clés :
- **Ce n’est pas un système de paiement en ligne** (pas de checkout intégré).
- La **décision** (acceptation/refus) est humaine (client et/ou vendeur).
- Le module doit être **modulaire**, réutilisable et robuste (contrairement à une implémentation “vibe-code” monolithique).

Inspirations :
- Catakana Orga : logique facture/validation + automatisations (mais non modulaire).
- Pratiques courantes B2B : devis PDF, échanges, statut, relances, paiement par virement/chèque/espèces/CB à distance, rapprochement manuel.

## Portée / Scope

- **Acteurs** : Demandeur (client), Vendeur/Manager, Admin, Super Admin.
- **Cas d’usage** :
  - création d’une demande via formulaire,
  - qualification + réponse par devis,
  - échanges et itérations,
  - acceptation/refus/expiration,
  - édition de facture,
  - confirmation de paiement (manuelle) + preuve.
- **Hors scope** :
  - paiement en ligne / PSP,
  - signature électronique qualifiée (peut être branchée plus tard via Documents).

---

## 1) Standard de composition (capabilities utilisées)

Le module **compose** des fonctionnalités existantes :

- **Auth/RBAC** : identité, rôles, RLS.
- **Documents** : génération PDF (devis/facture), pièces jointes, preuves de paiement.
- **Emailing** : envoi automatique (devis envoyé, relances, facture, reçu).
- **Budget** : recette réalisée à confirmation de paiement (optionnel mais recommandé).
- **Billing** (capability) : objets facture + numérotation (si déjà existant), sinon tables “invoice” internes au module.

> Règle : le module n’appelle pas Supabase depuis l’UI ; il passe par des ports/adapters.

---

## 2) Workflow métier (états)

### 2.1 Demande de devis (Quote Request)

États possibles (suggestion) :
- `draft` (brouillon)
- `submitted` (envoyée)
- `qualified` (prise en charge / clarifiée)
- `cancelled` (annulée)

### 2.2 Devis (Quote)

États possibles :
- `draft`
- `sent`
- `viewed` (optionnel si tracking)
- `accepted`
- `rejected`
- `expired`
- `cancelled`

Règles :
- un devis “sent” ne peut plus être modifié : il faut créer une **révision**.
- l’acceptation peut exiger : **confirmation** + checkbox “J’accepte les CGV”.

### 2.3 Facture (Invoice)

États possibles :
- `draft`
- `issued` (envoyée)
- `paid_confirmed` (confirmée manuellement)
- `cancelled`
- `credited` (avoir) (optionnel)

Règles :
- la facture peut être générée à partir d’un devis accepté.
- la confirmation de paiement n’est pas automatique : elle est faite par vendeur/admin sur preuve.

---

## 3) UX / écrans (front + back-office)

### 3.1 Front (demandeur)

- **Demander un devis**
  - formulaire dynamique (champ + files + consent)
  - taggage (type de demande) + catégorie (prestation)
- **Mes devis**
  - liste (statuts) + accès au PDF + messagerie
- **Détails devis**
  - timeline (envoyé, relancé, accepté/refusé)
  - CTA : accepter/refuser
  - pièces jointes (devis, annexes)
- **Factures**
  - liste factures liées + statut paiement

### 3.2 Back-office (vendeur/admin)

- **Inbox demandes**
  - filtres (tags, statut, priorité, assigné)
- **Qualification**
  - check-list, notes internes, estimation
- **Édition devis**
  - lignes, remises, taxes, conditions, délais
  - génération PDF + envoi email
  - révisions
- **Suivi**
  - relances automatiques (Emailing) + tâches
  - statut manuel (vue “pipeline”)
- **Facturation**
  - générer facture depuis devis accepté
  - confirmer paiement + uploader preuve (Documents)
  - déclenchement Budget (recette)

---

## 4) Data contract (proposition)

### 4.1 Tables principales

#### `quote_requests`
- `id` UUID
- `workspace_id` UUID (si multi-tenant)
- `requested_by` UUID (profiles.id)
- `assigned_to` UUID (profiles.id) NULL
- `status` ENUM (`draft`,`submitted`,`qualified`,`cancelled`)
- `title` TEXT
- `description` TEXT
- `tags` TEXT[]
- `category` TEXT (optionnel)
- `contact_email` TEXT (fallback si user non connecté)
- `contact_phone` TEXT (optionnel)
- `preferred_contact_channel` ENUM (`email`,`phone`) (optionnel)
- `meta` JSONB
- `created_at`, `updated_at`

#### `quote_request_files` (optionnel si Documents externalisé)
- `id` UUID
- `quote_request_id` UUID
- `document_ref` JSONB (ex: `{bucket, path}` ou `documents.id`)
- `created_at`

#### `quotes`
- `id` UUID
- `workspace_id` UUID
- `quote_request_id` UUID
- `quote_number` TEXT UNIQUE (par workspace + année)
- `status` ENUM (`draft`,`sent`,`viewed`,`accepted`,`rejected`,`expired`,`cancelled`)
- `revision` INT DEFAULT 1
- `valid_until` DATE
- `currency` TEXT DEFAULT 'EUR'
- `subtotal_net` NUMERIC
- `tax_amount` NUMERIC
- `total_gross` NUMERIC
- `terms` TEXT (conditions)
- `internal_notes` TEXT (visible staff only)
- `sent_at`, `accepted_at`, `rejected_at`
- `created_by`, `updated_by`, `created_at`, `updated_at`

#### `quote_items`
- `id` UUID
- `quote_id` UUID
- `label` TEXT
- `description` TEXT
- `quantity` NUMERIC
- `unit_price_net` NUMERIC
- `tax_rate` NUMERIC (ex: 0, 0.2)
- `line_total_net` NUMERIC
- `position` INT

#### `quote_messages` (thread)
- `id` UUID
- `quote_id` UUID
- `author_id` UUID NULL (si email externe)
- `author_role` ENUM (`requester`,`seller`,`admin`)
- `message` TEXT
- `is_internal` BOOLEAN DEFAULT false
- `created_at`

#### `quote_audit`
- `id` UUID
- `quote_id` UUID
- `event_type` TEXT (ex: `sent`, `status_changed`, `revision_created`)
- `actor_id` UUID
- `payload` JSONB
- `created_at`

### 4.2 Factures & paiements

Deux options :

**Option A (recommandée)** : réutiliser la capability Billing (si présente)
- `billing.invoices` + `billing.payments` (référence par `quote_id`)

**Option B** : tables module (si Billing non disponible)

#### `quote_invoices`
- `id` UUID
- `quote_id` UUID
- `invoice_number` TEXT UNIQUE
- `status` ENUM (`draft`,`issued`,`paid_confirmed`,`cancelled`,`credited`)
- `issued_at`, `due_date`
- `currency`, `subtotal_net`, `tax_amount`, `total_gross`
- `created_by`, `created_at`, `updated_at`

#### `invoice_payments`
- `id` UUID
- `invoice_id` UUID
- `method` ENUM (`bank_transfer`,`cash`,`check`,`card_remote`,`other`)
- `reference` TEXT
- `amount` NUMERIC
- `paid_at` TIMESTAMPTZ
- `confirmed_by` UUID
- `proof_document_ref` JSONB (Documents)
- `notes` TEXT
- `created_at`

---

## 5) Events (event map)

### 5.1 Events émis (domaine commerce-devis)

- `commerce.quote_request.submitted`
- `commerce.quote_request.assigned`
- `commerce.quote.created`
- `commerce.quote.sent`
- `commerce.quote.accepted`
- `commerce.quote.rejected`
- `commerce.quote.expired`
- `commerce.invoice.issued`
- `commerce.payment.confirmed`

### 5.2 Events consommés (cross-capabilities)

- `emailing.notification.sent` / `failed` (observabilité)
- `documents.file.uploaded` (attach proof)

### 5.3 Side-effects typiques

- `commerce.quote.sent` → Emailing : mail + PDF devis
- `commerce.quote.accepted` → génération facture (billing) + mail facture
- `commerce.payment.confirmed` → Budget : création entrée recette + notification au demandeur

---

## 6) Policies / RLS (modèle recommandé)

### 6.1 Principes

- **Demandeur** : voit uniquement ses demandes/devis/factures.
- **Vendeur/Manager** : voit les dossiers du workspace, modifie ceux assignés (ou tous selon rôle).
- **Admin/SuperAdmin** : tout, y compris modération/audit.
- **Public** : aucun accès (sauf lien “unlisted” optionnel vers un PDF, via token signé).

### 6.2 Règles (pseudo)

- `quote_requests`
  - `SELECT`: `auth.uid() = requested_by` OR `is_admin_user()`
  - `INSERT`: utilisateur connecté OU formulaire public (via edge + anti-spam) (recommandé)
  - `UPDATE`: `requested_by` sur champs limités tant que `status in ('draft','submitted')` ; staff sinon
- `quotes`, `quote_items`, `quote_messages`
  - `SELECT`: demandeur lié OU staff
  - `UPDATE`: staff uniquement, sauf action “accept/reject” côté demandeur (via RPC contrôlée)
- `quote_invoices` / `billing.invoices`
  - `SELECT`: demandeur lié OU staff
  - `UPDATE`: staff uniquement

> Important : éviter les subqueries récursives sur `profiles` dans les policies.  
> Utiliser les helpers `public.is_admin_user()` / `public.is_super_admin()` déjà standardisés côté Miyukini.

---

## 7) Robuste vs “vibe-code” (leçons Catakana)

Ce module doit éviter :

- la logique “tout dans une table” et statuts flous,
- les triggers “magiques” non documentés,
- l’absence d’idempotence (doublons facture/budget),
- l’absence de révisions (modifier un devis envoyé sans trace),
- la génération de PDF côté client.

Standards Miyukini :

- **statuts et transitions explicites** (avec validations),
- **audit** systématique sur envois / décisions / paiements,
- **idempotence** (dedupe sur événements),
- **PDF server-side** (Documents),
- **orchestration par events** (Emailing/Budget/Billing).

---

## 8) Emailing (fonction transactionnelle)

### 8.1 Objectifs

- **Transactionnel** uniquement : accusés de réception, devis, relances devis, facture, confirmation de paiement.
- **Server-side** uniquement (Edge Function/API) : pas d’envoi depuis le navigateur.
- **Traçabilité** : chaque email lié à un `event_type` + `dedupe_key` + entité (`quote_id`, `invoice_id`, etc.).

### 8.2 Déclencheurs (événements)

Événements du module (voir event map) :

- `commerce.quote_request.submitted` → accusé réception demandeur
- `commerce.quote_request.assigned` → notification staff
- `commerce.quote.sent` → email devis + PDF
- `commerce.quote.accepted` / `commerce.quote.rejected` → notifications
- `commerce.invoice.issued` → email facture + PDF
- `commerce.payment.confirmed` → email reçu/confirmation

Relances (recommandé pour éviter des “if/else” dans l’UI) :

- `commerce.quote.reminder.due` *(généré par scheduler)* → relance devis (séquence 1..N)

### 8.3 Relances devis (règles métier)

Proposition (paramétrable par workspace) :

- **Conditions** : relancer uniquement si `quote.status = sent` (ou `viewed`) et `valid_until` non dépassé.
- **Annulation** : stop relances dès `accepted`, `rejected`, `cancelled`, `expired`.
- **Cadence** (exemple) :
  - J+2 après `sent` : relance #1
  - J+5 après `sent` : relance #2
  - J-1 avant `valid_until` : relance “dernière chance”
- **Garde-fous** :
  - max `N` relances,
  - silence si le demandeur a répondu dans le thread `quote_messages` après l’envoi (optionnel),
  - respect des horaires (ex: 09:00–18:00 Europe/Paris) si on planifie des envois.

### 8.4 Tracking (optionnel mais utile)

- `commerce.quote.viewed` :
  - déclenché quand le demandeur ouvre la page `quote_url` (recommandé),
  - pas de promesse de tracking “ouverture PDF” (non fiable).
- Le tracking sert à :
  - améliorer le back-office (pipeline),
  - adapter la relance (ex: relance différente si “viewed”).

### 8.5 RGPD / consent (France)

- Base légale : communication **précontractuelle** / **exécution de contrat** → emails transactionnels autorisés (à distinguer du marketing).
- Données minimales :
  - `recipient_email`, `event_type`, `sent_at`, `provider_message_id` (si dispo), statut.
- Rétention (recommandation) :
  - logs techniques `email_logs` : 6 à 12 mois (selon besoins support/audit),
  - pièces jointes (PDF) : alignées avec la rétention Documents (et accès via URL signée).
- Marketing hors scope : si un jour “newsletter” → consentement explicite + désinscription.

### 8.6 Sécurité & délivrabilité

- **URLs signées** pour `quote_pdf_url` / `invoice_pdf_url` (expiration courte + renouvellement à la demande).
- `from` et `reply-to` :
  - `from`: identité workspace (ex: `devis@...`) si domaine validé,
  - `reply-to`: équipe commerciale (pour centraliser les réponses).
- Gestion erreurs :
  - email invalide → statut `failed` + action back-office,
  - retry exponentiel (capability Emailing),
  - bouton “renvoyer” (admin/super_admin) en respectant `dedupe_key` (nouvelle clé si renvoi volontaire).

---

## Prochaines étapes

1) Valider l’option A ou B pour la facturation (Billing capability vs tables module).
2) Finaliser la partie Emailing : relances (scheduler) + tracking (viewed) + règles RGPD/rétention.
3) Définir le formulaire “demande de devis” (fields + validation + anti-spam).
4) Préparer la migration SQL (tables + RLS + triggers optionnels) via MCP Supabase.
