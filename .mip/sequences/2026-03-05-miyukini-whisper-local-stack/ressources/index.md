# Index ressources -- miyukini-whisper-local-stack

## 1) STT open-source

| Ressource | Type | Licence | Lien |
|---|---|---|---|
| whisper.cpp | Inference STT locale C/C++ | MIT | https://github.com/ggml-org/whisper.cpp |
| faster-whisper | STT rapide (CTranslate2) | MIT | https://github.com/SYSTRAN/faster-whisper |
| sherpa-onnx | ASR/TTS/KWS on-device | Apache-2.0 | https://github.com/k2-fsa/sherpa-onnx |

Notes:
- `whisper.cpp` et `faster-whisper` sont les 2 candidats principaux pour V1.
- `sherpa-onnx` est une bonne option streaming basse latence.
- OpenAI Whisper indique des tailles de references:
  - tiny 39M, base 74M, small 244M, medium 769M, turbo 809M.
- Les modeles non `.en` sont multilingues et donc adaptes FR/EN.

---

## 2) TTS open-source

| Ressource | Type | Licence | Lien |
|---|---|---|---|
| kokoro-onnx | TTS ONNX local | MIT | https://github.com/thewh1teagle/kokoro-onnx |
| Coqui TTS | TTS multi-modele | MPL-2.0 | https://github.com/coqui-ai/TTS |
| Piper (archive historique) | TTS local classique | Archive (rhasspy), voir fork actif | https://github.com/rhasspy/piper |
| piper1-gpl (etat actuel) | TTS local | GPL (a verifier selon usage) | https://github.com/OHF-Voice/piper1-gpl |

Notes:
- Pour V1 COG, `kokoro-onnx` est prioritaire (licence claire, integration ONNX).
- `kokoro-onnx` annonce un modele ONNX ~82 MB et un support multilingue.
- Le depot `rhasspy/piper` est archive (annonce visible sur GitHub, 6 janvier 2025).
- Le fork `piper1-gpl` est actif (release recente visible, 27 aout 2025), mais sous GPL.
- Piper reste possible mais doit passer un gate legal explicite.

---

## 3) LLM open-weights pour post-traitement texte

| Modele | Usage | Licence (model card) | Lien |
|---|---|---|---|
| Qwen2.5-3B-Instruct | clean/rewrite leger | Apache-2.0 | https://huggingface.co/Qwen/Qwen2.5-3B-Instruct |
| Mistral-7B-Instruct-v0.3 | rewrite qualite | Apache-2.0 | https://huggingface.co/mistralai/Mistral-7B-Instruct-v0.3 |
| Llama-3.2-3B-Instruct | rewrite alternatif | Llama 3.2 Community License | https://huggingface.co/meta-llama/Llama-3.2-3B-Instruct |

Important:
- Ces modeles sont pour post-traitement optionnel.
- Le mode `verbatim` doit rester sans LLM.

---

## 4) Runtime local LLM open-source

| Ressource | Role | Licence | Lien |
|---|---|---|---|
| llama.cpp | inference locale GGUF | MIT | https://github.com/ggml-org/llama.cpp |
| Ollama | orchestrateur modeles locaux | MIT | https://github.com/ollama/ollama |

Dans ce repo:
- `apps/miou-llm-bridge` est deja base sur `llama-cpp-2` + fallback upstream local.

---

## 5) Reference produit comparable (open-source)

| Ressource | Positionnement | Lien |
|---|---|---|
| OpenWhispr | dictation locale multi-langue | https://github.com/Mhmd-Hisham/OpenWhispr |
| Voquill | voice to text local pour ecriture | https://www.voquill.com/ |

Usage:
- inspiration UX (hotkeys, profils, insertion texte)
- ne pas copier architecture sans verifier licences et perf

---

## 6) Recommandation stack V1

### Adopter maintenant

1. STT: `faster-whisper` + fallback `whisper.cpp` (modeles multilingues FR/EN)
2. TTS: `kokoro-onnx` (FR/EN)
3. LLM optionnel rewrite: `Qwen2.5-3B-Instruct` via `miou-llm-bridge`
4. Runtime local: `llama.cpp`/GGUF

### Evaluer ensuite

1. `sherpa-onnx` pour streaming ultra-faible latence
2. `Mistral 7B` pour qualite rewrite superieure
3. `Coqui TTS` pour scenario voix plus naturelle

### Eviter en V1

1. cloud obligatoire
2. voice cloning complexe
3. pipelines licence incertaine sans audit legal

---

## 7) Presets hardware recommandes

| Preset | STT | TTS | Machine cible |
|---|---|---|---|
| `compact` | Whisper `base` (74M) int8 | kokoro-onnx compact (ou espeak-ng fallback) | 8 GB RAM, CPU standard |
| `balanced` | Whisper `small` (244M) int8 | kokoro-onnx standard | 16 GB RAM, CPU/GPU moyen |
| `precision` | Whisper `medium` (769M) | kokoro-onnx qualite max | GPU present, RAM >= 16 GB |

Regle:
- selection auto au demarrage
- override manuel en UI Central
- fallback host/cloud uniquement en opt-in

---

## 8) Exemples inter-services

- Exemples d appels API pour Alicia et autres services COG:
  - `ressources/api-usage-exemples.md`
