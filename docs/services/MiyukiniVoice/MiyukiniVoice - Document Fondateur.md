# MiyukiniVoice - Document Fondateur

> @id service.voice.miyukinivoice
> @role operator
> @layer 7
> @do orchestrate_local_voice_assistant_pipeline
> @human Assistant vocal domotique 100% local integre dans Miyukini COG

---

## Contexte

Le projet Miyukini COG dispose d'une architecture de gouvernance mature (Strates 0-9, 8 Cores, 49+ toolkits, 13+ services). L'ecosysteme couvre la gestion d'evenements, la comptabilite, le CMS, le jeu video, le e-commerce, etc. Cependant, aucune interface vocale n'existe a ce jour.

L'utilisateur souhaite creer **MiyukiniVoice**, un assistant vocal domotique 100% local (zero cloud), couvrant 4 pieces du domicile via microphones filaires. Cet assistant doit detecter un mot-cle de reveil ("Hey Alicia"), transcrire la parole, interroger un LLM local, synthetiser la reponse vocalement, et la router vers la bonne piece.

MiyukiniVoice constituera l'interface vocale d'**Alicia**, l'assistante personnelle bienveillante de l'ecosysteme Miyukini, telle que definie dans le document d'equipe. Il s'agit d'un service interne COG Type 1, positionne en Strate 7 (Operateur), s'appuyant sur des toolkits Strate 6 dedies.

## Portee / Scope

### Inclus (v1)

- Pipeline vocal complet : capture, wake word, STT, LLM, TTS, routage audio
- 4 pieces couvertes : Chambre Theresa, Chambre parentale, Chambre Eleanore, Salon
- Microphones filaires MAX9814 via cartes son USB
- Haut-parleurs Tiardey 3W via amplis PAM8403
- Detection wake word "Hey Alicia"
- Transcription Whisper locale (GPU)
- Reponse LLM locale (Qwen3 4B, Qwen 2.5 7B en fallback)
- Synthese vocale Piper (voix francaise)
- Routage de la reponse vers la piece d'origine
- Integration complete dans l'architecture COG (Cores, toolkits, gouvernance)
- Support francais principal, anglais secondaire

### Exclus (v1)

- Aucun service cloud (ni STT, ni TTS, ni LLM distant)
- Pas de controle domotique physique (prises, lumieres, volets) en v1
- Pas d'interface visuelle dediee (ecran Raspberry Pi traite en v2+)
- Pas de multi-utilisateur avec identification vocale (speaker diarization)
- Pas de mode conversation continue (chaque echange = wake word + commande)
- Pas de musique/streaming audio
- Pas de support mobile/distant

---

## 1. Resume executif

MiyukiniVoice est un assistant vocal domotique 100% local, integre dans l'ecosysteme Miyukini COG comme Operateur Strate 7. Il capture l'audio de 4 pieces via microphones filaires, detecte le mot-cle "Hey Alicia", transcrit la parole avec Whisper local sur GPU, genere une reponse via LLM local (Qwen3 4B), synthetise cette reponse en voix francaise via Piper, et la diffuse dans la piece d'origine. Aucune donnee ne quitte le reseau local. Le budget hardware est deja engage (~89 EUR).

---

## 2. Analyse de la requete

### 2.1 Decomposition des besoins utilisateur

| Besoin | Categorie | Priorite | Complexite |
|--------|-----------|----------|------------|
| Ecouter en permanence dans 4 pieces | Capture audio | Critique | Moyenne |
| Detecter "Hey Alicia" | Wake word | Critique | Moyenne |
| Transcrire la parole en texte | STT | Critique | Elevee |
| Generer une reponse intelligente | LLM | Critique | Elevee |
| Synthetiser la reponse en voix | TTS | Critique | Moyenne |
| Router la reponse vers la bonne piece | Routage audio | Critique | Moyenne |
| 100% local, zero cloud | Contrainte architecturale | Absolue | Elevee |
| 100% Rust en production | Contrainte technique | Absolue | Elevee |
| Latence < 2s (fin parole -> debut TTS) | Performance | Importante | Elevee |
| Francais principal | Locale | Critique | Faible |
| Integration COG (Cores, gouvernance) | Architecture | Importante | Moyenne |
| Vie privee (traitement post-wake word uniquement) | Securite | Absolue | Faible |

### 2.2 Contraintes identifiees

1. **LOI-1 (Aucune dependance externe critique)** : tout doit fonctionner sans Internet. Les modeles IA (Whisper, LLM, Piper) doivent etre telecharges a l'avance et stockes localement.
2. **LOI-5 (Cout proportionnel au hardware)** : les modeles doivent etre dimensionnes pour le hardware disponible (RTX 5070 12 Go, RTX 3060 Ti 8 Go).
3. **`unsafe_code = "forbid"`** : tous les bindings C/C++ (whisper.cpp, llama.cpp, piper) doivent etre encapsules dans des crates Rust qui exposent des APIs safe. Les bindings FFI existants (`whisper-rs`, `llama-cpp-rs`) gerent deja cela via des wrappers safe.
4. **WSL2 et audio USB** : l'acces aux peripheriques audio USB depuis WSL2 est problematique. Deux strategies possibles : `usbipd-win` pour exposer les cartes son USB a WSL2, ou build natif Windows avec WASAPI.

### 2.3 Hardware disponible

#### Machine serveur unique

| Machine | CPU | GPU | RAM | Role |
|---------|-----|-----|-----|------|
| **Machine A (serveur MiyukiniVoice)** | Intel Core Ultra 7 265K | RTX 5070 12 Go | 32 Go DDR5 | LLM + STT + TTS + pipeline complet |

Machine B (i5-10K, 3060 Ti 8 Go) n'est **pas utilisee** pour MiyukiniVoice. Elle reste disponible pour les autres services COG.

#### Equipement par piece (x4)

| Composant | Modele | Role |
|-----------|--------|------|
| Microphone | MAX9814 (electret amplifie) | Capture voix |
| Haut-parleur | Tiardey 3W | Diffusion reponse |
| Amplificateur | PAM8403 | Amplification HP |
| Carte son | USB (entree micro + sortie HP) | Interface audio |
| Cable | Cat6 50m | Liaison piece -> machine |

#### Affichage salon

| Composant | Role |
|-----------|------|
| Raspberry Pi 4 | Client leger salon |
| Ecran 15.6" | Affichage visuel (v2+) |
| Moonlight/Parsec | Streaming GPU depuis Machine A |

### 2.4 Budget hardware

| Poste | Cout |
|-------|------|
| 4x MAX9814 | ~16 EUR |
| 4x PAM8403 | ~8 EUR |
| 4x HP Tiardey 3W | ~24 EUR |
| 4x Carte son USB | ~24 EUR |
| 4x Cat6 50m | ~Deja en stock / ~17 EUR |
| **Total** | **~89 EUR** |

**Statut : deja engage.**

---

## 3. Architecture haut niveau

### 3.1 Positionnement COG

```
Strate 7 (Operateur)    : miyukinivoice          -- Pipeline orchestration
Strate 6 (Toolkits)     : miyuvoicecapture       -- Capture audio multi-pieces
                           miyuwakeword           -- Detection wake word
                           miyustt                -- Speech-to-Text (Whisper)
                           miyutts                -- Text-to-Speech (Piper)
                           miyuvoicerouter        -- Routage audio sortie
Strate 5 (BondingBrother): mediation inter-Cores
Strate 4 (Cores)        : KindMother, StrongFather, CaringNanny,
                           MasterButler, WorrySentinel
```

### 3.2 Pipeline audio complet

```
[Piece N]                  [Machine A]

Micro MAX9814              miyuvoicecapture (cpal)
    |                          |
Cable Cat6 50m             Buffer circulaire PCM
    |                          |
Carte son USB -----------> miyuwakeword (rustpotter)
                               |
                          [Wake word detecte?]
                               |
                          miyustt (whisper-rs, GPU)
                               |
                          Texte transcrit
                               |
                          Alicia / LLM local
                          (miou-voice-bridge, fork dedie)
                               |
                          Reponse texte
                               |
                          miyutts (piper-rs, CPU)
                               |
                          Audio PCM synthetise
                               |
                          miyuvoicerouter
                               |
Carte son USB <----------- Sortie vers piece N
    |
Cable Cat6 50m
    |
PAM8403 -> HP Tiardey 3W
    |
[Piece N - reponse audible]
```

### 3.3 Interactions Cores

| Core | Role dans MiyukiniVoice |
|------|------------------------|
| **StrongFather** | Decide l'activation du pipeline, autorise les modeles IA |
| **KindMother** | Persiste les logs de sessions vocales, preferences utilisateur, configurations pieces |
| **CaringNanny** | Observe les metriques (latence, erreurs STT, taux de detection wake word) |
| **MasterButler** | Gere les permissions d'acces aux peripheriques audio, niveaux d'acces |
| **WorrySentinel** | Surveille la securite (pas d'enregistrement continu, vie privee), etat de confiance |

### 3.4 Structure crates

```
crates/
+-- miyukinivoice/                 # Operateur (Strate 7)
|   +-- Cargo.toml
|   +-- src/
|       +-- lib.rs                 # Point d'entree, orchestration pipeline
|       +-- data/
|       |   +-- mod.rs             # Feature flags
|       |   +-- types.rs           # VoiceSession, RoomConfig, PipelineState
|       |   +-- kindmother_db.rs   # Persistance sessions, configs
|       +-- services/
|       |   +-- mod.rs
|       |   +-- pipeline.rs        # Orchestration du flux complet
|       |   +-- room_manager.rs    # Gestion des 4 pieces
|       +-- domain/
|           +-- mod.rs
|           +-- session.rs         # Logique de session vocale
|
+-- miyuvoicecapture/              # Toolkit (Strate 6)
|   +-- Cargo.toml
|   +-- src/
|       +-- lib.rs
|       +-- admin_cell.rs
|       +-- context.rs
|       +-- errors.rs
|       +-- capture.rs             # Streams audio cpal, buffer circulaire
|       +-- devices.rs             # Detection/enumeration cartes son USB
|       +-- vad.rs                 # Voice Activity Detection simple (optionnel)
|
+-- miyuwakeword/                  # Toolkit (Strate 6)
|   +-- Cargo.toml
|   +-- src/
|       +-- lib.rs
|       +-- admin_cell.rs
|       +-- context.rs
|       +-- errors.rs
|       +-- detector.rs            # Rustpotter (principal) via trait WakeWordDetector (Porcupine en fallback)
|       +-- models.rs              # Gestion du modele wake word
|
+-- miyustt/                       # Toolkit (Strate 6)
|   +-- Cargo.toml
|   +-- src/
|       +-- lib.rs
|       +-- admin_cell.rs
|       +-- context.rs
|       +-- errors.rs
|       +-- transcriber.rs         # whisper-rs, gestion GPU
|       +-- language.rs            # Detection langue, config FR/EN
|
+-- miyutts/                       # Toolkit (Strate 6)
|   +-- Cargo.toml
|   +-- src/
|       +-- lib.rs
|       +-- admin_cell.rs
|       +-- context.rs
|       +-- errors.rs
|       +-- synthesizer.rs         # piper-rs ou process Piper
|       +-- voices.rs              # Catalogue de voix (fr siwis, etc.)
|
+-- miyuvoicerouter/               # Toolkit (Strate 6)
    +-- Cargo.toml
    +-- src/
        +-- lib.rs
        +-- admin_cell.rs
        +-- context.rs
        +-- errors.rs
        +-- router.rs              # Routage audio vers la bonne sortie
        +-- output.rs              # Playback cpal/rodio par device
```

### 3.5 Stack logicielle detaillee

| Couche | Crate Rust | Lib sous-jacente | VRAM/RAM | Latence estimee |
|--------|-----------|-----------------|----------|-----------------|
| Capture audio | `cpal` 0.17+ | WASAPI (Windows natif) | 0 | < 10ms |
| Serveur audio | WASAPI (Windows natif) | Natif OS | 0 | < 5ms |
| Wake word | `rustpotter` 3.x | Natif Rust | ~50 Mo RAM | < 100ms |
| STT | `whisper-rs` 0.15+ | whisper.cpp (CUDA) | 2-5 Go VRAM | 500-1500ms |
| LLM | Fork `miou-voice-bridge` (Qwen3 4B via LM Studio) | llama.cpp (CUDA) | ~3 Go VRAM | ~450ms (30 tokens) |
| TTS | `piper-rs` ou process Piper | ONNX Runtime (CPU) | ~100 Mo RAM | 200-500ms |
| Playback | `cpal` + `rodio` | WASAPI/ALSA | 0 | < 10ms |

### 3.6 Modeles IA et dimensionnement VRAM

#### Serveur unique : Machine A — RTX 5070 12 Go VRAM

**Decision :** Toute l'inference (LLM + STT + TTS) tourne sur Machine A uniquement. Machine B n'est pas utilisee pour MiyukiniVoice.

**Contrainte VRAM :** Whisper small (~2 Go) + Piper (CPU) = ~2 Go GPU. Il reste **~9-10 Go** pour le LLM. Le modele gpt-oss-20b (~10-12 Go) est trop gros pour cohabiter avec Whisper. Le modele LLM sera choisi par Denis parmi les candidats suivants :

| Modele candidat | VRAM estimee | Total avec Whisper | Marge | Notes |
|----------------|-------------|-------------------|-------|-------|
| Mistral 7B Q4_K_M | ~5 Go | ~7 Go | 5 Go | Rapide, bon FR, tool calling |
| Llama 3.1 8B Q4 | ~5-6 Go | ~7-8 Go | 4-5 Go | Bon tool calling natif |
| Qwen 2.5 14B Q4 | ~8-9 Go | ~10-11 Go | 1-2 Go | Meilleur raisonnement, serre |
| Phi-3 Medium 14B Q4 | ~8 Go | ~10 Go | 2 Go | Bon compromis |

**Criteres de selection (pour Denis) :**
1. Cohabitation stable avec Whisper small sur 12 Go VRAM
2. Latence inference < 1.5s pour des reponses courtes (vocal)
3. Support tool calling / function calling fiable
4. Qualite en francais
5. Marge VRAM suffisante pour eviter l'OOM sous charge

**Note :** Le modele gpt-oss-20b reste disponible dans l'ecosysteme COG pour les usages non-vocaux (via LM Studio). MiyukiniVoice utilisera un modele dedie plus leger.

---

## 4. Plan de phases

### Phase 1 — Capture audio + Wake word (1 piece test)

> @id phase.voice.p1
> @do validate_audio_capture_and_wake_word_detection

**Objectif :** Valider la chaine hardware (micro -> carte son USB -> OS -> Rust) et la detection du wake word dans une seule piece.

**Livrables :**
- Crate `miyuvoicecapture` fonctionnel (enumeration devices, capture PCM, buffer circulaire)
- Crate `miyuwakeword` fonctionnel (detection "Hey Alicia" via rustpotter)
- Test d'integration : micro branche -> wake word detecte -> log console
- Documentation technique des deux toolkits
- Choix definitif : build natif Windows (WASAPI) vs WSL2 (ALSA + usbipd-win)

**Criteres de succes :**
- Wake word detecte a 2m de distance avec bruit ambiant modere
- Taux de faux positifs < 5% sur 1h de test
- Taux de detection > 90% en conditions normales
- Latence detection < 200ms apres fin du mot-cle

**Estimation temps :** 2-3 semaines

**Risques specifiques :**
- Audio USB sous WSL2 peut ne pas fonctionner -> mitigation : build natif Windows
- Qualite du micro MAX9814 insuffisante a distance -> mitigation : tests avec distances variables
- Rustpotter peut ne pas bien gerer "Hey Alicia" en francais -> mitigation : entrainement modele custom ou Porcupine

### Phase 2 — Pipeline STT -> LLM -> TTS (1 piece)

> @id phase.voice.p2
> @do implement_full_voice_pipeline_single_room

**Objectif :** Pipeline complet fonctionnel dans une piece : de la parole a la reponse vocale.

**Livrables :**
- Crate `miyustt` fonctionnel (whisper-rs, GPU, francais)
- Crate `miyutts` fonctionnel (Piper, voix francaise siwis)
- Crate `miyuvoicerouter` fonctionnel (playback vers la bonne carte son)
- Crate `miyukinivoice` (orchestration pipeline complet)
- Integration LLM via `miou-llm-bridge` existant ou `llama-cpp-rs` direct
- Test end-to-end : parole -> texte -> reponse -> voix

**Criteres de succes :**
- Latence totale < 3s (objectif < 2s) entre fin de parole et debut TTS
- STT francais avec WER (Word Error Rate) < 15% sur phrases courtes
- TTS intelligible et naturel en francais
- Pipeline stable sur 30 minutes d'utilisation continue

**Estimation temps :** 3-4 semaines

**Dependances :**
- Phase 1 validee
- Modeles Whisper et Piper telecharges et testes
- `miou-llm-bridge` operationnel (deja existant dans `apps/miou-llm-bridge/`)

### Phase 3 — Multi-pieces (4 pieces)

> @id phase.voice.p3
> @do scale_to_four_rooms_with_routing

**Objectif :** Etendre le pipeline a 4 pieces avec routage audio correct.

**Livrables :**
- Configuration multi-devices dans `miyuvoicecapture` (4 cartes son USB simultanees)
- Routage intelligent dans `miyuvoicerouter` (reponse -> piece d'origine)
- Gestion des conflits (2 pieces parlent en meme temps)
- Configuration persistee dans KindMother (noms de pieces, devices associes)
- Interface de configuration (CLI minimum, UI Dioxus en v2+)

**Criteres de succes :**
- 4 pieces fonctionnent simultanement sans interference
- Reponse toujours routee vers la piece d'origine
- Gestion gracieuse des conflits (file d'attente ou rejet avec feedback)
- Pas de degradation de latence avec 4 flux simultanes

**Estimation temps :** 2-3 semaines

**Dependances :**
- Phase 2 validee
- 4 ensembles hardware installes et cables
- Strategie de nommage/identification des devices USB definie

### Phase 4 — Integration Alicia + domotique

> @id phase.voice.p4
> @do integrate_alicia_persona_and_home_automation

**Objectif :** Transformer le pipeline technique en assistant personnel "Alicia" avec personnalite et debut de controle domotique.

**Livrables :**
- Persona Alicia configuree dans le LLM (system prompt, ton bienveillant, contexte familial)
- Historique conversationnel par piece (KindMother)
- Commandes domotiques basiques (si materiel compatible disponible)
- Integration MiyukiniWatch pour metriques d'usage
- Interface visuelle salon via Raspberry Pi (streaming Dioxus)

**Criteres de succes :**
- Alicia repond avec sa personnalite definie
- Contexte conversationnel maintenu sur une session
- Metriques d'usage remontees dans MiyukiniWatch

**Estimation temps :** 3-4 semaines

**Dependances :**
- Phase 3 validee
- Definition de la personnalite Alicia validee par l'utilisateur
- Inventaire du materiel domotique compatible (si applicable)

### Synthese planning

| Phase | Description | Duree estimee | Jalon |
|-------|-------------|---------------|-------|
| P1 | Capture + wake word (1 piece) | 2-3 sem | Wake word detecte en conditions reelles |
| P2 | Pipeline complet (1 piece) | 3-4 sem | Conversation vocale fonctionnelle |
| P3 | Multi-pieces (4 pieces) | 2-3 sem | 4 pieces operationnelles |
| P4 | Integration Alicia | 3-4 sem | Assistant personnalise fonctionnel |
| **Total** | | **10-14 sem** | |

---

## 5. Estimation des couts

### 5.1 Budget hardware

| Poste | Cout | Statut |
|-------|------|--------|
| Materiel audio (micros, HP, amplis, cartes son, cables) | ~89 EUR | Deja engage |
| Machines de calcul (A + B) | Deja possedees | Existant |
| Raspberry Pi 4 + ecran 15.6" | Deja possede | Existant |
| **Total hardware supplementaire** | **~89 EUR** | **Engage** |

### 5.2 Budget logiciel

| Poste | Cout |
|-------|------|
| Modeles IA (Whisper, Piper, Qwen3 4B) | 0 EUR (open source) |
| Crates Rust (cpal, whisper-rs, rustpotter, rodio) | 0 EUR (open source) |
| Porcupine (si rustpotter insuffisant) | 0 EUR (free tier) ou ~5 EUR/mois (pro) |
| **Total logiciel** | **0 EUR** (scenario optimiste) / **~60 EUR/an** (si Porcupine pro) |

### 5.3 Temps de developpement estime

| Phase | Optimiste | Pessimiste |
|-------|-----------|------------|
| Phase 1 | 2 semaines | 4 semaines |
| Phase 2 | 3 semaines | 5 semaines |
| Phase 3 | 2 semaines | 4 semaines |
| Phase 4 | 3 semaines | 5 semaines |
| **Total** | **10 semaines** | **18 semaines** |

### 5.4 Couts operationnels

| Poste | Estimation |
|-------|------------|
| Electricite supplementaire (GPU actif) | ~5-15 EUR/mois |
| Maintenance / remplacement composants | ~20 EUR/an |

---

## 6. Risques identifies

| # | Risque | Probabilite | Impact | Mitigation |
|---|--------|-------------|--------|------------|
| R1 | Audio USB inaccessible sous WSL2 | Elevee | Bloquant | Build natif Windows avec WASAPI ; usbipd-win en fallback |
| R2 | Latence totale > 2s | Moyenne | Important | Whisper small (pas medium), quantification agressive LLM, pipeline streaming |
| R3 | Qualite STT insuffisante en francais | Moyenne | Important | Tester Whisper small vs medium ; modele fine-tune francais si disponible |
| R4 | Bruit ambiant degrade la detection | Moyenne | Modere | Seuil de confiance ajustable, VAD pre-filtrage, placement optimal micros |
| R5 | VRAM insuffisante pour Whisper + LLM simultanes sur 12 Go | Moyenne | Bloquant | Modele LLM dedie vocal (7-14B Q4, choix par Denis) ; gpt-oss-20b exclu du pipeline vocal |
| R6 | Rustpotter ne reconnait pas "Hey Alicia" en FR | Moyenne | Important | Entrainer modele custom ; basculer vers Porcupine (FFI) |
| R7 | `unsafe_code = "forbid"` bloque les bindings FFI | Faible | Modere | Les crates `whisper-rs`, `llama-cpp-rs` encapsulent l'unsafe ; le forbid s'applique a NOTRE code, pas aux dependances |
| R8 | 4 cartes son USB simultanees instables | Faible | Important | Tester incrementalement ; utiliser un hub USB alimente ; identifier par serial |
| R9 | Piper voix francaise de qualite insuffisante | Faible | Modere | Tester plusieurs voix Piper FR ; ajuster parametres debit/pitch |
| R10 | Cat6 50m introduit du bruit analogique | Faible | Modere | Signal analogique apres la carte son (numerique sur Cat6) ; ou USB extenders |

**Note sur R10 :** Si le signal est analogique sur Cat6 (micro -> carte son loin), il y aura du bruit. Si la carte son est dans la piece et que seul l'USB transite via Cat6, le signal reste numerique. Ce point merite clarification (voir Questions ouvertes).

---

## 7. Besoins en ressources

### 7.1 Distribution agents par phase

| Phase | Agent | Responsabilite | Livrables |
|-------|-------|----------------|-----------|
| **P1** | Denis | Doc technique miyuvoicecapture + miyuwakeword, choix WSL2 vs natif | Specs techniques, checklist |
| **P1** | Francois | Implementation capture audio + wake word | Crates fonctionnels, tests |
| **P1** | George | Audit qualite detection, taux faux positifs | Rapport de test |
| **P2** | Denis | Doc technique miyustt + miyutts + miyuvoicerouter | Specs, benchmarks |
| **P2** | Francois | Implementation STT, TTS, routage, orchestration | Crates fonctionnels, tests |
| **P2** | George | Audit latence, qualite STT/TTS | Rapport de performance |
| **P3** | Denis | Doc multi-room, strategie devices USB | Architecture multi-pieces |
| **P3** | Francois | Implementation multi-devices, gestion conflits | Extension crates |
| **P3** | Lise | UI configuration des pieces (si UI Dioxus) | Composant UI |
| **P4** | Denis | Doc persona Alicia, integration MiyukiniWatch | Spec integration |
| **P4** | Francois | Backend persona, historique, metriques | Integration complete |
| **P4** | Lise | Interface visuelle salon (Raspberry Pi) | UI streaming |
| **Toutes** | Arianne | Qualite, archivage, anti-hallucination | Archives, memoire |
| **Toutes** | Maria | Suivi avancement, mise a jour rapport | Rapports de suivi |

### 7.2 Dependances Rust (crates externes)

| Crate | Version estimee | Usage | Licence |
|-------|----------------|-------|---------|
| `cpal` | 0.15+ | Capture et playback audio | Apache-2.0 |
| `rodio` | 0.19+ | Playback audio haut niveau | MIT/Apache-2.0 |
| `rustpotter` | 3.x | Detection wake word | MIT |
| `whisper-rs` | 0.12+ | Bindings whisper.cpp | MIT |
| `llama-cpp-rs` | 0.4+ | Bindings llama.cpp (optionnel si miou-llm-bridge) | MIT |
| `piper-rs` | (a verifier) | Bindings Piper TTS | A verifier |
| `serde` | 1.x | Serialisation | MIT/Apache-2.0 |
| `uuid` | 1.x | Identifiants | MIT/Apache-2.0 |
| `tokio` | 1.x | Runtime async | MIT |
| `tracing` | 0.1+ | Logs structures | MIT |

### 7.3 Infrastructure requise

| Ressource | Disponibilite |
|-----------|--------------|
| Machine A (RTX 5070) | Disponible |
| Machine B (RTX 3060 Ti) | Disponible |
| Reseau local Ethernet | Disponible |
| 4 ensembles micro/HP/ampli/carte son | A recevoir / a installer |
| PipeWire ou WASAPI | Installe avec l'OS |
| Modeles IA (Whisper, Piper, Qwen3 4B) | A telecharger (~10 Go total) |

---

## 8. Questions ouvertes — Decisions

Reponses recueillies le 2026-02-28.

### Q1 — Architecture audio physique — DECIDE

**Decision :** Signal analogique sur Cat6. Soudure directe du micro MAX9814 sur le Cat6, signal analogique sur paire orange jusqu'a la carte son USB cote serveur. Meme principe pour la sortie : signal analogique depuis la carte son sur paire verte, soude sur l'entree du PAM8403 dans la piece.

**Implications :**
- Le risque R10 (bruit analogique sur 50m de Cat6) devient **reel et a surveiller**.
- Les paires Cat6 utilisees en signal analogique doivent etre blindees (FTP).
- Mitigation : le MAX9814 possede un AGC 60dB qui compense partiellement. Le PAM8403 avec potentiometre (modele Heevhas confirme, ~5.52 EUR/2 pieces) permet d'ajuster le gain en sortie.
- Si la qualite est insuffisante en pratique, la migration vers "carte son dans la piece + USB extender" reste possible.

### Q2 — Build natif Windows vs WSL2 — DECIDE

**Decision :** Build natif Windows avec WASAPI. Le pipeline audio MiyukiniVoice tourne en natif Windows, meme si le reste de l'ecosysteme COG peut tourner sous WSL2.

**Implications :**
- Le risque R1 (audio USB sous WSL2) est **elimine**.
- Les crates `cpal` et `rodio` supportent nativement WASAPI.
- Le cross-compile Linux reste possible pour un futur portage mais n'est pas la cible v1.
- La communication avec les services COG sous WSL2 se fera via localhost (TCP/HTTP).

### Q3 — Integration LLM — DECIDE

**Decision :** Fork de `miou-llm-bridge` dedie a MiyukiniVoice.

**Implications :**
- Creer un fork/branche du bridge existant (`apps/miou-llm-bridge/`) adapte aux besoins vocaux.
- Le fork herite du systeme d'agents et skills existant (17 agents) mais sera optimise pour la latence vocale (streaming, timeouts agressifs).
- Le fork supportera le tool calling natif pour les fonctionnalites Alicia (agenda, rappels, intercom).
- Nom provisoire : `miou-voice-bridge` ou integration directe dans `miyukinivoice`.

### Q4 — Scope Alicia v1 — DECIDE

**Decision :** Les capacites vocales d'Alicia en v1 sont :

| Capacite | Description | Complexite |
|----------|-------------|------------|
| **Gestion agenda** | Lire, creer, modifier des evenements (KindMother ou integration calendrier local) | Elevee |
| **Rappels** | Creer et declencher des rappels vocaux a heure fixe ou intervalle | Moyenne |
| **Intercom** | Communication entre pieces (parler dans une piece, diffuser dans une autre) | Moyenne |
| **Questions generales** | Repondre aux questions via LLM local | Faible (LLM natif) |
| **Tool calling** | Capacite du LLM a appeler des fonctions (agenda, rappels, intercom, etc.) | Elevee |

**Impact sur le plan :**
- L'intercom est une fonctionnalite **nouvelle** non prevue initialement. Elle necessite que `miyuvoicerouter` puisse diffuser un flux audio capture dans une piece vers les HP d'une autre piece (ou toutes les pieces). Cela implique un mode "broadcast" ou "room-to-room" dans le routeur.
- Le tool calling necessite un schema de fonctions (function calling) dans le fork du bridge LLM. Le LLM doit etre capable de generer des appels structures (JSON) vers les outils internes.
- L'agenda et les rappels necessitent une integration KindMother pour la persistance des evenements et rappels.

### Q5 — Gestion multi-utilisateur — OUVERT

En v1, doit-on distinguer les utilisateurs par piece (Theresa dans sa chambre, etc.) ou toutes les pieces repondent de facon identique ?

### Q6 — Mode silencieux / horaires — OUVERT

Faut-il un mode silencieux (nuit, sieste) ou le systeme ecoute 24/7 ? Si oui, qui decide des horaires (config manuelle, Alicia automatique, MiyukiniWatch) ?

### Q7 — Piper-rs ou process Piper — DECIDE (Denis, 2026-02-28)

**Decision :** Utiliser le crate `piper-rs` (par thewh1teagle) en integration directe Rust.

**Recherche Denis :** Plusieurs crates Rust pour Piper TTS existent :

1. **`piper-rs` (thewh1teagle)** — version 0.1.x sur crates.io. 99.8% Rust, utilise ONNX Runtime via le crate `ort`. Compatible Windows/Linux/macOS. API simple : chargement de modele ONNX + synthese en PCM. Le plus mature et le plus utilise.
2. **`piper-tts-rust` (WrldEngine)** — implementation directe en Rust 2024, plus recente (janvier 2026), bindings de bas niveau.
3. **`piper-tts-rs-sys`** — bindings systeme de bas niveau.

**Note importante :** Le depot officiel `rhasspy/piper` a ete **archive le 6 octobre 2025**. Cependant, cela n'impacte pas le projet car :
- Les modeles vocaux (ONNX) restent disponibles sur HuggingFace (`rhasspy/piper-voices`)
- Le crate `piper-rs` est independant du depot rhasspy/piper
- L'inference ONNX est stable et ne necessite pas de mises a jour de Piper lui-meme
- Alternative de secours : **Kokoro TTS** (82M parametres, support francais, implementations Rust via `kokorox` et `kokoro-onnx`, activement developpe en 2025-2026)

**Dependance dans `miyutts/Cargo.toml` :**
```toml
piper-rs = "0.1"
```

**Implications :**
- Le crate `piper-rs` depend de `ort` (ONNX Runtime) qui telecharge automatiquement les binaires precompiles Microsoft.
- Sur Windows, une version ancienne de `onnxruntime.dll` dans System32 peut causer des conflits ; la mitigation est de copier les DLLs ONNX dans le dossier du binaire.
- L'inference TTS tourne sur CPU (ONNX CPU), pas de consommation VRAM GPU.
- `unsafe_code = "forbid"` dans notre crate `miyutts` ; le crate `piper-rs` et `ort` gerent leur propre FFI en interne.

### Q8 — Persistance des modeles IA — DECIDE (Denis, 2026-02-28)

**Decision :** Stockage dans un repertoire dedie hors du depot Git, avec metadonnees dans KindMother.

**Architecture de stockage :**

```
C:\MiyukiniModels\                      # Racine modeles (hors depot Git)
+-- voice\
|   +-- whisper\
|   |   +-- ggml-small.bin              # Whisper small (~500 Mo)
|   +-- piper\
|   |   +-- fr_FR-siwis-medium.onnx     # Voix Piper FR (~60 Mo)
|   |   +-- fr_FR-siwis-medium.onnx.json # Config voix
|   |   +-- fr_FR-tom-medium.onnx       # Voix alternative (optionnel)
|   +-- wakeword\
|       +-- hey_alicia.rpw              # Modele rustpotter reference (~1 Mo)
|       +-- hey_alicia.rpwm             # Modele rustpotter entraine (optionnel)
+-- llm\
    +-- (geres par LM Studio, pas par MiyukiniVoice)
```

**Justification :**

1. **Hors du depot Git** : les modeles (~10 Go total) ne doivent jamais etre commites. Le `.gitignore` ne suffit pas comme protection contre les erreurs.
2. **Repertoire systeme dedie** : `C:\MiyukiniModels\` est predictible, accessible, et ne depend pas du chemin du workspace.
3. **Separation LLM / Voice** : les modeles LLM sont geres par LM Studio (son propre repertoire). MiyukiniVoice ne stocke que les modeles voice (Whisper, Piper, wake word).
4. **KindMother (metadonnees)** : la base KindMother stockera :
   - Le chemin vers chaque modele (pas le modele lui-meme)
   - La version du modele
   - Le hash de verification d'integrite
   - La date de derniere utilisation
   - Le statut (actif, inactif, en cours de telechargement)
5. **Variable d'environnement** : `MIYUKINI_MODELS_DIR` permet de surcharger le chemin par defaut.

**Configuration dans le code :**
```rust
/// Chemin par defaut du repertoire modeles.
const DEFAULT_MODELS_DIR: &str = "C:\\MiyukiniModels";

/// Retourne le repertoire des modeles (variable d'env ou defaut).
pub fn models_dir() -> String {
    std::env::var("MIYUKINI_MODELS_DIR")
        .unwrap_or_else(|_| DEFAULT_MODELS_DIR.to_string())
}
```

**Voix francaises Piper disponibles :**

| Voix | Qualite | Genre | Taille estimee | Notes |
|------|---------|-------|----------------|-------|
| `siwis` | medium | feminin | ~60 Mo | Voix de reference, legere robotisation |
| `tom` | medium | masculin | ~60 Mo | Voix masculine FR |
| `upmc` (Jessica) | medium | feminin | ~60 Mo | Bug rapporte (#3411) |
| `gilles` | medium | masculin | ~60 Mo | Alternative |
| `mls` | low/medium | varies | ~20-60 Mo | Multi-locuteurs |

**Recommandation Denis :** Commencer avec `siwis` (medium) pour le prototypage. Tester `tom` comme alternative. Evaluer Kokoro TTS en parallele pour comparer la qualite.

---

## 9. Plan de distribution

### Workflow de lancement

```
1. Maria        -> Valide ce rapport avec l'utilisateur, recueille les reponses aux questions ouvertes
2. Maria        -> Met a jour le rapport avec les decisions prises
3. Denis        -> Recoit le rapport, produit la doc technique Phase 1
                   (specs miyuvoicecapture, miyuwakeword, choix technique WSL2 vs natif)
4. Francois     -> Recoit la doc de Denis, implemente Phase 1
5. George       -> Audit Phase 1 (taux detection, faux positifs, stabilite)
6. Arianne      -> Archive Phase 1, met a jour les skills/memoire
7. Denis        -> Doc technique Phase 2 (miyustt, miyutts, miyuvoicerouter, miyukinivoice)
8. Francois     -> Implementation Phase 2
9. George       -> Audit Phase 2 (latence, qualite)
10. [Cycle identique pour P3, P4]
```

### Prochaines actions immediates

| Action | Responsable | Priorite | Statut |
|--------|-------------|----------|--------|
| ~~Repondre aux questions Q1-Q4~~ | ~~Utilisateur~~ | ~~Bloquant~~ | Fait |
| Valider le plan de phases | Utilisateur + Maria | Bloquant | En cours |
| Tester un micro MAX9814 + carte son USB sur Windows (WASAPI) | Utilisateur (hardware) | Haute | A faire |
| Tester qualite signal analogique sur 50m Cat6 (soudure) | Utilisateur (hardware) | Haute | A faire |
| ~~Verifier existence et maturite de `piper-rs`~~ | ~~Denis~~ | ~~Haute~~ | Fait (Q7 decide) |
| ~~Verifier existence et maturite de `rustpotter` pour FR~~ | ~~Denis~~ | ~~Haute~~ | Fait (v3.0.2, valide) |
| ~~Choisir le modele LLM vocal~~ | ~~Denis~~ | ~~Haute~~ | Fait (Qwen3 4B principal, Qwen 2.5 7B fallback) |
| ~~Repondre Q7 (piper-rs) et Q8 (stockage modeles)~~ | ~~Denis~~ | ~~Haute~~ | Fait |
| ~~Produire la doc technique Phase 1~~ | ~~Denis~~ | ~~Haute~~ | Fait |
| Telecharger et tester Whisper small FR | Denis / Francois | Haute | A faire (Phase 2) |
| Preparer le fork de `miou-llm-bridge` pour le vocal | Denis / Francois | Haute | A faire (Phase 2) |
| Definir le schema de tool calling (agenda, rappels, intercom) | Denis | Moyenne | A faire (Phase 2) |
| Creer les crates squelettes dans le workspace | Francois | **Prochaine action** | A faire |
| Creer le repertoire `C:\MiyukiniModels\voice\` | Francois | Haute | A faire |
| Telecharger le modele Piper siwis-medium FR | Francois | Haute (Phase 2) | A faire |

---

## 10. Suivi d'avancement

_Section mise a jour au fur et a mesure de l'avancement du projet._

| Date | Phase | Evenement | Statut |
|------|-------|-----------|--------|
| 2026-02-28 | Pre-P1 | Rapport fondateur redige par Maria | Termine |
| 2026-02-28 | Pre-P1 | Decisions Q1-Q4 validees par l'utilisateur | Termine |
| 2026-02-28 | Pre-P1 | Q1: Signal analogique sur Cat6 + soudure PAM8403 | Decide |
| 2026-02-28 | Pre-P1 | Q2: Build natif Windows (WASAPI) | Decide |
| 2026-02-28 | Pre-P1 | Q3: Fork miou-llm-bridge dedie MiyukiniVoice | Decide |
| 2026-02-28 | Pre-P1 | Q4: Scope v1 = agenda, rappels, intercom, questions, tool calling | Decide |
| 2026-02-28 | Pre-P1 | Serveur unique Machine A (5070 12 Go), modele LLM a choisir par Denis | Decide |
| 2026-02-28 | Pre-P1 | Q7: piper-rs valide (thewh1teagle, v0.1.x, ONNX Runtime) | Decide (Denis) |
| 2026-02-28 | Pre-P1 | Q8: Stockage modeles dans C:\MiyukiniModels\ + metadonnees KindMother | Decide (Denis) |
| 2026-02-28 | Pre-P1 | Denis : choix modele LLM vocal = Qwen3 4B (principal) + Qwen 2.5 7B (fallback) | Decide |
| 2026-02-28 | Pre-P1 | Denis : doc technique Phase 1 produite (specs miyuvoicecapture, miyuwakeword) | Termine |
| 2026-02-28 | Pre-P1 | Denis : rustpotter 3.0.2 valide, 100% Rust, MIT, stable | Termine |
| 2026-02-28 | Pre-P1 | Denis : whisper-rs 0.15.1 valide, CUDA feature flag, build Windows documente | Termine |
| 2026-02-28 | Pre-P1 | Denis : Piper archive oct 2025 mais modeles + piper-rs operationnels. Kokoro TTS = alternative | Termine |
| - | Pre-P1 | Reponses Q5-Q6 (non bloquantes P1) | En attente utilisateur |
| - | P1 | Lancement Phase 1 | Pret a demarrer |

---

## Annexes

### A. Compatibilite Lois d'Autonomie

| Loi | Respect | Justification |
|-----|---------|---------------|
| LOI-1 | Oui | Tous les modeles IA sont locaux, aucune API cloud |
| LOI-2 | Oui | Le systeme fonctionne en isolement total |
| LOI-3 | Oui | Etat local souverain (KindMother) |
| LOI-4 | Oui | Pas de synchronisation temporelle externe |
| LOI-5 | Oui | Modeles dimensionnes pour le hardware disponible |
| LOI-6 | N/A | Pas de federation prevue en v1 |
| LOI-7 | Oui | Aucune modification des Cores |
| LOI-8 | N/A | Pas de migration prevue en v1 |

### B. References internes

| Document / Ressource | Chemin |
|----------------------|--------|
| Conventions projet | `CLAUDE.md` |
| Definition des agents | `docs/Les agents.md` |
| Skill architecture | `.cursor/skills/miyukini-architecture/SKILL.md` |
| Skill services | `.cursor/skills/miyukini-services/SKILL.md` |
| Skill Rust patterns | `.cursor/skills/miyukini-rust-patterns/SKILL.md` |
| Skill docs | `.cursor/skills/miyukini-docs/SKILL.md` |
| miou-llm-bridge | `apps/miou-llm-bridge/` |
| Workspace Cargo | `Cargo.toml` (racine) |

---

*Rapport redige par Maria, Chef de Projet Miyukini AI Studio — 2026-02-28*
*A destination de Denis (doc technique), Francois (implementation), Arianne (archivage)*
