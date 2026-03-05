<!-- @id mem.project.miyuki_ui
     @do provide_miyuki_ui_project_decisions
     @role project
     @layer memory
     @human Décisions projet Miyuki UI — tokens, dioxus, egui -->

# Bibliothèque Miyuki UI (confirmé mars 2026)

> Librairie UI unifiée Miyukini, chantier T5 livré. Architecture « Design Tokens + Adaptateurs ».

**3 crates dans `crates/`** :
- `miyuki-ui-tokens` (Strate 5) : Design tokens agnostiques, zero dep graphique. Couleurs (`Rgba`), spacing (base 4px, 11 niveaux), typographie, radius, shadow, animation, z-index. 2 themes `const` : `COG_THEME` (Miyukini Gaming) + `D2_THEME` (Sodomight Medieval). Feature `serde` optionnelle. 15 fichiers, 54 tests.
- `miyuki-ui-dioxus` (Strate 6) : Adaptateur Dioxus 0.6 Atomic Design. 14 atoms + 10 molecules + 10 organisms + 7 templates = 41 composants. Hooks `use_theme()`, `use_palette()`. Consommateurs : apps/central + 8 apps standalone. 47 fichiers, 7 tests.
- `miyuki-ui-egui` (Strate 6) : Adaptateur egui 0.28, reproduction fidele UI Diablo 2. 12 atoms + 11 molecules + 18 organisms + 6 templates = 47 composants. D2-specific : orbs, slots, quality text, skill tree, inventory, HUD, trade, stash, horadric cube. 53 fichiers, 81 tests.

**Decisions verrouillees** :
- Themes `const` (pas runtime, pas serde par defaut) pour zero-cost
- Workspaces separes (COG + MGE), tokens partages par `path = "../../crates/miyuki-ui-tokens"`
- Hierarchie Atomic Design stricte : atoms ne dependent que de tokens, molecules composent atoms, etc.
- miyuwidgets reste independant (usage HTML serveur different)
- MSCM 100% (115/115 fichiers annotes, prefixes MUIT/MUID/MUIE)

**Metriques** : 115 fichiers, 88 composants, 142 tests, 0 clippy warnings, 0 unsafe, 0 unwrap en production.

**Reste a faire** : Sprint 6 migration apps existantes (central + 8 services) vers miyuki-ui-dioxus. Sprint 7 integration MGE workspace.
