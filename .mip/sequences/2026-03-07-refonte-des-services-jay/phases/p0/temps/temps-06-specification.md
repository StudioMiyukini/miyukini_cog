# P0 Temps 6 - Specification technique

## Statut

- Etat : Terminé
- Phase : P0 Temps 6
- Agent : François
- Date : 2026-03-07

## TL;DR

Spec complète dans `specs/2026-03-07-refonte-des-services-jay-spec.md`. Décisions clés : axum 0.7 pour Portal (pattern miyucloud), Dioxus 0.7 (`ReadSignal<T>`, `use_reactive!`) pour UI, MSCM obligatoire sur tout nouveau code, templating HTML = inline string Rust (pas de Tera/askama — cohérence avec miyucloud).

## Context7 IDs résolus (Dioxus 0.7)

| Lib | Context7 ID | Usage |
|-----|-------------|-------|
| **Dioxus 0.7** | `/dioxuslabs/dioxus/v0.7.2` | Composants, props, RSX, signals |
| **Dioxus 0.7 docs full** | `/llmstxt/dioxuslabs_learn_0_7_llms-full_txt` | Patterns avancés, migration 0.6→0.7 |
| **Dioxus learn** | `/websites/dioxuslabs_learn` | Référence apprentissage (score 90) |
| **Dioxus Components** | `/dioxuslabs/components` | Composants ARIA primitifs |
| **axum** | `/tokio-rs/axum/axum_v0_7_9` | Endpoints, middleware, extractors |

> Corriger `environment.md` : Dioxus 0.6 → **Dioxus 0.7**.

## Décisions architecture

### COG Web Portal
- **Stack** : axum 0.7 + tokio — même pattern que `apps/miyucloud`
- **Templating** : HTML inline Rust (string format, pas de Tera/askama) — cohérence miyucloud
- **Sécurité** : `CspNonce` par requête (copie `security_headers.rs`), HSTS, rate limiting tower-http
- **Routing** : `/` (portal accueil), `/:service` (surface service), `/:service/:slug` (page service)
- **Architecture** : `AppState` avec handles vers crates Jay (JayFestival, JayXpose)
- **MSCM** : `@id @role @layer @human` obligatoire sur tout module

### JayFestival + JayXpose UI (Dioxus 0.7)
- **Composants** : via `miyuki-ui-dioxus` (atoms/molecules/organisms) — plus d'inline style orphelins
- **Props réactives** : `ReadSignal<T>` pour props dans hooks réactifs, `Signal<T>` pour état mutable
- **Pattern** : `#[component]` shorthand, `use_reactive!(|prop| ...)` pour deps explicites
- **Design system** : enrichir `miyuki-ui-dioxus` si composant manquant (StatCard, PageHeader, EmptyState, SidebarNav)

### MSCM
- **Format** : `//! @id: snake_case_unique @do: action @role: role @layer: layer @human: description`
- **Obligatoire** : toute `fn pub`, tout `struct/enum pub`, tout `mod pub`, tout `impl` significatif
- **Audit** : George lit la MSCM Checklist avant P3

## API publiques — Contrats d'exposition Portal

### Trait `PortalContract` (à créer dans chaque crate Jay)
```rust
// @id: portal_contract_trait @role: api @layer: service
pub trait PortalContract: Send + Sync {
    fn service_slug(&self) -> &str;
    fn public_pages(&self) -> Vec<PublicPage>;
    fn page_by_slug(&self, slug: &str) -> Option<PublicPage>;
}

pub struct PublicPage {
    pub slug: String,
    pub title: String,
    pub html_content: String, // rendu par le service
}
```

### Routes Portal (axum)
```
GET /                    -> portal home (liste services actifs)
GET /:service            -> surface service (délègue à PortalContract)
GET /:service/:slug      -> page service spécifique
POST /:service/contact   -> formulaire contact (CSRF protégé)
```

## Schéma de données

### JayFestival — Données exposées via Portal
```
Edition { id, slug, nom, date_debut, date_fin, lieu, description_publique }
Exposant { id, entreprise, description_publique, vitrine_status }
Stand { id, numero, exposant_id, position }
```

### JayXpose — Données exposées via Portal
```
ExposantPublic { id, slug, company_name, description, vitrine_status }
Produit { id, nom, description, prix_affiche, categorie, image_url }
VitrinePage { id, slug, titre, contenu_html, publiee }
```

## Conformité architecturale

- Lois d'Autonomie : pas de `unwrap()`, erreurs propagées avec `?`, types `Result<T, E>`
- Couche architecture : Portal = strate 7 (Service Fondamental), Jay = strate 7 (services)
- Versions fixées dans workspace `Cargo.toml` — pas de `*`
- Sécurité : constant-time compare pour tokens, pas de secrets en clair, MSCM sur tout

## Anti-patterns chargés (depuis memory/)

- `lock_conn()` deadlock : `drop(conn)` avant appel récursif (pattern kindmother_db)
- `Extension<T>` axum : syntaxe `Extension(CspNonce(nonce)): Extension<CspNonce>` dans handler
- Pas de `unwrap()` sur `db.*` — utiliser `?` ou `unwrap_or_default()` documenté
- Pas de duplication composants UI entre services — centraliser dans miyuki-ui-dioxus

