# P0 Temps 4 - Inventaire prerequis

## Statut

- Etat : Terminé
- Phase : P0 Temps 4
- Agents : Denis (lead) + Hugo + Jean + François + Lise
- Date : 2026-03-07

## TL;DR

4 blocs d'impact. Design system miyuki-ui-dioxus déjà structuré (atoms/molecules/organisms). Crates backend stables. COG Web Portal = nouveau bin axum (apps/cog-web-portal). Dioxus 0.7 confirmé (environment.md à corriger — indique 0.6). Blocage principal : Context7 ID Dioxus 0.7 à résoudre en T6.

## Crates / modules à modifier

### Bloc A — MSCM Audit (tous services Jay)

| Crate | Fichier(s) | Modification | Raison |
|-------|-----------|-------------|--------|
| `crates/jayfestival` | `src/**/*.rs` | Ajout/correction MSCM (`@id @role @layer @human`) | MSCM compliance |
| `crates/jayxpose` | `src/**/*.rs` | Ajout/correction MSCM | MSCM compliance |
| `crates/jaykoa` | `src/**/*.rs` | Audit MSCM only | MSCM V2 |
| `crates/jaykonta` | `src/**/*.rs` | Audit MSCM only | MSCM V2 |
| `crates/jaymanga` | `src/**/*.rs` | Audit MSCM only | MSCM V2 |
| `crates/jay1tribu` | `src/**/*.rs` | Audit MSCM only | MSCM V2 |
| `apps/central/src/services/jayfestival/` | tous `.rs` | Ajout MSCM | MSCM compliance |
| `apps/central/src/services/jayxpose/` | tous `.rs` | Ajout MSCM | MSCM compliance |

### Bloc B — JayFestival prod-ready

| Crate | Fichier(s) | Modification | Raison |
|-------|-----------|-------------|--------|
| `apps/central/src/services/jayfestival/` | `components.rs`, `sidebar.rs`, tous écrans | Refonte UI design system, MSCM | Qualité production |
| `crates/miyuki-ui-dioxus` | `atoms/`, `molecules/`, `organisms/` | Enrichir si composants manquants | Design system partagé |
| `crates/jayfestival` | `data/kindmother_db.rs` | Hardening (validation, MSCM) | Sécurité production |

### Bloc C — JayXpose prod-ready

| Crate | Fichier(s) | Modification | Raison |
|-------|-----------|-------------|--------|
| `apps/central/src/services/jayxpose/` | tous `.rs` | Refonte UI, MSCM | Qualité production |
| `crates/jayxpose` | `data/`, `governance.rs` | Hardening + contrat exposition | Sécurité + Portal |

### Bloc D — COG Web Portal (nouveau)

| Crate | Fichier(s) | Modification | Raison |
|-------|-----------|-------------|--------|
| `apps/cog-web-portal/` | `Cargo.toml`, `src/main.rs`, `src/routes/`, `src/web_surface/`, `static/` | Création from scratch | Nouveau service HTTP |
| `Cargo.toml` (workspace) | `members` | Ajouter `apps/cog-web-portal` | Enregistrement workspace |

## Nouvelles dépendances Cargo

| Crate | Version | Features | Raison |
|-------|---------|----------|--------|
| axum | `0.7` | — | COG Web Portal HTTP (déjà dans miyucloud) |
| tower-http | `0.5` | `cors, fs` | Middleware HTTP Portal |
| tera ou askama | `1` | — | Templating HTML Portal (décider en T6) |

> JayFestival/JayXpose : aucune nouvelle dépendance — refonte UI avec stack existante.

## Tests existants concernés

| Test | Fichier | Impact |
|------|---------|--------|
| Tests DB JayFestival | `crates/jayfestival/src/data/` | Compatible (pas de breaking change data) |
| Tests DB JayXpose | `crates/jayxpose/src/data/` | Compatible |
| Tests auth JayFestival | `crates/jayfestival/src/auth/` | Vérifier si hardening modifie les types |
| Nouveaux tests Portal | `apps/cog-web-portal/tests/` | À créer (TDD P3) |

## Prérequis infrastructure (Hugo)

- COG Web Portal : bin Rust standalone, port configurable, local/VPS — pas de Docker requis
- Variables d'env : `COG_PORTAL_HOST`, `COG_PORTAL_PORT`, `COG_PORTAL_COG_ID`
- CSP/sécurité : Référence `apps/miyucloud/src/security_headers.rs` (nonce CSP, HSTS, rate limiting)
- **À corriger** : `environment.md` indique Dioxus 0.6 — code utilise 0.7 (discordance)

## Blocages potentiels

| Blocage | Probabilité | Action |
|---------|------------|--------|
| Context7 ID Dioxus 0.7 non résolu | Haute | T6 : `resolve-library-id` avant spec |
| MSCM audit scope (8 services) | Moyenne | Prioriser JayFestival+JayXpose P3, autres en V2 |
| Choix moteur templating HTML Portal | Moyenne | T6 François : Tera vs askama vs RSX pur |
| Intégration BorderGuard dans Portal | Faible | Référence miyucloud (pattern établi) |

## Étapes macro P3 (Denis)

### Étape 1 — MSCM Audit & Correction JayFestival + JayXpose
- Objectif : 100% MSCM sur les 2 services prioritaires
- Agents : George (audit) + François + Lise (corrections)
- Prérequis : Checklist MSCM chargée
- Livrables : Rapport audit + codebase corrigée, 0 clippy warning

### Étape 2 — Design System & Refonte JayFestival UI
- Objectif : JayFestival production-ready (Dioxus 0.7, UX org/exp/vis)
- Agents : Lise (UI) + François (back hardening)
- Prérequis : Étape 1, Context7 Dioxus 0.7 résolu
- Livrables : Toutes vues JayFestival refontées + sécurisées

### Étape 3 — Refonte JayXpose + Contrats d'exposition
- Objectif : JayXpose production-ready + contrats Portal définis
- Agents : Lise + François + Denis
- Prérequis : Étape 2 (design system stable)
- Livrables : JayXpose refontée + `portal_contract.rs` (JayFestival + JayXpose)

### Étape 4 — COG Web Portal
- Objectif : Portail HTTP générique multi-services (JayFestival branché)
- Agents : François (back) + Victor (sécu) + Lise (frontend web)
- Prérequis : Étape 3 (contrats disponibles)
- Livrables : `apps/cog-web-portal` compilable, testé, CSP/HSTS/rate-limit

### Étape 5 — Audit final P4
- Objectif : George + Victor — audit intégration, score sécu /100
- Agents : Denis + George + Victor
- Prérequis : Étapes 1-4 terminées

