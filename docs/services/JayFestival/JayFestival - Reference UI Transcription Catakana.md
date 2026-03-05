# JayFestival â€” RÃ©fÃ©rence UI : transcription Catakana â†’ stack actuelle

## Contexte

Ce document **retranscrit lâ€™UI complÃ¨te de Catakana** (Atomic Design, thÃ¨me, ui-kit, Ã©crans) dans la **stack actuelle** Miyukini : **Dioxus** (Rust), avec rÃ©fÃ©rence au thÃ¨me existant (miyukini-central `theme.rs`, `pixel_theme.rs`) et Ã  la Miyukini UI Library. Il sert de **rÃ©fÃ©rence pour lâ€™implÃ©mentation** de JayFestival et complÃ¨te lâ€™[Audit documentation Catakana](./JayFestival%20-%20Audit%20Documentation%20Catakana.md) et le [Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md).

**Sources Catakana** : `.Catakana/docs/ATOMIC_THEME_GUIDE.md`, `.Catakana/docs/reference/UI_ARCHITECTURE.md`, `.Catakana/src/components/` (atoms, molecules, organisms, theme, ui shadcn), `sectionsConfig.ts`, `adminCategoriesConfig.tsx`.

**ConformitÃ©** : L'implÃ©mentation doit respecter la [SpÃ©cification UI conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) : protocoles obligatoires, spÃ©cifications dÃ©taillÃ©es de chaque atome/molÃ©cule/organisme, parcours par Ã©cran avec composants ordonnÃ©s. Ce document (Reference UI) fournit le mapping global ; la spec fournit les bornes et les rÃ¨gles de conformitÃ©.

**Stack cible** : [Miyukini - Stack UI Dioxus](..//..//_index.md) ; `crates/miyukini-central/src/theme.rs`, `pixel_theme.rs`, `services/ui_library.rs`.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Transcription Atomic UI, tokens de thÃ¨me, ui-kit (composants), mapping Ã©crans Catakana â†’ Ã©crans JayFestival (Dioxus), checklist dâ€™implÃ©mentation.
- **Hors pÃ©rimÃ¨tre** : Code source React/TypeScript Catakana ; implÃ©mentation dÃ©taillÃ©e des widgets Dioxus (rÃ©fÃ©rence docs Dioxus et miyukini-central).

---

## 1. Principes et stack

### 1.1 Principes Catakana Ã  conserver

| Principe | Catakana | JayFestival / Dioxus |
|----------|----------|---------------------|
| **Source unique des styles** | `useActiveTheme()` â†’ tokens (couleurs, rayons, ombres, spacing, opacitÃ©s) | Struct `JayFestivalTheme` (ou extension `theme.rs`) ; appliquer via `ctx.set_style()` et helpers. |
| **OpacitÃ© des fonds** | Fonds principaux ~40 % opacitÃ© | `Dioxus::Color32::from_rgba_unmultiplied(r, g, b, 102)` (255*0.4â‰ˆ102) pour fonds section/carte. |
| **Atomic design** | Atoms â†’ Molecules â†’ Organisms â†’ Templates â†’ Pages | Ã‰quivalent Dioxus : **widgets de base** (bouton, label, champ) â†’ **composants composÃ©s** (Frame + label + bouton) â†’ **panels/sections** (SidePanel + liste + cartes) â†’ **layout page** (TopBottomPanel + SidePanel + CentralPanel) â†’ **Ã©cran** (Ã©tat + layout + sections). |
| **Tokens UI** | Jamais de couleurs/tailles en dur | Toutes les couleurs, rayons, espacements, polices viennent du thÃ¨me (struct ou `Dioxus::Style`). |

### 1.2 Stack actuelle (rappel)

- **UI** : Dioxus 0.33, Dioxus 0.33 (Rust).
- **Layout** : `CentralPanel`, `SidePanel`, `TopBottomPanel`, `Window`, `Area`.
- **Widgets** : `ui.button()`, `ui.label()`, `ui.heading()`, `TextEdit`, `checkbox`, `selectable_value`, `collapsing`, `horizontal` / `vertical`, etc.
- **Style** : `ctx.set_style()`, `ctx.set_visuals()` ; `Dioxus::Style`, `Dioxus::Visuals`, `Dioxus::WidgetVisuals` (corner_radius, bg_fill, â€¦).
- **ThÃ¨me existant** : `theme.rs` (store : accent, card radius, spacing), `pixel_theme.rs` (Chrome tabs).

---

## 2. ThÃ¨me â€” tokens Catakana â†’ Dioxus / JayFestival

### 2.1 Noms de tokens (Catakana) â†’ champs Dioxus / struct

| Catakana (theme) | Type / usage | Dioxus / JayFestival |
|------------------|--------------|---------------------|
| `colors.background.primary` | Fond gÃ©nÃ©ral | `Visuals::window_fill()` ou couleur fond CentralPanel ; avec opacitÃ© 0.4 si overlay. |
| `colors.section.background` | Fond section | `Frame::fill(card_bg_color(dark))` ; opacitÃ© 0.4 recommandÃ©e. |
| `colors.section.card.background` | Fond carte | Idem `theme.rs` `card_bg_color(dark)`. |
| `colors.section.border` | Bordure section/carte | `Frame::stroke(Stroke::new(1., section_border_color()))`. |
| `colors.section.title` | Titre section | `ui.heading()` ou `RichText::new().color(section_title_color())`. |
| `colors.section.description` | Description | `ui.label()` avec couleur secondaire. |
| `colors.navigation.container.background` | Barre navigation | `SidePanel::default().frame(Frame::fill(nav_bg))`. |
| `colors.navigation.button.*` | Bouton nav (normal, hover, active) | `WidgetVisuals::inactive/hovered/active.bg_fill`. |
| `colors.header.*` | Logo, titre header | `TopBottomPanel` fill + texte avec `header_title_color()`. |
| `colors.sidebar.*` | Sidebar, menuItem | `SidePanel` fill, stroke ; `selectable_value` avec bg_fill actif. |
| `fonts.sizes.*` | Tailles texte | `Dioxus::FontId::new(size, FontFamily::Proportional)`. |
| `fonts.weights.*` | Graisse | Non utilisÃ© tel quel en Dioxus (pas de font-weight) ; diffÃ©rencier par `FontId::new(size_large, â€¦)` si besoin. |
| `borders.radius.small/medium/large` | Rayon coins | `CornerRadius::same(4)` small, `8` medium, `12` large (u8). |
| `spacing.*` | Marges, padding | `style.spacing.item_spacing`, `button_padding`, `window_margin` ; `Margin::same(n)`. |
| `shadows.small/medium/large` | Ombres | Dioxus nâ€™a pas de box-shadow ; simuler par `Frame::stroke` lÃ©gÃ¨re ou accepter lâ€™absence. |
| `opacity.overlay/cardBackground` | OpacitÃ©s | Alpha dans `Color32::from_rgba_unmultiplied(r,g,b,alpha)`. |

### 2.2 Palette Catakana â†’ couleurs JayFestival (Dioxus)

| Usage Catakana | Couleur / valeur | Dioxus Color32 (exemple) |
|----------------|------------------|-------------------------|
| **Catakana Purple** (primaire) | `#8B5CF6` | `Color32::from_rgb(139, 92, 246)` |
| **Catakana Blue** (secondaire) | `#3B82F6` | `Color32::from_rgb(59, 130, 246)` |
| **Amber** (bÃ©nÃ©voles) | `#F59E0B` | `Color32::from_rgb(245, 158, 11)` |
| **Green** (exposants) | `#10B981` | `Color32::from_rgb(16, 185, 129)` |
| **Fond section (dark)** | rgba + 0.4 | `from_rgba_unmultiplied(30, 30, 35, 102)` |
| **Fond carte (dark)** | rgba + 0.05â€“0.1 | `from_rgba_unmultiplied(50, 50, 55, 25)` |

Recommandation : dÃ©finir un module `jay_festival_theme.rs` (ou Ã©tendre `theme.rs`) avec des fonctions du type `section_bg(dark: bool)`, `card_bg(dark: bool)`, `accent_primary()`, `accent_organisateur()`, `accent_exposant()`, `accent_visiteur()`, et appliquer dans les Ã©crans JayFestival.

### 2.3 Responsive / breakpoints

Catakana : breakpoint 800px, 14px mobile / 16px+ desktop. En Dioxus : utiliser `ctx.screen_rect().width()` pour choisir layout (sidebar large vs icÃ´nes seules), et `FontId::new(14., â€¦)` vs `16.` selon largeur.

---

## 3. Atomic UI â€” Catakana â†’ Dioxus

### 3.1 Atoms (Ã©lÃ©ments de base)

| Catakana | Fichier / usage | Ã‰quivalent Dioxus |
|----------|------------------|------------------|
| **IconWrapper** | `atoms/IconWrapper.tsx` (Lucide, variantes couleur, taille sm/md/lg) | Pas dâ€™icÃ´nes Lucide en Dioxus ; utiliser `ui.label("ðŸ“…")` (emoji) ou intÃ©grer une font dâ€™icÃ´nes (Dioxus_extras) ; taille = `FontId::new(14.|16.|20., â€¦)`. |
| **Button** | shadcn `button.tsx` | `ui.button(RichText::new("Label").color(text_color()))` ; style via `ctx.style().visuals.widgets`. |
| **Input** | shadcn `input.tsx` | `ui.add(Dioxus::TextEdit::singleline(&mut string))`. |
| **Label** | shadcn `label.tsx` | `ui.label()` ou `ui.heading()`. |
| **Badge** | shadcn `badge.tsx`, `ExhibitorStatusBadge` | `Frame::group(ui).fill(badge_bg).show(ui, \|ui\| ui.label("ValidÃ©"))`. |
| **Checkbox** | shadcn `checkbox.tsx` | `ui.checkbox(&mut bool, "Label")`. |
| **Select** | shadcn `select.tsx` | `Dioxus::ComboBox::from_id_salt("id").selected_text(...).show_ui(ui, \|ui\| { ui.selectable_value(...) })`. |

### 3.2 Molecules (combinaisons)

| Catakana | Fichier / usage | Ã‰quivalent Dioxus |
|----------|------------------|------------------|
| **FeatureCard** | `molecules/FeatureCard.tsx` (titre, description, icÃ´ne, variante) | `Frame::card(ui).show(ui, \|ui\| { ui.heading("Titre"); ui.label("Description"); })` avec fill/stroke du thÃ¨me. |
| **DirectoryCard** | `molecules/DirectoryCard.tsx` (gradient, CTA) | Idem + bouton en bas ; gradient en Dioxus = dÃ©gradÃ© manuel ou fill uniforme. |
| **RoleCard** | `molecules/RoleCard.tsx` (pastille colorÃ©e) | Frame + petit cercle/rect colorÃ© + label. |
| **CTACard** | `molecules/CTACard.tsx` | Frame + titre + description + `ui.button()`. |
| **Card** (shadcn) | `ui/card.tsx` | `Frame::default().fill(card_bg).stroke(...).rounding(...).show(ui, ...)`. |

### 3.3 Organisms (sections complÃ¨tes)

| Catakana | Fichier / usage | Ã‰quivalent Dioxus |
|----------|------------------|------------------|
| **Header** | `organisms/Header.tsx` (nav responsive, auth) | `TopBottomPanel::top("header").show(ctx, \|ui\| { ui.horizontal(\|ui\| { ui.label("JayFestival"); ui.button("Catalogue"); ... }) })`. |
| **HeaderWithEdition** | Avec sÃ©lecteur dâ€™Ã©dition | Idem + `ComboBox` Ã©dition. |
| **HeroSection** | `organisms/HeroSection.tsx` | `CentralPanel` ou zone avec titre grand + sous-texte. |
| **FeaturesGrid** | `organisms/FeaturesGrid.tsx` (onglets + grille FeatureCard) | `ui.horizontal` pour onglets (selectable_value) ; `ui.grid()` ou boucle vertical/horizontal pour cartes. |
| **DirectoryBanner** | `organisms/DirectoryBanner.tsx` | Deux `Frame::card` cÃ´te Ã  cÃ´te. |
| **RolesGrid** | `organisms/RolesGrid.tsx` | Grille de RoleCard (4 rÃ´les). |
| **CTASection** | `organisms/CTASection.tsx` | Grille de CTACard + bouton principal. |
| **Layout** | `Layout.tsx` (structure page) | `SidePanel::left` + `CentralPanel::default()` ; contenu central = scroll ou panels. |
| **GestionLayout** | `layouts/GestionLayout.tsx` | Idem avec menu admin (catÃ©gories/sections). |

### 3.4 Templates / Pages

- **Catakana** : templates = layout (header, sidebar, body, footer) ; pages = assemblage + donnÃ©es.
- **Dioxus** : mÃªme idÃ©e â€” une fonction `fn ui_organisateur_dashboard(ctx, app_state)` qui appelle `ui_sidebar()`, puis dans `CentralPanel` `ui_edition_list()` ou `ui_edition_dashboard(edition_id)` ; chaque Â« page Â» = une branche de `match current_view { ... }`.

---

## 4. UI-kit â€” composants Catakana (shadcn + custom) â†’ spec implÃ©mentation

Liste non exhaustive ; chaque entrÃ©e donne le **nom**, lâ€™**usage** et la **cible Dioxus**.

| Composant Catakana | Usage | ImplÃ©mentation Dioxus / rÃ©fÃ©rence |
|--------------------|--------|----------------------------------|
| **button** | Primaire, secondaire, outline, ghost | `ui.button()` + `ctx.style().visuals.widgets` (inactive/hovered/active). |
| **input** | Texte une ligne | `TextEdit::singleline()`. |
| **textarea** | Texte multiligne | `TextEdit::multiline()`. |
| **label** | LibellÃ© champ | `ui.label()`. |
| **card** | Conteneur avec titre/corps/footer | `Frame::card().inner_margin(...).show(ui, ...)`. |
| **badge** | Statut (ex. exposant validÃ©/refusÃ©) | Frame petit + texte ; couleur selon statut. |
| **table** | Listes tabulaires (exposants, budget) | `Dioxus::Grid` ou `ui.horizontal` rÃ©pÃ©tÃ© ; en-tÃªtes en `ui.heading()` ou premiÃ¨re ligne en gras. |
| **tabs** | Onglets (dashboard Ã©dition) | `ui.horizontal` + `selectable_value` ou `Dioxus::TopBottomPanel` avec boutons. |
| **dialog** | Modale | `Window::new("Titre").anchor(Align2::CENTER_CENTER, [0.,0.]).show(ctx, \|ui\| ...)`. |
| **dropdown-menu** | Actions | `ui.menu_button("Actions", \|ui\| { if ui.button("Exporter").clicked() { ... } })`. |
| **select** | Liste dÃ©roulante | `ComboBox::from_id_salt(...).selected_text(...).show_ui(...)`. |
| **checkbox** | BoolÃ©en | `ui.checkbox()`. |
| **calendar** | Date (programme, filtres) | Dioxus_extras `DatePicker` ou champ texte + parsing. |
| **breadcrumb** | Fil dâ€™Ariane (Mes Ã©ditions > Nom Ã©dition) | `ui.horizontal` avec `ui.link("Mes Ã©ditions")` + `ui.label(" > ")` + `ui.label(nom_edition)`. |
| **pagination** | Liste paginÃ©e | `ui.horizontal` avec boutons Â« PrÃ©cÃ©dent Â» / Â« Suivant Â» + label Â« Page n / N Â». |
| **ExhibitorStatusBadge** | Badge statut exposant | Frame colorÃ© (vert/jaune/rouge) + texte (ValidÃ© / En attente / RefusÃ©). |
| **DocumentViewer** | Affichage document (PDF/texte) | Lien tÃ©lÃ©chargement ou iframe non disponible en Dioxus ; afficher mÃ©tadonnÃ©es + bouton Â« TÃ©lÃ©charger Â». |
| **FloorPlanCanvas** | Plan de salle interactif (Fabric.js) | Dioxus : dessin personnalisÃ© avec `Painter` (rectangles, texte) ou intÃ©gration widget 2D ; drag & drop = `Response::dragged()` + mise Ã  jour positions. |
| **ScheduleGrid** | Grille programme (crÃ©neaux Ã— salles) | `ui.grid()` ou tableau avec `ui.label()` par cellule ; clic = ouvrir fenÃªtre Ã©dition animation. |

RÃ©fÃ©rence complÃ©mentaire : **Miyukini UI Library** (`crates/miyukini-central/src/services/ui_library.rs`) pour boutons, cartes, champs, barres â€” rÃ©utiliser ou aligner les styles (padding, corner_radius, couleurs) sur le thÃ¨me JayFestival.

---

## 5. Mapping Ã©crans Catakana â†’ Ã©crans JayFestival (Dioxus)

### 5.1 Par public

Les Ã©crans JayFestival sont dÃ©crits dans les documents Â« Ã‰crans et cycle Â» (Organisateurs, Exposants, Visiteurs, UNC). Le tableau ci-dessous fait le lien avec les **composants / sections Catakana** pour la transcription.

#### 5.1.1 Utilisateur non connectÃ© (catalogue)

| Ã‰cran JayFestival | Id | Composants / sections Catakana |
|-------------------|-----|-------------------------------|
| Landing / Accueil catalogue | UNC-E01 | `HomePage`, `HeroSection`, `FeaturesGrid`, `DirectoryBanner`, `CTASection`, navigation publique. |
| Liste des Ã©vÃ©nements | UNC-E04â€“E06 | Liste + filtres (category2 Informations, Programme, Plan, Exposants). |
| Fiche Ã©vÃ©nement | UNC-E07 | Section Informations + Programme + Plan + Exposants + RÃ¨glements (category2). |
| RÃ©pertoire organisateurs / exposants | UNC-E10â€“E14 | Annuaire exposants (category7 section2), fiches. |

#### 5.1.2 Organisateurs

| Ã‰cran JayFestival | Id | Composants / sections Catakana |
|-------------------|-----|-------------------------------|
| Connexion / Inscription organisateur | ORG-E02, ORG-E03 | `LoginModal`, `SignupModal` (auth) ; formulaire structure. |
| Tableau de bord organisateur | ORG-E04 | `Layout` + sidebar admin ; vue synthÃ¨se Ã©ditions (cartes ou liste). |
| Liste des Ã©ditions | ORG-E05 | Grille/liste type `EditionArchive` ou liste avec filtres. |
| Dashboard Ã©dition | ORG-E07 | Onglets type `GestionLayout` : Exposants, Plan, Programme, Budget, Documents, etc. |
| Liste exposants / Candidatures | ORG-E09, ORG-E10 | `EditionExhibitors`, `EditionCandidaturesList` ; table + badges statut. |
| Fiche exposant / Devis / Factures | ORG-E11â€“E13 | `ExhibitorModal`, formulaires devis/facture (Miyuinvoice). |
| Plan de salle / Attribution | ORG-E14â€“E16 | `FloorPlanCanvas`, `ExhibitorBoothAssignment`, `FloorPlanItemsList`, `FloorPlanToolbar`. |
| Programme | ORG-E17aâ€“E17b | `EditionAgenda`, `ProgramSection`, `ScheduleGrid`, `EventCreatorModal`, `SlotEditorModal`. |
| Budget | ORG-E19 | `EditionBudget`, `BudgetTable`, `BudgetCharts`, `BudgetEntryForm`. |
| Documents et lÃ©gal | ORG-E22 | `EditionDocuments`, `DocumentViewer`. |
| Annonces et notifications | ORG-E23 | Section communication (config + liste envois). |
| Services visiteur | ORG-E24 | Formulaire activation (jeux, concours, ateliers, pass). |
| Publication / ClÃ´ture | ORG-E25 | Boutons + confirmation. |
| Ã‰quipe | ORG-E21 | Liste membres + invitations (rÃ´les Admin, Manager, BÃ©nÃ©vole). |

#### 5.1.3 Exposants

| Ã‰cran JayFestival | Id | Composants / sections Catakana |
|-------------------|-----|-------------------------------|
| Dashboard exposant | EXP-E01â€“E04 | Layout type Â« espace exposant Â» : onglets Candidatures, Participations, Agenda, Documents, Factures. |
| Candidatures / DÃ©pÃ´t | EXP-E05â€“E06 | Formulaire candidature (champs dynamiques) + upload (Dioxus : chemin fichier ou base64 si limitÃ©). |
| Participations / Fiche Ã©dition | EXP-E07â€“E08 | Liste Ã©ditions validÃ©es + fiche dÃ©tail (plan, programme, documents). |
| Agenda / Conflits | EXP-E09 | Vue calendrier (donnÃ©es JayKoa) + alerte conflit. |
| Documents / Factures | EXP-E10â€“E12 | Liste + tÃ©lÃ©chargement (lien ou mÃ©tadonnÃ©es). |

#### 5.1.4 Visiteurs

| Ã‰cran JayFestival | Id | Composants / sections Catakana |
|-------------------|-----|-------------------------------|
| Espace visiteur | VIS-E04â€“E14 | Layout : Agenda, Billets, RÃ©servations, Pass VIP, ActivitÃ©s (jeux, concours). |
| RÃ©servation (atelier, crÃ©neau, pass) | VIS-E09 | Formulaire choix crÃ©neau + vÃ©rification conflit (JayKoa) + confirmation. |
| Fiche Ã©vÃ©nement (connectÃ©) | VIS-E10 | MÃªme contenu que UNC fiche Ã©vÃ©nement + CTAs (RÃ©server, Sâ€™inscrire). |

### 5.2 CatÃ©gories / sections Catakana (config) â†’ usage JayFestival

- **category1** (News) â†’ Annonces organisateur + optionnel phase 2 ActualitÃ©s public.
- **category2** (Informations, Programme, Plan, Exposants, RÃ¨glements) â†’ Catalogue public + dashboard Ã©dition (Programme, Plan, Exposants, Documents).
- **category3** (InvitÃ©s, Ateliers, Animations, Concours, Jeux) â†’ Services visiteur (phase 2 complet) ; MVP = rÃ©servations ateliers/crÃ©neaux.
- **category4** (Plans) â†’ Plan de salle (organisateur + exposant + public).
- **category5** (RPG, Inventaire, RÃ©compenses, Leaderboard) â†’ Hors scope JayFestival v1.
- **category6** (Galeries) â†’ Phase 2 ou Miyumedia.
- **category7** (Organisation : Annuaire, Emplacements, RÃ©servation stands, PrÃ©sentation) â†’ Exposants (annuaire, rÃ©servation) + catalogue.
- **category8** (Gestion : appli, compte, candidatures, Ã©ditions, utilisateurs, factures, Ã©vÃ©nements, matÃ©riel) â†’ Organisateurs (Ã‰quipe, Ã‰ditions, Exposants, Programme, Budget, Documents, etc.).

---

## 6. Checklist dâ€™implÃ©mentation UI

Ã€ utiliser en complÃ©ment du [Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md).

### 6.1 ThÃ¨me

- [ ] DÃ©finir struct ou module **JayFestivalTheme** (couleurs section, carte, navigation, header, sidebar, accents organisateur/exposant/visiteur).
- [ ] Appliquer **opacitÃ© 0.4** sur fonds principaux (section/corps).
- [ ] Exposer **rayons** (small 4, medium 8, large 12), **spacing** (item_spacing, button_padding, window_margin).
- [ ] Appliquer le thÃ¨me au dÃ©marrage (`ctx.set_style()`) ou par Ã©cran.

### 6.2 Atoms / base

- [ ] Bouton : style cohÃ©rent (inactive/hovered/active) depuis thÃ¨me.
- [ ] Champ texte : `TextEdit::singleline` / `multiline` avec marge.
- [ ] Label / heading : couleurs titre vs description depuis thÃ¨me.
- [ ] Badge : Frame + couleur par statut (validÃ© / en attente / refusÃ©).

### 6.3 Molecules

- [ ] **Carte** (Frame avec titre, corps, optionnel footer) rÃ©utilisable.
- [ ] **Ligne de liste** (icÃ´ne + titre + sous-texte + action) pour listes Ã©ditions, exposants, candidatures.

### 6.4 Organisms / layout

- [ ] **Header** (TopBottomPanel) : logo/titre + liens (Catalogue, Connexion, Inscription) ou selon rÃ´le.
- [ ] **Sidebar** (SidePanel) : menu par public (organisateur : Ã‰ditions, Ã‰quipe ; Ã©dition : Exposants, Plan, Programme, Budget, Documents, â€¦).
- [ ] **CentralPanel** : contenu selon vue (liste, dashboard, formulaire).

### 6.5 Ã‰crans prioritaires (MVP)

- [ ] Catalogue : landing + liste Ã©vÃ©nements + fiche Ã©vÃ©nement (lecture seule).
- [ ] Organisateur : connexion/inscription, tableau de bord, liste Ã©ditions, dashboard Ã©dition (onglets), liste exposants + candidatures, fiche exposant, plan de salle (vue + attribution), programme (liste + Ã©dition animation), budget (saisie + ventilation), documents, annonces.
- [ ] Exposant : dashboard (candidatures, participations, agenda, documents, factures), dÃ©pÃ´t candidature.
- [ ] Visiteur : espace (agenda, billets, rÃ©servations, pass), rÃ©servation (flux).

### 6.6 AccessibilitÃ© et responsive

- [ ] Focus visible (Dioxus par dÃ©faut).
- [ ] Tailles touch minimales (boutons hauteur â‰¥ 40 px si possible).
- [ ] DÃ©tection largeur Ã©cran pour sidebar rÃ©duite (icÃ´nes seules) vs complÃ¨te.

---

## 7. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [JayFestival - Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) | **Normatif** : protocoles, specs dÃ©taillÃ©es atoms/molecules/organisms, parcours par Ã©cran, checklist conformitÃ©. |
| [JayFestival - Document Fondateur](./JayFestival%20-%20Document%20Fondateur.md) | Vision, macro, distribution. |
| [JayFestival - Audit Documentation Catakana](./JayFestival%20-%20Audit%20Documentation%20Catakana.md) | MÃ©triques, manques fonctionnels. |
| [JayFestival - Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md) | PÃ©rimÃ¨tre MVP/phase 2, critÃ¨res de livraison. |
| [Miyukini - Stack UI Dioxus](..//..//_index.md) | Stack UI officielle. |
| Catakana `.Catakana/docs/ATOMIC_THEME_GUIDE.md` | Guide thÃ¨me et Atomic (source). |
| Catakana `.Catakana/docs/reference/UI_ARCHITECTURE.md` | Architecture UI Catakana (source). |
| Organisateurs / Exposants / Visiteurs / UNC â€” Ã‰crans et cycle | Liste officielle des Ã©crans JayFestival. |
| `crates/miyukini-central/src/theme.rs` | ThÃ¨me store existant. |
| `crates/miyukini-central/src/services/ui_library.rs` | Ã‰lÃ©ments UI Miyukini. |

---

**Document** : JayFestival â€” RÃ©fÃ©rence UI : transcription Catakana â†’ stack actuelle  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de rÃ©fÃ©rence (implÃ©mentation UI)

