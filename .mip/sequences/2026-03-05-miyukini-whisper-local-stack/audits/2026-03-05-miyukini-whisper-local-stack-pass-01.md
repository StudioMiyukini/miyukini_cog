# PASS-01 -- Audit endpoints et auth

## Perimetre

- `crates/miyustt/src/lib.rs`
- `crates/miyutts/src/lib.rs`

## Verifications

- Auth bearer optionnelle implementee et testee.
- Policy Origin localhost active par defaut et testee.
- Endpoints sensibles verifies:
  - STT: `/api/stt`, `/api/stt/stream`
  - TTS: `/api/tts`, `/api/tts/wav`, `/api/tts/voices`

## Resultats

- `miyustt`: tests auth/origin OK (3 tests securite)
- `miyutts`: tests auth/origin OK (3 tests securite)
- Aucun bypass detecte dans le scope sequence.

## Verdict PASS-01

PASS.
