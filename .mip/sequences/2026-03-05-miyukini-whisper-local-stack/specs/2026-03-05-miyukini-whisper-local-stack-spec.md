# Specification technique -- MiyuSTT, MiyuTTS, Miyukini Whisper

## TL;DR

Architecture en 3 briques:
- `miyustt` pour transcription locale (streaming + batch)
- `miyutts` pour synthese locale (wav/pcm + cache)
- `miyukini-whisper` pour UX dictee et insertion dans champs texte

Contrats API versionnes `v1`, usage interne COG et reusage Alicia.
Les modeles STT/TTS sont locaux et independants de tout LLM texte externe.

---

## 1. Architecture cible

```text
Mic input
  -> MiyuSTT (VAD + decoder + postprocess) [runtime local dedie]
  -> Miyukini Whisper (UX, hotkeys, text insertion)
  -> Active text field

Text input
  -> MiyuTTS (synth audio) [runtime local dedie]
  -> Speaker output

Alicia / autres services
  -> API v1 MiyuSTT / MiyuTTS
```

### 1.1 Composants

1. `crates/miyustt`
- core STT
- session manager
- streaming decoder
- REST + WS adapters

2. `crates/miyutts`
- core TTS
- voice registry
- synthesis queue
- REST adapter

3. `apps/miyukini-whisper`
- hotkey global
- capture push-to-talk
- UI minimal (status, profil, logs)
- injection texte dans champ actif
- profil utilisateur (dev/auteur/etudiant)
- fallback manager (local -> host bridge -> cloud opt-in)

### 1.2 Independance stricte STT/TTS

1. `MiyuSTT` et `MiyuTTS` demarrent sans `miou-llm-bridge`.
2. Aucun appel LLM texte requis pour produire transcript/audio.
3. Le mode `rewrite` de `Miyukini Whisper` reste optionnel et separable.
4. Si bridge/cloud indisponible, STT/TTS local continuent de fonctionner.

---

## 2. API MiyuSTT v1

Base URL proposee: `http://127.0.0.1:3003`

### 2.1 Health

- `GET /api/health`
- reponse:

```json
{
  "status": "ok",
  "service": "miyustt",
  "version": "0.1.0"
}
```

### 2.2 Transcription batch

- `POST /api/stt`
- request:

```json
{
  "samples": [0.001, -0.012, 0.098],
  "sample_rate": 16000,
  "model": "stt-auto",
  "language": "auto",
  "profile": "balanced",
  "languages_hint": ["fr", "en"]
}
```

- response:

```json
{
  "transcript": "bonjour ceci est un test",
  "confidence": 0.93,
  "duration_ms": 842,
  "engine": "faster-whisper",
  "model_used": "base",
  "language_detected": "fr"
}
```

### 2.3 Streaming

- `WS /api/stt/stream`
- events:
  - `partial`
  - `final`
  - `error`

---

## 3. API MiyuTTS v1

Base URL proposee: `http://127.0.0.1:3004`

### 3.1 Health

- `GET /api/health`

### 3.2 Synthesis

- `POST /api/tts`
- request:

```json
{
  "text": "Salut, je suis prete.",
  "voice": "fr_female_01_compact",
  "format": "wav",
  "sample_rate": 22050,
  "speed": 1.0,
  "profile": "balanced"
}
```

- response:
  - audio binaire (`audio/wav`) ou JSON + URL locale temporaire

### 3.3 Voices

- `GET /api/tts/voices`
- response:

```json
{
  "voices": [
    {"id": "fr_female_01_compact", "lang": "fr", "engine": "kokoro-onnx"},
    {"id": "en_female_01_compact", "lang": "en", "engine": "kokoro-onnx"}
  ]
}
```

---

## 4. Presets hardware (auto selection)

Objectif:
- charger le plus petit modele qui atteint la qualite attendue FR/EN sur la machine.

### 4.1 STT presets

Reference tailles OpenAI Whisper:
- `tiny`: 39M
- `base`: 74M
- `small`: 244M
- `medium`: 769M
- `turbo`: 809M

| Preset | Detection machine | Moteur | Modele STT | Precision cible |
|---|---|---|---|---|
| `compact` | CPU <= 4 coeurs ou RAM <= 8 GB | faster-whisper | `base` int8 | usage quotidien FR/EN |
| `balanced` | RAM 8-16 GB ou CPU >= 6 coeurs | faster-whisper | `small` int8 | meilleure robustesse FR/EN |
| `precision` | GPU >= 8 GB ou CPU haut de gamme | faster-whisper | `medium` fp16/int8 | meilleure precision globale |
| `safe-fallback` | erreur moteur primaire | whisper.cpp | `base` | robustesse locale |

### 4.2 TTS presets

Reference kokoro-onnx:
- modele ONNX ~82 MB
- support multi-langue (incluant FR et EN)

| Preset | Detection machine | Moteur | Voix |
|---|---|---|---|
| `compact` | RAM <= 8 GB | kokoro-onnx (quantized si dispo) | 1 FR + 1 EN compact |
| `balanced` | RAM 8-16 GB | kokoro-onnx | 2 FR + 2 EN |
| `precision` | RAM >= 16 GB | kokoro-onnx full + post-filter | 4+ voix FR/EN |
| `safe-fallback` | erreur TTS primaire | espeak-ng | voix systeme FR/EN |

---

## 5. Contrat Alicia

Alicia doit consommer:
- `POST /api/stt` pour transcription voix commande
- `POST /api/nlu` (reste gerer par bridge NLU existant)
- `POST /api/tts` pour reponse vocale (phase future)

Decision d alignement:
- conserver `:3003` pour STT afin de rester compatible avec `miyualicia` actuel
- exposer `:3004` pour TTS
- fournir `X-Request-ID` et `X-Source` dans les appels inter-services

---

## 6. Moteurs et modeles

## 6.1 STT chain

1. `faster-whisper` (perf prioritaire)
2. `whisper.cpp` (fallback robuste local)
3. `sherpa-onnx` (streaming temps reel si profil low-latency)

## 6.2 TTS chain

1. `kokoro-onnx` (priorite permissive + qualite + compacite)
2. `coqui-tts` (qualite haute, cout compute plus eleve)
3. `espeak-ng` en fallback ultra-compact

## 6.3 LLM post-processing (optionnel)

Modes:
- `verbatim`: pas de LLM
- `clean`: regles + LLM court
- `rewrite`: LLM local complet

Modeles recommandes:
- Qwen2.5 3B Instruct
- Mistral 7B Instruct
- Llama 3.x Instruct

Backend:
- `miou-llm-bridge` natif GGUF par defaut
- fallback upstream local (`localhost:1234` LM Studio, `localhost:11434` Ollama)

---

## 7. Fallback host/cloud (opt-in)

Ordre de routage:
1. local-only (defaut)
2. host bridge COG utilisateur (si configure et sain)
3. cloud provider (si active explicitement par utilisateur)

Regles:
- aucun fallback distant sans consentement utilisateur explicite.
- afficher en UI le backend reel utilise.
- timeout strict puis retour local/fallback suivant.

---

## 8. Interface Central -- test et configuration

Nouveau panneau dans Central: `Miyukini Whisper`.

### 8.1 Onglets UI

1. `STT Test`
- bouton enregistrer
- transcript partiel/final
- latence mesuree

2. `TTS Test`
- zone texte
- choix voix FR/EN
- bouton lire

3. `Presets`
- auto detect hardware
- override manuel (`compact|balanced|precision`)

4. `Fallback`
- local-only
- host bridge URL + health test
- cloud provider (opt-in) + statut auth

5. `Diagnostics`
- etat `/api/health` STT/TTS
- modele charge, moteur actif, erreurs recentes

### 8.2 Integration Central cible

- `apps/central/src/services/miyukini_whisper/mod.rs`
- ajout dans registre services Central
- preferences persistees dans profil utilisateur

---

## 9. Objectifs non-fonctionnels

| Metrique | Cible V1 |
|---|---|
| Latence STT (fin parole -> texte final) | < 1200 ms sur machine cible |
| Latence TTS (texte -> debut audio) | < 500 ms |
| WER FR/EN compact | < 12% |
| WER FR/EN balanced | < 8% |
| WER FR/EN precision | < 5% |
| Availability service local | > 99% session locale |
| Temps demarrage service | < 3 s |

---

## 10. Securite et privacy

- local-first obligatoire
- aucune sortie cloud par defaut
- auth bearer optionnelle inter-services
- origin restriction localhost par defaut
- logs sans contenu sensible brut
- purge buffers audio temporaires

---

## 11. Tests requis

1. Unit tests
- decoding STT
- segmentation VAD
- queue TTS
- mapping profils

2. Integration tests
- `/api/stt`, `/api/tts`, `/api/health`
- compat Alicia (`X-Source: alicia-home`)
- fallback chain moteur

3. E2E produit
- hotkey -> dictee -> insertion texte
- mode `clean` et `rewrite`
- lecture TTS de confirmation

---

## 12. Livrables P3 attendus

- crates:
  - `crates/miyustt`
  - `crates/miyutts`
- app:
  - `apps/miyukini-whisper`
- UI:
  - panneau Central `Miyukini Whisper`
- docs API:
  - OpenAPI STT/TTS
- tests:
  - suite integration + bench latence
