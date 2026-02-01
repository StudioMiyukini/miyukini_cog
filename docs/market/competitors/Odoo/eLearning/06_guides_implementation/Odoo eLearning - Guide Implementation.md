# Odoo eLearning — Guide d'Implémentation

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent eLearning (LMS) dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée
- Spécifications des crates Rust (ou modules)
- Schémas de données (cours, contenu, section, inscription, certification, groupes, tags)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des crates (proposition)

```
crates/
├── miyuelearning-course/           # eLearningCourseOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── course.rs                # Modèle Course, sections
│   │   ├── options.rs               # Options, karma, prérequis
│   │   ├── publication.rs           # Published / Unpublished
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuelearning-content/           # eLearningContentOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── content.rs               # Modèle Content (Image, Article, Document, Video, Quiz)
│   │   ├── quiz.rs                  # Questions, réponses, karma
│   │   ├── resources.rs             # Additional Resources
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuelearning-enrollment/        # eLearningEnrollmentOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── enrollment.rs            # Inscription (open, invitation, payment)
│   │   ├── progress.rs             # Progression (contenus complétés)
│   │   ├── karma.rs                 # Karma (récompenses, seuils)
│   │   ├── invite.rs                # Lien invitation, email
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuelearning-group/             # eLearningGroupOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── group.rs                 # Course Groups
│   │   ├── tag.rs                   # Tags cours, Content Tags
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuelearning-certification/      # eLearningCertificationOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── certification.rs        # Liaison survey, délivrance
│   │   ├── result.rs                # Résultat, karma
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyuelearning-ui/                # eLearningUI
    ├── src/
    │   ├── lib.rs
    │   ├── backend/
    │   │   ├── courses.rs           # Liste, formulaire cours
    │   │   ├── contents.rs          # Liste, formulaire contenu
    │   │   ├── config.rs            # Course Groups, Content Tags, Settings
    │   │   └── go_to_website.rs     # Lien front-end
    │   ├── frontend/
    │   │   ├── catalog.rs           # All Courses, filtres
    │   │   ├── course_page.rs       # Page cours, inscription
    │   │   ├── content_page.rs      # Page contenu (leçon, quiz)
    │   │   ├── certification_page.rs # Passage certification
    │   │   └── publish_switch.rs    # Published / Unpublished
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Kits existants :**
- `miyunotify` : Notifications (New Content, Completion), invitations, Contact Attendees
- `miyumedia` : Stockage blobs (images, documents, vidéos si hébergement)
- `miyucontacts` : Responsables, utilisateurs
- `miyuweb` : Pages catalogue, cours, contenus ; Article = page
- `miyusurveys` : Certifications (questionnaires, résultats)
- `miustore` / `miyubilling` : Produits type Course, paiement
- `miuforum` : Forum dédié par cours (optionnel)
- `miyucms` : Contenu type Article (optionnel si distinct de MiyuWeb)
- `miyuclock` : Durées, dates (progression, délais)

---

## 2. Schémas de données

### 2.1 Modèle Course

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: CourseId,
    pub title: String,
    pub tag_ids: Vec<TagId>,
    pub image_attachment_id: Option<AttachmentId>,
    pub responsible_id: UserId,
    pub website_id: Option<WebsiteId>,
    pub description: Option<String>,
    pub show_course_to: ShowCourseTo,       // Everyone | SignedIn | CourseAttendees
    pub enroll_policy: EnrollPolicy,        // Open | OnInvitation | OnPayment
    pub enroll_message: Option<String>,
    pub product_id: Option<ProductId>,      // si OnPayment
    pub prerequisite_course_ids: Vec<CourseId>,
    pub display_mode: DisplayMode,          // Training | Documentation
    pub featured_content_ids: Vec<ContentId>,
    pub allow_reviews: bool,
    pub forum_id: Option<ForumId>,
    pub new_content_notification_template_id: Option<TemplateId>,
    pub completion_notification_template_id: Option<TemplateId>,
    pub karma_review: u32,
    pub karma_finish: u32,
    pub karma_add_review: u32,
    pub karma_add_comment: u32,
    pub karma_vote: u32,
    pub published: bool,
    pub company_id: CompanyId,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
```

### 2.2 Modèle Section

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseSection {
    pub id: SectionId,
    pub course_id: CourseId,
    pub name: String,
    pub sequence: u32,
}
```

### 2.3 Modèle Content

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub id: ContentId,
    pub course_id: CourseId,
    pub section_id: Option<SectionId>,
    pub title: String,
    pub content_type: ContentType,  // Image | Article | Document | Video | Quiz
    pub responsible_id: UserId,
    pub duration_minutes: Option<u32>,
    pub allow_preview: bool,
    pub allow_download: bool,
    pub description: Option<String>,
    pub sequence: u32,
    pub published: bool,
    pub public_views: u64,
    pub total_views: u64,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

// Variantes par type : image_url, article_page_id, document_attachment_id, video_url, quiz_id
```

### 2.4 Modèle Enrollment

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: EnrollmentId,
    pub course_id: CourseId,
    pub user_id: UserId,
    pub source: EnrollmentSource,   // Open | Invitation(InviteId) | Payment(PaymentId)
    pub enrolled_at: Timestamp,
    pub progress: Progress,         // contenus complétés, certification passée
    pub karma_balance: u32,
}
```

### 2.5 Modèle Certification

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certification {
    pub id: CertificationId,
    pub course_id: CourseId,
    pub user_id: UserId,
    pub survey_result_id: SurveyResultId,
    pub delivered_at: Timestamp,
}
```

### 2.6 Modèles Group et Tag

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseGroup {
    pub id: CourseGroupId,
    pub name: String,
    pub menu_entry: bool,
    pub tag_ids: Vec<TagId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentTag {
    pub id: ContentTagId,
    pub name: String,
}
```

---

## 3. API et contrats par Opérateur

### 3.1 eLearningCourseOperator

- `create_course(intent, mandate) -> Result<Course>`
- `update_course(course_id, payload, mandate) -> Result<Course>`
- `delete_course(course_id, mandate) -> Result<()>`
- `publish_course(course_id, publish, mandate) -> Result<Course>`
- `get_course(course_id, mandate) -> Result<Course>`
- `list_sections(course_id, mandate) -> Result<Vec<CourseSection>>`
- `add_section(course_id, name, sequence, mandate) -> Result<CourseSection>`
- `reorder_contents(course_id, section_id, content_ids, mandate) -> Result<()>`

### 3.2 eLearningContentOperator

- `create_content(intent, mandate) -> Result<Content>`
- `update_content(content_id, payload, mandate) -> Result<Content>`
- `delete_content(content_id, mandate) -> Result<()>`
- `publish_content(content_id, publish, mandate) -> Result<Content>`
- `get_content(content_id, mandate) -> Result<Content>`
- `add_quiz_question(content_id, question, mandate) -> Result<QuizQuestion>`
- `record_view(content_id, user_id, is_public, mandate) -> Result<()>`

### 3.3 eLearningEnrollmentOperator

- `enroll(user_id, course_id, source, mandate) -> Result<Enrollment>`
- `invite(course_id, email, copy_link, send_email, enroll_message, mandate) -> Result<InviteResult>`
- `list_attendees(course_id, mandate) -> Result<Vec<Enrollment>>`
- `update_progress(enrollment_id, content_id, completed, mandate) -> Result<Enrollment>`
- `grant_karma(user_id, course_id, reason, points, mandate) -> Result<()>`
- `can_access(user_id, course_id, mandate) -> Result<AccessLevel>`

### 3.4 eLearningGroupOperator

- `create_group(name, menu_entry, tag_ids, mandate) -> Result<CourseGroup>`
- `create_content_tag(name, mandate) -> Result<ContentTag>`
- `list_groups(mandate) -> Result<Vec<CourseGroup>>`
- `list_content_tags(mandate) -> Result<Vec<ContentTag>>`

### 3.5 eLearningCertificationOperator

- `link_certification(course_id, survey_id, mandate) -> Result<()>`
- `pass_certification(course_id, user_id, survey_answers, mandate) -> Result<SurveyResult>`
- `deliver_certification(course_id, user_id, survey_result_id, passed, attempts, mandate) -> Result<Certification>`

---

## 4. Plan de développement par phases

### Phase 1 — MVP (2–3 sprints)

**Objectif :** Catalogue de cours, création cours/contenus (back-end), publication, inscription Open, parcours Training.

- [ ] Crates : miyuelearning-course, miyuelearning-content, miyuelearning-group, miyuelearning-ui (backend + frontend catalogue et page cours/contenu)
- [ ] Modèles : Course, Section, Content (types Image, Document, Video uniquement), CourseGroup, Tag, ContentTag
- [ ] Inscription : EnrollPolicy Open uniquement ; Show course to Everyone / SignedIn
- [ ] Publication : switch Published sur cours et contenu (front-end)
- [ ] Pas de certification, pas de quiz, pas de Paid Courses, pas de Forum/Mailing
- [ ] Intégration MiyuWeb : pages All Courses, page cours, page contenu

### Phase 2 — Inscriptions et karma (1–2 sprints)

- [ ] Crate miyuelearning-enrollment
- [ ] EnrollPolicy : On Invitation (lien, email via MiyuNotify)
- [ ] EnrollPolicy : On Payment (MiyuStore, produit type Course)
- [ ] Progression : enregistrement contenus complétés
- [ ] Karma : récompenses Review/Finish, seuils Add Review/Add Comment/Vote
- [ ] Contact Attendees (MiyuNotify) si Mailing activé

### Phase 3 — Quiz et certification (2 sprints)

- [ ] Content type Quiz : questions, réponses, Is correct answer, Comment, récompenses karma par tentatives
- [ ] Crate miyuelearning-certification
- [ ] Intégration MiyuSurveys : liaison survey ↔ cours, passage certification, délivrance
- [ ] Affichage certification et karma sur le front-end

### Phase 4 — Complet (1–2 sprints)

- [ ] Content type Article (page MiyuWeb / MiyuCMS)
- [ ] Display mode Documentation (ordre libre, Featured Content)
- [ ] Prérequis (prerequisite_course_ids)
- [ ] Forum dédié (MiyuForum)
- [ ] Notifications (New Content, Completion) via MiyuNotify
- [ ] Multi-website (website_id sur cours)
- [ ] Paramètres Settings (Certifications, Paid Courses, Mailing, Forum) en configuration
- [ ] Statistiques (# Public Views, # Total Views) et rapports

---

## 5. Bornage fonctionnel et critères d'acceptation

### 5.1 MVP (Phase 1)

| Critère | Acceptation |
|---------|-------------|
| Création cours | Titre, tags, image, sections, options (Show course to, Display Training) |
| Création contenu | Types Image, Document, Video ; durée, Allow Preview |
| Publication | Switch Published sur cours et contenu (front-end) ; visibilité selon statut |
| Catalogue | Page All Courses avec cartes, filtres par Course Groups |
| Inscription Open | Bouton Join sur page cours ; création Enrollment |
| Parcours Training | Liste sections/contenus dans l’ordre ; accès aux leçons |
| Back-end | Menus Courses (Courses, Contents), Configuration (Course Groups, Content Tags) |

### 5.2 Phase 2

| Critère | Acceptation |
|---------|-------------|
| Inscription On Invitation | Lien généré, email envoyé (MiyuNotify) ; inscription via lien |
| Inscription On Payment | Produit Course acheté → Enrollment créé automatiquement |
| Progression | Marquer contenu complété ; affichage barre de progression |
| Karma | Attribution Review/Finish ; vérification seuils Add Review/Add Comment/Vote |
| Contact Attendees | Liste inscrits, envoi mailing (MiyuNotify) |

### 5.3 Phase 3

| Critère | Acceptation |
|---------|-------------|
| Quiz | Questions, réponses, feedback, récompenses karma par tentatives |
| Certification | Survey liée au cours ; passage questionnaire ; délivrance si succès ; karma |
| Affichage | Certification et karma visibles sur le profil / page cours |

### 5.4 Phase 4

| Critère | Acceptation |
|---------|-------------|
| Article | Contenu type Article = page éditable (MiyuWeb/MiyuCMS) |
| Documentation | Ordre libre, Featured Content sur page cours |
| Prérequis | Cours B exige Cours A ; déblocage après complétion A |
| Forum | Forum dédié par cours (MiyuForum) |
| Notifications | New Content, Completion (MiyuNotify) |
| Multi-website | Cours restreint à un site (website_id) |
| Settings | Activation/désactivation Certifications, Paid Courses, Mailing, Forum |

---

**Document créé le :** 2026-02-01  
**Version :** 1.0
