# Plan P3 -- miyukini-whisper-local-stack

## TL;DR

Plan en 8 vagues:
1) socle `miyustt`
2) socle `miyutts`
3) service `miyukini-whisper` + presets hardware
4) insertion texte systeme + UI Central
5) integration Alicia
6) hardening + benchmarks
7) docs + packaging
8) gate final P4/P5

---

## V0 -- Preparation (A faire)

- [x] Creer branche `feat/miyukini-whisper-local-stack`
- [x] Ajouter crates `miyustt` et `miyutts` au workspace
- [x] Ajouter app `miyukini-whisper`
- [x] Ajouter squelette OpenAPI STT/TTS
- [x] Valider analyse PR + comparative concurrence (brief PR dedie)
- [x] Smoke build cible (`miyustt`, `miyutts`, `miyukini-whisper-app`)

Commandes:

```powershell
cargo check --workspace
```

---

## V1 -- Toolkit MiyuSTT (A faire)

- [x] Crate `crates/miyustt` (API publique + erreurs + config)
- [x] Endpoint `GET /api/health`
- [x] Endpoint `POST /api/stt`
- [x] WS `GET /api/stt/stream`
- [x] Adaptateur moteur primaire (placeholder V1)
- [x] Adaptateur fallback chain complet
- [x] Presets hardware STT (`compact`, `balanced`, `precision`)
- [x] Support FR/EN par defaut (payload API V1)
- [x] Tests unitaires STT
- [x] Tests integration API STT

Gate V1:
- [x] `cargo test -p miyustt` passe
- [x] Contrat JSON stable (`transcript/confidence/duration_ms`)
- [x] auto-selection preset validee localement (tests)

---

## V2 -- Toolkit MiyuTTS (A faire)

- [x] Crate `crates/miyutts` (engine abstraction, voices, cache)
- [x] Endpoint `GET /api/health`
- [x] Endpoint `GET /api/tts/voices`
- [x] Endpoint `POST /api/tts`
- [x] Streaming/restitution audio WAV
- [x] Presets hardware TTS (`compact`, `balanced`, `precision`)
- [x] Voix FR/EN par defaut
- [x] Tests unitaires TTS
- [x] Tests integration API TTS

Gate V2:
- [x] `cargo test -p miyutts` passe
- [x] latence synthese initiale mesuree

---

## V3 -- Service Miyukini Whisper (A faire)

- [x] App `apps/miyukini-whisper` (runtime, UI minimale)
- [ ] Hotkey global push-to-talk
- [ ] Capture micro + pipeline vers MiyuSTT
- [x] Affichage transcription partielle/finale (simulation V3)
- [ ] Injection texte dans champ actif
- [x] Profils `verbatim`, `clean`, `rewrite` (backend complet)
- [x] Routage fallback opt-in (config UI V3)
- [ ] Telemetrie locale de latence (sans envoi externe)

Gate V3:
- [ ] demo locale: hotkey -> texte insere dans champ

---

## V4 -- Post-processing LLM local (A faire)

- [x] Mode `clean` sans LLM (regles locales)
- [ ] Mode `rewrite` via `miou-llm-bridge`
- [x] Timeout strict + fallback mode `clean`
- [ ] Parametrage modele local utilisateur
- [x] Garantir independance STT/TTS si LLM indisponible

Gate V4:
- [x] pas de dependance cloud obligatoire
- [x] fallback local valide

---

## V5 -- Integration Alicia et autres services (A faire)

- [x] Aligner Alicia sur endpoint STT final (`/api/stt`)
- [x] Ajouter client TTS Alicia (feature flag)
- [x] Ajouter tests contrat inter-services
- [x] Ajouter exemples d usage pour services COG
- [x] Ajouter contrat fallback host bridge pour COG utilisateur

Gate V5:
- [x] `miyualicia` compile avec integration STT/TTS

---

## V6 -- Hardening, Bench, Security (A faire)

- [x] Auth bearer optionnelle sur endpoints
- [x] Origin policy localhost par defaut
- [x] purge buffers audio temporaires
- [x] Bench latence STT/TTS sur 3 profils machine
- [x] Rapport securite + licence

Gate V6:
- [x] aucun defaut critique securite
- [x] objectifs latence V1 tenables

---

## V7 -- Packaging, docs, UX (A faire)

- [ ] docs utilisateur (hotkeys, profils, depannage)
- [ ] docs API STT/TTS (OpenAPI)
- [ ] scripts installation locale
- [x] support Central initial (catalogue + vue dédiée)
- [ ] interface Central `Miyukini Whisper`:
  - [x] onglet STT test (simulation)
  - [x] onglet TTS test (simulation)
  - [x] onglet presets hardware (UI)
  - [x] onglet fallback local/host/cloud (UI)
  - [x] onglet diagnostics (health live + erreurs)
- [ ] assets PR:
  - [ ] message court produit
  - [ ] comparatif concurrence public
  - [ ] script demo 60 secondes

Gate V7:
- [ ] onboarding local en moins de 5 minutes

---

## V8 -- Validation finale P4/P5 (A faire)

- [x] audit conformite George
- [x] audit securite Victor
- [x] verification tests globaux
- [x] test humain dictee sur cas reel (dev + auteur + etudiant)
- [x] rapport final P6

---

## Strategie de livraison

1. Livrer `MiyuSTT` utilisable meme sans UI.
2. Livrer `MiyuTTS` utilisable meme sans service de dictee.
3. Livrer `Miyukini Whisper` comme assembleur UX des deux toolkits.
4. Stabiliser contrat API avant optimisation avancee.
5. Valider l UI Central avant gate P5.
