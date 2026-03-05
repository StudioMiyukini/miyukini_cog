# Brief de cadrage P0 -- MiyuSTT, MiyuTTS, Miyukini Whisper (local-first)

## TL;DR

Objectif: lancer une suite vocale locale pour dictee rapide dans tous les champs texte du COG.
La sequence cree deux toolkits reutilisables (`MiyuSTT`, `MiyuTTS`) et un service final utilisateur (`Miyukini Whisper`).
Priorite produit: productivite clavier (dev, auteur, etudiant), faible latence, zero cloud obligatoire.
Priorite architecture: APIs stables pour Alicia et autres services COG.
Approche recommandee: execution en 3 blocs (toolkits -> service -> integrations).

---

## Metadonnees

- Classe: T5
- Date: 2026-03-05
- Sequence: `.mip/sequences/2026-03-05-miyukini-whisper-local-stack/`
- Statut: P0 lance
- Mode autonomie recommande: BIG_STEPS
- Cible principale: Windows desktop (COG local), extension Linux ensuite

---

## Contexte

Le repo contient deja:
- capture audio/VAD (`miyualicia-capture`)
- wake word (`miyualicia-wakeword`)
- orchestration LLM locale (`miou-llm-bridge`)
- TTS eSpeak integre dans Central

Manques actuels:
- toolkit STT dedie, industrialisable et API-first
- toolkit TTS dedie, industrialisable et API-first
- service de dictee utilisateur "always-on" comparable a WhisperFlow/SuperWhisper
- contrat API vocal uniforme pour Alicia et services COG

---

## Objectifs

| Priorite | Objectif | Critere de succes |
|---|---|---|
| P0 | Creer toolkit `MiyuSTT` | API REST/WS stable + tests + bench latence |
| P0 | Creer toolkit `MiyuTTS` | API REST stable + streaming audio + voix FR/EN |
| P0 | Creer service `Miyukini Whisper` | Dictee locale dans champs texte, mode push-to-talk |
| P1 | Reutilisation inter-services | Alicia consomme STT/TTS via API versionnee |
| P1 | Qualite produit | WER et latence cibles atteints, UX clavier fluide |

---

## Contraintes imposees

1. Les modeles STT et TTS tournent localement, en runtime dedie, sans dependance obligatoire a un autre LLM.
2. Les modeles doivent etre compacts en mode par defaut, avec precision maximale possible en FR + EN.
3. Le systeme doit gerer des presets hardware pour charger automatiquement le modele adapte a la machine.
4. Le fallback est autorise uniquement en mode opt-in:
   - bridge sur COG host de l utilisateur (si present)
   - cloud (si active explicitement)
5. Une interface utilisateur de test/configuration doit etre integree dans Central.

---

## Scope

### Inclus

- Nouveau crate `crates/miyustt/`
- Nouveau crate `crates/miyutts/`
- Nouveau service `apps/miyukini-whisper/`
- API versionnee `v1` pour STT/TTS
- Integration initiale Alicia (via endpoints stables)
- Contrat "local-first" avec fallback explicite
- Evaluation techno open-source STT/TTS/LLM local
- UI Central pour tester et configurer Miyukini Whisper
- Presets hardware auto (compact/balanced/precision)

### Exclus

- Cloud obligatoire
- Telemetrie distante
- Training modele maison
- Mobile natif (Android/iOS) en V1
- Voice cloning avance en V1

---

## Cibles produit

1. Dev: dicter code commentaires, commit messages, notes techniques.
2. Auteur: dicter paragraphes, correction ponctuation rapide.
3. Etudiant: prise de notes, synthese vocale de relecture.
4. Power users clavier: insertion rapide sans quitter le focus courant.

---

## Vision fonctionnelle

`Miyukini Whisper` doit fournir:
- hotkey global (push-to-talk et toggle)
- transcription streaming en direct
- insertion texte dans le champ actif
- commandes vocales de base (nouvelle ligne, ponctuation, annuler)
- profils de sortie:
  - `verbatim` (brut)
  - `clean` (ponctuation/formatage leger)
  - `rewrite` (post-traitement LLM local optionnel)
- presets hardware:
  - `compact` (priorite RAM/CPU)
  - `balanced` (equilibre qualite/latence)
  - `precision` (priorite qualite transcription)
- fallback policy configurable:
  - local only (defaut)
  - host bridge
  - host bridge + cloud

---

## Interface Central (obligatoire V1)

Ajouter un panneau `Miyukini Whisper` dans Central avec:
1. Test STT en direct (micro -> transcript)
2. Test TTS (texte -> audio)
3. Choix preset hardware auto ou manuel
4. Choix langues actives FR/EN
5. Configuration fallback (local/bridge/cloud)
6. Diagnostic latence, charge modele, etat endpoints `/api/health`

---

## Stack open-source candidate

### STT

- `whisper.cpp` (C/C++): inference locale, support GGML/GGUF et streaming.
- `faster-whisper` (Python/CTranslate2): perf elevee sur GPU/CPU.
- `sherpa-onnx` (C++/Python): ASR streaming on-device multi-plateforme.

Decision cible:
- defaut `faster-whisper` en modele compact multilingue
- fallback `whisper.cpp` pour robustesse locale

### TTS

- `Kokoro ONNX` (MIT): TTS ONNX moderne, pipeline local.
- `Piper` (etat ecosysteme a verifier selon licence/projet cible): option legacy.
- `Coqui TTS` (MPL-2.0): option qualite elevee, plus lourde.

Decision cible:
- defaut `kokoro-onnx` (compact + multilingue)
- fallback `espeak-ng` sur machines tres contraintes

### LLM local (post-traitement texte optionnel)

- `Qwen2.5 Instruct` (open-weights permissif) pour clean/rewrite.
- `Mistral 7B Instruct` (open-weights) pour rewrite plus riche.
- `Llama 3.x Instruct` (open-weights, licence Meta) si deja present en local.

Inference recommande:
- priorite `miou-llm-bridge` natif GGUF (`llama-cpp-2`)
- fallback upstream local (`LM Studio` / `Ollama`)

---

## Approches

### Approche A -- Toolkits d abord, service ensuite (recommandee)

- Etape 1: stabiliser `MiyuSTT` API + tests.
- Etape 2: stabiliser `MiyuTTS` API + tests.
- Etape 3: brancher `Miyukini Whisper` sur ces APIs.
- Etape 4: integration Alicia.

Avantages: dette technique faible, reusage immediat.
Risque: time-to-first-demo un peu plus long.

### Approche B -- Service d abord, toolkits ensuite

- Prototyper vite le service final puis extraire.

Avantages: demo rapide.
Risque: rework important, API moins propres.

### Approche C -- Monolithe unique vocal

- Un seul service pour STT/TTS/dictee sans toolkits separables.

Avantages: implementation initiale rapide.
Risque: non alignement objectif "API pour Alicia", couplage fort.

Recommandation: Approche A.

---

## Risques principaux

| Risque | Probabilite | Impact | Mitigation |
|---|---|---|---|
| Latence STT trop elevee sur CPU faible | Moyen | Eleve | profils modele (tiny/base/small), bench auto |
| Qualite FR insuffisante en mode rapide | Moyen | Eleve | eval FR sur corpus cible + fallback modele |
| Contrat API divergent Alicia | Moyen | Moyen | spec API unique `v1`, tests contrat |
| Licences incompatibles | Faible | Eleve | gate legal en P4, matrice licence stricte |
| Hotkey global instable multi-apps | Moyen | Moyen | abstraction OS + tests e2e focus champs |

---

## Decision P0

- Sequence creee et validee pour execution.
- Priorite d execution:
  1. `MiyuSTT`
  2. `MiyuTTS`
  3. `Miyukini Whisper` + UI Central
  4. Integration Alicia
- Gate P3->P4 obligatoire avant packaging final.

---

## Prochaines etapes

1. Finaliser spec technique des APIs STT/TTS et flux dictee.
2. Executer plan P3 par vagues.
3. Produire audit conformite + securite + licences.
4. Livrer demo locale "dictee -> champ texte" + endpoints Alicia.
5. Exploiter l analyse PR + concurrence:
   - `.mip/sequences/2026-03-05-miyukini-whisper-local-stack/briefs/2026-03-05-miyukini-whisper-local-stack-pr-concurrence.md`
