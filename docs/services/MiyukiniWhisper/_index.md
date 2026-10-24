# Miyukini Whisper — index

<!-- @id services.miyukiniwhisper.index -->
<!-- @role service-reference -->
<!-- @layer S -->
<!-- @human Dictée locale STT/TTS avec presets matériel, profils de sortie et repli opt-in ; squelette V0 assemblant les toolkits MiyuSTT / MiyuTTS -->
<!-- @do reference_miyukini_whisper -->

- **Strate** : S (service) ; s'appuie sur deux toolkits T (`miyustt`, `miyutts`)
- **Statut** : squelette V0 — logique de configuration testée, **aucune transcription ni synthèse réelle** (§6)
- **Voisin** : [MiyukiniVoice](../MiyukiniVoice/) (assistant vocal, spécifications séparées)

## 1. Identité

<!-- @id services.miyukiniwhisper.identite -->
<!-- @role component-inventory -->
<!-- @do list_whisper_components -->

| Élément | Chemin | Rôle |
|---|---|---|
| `miyukini-whisper-app` (lib `miyukini_whisper_app`, bin `miyukini-whisper`) | `apps/miyukini-whisper` | config, choix de backend, post-traitement du texte |
| `miyustt` | `crates/miyustt` | toolkit STT : presets, routeur HTTP `/api/stt` |
| `miyutts` | `crates/miyutts` | toolkit TTS : presets, routeur HTTP `/api/tts` |
| `MiyukiniWhisperView` | `apps/central/src/services/miyukini_whisper/mod.rs` | panneau Central : test/config + diagnostic `/api/health` |
| `maia-stt` (hors service) | `crates/maia-stt` | moteur whisper.cpp réel (feature `whisper`) pour MAIA ; **non branché** à Miyukini Whisper |

## 2. Ce que fait le service

<!-- @id services.miyukiniwhisper.fonctions -->
<!-- @role feature-summary -->
<!-- @do summarize_whisper_features -->

- Choisit un preset STT/TTS selon le matériel (`select_preset`, défaut : 4 cœurs, 8 Go, sans GPU)
- Profils de sortie : `Verbatim` (texte brut) · `Clean` (espaces, majuscule, point final) · `Rewrite` (LLM)
- Repli : `LocalOnly` (défaut) · `HostBridge` · `HostBridgeAndCloud` (cloud seulement si `cloud_opt_in`)
- `Rewrite` : délai max `rewrite_timeout_ms` (800 ms) puis repli sur `Clean`
- Binaire : `run()` journalise la config et un texte de test codé en dur, puis s'arrête (`src/lib.rs` l. 272)

## 3. API et points d'entrée

<!-- @id services.miyukiniwhisper.api -->
<!-- @role public-api -->
<!-- @do list_whisper_api -->

- `WhisperConfig` (`Default`), `OutputProfile`, `FallbackMode`, `RewriteBackend`, `ServiceBackend`
- `resolve_rewrite_backend(cfg, local_llm, host_bridge) -> RewriteBackend`
- `resolve_service_backend(cfg, local, host_bridge, cloud) -> Option<ServiceBackend>`
- `clean_text(&str)`, `post_process_text(transcript, cfg, …, rewrite_fn)`
- HTTP `miyustt` : `GET /api/health` · `POST /api/stt` · `GET /api/stt/stream` (WebSocket)
- HTTP `miyutts` : `GET /api/health` · `GET /api/tts/voices` · `POST /api/tts` · `POST /api/tts/wav`
- Central : URL par défaut STT `127.0.0.1:3003`, TTS `127.0.0.1:3004` (modifiables dans l'UI)

## 4. Données et stockage

<!-- @id services.miyukiniwhisper.donnees -->
<!-- @role data-storage -->
<!-- @do describe_whisper_storage -->

- Aucune persistance : config en mémoire (`WhisperConfig::default()`), pas de fichier
- STT : échantillons remis à zéro après traitement (`req.samples.fill(0.0)`, `miyustt/src/lib.rs`)
- TTS : texte effacé après synthèse (`scrub_text`, `miyutts/src/lib.rs`)
- Aucun modèle chargé (moteurs simulés)

## 5. Posture sécurité

<!-- @id services.miyukiniwhisper.securite -->
<!-- @role security-posture -->
<!-- @do assess_whisper_security -->

- **Exposition** : le binaire n'ouvre aucun port ; les routeurs ne sont servis que par `alicia-mvp` (`apps/alicia-mvp/src/voice.rs` l. 141-175 : `127.0.0.1:0`, bearer `None`, origine non contrôlée) — aucun serveur `3003/3004` trouvé dans le dépôt (**à vérifier**)
- **Auth** : bearer optionnel, `None` par défaut → **fail-open** — **gap**
- **Comparaison du jeton** : `token == expected_token`, non constant (`miyustt` / `miyutts` `enforce_bearer_auth`) — **gap**
- **Origine** : `is_local_origin` par `starts_with("http://localhost")` etc. → `http://localhost.exemple.com` ou `http://127.0.0.1.exemple.com` acceptés — **gap**
- **Sans en-tête `Origin`** : accepté (clients non navigateur)
- **Cloud** : jamais sans `cloud_opt_in` (testé) ; `HostBridge` sans authentification définie (à vérifier)
- **Ressources** : `post_process_text` laisse tourner le thread de réécriture après expiration du délai (`lib.rs` l. 250-255)
- **Gouvernance COG** : aucune (pas de MasterButler / StrongFather)

## 6. Statut

<!-- @id services.miyukiniwhisper.statut -->
<!-- @role status -->
<!-- @do report_whisper_status -->

| Composant | Tests | État |
|---|---|---|
| `miyukini-whisper-app` | 6 | logique de config / repli fonctionnelle ; pas de pipeline audio |
| `miyustt` | 8 | **stub** : transcription fixe (« ceci est une transcription de démonstration ») ; streaming WS figé |
| `miyutts` | 7 | **stub** : WAV silencieux (durée = 22 ms × caractères, 280–3 500 ms) |
| Vue Central | 0 | « Simuler transcription » / « Simuler lecture TTS » ; seul `/api/health` fait un vrai appel |
| `maia-stt` | 16 | VAD, confiance, pipeline ; whisper.cpp derrière feature `whisper` (désactivée par défaut) |

## 7. Lacunes connues

<!-- @id services.miyukiniwhisper.lacunes -->
<!-- @role known-gaps -->
<!-- @do list_whisper_gaps -->

- Aucun moteur réel : faster-whisper / whisper.cpp / sherpa-onnx / Kokoro / eSpeak seulement nommés
- `maia-stt` (whisper.cpp réel) non relié à `miyustt`
- Pas de capture micro ni de sortie audio dans le service
- Conflit de port : `127.0.0.1:3003` sert à la fois l'URL NLU (miou-llm-bridge) et STT dans `crates/miyualicia/src/config.rs` l. 135-141 — **à vérifier**
- Correctifs sécurité attendus : correspondance d'origine stricte (hôte exact), jeton obligatoire hors loopback, comparaison à temps constant
- Pas de doc toolkit `docs/tools/MiyuSTT` / `MiyuTTS`
