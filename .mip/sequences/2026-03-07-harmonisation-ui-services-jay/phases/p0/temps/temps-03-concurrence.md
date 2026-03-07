# P0 Temps 3 - Analyse concurrentielle

## Statut

- Etat : TERMINE
- Phase : P0
- Responsable principal : Fabrice

## Analyse

Perimetre interne — pas de concurrents externes pertinents pour une harmonisation de design system propriétaire.

## References internes

| Pattern | Source | Applicable |
|---------|--------|-----------|
| miyuki-ui-dioxus | `crates/miyuki-ui-dioxus/` | OUI — design system officiel |
| miyuki-ui-tokens | `crates/miyuki-ui-tokens/` | OUI — Palette, spacing, colors |
| miyucloud UI | `apps/miyucloud/static/` | NON — axum HTML, pas Dioxus |
| mipower UI | `apps/mipower/static/` | NON — HTML/JS vanilla |
| miyukini-service-ui | `crates/miyukini-service-ui/` | MIGRATION CIBLE — legacy a eliminer |

## Conclusion

Design system cible : `miyuki-ui-dioxus` + `miyuki-ui-tokens`. Palette::default() pour commencer. Pas de reference externe necessaire — perimetre 100% interne.
