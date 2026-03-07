# Specification 2026-03-07-refonte-des-services-jay

## Statut

- Etat : Terminé
- Phase : P0 Temps 6
- Agent : François
- Date : 2026-03-07

## TL;DR

Refonte T5/C5 — 4 blocs : MSCM audit complet famille Jay, JayFestival prod-ready (Dioxus 0.7), JayXpose prod-ready, COG Web Portal (axum 0.7, portail HTTP générique multi-services). Stack : Rust + Dioxus 0.7 + axum 0.7 + KindMother. Sécurité niveau DURCI (pattern miyucloud).

## Architecture globale

```
Miyukini COG — Refonte Services Jay
├── apps/cog-web-portal/           [NOUVEAU] Portail HTTP externe
│   ├── src/main.rs                 axum router + AppState
│   ├── src/routes/                 handlers par service
│   ├── src/web_surface/            rendu HTML + sécurité
│   └── src/security_headers.rs    CSP nonce, HSTS, rate limit
│
├── apps/central/src/services/
│   ├── jayfestival/               [REFONTE UI] Dioxus 0.7 design system
│   └── jayxpose/                  [REFONTE UI] Dioxus 0.7 design system
│
├── crates/miyuki-ui-dioxus/       [ENRICHIR] Design system partagé
│   ├── src/atoms/                  Boutons, Badge, Icon, Input...
│   ├── src/molecules/              Card, FormField, StatRow...
│   └── src/organisms/             [NOUVEAU] SidebarNav, PageHeader, EmptyState
│
├── crates/jayfestival/            [HARDENING + MSCM + CONTRAT]
│   └── src/portal_contract.rs     [NOUVEAU] impl PortalContract
│
└── crates/jayxpose/               [HARDENING + MSCM + CONTRAT]
    └── src/portal_contract.rs     [NOUVEAU] impl PortalContract
```

## Spécification COG Web Portal

### AppState
```rust
// @id: cog_portal_app_state @role: data @layer: service
pub struct AppState {
    pub cog_id: String,
    pub services: Vec<Arc<dyn PortalContract>>,
    pub config: PortalConfig,
}
```

### PortalContract trait
```rust
// @id: portal_contract_trait @role: api @layer: service
pub trait PortalContract: Send + Sync {
    fn service_slug(&self) -> &str;
    fn service_name(&self) -> &str;
    fn public_pages(&self) -> Result<Vec<PublicPage>, PortalError>;
    fn page_by_slug(&self, slug: &str) -> Result<Option<PublicPage>, PortalError>;
}
```

### Routing axum
```
GET  /                        -> portail accueil (liste services)
GET  /:service                -> surface service (délègue PortalContract)
GET  /:service/:slug          -> page spécifique
POST /:service/contact        -> formulaire (CSRF + rate limit)
GET  /health                  -> health check
```

### Sécurité Portal (pattern miyucloud)
- `CspNonce` per-request via tower middleware
- `Content-Security-Policy: default-src 'self'; script-src 'nonce-{nonce}'; style-src 'nonce-{nonce}'`
- `Strict-Transport-Security: max-age=31536000; includeSubDomains`
- Rate limiting : tower-http `RateLimitLayer` — 60 req/min par IP (hachée RGPD)
- CSRF tokens HMAC sur formulaires POST
- Validation entrées : longueur max, pas de null bytes, format strict

## Spécification JayFestival UI (Dioxus 0.7)

### Patterns Dioxus 0.7 validés
```rust
// Props réactives
#[component]
pub fn StatCard(label: String, value: String, color: ReadSignal<String>) -> Element { ... }

// Signal mutable
let state: Signal<JayFestivalState> = use_signal(JayFestivalState::default);

// Dépendance explicite
let doubled = use_memo(use_reactive!(|count| count * 2));
```

### Composants à créer/migrer vers miyuki-ui-dioxus
| Composant | Niveau | Action |
|-----------|--------|--------|
| `StatCard` | molecule | Migrer depuis services → miyuki-ui-dioxus/molecules |
| `ActionButton` | atom | Migrer + variantes (accent/ghost/danger) |
| `SidebarNav` | organism | Créer (pattern commun JayFestival + JayXpose) |
| `PageHeader` | organism | Créer (h1 + sous-titre + slot actions) |
| `EmptyState` | molecule | Créer |
| `StatusBadge` | atom | Créer (publiée/brouillon/suspendue) |

### MSCM sur composants Dioxus
```rust
//! @id: jayfestival_org_dashboard @do: render_org_dashboard
//! @role: ui @layer: service @human: Tableau de bord organisateur JayFestival
```

## Spécification MSCM Audit

### Format canonique obligatoire
```rust
// Sur fn/struct/enum pub — en-tête de fichier ou au-dessus du bloc
//! @id: unique_snake_case_id
//! @do: action_verb_object
//! @role: security|data|logic|api|infra|domain|service|ui
//! @layer: domain|infra|api|service|core|kernel
//! @human: Description claire du bloc
```

### Critères d'audit
- [ ] Tous fichiers `.rs` publics ont un en-tête `@id @do @role @layer @human`
- [ ] Pas de doublon `@id` dans tout le codebase
- [ ] `@role` cohérent avec la fonction du bloc
- [ ] Dépendances inter-blocs déclarées si applicable

## Conformité architecturale

- Pas de `unwrap()` — `?` ou `unwrap_or_default()` documenté
- Pas de `any` TypeScript (N/A — Rust uniquement)
- Erreurs : `thiserror` pour types d'erreur, pas de `Box<dyn Error>` en API publique
- Tests : TDD obligatoire P3 — smoke test RED avant GREEN
- Commits : feat/fix/chore + scope entre parenthèses (ex. `feat(cog-web-portal): ...`)

## Schema de donnees

[A completer]

## API / Interfaces

[A completer]

## Securite

[A completer]

## Dependances

[A completer]

## Criteres d'acceptance

[A completer]

