# RAS securite miyucloud-oxicloud-refonte

## Statut

- Etat : RAS
- Phase : P4
- Responsable principal : Victor
- Date : 2026-03-06

## TL;DR

Rien A Signaler. Aucun probleme de securite ouvert. Score 97/100. Les 2 points manquants sont des ameliorations futures non bloquantes (TLS client cert, OCSP stapling).

## Sources auditees

| Source | Perimetre |
|--------|-----------|
| PASS-0 | Path traversal, XXE, auth bypass, SQL injection |
| PASS-01 | CSP nonce, HSTS, rate limiting, HMAC, IP hashed, dedup SHA-256 |
| E10-01 a E10-07 | Implementation directe des controles |
| `cargo audit` | Dependances CVE (E0-06 rusqlite CVE-2025-6965 corrige) |

## Conclusion securite

Toutes les surfaces d'attaque critiques identifiees lors du P2 (design) sont couvertes par implementation et tests.

La crate `miyucloud-dav` (WebDAV/CalDAV/CardDAV) utilise `quick-xml` en mode securise (pas d'expansion d'entites). Les uploads passent par `validate_path` avant tout acces disque. Les tokens sont verifies en temps constant via `subtle`. Les logs ne persistent pas d'IP en clair.

## Recommandations futures (non bloquantes)

| Priorite | Recommandation | Effort |
|----------|---------------|--------|
| P2 | TLS client certificate optionnel pour admin endpoints | M |
| P3 | OCSP stapling sur le serveur TLS | S |
| P3 | Rotation automatique des HMAC keys (24h) | M |

## Score securite

| Critere | Score | /20 |
|---------|-------|-----|
| Authentification & autorisation | 20 | /20 |
| Validation des entrees | 19 | /20 |
| Cryptographie | 19 | /20 |
| Logging & monitoring | 20 | /20 |
| Configuration & hardening | 19 | /20 |
| **TOTAL** | **97** | **/100** |

## Verdict

**RAS -- Score 97/100 -- Aucune vulnerabilite ouverte**
