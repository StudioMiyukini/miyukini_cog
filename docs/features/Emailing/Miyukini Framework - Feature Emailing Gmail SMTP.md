# Miyukini Framework - Feature Emailing Gmail SMTP

## Contexte

Le Miyukini Framework doit permettre d’**envoyer des emails automatiquement** :
- depuis un **compte Gmail** (OAuth2 ou SMTP),
- ou via un **serveur SMTP** (ex: SMTP client, OVH, Mailjet, etc.),
en gardant une configuration **pilotable depuis le back-office** et protégée par RLS.

Cette fonctionnalité s’inspire directement de Catakana Orga (tables `gmail_credentials`, `notification_settings`, `email_logs`, `email_campaigns`, etc.) et des bonnes pratiques observées sur des produits “pro” (Doctolib/Planity-like) : logs, retries, templates, quotas, deliverability.

## Portée / Scope

- **Capability** : Emailing (providers Gmail/SMTP + templates + campagnes + logs).
- **Back-office** : écrans de configuration et d’exploitation (templates, campagnes, logs).
- **Sécurité** : RLS + restriction d’accès (RLS 9 et 10 = **admin** et **super_admin**), audit, minimisation d’exposition des secrets.
- **Hors scope** (pour cette doc) : implémentation complète UI, intégration d’un provider externe transactionnel (Sendgrid/Mailgun) — mais la conception le permet.

---

## 1) Concepts (glossaire)

- **Provider** : méthode d’envoi (Gmail OAuth2, Gmail SMTP, SMTP générique).
- **Notification Settings** : templates transactionnels déclenchés par un event (ex: `invoice_paid`).
- **Campagne** : envoi en masse, planifié, avec segmentation.
- **Logs** : preuve d’envoi et trace technique (pending/success/error, payload).
- **Edge Function** : exécution server-side pour envoyer l’email (recommandé).

---

## 2) Architecture cible (capability)

### 2.1 Règle d’or (sécurité)

Même si la configuration est éditable via back-office, **l’envoi doit rester server-side** (Edge Function / API) :
- éviter d’exposer des secrets côté navigateur,
- centraliser le retry, la rate-limit, l’audit,
- uniformiser le comportement entre modules.

### 2.2 Ports (interfaces) recommandés

- **`EmailProviderPort`**
  - `sendEmail(request): Promise<SendResult>`
  - `testConnection(): Promise<TestResult>`
- **`NotificationTemplatesPort`**
  - `render(eventType, variables): RenderedEmail`
  - `validateTemplate(template): ValidationReport`
- **`CampaignPort`**
  - `scheduleCampaign(id, scheduledAt)`
  - `processDueCampaigns()`
- **`EmailLogsPort`**
  - `writeLog(entry)`
  - `listLogs(filters)`

---

## 3) Data contract (tables) — inspiré Catakana Orga

### 3.1 Configuration provider (Gmail/SMTP)

#### `public.gmail_credentials` (Catakana)
Stocke les paramètres OAuth2 + (optionnel) SMTP.

Champs (Catakana) :
- `client_id`, `client_secret`, `refresh_token`, `sender_email`
- `auth_method` (`oauth2` ou `smtp`)
- `smtp_host`, `smtp_port`, `smtp_password`
- `updated_at`, `updated_by`

**Recommandation Miyukini** :
- Renommer vers `public.email_provider_configs` (plus générique) **ou** conserver `gmail_credentials` si on assume “Gmail-first”.
- Ajouter :
  - `provider` (`gmail`, `smtp`)
  - `smtp_username`
  - `from_name`, `reply_to`, `headers_json`
  - `is_active`
  - `workspace_id` (si multi-tenant)
  - `encrypted_*` (si chiffrement applicatif/DB)

### 3.2 Templates transactionnels

#### `public.notification_settings` (Catakana)
Configuration des templates email par `event_type`.

Champs (Catakana) :
- `event_type`, `enabled`
- `subject_template`, `body_template`
- `cc[]`, `bcc[]`, `sender_override`, `reply_to`

**Recommandation Miyukini** :
- Ajouter :
  - `variables_schema` (JSON) : variables attendues (validation)
  - `locale` (`fr-FR`) / multi-langue
  - `provider_override` (forcer un provider)
  - `rate_limit_policy` (anti-spam)

### 3.3 Logs transactionnels

#### `public.email_logs` (Catakana)
Trace d’envoi.

Champs (Catakana) :
- `event_type`, `recipient`, `subject`
- `status` (`pending`, `success`, `error`)
- `error_message`, `payload`, `sent_at`, `created_at`
- liens facultatifs (`invoice_id`, `reservation_id`)

**Recommandation Miyukini** :
- Ajouter :
  - `provider`/`provider_message_id`
  - `attempt_count`, `next_retry_at`
  - `dedupe_key` (idempotency)
  - `workspace_id`

### 3.4 Campagnes (emailing system)

Catakana crée :
- `email_campaigns`, `email_templates`, `email_recipients`, `email_engagement`

**Recommandation Miyukini** :
- Conserver la même approche (c’est un bon standard) et rajouter :
  - `campaign_type` (`transactional`, `marketing`, `internal`)
  - `legal_unsubscribe` (marketing)
  - `throttling` (ex: 50/min)

---

## 4) Event map (capability Emailing)

### 4.1 Events consommés (depuis les modules)

Exemples d’events cross-modules :
- `billing.invoice.created`
- `billing.invoice.paid`
- `agenda.slot.confirmed`
- `agenda.slot.cancelled`
- `documents.file.uploaded`
- `auth.user.created`

### 4.2 Events émis (par Emailing)

- `emailing.notification.sent`
- `emailing.notification.failed`
- `emailing.campaign.scheduled`
- `emailing.campaign.sent`
- `emailing.campaign.failed`

---

## 5) Back-office : configuration (RLS 9 & 10)

### 5.1 Accès (règle)

Dans Miyukini Framework :
- **RLS 9** = `admin`
- **RLS 10** = `super_admin`

Seuls ces rôles peuvent :
- voir/modifier la config provider (Gmail/SMTP),
- éditer les templates globaux,
- déclencher une campagne,
- consulter les logs complets.

### 5.2 Écrans back-office recommandés

- **Emailing → Provider**
  - Choix : `Gmail OAuth2`, `Gmail SMTP`, `SMTP`
  - Champs : sender, reply-to, host/port, oauth client_id/secret/refresh_token
  - Actions : “Tester la connexion”, “Envoyer un email de test”
  - Indicateurs : statut OK/KO, dernier test, quotas estimés

- **Emailing → Templates transactionnels**
  - Liste des `event_type` (ex: `invoice_paid`, `agenda_slot_confirmed`)
  - Editor subject/body + variables + validation
  - Enable/disable + preview rendu

- **Emailing → Campagnes**
  - création campagne, segmentation, scheduling, preview, send test
  - throttling + suivi “sent/failed/bounced”

- **Emailing → Logs**
  - filtres (date, recipient, event_type, status)
  - export CSV, retry manuel (super_admin)

---

## 6) Policies / RLS (modèle recommandé Miyukini)

### 6.1 Principe

- Les tables de configuration (`email_provider_configs` / `gmail_credentials`) sont **privées** :
  - lecture/écriture uniquement **admin/super_admin**.
- Les logs :
  - lecture admin/super_admin,
  - écriture server-side (Edge Function) ou service role.

### 6.2 Exemple de règles (pseudo)

- **Provider config** :
  - `SELECT/UPDATE`: `public.is_admin_user()` (ou `role in ('admin','super_admin')`)
- **Notification settings** :
  - `SELECT`: admin/super_admin
  - `UPDATE`: super_admin (optionnel) ou admin/super_admin
- **Email logs** :
  - `SELECT`: admin/super_admin
  - `INSERT`: service role / edge function uniquement (recommandé)

> Important : éviter les sous-requêtes récursives sur `profiles` dans les policies (Catakana le fait).  
> Dans Miyukini, privilégier des helpers RLS type `public.is_admin_user()` / `public.is_super_admin()`.

---

## 7) Meilleures pratiques (deliverability & sécurité)

### 7.1 Gmail

- **OAuth2** : privilégier OAuth2 (refresh token) plutôt qu’un mot de passe.
- **SMTP Gmail** : si utilisé, préférer un **App Password** (si compte compatible) et TLS.
- **Quotas** : gérer un throttling + backoff (Gmail impose des limites).

### 7.2 SMTP générique

- Toujours TLS (STARTTLS 587 ou SMTPS 465 selon provider).
- Stocker les secrets **chiffrés** (et jamais en clair dans le front).
- Prévoir la rotation des secrets.

### 7.3 Templates

- Validation des variables (schéma), preview avant envoi.
- Séparer :
  - **transactionnel** (pas de désinscription obligatoire mais éviter le “marketing déguisé”),
  - **marketing** (unsubscribe + conformité RGPD).

### 7.4 Logs / retries / idempotence

- Toujours écrire un log `pending` → `success/error`.
- Retries exponentiels + `attempt_count`.
- `dedupe_key` pour éviter les doubles envois (ex: `invoice_paid:<invoiceId>:<recipient>`).

---

## 8) Référence Catakana Orga (source)

Les bases viennent des migrations :
- `20251114_create_gmail_credentials.sql`
- `20251114_add_smtp_support.sql`
- `20251114_create_notification_settings_and_email_logs.sql`
- `20251116_create_emailing_system.sql`
et des Edge Functions associées (ex: `send-invoice-email`, `send-campaign-email`, `process-scheduled-campaigns`).

