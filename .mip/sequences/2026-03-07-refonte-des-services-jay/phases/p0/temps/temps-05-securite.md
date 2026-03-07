# P0 Temps 5 - Analyse securite

## Statut

- Etat : Terminé — RPS produit
- Phase : P0 Temps 5
- Agent : Victor
- Date : 2026-03-07

## TL;DR

3 zones de risque : COG Web Portal (CRIT — surface HTTP externe), JayFestival/JayXpose backend (HIGH — données PII, docs), UI Dioxus (LOW). Référence sécurité : apps/miyucloud (CSP nonce, HSTS, score 97/100). Score cible P4 : ≥ 90/100. RPS produit ci-dessous.

## Classification des données

- Niveau : MIXTE — PUBLIC (pages portal, catalogues vitrine) + PRIVÉ (profils exposants, documents, données financières JayKonta)
- Surfaces exposées : HTTP web (COG Web Portal), IPC KindMother (services Jay), SQLite local (données)
- Conformité requise : RGPD (données exposants PII, IP logs hachés), pas de certification sectorielle requise

## Surfaces d'attaque

| Surface | Risque | Niveau | Mitigation ciblée |
|---------|--------|--------|------------------|
| **COG Web Portal — HTTP externe** | Accès non autorisé, injection, path traversal, DoS | CRIT | CSP nonce per-request, HSTS, rate limiting, BorderGuard, validation stricte routes |
| **Formulaires Portal (contact, réservation)** | XSS, CSRF, spam, abus | HIGH | CSRF tokens (HMAC), Content-Type strict, antispam, validation entrées |
| **Auth JayFestival/JayXpose** | Session hijacking, brute force, auth bypass | HIGH | HMAC tokens, constant-time compare, rate limiting login, sessions bornées |
| **Upload documents JayXpose (coffre-fort)** | Malware upload, path traversal, taille excessive | HIGH | Validation MIME, taille max, chemin canonique, stockage hors webroot |
| **SQL/KindMother** | SQL injection (rusqlite paramétré) | MED | rusqlite paramétré déjà en place, `INSERT OR IGNORE`, `PRAGMA foreign_keys ON` |
| **Données PII exposants** | RGPD — fuite email/phone/SIRET | MED | IP logs hachés (SHA-256), pas de log PII brut, accès par rôle |
| **UI Dioxus (desktop, local)** | Surface d'attaque faible (pas HTTP) | LOW | Aucune surface réseau directe en mode desktop |

## CVE / dépendances à surveiller

| Crate | Risque | Action |
|-------|--------|--------|
| `axum 0.7` | RAS (actif, maintenus) | Monitorer |
| `rusqlite 0.32` | RAS | Cargo audit P4 |
| `serde / serde_json` | RAS | Cargo audit P4 |
| `sha2 0.10` | RAS | Cargo audit P4 |
| `dioxus 0.7` | Nouveau — peu d'historique CVE | `cargo audit` obligatoire P4 |
| `tower-http` | RAS | Cargo audit P4 |
| Moteur templating (Tera/askama) | À évaluer selon choix T6 | Choisir lib active + auditée |

## Contrôles P4 applicables

| Contrôle | Applicable | Priorité |
|----------|-----------|---------|
| PASS-0 : path traversal | Oui (Portal routes, uploads JayXpose) | CRIT |
| PASS-0 : XXE injection | Non (pas de XML) | — |
| PASS-0 : auth bypass | Oui (Jay auth, Portal BorderGuard) | CRIT |
| PASS-0 : SQL injection | Oui (rusqlite) | CRIT |
| PASS-01 : CSP nonce per-request | Oui (COG Web Portal) | CRIT |
| PASS-01 : HSTS + Secure headers | Oui (Portal) | CRIT |
| PASS-01 : Rate limiting | Oui (Portal + login Jay) | HIGH |
| PASS-01 : HMAC + constant-time compare | Oui (tokens auth, CSRF) | HIGH |
| PASS-01 : IP hashed logs (RGPD) | Oui (Portal) | HIGH |
| PASS-01 : cargo audit CVE | Oui | CRIT |
| PASS-01 : CSRF / replay tokens | Oui (formulaires Portal) | HIGH |
| PASS-01 : Content-Type enforcement | Oui (Portal API + uploads) | HIGH |

## Checklist sécurité pour spec (François — T6)

- [x] Authentification : HMAC tokens (pattern miyucloud) — Oui, requis Portal + Jay
- [x] Autorisation : Rôle-based (org/exp/vis pour Jay, MPA pour Portal)
- [x] Validation entrées : Toutes entrées Portal validées (longueur, format, pas de null bytes)
- [x] Chiffrement : SQLCipher optionnel (déjà disponible via `kindmother-db-key`), TLS en prod
- [x] Gestion secrets : Pas de secrets hardcodés, env vars uniquement
- [x] Logging sécurité : Événements auth (succès/échec), accès routes Portal, uploads
- [x] Rate limiting : Login Jay + toutes routes Portal publiques
- [x] CORS : Portal = politique restrictive (origines explicites), pas de wildcard

## Score cible P4

- RAS sécurité cible : **≥ 90/100**
- Référence : `apps/miyucloud` = 97/100 (pattern à reproduire pour COG Web Portal)
- Brèche critique ou score < 60 → rebouclage MIP (P0 Temps 1)

## RPS — Rapport Préliminaire de Sécurité

### 1. Surfaces d'attaque et risques majeurs
Zone CRIT : COG Web Portal (HTTP externe, formulaires publics, uploads). Zone HIGH : Auth Jay (brute force, session), Documents JayXpose (uploads malveillants). Zone MED : SQL injection (rusqlite paramétré, bien contrôlé). Zone LOW : UI Dioxus desktop (pas de surface réseau).

### 2. Ressources sécurité nécessaires
- Victor dédié audit P4 (PASS-0 : traversal + auth bypass + SQL, PASS-01 : CSP + headers + rate limit)
- `apps/miyucloud/src/security_headers.rs` — module de référence à copier/adapter pour Portal
- `cargo audit` obligatoire avant chaque livraison (CI/CD P3 checkpoint)

### 3. Normes applicables
- OWASP Top 10 (XSS, SQLi, CSRF, path traversal, broken auth)
- RGPD minimal : IP logs hachés, pas de PII brut en log, droit accès/suppression

### 4. Niveau de sécurité requis par zone
- COG Web Portal : Niveau DURCI (CSP, HSTS, rate limit, CSRF, audit formel)
- JayFestival/JayXpose backend : Niveau DURCI (auth HMAC, validation, logs)
- UI Dioxus desktop : Niveau STANDARD (Lois d'Autonomie, no unwrap)

### 5. Conclusion
Niveau maximal requis : **DURCI** (pas critique — pas de données de santé, pas de paiement en ligne). Pattern miyucloud (97/100) est la référence directe pour le COG Web Portal. Les patterns sont connus et maîtrisés dans ce projet.

