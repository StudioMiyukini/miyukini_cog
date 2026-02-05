# Odoo Email Marketing — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Email Marketing dans Miyukini, avec **bornage fonctionnel**, **spécifications techniques** et **plan de développement**.

**Références :**
- [Logique Métier](../00_logique_metier/Odoo%20Email%20Marketing%20-%20Logique%20Metier%20Complete.md)
- [Spécifications Opérateurs](../04_specifications_miyukini/Odoo%20Email%20Marketing%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Guide Intégration COG](../05_integration_cog/Odoo%20Email%20Marketing%20-%20Guide%20Integration%20COG.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée (crates / modules)
- Schémas de données principaux
- API et contrats
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel et critères d'acceptation
- Risques et mitigation

**Hors scope :**
- Implémentation complète du code (sera dans les crates)
- Tests unitaires détaillés (sera dans les tests)

---

## 1. Architecture technique

### 1.1 Structure des crates (proposition)

```
crates/
├── miyukini-mailing-campaign/     # MailingCampaignOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── mailing.rs              # Modèle Mailing, états, KPIs
│   │   ├── schedule.rs             # Planification, envoi, A/B test
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-mailing-list/         # MailingListOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── list.rs
│   │   ├── import.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-mailing-contact/      # MailingContactOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── contact.rs
│   │   ├── subscription.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-mailing-blacklist/    # MailingBlacklistOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── blacklist.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-mailing-trace/        # MailingTraceOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── trace.rs
│   │   ├── kpi.rs                  # Agrégation KPIs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-mailing-content/      # MailingContentOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── content.rs             # Sujet, body_html, preview
│   │   ├── template.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-mailing-filter/       # MailingFilterOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── filter.rs
│   │   ├── resolve.rs             # Résolution destinataires (lecture)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-mailing-campaign-utm/ # MailingCampaignUTMOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── campaign_utm.rs
│   │   ├── metrics.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyukini-mailing-ui/           # MailingUI (frontend / API)
    ├── src/
    │   ├── lib.rs
    │   ├── api.rs                  # Endpoints REST ou GraphQL
    │   ├── portal.rs               # Désabonnement (Façade)
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances principales

**Cores Miyukini :**
- `miyukini-kernel` : Id, Logger, Clock
- `miyukini-central` : StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy

**Kits / Opérateurs externes :**
- Envoi SMTP : crate ou service dédié (hors Cores)
- Link tracker : crate dédiée pour clics (optionnel en MVP)
- Contacts / CRM / Sales / Events : selon contrats (lecture pour filtres)

**Externes :**
- `serde`, `chrono`, `uuid`, `thiserror`, `async-trait`

---

## 2. Schémas de données (résumé)

### 2.1 Mailing

```rust
pub struct Mailing {
    pub id: MailingId,
    pub subject: String,
    pub email_from: Option<String>,
    pub reply_to: Option<String>,
    pub body_html: String,
    pub preview: Option<String>,
    pub state: MailingState,
    pub sent_date: Option<DateTime<Utc>>,
    pub schedule_date: Option<DateTime<Utc>>,
    pub contact_list_ids: Vec<ListId>,
    pub mailing_filter_id: Option<FilterId>,
    pub campaign_id: Option<CampaignUtmId>,
    pub user_id: Option<UserId>,
    pub ab_testing_enabled: bool,
    pub ab_testing_percentage: Option<u8>,
    pub ab_testing_winner_selection: Option<AbWinnerSelection>,
    pub ab_testing_schedule_date: Option<DateTime<Utc>>,
    pub kpi_mail_sent: u64,
    pub kpi_mail_delivered: u64,
    pub kpi_mail_opened: u64,
    pub kpi_mail_clicked: u64,
    pub kpi_mail_replied: u64,
    pub kpi_mail_bounced: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum MailingState { Draft, InQueue, Sending, Done, Cancel }
```

### 2.2 MailingList, MailingContact, Blacklist, Trace, Filter, CampaignUtm

- **MailingList :** id, name, contact_count, company_id, created_at, updated_at.
- **MailingContact :** id, email, name, list_ids (Many2many), partner_id (optionnel), company_id, created_at, updated_at.
- **Blacklist :** id, email, active, reason, created_at, updated_at.
- **MailingTrace :** id, mailing_id, email, res_id, model, state (outgoing, sent, open, reply, bounce, exception, cancel), sent_datetime, open_datetime, failure_type, created_at, updated_at.
- **MailingFilter :** id, name, model_id, mailing_domain (sérialisé), mailing_id (optionnel), created_at, updated_at.
- **CampaignUtm :** id, name, user_id, tag_ids, created_at, updated_at ; métriques agrégées (revenues, quotations, opportunities, clicks) calculées ou lues depuis CRM/Sales selon contrats.

---

## 3. API et contrats (résumé)

- **MailingCampaignOperator :** `create_mailing`, `update_mailing`, `schedule_mailing`, `send_mailing`, `cancel_mailing`, `get_kpis`.
- **MailingListOperator :** `create_list`, `update_list`, `delete_list`, `add_contacts`, `remove_contacts`, `import_contacts`.
- **MailingContactOperator :** `create_contact`, `update_contact`, `subscribe`, `unsubscribe`.
- **MailingBlacklistOperator :** `add`, `remove`, `check` (liste d’emails exclus).
- **MailingTraceOperator :** `create_trace`, `update_trace` (état), `aggregate_kpis`, `report`.
- **MailingContentOperator :** `create_content`, `update_content`, `save_template`, `render_preview`.
- **MailingFilterOperator :** `create_filter`, `update_filter`, `resolve_recipients`, `count`.
- **MailingCampaignUTMOperator :** `create_campaign`, `update_campaign`, `attach_mailing`, `metrics`.
- **MailingUI / Portal :** Endpoints pour listes, mailings, campagnes, paramètres ; endpoint désabonnement (email, list_ids, add_to_blacklist).

---

## 4. Plan de développement par phases

### Phase 1 — MVP (bornage fonctionnel)

**Objectif :** Envoi d’une campagne simple à des listes figées, sans A/B test ni campagnes UTM, avec blacklist et désabonnement de base.

**Livrables :**
- MailingListOperator, MailingContactOperator, MailingBlacklistOperator (CRUD, abonnements, blacklist).
- MailingCampaignOperator (création mailing brouillon, envoi immédiat ou planification simple, états draft/in_queue/sending/done/cancel).
- MailingContentOperator (sujet, body_html, preview ; pas de builder drag-and-drop en MVP).
- MailingTraceOperator (traces sent/open/click/reply/bounce ; agrégation KPIs sur mailing).
- Intégration COG : WriteIntent, StrongFather (décision envoi), WorrySentinel (quota, blacklist).
- Portail désabonnement (unsubscribe + option blacklist).
- Pas de filtres dynamiques (Contact, Lead, Event) en MVP ; uniquement listes.
- Pas de campagnes UTM ni A/B test en MVP.

**Critères d’acceptation :**
- Création liste + contacts + blacklist ; création mailing avec liste(s) ; envoi ou planification ; exclusion blacklist ; KPIs (sent, opened, clicked, replied) ; désabonnement avec option blacklist.

**Durée estimée :** 8–12 semaines (équipe réduite).

### Phase 2 — Filtres dynamiques et campagnes UTM

**Objectif :** Ciblage par filtre (Contact, Lead/Opportunity, Event Registration, Sales Order) et regroupement sous campagnes UTM avec métriques agrégées.

**Livrables :**
- MailingFilterOperator (création filtre, résolution destinataires en lecture sur Contacts, CRM, Events, Sales).
- MailingCampaignUTMOperator (campagnes, association mailings, métriques Revenues, Quotations, Opportunities, Clicks si contrats CRM/Sales).
- MailingCampaignOperator : champ campaign_id et mailing_filter_id ; résolution destinataires via MailingFilterOperator.

**Critères d’acceptation :**
- Création mailing avec filtre dynamique ; affichage du nombre de destinataires ; envoi au filtre ; campagnes UTM avec smart buttons métriques (si données CRM/Sales disponibles).

**Durée estimée :** 4–6 semaines.

### Phase 3 — A/B test et contenu avancé

**Objectif :** A/B test (pourcentage, critère gagnant, date d’envoi final) et contenu avancé (templates, builder simple ou intégration html_builder).

**Livrables :**
- MailingCampaignOperator : champs et workflow A/B test (ab_testing_enabled, ab_testing_percentage, ab_testing_winner_selection, ab_testing_schedule_date) ; envoi en 2 temps (échantillon puis reste).
- MailingContentOperator : templates réutilisables ; optionnel : intégration builder drag-and-drop (snippets, thèmes).

**Critères d’acceptation :**
- Création de 2 versions d’un mailing ; envoi à un pourcentage ; sélection du gagnant à une date ; envoi de la version gagnante au reste ; sauvegarde et réutilisation de templates.

**Durée estimée :** 4–6 semaines.

### Phase 4 — Digest, paramètres avancés, conformité

**Objectif :** Rapport 24H (digest), paramètres (serveur dédié, 24H Stat Reports), renforcement conformité (consentement, audit, preuve désabonnement).

**Livrables :**
- Intégration digest (résumé performances mailings envoyés la veille).
- Paramètres : Mailing Campaigns (déjà en phase 2), Blacklist when unsubscribing, Dedicated Server (config SMTP dédié), 24H Stat Mailing Reports.
- Champs et processus pour consentement et audit (dates, preuves désabonnement).

**Critères d’acceptation :**
- Digest quotidien configurable ; paramètres appliqués ; traces d’audit pour désabonnement et blacklist.

**Durée estimée :** 2–4 semaines.

---

## 5. Bornage fonctionnel

**In scope (équivalent Odoo Email Marketing) :**
- Listes, contacts, abonnements, blacklist.
- Mailings (sujet, corps, preview, destinataires par liste ou filtre).
- Envoi immédiat, planification, annulation.
- A/B test (pourcentage, critère gagnant, envoi final).
- Traces et KPIs (sent, delivered, opened, clicked, replied, bounced).
- Campagnes UTM et métriques agrégées (Revenues, Quotations, Opportunities, Clicks) si contrats CRM/Sales.
- Portail désabonnement et option blacklist.
- Paramètres : campagnes, blacklist, serveur dédié, 24H reports.
- Conformité : exclusion blacklist, désabonnement en 1 clic, audit.

**Hors scope (référence Odoo) :**
- Builder drag-and-drop identique à Odoo (snippets exacts) ; un builder minimal ou une intégration tierce suffit en MVP.
- Intégration Social Media (Send SMS, Add Post, Push) : hors scope sauf si spécifié (Contrat d’équipe avec Opérateurs SMS/Social).
- Intégration Events / CRM / Sales : en scope pour ciblage et métriques campagnes ; pas de duplication de toute la logique métier Events/CRM/Sales.

---

## 6. Risques et mitigation

| Risque | Mitigation |
|--------|------------|
| Quota et limites SMTP | WorrySentinel + paramètre configurable ; message clair « Retry » si limite atteinte. |
| Données personnelles (RGPD) | Niveau Sensitive ; consentement et preuve ; audit désabonnement ; pas de rétention excessive des traces sans politique. |
| Performance (envoi massif) | File d’envoi asynchrone ; batch ; pas de blocage de l’UI. |
| Dépendances CRM/Sales/Events | Contrats d’équipe en lecture seule ; pas d’écriture sur ces modèles par Email Marketing. |
| Tracking (ouverture/clics) | Pixel et liens trackés ; gestion des bloqueurs (taux d’ouverture sous-estimé) ; documenter la limite. |

---

**Document :** Odoo Email Marketing — Guide d'Implémentation avec Bornage  
**Version :** 1.0  
**Date :** 2026-02-01
