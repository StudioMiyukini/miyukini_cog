# PASS-0 securite 2026-03-06-mipower-workflow-editor

## Statut

- Etat : TERMINE
- Phase : P4
- Responsable principal : Victor
- Date : 07/03/2026

## TL;DR

PASS. 4 controles fondamentaux verifies : path traversal bloque par canonicalize + starts_with, SQL
parameterized sur toutes les requetes, pas d'interpolation directe, XSS bloque par DOMPurify dans
le frontend, slug injection impossible (validation [a-z0-9-] + liste blanche complexite).

## Perimetre

| Controle | Fichier | Resultat |
|----------|---------|---------|
| Path traversal | apps/mipower/src/api.rs — artefact_handler | PASS |
| SQL injection | apps/mipower/src/api.rs — sequences_handler | PASS |
| XSS / HTML injection | apps/mipower/static/app.js — loadArtefact() | PASS |
| Slug injection (init-sequence) | apps/mipower/src/api.rs — init_sequence_handler | PASS |

## Taches executees

### CT-01 : Path traversal (artefact_handler)

Code analyse :
```rust
let canonical_root = PathBuf::from(&mip_root).canonicalize()...;
let canonical_file = resolved.canonicalize()...;
if !canonical_file.starts_with(&canonical_root) {
    return Err(ApiError::forbidden("Chemin hors du workspace MIP"));
}
if canonical_file.extension().is_none_or(|e| e != "md") {
    return Err(ApiError::bad_request("Seuls les fichiers .md sont accessibles"));
}
```
Verdict : Double protection (canonicalize + starts_with + extension filter). Test unitaire confirme
dans test_path_traversal_rejected. PASS.

### CT-02 : SQL injection (sequences_handler)

Code analyse :
```rust
conn.execute(
    "INSERT INTO sequences ... VALUES (?1,?2,?3,?4,?5,?6,'[]',?7)
     ON CONFLICT(slug) DO UPDATE SET ...",
    rusqlite::params![s.slug, s.date, s.status, ...],
);
```
Toutes les requetes SQLite utilisent `rusqlite::params![]` (parameterized). Aucune interpolation
directe de string detectee. PASS.

### CT-03 : XSS (loadArtefact frontend)

Code analyse :
```javascript
const rawHtml = marked.parse(data.content || '');
const clean   = DOMPurify.sanitize(rawHtml, { ADD_ATTR: ['class'] });
body.innerHTML = clean;
```
Tout HTML issu du Markdown passe par DOMPurify.sanitize() avant assignation a innerHTML. PASS.

### CT-04 : Slug / command injection (init_sequence_handler)

Code analyse :
```rust
if input.slug.is_empty()
    || !input.slug.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
    || input.slug.contains("..") {
    return Err(ApiError::bad_request("Slug invalide (a-z0-9- uniquement)"));
}
const VALID_COMPLEXITIES: &[&str] = &["C1", "C2", "C3", "C4", "C5"];
if !VALID_COMPLEXITIES.contains(&input.complexity.as_str()) { ... }
```
Slug whitelist [a-z0-9-] uniquement, pas de separateurs de chemin possibles. Complexite via liste
blanche exacte. Arguments passes sans interpolation shell (tableau args). PASS.

## Evidences

```
running 8 tests
test api::tests::test_generate_prompt_non_empty ... ok
test api::tests::test_init_sequence_slug_validation ... ok
test api::tests::test_path_traversal_rejected ... ok
test api::tests::test_sequences_index_parse ... ok
test db::tests::test_open_creates_schema ... ok
test db::tests::test_open_idempotent ... ok
test watcher::tests::test_extract_slug ... ok
test watcher::tests::test_extract_slug_no_match ... ok
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Resultat PASS-0

**VERDICT : PASS**

Tous les controles fondamentaux sont satisfaits. Aucune vulnerabilite bloquante detectee.
