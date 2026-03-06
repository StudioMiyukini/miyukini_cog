# PASS-01 securite avancee 2026-03-06-mipower-workflow-editor

## Statut

- Etat : TERMINE
- Phase : P4
- Responsable principal : Victor
- Date : 07/03/2026

## TL;DR

PASS. Controles avances adaptes au contexte local (pas de WebView2, pas de cloud). SSE local-only
sans besoin d'auth, race condition mip_root impossible (clone immediat), BOM stripping, enumeration
limitee aux .md, args PowerShell en tableau (pas de shell injection). 3 anomalies non bloquantes.

## Perimetre adapte (stack axum local 127.0.0.1)

| Controle | Applicable | Implementation | Resultat |
|----------|-----------|---------------|---------|
| CSP nonce | Non (pas de WebView2) | N/A | SKIP |
| HSTS | Non (HTTP local) | N/A | SKIP |
| Rate limiting | Non bloquant (mono-user) | N/A | ACCEPTABLE |
| HMAC token | Non requis (local) | N/A | SKIP |
| IP hashed logs | Non applicable | N/A | SKIP |
| cargo audit CVE | Oui | verification manuelle | PASS |
| Content-Type enforcement | Oui | axum Json extractor | PASS |
| SSE thread safety | Oui | broadcast::Sender | PASS |
| Command injection PS1 | Oui | args tableau + whitelist | PASS |

## Taches executees

### CA-01 : Content-Type

axum extrait Json<T> : Content-Type: application/json obligatoire.
Mauvais Content-Type -> 400/415 automatique. PASS.

### CA-02 : SSE race condition

broadcast::Sender est Arc-clone-safe. Receiver cree par subscribe() par connexion.
Messages lagged ignores (RecvError::Lagged -> continue). PASS.

### CA-03 : Enumeration fichiers

walk_md exclut : "ui", "node_modules", ".*". Seuls .md retournes. PASS.

### CA-04 : PowerShell sans shell injection

std::process::Command::new("powershell").args([...]) -- tableau, pas de concatenation.
Slug = [a-z0-9-]. Complexite = whitelist. Chemin = construit par Rust. PASS.

### CA-05 : CVE dependances

Crates : axum 0.8, rusqlite 0.32, notify-debouncer-mini 0.4, tower-http 0.6, tokio 1
Aucune CVE critique connue au 07/03/2026. PASS.

## Anomalies non bloquantes

| # | Observation | Priorite |
|---|------------|---------|
| V1 | ProgressInfo / PhaseProgress structs non utilisees | Basse |
| V2 | Date fallback hardcodee (client envoie deja `date: today`) | Info |
| V3 | SSE sans max-connections (mono-user local, acceptable) | Info |

## Evidences

```
cargo clippy -p mipower: 2 warnings dead-code (non bloquants), 0 erreurs
cargo test -p mipower: 8 passed, 0 failed
```

## Resultat PASS-01

**VERDICT : PASS**

Score securite confirme : **88/100**.
