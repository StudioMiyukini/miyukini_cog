<!-- @id mem.security.patterns
     @do document_security_patterns
     @role patterns
     @layer memory
     @human Patterns securite valides et interdits -->

# Patterns securite

> Complete depuis P0 T5 -- sequence miyucloud-oxicloud-refonte

## Patterns approuves

- SEC-01: `unsafe_code = "forbid"` dans tous les Cargo.toml crates
- SEC-02: Comparaisons constant-time via `subtle` pour tokens/secrets
- SEC-03: `SandboxedStore` pour isolation fichiers utilisateur
- SEC-04: Nonces `OsRng` pour operations crypto
- SEC-05: `Debug` masque les secrets (impl custom)
- SEC-06: Rate limiting par IP sur endpoints auth
- SEC-07: Journal RGPD pour operations sensibles
- SEC-08: ChaCha20-Poly1305 pour chiffrement fichiers
- SEC-09: Argon2id + HKDF pour derivation cles
- SEC-10: X25519 pour echange cles E2E
- SEC-11: WAL mode SQLite avec `busy_timeout=5000`
- SEC-12: `PRAGMA foreign_keys=ON` systematique

## Interdits

- AP-SEC-01: Pas de `unsafe` dans le code applicatif
- AP-SEC-02: Pas de `RETURNING` en SQL (incompatible SQLite < 3.35)
- AP-SEC-03: Pas de DTD dans le parsing XML (CalDAV/CardDAV) -- risque XXE
- AP-SEC-04: Pas de `unsafe-inline` dans le CSP
- AP-SEC-05: Pas de chemins relatifs avec `..` dans WebDAV (path traversal)
- AP-SEC-06: Pas de secrets en clair dans les logs
- AP-SEC-07: Pas de `sqlx` -- rester sur `rusqlite` (KindMother)

## CVEs a surveiller

- CVE-2025-6965: SQLite < 3.50.2 (bumper rusqlite en E0)
