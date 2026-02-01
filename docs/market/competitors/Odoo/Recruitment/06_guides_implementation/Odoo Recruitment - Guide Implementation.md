# Odoo Recruitment — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique** pour développer l'équivalent Recruitment dans Miyukini, avec **bornage fonctionnel**, **spécifications techniques**, et **plan de développement**.

**Références :**
- [Logique Métier](../00_logique_metier/Odoo%20Recruitment%20-%20Logique%20Metier%20Complete.md)
- [Spécifications Opérateurs](../04_specifications_miyukini/Odoo%20Recruitment%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Guide Intégration COG](../05_integration_cog/Odoo%20Recruitment%20-%20Guide%20Integration%20COG.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique (crates / modules)
- Schémas de données (Job, Applicant, Stage)
- API et contrats
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel et critères d'acceptation

**Hors scope :**
- Implémentation complète du code (sera dans les crates)
- Tests unitaires détaillés (sera dans les tests)

---

## 1. Architecture Technique

### 1.1 Structure des Crates (proposition)

```
crates/
├── miyujobs/                    # Existant ou à étendre — postes / jobs
│   └── src/
│       ├── lib.rs
│       ├── job.rs               # Modèle Job (poste)
│       └── ...
│
├── miu-recruitment-applicant/   # Candidatures et pipeline
│   ├── src/
│   │   ├── lib.rs
│   │   ├── applicant.rs         # Modèle Applicant
│   │   ├── stage.rs             # Stage, transitions
│   │   ├── sourcing.rs          # UTM, referrer
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miu-recruitment-stage/       # Stages (ou intégré dans applicant)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── stage.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyukini-recruitment-ui/    # Interface (ou intégré dans miyukini-central)
    └── ...
```

**Alternative :** Un seul crate `miu-recruitment` regroupant Job (si distinct de miyujobs), Applicant, Stage, Offer, avec sous-modules.

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Id, Logger, Clock
- `miyukini-central` : StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy
- `miyukini-admin` : Admin cell

**Kits existants :**
- `miyujobs` : Postes (si déjà utilisé pour recrutement)
- `miyuhr` : Employés (création depuis candidature)
- `miunotify` : Emails, templates
- `miucontacts` : Partenaires (candidats / employés)
- `miudocuments` ou `miumedia` : CV, pièces jointes

**Externes :**
- `serde`, `chrono`, `uuid` : Données, dates, identifiants

---

## 2. Schémas de Données

### 2.1 Modèle Job (Poste)

```rust
// Proposition schéma
pub struct Job {
    pub id: JobId,
    pub name: String,
    pub department_id: Option<DepartmentId>,
    pub address_id: Option<PartnerId>,
    pub company_id: CompanyId,
    pub employment_type: EmploymentType,
    pub no_of_recruitment: u32,
    pub recruiter_id: Option<UserId>,
    pub interviewer_ids: Vec<UserId>,
    pub alias_email: Option<String>,
    pub job_summary: Option<String>,
    pub process_details: Option<String>,
    pub interview_form_id: Option<SurveyId>,
    pub contract_template_id: Option<ContractTemplateId>,
    pub website_published: bool,
    pub active: bool,
    pub sequence: i32,
    // ... (salary range, expected_skills, mission dates, etc.)
}
```

### 2.2 Modèle Applicant (Candidature)

```rust
pub struct Applicant {
    pub id: ApplicantId,
    pub job_id: JobId,
    pub partner_name: String,
    pub email_from: Option<String>,
    pub partner_phone: Option<String>,
    pub partner_id: Option<PartnerId>,
    pub stage_id: StageId,
    pub status: ApplicantCardStatus, // InProgress, Blocked, ReadyForNextStage
    pub user_id: Option<UserId>,
    pub company_id: CompanyId,
    pub department_id: Option<DepartmentId>,
    pub degree_id: Option<DegreeId>,
    pub availability: Option<NaiveDate>,
    pub expected_salary: Option<Decimal>,
    pub proposed_salary: Option<Decimal>,
    pub salary_expected_extra: Option<String>,
    pub salary_proposed_extra: Option<String>,
    pub utm_source_id: Option<UtmSourceId>,
    pub utm_medium_id: Option<UtmMediumId>,
    pub utm_campaign_id: Option<UtmCampaignId>,
    pub referrer_id: Option<UserId>,
    pub refuse_reason_id: Option<RefuseReasonId>,
    pub offer_date: Option<NaiveDate>,
    pub day_to_offer: Option<u32>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub write_date: DateTime<Utc>,
    // ... (notes, description, tags)
}
```

### 2.3 Modèle Stage

```rust
pub struct RecruitmentStage {
    pub id: StageId,
    pub name: String,
    pub sequence: i32,
    pub fold: bool,
    pub template_id: Option<MailTemplateId>,
    pub hired_stage: bool,
    pub job_ids: Vec<JobId>,        // vide = global
    pub show_in_referral: bool,
    pub points: u32,
    pub requirements: Option<String>,
    // Libellés statuts : in_progress_label, blocked_label, ready_label
}
```

---

## 3. API et Contrats (résumé)

- **RecruitmentJob** : `job.create`, `job.update`, `job.publish`, `job.get`, `job.list`
- **RecruitmentApplicant** : `applicant.create`, `applicant.update`, `applicant.stage.move`, `applicant.refuse`, `applicant.create_employee`, `applicant.get`, `applicant.list`
- **RecruitmentStage** : `stage.create`, `stage.update`, `stage.delete`, `stage.get`, `stage.list`
- **RecruitmentOffer** : `offer.config`, `offer.expiry.check`
- **RecruitmentReporting** : `report.source`, `report.velocity`, `report.team_performance`

Tous les appels sensibles (création, modification, refus, embauche) passent par Mandat et WriteIntent / StrongFather comme décrit dans le Guide Intégration COG.

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (Bornage fonctionnel)

**Objectif :** Pipeline minimal (postes, candidatures, stages) sans intégrations lourdes.

- **Périmètre :**
  - Modèles Job, Applicant, Stage (CRUD)
  - Workflow : 6 stages par défaut, passage par glisser-déposer ou changement de stage sur fiche
  - Statuts par carte : In Progress, Blocked, Ready for Next Stage
  - Création candidature manuelle (Quick Add + formulaire complet)
  - Refus avec motif (sans obligation d’envoi email)
  - Création employé depuis candidature (intégration MiyuHR minimale)
- **Hors scope MVP :** Publication site, alias email, enquêtes, Documents, Referrals, UTM avancé, IAP (SMS, OCR)
- **Livrables :** Crates Applicant + Stage (ou 1 crate Recruitment), intégration COG (Mandats, WriteIntent, StrongFather pour refus/embauche), UI minimale (Kanban + formulaire candidat)

**Critères d’acceptation :**
- Création / modification poste et candidature
- Passage de stage et refus
- Création employé depuis candidature (avec décision StrongFather)
- Pas de régression sur les Cores

### Phase 2 — Intégrations

- **Périmètre :**
  - Mail : templates, envoi auto par stage, chatter (MiyuNotify)
  - Documents : stockage CV, affichage conditionnel (MiyuDocuments / MiyuMedia)
  - Website : publication postes, formulaire candidature (MiyuWeb)
  - Surveys : Send Interview, liaison candidature (MiyuPolls ou équivalent)
- **Livrables :** Branchements opérateurs Mail, Documents, Website, Surveys ; paramètres (Résumé Display, Send Interview Survey)

**Critères d’acceptation :**
- Envoi email automatique à l’entrée dans un stage (si template configuré)
- CV attaché et affichage optionnel sur fiche candidat
- Candidature créée depuis formulaire site
- Envoi enquête et lien avec candidature

### Phase 3 — Reporting et Referrals

- **Périmètre :**
  - UTM : champs source/medium/campaign sur candidature, rapports Source analysis, Velocity, Team performance
  - Referrals : Referred By User, stages « Show in Referrals » + Points, intégration app Referrals
- **Livrables :** Opérateur RecruitmentReporting, intégration Referrals, écrans rapports

**Critères d’acceptation :**
- Rapports par source/medium/campagne et par recruteur
- Attribution de points au référent quand candidature atteint un stage avec points

### Phase 4 — Optionnel (IAP, Alias email)

- **Périmètre :** Alias email (création candidature à réception email), optionnellement SMS et OCR CV (IAP)
- **Livrables :** Service entrée email gouverné ; extensions IAP si besoin

---

## 5. Bornage Fonctionnel et Risques

**In scope :**
- Pipeline postes → candidatures → stages ; refus ; création employé
- Intégrations Mail, Documents, Website, Surveys, Referrals, UTM (selon phases)
- Gouvernance COG (Mandats, WriteIntent, StrongFather, WorrySentinel)

**Out of scope (ou découplé) :**
- IAP (SMS, OCR) : optionnel, découplé du cœur recrutement
- Portail candidat (consultation statut) : possible en Phase 2+ si besoin
- Contrats salariaux détaillés (hr_contract_salary) : possible intégration ultérieure avec MiyuHR

**Risques et mitigation :**
- **Données sensibles (CV, salaires)** : WorrySentinel, permissions strictes, audit des accès
- **Spam / abus (formulaire site, alias email)** : validation, rate limiting, modération
- **Compatibilité stages (suppression, évolution)** : Ever Buddy, contraintes « aucun candidat dans le stage » avant suppression

---

**Document** : Odoo Recruitment — Guide Implémentation  
**Version** : 1.0  
**Date** : 2026-02-01
