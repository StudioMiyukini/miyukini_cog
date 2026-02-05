# Odoo Social Marketing — Guide Implémentation

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Social Marketing dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée
- Spécifications des crates Rust (ou modules)
- Schémas de données
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates (Proposition)

```
crates/
├── miyusocial-account/           # SocialAccountOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── account.rs             # Modèle SocialAccount, OAuth
│   │   ├── token_storage.rs       # Stockage sécurisé tokens (WorrySentinel)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyusocial-stream/             # SocialStreamOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── stream.rs              # Modèle SocialStream
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyusocial-post/               # SocialPostOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── post.rs                # Modèle SocialPost, états
│   │   ├── scheduler.rs            # Publication planifiée
│   │   ├── api_publisher.rs       # Appels APIs Facebook, etc.
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyusocial-campaign/           # SocialCampaignOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── campaign.rs            # Modèle Campaign, stages
│   │   ├── content_binding.rs     # Liens post / mailing / SMS / push
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyusocial-lead/               # SocialLeadOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── lead_from_comment.rs   # Create Lead from comment
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyusocial-visitor/            # SocialVisitorOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── visitor.rs             # Lecture website.visitor
│   │   ├── actions.rs             # Send Email / SMS
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyusocial-insights/           # SocialInsightsOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── insights.rs            # Agrégation KPIs par stream
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyusocial-ui/                 # SocialMarketingUI
    ├── src/
    │   ├── lib.rs
    │   ├── views/
    │   │   ├── feed.rs            # Tableau de bord colonnes streams
    │   │   ├── posts_kanban.rs
    │   │   ├── posts_calendar.rs
    │   │   ├── posts_list.rs
    │   │   ├── posts_pivot.rs
    │   │   ├── post_form.rs       # Formulaire détail post
    │   │   ├── campaigns_kanban.rs
    │   │   ├── campaign_template.rs
    │   │   ├── visitors_kanban.rs
    │   │   ├── visitors_list.rs
    │   │   └── config.rs         # Social Media, Social Accounts, Social Streams
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Kits existants :**
- `miyucontacts` : Partenaires (liens leads, visiteurs)
- `miyuclock` : Dates, planification
- `miyunotify` : Notifications, chatter
- `miyumedia` : Pièces jointes (images posts)
- `miyuutm` : Campagnes UTM

**Optionnels :**
- `miyucrm` : Création leads (SocialLeadOperator)
- `miyusales` / `miyuinvoice` : Smart buttons Revenues, Quotations
- `miyuweb` : Visiteurs, paramètres push
- `miyumail` : Mailings dans campagnes
- `miyusms` : SMS dans campagnes et visiteurs

**APIs externes :**
- Facebook Graph API (Facebook, Instagram)
- LinkedIn API, Twitter API, YouTube API (selon périmètre)
- Web Push (navigateur) pour push notifications

---

## 2. Schémas de Données

### 2.1 SocialAccount

| Champ | Type | Description |
|-------|------|-------------|
| id | Uuid | PK |
| name | String | Nom du compte |
| handle | String | Handle / short name |
| media_type | Enum | facebook, instagram, linkedin, twitter, youtube, push_notification |
| company_id | Uuid | FK res.company |
| create_uid | Uuid | FK res.users |
| website_id | Option<Uuid> | FK website (push) |
| token_encrypted | Blob | Token chiffré (WorrySentinel) |
| created_at | DateTime | |
| updated_at | DateTime | |

### 2.2 SocialStream

| Champ | Type | Description |
|-------|------|-------------|
| id | Uuid | PK |
| account_id | Uuid | FK SocialAccount |
| title | String | Titre du flux |
| stream_type | Enum | posts, keyword, ... |
| company_id | Uuid | FK res.company |
| create_uid | Uuid | FK res.users |
| created_at | DateTime | |
| updated_at | DateTime | |

### 2.3 SocialPost

| Champ | Type | Description |
|-------|------|-------------|
| id | Uuid | PK |
| message | Text | Contenu |
| account_ids | Vec<Uuid> | Comptes cibles (M2M) |
| image_ids | Vec<Uuid> | Pièces jointes (M2M) |
| campaign_id | Option<Uuid> | FK utm.campaign |
| company_id | Uuid | FK res.company |
| state | Enum | draft, scheduled, published, failed |
| scheduled_date | Option<DateTime> | |
| published_date | Option<DateTime> | |
| push_notification_title | Option<String> | |
| push_notification_target_url | Option<String> | |
| push_notification_icon | Option<Uuid> | |
| push_notification_local_time | bool | |
| push_match_rules | Option<Json> | Match all records |
| created_at | DateTime | |
| updated_at | DateTime | |

### 2.4 SocialCampaign (extension utm.campaign ou modèle dédié)

| Champ | Type | Description |
|-------|------|-------------|
| id | Uuid | PK |
| name | String | |
| user_id | Uuid | FK res.users (responsable) |
| tag_ids | Vec<Uuid> | M2M tags |
| stage_id | Uuid | FK campaign.stage |
| created_at | DateTime | |
| updated_at | DateTime | |

### 2.5 Liens campagne ↔ contenu

- campaign_post_ids (One2many / M2M) : posts liés
- campaign_mailing_ids, campaign_sms_ids (si MiyuMail, MiyuSMS)
- Métriques : revenus, devis, leads via UTM (requêtes sur sale.order, account.move, crm.lead avec campaign_id)

---

## 3. API et Contrats

### 3.1 SocialAccountOperator

- `connect_account(intent, mandate) -> Result<SocialAccountId>`
- `disconnect_account(account_id, mandate) -> Result<()>`
- `list_accounts(company_id?, mandate) -> Result<Vec<SocialAccount>>`
- `read_account(account_id, mandate) -> Result<SocialAccount>`

### 3.2 SocialPostOperator

- `create_post(intent, mandate) -> Result<PostId>`
- `update_post(post_id, intent, mandate) -> Result<()>`
- `schedule_post(post_id, scheduled_date, mandate) -> Result<()>`
- `publish_post_now(post_id, mandate) -> Result<()>`
- `list_posts(filters?, mandate) -> Result<Vec<SocialPost>>`
- `read_post(post_id, mandate) -> Result<SocialPost>>` (avec commentaires si API)

### 3.3 SocialCampaignOperator

- `create_campaign(intent, mandate) -> Result<CampaignId>`
- `update_campaign(campaign_id, intent, mandate) -> Result<()>`
- `add_content(campaign_id, content_type, content_intent, mandate) -> Result<ContentId>`
- `list_stages(mandate) -> Result<Vec<Stage>>`
- `list_campaigns(filters?, mandate) -> Result<Vec<Campaign>>`
- `campaign_metrics(campaign_id, mandate) -> Result<CampaignMetrics>>` (Revenues, Quotations, Leads)

### 3.4 SocialLeadOperator

- `create_lead_from_comment(intent, mandate) -> Result<LeadId>`

### 3.5 SocialVisitorOperator

- `list_visitors(filters?, mandate) -> Result<Vec<Visitor>>`
- `read_visitor(visitor_id, mandate) -> Result<Visitor>>`
- `send_email(visitor_id, mailing_intent, mandate) -> Result<()>`
- `send_sms(visitor_id, sms_intent, mandate) -> Result<()>`

### 3.6 SocialInsightsOperator

- `insights_by_stream(stream_id, mandate) -> Result<InsightsData>>`
- `insights_link(stream_id, mandate) -> Result<Url>>`

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (Bornage minimal)

- **Comptes et streams** : Connexion OAuth un seul type (ex. Facebook ou Twitter), liste comptes, création streams.
- **Posts** : Création post (message, comptes), Send Now uniquement ; pas de planification ni push.
- **UI** : Feed simple (une colonne par stream), formulaire New Post, liste Posts (List ou Kanban).
- **Sécurité** : Stockage token chiffré, permissions Master Butler, WorrySentinel sur tokens.
- **Pas encore** : Campagnes, leads depuis commentaires, visiteurs, insights, planification, push, email/SMS.

### Phase 2 — Publications avancées

- Planification (Schedule later) + job scheduler.
- Push notifications (si MiyuWeb disponible) : champs push dans formulaire post, envoi via Web Push.
- Images (Attach Images) : intégration MiyuMedia.
- Campagnes : modèle campagne, stages Kanban, liaison post ↔ campagne ; pas encore mailing/SMS.

### Phase 3 — Campagnes et CRM

- Création lead depuis commentaire (SocialLeadOperator + MiyuCRM).
- Campagnes : Send New Mailing, Send SMS (si MiyuMail, MiyuSMS) ; onglets et smart buttons Revenues, Quotations, Leads (UTM + MiyuSales, MiyuInvoice, MiyuCRM).

### Phase 4 — Visiteurs et insights

- Visiteurs : lecture website.visitor (MiyuWeb), vues Kanban/List/Graph, actions Email/SMS.
- Insights : lien par stream vers KPIs plateforme (lecture API ou URL externe).

### Phase 5 — Multi-plateformes et robustesse

- OAuth et publication pour Instagram, LinkedIn, YouTube (en plus Facebook/Twitter).
- Gestion erreurs API (quota, token révoqué), état `failed` sur post, messages utilisateur.
- Multi-company : règles documentées et messages d’erreur explicites.

---

## 5. Bornage Fonctionnel

| Fonctionnalité | MVP (Phase 1) | Phase 2 | Phase 3 | Phase 4 | Phase 5 |
|----------------|---------------|---------|---------|---------|---------|
| Connexion comptes (1 type) | Oui | Oui | Oui | Oui | Oui |
| Connexion multi-plateformes | Non | Optionnel | Oui | Oui | Oui |
| Posts Send Now | Oui | Oui | Oui | Oui | Oui |
| Posts Schedule | Non | Oui | Oui | Oui | Oui |
| Push notifications | Non | Oui | Oui | Oui | Oui |
| Images posts | Non | Oui | Oui | Oui | Oui |
| Campagnes (étapes, liaison posts) | Non | Oui | Oui | Oui | Oui |
| Create Lead from comment | Non | Non | Oui | Oui | Oui |
| Mailings / SMS campagnes | Non | Non | Oui | Oui | Oui |
| Visiteurs (liste, Email/SMS) | Non | Non | Non | Oui | Oui |
| Insights par stream | Non | Non | Non | Oui | Oui |
| Gestion erreurs API / multi-company | Non | Partiel | Partiel | Oui | Oui |

---

## 6. Critères d'Acceptation (MVP)

1. Un compte social (ex. Facebook) peut être connecté via OAuth et apparaît comme stream sur le Feed.
2. Un post peut être créé avec message et un ou plusieurs comptes, et publié immédiatement (Send Now).
3. Le Feed affiche au moins une colonne par stream avec les posts récents.
4. Les tokens sont stockés de manière chiffrée (WorrySentinel).
5. Les actions (connexion, publication) passent par StrongFather et KindMother (WriteIntent).
6. L’accès à l’UI et aux actions est contrôlé par Master Butler (permissions).

---

## 7. Risques et Mitigation

| Risque | Mitigation |
|--------|------------|
| Changements APIs plateformes (Facebook, etc.) | Adapter api_publisher par plateforme ; tests d’intégration mockés ; veille release notes. |
| Tokens révoqués ou quota dépassé | État `failed` sur post ; messages clairs ; réautorisation OAuth guidée. |
| Multi-company / permission errors | Documentation et messages d’erreur explicites ; recommandation « activer toutes les pages pour toutes les sociétés ». |
| RGPD (visiteurs, push) | Consentement et opt-out gérés côté Website ; WorrySentinel sur envoi Email/SMS visiteurs. |
| Dépendances optionnelles (CRM, Mail, SMS) | Contrats d’équipe et Mandats conditionnels ; désactivation des boutons/onglets si app absente. |

---

**Document** : Odoo Social Marketing — Guide Implémentation  
**Version** : 1.0  
**Date** : 2026-02-01
