# Ressources sequence -- miyucloud-oxicloud-refonte

## Competences requises
- Rust avance (async, traits, generics, error handling)
- Architecture hexagonale / Clean Architecture (pattern OxiCloud)
- Protocoles : WebDAV (RFC 4918), CalDAV (RFC 4791), CardDAV (RFC 6352)
- SQL : SQLite dialecte, rusqlite
- Cryptographie : ChaCha20-Poly1305, Argon2id, HKDF, X25519
- Streaming HTTP : chunked uploads/downloads
- axum 0.8+ (framework web)

## Certifications pertinentes
- OWASP Top 10 (2021)
- OWASP ASVS L2
- RGPD (IP hashees, droit effacement)
- ANSSI RGS v2 (crypto)

## Procedures securite
- Validation WebDAV paths : whitelist chars, rejet .., normalisation unicode
- Defenses XML : desactiver DTD, limite taille (1MB), profondeur (20), timeout
- Zeroize secrets en memoire
- cargo audit en CI

## References externes
- OxiCloud : https://github.com/DioCrafts/OxiCloud
- RFC 4918 (WebDAV) : https://tools.ietf.org/html/rfc4918
- RFC 4791 (CalDAV) : https://tools.ietf.org/html/rfc4791
- RFC 6352 (CardDAV) : https://tools.ietf.org/html/rfc6352
