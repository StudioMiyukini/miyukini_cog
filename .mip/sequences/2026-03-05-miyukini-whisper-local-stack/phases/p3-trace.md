# P3 -- Trace d execution

| Vague | Agent(s) | Statut | Artefacts |
|---|---|---|---|
| V0 Preparation | Denis | Termine | branch + workspace + openapi skeleton + smoke build cible |
| V1 MiyuSTT | Francois | Partiel avance | `crates/miyustt/` (health + stt + ws stream + fallback chain + tests) |
| V2 MiyuTTS | Francois | Partiel avance | `crates/miyutts/` (health + voices + tts + wav + tests) |
| V3 Miyukini Whisper | Lise + Denis | Partiel avance | `apps/miyukini-whisper/` (profils texte + timeout fallback) + vue Central diagnostics live |
| V4 LLM post-process | Denis | Partiel | mode `clean` local + timeout strict; bridge `miou-llm-bridge` restant |
| V5 Integration Alicia | Francois | Termine (V1 contrat) | `miyualicia` aligne STT `/api/stt` + client TTS feature-flag + tests contrat STT/TTS + exemples usages |
| V6 Hardening + bench | Victor + George | Termine (scope sequence) | auth bearer optionnelle + policy origin localhost + purge buffers STT/TTS + bench latence preset |
| V7 Packaging + docs | Hugo + Maria | En attente | docs + scripts |

**p3_start**: 2026-03-05T10:40:00Z  
**p3_end**: 2026-03-05T13:51:22Z

## Verification executee

- `cargo check -p miyustt -p miyutts -p miyukini-whisper-app` : OK
- `cargo test -p miyustt -p miyutts` : OK
- `cargo test -p miyustt -p miyutts` (apres ajout stream/wav/fallback) : OK
- `cargo check -p miyukini-whisper-app` (apres ajout post-process V4) : OK
- `cargo test -p miyukini-whisper-app` : OK
- `cargo test -p miyualicia` : OK (incluant tests contrat inter-services avec `miyustt`/`miyutts`)
- `cargo check -p miyualicia-api` : OK
- `cargo metadata --format-version 1 --no-deps` : OK (workspace reparee via stubs manquants)
- `cargo test -p miyustt` : OK (7 tests)
- `cargo test -p miyutts` : OK (6 tests)
- `cargo test -p miyukini-whisper-app -p miyualicia` : OK (58 tests)
- `cargo clippy -p miyustt -p miyutts -p miyukini-whisper-app -p miyualicia -p miyualicia-api -- -D warnings` : OK
- `cargo check --workspace` : ECHEC hors scope sequence (erreurs preexistantes dans `apps/central/src/services/miyucloud/auth_security.rs`)
- `cargo check -p miyukini-central-native` : ECHEC hors scope sequence (meme erreur `miyucloud/auth_security.rs`)
