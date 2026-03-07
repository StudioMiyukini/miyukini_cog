# PASS-0 securite 2026-03-07-refonte-des-services-jay

## Statut

- Etat : TERMINE
- Phase : P4
- Responsable principal : Victor

## TL;DR

PASS. Controles fondamentaux OK. Path traversal bloque (canonicalize). Auth hardened (0 unwrap). SQL injection impossible (prepared statements). XXE N/A (pas de XML).

## Perimetre

| Controle | Fichier | Resultat |
|----------|---------|---------|
| Path traversal | `apps/cog-web-portal/src/routes/service.rs` — slug sanitise, pas de FS direct | PASS |
| XXE injection | Non applicable — aucun parsing XML dans le perimetre | N/A |
| Auth bypass | `crates/jayfestival/src/auth/mod.rs`, `crates/jayxpose/src/auth/mod.rs` — 0 unwrap(), HMAC tokens, constant-time compare | PASS |
| SQL injection | Tous les appels DB via rusqlite `params![]` prepares — aucune concatenation SQL | PASS |
| Upload path traversal | `crates/jayxpose/src/data/upload_validation.rs` — `contains("..") \|\| contains('/')` | PASS |
| MIME spoofing | `upload_validation.rs` — allowlist stricte (PDF, JPEG, PNG, WEBP, DOCX) | PASS |
| Taille fichier | `upload_validation.rs` — 20 MB max, rejet explicite | PASS |

## Taches executees

- E02-10 : Auth hardening JayFestival — 0 unwrap() en prod, HMAC constant-time
- E03-07 : Upload validation JayXpose — MIME allowlist, path traversal, 20 MB max — 5 tests
- E04 : Routes portal — slugs valides seulement, pas d'acces FS direct
- BUF : dedup @id MSCM (non securite)

## Evidences

```
test data::upload_validation::tests::valid_document_passes ... ok
test data::upload_validation::tests::invalid_mime_rejected ... ok
test data::upload_validation::tests::path_traversal_rejected ... ok
test data::upload_validation::tests::file_too_large_rejected ... ok
test data::upload_validation::tests::missing_exposant_id_rejected ... ok

test result: ok. 40 passed; 0 failed — jayfestival + jayxpose + cog-web-portal
cargo clippy -p jayfestival -p jayxpose -p cog-web-portal -- -D warnings : 0 violations
```

## Resultat PASS-0

**VERDICT : PASS**

Tous les controles fondamentaux valides. Aucun vecteur d'injection identifie dans le perimetre sequence.
