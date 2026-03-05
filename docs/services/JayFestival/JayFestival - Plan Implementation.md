# JayFestival â€” Plan dâ€™implÃ©mentation exhaustif

## Contexte

Ce document est le **plan dâ€™implÃ©mentation** du service **JayFestival** et de ses **services dÃ©pendants** (JayXpose, JayKoa, JayKonta, Miyu*, Supabase alpha). Il est **exhaustif et prÃ©cis**, divisÃ© en **phases**, et respecte les protocoles :

- [Miyukini Prompt Protocol - ImplÃ©mentation gÃ©nÃ©rale](..//..//_index.md)
- [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md)

**RÃ©fÃ©rences** : [Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md), [Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md), [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md), [Etat Documentation Services Interfaces](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : ImplÃ©mentation alpha JayFestival (Dioxus, Supabase backend) + services dÃ©pendants dans le scope alpha (JayXpose, intÃ©grations Miyu*, JayKoa, JayKonta).
- **Nomenclature des tÃ¢ches** : `[xx] - [nom du fichier Ã  produire]` ; `xx` = prÃ©fixe de regroupement (01, 02, â€¦) ; **maximum 4 tÃ¢ches par prÃ©fixe** pour exÃ©cution parallÃ¨le (max 4 agents simultanÃ©s).
- **RÃ¨gles** : 1 Ã©tape = 1 fichier ; 1 agent = 1 tÃ¢che ; contexte vierge pour chaque dÃ©lÃ©gation ; balisage MSCM obligatoire ; index MIP rÃ©gÃ©nÃ©rÃ© en Phase 4.

---

## Phase 0 â€” Planification (protocole)

### 0.1 Titre de lâ€™Ã©tape

Planification globale JayFestival et services dÃ©pendants.

### 0.2 Explication rapide

- **Objectif** : Livrer une version **alpha fonctionnelle** de JayFestival (reprise Catakana en Dioxus, backend Supabase), avec catalogue (Ã©vÃ©nements, organisateurs, exposants), espaces organisateur / exposant / visiteur, et intÃ©grations documentÃ©es (JayXpose, JayKoa, JayKonta, Miyuinvoice, Miyunotify, Miyubooking, MiyuClock).
- **PÃ©rimÃ¨tre** : CrÃ©ation crate(s) JayFestival, UI conforme [Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md), parcours UNC / ORG / EXP / VIS selon docs Ã‰crans et cycle, client Supabase (Auth + REST), services dÃ©pendants dans le scope alpha.
- **Limites** : JayFaim hors alpha ; KindMother/SQLite post-alpha ; pas de migration de donnÃ©es dans ce plan (documentÃ©e Ã  part).

### 0.3 SÃ©lection du modÃ¨le IA (obligatoire en entÃªte de tout prompt)

```
COMPLEXITÃ‰ : Complexe | Extreme (selon phase)
CHARGE CONTEXTUELLE : Moyenne | Ã‰levÃ©e

MODÃˆLE AUTORISÃ‰ :
- Phase 0â€“2 (thÃ¨me, atoms, molecules) â†’ 1 modÃ¨le premium (Composer / Sonnet)
- Phase 3+ (Ã©crans, intÃ©grations) â†’ LLM Ã©tendu si nÃ©cessaire

MODE IA ACTIF : AI Mode 1 | AI Mode 2
```

### 0.4 Cadre de travail (agents)

- **Documentation autorisÃ©e (liste fermÃ©e)** : JayFestival (Document fondateur, Bornage, Specification UI, Reference UI, Reference Base de Donnees, Etat Documentation Services, Interpolarite) ; JayXpose (Document fondateur, Analyse besoins, Parcours, Operateurs, Ecrans, Reference Base de donnees Supabase) ; publics JayFestival (Exposants, Organisateurs, Visiteurs, UNC â€” Analyse, Ecrans et cycle, Operateurs) ; protocoles ImplÃ©mentation gÃ©nÃ©rale et MIP v1.
- **Outils autorisÃ©s** : Ã‰dition fichiers Rust/md, grep, lecture docs, exÃ©cution tests `cargo test`.
- **Outils interdits** : Modification manuelle de `mscm_index/` (index MIP gÃ©nÃ©rÃ© uniquement par pipeline).

### 0.5 Contraintes absolues

- Ne pas anticiper les Ã©tapes suivantes.
- Ne pas fusionner plusieurs fichiers en une seule livraison.
- Ne pas corriger hors pÃ©rimÃ¨tre de la tÃ¢che.
- ArrÃªt immÃ©diat si ambiguÃ¯tÃ© bloquante, dÃ©pendance manquante ou contexte insuffisant.

### 0.6 Tests

- Tests unitaires console (`cargo test`) pour modules logique mÃ©tier et services lorsque possible.
- Justification explicite si absence de tests (ex. UI pure Dioxus).

### 0.7 Mini log de planification

| Ã‰lÃ©ment | DÃ©cision / Risque |
|--------|-------------------|
| **AmbiguÃ¯tÃ©s** | Toutes P0/P1 tranchÃ©es (Miyuinvoice+JayKonta, JayXpose alpha, Miyuprofile Supabase, JayKoa/MiyuClock). |
| **DÃ©pendances critiques** | JayXpose (fiche exposant, rÃ©pertoire) dans alpha ; Supabase Auth + tables (profiles, exposants, editions, editions_exposants, etc.). |
| **DÃ©cisions structurantes** | Ordre de construction UI : ThÃ¨me â†’ Atoms â†’ Molecules â†’ Organisms â†’ Layout â†’ Ã‰crans ; 1 Ã©cran = 1 module (ou groupe cohÃ©rent) avec nomenclature [xx]-[screen_id]. |

---

## Phase 1 â€” Fondations projet et thÃ¨me (crate, config, UI base)

**Objectif** : CrÃ©er la crate JayFestival (ou binaire dans un crate existant), dÃ©pendances Cargo, et **thÃ¨me** conforme Specification UI (tokens : couleurs, rayons, espacements, polices). Aucun composant atom/molecule ne doit exister avant le thÃ¨me.

### 1.1 TÃ¢ches Phase 1

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus (@id, @do, @layer) |
|----------|--------------------|-------------|----------------------------------------|
| **[01] - Crate et config** | `crates/jayfestival/Cargo.toml` + `crates/jayfestival/src/lib.rs` (stub) | â€” | `jayfestival_crate_config`, `jayfestival_lib_stub` ; layer: infra |
| **[02] - ThÃ¨me** | `crates/jayfestival/src/theme.rs` (ou `ui/theme.rs`) | [01] | `jayfestival_theme_struct`, `jayfestival_theme_tokens` (couleurs, borders.radius, spacing, fonts.sizes) ; layer: ui |
| **[03] - Main et boucle app** | `crates/jayfestival/src/main.rs` (Dioxus) | [01], [02] | `jayfestival_main_entry`, `jayfestival_app_loop` ; layer: app |
| **[04] - Constantes Ã©crans** | `crates/jayfestival/src/screens.rs` (ScreenId enum / consts) | [01] | `jayfestival_screen_ids` ; layer: app |

**Balisage MSCM** : Chaque bloc avec `@id` unique, `@do` (description fonctionnelle), `@layer` (infra | ui | app). Optionnel : `@role`, `@human`.

**Ordre dâ€™exÃ©cution** : [01] puis [02], [03], [04] en parallÃ¨le possible (02, 03, 04 dÃ©pendent de 01 uniquement).

---

## Phase 2 â€” Atoms et Molecules (Specification UI Â§ 2)

**Objectif** : ImplÃ©menter tous les **atoms** puis toutes les **molecules** dans lâ€™ordre imposÃ© par la Specification UI (PROTO-2). Aucun style en dur (PROTO-1) ; tous les tokens viennent du thÃ¨me.

### 2.1 Atoms (ordre obligatoire)

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[11] - IconWrapper** | `crates/jayfestival/src/ui/atoms/icon_wrapper.rs` | [02] | `atom_icon_wrapper`, `icon_wrapper_render` ; layer: ui |
| **[12] - Button** | `crates/jayfestival/src/ui/atoms/button.rs` | [02] | `atom_button`, `button_render`, variants Primary/Secondary/Outline/Ghost ; layer: ui |
| **[13] - Input** | `crates/jayfestival/src/ui/atoms/input.rs` | [02] | `atom_input`, `input_render` ; layer: ui |
| **[14] - Label** | `crates/jayfestival/src/ui/atoms/label.rs` | [02] | `atom_label`, `label_render`, levels Heading/Body/Small/Muted ; layer: ui |

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[21] - Badge** | `crates/jayfestival/src/ui/atoms/badge.rs` | [02] | `atom_badge`, `badge_render`, variants Default/Success/Warning/Error ; layer: ui |
| **[22] - Checkbox** | `crates/jayfestival/src/ui/atoms/checkbox.rs` | [02] | `atom_checkbox`, `checkbox_render` ; layer: ui |
| **[23] - Select** | `crates/jayfestival/src/ui/atoms/select.rs` | [02] | `atom_select`, `select_render` ; layer: ui |
| **[24] - mod atoms** | `crates/jayfestival/src/ui/atoms/mod.rs` | [11]â€“[23] | rÃ©export uniquement ; pas de bloc mÃ©tier |

**Ordre** : [11] â†’ [12] â†’ [13] â†’ [14] (sÃ©quentiel ou 11 puis 12,13,14 en parallÃ¨le). Puis [21]â€“[23] puis [24].

### 2.2 Molecules (ordre obligatoire)

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[31] - FeatureCard** | `crates/jayfestival/src/ui/molecules/feature_card.rs` | atoms | `molecule_feature_card`, `feature_card_render` ; layer: ui |
| **[32] - DirectoryCard** | `crates/jayfestival/src/ui/molecules/directory_card.rs` | atoms | `molecule_directory_card`, `directory_card_render` ; layer: ui |
| **[33] - RoleCard** | `crates/jayfestival/src/ui/molecules/role_card.rs` | atoms | `molecule_role_card`, `role_card_render` ; layer: ui |
| **[34] - CTACard** | `crates/jayfestival/src/ui/molecules/cta_card.rs` | atoms | `molecule_cta_card`, `cta_card_render` ; layer: ui |

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[41] - Card** | `crates/jayfestival/src/ui/molecules/card.rs` | atoms | `molecule_card`, `card_render` (shadcn-like) ; layer: ui |
| **[42] - mod molecules** | `crates/jayfestival/src/ui/molecules/mod.rs` | [31]â€“[41] | rÃ©export ; layer: ui |

**DÃ©pendances inter-blocs** : Chaque molecule dÃ©pend des atoms (IconWrapper, Button, Label, Badge, etc.) ; dÃ©clarer dans MSCM si le parseur le permet.

---

## Phase 3 â€” Organisms et Layout (Specification UI Â§ 2)

**Objectif** : ImplÃ©menter les **organisms** puis les **layouts** (Layout, GestionLayout). Ordre : Header â†’ HeaderWithEdition â†’ HeroSection â†’ FeaturesGrid â†’ DirectoryBanner â†’ RolesGrid â†’ CTASection â†’ Layout â†’ GestionLayout.

### 3.1 Organisms

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[51] - Header** | `crates/jayfestival/src/ui/organisms/header.rs` | molecules, atoms | `organism_header`, `header_render` ; layer: ui |
| **[52] - HeaderWithEdition** | `crates/jayfestival/src/ui/organisms/header_with_edition.rs` | [51], Select | `organism_header_with_edition`, `header_with_edition_render` ; layer: ui |
| **[53] - HeroSection** | `crates/jayfestival/src/ui/organisms/hero_section.rs` | molecules, atoms | `organism_hero_section`, `hero_section_render` ; layer: ui |
| **[54] - FeaturesGrid** | `crates/jayfestival/src/ui/organisms/features_grid.rs` | FeatureCard | `organism_features_grid`, `features_grid_render` ; layer: ui |

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[61] - DirectoryBanner** | `crates/jayfestival/src/ui/organisms/directory_banner.rs` | DirectoryCard | `organism_directory_banner`, `directory_banner_render` ; layer: ui |
| **[62] - RolesGrid** | `crates/jayfestival/src/ui/organisms/roles_grid.rs` | RoleCard | `organism_roles_grid`, `roles_grid_render` ; layer: ui |
| **[63] - CTASection** | `crates/jayfestival/src/ui/organisms/cta_section.rs` | CTACard | `organism_cta_section`, `cta_section_render` ; layer: ui |
| **[64] - Layout** | `crates/jayfestival/src/ui/organisms/layout.rs` | Header | `organism_layout`, `layout_render` (SidePanel + CentralPanel) ; layer: ui |

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[71] - GestionLayout** | `crates/jayfestival/src/ui/organisms/gestion_layout.rs` | [51], [52], [64] | `organism_gestion_layout`, `gestion_layout_render` ; layer: ui |
| **[72] - mod organisms** | `crates/jayfestival/src/ui/organisms/mod.rs` | [51]â€“[71] | rÃ©export ; layer: ui |

---

## Phase 4 â€” Client Supabase et Auth (backend alpha)

**Objectif** : Client Supabase (REST + Auth) pour lâ€™alpha ; pas de Miyauth natif. Session, rÃ´les via `profiles.user_type`. Ã‰crans Connexion / Inscription (formulaires) utilisÃ©s par tous les publics.

### 4.1 Client et types

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[81] - Client Supabase** | `crates/jayfestival/src/supabase/client.rs` | [01] | `supabase_client_init`, `supabase_client_request` ; layer: infra |
| **[82] - Types Supabase** | `crates/jayfestival/src/supabase/types.rs` | â€” | `supabase_profiles`, `supabase_editions`, `supabase_exposants`, `supabase_editions_exposants` (structs alignÃ©s Reference Base de Donnees) ; layer: domain |
| **[83] - Auth service** | `crates/jayfestival/src/auth/mod.rs` (+ `sign_in.rs`, `sign_up.rs`, `session.rs` si dÃ©coupÃ©) | [81], [82] | `auth_sign_in`, `auth_sign_up`, `auth_session_current`, `auth_sign_out` ; layer: domain |
| **[84] - RLS et permissions** | `crates/jayfestival/src/auth/permissions.rs` | [82] | `auth_user_type_from_profile`, `auth_can_access_edition` ; layer: domain |

**RÃ©fÃ©rence** : [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) (tables, RLS).

---

## Phase 5 â€” Ã‰crans catalogue (Utilisateur non connectÃ© â€” UNC)

**Objectif** : Ã‰crans UNC-E01 Ã  UNC-E14 selon [Specification UI Â§ 4.1](.\JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) et docs [Utilisateur non connectÃ© â€” Ã‰crans et cycle](./publics/UtilisateurNonConnecte/UtilisateurNonConnecte%20-%20Ecrans%20et%20cycle.md). Ordre des zones et composants strict (PROTO-4).

### 5.1 TÃ¢ches Phase 5 (1 Ã©cran = 1 fichier ou 1 module)

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[91] - UNC-E01 Landing** | `crates/jayfestival/src/screens/unc/e01_landing.rs` | Layout, HeroSection, FeaturesGrid, DirectoryBanner, RolesGrid, CTASection | `screen_unc_e01`, `unc_e01_show` ; layer: app |
| **[92] - UNC-E02 Liste Ã©vÃ©nements** | `crates/jayfestival/src/screens/unc/e02_liste_evenements.rs` | Layout, Card, Input, Select, Button | `screen_unc_e02`, `unc_e02_show` ; layer: app |
| **[93] - UNC-E03 Fiche Ã©vÃ©nement** | `crates/jayfestival/src/screens/unc/e03_fiche_evenement.rs` | Layout, Card, Label, Button | `screen_unc_e03`, `unc_e03_show` ; layer: app |
| **[94] - UNC-E06 Liste organisateurs** | `crates/jayfestival/src/screens/unc/e06_liste_organisateurs.rs` | Layout, Card, Input, Select | `screen_unc_e06`, `unc_e06_show` ; layer: app |

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[101] - UNC-E07 Fiche organisateur** | `crates/jayfestival/src/screens/unc/e07_fiche_organisateur.rs` | Layout, Card, Label, Button | `screen_unc_e07`, `unc_e07_show` ; layer: app |
| **[102] - UNC-E08 Liste exposants** | `crates/jayfestival/src/screens/unc/e08_liste_exposants.rs` | Layout, Card, Input, Select (donnÃ©es JayXpose/Supabase) | `screen_unc_e08`, `unc_e08_show` ; layer: app |
| **[103] - UNC-E09 Fiche exposant** | `crates/jayfestival/src/screens/unc/e09_fiche_exposant.rs` | Layout, Card (donnÃ©es JayXpose/Supabase) | `screen_unc_e09`, `unc_e09_show` ; layer: app |
| **[104] - UNC-E10 Recherche** | `crates/jayfestival/src/screens/unc/e10_recherche.rs` | Layout, Input, Card, Select | `screen_unc_e10`, `unc_e10_show` ; layer: app |

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[111] - UNC-E11 CTA contextuels** | `crates/jayfestival/src/screens/unc/e11_cta_contextuels.rs` | Window/Modal, Label, Button | `screen_unc_e11`, `unc_e11_show` ; layer: app |
| **[112] - UNC-E12 Connexion** | `crates/jayfestival/src/screens/unc/e12_connexion.rs` | Input, Button, Label, [83] Auth | `screen_unc_e12`, `unc_e12_show` ; layer: app |
| **[113] - UNC-E13 Inscription** | `crates/jayfestival/src/screens/unc/e13_inscription.rs` | RoleCard/CTACard, Button, [83] Auth | `screen_unc_e13`, `unc_e13_show` ; layer: app |
| **[114] - UNC-E14 Mentions / CGU** | `crates/jayfestival/src/screens/unc/e14_mentions.rs` | Layout, Label, Button | `screen_unc_e14`, `unc_e14_show` ; layer: app |

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[115] - Router UNC** | `crates/jayfestival/src/screens/unc/mod.rs` | [91]â€“[114] | `router_unc`, `unc_navigate` ; layer: app |

**DonnÃ©es** : E02, E03, E06, E07, E08, E09, E10 consomment Supabase (Ã©ditions, organisateurs, exposants) ; E08/E09 alignÃ©s avec JayXpose (table `exposants`, rÃ©pertoire). Voir [JayXpose - Base de donnees Supabase](..//JayXpose//reference//JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md).

---

## Phase 6 â€” Services dÃ©pendants alpha (JayXpose, donnÃ©es)

**Objectif** : Module **JayXpose** (fiche exposant, rÃ©pertoire) consommÃ© par JayFestival : lecture profil exposant, liste rÃ©pertoire (filtres). Alpha = donnÃ©es Supabase (tables `exposants`, `editions_exposants`). Pas dâ€™UI propre JayXpose dans ce plan â€” UI = Ã©crans JayFestival (UNC-E08, E09, ORG exposants, etc.).

### 6.1 TÃ¢ches Phase 6

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[121] - JayXpose client** | `crates/jayfestival/src/services/jayxpose/client.rs` (ou `crates/jayxpose/` si crate dÃ©diÃ©e) | [81], [82] | `jayxpose_get_profile`, `jayxpose_list_repertoire`, `jayxpose_fiche_by_id` ; layer: domain |
| **[122] - Contrat JayXpose** | `crates/jayfestival/src/services/jayxpose/contract.rs` (types entrÃ©e/sortie) | [82] | `jayxpose_profile_type`, `jayxpose_repertoire_filters` ; layer: domain |
| **[123] - IntÃ©gration Ã©crans** | Utilisation dans E08, E09, ORG exposants (dÃ©jÃ  prÃ©vus en Phase 5 / 7) | [121], [122] | â€” (pas de fichier dÃ©diÃ© ; les Ã©crans appellent le client) |

**RÃ©fÃ©rence** : [JayXpose - Analyse des besoins](../JayXpose/JayXpose%20-%20Analyse%20des%20besoins.md), [JayXpose - Base de donnees Supabase](../JayXpose/reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md).

---

## Phase 7 â€” Parcours Organisateurs (Ã©crans ORG)

**Objectif** : Ã‰crans ORG-E04 Ã  ORG-E25 (ou sous-ensemble alpha) selon [Specification UI Â§ 4.2](.\JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) et [Organisateurs â€” Ã‰crans et cycle](./publics/Organisateurs/Organisateurs%20-%20Ecrans%20et%20cycle.md). GestionLayout, HeaderWithEdition, donnÃ©es Supabase (Ã©ditions, editions_exposants, stands, budget_entries, invoices, schedule_slots, documents).

### 7.1 TÃ¢ches Phase 7 (groupes de 4 max par prÃ©fixe)

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[131] - ORG-E04 Dashboard** | `crates/jayfestival/src/screens/org/e04_dashboard.rs` | GestionLayout, HeaderWithEdition, Card, [81] | `screen_org_e04`, `org_e04_show` ; layer: app |
| **[132] - ORG-E05 Liste Ã©ditions** | `crates/jayfestival/src/screens/org/e05_liste_editions.rs` | GestionLayout, Card, Select, Button | `screen_org_e05`, `org_e05_show` ; layer: app |
| **[133] - ORG-E06 CrÃ©ation Ã©dition** | `crates/jayfestival/src/screens/org/e06_creation_edition.rs` | GestionLayout, Input, Button | `screen_org_e06`, `org_e06_show` ; layer: app |
| **[134] - ORG-E07 Dashboard Ã©dition** | `crates/jayfestival/src/screens/org/e07_dashboard_edition.rs` | GestionLayout, Card, Badge | `screen_org_e07`, `org_e07_show` ; layer: app |

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[141] - ORG-E08 Liste exposants** | `crates/jayfestival/src/screens/org/e08_liste_exposants.rs` | GestionLayout, Card, Badge, [121] | `screen_org_e08`, `org_e08_show` ; layer: app |
| **[142] - ORG-E09 Candidatures** | `crates/jayfestival/src/screens/org/e09_candidatures.rs` | GestionLayout, Card, Badge, Button | `screen_org_e09`, `org_e09_show` ; layer: app |
| **[143] - ORG-E10 Fiche exposant (org)** | `crates/jayfestival/src/screens/org/e10_fiche_exposant.rs` | GestionLayout, Card, [121] | `screen_org_e10`, `org_e10_show` ; layer: app |
| **[144] - ORG-E11 Plan de salle** | `crates/jayfestival/src/screens/org/e11_plan_salle.rs` | GestionLayout, zones/stands (widget ou canvas) | `screen_org_e11`, `org_e11_show` ; layer: app |

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[151] - ORG-E12 Programme** | `crates/jayfestival/src/screens/org/e12_programme.rs` | GestionLayout, Card, liste crÃ©neaux/salles | `screen_org_e12`, `org_e12_show` ; layer: app |
| **[152] - ORG-E13 Budget** | `crates/jayfestival/src/screens/org/e13_budget.rs` | GestionLayout, Card, Input (revenus/dÃ©penses) | `screen_org_e13`, `org_e13_show` ; layer: app |
| **[153] - ORG-E14 Devis / Factures** | `crates/jayfestival/src/screens/org/e14_devis_factures.rs` | GestionLayout, Card (Miyuinvoice/JayKonta) | `screen_org_e14`, `org_e14_show` ; layer: app |
| **[154] - ORG-E15 Documents** | `crates/jayfestival/src/screens/org/e15_documents.rs` | GestionLayout, Card, Button | `screen_org_e15`, `org_e15_show` ; layer: app |

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[161] - ORG-E16â€“E25 (reste)** | Fichiers sÃ©parÃ©s ou regroupÃ©s (Annonces, Notifications, Mon compte, Ã‰quipe, etc.) | GestionLayout, atoms/molecules | `screen_org_eXX`, `org_eXX_show` ; layer: app |
| **[162] - Router ORG** | `crates/jayfestival/src/screens/org/mod.rs` | [131]â€“[161] | `router_org`, `org_navigate` ; layer: app |

**Note** : Ã‰crans ORG-E12 (Programme), E13 (Budget), E14 (Devis/Factures) sâ€™appuient sur les flux documentÃ©s dans [JayKonta - Integration Services](../JayKonta/reference/JayKonta%20-%20Integration%20Services.md) et [Miyuinvoice] ; en alpha, lecture/Ã©criture via Supabase (tables budget_entries, invoices).

---

## Phase 8 â€” Parcours Exposants (Ã©crans EXP)

**Objectif** : Ã‰crans EXP selon [Exposants â€” Ã‰crans et cycle](./publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md) et Specification UI : dashboard exposant, candidatures, participations, documents, factures.

### 8.1 TÃ¢ches Phase 8

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[171] - EXP-E04 Dashboard exposant** | `crates/jayfestival/src/screens/exp/e04_dashboard.rs` | GestionLayout, Card, [81], [121] | `screen_exp_e04`, `exp_e04_show` ; layer: app |
| **[172] - EXP-E05 Candidatures** | `crates/jayfestival/src/screens/exp/e05_candidatures.rs` | GestionLayout, Card, Badge | `screen_exp_e05`, `exp_e05_show` ; layer: app |
| **[173] - EXP-E06 Participations** | `crates/jayfestival/src/screens/exp/e06_participations.rs` | GestionLayout, Card | `screen_exp_e06`, `exp_e06_show` ; layer: app |
| **[174] - EXP-E07â€“E19 (reste)** | Fichiers par Ã©cran (Documents, Factures, Mon compte, etc.) | GestionLayout, atoms/molecules | `screen_exp_eXX`, `exp_eXX_show` ; layer: app |

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[175] - Router EXP** | `crates/jayfestival/src/screens/exp/mod.rs` | [171]â€“[174] | `router_exp`, `exp_navigate` ; layer: app |

---

## Phase 9 â€” Parcours Visiteurs (Ã©crans VIS)

**Objectif** : Ã‰crans VIS selon [Visiteurs â€” Ã‰crans et cycle](./publics/Visiteurs/Visiteurs%20-%20Ecrans%20et%20cycle.md) : espace visiteur (agenda, billets, rÃ©servations, pass). IntÃ©gration Miyubooking (crÃ©neaux, rÃ©servation) documentÃ©e ; en alpha donnÃ©es Supabase.

### 9.1 TÃ¢ches Phase 9

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[181] - VIS-E04 Dashboard visiteur** | `crates/jayfestival/src/screens/vis/e04_dashboard.rs` | GestionLayout, Card | `screen_vis_e04`, `vis_e04_show` ; layer: app |
| **[182] - VIS-E05 Agenda** | `crates/jayfestival/src/screens/vis/e05_agenda.rs` | GestionLayout, vue calendrier (JayKoa) ou liste | `screen_vis_e05`, `vis_e05_show` ; layer: app |
| **[183] - VIS-E06â€“E15 (billets, rÃ©servations, pass)** | Fichiers par Ã©cran | GestionLayout, Card, Button | `screen_vis_eXX`, `vis_eXX_show` ; layer: app |
| **[184] - Router VIS** | `crates/jayfestival/src/screens/vis/mod.rs` | [181]â€“[183] | `router_vis`, `vis_navigate` ; layer: app |

---

## Phase 10 â€” IntÃ©grations (JayKoa, JayKonta, Miyunotify, Miyubooking, MiyuClock)

**Objectif** : Modules dâ€™intÃ©gration **appelants** (JayFestival appelle les services) ; pas dâ€™implÃ©mentation des services eux-mÃªmes. Contrats documentÃ©s dans [Etat Documentation Services Interfaces](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md) et [Interpolarite Services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md).

### 10.1 TÃ¢ches Phase 10

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[191] - Adapter JayKoa** | `crates/jayfestival/src/services/jaykoa/adapter.rs` | [81] ou client JayKoa si crate | `jaykoa_publish_edition`, `jaykoa_get_conflicts` ; layer: domain |
| **[192] - Adapter JayKonta / Miyuinvoice** | `crates/jayfestival/src/services/jaykonta/adapter.rs` | [81], tables invoices | `jaykonta_create_quote`, `jaykonta_emit_invoice`, `miyuinvoice_facade` ; layer: domain |
| **[193] - Adapter Miyunotify** | `crates/jayfestival/src/services/miyunotify/adapter.rs` | [81] ou client Miyunotify | `miyunotify_send_announcement`, `miyunotify_send_targeted` ; layer: domain |
| **[194] - Adapter Miyubooking** | `crates/jayfestival/src/services/miyubooking/adapter.rs` | [81] ou client Miyubooking | `miyubooking_list_slots`, `miyubooking_create_booking` ; layer: domain |

| Id tÃ¢che | Fichier Ã  produire | DÃ©pendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[195] - Adapter MiyuClock** | `crates/jayfestival/src/services/miyuclock/adapter.rs` | miyuclock crate | `miyuclock_now`, `miyuclock_attest_date` ; layer: domain |
| **[196] - Router global et Ã©tat app** | `crates/jayfestival/src/app_state.rs` (ou `state.rs`) | [04], routers UNC/ORG/EXP/VIS, Auth | `app_state_current_screen`, `app_state_navigate`, `app_state_user` ; layer: app |

**RÃ©fÃ©rence** : [Miyukini Conceptual References - Interpolarite Services Jay](..//..//miyukini-webway-system//reference//_index.md) ; JayKoa = donnÃ©es + interface ; MiyuClock = attestation horaire/date IRL.

---

## Phase 11 â€” VÃ©rification, corrections et tests (protocole Phase 3)

**Objectif** : ConformitÃ© globale, tests, conformitÃ© MSCM, rÃ©gÃ©nÃ©ration index MIP.

### 11.1 TÃ¢ches Phase 11

| Id tÃ¢che | ActivitÃ© | Livrable |
|----------|----------|----------|
| **[201] - VÃ©rification globale** | IncohÃ©rences inter-fichiers, non-conformitÃ© docs, violations PROTO-1 Ã  PROTO-8 | Rapport ou checklist (pas de fichier code) |
| **[202] - Tests unitaires** | `cargo test` pour modules domain/auth/supabase/services | Fichiers `*_test.rs` ou `tests/` Ã  complÃ©ter |
| **[203] - ConformitÃ© MSCM** | VÃ©rifier @id, @do, @layer sur tous les blocs ; pas de bloc orphelin | Checklist Â§ 5.4 protocole ImplÃ©mentation |
| **[204] - RÃ©gÃ©nÃ©ration MIP** | Lancer pipeline MIP (scan â†’ parse MSCM â†’ gÃ©nÃ©ration mscm_index/) | `mscm_index/` Ã  jour (registry.json, blocks.json, â€¦) |

**RÃ¨gle** : Toute correction = nouvelle tÃ¢che (nomenclature [xx]-[fichier]) ; Phase 2 du protocole sâ€™applique.

---

## Phase 12 â€” Gel et versionnement (protocole Phase 4)

**Objectif** : Document de gel, index MIP final, version explicite.

### 12.1 TÃ¢ches Phase 12

| Id tÃ¢che | ActivitÃ© | Livrable |
|----------|----------|----------|
| **[211] - Document de gel** | Liste exhaustive des Ã©lÃ©ments gelÃ©s (fichiers, blocs MSCM, Ã©crans) | `docs/services/JayFestival/JayFestival - Gel Implementation Alpha vX.Y.Z.md` |
| **[212] - Index MIP final** | GÃ©nÃ©ration et archivage `mscm_index/` ; vÃ©rification integrity: "ok" | `mscm_index/registry.json` + tous les fichiers Â§ 6 protocole MIP |
| **[213] - Version** | Attribution version (ex. v0.1.0-alpha) ; rÃ¨gles dâ€™Ã©volution ; conditions de dÃ©gel | Mention dans Document de gel + tag git si applicable |

---

## Todo list dâ€™implÃ©mentation (synthÃ¨se)

Les tÃ¢ches sont Ã  exÃ©cuter **dans lâ€™ordre des phases** ; au sein dâ€™une phase, les tÃ¢ches peuvent Ãªtre parallÃ©lisÃ©es selon les dÃ©pendances (max 4 agents simultanÃ©s par groupe de prÃ©fixe).

### Phase 1 â€” Fondations
- [ ] [01] Crate et config
- [ ] [02] ThÃ¨me
- [ ] [03] Main et boucle app
- [ ] [04] Constantes Ã©crans

### Phase 2 â€” Atoms
- [ ] [11] IconWrapper
- [ ] [12] Button
- [ ] [13] Input
- [ ] [14] Label
- [ ] [21] Badge
- [ ] [22] Checkbox
- [ ] [23] Select
- [ ] [24] mod atoms

### Phase 2 â€” Molecules
- [ ] [31] FeatureCard
- [ ] [32] DirectoryCard
- [ ] [33] RoleCard
- [ ] [34] CTACard
- [ ] [41] Card
- [ ] [42] mod molecules

### Phase 3 â€” Organisms et Layout
- [ ] [51] Header
- [ ] [52] HeaderWithEdition
- [ ] [53] HeroSection
- [ ] [54] FeaturesGrid
- [ ] [61] DirectoryBanner
- [ ] [62] RolesGrid
- [ ] [63] CTASection
- [ ] [64] Layout
- [ ] [71] GestionLayout
- [ ] [72] mod organisms

### Phase 4 â€” Supabase et Auth
- [ ] [81] Client Supabase
- [ ] [82] Types Supabase
- [ ] [83] Auth service
- [ ] [84] RLS et permissions

### Phase 5 â€” Ã‰crans UNC
- [ ] [91] UNC-E01 Landing
- [ ] [92] UNC-E02 Liste Ã©vÃ©nements
- [ ] [93] UNC-E03 Fiche Ã©vÃ©nement
- [ ] [94] UNC-E06 Liste organisateurs
- [ ] [101] UNC-E07 Fiche organisateur
- [ ] [102] UNC-E08 Liste exposants
- [ ] [103] UNC-E09 Fiche exposant
- [ ] [104] UNC-E10 Recherche
- [ ] [111] UNC-E11 CTA contextuels
- [ ] [112] UNC-E12 Connexion
- [ ] [113] UNC-E13 Inscription
- [ ] [114] UNC-E14 Mentions
- [ ] [115] Router UNC

### Phase 6 â€” JayXpose
- [ ] [121] JayXpose client
- [ ] [122] Contrat JayXpose

### Phase 7 â€” Ã‰crans ORG
- [ ] [131]â€“[134] ORG-E04 Ã  E07
- [ ] [141]â€“[144] ORG-E08 Ã  E11
- [ ] [151]â€“[154] ORG-E12 Ã  E15
- [ ] [161]â€“[162] ORG reste + Router ORG

### Phase 8 â€” Ã‰crans EXP
- [ ] [171]â€“[175] EXP dashboard, candidatures, participations, reste, Router EXP

### Phase 9 â€” Ã‰crans VIS
- [ ] [181]â€“[184] VIS dashboard, agenda, billets/rÃ©servations, Router VIS

### Phase 10 â€” IntÃ©grations
- [ ] [191] Adapter JayKoa
- [ ] [192] Adapter JayKonta / Miyuinvoice
- [ ] [193] Adapter Miyunotify
- [ ] [194] Adapter Miyubooking
- [ ] [195] Adapter MiyuClock
- [ ] [196] Router global et Ã©tat app

### Phase 11 â€” VÃ©rification
- [ ] [201] VÃ©rification globale
- [ ] [202] Tests unitaires
- [ ] [203] ConformitÃ© MSCM
- [ ] [204] RÃ©gÃ©nÃ©ration MIP

### Phase 12 â€” Gel
- [ ] [211] Document de gel
- [ ] [212] Index MIP final
- [ ] [213] Version

---

## RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [Miyukini Prompt Protocol - ImplÃ©mentation gÃ©nÃ©rale](..//..//_index.md) | Cycle Planification â†’ Distribution â†’ VÃ©rification â†’ Gel ; nomenclature ; MSCM. |
| [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Structure mscm_index/, rÃ¨gles intÃ©gritÃ©. |
| [JayFestival - Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md) | PÃ©rimÃ¨tre alpha, phase 2, critÃ¨res CF-ALPHA-* |
| [JayFestival - Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) | PROTO-1 Ã  PROTO-8, ordre construction, composants et Ã©crans. |
| [JayFestival - Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) | Tables Supabase, RLS, mapping services. |
| [JayFestival - Etat Documentation Services Interfaces](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md) | Ã‰tat doc chaque service ; dÃ©cisions P0/P1. |
| [JayXpose - Analyse des besoins](../JayXpose/JayXpose%20-%20Analyse%20des%20besoins.md) | Besoins fiche exposant, rÃ©pertoire. |
| [JayXpose - Base de donnees Supabase](../JayXpose/reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md) | Tables exposants, requÃªtes alpha. |

---

**Document** : JayFestival â€” Plan dâ€™implÃ©mentation exhaustif  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de rÃ©fÃ©rence â€” plan dâ€™implÃ©mentation (phases, nomenclature, MSCM, todo list)



