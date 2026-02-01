# Odoo Referrals — Guide d'Implémentation

## Contexte

Ce document fournit un **guide d'implémentation technique** pour développer l'équivalent Referrals dans Miyukini, avec **bornage fonctionnel**, **spécifications techniques** et **plan de développement**.

**Références :**
- [Logique Métier](../00_logique_metier/Odoo%20Referrals%20-%20Logique%20Metier%20Complete.md)
- [Spécifications Opérateurs](../04_specifications_miyukini/Odoo%20Referrals%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Guide Intégration COG](../05_integration_cog/Odoo%20Referrals%20-%20Guide%20Integration%20COG.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique (crates / modules)
- Schémas de données (points, récompenses, niveaux, onboarding, achats)
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
├── miyuhr/                        # Existant — employé référent
│   └── ...
│
├── miyujobs/                      # Existant ou miyu-recruitment — postes
│   └── ...
│
├── miyu-referrals/                # Nouveau — ReferralsService
│   ├── src/
│   │   ├── lib.rs
│   │   ├── points.rs               # Solde, crédit, débit
│   │   ├── rewards.rs              # Catalogue, achat
│   │   ├── levels.rs               # Niveaux, level up
│   │   ├── share.rs                # Liens de suivi, envoi email/SMS/WhatsApp
│   │   ├── onboarding.rs          # Slides, état première utilisation
│   │   ├── reporting.rs            # Rapport referral analysis (admin)
│   │   ├── models.rs               # Reward, Level, OnboardingSlide, Friend, etc.
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-central/              # Intégration événements Recruitment → Referrals
│   └── ...
│
└── miyukini-admin/                # Configuration Referrals (optionnel)
    └── ...
```

**Alternative :** Un seul crate `miyu-referrals` avec sous-modules points, rewards, levels, share, onboarding, reporting ; dépendances vers miyuhr, miyu-recruitment (ou miyujobs), miyuweb, miunotify.

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Id, Logger, Clock
- Référence aux Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy) via miyukini-central ou interfaces partagées

**Kits existants :**
- `miyuhr` : Employé (référent)
- `miyujobs` ou crate recrutement : Postes, Candidatures, Stages (referrer_id, points par stage)
- `miyuweb` : URLs, postes publiés
- `miunotify` : Email, SMS, WhatsApp, notifications
- `miyucontacts` : Partenaires (optionnel pour responsable récompense)

**Externes :**
- `serde`, `chrono`, `uuid` : Données, dates, identifiants

---

## 2. Schémas de Données

### 2.1 Points (mouvements et solde)

```rust
pub struct PointsBalance {
    pub referrer_id: EmployeeId,
    pub total_earned: u32,
    pub total_spent: u32,
    pub to_spend: u32, // total_earned - total_spent
}

pub struct PointsMovement {
    pub id: MovementId,
    pub referrer_id: EmployeeId,
    pub kind: MovementKind, // Credit | Debit
    pub amount: u32,
    pub applicant_id: Option<ApplicantId>,
    pub stage_id: Option<StageId>,
    pub reward_id: Option<RewardId>,
    pub at: DateTime<Utc>,
}
```

### 2.2 Récompenses et achats

```rust
pub struct Reward {
    pub id: RewardId,
    pub name: String,
    pub cost: u32,
    pub company_id: Option<CompanyId>,
    pub gift_responsible_id: Option<UserId>,
    pub description: String,
    pub image_url: Option<String>,
    pub active: bool,
}

pub struct RewardPurchase {
    pub id: PurchaseId,
    pub referrer_id: EmployeeId,
    pub reward_id: RewardId,
    pub cost: u32,
    pub gift_responsible_id: Option<UserId>,
    pub at: DateTime<Utc>,
}
```

### 2.3 Niveaux

```rust
pub struct Level {
    pub id: LevelId,
    pub name: String,
    pub points_required: u32,   // total accumulé pour ce niveau
    pub image_url: Option<String>,
    pub sequence: i32,
}

pub struct UserLevel {
    pub referrer_id: EmployeeId,
    pub current_level_id: LevelId,
    pub updated_at: DateTime<Utc>,
}
```

### 2.4 Onboarding et amis (avatars)

```rust
pub struct OnboardingSlide {
    pub id: SlideId,
    pub sequence: i32,
    pub text: String,
    pub company_id: Option<CompanyId>,
    pub image_url: Option<String>,
}

pub struct OnboardingState {
    pub user_id: UserId,
    pub completed_at: Option<DateTime<Utc>>,
    pub skipped: bool,
}

pub struct ReferralFriend {
    pub id: FriendId,
    pub name: String,
    pub position: FriendPosition, // Front | Back
    pub image_url: Option<String>,
    pub dashboard_image_url: Option<String>,
}

pub struct HiredReferralAvatar {
    pub applicant_id: ApplicantId,
    pub referrer_id: EmployeeId,
    pub friend_id: FriendId,
    pub assigned_at: DateTime<Utc>,
}
```

---

## 3. API et Contrats

### 3.1 ReferralsPoints

- `balance(ctx, referrer_id) -> Result<PointsBalance>`
- `credit_for_stage(ctx, referrer_id, applicant_id, stage_id, points) -> Result<()>`
- `debit_for_reward(ctx, referrer_id, reward_id, cost) -> Result<()>`

### 3.2 ReferralsRewards

- `list_rewards(ctx, company_id?) -> Result<Vec<Reward>>`
- `get_reward(ctx, reward_id) -> Result<Reward>`
- `buy(ctx, reward_id) -> Result<RewardPurchaseId>`

### 3.3 ReferralsLevels

- `current_level(ctx, referrer_id) -> Result<(Level, Option<Level>)>` // niveau actuel + suivant
- `level_up(ctx, referrer_id) -> Result<()>`
- `progress_ring(ctx, referrer_id) -> Result<(u32, u32)>` // earned toward next, required

### 3.4 ReferralsShare

- `published_jobs(ctx) -> Result<Vec<JobCard>>`
- `build_tracking_link(ctx, referrer_id, job_id?) -> Result<String>`
- `send_job_email(ctx, job_id?, recipient_emails, subject?, body?) -> Result<()>`
- `send_job_sms(ctx, job_id, recipient_phone, body?) -> Result<()>` (optionnel)
- `send_job_whatsapp(ctx, job_id, recipient_phone, message?) -> Result<()>` (optionnel)

### 3.5 ReferralsOnboarding

- `slides(ctx, company_id?) -> Result<Vec<OnboardingSlide>>`
- `complete(ctx) -> Result<()>`
- `skip(ctx) -> Result<()>`
- `state(ctx) -> Result<OnboardingState>`

### 3.6 ReferralsReporting (admin)

- `referral_analysis(ctx, filters: DateFilter?, company_id?) -> Result<ReferralAnalysisReport>`
- `referral_analysis_pivot(ctx, filters) -> Result<PivotDataSet>` (optionnel)
- Export / Insert in Spreadsheet (optionnel)

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (2–3 semaines)

- **Points** : modèle PointsBalance, PointsMovement ; crédit manuel ou via événement Recruitment (stage changé) ; débit à l’achat.
- **Rewards** : CRUD récompenses (admin), liste, achat (vérification solde + débit + enregistrement achat + notification responsable).
- **Levels** : config niveaux (admin), calcul niveau actuel, level up (sans déduction).
- **Dashboard UI** : résumé points (earned / to spend), niveau, anneau, boutons Referrals / Ongoing / Successful (liste basique), View Jobs (liste postes publiés), Rewards (liste + Buy).
- **Intégration** : MiyuHR (référent), MiyuRecruitment (referrer_id sur candidature, événement stage → crédit points), MiyuWeb (postes publiés, lien de suivi basique).

### Phase 2 — Partage et Onboarding (1–2 semaines)

- **Share** : génération lien de suivi (token signé), envoi email (template), optionnel SMS/WhatsApp.
- **Onboarding** : slides (CRUD admin), état completed/skipped, affichage conditionnel dashboard vs onboarding.
- **My Referrals** : liste des candidatures parrainées (ongoing + successful), badges, barre de progression, stages avec points.

### Phase 3 — Gamification et Reporting (1–2 semaines)

- **Hired** : écran choix avatar pour parrainé embauché ; modèle Friends (avatars), HiredReferralAvatar.
- **Levels / Friends** : configuration avancée (images, position Front/Back), affichage avatars sur dashboard et cartes parrainages.
- **Reporting** : Employees Referral Analysis (bar chart par medium, filtres), vue Pivot (par référent, embauchés/total). Droits : Administrator uniquement.
- **Alerts** : configuration alertes et fond dashboard (admin).

### Phase 4 — Optionnel

- Insert in Spreadsheet (intégration Documents/Spreadsheet).
- Personnalisation avancée des templates email/SMS/WhatsApp.
- Statistiques côté référent (classement anonyme, objectifs).

---

## 5. Bornage Fonctionnel et Critères d'Acceptation

**In scope :**
- Gestion des points (crédit par stage, débit à l’achat, solde total earned / to spend).
- Catalogue récompenses et achat avec notification responsable.
- Niveaux et level up (affichage, pas de coût en points).
- Partage de postes (liste publiée, lien de suivi, envoi email).
- Liste « Mes parrainages » (ongoing, successful, points par stage).
- Onboarding (slides, Skip, Start Now).
- Écran Hired (choix avatar pour parrainé embauché).
- Reporting admin (referral analysis par canal et par référent).
- Configuration (récompenses, niveaux, onboarding, amis, alertes) réservée aux Administrators.

**Out of scope (MVP) :**
- Envoi SMS/WhatsApp (dépendance IAP/WhatsApp) peut être simulé ou reporté.
- Insert in Spreadsheet (dépendance Documents/Spreadsheet).
- Portail candidat (hors Referrals).

**Critères d’acceptation (exemples) :**
- Un référent voit son solde et peut acheter une récompense si solde suffisant ; après achat, solde décrémenté et responsable notifié.
- Un candidat parrainé qui passe au stage « First Interview » crédite 20 points au référent (config stage).
- Un référent qui atteint le seuil du niveau suivant peut level up sans perdre de points.
- Un admin accède au rapport Referral Analysis filtré par date et voit les données par medium et par référent (Pivot).

---

## 6. Risques et Mitigation

| Risque | Mitigation |
|--------|------------|
| Désynchronisation points (double crédit ou oubli) | Événements idempotents (clé applicant_id + stage_id), traitement unique par événement |
| Liens de suivi falsifiables | Token signé (HMAC ou JWT), TTL court, validation côté formulaire candidature |
| Données personnelles dans le reporting | Accès strict Administrator, audit des accès, agrégation minimale |
| Dépendance forte à Recruitment | Contrat d’interface clair (événements, champs referrer_id, points par stage) ; Referrals peut tourner avec des données mock si besoin |

---

**Document** : Odoo Referrals — Guide d'Implémentation  
**Version** : 1.0  
**Date** : 2026-02-01
