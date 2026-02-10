# JayFestival — Plan d’implémentation exhaustif

## Contexte

Ce document est le **plan d’implémentation** du service **JayFestival** et de ses **services dépendants** (JayXpose, JayKoa, JayKonta, Miyu*, Supabase alpha). Il est **exhaustif et précis**, divisé en **phases**, et respecte les protocoles :

- [Miyukini Prompt Protocol - Implémentation générale](../../protocols/Miyukini%20Prompt%20Protocol%20-%20Implémentation%20générale.md)
- [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)

**Références** : [Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md), [Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md), [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md), [Etat Documentation Services Interfaces](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md).

## Portée / Scope

- **Périmètre** : Implémentation alpha JayFestival (Dioxus, Supabase backend) + services dépendants dans le scope alpha (JayXpose, intégrations Miyu*, JayKoa, JayKonta).
- **Nomenclature des tâches** : `[xx] - [nom du fichier à produire]` ; `xx` = préfixe de regroupement (01, 02, …) ; **maximum 4 tâches par préfixe** pour exécution parallèle (max 4 agents simultanés).
- **Règles** : 1 étape = 1 fichier ; 1 agent = 1 tâche ; contexte vierge pour chaque délégation ; balisage MSCM obligatoire ; index MIP régénéré en Phase 4.

---

## Phase 0 — Planification (protocole)

### 0.1 Titre de l’étape

Planification globale JayFestival et services dépendants.

### 0.2 Explication rapide

- **Objectif** : Livrer une version **alpha fonctionnelle** de JayFestival (reprise Catakana en Dioxus, backend Supabase), avec catalogue (événements, organisateurs, exposants), espaces organisateur / exposant / visiteur, et intégrations documentées (JayXpose, JayKoa, JayKonta, Miyuinvoice, Miyunotify, Miyubooking, MiyuClock).
- **Périmètre** : Création crate(s) JayFestival, UI conforme [Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md), parcours UNC / ORG / EXP / VIS selon docs Écrans et cycle, client Supabase (Auth + REST), services dépendants dans le scope alpha.
- **Limites** : JayFaim hors alpha ; KindMother/SQLite post-alpha ; pas de migration de données dans ce plan (documentée à part).

### 0.3 Sélection du modèle IA (obligatoire en entête de tout prompt)

```
COMPLEXITÉ : Complexe | Extreme (selon phase)
CHARGE CONTEXTUELLE : Moyenne | Élevée

MODÈLE AUTORISÉ :
- Phase 0–2 (thème, atoms, molecules) → 1 modèle premium (Composer / Sonnet)
- Phase 3+ (écrans, intégrations) → LLM étendu si nécessaire

MODE IA ACTIF : AI Mode 1 | AI Mode 2
```

### 0.4 Cadre de travail (agents)

- **Documentation autorisée (liste fermée)** : JayFestival (Document fondateur, Bornage, Specification UI, Reference UI, Reference Base de Donnees, Etat Documentation Services, Interpolarite) ; JayXpose (Document fondateur, Analyse besoins, Parcours, Operateurs, Ecrans, Reference Base de donnees Supabase) ; publics JayFestival (Exposants, Organisateurs, Visiteurs, UNC — Analyse, Ecrans et cycle, Operateurs) ; protocoles Implémentation générale et MIP v1.
- **Outils autorisés** : Édition fichiers Rust/md, grep, lecture docs, exécution tests `cargo test`.
- **Outils interdits** : Modification manuelle de `mscm_index/` (index MIP généré uniquement par pipeline).

### 0.5 Contraintes absolues

- Ne pas anticiper les étapes suivantes.
- Ne pas fusionner plusieurs fichiers en une seule livraison.
- Ne pas corriger hors périmètre de la tâche.
- Arrêt immédiat si ambiguïté bloquante, dépendance manquante ou contexte insuffisant.

### 0.6 Tests

- Tests unitaires console (`cargo test`) pour modules logique métier et services lorsque possible.
- Justification explicite si absence de tests (ex. UI pure Dioxus).

### 0.7 Mini log de planification

| Élément | Décision / Risque |
|--------|-------------------|
| **Ambiguïtés** | Toutes P0/P1 tranchées (Miyuinvoice+JayKonta, JayXpose alpha, Miyuprofile Supabase, JayKoa/MiyuClock). |
| **Dépendances critiques** | JayXpose (fiche exposant, répertoire) dans alpha ; Supabase Auth + tables (profiles, exposants, editions, editions_exposants, etc.). |
| **Décisions structurantes** | Ordre de construction UI : Thème → Atoms → Molecules → Organisms → Layout → Écrans ; 1 écran = 1 module (ou groupe cohérent) avec nomenclature [xx]-[screen_id]. |

---

## Phase 1 — Fondations projet et thème (crate, config, UI base)

**Objectif** : Créer la crate JayFestival (ou binaire dans un crate existant), dépendances Cargo, et **thème** conforme Specification UI (tokens : couleurs, rayons, espacements, polices). Aucun composant atom/molecule ne doit exister avant le thème.

### 1.1 Tâches Phase 1

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus (@id, @do, @layer) |
|----------|--------------------|-------------|----------------------------------------|
| **[01] - Crate et config** | `crates/jayfestival/Cargo.toml` + `crates/jayfestival/src/lib.rs` (stub) | — | `jayfestival_crate_config`, `jayfestival_lib_stub` ; layer: infra |
| **[02] - Thème** | `crates/jayfestival/src/theme.rs` (ou `ui/theme.rs`) | [01] | `jayfestival_theme_struct`, `jayfestival_theme_tokens` (couleurs, borders.radius, spacing, fonts.sizes) ; layer: ui |
| **[03] - Main et boucle app** | `crates/jayfestival/src/main.rs` (Dioxus) | [01], [02] | `jayfestival_main_entry`, `jayfestival_app_loop` ; layer: app |
| **[04] - Constantes écrans** | `crates/jayfestival/src/screens.rs` (ScreenId enum / consts) | [01] | `jayfestival_screen_ids` ; layer: app |

**Balisage MSCM** : Chaque bloc avec `@id` unique, `@do` (description fonctionnelle), `@layer` (infra | ui | app). Optionnel : `@role`, `@human`.

**Ordre d’exécution** : [01] puis [02], [03], [04] en parallèle possible (02, 03, 04 dépendent de 01 uniquement).

---

## Phase 2 — Atoms et Molecules (Specification UI § 2)

**Objectif** : Implémenter tous les **atoms** puis toutes les **molecules** dans l’ordre imposé par la Specification UI (PROTO-2). Aucun style en dur (PROTO-1) ; tous les tokens viennent du thème.

### 2.1 Atoms (ordre obligatoire)

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[11] - IconWrapper** | `crates/jayfestival/src/ui/atoms/icon_wrapper.rs` | [02] | `atom_icon_wrapper`, `icon_wrapper_render` ; layer: ui |
| **[12] - Button** | `crates/jayfestival/src/ui/atoms/button.rs` | [02] | `atom_button`, `button_render`, variants Primary/Secondary/Outline/Ghost ; layer: ui |
| **[13] - Input** | `crates/jayfestival/src/ui/atoms/input.rs` | [02] | `atom_input`, `input_render` ; layer: ui |
| **[14] - Label** | `crates/jayfestival/src/ui/atoms/label.rs` | [02] | `atom_label`, `label_render`, levels Heading/Body/Small/Muted ; layer: ui |

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[21] - Badge** | `crates/jayfestival/src/ui/atoms/badge.rs` | [02] | `atom_badge`, `badge_render`, variants Default/Success/Warning/Error ; layer: ui |
| **[22] - Checkbox** | `crates/jayfestival/src/ui/atoms/checkbox.rs` | [02] | `atom_checkbox`, `checkbox_render` ; layer: ui |
| **[23] - Select** | `crates/jayfestival/src/ui/atoms/select.rs` | [02] | `atom_select`, `select_render` ; layer: ui |
| **[24] - mod atoms** | `crates/jayfestival/src/ui/atoms/mod.rs` | [11]–[23] | réexport uniquement ; pas de bloc métier |

**Ordre** : [11] → [12] → [13] → [14] (séquentiel ou 11 puis 12,13,14 en parallèle). Puis [21]–[23] puis [24].

### 2.2 Molecules (ordre obligatoire)

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[31] - FeatureCard** | `crates/jayfestival/src/ui/molecules/feature_card.rs` | atoms | `molecule_feature_card`, `feature_card_render` ; layer: ui |
| **[32] - DirectoryCard** | `crates/jayfestival/src/ui/molecules/directory_card.rs` | atoms | `molecule_directory_card`, `directory_card_render` ; layer: ui |
| **[33] - RoleCard** | `crates/jayfestival/src/ui/molecules/role_card.rs` | atoms | `molecule_role_card`, `role_card_render` ; layer: ui |
| **[34] - CTACard** | `crates/jayfestival/src/ui/molecules/cta_card.rs` | atoms | `molecule_cta_card`, `cta_card_render` ; layer: ui |

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[41] - Card** | `crates/jayfestival/src/ui/molecules/card.rs` | atoms | `molecule_card`, `card_render` (shadcn-like) ; layer: ui |
| **[42] - mod molecules** | `crates/jayfestival/src/ui/molecules/mod.rs` | [31]–[41] | réexport ; layer: ui |

**Dépendances inter-blocs** : Chaque molecule dépend des atoms (IconWrapper, Button, Label, Badge, etc.) ; déclarer dans MSCM si le parseur le permet.

---

## Phase 3 — Organisms et Layout (Specification UI § 2)

**Objectif** : Implémenter les **organisms** puis les **layouts** (Layout, GestionLayout). Ordre : Header → HeaderWithEdition → HeroSection → FeaturesGrid → DirectoryBanner → RolesGrid → CTASection → Layout → GestionLayout.

### 3.1 Organisms

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[51] - Header** | `crates/jayfestival/src/ui/organisms/header.rs` | molecules, atoms | `organism_header`, `header_render` ; layer: ui |
| **[52] - HeaderWithEdition** | `crates/jayfestival/src/ui/organisms/header_with_edition.rs` | [51], Select | `organism_header_with_edition`, `header_with_edition_render` ; layer: ui |
| **[53] - HeroSection** | `crates/jayfestival/src/ui/organisms/hero_section.rs` | molecules, atoms | `organism_hero_section`, `hero_section_render` ; layer: ui |
| **[54] - FeaturesGrid** | `crates/jayfestival/src/ui/organisms/features_grid.rs` | FeatureCard | `organism_features_grid`, `features_grid_render` ; layer: ui |

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[61] - DirectoryBanner** | `crates/jayfestival/src/ui/organisms/directory_banner.rs` | DirectoryCard | `organism_directory_banner`, `directory_banner_render` ; layer: ui |
| **[62] - RolesGrid** | `crates/jayfestival/src/ui/organisms/roles_grid.rs` | RoleCard | `organism_roles_grid`, `roles_grid_render` ; layer: ui |
| **[63] - CTASection** | `crates/jayfestival/src/ui/organisms/cta_section.rs` | CTACard | `organism_cta_section`, `cta_section_render` ; layer: ui |
| **[64] - Layout** | `crates/jayfestival/src/ui/organisms/layout.rs` | Header | `organism_layout`, `layout_render` (SidePanel + CentralPanel) ; layer: ui |

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[71] - GestionLayout** | `crates/jayfestival/src/ui/organisms/gestion_layout.rs` | [51], [52], [64] | `organism_gestion_layout`, `gestion_layout_render` ; layer: ui |
| **[72] - mod organisms** | `crates/jayfestival/src/ui/organisms/mod.rs` | [51]–[71] | réexport ; layer: ui |

---

## Phase 4 — Client Supabase et Auth (backend alpha)

**Objectif** : Client Supabase (REST + Auth) pour l’alpha ; pas de Miyauth natif. Session, rôles via `profiles.user_type`. Écrans Connexion / Inscription (formulaires) utilisés par tous les publics.

### 4.1 Client et types

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[81] - Client Supabase** | `crates/jayfestival/src/supabase/client.rs` | [01] | `supabase_client_init`, `supabase_client_request` ; layer: infra |
| **[82] - Types Supabase** | `crates/jayfestival/src/supabase/types.rs` | — | `supabase_profiles`, `supabase_editions`, `supabase_exposants`, `supabase_editions_exposants` (structs alignés Reference Base de Donnees) ; layer: domain |
| **[83] - Auth service** | `crates/jayfestival/src/auth/mod.rs` (+ `sign_in.rs`, `sign_up.rs`, `session.rs` si découpé) | [81], [82] | `auth_sign_in`, `auth_sign_up`, `auth_session_current`, `auth_sign_out` ; layer: domain |
| **[84] - RLS et permissions** | `crates/jayfestival/src/auth/permissions.rs` | [82] | `auth_user_type_from_profile`, `auth_can_access_edition` ; layer: domain |

**Référence** : [Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) (tables, RLS).

---

## Phase 5 — Écrans catalogue (Utilisateur non connecté — UNC)

**Objectif** : Écrans UNC-E01 à UNC-E14 selon [Specification UI § 4.1](.\JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) et docs [Utilisateur non connecté — Écrans et cycle](./publics/UtilisateurNonConnecte/UtilisateurNonConnecte%20-%20Ecrans%20et%20cycle.md). Ordre des zones et composants strict (PROTO-4).

### 5.1 Tâches Phase 5 (1 écran = 1 fichier ou 1 module)

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[91] - UNC-E01 Landing** | `crates/jayfestival/src/screens/unc/e01_landing.rs` | Layout, HeroSection, FeaturesGrid, DirectoryBanner, RolesGrid, CTASection | `screen_unc_e01`, `unc_e01_show` ; layer: app |
| **[92] - UNC-E02 Liste événements** | `crates/jayfestival/src/screens/unc/e02_liste_evenements.rs` | Layout, Card, Input, Select, Button | `screen_unc_e02`, `unc_e02_show` ; layer: app |
| **[93] - UNC-E03 Fiche événement** | `crates/jayfestival/src/screens/unc/e03_fiche_evenement.rs` | Layout, Card, Label, Button | `screen_unc_e03`, `unc_e03_show` ; layer: app |
| **[94] - UNC-E06 Liste organisateurs** | `crates/jayfestival/src/screens/unc/e06_liste_organisateurs.rs` | Layout, Card, Input, Select | `screen_unc_e06`, `unc_e06_show` ; layer: app |

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[101] - UNC-E07 Fiche organisateur** | `crates/jayfestival/src/screens/unc/e07_fiche_organisateur.rs` | Layout, Card, Label, Button | `screen_unc_e07`, `unc_e07_show` ; layer: app |
| **[102] - UNC-E08 Liste exposants** | `crates/jayfestival/src/screens/unc/e08_liste_exposants.rs` | Layout, Card, Input, Select (données JayXpose/Supabase) | `screen_unc_e08`, `unc_e08_show` ; layer: app |
| **[103] - UNC-E09 Fiche exposant** | `crates/jayfestival/src/screens/unc/e09_fiche_exposant.rs` | Layout, Card (données JayXpose/Supabase) | `screen_unc_e09`, `unc_e09_show` ; layer: app |
| **[104] - UNC-E10 Recherche** | `crates/jayfestival/src/screens/unc/e10_recherche.rs` | Layout, Input, Card, Select | `screen_unc_e10`, `unc_e10_show` ; layer: app |

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[111] - UNC-E11 CTA contextuels** | `crates/jayfestival/src/screens/unc/e11_cta_contextuels.rs` | Window/Modal, Label, Button | `screen_unc_e11`, `unc_e11_show` ; layer: app |
| **[112] - UNC-E12 Connexion** | `crates/jayfestival/src/screens/unc/e12_connexion.rs` | Input, Button, Label, [83] Auth | `screen_unc_e12`, `unc_e12_show` ; layer: app |
| **[113] - UNC-E13 Inscription** | `crates/jayfestival/src/screens/unc/e13_inscription.rs` | RoleCard/CTACard, Button, [83] Auth | `screen_unc_e13`, `unc_e13_show` ; layer: app |
| **[114] - UNC-E14 Mentions / CGU** | `crates/jayfestival/src/screens/unc/e14_mentions.rs` | Layout, Label, Button | `screen_unc_e14`, `unc_e14_show` ; layer: app |

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[115] - Router UNC** | `crates/jayfestival/src/screens/unc/mod.rs` | [91]–[114] | `router_unc`, `unc_navigate` ; layer: app |

**Données** : E02, E03, E06, E07, E08, E09, E10 consomment Supabase (éditions, organisateurs, exposants) ; E08/E09 alignés avec JayXpose (table `exposants`, répertoire). Voir [JayXpose - Base de donnees Supabase](./JayXpose/reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md).

---

## Phase 6 — Services dépendants alpha (JayXpose, données)

**Objectif** : Module **JayXpose** (fiche exposant, répertoire) consommé par JayFestival : lecture profil exposant, liste répertoire (filtres). Alpha = données Supabase (tables `exposants`, `editions_exposants`). Pas d’UI propre JayXpose dans ce plan — UI = écrans JayFestival (UNC-E08, E09, ORG exposants, etc.).

### 6.1 Tâches Phase 6

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[121] - JayXpose client** | `crates/jayfestival/src/services/jayxpose/client.rs` (ou `crates/jayxpose/` si crate dédiée) | [81], [82] | `jayxpose_get_profile`, `jayxpose_list_repertoire`, `jayxpose_fiche_by_id` ; layer: domain |
| **[122] - Contrat JayXpose** | `crates/jayfestival/src/services/jayxpose/contract.rs` (types entrée/sortie) | [82] | `jayxpose_profile_type`, `jayxpose_repertoire_filters` ; layer: domain |
| **[123] - Intégration écrans** | Utilisation dans E08, E09, ORG exposants (déjà prévus en Phase 5 / 7) | [121], [122] | — (pas de fichier dédié ; les écrans appellent le client) |

**Référence** : [JayXpose - Analyse des besoins](../JayXpose/JayXpose%20-%20Analyse%20des%20besoins.md), [JayXpose - Base de donnees Supabase](../JayXpose/reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md).

---

## Phase 7 — Parcours Organisateurs (écrans ORG)

**Objectif** : Écrans ORG-E04 à ORG-E25 (ou sous-ensemble alpha) selon [Specification UI § 4.2](.\JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) et [Organisateurs — Écrans et cycle](./publics/Organisateurs/Organisateurs%20-%20Ecrans%20et%20cycle.md). GestionLayout, HeaderWithEdition, données Supabase (éditions, editions_exposants, stands, budget_entries, invoices, schedule_slots, documents).

### 7.1 Tâches Phase 7 (groupes de 4 max par préfixe)

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[131] - ORG-E04 Dashboard** | `crates/jayfestival/src/screens/org/e04_dashboard.rs` | GestionLayout, HeaderWithEdition, Card, [81] | `screen_org_e04`, `org_e04_show` ; layer: app |
| **[132] - ORG-E05 Liste éditions** | `crates/jayfestival/src/screens/org/e05_liste_editions.rs` | GestionLayout, Card, Select, Button | `screen_org_e05`, `org_e05_show` ; layer: app |
| **[133] - ORG-E06 Création édition** | `crates/jayfestival/src/screens/org/e06_creation_edition.rs` | GestionLayout, Input, Button | `screen_org_e06`, `org_e06_show` ; layer: app |
| **[134] - ORG-E07 Dashboard édition** | `crates/jayfestival/src/screens/org/e07_dashboard_edition.rs` | GestionLayout, Card, Badge | `screen_org_e07`, `org_e07_show` ; layer: app |

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[141] - ORG-E08 Liste exposants** | `crates/jayfestival/src/screens/org/e08_liste_exposants.rs` | GestionLayout, Card, Badge, [121] | `screen_org_e08`, `org_e08_show` ; layer: app |
| **[142] - ORG-E09 Candidatures** | `crates/jayfestival/src/screens/org/e09_candidatures.rs` | GestionLayout, Card, Badge, Button | `screen_org_e09`, `org_e09_show` ; layer: app |
| **[143] - ORG-E10 Fiche exposant (org)** | `crates/jayfestival/src/screens/org/e10_fiche_exposant.rs` | GestionLayout, Card, [121] | `screen_org_e10`, `org_e10_show` ; layer: app |
| **[144] - ORG-E11 Plan de salle** | `crates/jayfestival/src/screens/org/e11_plan_salle.rs` | GestionLayout, zones/stands (widget ou canvas) | `screen_org_e11`, `org_e11_show` ; layer: app |

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[151] - ORG-E12 Programme** | `crates/jayfestival/src/screens/org/e12_programme.rs` | GestionLayout, Card, liste créneaux/salles | `screen_org_e12`, `org_e12_show` ; layer: app |
| **[152] - ORG-E13 Budget** | `crates/jayfestival/src/screens/org/e13_budget.rs` | GestionLayout, Card, Input (revenus/dépenses) | `screen_org_e13`, `org_e13_show` ; layer: app |
| **[153] - ORG-E14 Devis / Factures** | `crates/jayfestival/src/screens/org/e14_devis_factures.rs` | GestionLayout, Card (Miyuinvoice/JayKonta) | `screen_org_e14`, `org_e14_show` ; layer: app |
| **[154] - ORG-E15 Documents** | `crates/jayfestival/src/screens/org/e15_documents.rs` | GestionLayout, Card, Button | `screen_org_e15`, `org_e15_show` ; layer: app |

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[161] - ORG-E16–E25 (reste)** | Fichiers séparés ou regroupés (Annonces, Notifications, Mon compte, Équipe, etc.) | GestionLayout, atoms/molecules | `screen_org_eXX`, `org_eXX_show` ; layer: app |
| **[162] - Router ORG** | `crates/jayfestival/src/screens/org/mod.rs` | [131]–[161] | `router_org`, `org_navigate` ; layer: app |

**Note** : Écrans ORG-E12 (Programme), E13 (Budget), E14 (Devis/Factures) s’appuient sur les flux documentés dans [JayKonta - Integration Services](../JayKonta/reference/JayKonta%20-%20Integration%20Services.md) et [Miyuinvoice] ; en alpha, lecture/écriture via Supabase (tables budget_entries, invoices).

---

## Phase 8 — Parcours Exposants (écrans EXP)

**Objectif** : Écrans EXP selon [Exposants — Écrans et cycle](./publics/Exposants/Exposants%20-%20Ecrans%20et%20cycle.md) et Specification UI : dashboard exposant, candidatures, participations, documents, factures.

### 8.1 Tâches Phase 8

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[171] - EXP-E04 Dashboard exposant** | `crates/jayfestival/src/screens/exp/e04_dashboard.rs` | GestionLayout, Card, [81], [121] | `screen_exp_e04`, `exp_e04_show` ; layer: app |
| **[172] - EXP-E05 Candidatures** | `crates/jayfestival/src/screens/exp/e05_candidatures.rs` | GestionLayout, Card, Badge | `screen_exp_e05`, `exp_e05_show` ; layer: app |
| **[173] - EXP-E06 Participations** | `crates/jayfestival/src/screens/exp/e06_participations.rs` | GestionLayout, Card | `screen_exp_e06`, `exp_e06_show` ; layer: app |
| **[174] - EXP-E07–E19 (reste)** | Fichiers par écran (Documents, Factures, Mon compte, etc.) | GestionLayout, atoms/molecules | `screen_exp_eXX`, `exp_eXX_show` ; layer: app |

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[175] - Router EXP** | `crates/jayfestival/src/screens/exp/mod.rs` | [171]–[174] | `router_exp`, `exp_navigate` ; layer: app |

---

## Phase 9 — Parcours Visiteurs (écrans VIS)

**Objectif** : Écrans VIS selon [Visiteurs — Écrans et cycle](./publics/Visiteurs/Visiteurs%20-%20Ecrans%20et%20cycle.md) : espace visiteur (agenda, billets, réservations, pass). Intégration Miyubooking (créneaux, réservation) documentée ; en alpha données Supabase.

### 9.1 Tâches Phase 9

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[181] - VIS-E04 Dashboard visiteur** | `crates/jayfestival/src/screens/vis/e04_dashboard.rs` | GestionLayout, Card | `screen_vis_e04`, `vis_e04_show` ; layer: app |
| **[182] - VIS-E05 Agenda** | `crates/jayfestival/src/screens/vis/e05_agenda.rs` | GestionLayout, vue calendrier (JayKoa) ou liste | `screen_vis_e05`, `vis_e05_show` ; layer: app |
| **[183] - VIS-E06–E15 (billets, réservations, pass)** | Fichiers par écran | GestionLayout, Card, Button | `screen_vis_eXX`, `vis_eXX_show` ; layer: app |
| **[184] - Router VIS** | `crates/jayfestival/src/screens/vis/mod.rs` | [181]–[183] | `router_vis`, `vis_navigate` ; layer: app |

---

## Phase 10 — Intégrations (JayKoa, JayKonta, Miyunotify, Miyubooking, MiyuClock)

**Objectif** : Modules d’intégration **appelants** (JayFestival appelle les services) ; pas d’implémentation des services eux-mêmes. Contrats documentés dans [Etat Documentation Services Interfaces](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md) et [Interpolarite Services Jay](./reference/JayFestival%20-%20Interpolarite%20Services%20Jay.md).

### 10.1 Tâches Phase 10

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[191] - Adapter JayKoa** | `crates/jayfestival/src/services/jaykoa/adapter.rs` | [81] ou client JayKoa si crate | `jaykoa_publish_edition`, `jaykoa_get_conflicts` ; layer: domain |
| **[192] - Adapter JayKonta / Miyuinvoice** | `crates/jayfestival/src/services/jaykonta/adapter.rs` | [81], tables invoices | `jaykonta_create_quote`, `jaykonta_emit_invoice`, `miyuinvoice_facade` ; layer: domain |
| **[193] - Adapter Miyunotify** | `crates/jayfestival/src/services/miyunotify/adapter.rs` | [81] ou client Miyunotify | `miyunotify_send_announcement`, `miyunotify_send_targeted` ; layer: domain |
| **[194] - Adapter Miyubooking** | `crates/jayfestival/src/services/miyubooking/adapter.rs` | [81] ou client Miyubooking | `miyubooking_list_slots`, `miyubooking_create_booking` ; layer: domain |

| Id tâche | Fichier à produire | Dépendances | Blocs MSCM attendus |
|----------|--------------------|-------------|--------------------|
| **[195] - Adapter MiyuClock** | `crates/jayfestival/src/services/miyuclock/adapter.rs` | miyuclock crate | `miyuclock_now`, `miyuclock_attest_date` ; layer: domain |
| **[196] - Router global et état app** | `crates/jayfestival/src/app_state.rs` (ou `state.rs`) | [04], routers UNC/ORG/EXP/VIS, Auth | `app_state_current_screen`, `app_state_navigate`, `app_state_user` ; layer: app |

**Référence** : [Miyukini Conceptual References - Interpolarite Services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) ; JayKoa = données + interface ; MiyuClock = attestation horaire/date IRL.

---

## Phase 11 — Vérification, corrections et tests (protocole Phase 3)

**Objectif** : Conformité globale, tests, conformité MSCM, régénération index MIP.

### 11.1 Tâches Phase 11

| Id tâche | Activité | Livrable |
|----------|----------|----------|
| **[201] - Vérification globale** | Incohérences inter-fichiers, non-conformité docs, violations PROTO-1 à PROTO-8 | Rapport ou checklist (pas de fichier code) |
| **[202] - Tests unitaires** | `cargo test` pour modules domain/auth/supabase/services | Fichiers `*_test.rs` ou `tests/` à compléter |
| **[203] - Conformité MSCM** | Vérifier @id, @do, @layer sur tous les blocs ; pas de bloc orphelin | Checklist § 5.4 protocole Implémentation |
| **[204] - Régénération MIP** | Lancer pipeline MIP (scan → parse MSCM → génération mscm_index/) | `mscm_index/` à jour (registry.json, blocks.json, …) |

**Règle** : Toute correction = nouvelle tâche (nomenclature [xx]-[fichier]) ; Phase 2 du protocole s’applique.

---

## Phase 12 — Gel et versionnement (protocole Phase 4)

**Objectif** : Document de gel, index MIP final, version explicite.

### 12.1 Tâches Phase 12

| Id tâche | Activité | Livrable |
|----------|----------|----------|
| **[211] - Document de gel** | Liste exhaustive des éléments gelés (fichiers, blocs MSCM, écrans) | `docs/services/JayFestival/JayFestival - Gel Implementation Alpha vX.Y.Z.md` |
| **[212] - Index MIP final** | Génération et archivage `mscm_index/` ; vérification integrity: "ok" | `mscm_index/registry.json` + tous les fichiers § 6 protocole MIP |
| **[213] - Version** | Attribution version (ex. v0.1.0-alpha) ; règles d’évolution ; conditions de dégel | Mention dans Document de gel + tag git si applicable |

---

## Todo list d’implémentation (synthèse)

Les tâches sont à exécuter **dans l’ordre des phases** ; au sein d’une phase, les tâches peuvent être parallélisées selon les dépendances (max 4 agents simultanés par groupe de préfixe).

### Phase 1 — Fondations
- [ ] [01] Crate et config
- [ ] [02] Thème
- [ ] [03] Main et boucle app
- [ ] [04] Constantes écrans

### Phase 2 — Atoms
- [ ] [11] IconWrapper
- [ ] [12] Button
- [ ] [13] Input
- [ ] [14] Label
- [ ] [21] Badge
- [ ] [22] Checkbox
- [ ] [23] Select
- [ ] [24] mod atoms

### Phase 2 — Molecules
- [ ] [31] FeatureCard
- [ ] [32] DirectoryCard
- [ ] [33] RoleCard
- [ ] [34] CTACard
- [ ] [41] Card
- [ ] [42] mod molecules

### Phase 3 — Organisms et Layout
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

### Phase 4 — Supabase et Auth
- [ ] [81] Client Supabase
- [ ] [82] Types Supabase
- [ ] [83] Auth service
- [ ] [84] RLS et permissions

### Phase 5 — Écrans UNC
- [ ] [91] UNC-E01 Landing
- [ ] [92] UNC-E02 Liste événements
- [ ] [93] UNC-E03 Fiche événement
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

### Phase 6 — JayXpose
- [ ] [121] JayXpose client
- [ ] [122] Contrat JayXpose

### Phase 7 — Écrans ORG
- [ ] [131]–[134] ORG-E04 à E07
- [ ] [141]–[144] ORG-E08 à E11
- [ ] [151]–[154] ORG-E12 à E15
- [ ] [161]–[162] ORG reste + Router ORG

### Phase 8 — Écrans EXP
- [ ] [171]–[175] EXP dashboard, candidatures, participations, reste, Router EXP

### Phase 9 — Écrans VIS
- [ ] [181]–[184] VIS dashboard, agenda, billets/réservations, Router VIS

### Phase 10 — Intégrations
- [ ] [191] Adapter JayKoa
- [ ] [192] Adapter JayKonta / Miyuinvoice
- [ ] [193] Adapter Miyunotify
- [ ] [194] Adapter Miyubooking
- [ ] [195] Adapter MiyuClock
- [ ] [196] Router global et état app

### Phase 11 — Vérification
- [ ] [201] Vérification globale
- [ ] [202] Tests unitaires
- [ ] [203] Conformité MSCM
- [ ] [204] Régénération MIP

### Phase 12 — Gel
- [ ] [211] Document de gel
- [ ] [212] Index MIP final
- [ ] [213] Version

---

## Références

| Document | Rôle |
|----------|------|
| [Miyukini Prompt Protocol - Implémentation générale](../../protocols/Miyukini%20Prompt%20Protocol%20-%20Implémentation%20générale.md) | Cycle Planification → Distribution → Vérification → Gel ; nomenclature ; MSCM. |
| [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) | Structure mscm_index/, règles intégrité. |
| [JayFestival - Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md) | Périmètre alpha, phase 2, critères CF-ALPHA-* |
| [JayFestival - Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md) | PROTO-1 à PROTO-8, ordre construction, composants et écrans. |
| [JayFestival - Reference Base de Donnees et Migration](./reference/JayFestival%20-%20Reference%20Base%20de%20Donnees%20et%20Migration%20Supabase%20vers%20SQLite.md) | Tables Supabase, RLS, mapping services. |
| [JayFestival - Etat Documentation Services Interfaces](./reference/JayFestival%20-%20Etat%20Documentation%20Services%20Interfaces.md) | État doc chaque service ; décisions P0/P1. |
| [JayXpose - Analyse des besoins](../JayXpose/JayXpose%20-%20Analyse%20des%20besoins.md) | Besoins fiche exposant, répertoire. |
| [JayXpose - Base de donnees Supabase](../JayXpose/reference/JayXpose%20-%20Base%20de%20donnees%20Supabase%20et%20Migration%20SQLite.md) | Tables exposants, requêtes alpha. |

---

**Document** : JayFestival — Plan d’implémentation exhaustif  
**Version** : 1.0  
**Date** : 2026-02-03  
**Statut** : Document de référence — plan d’implémentation (phases, nomenclature, MSCM, todo list)
