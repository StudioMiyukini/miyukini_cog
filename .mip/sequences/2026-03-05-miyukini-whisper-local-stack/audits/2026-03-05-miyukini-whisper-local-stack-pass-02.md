# PASS-02 -- Audit validation et robustesse

## Perimetre

- `crates/miyustt/src/lib.rs`
- `crates/miyutts/src/lib.rs`
- `apps/miyukini-whisper/src/lib.rs`
- `crates/miyualicia/src/nlu_bridge.rs`

## Verifications

- Validation payload STT: sample_rate > 0, langue autorisee.
- Validation payload TTS: texte non vide, format gere.
- Timeout strict et fallback local `clean` sur rewrite.
- Contrat inter-services Alicia STT/TTS actif.

## Resultats

- Tests sequence OK:
  - `cargo test -p miyustt -p miyutts -p miyukini-whisper-app -p miyualicia`
- Fallback et gestion d indisponibilite verifies.
- Purge best-effort buffers audio/texte implementee.

## Verdict PASS-02

PASS.
