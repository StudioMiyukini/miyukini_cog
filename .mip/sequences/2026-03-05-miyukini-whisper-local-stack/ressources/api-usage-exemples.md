# Exemples usage API -- MiyuSTT / MiyuTTS (inter-services COG)

## 1) Alicia -> MiyuSTT (`/api/stt`)

```bash
curl -X POST "http://127.0.0.1:3003/api/stt" \
  -H "Content-Type: application/json" \
  -H "X-Request-ID: 7f2b93f4-6d0a-4c48-a15c-9fbb8b5f0e30" \
  -H "X-Source: alicia-home" \
  -d '{
    "samples": [0.01, -0.03, 0.02, -0.01],
    "sample_rate": 16000,
    "model": "whisper-local",
    "language": "fr",
    "languages_hint": ["fr", "en"],
    "preset": "balanced"
  }'
```

Reponse attendue (exemple):

```json
{
  "transcript": "ceci est une transcription de demonstration",
  "confidence": 0.93,
  "duration_ms": 120,
  "engine": "faster-whisper",
  "model_used": "whisper-local",
  "language_detected": "fr"
}
```

## 2) Alicia -> MiyuTTS (`/api/tts/wav`)

```bash
curl -X POST "http://127.0.0.1:3004/api/tts/wav" \
  -H "Content-Type: application/json" \
  -H "X-Request-ID: 6305fd0f-5f1c-4d38-ae31-1ec7f231a0ac" \
  -H "X-Source: alicia-home" \
  -d '{
    "text": "Bonjour, commande executee.",
    "voice": "fr_female_01_compact",
    "format": "wav",
    "language": "fr",
    "sample_rate": 22050
  }' \
  --output alicia_reply.wav
```

## 3) Contrat fallback host/cloud (Miyukini Whisper)

Resolution cible:
1. `local` si service local joignable
2. `host_bridge` si local indisponible et mode `host_bridge`
3. `cloud` seulement si mode `host_bridge_and_cloud` ET `cloud_opt_in=true`

Regles:
1. `local_only` bloque tout fallback distant
2. `host_bridge` n autorise jamais le cloud
3. `host_bridge_and_cloud` n autorise cloud qu en cas d indisponibilite host bridge
