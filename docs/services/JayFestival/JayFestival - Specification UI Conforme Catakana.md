# JayFestival — Spécification UI conforme Catakana

## Contexte

Ce document est la **spécification normative** de l’UI JayFestival. Il impose une correspondance **exacte** avec l’UI Catakana : éléments Atomic Design (atoms, molecules, organisms), parcours, zones et ordre des composants. Toute implémentation doit **respecter les protocoles** et les **bornes** définis ici.

**Référence** : [JayFestival - Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) (mapping global et stack). Le présent document en est le **complément obligatoire** pour la conformité.

## Portée / Scope

- **Périmètre** : Protocoles d’implémentation, spécifications détaillées de chaque composant UI (atoms, molecules, organisms), parcours par écran avec liste ordonnée des composants, critères de conformité.
- **Statut** : **Normatif** — l’implémentation JayFestival doit s’y conformer pour être considérée conforme à Catakana.
- **Hors périmètre** : Code source Catakana ; implémentation détaillée des primitives egui (voir docs egui et miyukini-central).

---

## 1. Protocoles d’implémentation

### 1.1 Règles obligatoires

| Règle | Énoncé | Contrôle |
|-------|--------|----------|
| **PROTO-1** | Aucun style en dur : couleurs, rayons, espacements, tailles de police proviennent **uniquement** du thème JayFestival (struct ou module dédié). | Revue / grep interdits : `Color32::from_rgb(`, `Margin::same(` sans variable thème, etc. |
| **PROTO-2** | Ordre de construction : **Thème → Atoms → Molecules → Organisms → Layout → Écrans**. Aucun composant de niveau N ne doit dépendre d’un composant de niveau > N. | Dépendances vérifiables (modules / imports). |
| **PROTO-3** | Chaque composant listé en section 2 (Atoms, Molecules, Organisms) doit exister avec l’**identifiant** et les **paramètres** décrits ; les variantes (primary, secondary, sm/md/lg) sont obligatoires quand précisées. | Checklist par composant. |
| **PROTO-4** | Chaque écran listé en section 4 doit afficher les **zones** dans l’ordre défini et utiliser les **composants** listés dans l’ordre indiqué ; aucun composant Catakana référencé ne doit être omis. | Revue écran par écran. |
| **PROTO-5** | Opacité des fonds principaux (section, carte, corps) : **0,4** (alpha 102 sur 255) sauf exception documentée. | Vérification thème et usage `fill()`. |
| **PROTO-6** | Responsive : breakpoint unique **800 px** (largeur) ; en dessous : sidebar réduite (icônes seules si applicable), taille police de base 14 px ; au-dessus : sidebar complète, 16 px. | `ctx.screen_rect().width()` utilisé pour choisir layout. |
| **PROTO-7** | Accessibilité : zone cliquable minimale **40 px** (hauteur boutons, liens) ; focus visible (egui par défaut). | Mesure des widgets. |
| **PROTO-8** | Parcours : les **entrées/sorties** documentées dans les docs « Écrans et cycle » (UNC, Organisateurs, Exposants, Visiteurs) sont les seules navigations autorisées entre écrans. | Pas de lien ou bouton vers un écran non prévu. |

### 1.2 Ordre de construction (obligatoire)

1. **Thème** : struct ou module `JayFestivalTheme` (ou extension `theme.rs`) avec toutes les couleurs, rayons, spacing, polices listés en [Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) § 2.
2. **Atoms** : implémenter dans l’ordre : IconWrapper → Button → Input → Label → Badge → Checkbox → Select.
3. **Molecules** : FeatureCard → DirectoryCard → RoleCard → CTACard → Card (shadcn-like).
4. **Organisms** : Header → HeaderWithEdition → HeroSection → FeaturesGrid → DirectoryBanner → RolesGrid → CTASection → Layout → GestionLayout.
5. **Layout** : Layout (SidePanel + CentralPanel), GestionLayout (avec menu admin).
6. **Écrans** : par public dans l’ordre des docs Écrans et cycle (UNC puis ORG, EXP, VIS).

### 1.3 Critères de conformité (bornes)

- **Conforme** : tous les composants de la section 2 sont implémentés avec les props/tokens indiqués ; tous les écrans de la section 4 respectent la structure zones + liste de composants ; PROTO-1 à PROTO-8 respectés.
- **Non conforme** : absence d’un composant, ordre des zones/composants différent, style en dur, opacité fond ≠ 0,4 sans justification, navigation vers un écran non documenté.
- **Hors périmètre conforme** : composants ou écrans marqués « phase 2 » ou « hors alpha » dans le [Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md) peuvent être absents mais doivent respecter la même spec dès qu’ils sont livrés.

---

## 2. Spécifications détaillées des composants

### 2.1 Atoms

#### A1 — IconWrapper

| Attribut | Spécification |
|----------|----------------|
| **Id** | `IconWrapper` |
| **Source Catakana** | `atoms/IconWrapper.tsx` (Lucide, variantes couleur, taille sm/md/lg) |
| **Props / paramètres** | `icon_id: IconId` (ou emoji unicode), `size: Size` (sm=14, md=16, lg=20), `color: Option<Color32>` (défaut = `section_title_color()` ou texte normal). |
| **Tokens** | `fonts.sizes.sm/md/lg`, `colors.section.title` ou `colors.text.primary`. |
| **Comportement** | Affiche une icône (emoji ou glyphe) à la taille et couleur demandées ; pas d’interaction. |
| **Contrat egui** | `fn icon_wrapper(ui: &mut Ui, icon_id: IconId, size: Size, color: Option<Color32>) -> Response` ; utilise `ui.label(RichText::new(icon_char).size(size_pt).color(c))` ou équivalent. |
| **Variantes** | sm (14 pt), md (16 pt), lg (20 pt). Couleur : default, primary (accent), muted (description). |

#### A2 — Button

| Attribut | Spécification |
|----------|----------------|
| **Id** | `Button` |
| **Source Catakana** | shadcn `button.tsx` (primary, secondary, outline, ghost) |
| **Props / paramètres** | `label: &str`, `variant: ButtonVariant` (Primary, Secondary, Outline, Ghost), `size: Size` (sm, md, lg). |
| **Tokens** | `colors.navigation.button.normal/hover/active`, `borders.radius.medium`, `spacing.button_padding`, `fonts.sizes.md`. |
| **Comportement** | Clic déclenche callback ; états hover/active visibles. |
| **Contrat egui** | `fn button(ui: &mut Ui, label: &str, variant: ButtonVariant, size: Size) -> Response` ; style via `ctx.style().visuals.widgets` (inactive/hovered/active) et couleurs thème. |
| **Variantes** | Primary (fond accent), Secondary (fond secondaire), Outline (bordure, fond transparent), Ghost (transparent, hover léger). |

#### A3 — Input

| Attribut | Spécification |
|----------|----------------|
| **Id** | `Input` |
| **Source Catakana** | shadcn `input.tsx` |
| **Props / paramètres** | `value: &mut String`, `placeholder: Option<&str>`, `password: bool`, `enabled: bool`. |
| **Tokens** | `colors.section.border`, `borders.radius.small`, `spacing.input_padding`, `fonts.sizes.md`. |
| **Comportement** | Champ texte une ligne ; placeholder si vide. |
| **Contrat egui** | `fn input(ui: &mut Ui, value: &mut String, placeholder: Option<&str>, password: bool) -> Response` ; `TextEdit::singleline(value)` avec frame et couleurs thème. |

#### A4 — Label

| Attribut | Spécification |
|----------|----------------|
| **Id** | `Label` |
| **Source Catakana** | shadcn `label.tsx` |
| **Props / paramètres** | `text: &str`, `level: LabelLevel` (Heading, Body, Small, Muted). |
| **Tokens** | `colors.section.title` (heading), `colors.text.primary` (body), `colors.section.description` (muted), `fonts.sizes.*`. |
| **Comportement** | Texte seul, non interactif. |
| **Contrat egui** | `fn label(ui: &mut Ui, text: &str, level: LabelLevel)` ; `ui.heading()` ou `ui.label(RichText::new(text).color(...).size(...))`. |

#### A5 — Badge

| Attribut | Spécification |
|----------|----------------|
| **Id** | `Badge` |
| **Source Catakana** | shadcn `badge.tsx`, custom `ExhibitorStatusBadge` |
| **Props / paramètres** | `text: &str`, `variant: BadgeVariant` (Default, Success, Warning, Error — pour Validé, En attente, Refusé, etc.). |
| **Tokens** | Couleurs par variant (vert/jaune/rouge), `borders.radius.small`, `spacing.badge_padding`, `fonts.sizes.sm`. |
| **Comportement** | Pastille de statut ; pas d’interaction. |
| **Contrat egui** | `fn badge(ui: &mut Ui, text: &str, variant: BadgeVariant) -> Rect` ; `Frame::none().fill(badge_bg(variant)).inner_margin(...).show(ui, \|ui\| ui.label(text))`. |

#### A6 — Checkbox

| Attribut | Spécification |
|----------|----------------|
| **Id** | `Checkbox` |
| **Source Catakana** | shadcn `checkbox.tsx` |
| **Props / paramètres** | `checked: &mut bool`, `label: &str`, `enabled: bool`. |
| **Tokens** | `colors.section.border`, accent pour coché, `fonts.sizes.md`. |
| **Comportement** | Case à cocher ; clic inverse la valeur. |
| **Contrat egui** | `ui.checkbox(checked, label)` ; style cohérent thème. |

#### A7 — Select

| Attribut | Spécification |
|----------|----------------|
| **Id** | `Select` |
| **Source Catakana** | shadcn `select.tsx` |
| **Props / paramètres** | `selected: &mut usize`, `options: &[String]`, `label: Option<&str>`, `id: impl Hash`. |
| **Tokens** | Même que Input + `colors.section.title` pour libellé. |
| **Comportement** | Liste déroulante ; sélection met à jour `selected`. |
| **Contrat egui** | `ComboBox::from_id_salt(id).selected_text(options[*selected].clone()).show_ui(ui, \|ui\| { for (i, o) in options.iter().enumerate() { ui.selectable_value(selected, i, o); } })`. |

---

### 2.2 Molecules

#### M1 — FeatureCard

| Attribut | Spécification |
|----------|----------------|
| **Id** | `FeatureCard` |
| **Source Catakana** | `molecules/FeatureCard.tsx` (titre, description, icône, variante) |
| **Props / paramètres** | `title: &str`, `description: &str`, `icon: IconId`, `variant: CardVariant` (default, accent). |
| **Tokens** | `colors.section.card.background`, `colors.section.border`, `borders.radius.medium`, `spacing.card_padding`, `colors.section.title`, `colors.section.description`. |
| **Comportement** | Carte avec titre, description, icône ; optionnellement cliquable (lien). |
| **Contrat egui** | `Frame::default().fill(card_bg).stroke(...).rounding(radius_medium).inner_margin(...).show(ui, \|ui\| { IconWrapper; Label(title, Heading); Label(description, Muted) })`. |

#### M2 — DirectoryCard

| Attribut | Spécification |
|----------|----------------|
| **Id** | `DirectoryCard` |
| **Source Catakana** | `molecules/DirectoryCard.tsx` (gradient, CTA) |
| **Props / paramètres** | `title: &str`, `description: Option<&str>`, `cta_label: &str`, `on_click: impl FnOnce()`. |
| **Tokens** | Idem FeatureCard + `colors.navigation.button.*` pour CTA. Gradient en egui : fill uniforme ou dégradé manuel si disponible. |
| **Comportement** | Carte avec CTA en bas ; clic sur le bouton déclenche `on_click`. |
| **Contrat egui** | FeatureCard + `ui.add_space()` + `Button(cta_label, Primary, md)` dans le même Frame. |

#### M3 — RoleCard

| Attribut | Spécification |
|----------|----------------|
| **Id** | `RoleCard` |
| **Source Catakana** | `molecules/RoleCard.tsx` (pastille colorée) |
| **Props / paramètres** | `title: &str`, `description: Option<&str>`, `accent_color: Color32`, `on_click: Option<impl FnOnce()>`. |
| **Tokens** | `colors.section.card.background`, `borders.radius.medium`, `spacing.card_padding`. `accent_color` = organisateur / exposant / visiteur / bénévole (Amber, Green, Blue, Purple). |
| **Comportement** | Petite carte avec pastille colorée (cercle ou rectangle) + titre (+ description) ; optionnellement cliquable. |
| **Contrat egui** | `Frame::default().fill(card_bg).rounding(...).show(ui, \|ui\| { ui.horizontal(\|ui\| { ui.add(PaintCircle(accent_color)); ui.label(title); }); optional description })`. |

#### M4 — CTACard

| Attribut | Spécification |
|----------|----------------|
| **Id** | `CTACard` |
| **Source Catakana** | `molecules/CTACard.tsx` |
| **Props / paramètres** | `title: &str`, `description: Option<&str>`, `button_label: &str`, `on_click: impl FnOnce()`. |
| **Tokens** | Idem FeatureCard + bouton Primary. |
| **Comportement** | Carte avec titre, description optionnelle, bouton d’action. |
| **Contrat egui** | FeatureCard + `Button(button_label, Primary, md)` ; clic = `on_click`. |

#### M5 — Card (conteneur shadcn-like)

| Attribut | Spécification |
|----------|----------------|
| **Id** | `Card` |
| **Source Catakana** | `ui/card.tsx` (header, body, footer optionnels) |
| **Props / paramètres** | `header: Option<&str>`, `body: impl FnOnce(&mut Ui)`, `footer: Option<impl FnOnce(&mut Ui)>`. |
| **Tokens** | `colors.section.card.background`, `colors.section.border`, `borders.radius.medium`, `spacing.card_padding`. |
| **Comportement** | Conteneur avec zones header/body/footer ; body obligatoire. |
| **Contrat egui** | `Frame::default().fill(card_bg).stroke(...).rounding(...).show(ui, \|ui\| { optional heading(header); body(ui); optional footer(ui) })`. |

---

### 2.3 Organisms

#### O1 — Header

| Attribut | Spécification |
|----------|----------------|
| **Id** | `Header` |
| **Source Catakana** | `organisms/Header.tsx` (nav responsive, auth) |
| **Props / paramètres** | `logo_label: &str` ("JayFestival"), `nav_links: &[(&str, ScreenId)]`, `show_auth: bool`, `on_login: impl FnOnce()`, `on_signup: impl FnOnce()`, `user_menu: Option<UserMenu>` (si connecté). |
| **Tokens** | `colors.header.*`, `colors.navigation.container.background`, `colors.navigation.button.*`, `fonts.sizes.md`. |
| **Comportement** | Barre haute : logo + liens nav + boutons Se connecter / S’inscrire ou menu utilisateur. Responsive : sous 800 px, liens regroupés ou icônes. |
| **Contrat egui** | `TopBottomPanel::top("header").show(ctx, \|ui\| { ui.horizontal(\|ui\| { Label(logo_label, Heading); for (label, _) in nav_links { Button(label, Ghost, sm); } if show_auth { Button("Se connecter", Outline, sm); Button("S'inscrire", Primary, sm); } else { user_menu } }) })`. |

#### O2 — HeaderWithEdition

| Attribut | Spécification |
|----------|----------------|
| **Id** | `HeaderWithEdition` |
| **Source Catakana** | Header avec sélecteur d’édition (organisateur) |
| **Props / paramètres** | Comme Header + `edition_id: &mut Option<Id>`, `editions: &[(Id, String)]`, `on_edition_change: impl FnOnce(Id)`. |
| **Comportement** | Header + ComboBox édition à droite (ou dans la barre). |
| **Contrat egui** | Header + `Select` ou `ComboBox` pour édition. |

#### O3 — HeroSection

| Attribut | Spécification |
|----------|----------------|
| **Id** | `HeroSection` |
| **Source Catakana** | `organisms/HeroSection.tsx` |
| **Props / paramètres** | `title: &str`, `subtitle: Option<&str>`, `optional_search_placeholder: Option<&str>`, `optional_cta: Option<(&str, impl FnOnce())>`. |
| **Tokens** | `colors.section.title`, `colors.section.description`, `fonts.sizes.*` (title grand, subtitle plus petit). |
| **Comportement** | Zone d’accroche : titre principal, sous-texte, optionnellement champ recherche et/ou CTA. |
| **Contrat egui** | `CentralPanel` ou zone : `Label(title, Heading)` grande, `Label(subtitle, Muted)`, optionnel `Input` (recherche), optionnel `Button` (CTA). |

#### O4 — FeaturesGrid

| Attribut | Spécification |
|----------|----------------|
| **Id** | `FeaturesGrid` |
| **Source Catakana** | `organisms/FeaturesGrid.tsx` (onglets + grille FeatureCard) |
| **Props / paramètres** | `tabs: &[(&str, &[FeatureCardData])]` (onglet label + liste de cartes). |
| **Tokens** | Idem FeatureCard + couleurs onglets (actif / inactif). |
| **Comportement** | Onglets horizontaux ; au clic, affichage de la grille de FeatureCard correspondante. |
| **Contrat egui** | `ui.horizontal` avec `selectable_value` pour onglets ; puis `ui.grid()` ou boucle vertical/horizontal pour `FeatureCard`. |

#### O5 — DirectoryBanner

| Attribut | Spécification |
|----------|----------------|
| **Id** | `DirectoryBanner` |
| **Source Catakana** | `organisms/DirectoryBanner.tsx` |
| **Props / paramètres** | `left_card: DirectoryCardData`, `right_card: DirectoryCardData`. |
| **Tokens** | Idem DirectoryCard. |
| **Comportement** | Deux DirectoryCard côte à côte (ou empilés si largeur < 800 px). |
| **Contrat egui** | `ui.horizontal(\|ui\| { DirectoryCard(left_card); DirectoryCard(right_card); })` ou vertical si responsive. |

#### O6 — RolesGrid

| Attribut | Spécification |
|----------|----------------|
| **Id** | `RolesGrid` |
| **Source Catakana** | `organisms/RolesGrid.tsx` |
| **Props / paramètres** | `roles: &[RoleCardData]` (4 rôles : Organisateur, Exposant, Visiteur, Bénévole). |
| **Tokens** | Idem RoleCard. |
| **Comportement** | Grille 2×2 (ou 4 en ligne sur grand écran) de RoleCard. |
| **Contrat egui** | `ui.grid()` ou 2×2 `RoleCard`. |

#### O7 — CTASection

| Attribut | Spécification |
|----------|----------------|
| **Id** | `CTASection` |
| **Source Catakana** | `organisms/CTASection.tsx` |
| **Props / paramètres** | `cards: &[CTACardData]`, `main_button: Option<(&str, impl FnOnce())>`. |
| **Tokens** | Idem CTACard. |
| **Comportement** | Grille de CTACard + optionnel bouton principal en bas ou à part. |
| **Contrat egui** | Grille de `CTACard` + optionnel `Button(..., Primary, lg)`. |

#### O8 — Layout (structure page catalogue)

| Attribut | Spécification |
|----------|----------------|
| **Id** | `Layout` |
| **Source Catakana** | `Layout.tsx` (header, sidebar, body, footer) |
| **Props / paramètres** | `header: Header`, `sidebar: Option<SidebarContent>`, `body: impl FnOnce(&mut Ui)`, `footer: Option<impl FnOnce(&mut Ui)>`. |
| **Tokens** | `colors.navigation.container.background`, `colors.section.background`. |
| **Comportement** | TopBottomPanel (header) + SidePanel (sidebar si présent) + CentralPanel (body) ; footer optionnel en bas du central. |
| **Contrat egui** | `Header.show(ctx)` ; `SidePanel::left(...).show(ctx, sidebar)` si présent ; `CentralPanel::default().show(ctx, body)` ; footer dans body ou Area en bas. |

#### O9 — GestionLayout (organisateur / exposant / visiteur connecté)

| Attribut | Spécification |
|----------|----------------|
| **Id** | `GestionLayout` |
| **Source Catakana** | `layouts/GestionLayout.tsx` (menu admin / catégories) |
| **Props / paramètres** | Comme Layout + `menu_items: &[(ScreenId, &str, Option<IconId>)]`, `breadcrumb: Option<&[(&str, Option<ScreenId)>]>`, `edition_selector: Option<...>` (organisateur). |
| **Comportement** | Sidebar = menu (Éditions, Exposants, Plan, Programme, Budget, Documents, etc.) ; breadcrumb au-dessus du body ; body = contenu de l’écran courant. |
| **Contrat egui** | SidePanel avec `selectable_value` pour menu ; CentralPanel avec optional breadcrumb (horizontal links + labels) puis body. |

---

## 3. Récapitulatif composants → écrans Catakana

| Composant | Utilisé dans écrans Catakana (référence) |
|-----------|------------------------------------------|
| IconWrapper | Partout (FeatureCard, Header, HeroSection, badges). |
| Button | Header, HeroSection, CTACard, DirectoryCard, formulaires, listes. |
| Input | HeroSection (recherche), Connexion, Inscription, filtres listes. |
| Label | Tous les écrans. |
| Badge | Liste exposants/candidatures (statut), listes avec statut. |
| Checkbox | Inscription (CGU), formulaires (options). |
| Select | Filtres, édition (HeaderWithEdition), formulaires. |
| FeatureCard | Landing (FeaturesGrid). |
| DirectoryCard | Landing (DirectoryBanner). |
| RoleCard | Landing (RolesGrid). |
| CTACard | Landing (CTASection). |
| Card | Listes (ligne = carte), fiches (blocs), dashboard (cartes synthèse). |
| Header | UNC-E01, tous les écrans publics. |
| HeaderWithEdition | ORG-E04 à ORG-E25 (organisateur), EXP (si applicable). |
| HeroSection | UNC-E01 (landing). |
| FeaturesGrid | UNC-E01. |
| DirectoryBanner | UNC-E01. |
| RolesGrid | UNC-E01. |
| CTASection | UNC-E01. |
| Layout | UNC-E01, UNC-E02 à UNC-E14. |
| GestionLayout | ORG-E04 à ORG-E25, EXP-E04 à EXP-E19, VIS-E04 à VIS-E15. |

---

## 4. Parcours par écran — structure et composants ordonnés

Pour chaque écran, la **structure** (zones) et la **liste ordonnée des composants** doivent être respectées. Les identifiants d’écran (UNC-E01, ORG-E04, etc.) sont ceux des documents « Écrans et cycle » des publics.

### 4.1 Utilisateur non connecté

#### UNC-E01 — Landing / Accueil catalogue

| Zone | Ordre | Composants (dans l’ordre) |
|------|-------|----------------------------|
| En-tête | 1 | **Header** (logo "JayFestival", nav: Événements, Organisateurs, Exposants ; boutons Se connecter, S’inscrire). |
| Zone principale | 2 | **HeroSection** (titre accroche, sous-texte, champ recherche global, optionnel CTA). |
| | 3 | **FeaturesGrid** (onglets + grille FeatureCard). |
| | 4 | **DirectoryBanner** (2 DirectoryCard : ex. Événements à la une, Prochains événements). |
| | 5 | **RolesGrid** (4 RoleCard : Organisateur, Exposant, Visiteur, Bénévole). |
| | 6 | **CTASection** (grille CTACard + bouton principal). |
| Pied | 7 | **Footer** (liens Mentions légales, CGU, Confidentialité, Accessibilité) — composant non détaillé ici ; utiliser Label + liens ou Button Ghost. |

#### UNC-E02 — Liste des événements

| Zone | Ordre | Composants |
|------|-------|-------------|
| En-tête | 1 | Header (idem UNC-E01). |
| Filtres | 2 | Horizontal : **Input** (recherche), **Select** (date, lieu, organisateur, thème), **Button** "Réinitialiser". |
| Vue | 3 | **Button** (bascule Liste / Carte). |
| Liste | 4 | Grille ou liste de **Card** (vignette optionnelle, **Label** titre, **Label** dates/lieu, **Label** organisateur, **Button** "Voir la fiche"). Pagination : **Button** Précédent/Suivant + **Label** "Page n / N". |
| Pied | 5 | Footer. |

#### UNC-E03 — Fiche événement

| Zone | Ordre | Composants |
|------|-------|-------------|
| En-tête | 1 | Header. |
| Bloc 1 | 2 | **Label** (titre, Heading), **Label** (dates, lieu), **Label** (organisateur avec lien). |
| Bloc 2 | 3 | **Card** (body = **Label** description). |
| Bloc 3 | 4 | **Label** "Programme public" ; grille/table ou **Card** par animation (Label horaire, salle, type). |
| Bloc 4 | 5 | **Label** "Exposants" ; liste **Card** (Label nom, **Button** "Voir la fiche"). |
| Bloc 5 | 6 | **Label** "Services" ; **CTACard** ou **Button** (Réserver, Acheter pass, Déposer candidature). |
| Pied fiche | 7 | **Button** "Retour liste" ; optionnel partage. Footer. |

#### UNC-E06 — Liste des organisateurs

| Zone | Ordre | Composants |
|------|-------|-------------|
| En-tête | 1 | Header. |
| Titre + Filtres | 2 | **Label** "Organisateurs" ; **Input** + **Select** (région, type, année). |
| Liste | 3 | Grille/liste **Card** (Label nom, région, nb événements, **Button** "Voir la fiche"). Pagination. |
| Pied | 4 | Footer. |

#### UNC-E07 — Fiche organisateur

| Zone | Ordre | Composants |
|------|-------|-------------|
| En-tête | 1 | Header. |
| Bloc 1–4 | 2 | **Card** (nom, description) ; **Card** (liste événements avec **Button** liens) ; **Card** (coordonnées) ; **Card** (charte optionnel). |
| Pied | 3 | **Button** Retour. Footer. |

#### UNC-E08 — Liste des exposants

| Zone | Ordre | Composants |
|------|-------|-------------|
| En-tête | 1 | Header. |
| Titre + Filtres | 2 | **Label** "Exposants" ; **Input**, **Select** (catégorie, événement, région) ; bascule vue. |
| Liste | 3 | Grille/liste **Card** (Label nom, catégorie, **Button** "Voir la fiche"). Pagination. |
| Pied | 4 | Footer. |

#### UNC-E09 — Fiche exposant

| Zone | Ordre | Composants |
|------|-------|-------------|
| En-tête | 1 | Header. |
| Bloc 1–3 | 2 | **Card** (nom, description, secteur) ; **Card** (éditions participées, liens) ; **Card** (coordonnées). |
| Pied | 3 | **Button** Retour. Footer. |

#### UNC-E10 — Recherche (résultats et affinage)

| Zone | Ordre | Composants |
|------|-------|-------------|
| En-tête | 1 | Header. |
| Champ recherche | 2 | **Input** (placeholder "Rechercher…") + **Button** "Rechercher". |
| Résultats | 3 | Onglets ou sections : **Label** "Événements" / "Organisateurs" / "Exposants" ; liste **Card** (titre/nom, extrait, **Button** "Voir la fiche"). Filtres affinage : **Select** ; **Label** "Aucun résultat" si vide. |
| Pied | 4 | Footer. |

#### UNC-E11 — CTA contextuels (message non connecté)

| Zone | Ordre | Composants |
|------|-------|-------------|
| Modal | 1 | **Window** (ancrage centre) : **Label** (message "Connectez-vous ou créez un compte…") ; **Button** "Se connecter" ; **Button** "S'inscrire" ; **Button** "Retour". |

#### UNC-E12 — Connexion

| Zone | Ordre | Composants |
|------|-------|-------------|
| Formulaire | 1 | **Label** "Se connecter" ; **Input** (email), **Input** (mot de passe, password) ; **Button** "Mot de passe oublié" (Ghost) ; **Button** "S'inscrire" (Ghost) ; **Button** "Se connecter" (Primary). **Label** message d’erreur si échec. |

#### UNC-E13 — Inscription (choix type)

| Zone | Ordre | Composants |
|------|-------|-------------|
| Choix | 1 | **Label** "S'inscrire" ; **RoleCard** ou **CTACard** x3 (Organisateur, Exposant, Visiteur) ; **Button** "Déjà un compte ? Se connecter" (Ghost). |

#### UNC-E14 — Mentions légales, CGU, Confidentialité, Accessibilité

| Zone | Ordre | Composants |
|------|-------|-------------|
| En-tête | 1 | Header. |
| Contenu | 2 | **Label** (titre page) ; **Label** (texte lecture seule). |
| Pied | 3 | **Button** Retour. Footer. |

---

### 4.2 Organisateurs

#### ORG-E04 — Tableau de bord organisateur

| Zone | Ordre | Composants |
|------|-------|-------------|
| En-tête | 1 | **HeaderWithEdition** (ou Header) ; menu : Éditions, Mon compte, Équipe, Déconnexion. |
| Corps | 2 | **GestionLayout** : sidebar = liens (Éditions, Mon compte, Équipe) ; body = **Label** "Synthèse" ; **Card** x N (indicateurs : nb éditions, prochain événement, alertes) ; **Label** "Mes éditions" ; grille/liste **Card** (édition : nom, dates, statut, **Button** "Voir") ; **Button** "Créer une édition". |
| Pied | 3 | Footer optionnel. |

#### ORG-E05 — Liste des éditions

| Zone | Ordre | Composants |
|------|-------|-------------|
| Layout | 1 | **GestionLayout** ; body = **Label** "Mes éditions" ; **Select** (statut, année), **Input** (recherche) ; bascule Liste/Cartes ; grille **Card** (nom, dates, lieu, **Badge** statut, indicateurs, **Button** "Voir" / "Dupliquer") ; **Button** "Créer une édition". |
| | 2 | Pagination. |

#### ORG-E06 — Création édition

| Zone | Ordre | Composants |
|------|-------|-------------|
| Layout | 1 | **GestionLayout** ; body = **Label** "Nouvelle édition" ; **Input** (nom), champs dates/lieu/thème, **Select** statut ; **Button** "Enregistrer", **Button** "Annuler". |
| | 2 | Option "Dupliquer depuis" : **Select** édition source + **Checkbox** éléments à dupliquer. |

#### ORG-E07 — Dashboard édition

| Zone | Ordre | Composants |
|------|-------|-------------|
| Layout | 1 | **GestionLayout** ; **breadcrumb** "Mes éditions > [Nom]" ; sidebar = onglets (Vue d’ensemble, Exposants, Plan, Programme, Budget, Documents, Notifications, Services visiteur, Paramètres, Publication). |
| Corps | 2 | **Label** "Vue d’ensemble" ; **Card** indicateurs (nb exposants, candidatures en attente, budget, animations, stands) ; liens rapides vers chaque module. |
| Pied | 3 | Footer optionnel. |

#### ORG-E09 — Liste exposants (édition)

| Zone | Ordre | Composants |
|------|-------|-------------|
| Layout | 1 | **GestionLayout** (sidebar = menu édition) ; body = **Label** "Exposants — [Nom édition]" ; **Input** + **Select** (statut, catégorie) ; **Button** "Importer". |
| Tableau | 2 | Grille/table : colonnes (nom, contact, **Badge** statut, emplacement) ; **Button** "Voir", "Modifier" ; **Button** "Export CSV/Excel". |
| Pied | 3 | Pagination. |

#### ORG-E10 — Candidatures

| Zone | Ordre | Composants |
|------|-------|-------------|
| Layout | 1 | **GestionLayout** ; body = **Label** "Candidatures en attente" ; liste **Card** (exposant, date dépôt, **Badge** statut, **Button** "Voir" / "Valider" / "Refuser"). |
| Détail | 2 | Panneau ou **Window** : données exposant, pièces, **Button** "Valider", **Button** "Refuser" (avec **Input** motif si refus). |
| Pied | 3 | Pagination. |

#### ORG-E11 — Fiche exposant

| Zone | Ordre | Composants |
|------|-------|-------------|
| Layout | 1 | **GestionLayout** ; body = **Label** "Fiche exposant — [Nom]" ; **Card** (Identité : nom, contact, catégorie) ; **Card** (Statut : **Badge**, motif) ; **Card** (Emplacement, lien plan) ; **Card** (Documents, liste + **Button** Télécharger) ; **Card** (Historique) ; **Button** "Modifier", "Générer devis", "Convertir en facture". |
| Pied | 2 | **Button** Retour liste. |

Les autres écrans organisateur (ORG-E08, ORG-E12 à ORG-E25) suivent le même schéma : **GestionLayout** + sidebar + body avec **Card**, **Input**, **Select**, **Button**, **Badge**, **Label**, tableaux (grille egui), modales (**Window**) pour formulaires secondaires. Détail complet à déduire des sections 2 et 3 et des docs « Écrans et cycle ».

---

### 4.3 Exposants

#### EXP-E04 — Dashboard exposant

| Zone | Ordre | Composants |
|------|-------|-------------|
| En-tête | 1 | **Header** ou **GestionLayout** ; menu : Candidatures, Participations, Agenda, Documents, Factures, Mon compte, Déconnexion. |
| Corps | 2 | **Label** "Synthèse" ; **Card** (candidatures en attente, prochain événement, alertes) ; onglets ou blocs : **Card** Candidatures (raccourci), **Card** Participations, **Card** Agenda (aperçu), **Card** Documents, **Card** Factures. |
| Pied | 3 | Footer optionnel. |

Les écrans EXP-E05 à EXP-E19 respectent la même logique : **GestionLayout** (ou Layout) + body avec **Card**, listes, **Badge**, **Button**, **Input**, **Select**, **Window** pour formulaires (dépôt candidature, envoi document, etc.).

---

### 4.4 Visiteurs

#### VIS-E04 — Page d’accueil espace visiteur

| Zone | Ordre | Composants |
|------|-------|-------------|
| En-tête | 1 | **Header** / **GestionLayout** ; menu : Agenda, Billets, Réservations, Pass VIP, Activités, Catalogue, Mon compte, Déconnexion. |
| Corps | 2 | **Label** "Prochain événement : [Nom], dans X jours" ; blocs/onglets : **Card** Agenda (aperçu), **Card** Billets, **Card** Réservations, **Card** Pass VIP, **Card** Activités ; filtre **Par événement** (**Select**). |
| Pied | 3 | Footer optionnel. |

Les écrans VIS-E05 à VIS-E15 suivent le même principe : **GestionLayout** + body avec **Card**, **Label**, **Button**, **Input**, **Select**, **Badge**, **Window** pour réservation, jeux, concours, compte, préférences.

---

## 5. Checklist de conformité (implémentation)

À valider avant livraison alpha (voir [Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md)).

### 5.1 Protocoles

- [ ] PROTO-1 : Aucun style en dur (couleurs, rayons, spacing, polices) hors thème.
- [ ] PROTO-2 : Ordre de construction Thème → Atoms → Molecules → Organisms → Layout → Écrans respecté.
- [ ] PROTO-3 : Tous les composants § 2 implémentés avec id et paramètres indiqués.
- [ ] PROTO-4 : Tous les écrans § 4 présents avec zones et composants dans l’ordre.
- [ ] PROTO-5 : Opacité 0,4 sur fonds section/carte.
- [ ] PROTO-6 : Breakpoint 800 px appliqué (sidebar, police).
- [ ] PROTO-7 : Zones cliquables ≥ 40 px ; focus visible.
- [ ] PROTO-8 : Navigation uniquement selon docs Écrans et cycle.

### 5.2 Atoms

- [ ] A1 IconWrapper (sm/md/lg, couleurs).
- [ ] A2 Button (Primary, Secondary, Outline, Ghost ; sm/md/lg).
- [ ] A3 Input (placeholder, password).
- [ ] A4 Label (Heading, Body, Small, Muted).
- [ ] A5 Badge (Default, Success, Warning, Error).
- [ ] A6 Checkbox.
- [ ] A7 Select.

### 5.3 Molecules

- [ ] M1 FeatureCard.
- [ ] M2 DirectoryCard.
- [ ] M3 RoleCard.
- [ ] M4 CTACard.
- [ ] M5 Card (header/body/footer).

### 5.4 Organisms

- [ ] O1 Header.
- [ ] O2 HeaderWithEdition.
- [ ] O3 HeroSection.
- [ ] O4 FeaturesGrid.
- [ ] O5 DirectoryBanner.
- [ ] O6 RolesGrid.
- [ ] O7 CTASection.
- [ ] O8 Layout.
- [ ] O9 GestionLayout.

### 5.5 Écrans (par public)

- [ ] UNC : E01, E02, E03, E06, E07, E08, E09, E10, E11, E12, E13, E14 (structure + composants ordonnés).
- [ ] ORG : E04, E05, E06, E07, E08, E09, E10, E11, E12, E13, E14–E25 (idem).
- [ ] EXP : E04–E19 (idem).
- [ ] VIS : E04–E15 (idem).

---

## 6. Références

| Document | Rôle |
|----------|------|
| [JayFestival - Reference UI Transcription Catakana](./JayFestival%20-%20Reference%20UI%20Transcription%20Catakana.md) | Mapping global, thème, stack egui. |
| [JayFestival - Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md) | Périmètre alpha, critères de livraison. |
| [UtilisateurNonConnecte - Ecrans et cycle](./publics/UtilisateurNonConnecte/UtilisateurNonConnecte%20-%20Ecrans%20et%20cycle.md) | Écrans UNC. |
| [Organisateurs - Ecrans et cycle](./publics/Organisateurs/Organisateurs%20-%20Ecrans%20et%20cycle.md) | Écrans ORG. |
| [Exposants - Ecrans et cycle](./publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md) | Écrans EXP. |
| [Visiteurs - Ecrans et cycle](./publics/Visiteurs/Visiteurs%20-%20Ecrans%20et%20cycle.md) | Écrans VIS. |

---

**Document** : JayFestival — Spécification UI conforme Catakana  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Normatif (implémentation UI)
