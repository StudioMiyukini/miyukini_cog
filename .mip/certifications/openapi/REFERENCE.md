<!-- @id cert.francois.openapi -->
<!-- @do provide_openapi_reference_knowledge -->
<!-- @role api_standards -->
<!-- @layer reference -->
<!-- @human Referentiel OpenAPI 3.1 pour Francois -->

# OpenAPI 3.1 â€” Francois

> **TL;DR** Spec API REST. Paths, schemas, security. Routes axum = paths. serde = components/schemas.

**IdentitÃ©** : OpenAPI Initiative | Volontaire | 3.1.0

## Structure | MÃ©thodes | Codes

| Section | Requis | Exemple |
|---------|--------|---------|
| paths | Oui | GET/POST/PUT/DELETE |
| components/schemas | Non | $ref rÃ©utilisables |
| security | Non | apiKey, bearer |

| MÃ©thode | Usage | Idempotent |
|---------|-------|------------|
| GET | Lire | Oui |
| POST | CrÃ©er | Non |
| PUT | Remplacer | Oui |
| DELETE | Supprimer | Oui |

| Plage | Codes |
|-------|-------|
| 2xx | 200, 201, 204 |
| 4xx | 400, 401, 403, 404, 422 |
| 5xx | 500, 503 |

## Bonnes pratiques

Ressources pluriel, kebab-case. Versioning `/api/v1/`. Erreur: `{error:{code,message}}`. Pagination: `?page=1&per_page=20`.

## Checklist

- [ ] Spec par service axum
- [ ] Schemas dans components (pas inline)
- [ ] Codes HTTP corrects (201 Create, 404 Not Found)
- [ ] Format erreur standardisÃ©
- [ ] Security scheme dÃ©clarÃ©

## Anti-patterns

| Erreur | Correction |
|--------|------------|
| Verbes dans URL | Ressources + mÃ©thode HTTP |
| Tout 200/500 | 201, 404, 422 |
| Schemas dupliquÃ©s | $ref components |

## Miyukini

Structs serde = schemas. utoipa ou aide pour gÃ©nÃ©ration. Erreurs explicites par module.
## Parcours obtention
Voir KNOWLEDGE.md pour les connaissances requises et les preuves de maitrise.

