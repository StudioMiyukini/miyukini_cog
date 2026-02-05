# JayFestival — Audit du code et métriques

**Date :** 2026-02-03  
**Périmètre :** Crate `jayfestival` — bornage, documentation, protocole MIP, tests unitaires.  
**Références :** [Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md), [Plan Implementation](./JayFestival%20-%20Plan%20Implementation.md), [Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md), [Verification Phase 11 Rapport](./JayFestival%20-%20Verification%20Phase%2011%20Rapport.md).

---

## Contexte

Cet audit vérifie :
1. **Respect du bornage** et de la documentation (plan, specification UI, bornage alpha).
2. **Qualité du suivi du protocole MIP** (MSCM, index, intégrité).
3. **Tests unitaires** (présence, exécution, taux de réussite).
4. **Métrique globale** en %.

---

## 1. Audit bornage et documentation

### 1.1 Périmètre alpha (Bornage)

| Critère bornage | Attendu (alpha) | Implémenté | Écart |
|-----------------|-----------------|------------|--------|
| Crate + thème + main + ScreenId | [01]–[04] | Oui (lib, main, theme, screens/mod) | Aucun |
| Atoms (IconWrapper → Select) | [11]–[24] | 7 atoms + mod | Aucun |
| Molecules (FeatureCard → Card) | [31]–[42] | 5 molecules + mod | Aucun |
| Organisms + Layout + GestionLayout | [51]–[72] | 9 organisms + mod | Aucun |
| Client Supabase + types + Auth + RLS | [81]–[84] | client, types, auth, permissions | Aucun |
| Écrans UNC E01–E14 + Router UNC | [91]–[115] | 14 écrans UNC + mod | Aucun |
| JayXpose client + contrat | [121]–[122] | client, contract, mod | Aucun |
| Écrans ORG E04–E25 + Router ORG | [131]–[162] | 14 écrans ORG + mod | Aucun |
| Écrans EXP + Router EXP | [171]–[175] | 4 écrans EXP + mod | Aucun |
| Écrans VIS + Router VIS | [181]–[184] | 3 écrans VIS + mod | Aucun |
| Adapters (JayKoa, JayKonta, Miyunotify, Miyubooking, MiyuClock) | [191]–[195] | 5 adapters + mod | Aucun |
| Router global + AppState | [196] | app_state, app | Aucun |

**Hors scope alpha respecté :** Pas de KindMother/SQLite, pas de JayFaim, pas de migration de données dans le code audité. Backend alpha = Supabase (client + Auth).

**Dérives identifiées (documentation / Specification UI) :**

| Dérive | Sévérité | Détail |
|--------|----------|--------|
| PROTO-1 (style en dur) | Mineure | `ui/atoms/badge.rs` : couleur texte badge Warning en dur `Color32::from_rgb(24,24,27)` au lieu d’un token thème. |
| PROTO-6 (responsive 800 px) | Mineure | Thème documente 14/16 px ; pas d’usage de `ctx.screen_rect().width()` pour adapter sidebar ni taille de police au breakpoint 800 px (limite alpha documentée). |

**Score bornage et documentation :** 98 %  
- 100 % des fichiers/écrans du plan alpha présents.  
- −2 % pour les 2 dérives mineures (PROTO-1, PROTO-6).

---

## 2. Qualité du suivi du protocole MIP

### 2.1 Balisage MSCM

| Vérification | Résultat |
|--------------|----------|
| Blocs avec `@id` | 137 blocs jayfestival dans `mscm_index/blocks.json` |
| Blocs avec `@do` | Tous les blocs indexés ont une description fonctionnelle |
| Blocs avec `@layer` | Couches utilisées : `ui`, `app`, `domain`, `infra` — conformes au plan |
| Fichiers avec MSCM | 77 fichiers `.rs` contenant au moins un bloc `@id` |
| Blocs orphelins | Aucun (tout bloc indexé a @id) |

### 2.2 Index MIP

| Fichier index | Présent | Contenu / intégrité |
|---------------|---------|----------------------|
| `mscm_index/registry.json` | Oui | `version: "mip_v1"`, `mscm_version: "v1"`, `integrity: "ok"` |
| `mscm_index/blocks.json` | Oui | 1520 blocs (dont 137 jayfestival) |
| `mscm_index/files.json` | Oui | 71 entrées `crates/jayfestival/` |
| `mscm_index/hierarchy.json` | Oui | Présent |
| `mscm_index/graph.json` | Oui | Présent |
| `mscm_index/layers.json` | Oui | Présent |
| `mscm_index/domains.json` | Oui | Présent |
| `mscm_index/dependencies.json` | Oui | Présent |
| `mscm_index/stats.json` | Oui | Présent |

**Règles MIP :** Index généré uniquement par le pipeline (`tools/mip-generator`) ; pas de modification manuelle. Dernière régénération : Phase 11 [204].

**Score MIP :** 100 %  
- Index à jour, intégrité OK, jayfestival inclus, pas de modification manuelle.

---

## 3. Tests unitaires

### 3.1 Modules testés

| Module | Fichier | Tests | Rôle |
|--------|---------|-------|------|
| `supabase::types` | `supabase/types.rs` | `user_type_from_str_roundtrip`, `user_type_as_str_roundtrip` | UserType parse / as_str |
| `auth::permissions` | `auth/permissions.rs` | `auth_user_type_from_profile_known`, `auth_user_type_from_profile_unknown`, `auth_can_access_edition_admin_always`, `auth_can_access_edition_manager_only_when_is_manager` | RLS / permissions |
| `app_state` | `app_state.rs` | `app_state_default_landing`, `app_state_navigate_apply`, `app_state_set_current_screen` | Router / état global |

### 3.2 Exécution

```text
cargo test -p jayfestival
running 9 tests
test auth::permissions::tests::auth_can_access_edition_admin_always ... ok
test auth::permissions::tests::auth_can_access_edition_manager_only_when_is_manager ... ok
test app_state::tests::app_state_navigate_apply ... ok
test app_state::tests::app_state_set_current_screen ... ok
test auth::permissions::tests::auth_user_type_from_profile_known ... ok
test auth::permissions::tests::auth_user_type_from_profile_unknown ... ok
test app_state::tests::app_state_default_landing ... ok
test supabase::types::tests::user_type_as_str_roundtrip ... ok
test supabase::types::tests::user_type_from_str_roundtrip ... ok
test result: ok. 9 passed; 0 failed
```

### 3.3 Couverture (estimation)

- **Modules testables sans UI/mock :** supabase/types, auth/permissions, app_state (+ auth mod, supabase client avec mocks).
- **Modules avec tests :** 3 (types, permissions, app_state).
- **Couverture des modules « logique pure » :** 3/5 ≈ **60 %** (types, permissions, app_state couverts ; auth sign_in/sign_up et supabase client non couverts sans mock).
- **Taux de réussite des tests :** **100 %** (9/9).

**Score tests :** 85 %  
- 100 % de réussite ; présence de tests sur domain + app ; couverture partielle des modules testables (justification : UI egui et services I/O en alpha).

---

## 4. Synthèse et métrique globale

### 4.1 Scores par axe

| Axe | Score | Commentaire |
|-----|--------|-------------|
| Bornage et documentation | **98 %** | Plan alpha respecté ; 2 dérives mineures (PROTO-1, PROTO-6). |
| Protocole MIP | **100 %** | Index à jour, intégrité OK, MSCM conforme. |
| Tests unitaires | **85 %** | 9 tests, 100 % passants ; couverture partielle des modules testables. |

### 4.2 Métrique globale (moyenne pondérée)

Pondération proposée : Bornage 40 %, MIP 30 %, Tests 30 %.

**Score global = 0,40 × 98 + 0,30 × 100 + 0,30 × 85 = 39,2 + 30 + 25,5 = 94,7 %**

**Métrique globale : 95 %** (arrondi).

### 4.3 Recommandations

1. **PROTO-1 :** Ajouter un token `badge_warning_text()` dans le thème et l’utiliser dans `badge.rs` pour le texte du badge Warning.
2. **PROTO-6 :** Documenter en alpha l’absence de breakpoint dynamique 800 px ; ou implémenter `ctx.screen_rect().width()` pour sidebar et police (post-alpha).
3. **Tests :** Enrichir avec tests sur `auth::auth_sign_in` / `auth_sign_up` (mocks client Supabase) et sur helpers thème si exposés, pour viser ~80 % de couverture des modules testables.

---

## 5. Références

- [JayFestival - Bornage Implementation](./JayFestival%20-%20Bornage%20Implementation.md)
- [JayFestival - Plan Implementation](./JayFestival%20-%20Plan%20Implementation.md)
- [JayFestival - Specification UI Conforme Catakana](./JayFestival%20-%20Specification%20UI%20Conforme%20Catakana.md)
- [JayFestival - Verification Phase 11 Rapport](./JayFestival%20-%20Verification%20Phase%2011%20Rapport.md)
- [Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)

---

**Document :** JayFestival — Audit du code et métriques  
**Version :** 1.0  
**Date :** 2026-02-03  
**Statut :** Rapport d’audit (métrique globale 95 %)
