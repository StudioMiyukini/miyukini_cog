# JayFestival — Référence UI : transcription Catakana → stack actuelle

## Contexte

Ce document **retranscrit l’UI complète de Catakana** (Atomic Design, thème, ui-kit, écrans) dans la **stack actuelle** Miyukini : **Dioxus** (Rust), avec référence au thème existant (miyukini-central `theme.rs`, `pixel_theme.rs`) et à la Miyukini UI Library. Il sert de **référence pour l’implémentation** de JayFestival et complète l’[Audit documentation Catakana](./JayFestival%20-%20Audit%20Documentation%20Catakana.md) et le [Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md).

**Sources Catakana** : `.Catakana/docs/ATOMIC_THEME_GUIDE.md`, `.Catakana/docs/reference/UI_ARCHITECTURE.md`, `.Catakana/src/components/` (atoms, molecules, organisms, theme, ui shadcn), `sectionsConfig.ts`, `adminCategoriesConfig.tsx`.

**Conformité** : L'implémentation doit respecter la [Spécification UI conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) : protocoles obligatoires, spécifications détaillées de chaque atome/molécule/organisme, parcours par écran avec composants ordonnés. Ce document (Reference UI) fournit le mapping global ; la spec fournit les bornes et les règles de conformité.

**Stack cible** : [Miyukini - Stack UI Dioxus](../../ux_ui/Miyukini%20-%20Stack%20UI%20Dioxus%20Dioxus.md) ; `crates/miyukini-central/src/theme.rs`, `pixel_theme.rs`, `services/ui_library.rs`.

## Portée / Scope

- **Périmètre** : Transcription Atomic UI, tokens de thème, ui-kit (composants), mapping écrans Catakana → écrans JayFestival (Dioxus), checklist d’implémentation.
- **Hors périmètre** : Code source React/TypeScript Catakana ; implémentation détaillée des widgets Dioxus (référence docs Dioxus et miyukini-central).

---

## 1. Principes et stack

### 1.1 Principes Catakana à conserver

| Principe | Catakana | JayFestival / Dioxus |
|----------|----------|---------------------|
| **Source unique des styles** | `useActiveTheme()` → tokens (couleurs, rayons, ombres, spacing, opacités) | Struct `JayFestivalTheme` (ou extension `theme.rs`) ; appliquer via `ctx.set_style()` et helpers. |
| **Opacité des fonds** | Fonds principaux ~40 % opacité | `Dioxus::Color32::from_rgba_unmultiplied(r, g, b, 102)` (255*0.4≈102) pour fonds section/carte. |
| **Atomic design** | Atoms → Molecules → Organisms → Templates → Pages | Équivalent Dioxus : **widgets de base** (bouton, label, champ) → **composants composés** (Frame + label + bouton) → **panels/sections** (SidePanel + liste + cartes) → **layout page** (TopBottomPanel + SidePanel + CentralPanel) → **écran** (état + layout + sections). |
| **Tokens UI** | Jamais de couleurs/tailles en dur | Toutes les couleurs, rayons, espacements, polices viennent du thème (struct ou `Dioxus::Style`). |

### 1.2 Stack actuelle (rappel)

- **UI** : Dioxus 0.33, Dioxus 0.33 (Rust).
- **Layout** : `CentralPanel`, `SidePanel`, `TopBottomPanel`, `Window`, `Area`.
- **Widgets** : `ui.button()`, `ui.label()`, `ui.heading()`, `TextEdit`, `checkbox`, `selectable_value`, `collapsing`, `horizontal` / `vertical`, etc.
- **Style** : `ctx.set_style()`, `ctx.set_visuals()` ; `Dioxus::Style`, `Dioxus::Visuals`, `Dioxus::WidgetVisuals` (corner_radius, bg_fill, …).
- **Thème existant** : `theme.rs` (store : accent, card radius, spacing), `pixel_theme.rs` (Chrome tabs).

---

## 2. Thème — tokens Catakana → Dioxus / JayFestival

### 2.1 Noms de tokens (Catakana) → champs Dioxus / struct

| Catakana (theme) | Type / usage | Dioxus / JayFestival |
|------------------|--------------|---------------------|
| `colors.background.primary` | Fond général | `Visuals::window_fill()` ou couleur fond CentralPanel ; avec opacité 0.4 si overlay. |
| `colors.section.background` | Fond section | `Frame::fill(card_bg_color(dark))` ; opacité 0.4 recommandée. |
| `colors.section.card.background` | Fond carte | Idem `theme.rs` `card_bg_color(dark)`. |
| `colors.section.border` | Bordure section/carte | `Frame::stroke(Stroke::new(1., section_border_color()))`. |
| `colors.section.title` | Titre section | `ui.heading()` ou `RichText::new().color(section_title_color())`. |
| `colors.section.description` | Description | `ui.label()` avec couleur secondaire. |
| `colors.navigation.container.background` | Barre navigation | `SidePanel::default().frame(Frame::fill(nav_bg))`. |
| `colors.navigation.button.*` | Bouton nav (normal, hover, active) | `WidgetVisuals::inactive/hovered/active.bg_fill`. |
| `colors.header.*` | Logo, titre header | `TopBottomPanel` fill + texte avec `header_title_color()`. |
| `colors.sidebar.*` | Sidebar, menuItem | `SidePanel` fill, stroke ; `selectable_value` avec bg_fill actif. |
| `fonts.sizes.*` | Tailles texte | `Dioxus::FontId::new(size, FontFamily::Proportional)`. |
| `fonts.weights.*` | Graisse | Non utilisé tel quel en Dioxus (pas de font-weight) ; différencier par `FontId::new(size_large, …)` si besoin. |
| `borders.radius.small/medium/large` | Rayon coins | `CornerRadius::same(4)` small, `8` medium, `12` large (u8). |
| `spacing.*` | Marges, padding | `style.spacing.item_spacing`, `button_padding`, `window_margin` ; `Margin::same(n)`. |
| `shadows.small/medium/large` | Ombres | Dioxus n’a pas de box-shadow ; simuler par `Frame::stroke` légère ou accepter l’absence. |
| `opacity.overlay/cardBackground` | Opacités | Alpha dans `Color32::from_rgba_unmultiplied(r,g,b,alpha)`. |

### 2.2 Palette Catakana → couleurs JayFestival (Dioxus)

| Usage Catakana | Couleur / valeur | Dioxus Color32 (exemple) |
|----------------|------------------|-------------------------|
| **Catakana Purple** (primaire) | `#8B5CF6` | `Color32::from_rgb(139, 92, 246)` |
| **Catakana Blue** (secondaire) | `#3B82F6` | `Color32::from_rgb(59, 130, 246)` |
| **Amber** (bénévoles) | `#F59E0B` | `Color32::from_rgb(245, 158, 11)` |
| **Green** (exposants) | `#10B981` | `Color32::from_rgb(16, 185, 129)` |
| **Fond section (dark)** | rgba + 0.4 | `from_rgba_unmultiplied(30, 30, 35, 102)` |
| **Fond carte (dark)** | rgba + 0.05–0.1 | `from_rgba_unmultiplied(50, 50, 55, 25)` |

Recommandation : définir un module `jay_festival_theme.rs` (ou étendre `theme.rs`) avec des fonctions du type `section_bg(dark: bool)`, `card_bg(dark: bool)`, `accent_primary()`, `accent_organisateur()`, `accent_exposant()`, `accent_visiteur()`, et appliquer dans les écrans JayFestival.

### 2.3 Responsive / breakpoints

Catakana : breakpoint 800px, 14px mobile / 16px+ desktop. En Dioxus : utiliser `ctx.screen_rect().width()` pour choisir layout (sidebar large vs icônes seules), et `FontId::new(14., …)` vs `16.` selon largeur.

---

## 3. Atomic UI — Catakana → Dioxus

### 3.1 Atoms (éléments de base)

| Catakana | Fichier / usage | Équivalent Dioxus |
|----------|------------------|------------------|
| **IconWrapper** | `atoms/IconWrapper.tsx` (Lucide, variantes couleur, taille sm/md/lg) | Pas d’icônes Lucide en Dioxus ; utiliser `ui.label("📅")` (emoji) ou intégrer une font d’icônes (Dioxus_extras) ; taille = `FontId::new(14.|16.|20., …)`. |
| **Button** | shadcn `button.tsx` | `ui.button(RichText::new("Label").color(text_color()))` ; style via `ctx.style().visuals.widgets`. |
| **Input** | shadcn `input.tsx` | `ui.add(Dioxus::TextEdit::singleline(&mut string))`. |
| **Label** | shadcn `label.tsx` | `ui.label()` ou `ui.heading()`. |
| **Badge** | shadcn `badge.tsx`, `ExhibitorStatusBadge` | `Frame::group(ui).fill(badge_bg).show(ui, \|ui\| ui.label("Validé"))`. |
| **Checkbox** | shadcn `checkbox.tsx` | `ui.checkbox(&mut bool, "Label")`. |
| **Select** | shadcn `select.tsx` | `Dioxus::ComboBox::from_id_salt("id").selected_text(...).show_ui(ui, \|ui\| { ui.selectable_value(...) })`. |

### 3.2 Molecules (combinaisons)

| Catakana | Fichier / usage | Équivalent Dioxus |
|----------|------------------|------------------|
| **FeatureCard** | `molecules/FeatureCard.tsx` (titre, description, icône, variante) | `Frame::card(ui).show(ui, \|ui\| { ui.heading("Titre"); ui.label("Description"); })` avec fill/stroke du thème. |
| **DirectoryCard** | `molecules/DirectoryCard.tsx` (gradient, CTA) | Idem + bouton en bas ; gradient en Dioxus = dégradé manuel ou fill uniforme. |
| **RoleCard** | `molecules/RoleCard.tsx` (pastille colorée) | Frame + petit cercle/rect coloré + label. |
| **CTACard** | `molecules/CTACard.tsx` | Frame + titre + description + `ui.button()`. |
| **Card** (shadcn) | `ui/card.tsx` | `Frame::default().fill(card_bg).stroke(...).rounding(...).show(ui, ...)`. |

### 3.3 Organisms (sections complètes)

| Catakana | Fichier / usage | Équivalent Dioxus |
|----------|------------------|------------------|
| **Header** | `organisms/Header.tsx` (nav responsive, auth) | `TopBottomPanel::top("header").show(ctx, \|ui\| { ui.horizontal(\|ui\| { ui.label("JayFestival"); ui.button("Catalogue"); ... }) })`. |
| **HeaderWithEdition** | Avec sélecteur d’édition | Idem + `ComboBox` édition. |
| **HeroSection** | `organisms/HeroSection.tsx` | `CentralPanel` ou zone avec titre grand + sous-texte. |
| **FeaturesGrid** | `organisms/FeaturesGrid.tsx` (onglets + grille FeatureCard) | `ui.horizontal` pour onglets (selectable_value) ; `ui.grid()` ou boucle vertical/horizontal pour cartes. |
| **DirectoryBanner** | `organisms/DirectoryBanner.tsx` | Deux `Frame::card` côte à côte. |
| **RolesGrid** | `organisms/RolesGrid.tsx` | Grille de RoleCard (4 rôles). |
| **CTASection** | `organisms/CTASection.tsx` | Grille de CTACard + bouton principal. |
| **Layout** | `Layout.tsx` (structure page) | `SidePanel::left` + `CentralPanel::default()` ; contenu central = scroll ou panels. |
| **GestionLayout** | `layouts/GestionLayout.tsx` | Idem avec menu admin (catégories/sections). |

### 3.4 Templates / Pages

- **Catakana** : templates = layout (header, sidebar, body, footer) ; pages = assemblage + données.
- **Dioxus** : même idée — une fonction `fn ui_organisateur_dashboard(ctx, app_state)` qui appelle `ui_sidebar()`, puis dans `CentralPanel` `ui_edition_list()` ou `ui_edition_dashboard(edition_id)` ; chaque « page » = une branche de `match current_view { ... }`.

---

## 4. UI-kit — composants Catakana (shadcn + custom) → spec implémentation

Liste non exhaustive ; chaque entrée donne le **nom**, l’**usage** et la **cible Dioxus**.

| Composant Catakana | Usage | Implémentation Dioxus / référence |
|--------------------|--------|----------------------------------|
| **button** | Primaire, secondaire, outline, ghost | `ui.button()` + `ctx.style().visuals.widgets` (inactive/hovered/active). |
| **input** | Texte une ligne | `TextEdit::singleline()`. |
| **textarea** | Texte multiligne | `TextEdit::multiline()`. |
| **label** | Libellé champ | `ui.label()`. |
| **card** | Conteneur avec titre/corps/footer | `Frame::card().inner_margin(...).show(ui, ...)`. |
| **badge** | Statut (ex. exposant validé/refusé) | Frame petit + texte ; couleur selon statut. |
| **table** | Listes tabulaires (exposants, budget) | `Dioxus::Grid` ou `ui.horizontal` répété ; en-têtes en `ui.heading()` ou première ligne en gras. |
| **tabs** | Onglets (dashboard édition) | `ui.horizontal` + `selectable_value` ou `Dioxus::TopBottomPanel` avec boutons. |
| **dialog** | Modale | `Window::new("Titre").anchor(Align2::CENTER_CENTER, [0.,0.]).show(ctx, \|ui\| ...)`. |
| **dropdown-menu** | Actions | `ui.menu_button("Actions", \|ui\| { if ui.button("Exporter").clicked() { ... } })`. |
| **select** | Liste déroulante | `ComboBox::from_id_salt(...).selected_text(...).show_ui(...)`. |
| **checkbox** | Booléen | `ui.checkbox()`. |
| **calendar** | Date (programme, filtres) | Dioxus_extras `DatePicker` ou champ texte + parsing. |
| **breadcrumb** | Fil d’Ariane (Mes éditions > Nom édition) | `ui.horizontal` avec `ui.link("Mes éditions")` + `ui.label(" > ")` + `ui.label(nom_edition)`. |
| **pagination** | Liste paginée | `ui.horizontal` avec boutons « Précédent » / « Suivant » + label « Page n / N ». |
| **ExhibitorStatusBadge** | Badge statut exposant | Frame coloré (vert/jaune/rouge) + texte (Validé / En attente / Refusé). |
| **DocumentViewer** | Affichage document (PDF/texte) | Lien téléchargement ou iframe non disponible en Dioxus ; afficher métadonnées + bouton « Télécharger ». |
| **FloorPlanCanvas** | Plan de salle interactif (Fabric.js) | Dioxus : dessin personnalisé avec `Painter` (rectangles, texte) ou intégration widget 2D ; drag & drop = `Response::dragged()` + mise à jour positions. |
| **ScheduleGrid** | Grille programme (créneaux × salles) | `ui.grid()` ou tableau avec `ui.label()` par cellule ; clic = ouvrir fenêtre édition animation. |

Référence complémentaire : **Miyukini UI Library** (`crates/miyukini-central/src/services/ui_library.rs`) pour boutons, cartes, champs, barres — réutiliser ou aligner les styles (padding, corner_radius, couleurs) sur le thème JayFestival.

---

## 5. Mapping écrans Catakana → écrans JayFestival (Dioxus)

### 5.1 Par public

Les écrans JayFestival sont décrits dans les documents « Écrans et cycle » (Organisateurs, Exposants, Visiteurs, UNC). Le tableau ci-dessous fait le lien avec les **composants / sections Catakana** pour la transcription.

#### 5.1.1 Utilisateur non connecté (catalogue)

| Écran JayFestival | Id | Composants / sections Catakana |
|-------------------|-----|-------------------------------|
| Landing / Accueil catalogue | UNC-E01 | `HomePage`, `HeroSection`, `FeaturesGrid`, `DirectoryBanner`, `CTASection`, navigation publique. |
| Liste des événements | UNC-E04–E06 | Liste + filtres (category2 Informations, Programme, Plan, Exposants). |
| Fiche événement | UNC-E07 | Section Informations + Programme + Plan + Exposants + Règlements (category2). |
| Répertoire organisateurs / exposants | UNC-E10–E14 | Annuaire exposants (category7 section2), fiches. |

#### 5.1.2 Organisateurs

| Écran JayFestival | Id | Composants / sections Catakana |
|-------------------|-----|-------------------------------|
| Connexion / Inscription organisateur | ORG-E02, ORG-E03 | `LoginModal`, `SignupModal` (auth) ; formulaire structure. |
| Tableau de bord organisateur | ORG-E04 | `Layout` + sidebar admin ; vue synthèse éditions (cartes ou liste). |
| Liste des éditions | ORG-E05 | Grille/liste type `EditionArchive` ou liste avec filtres. |
| Dashboard édition | ORG-E07 | Onglets type `GestionLayout` : Exposants, Plan, Programme, Budget, Documents, etc. |
| Liste exposants / Candidatures | ORG-E09, ORG-E10 | `EditionExhibitors`, `EditionCandidaturesList` ; table + badges statut. |
| Fiche exposant / Devis / Factures | ORG-E11–E13 | `ExhibitorModal`, formulaires devis/facture (Miyuinvoice). |
| Plan de salle / Attribution | ORG-E14–E16 | `FloorPlanCanvas`, `ExhibitorBoothAssignment`, `FloorPlanItemsList`, `FloorPlanToolbar`. |
| Programme | ORG-E17a–E17b | `EditionAgenda`, `ProgramSection`, `ScheduleGrid`, `EventCreatorModal`, `SlotEditorModal`. |
| Budget | ORG-E19 | `EditionBudget`, `BudgetTable`, `BudgetCharts`, `BudgetEntryForm`. |
| Documents et légal | ORG-E22 | `EditionDocuments`, `DocumentViewer`. |
| Annonces et notifications | ORG-E23 | Section communication (config + liste envois). |
| Services visiteur | ORG-E24 | Formulaire activation (jeux, concours, ateliers, pass). |
| Publication / Clôture | ORG-E25 | Boutons + confirmation. |
| Équipe | ORG-E21 | Liste membres + invitations (rôles Admin, Manager, Bénévole). |

#### 5.1.3 Exposants

| Écran JayFestival | Id | Composants / sections Catakana |
|-------------------|-----|-------------------------------|
| Dashboard exposant | EXP-E01–E04 | Layout type « espace exposant » : onglets Candidatures, Participations, Agenda, Documents, Factures. |
| Candidatures / Dépôt | EXP-E05–E06 | Formulaire candidature (champs dynamiques) + upload (Dioxus : chemin fichier ou base64 si limité). |
| Participations / Fiche édition | EXP-E07–E08 | Liste éditions validées + fiche détail (plan, programme, documents). |
| Agenda / Conflits | EXP-E09 | Vue calendrier (données JayKoa) + alerte conflit. |
| Documents / Factures | EXP-E10–E12 | Liste + téléchargement (lien ou métadonnées). |

#### 5.1.4 Visiteurs

| Écran JayFestival | Id | Composants / sections Catakana |
|-------------------|-----|-------------------------------|
| Espace visiteur | VIS-E04–E14 | Layout : Agenda, Billets, Réservations, Pass VIP, Activités (jeux, concours). |
| Réservation (atelier, créneau, pass) | VIS-E09 | Formulaire choix créneau + vérification conflit (JayKoa) + confirmation. |
| Fiche événement (connecté) | VIS-E10 | Même contenu que UNC fiche événement + CTAs (Réserver, S’inscrire). |

### 5.2 Catégories / sections Catakana (config) → usage JayFestival

- **category1** (News) → Annonces organisateur + optionnel phase 2 Actualités public.
- **category2** (Informations, Programme, Plan, Exposants, Règlements) → Catalogue public + dashboard édition (Programme, Plan, Exposants, Documents).
- **category3** (Invités, Ateliers, Animations, Concours, Jeux) → Services visiteur (phase 2 complet) ; MVP = réservations ateliers/créneaux.
- **category4** (Plans) → Plan de salle (organisateur + exposant + public).
- **category5** (RPG, Inventaire, Récompenses, Leaderboard) → Hors scope JayFestival v1.
- **category6** (Galeries) → Phase 2 ou Miyumedia.
- **category7** (Organisation : Annuaire, Emplacements, Réservation stands, Présentation) → Exposants (annuaire, réservation) + catalogue.
- **category8** (Gestion : appli, compte, candidatures, éditions, utilisateurs, factures, événements, matériel) → Organisateurs (Équipe, Éditions, Exposants, Programme, Budget, Documents, etc.).

---

## 6. Checklist d’implémentation UI

À utiliser en complément du [Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md).

### 6.1 Thème

- [ ] Définir struct ou module **JayFestivalTheme** (couleurs section, carte, navigation, header, sidebar, accents organisateur/exposant/visiteur).
- [ ] Appliquer **opacité 0.4** sur fonds principaux (section/corps).
- [ ] Exposer **rayons** (small 4, medium 8, large 12), **spacing** (item_spacing, button_padding, window_margin).
- [ ] Appliquer le thème au démarrage (`ctx.set_style()`) ou par écran.

### 6.2 Atoms / base

- [ ] Bouton : style cohérent (inactive/hovered/active) depuis thème.
- [ ] Champ texte : `TextEdit::singleline` / `multiline` avec marge.
- [ ] Label / heading : couleurs titre vs description depuis thème.
- [ ] Badge : Frame + couleur par statut (validé / en attente / refusé).

### 6.3 Molecules

- [ ] **Carte** (Frame avec titre, corps, optionnel footer) réutilisable.
- [ ] **Ligne de liste** (icône + titre + sous-texte + action) pour listes éditions, exposants, candidatures.

### 6.4 Organisms / layout

- [ ] **Header** (TopBottomPanel) : logo/titre + liens (Catalogue, Connexion, Inscription) ou selon rôle.
- [ ] **Sidebar** (SidePanel) : menu par public (organisateur : Éditions, Équipe ; édition : Exposants, Plan, Programme, Budget, Documents, …).
- [ ] **CentralPanel** : contenu selon vue (liste, dashboard, formulaire).

### 6.5 Écrans prioritaires (MVP)

- [ ] Catalogue : landing + liste événements + fiche événement (lecture seule).
- [ ] Organisateur : connexion/inscription, tableau de bord, liste éditions, dashboard édition (onglets), liste exposants + candidatures, fiche exposant, plan de salle (vue + attribution), programme (liste + édition animation), budget (saisie + ventilation), documents, annonces.
- [ ] Exposant : dashboard (candidatures, participations, agenda, documents, factures), dépôt candidature.
- [ ] Visiteur : espace (agenda, billets, réservations, pass), réservation (flux).

### 6.6 Accessibilité et responsive

- [ ] Focus visible (Dioxus par défaut).
- [ ] Tailles touch minimales (boutons hauteur ≥ 40 px si possible).
- [ ] Détection largeur écran pour sidebar réduite (icônes seules) vs complète.

---

## 7. Références

| Document | Rôle |
|----------|------|
| [JayFestival - Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) | **Normatif** : protocoles, specs détaillées atoms/molecules/organisms, parcours par écran, checklist conformité. |
| [JayFestival - Document Fondateur](./JayFestival%20-%20Document%20Fondateur.md) | Vision, macro, distribution. |
| [JayFestival - Audit Documentation Catakana](./JayFestival%20-%20Audit%20Documentation%20Catakana.md) | Métriques, manques fonctionnels. |
| [JayFestival - Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md) | Périmètre MVP/phase 2, critères de livraison. |
| [Miyukini - Stack UI Dioxus](../../ux_ui/Miyukini%20-%20Stack%20UI%20Dioxus%20Dioxus.md) | Stack UI officielle. |
| Catakana `.Catakana/docs/ATOMIC_THEME_GUIDE.md` | Guide thème et Atomic (source). |
| Catakana `.Catakana/docs/reference/UI_ARCHITECTURE.md` | Architecture UI Catakana (source). |
| Organisateurs / Exposants / Visiteurs / UNC — Écrans et cycle | Liste officielle des écrans JayFestival. |
| `crates/miyukini-central/src/theme.rs` | Thème store existant. |
| `crates/miyukini-central/src/services/ui_library.rs` | Éléments UI Miyukini. |

---

**Document** : JayFestival — Référence UI : transcription Catakana → stack actuelle  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de référence (implémentation UI)
