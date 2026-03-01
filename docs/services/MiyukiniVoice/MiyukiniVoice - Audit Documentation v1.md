# MiyukiniVoice - Audit Documentation v1

> @id service.voice.miyukinivoice.audit.doc.v1
> @role audit
> @layer 7
> @do audit_documentation_quality_conformity_and_coherence
> @human Rapport d'audit de la documentation MiyukiniVoice (Document Fondateur, Specs Phase 1, Choix LLM) par George, Audit Expert Analyste

---

## Contexte

Cet audit porte sur les trois documents de specification du projet MiyukiniVoice, produits le 2026-02-28 :

1. **MiyukiniVoice - Document Fondateur.md** (Maria, mises a jour Denis)
2. **MiyukiniVoice - Specifications Techniques Phase 1.md** (Denis)
3. **MiyukiniVoice - Choix Modele LLM.md** (Denis)

L'audit est realise par George (Audit Expert Analyste) a destination d'Alicia et de l'utilisateur. Il verifie la conformite documentaire, architecturale, la coherence technique, la completude, la securite et la qualite globale, en s'appuyant sur les references suivantes :

- `CLAUDE.md` (conventions projet)
- `.cursor/skills/miyukini-architecture/SKILL.md` (Pyramide COG, Lois d'Autonomie)
- `.cursor/skills/miyukini-services/SKILL.md` (pattern services)
- `.cursor/skills/miyukini-rust-patterns/SKILL.md` (structure crates standard)
- `.cursor/skills/miyukini-docs/SKILL.md` (nomenclature documentation)
- `.cursor/skills/miyukini-mscm-mip/SKILL.md` (balisage MSCM)

## Portee / Scope

### Inclus
- Verification de la conformite documentaire (nomenclature, MSCM, structure)
- Verification de la conformite architecturale (Pyramide COG, Lois d'Autonomie, patterns crates)
- Analyse de la coherence technique inter-documents
- Evaluation de la completude pour l'implementation Phase 1
- Audit securite et vie privee
- Verification factuelle des versions de crates et modeles IA

### Exclus
- Audit du code source (pas encore implemente)
- Tests de performance reels (pas de code executable)
- Audit UX (pas d'interface utilisateur en Phase 1)

---

## 1. Conformite documentaire

### 1.1 Nomenclature des fichiers

**Reference :** `.cursor/skills/miyukini-docs/SKILL.md` -- format `<PREFIX> - <SUJET> <DETAIL>.<ext>`

| Document | Nomenclature | Conforme |
|----------|-------------|----------|
| `MiyukiniVoice - Document Fondateur.md` | PREFIX = MiyukiniVoice, SUJET = Document Fondateur | OUI |
| `MiyukiniVoice - Specifications Techniques Phase 1.md` | PREFIX = MiyukiniVoice, SUJET = Specifications Techniques, DETAIL = Phase 1 | OUI |
| `MiyukiniVoice - Choix Modele LLM.md` | PREFIX = MiyukiniVoice, SUJET = Choix Modele LLM | OUI |

Les trois fichiers sont dans `docs/services/MiyukiniVoice/` conformement a l'arborescence standard.

**Verdict : CONFORME**

### 1.2 Structure requise (H1, Contexte, Portee/Scope)

**Reference :** `.cursor/skills/miyukini-docs/SKILL.md` -- tout document DOIT avoir : titre H1, section Contexte, section Portee/Scope.

| Document | H1 | Contexte | Portee/Scope | Conforme |
|----------|-------|----------|-------------|----------|
| Document Fondateur | OUI (ligne 1) | OUI (section 2) | OUI (section "Portee / Scope") | OUI |
| Specs Phase 1 | OUI (ligne 1) | OUI (section 2) | OUI (Inclus/Exclus) | OUI |
| Choix LLM | OUI (ligne 1) | OUI (section 2) | OUI (Inclus/Exclus) | OUI |

**Verdict : CONFORME**

### 1.3 Annotations MSCM

**Reference :** `.cursor/skills/miyukini-mscm-mip/SKILL.md` -- champs obligatoires : `@id`, `@do`. Champs optionnels : `@role`, `@layer`, `@human`.

| Document | @id | @do | @role | @layer | @human | Conforme |
|----------|-----|-----|-------|--------|--------|----------|
| Document Fondateur | `service.voice.miyukinivoice` | `orchestrate_local_voice_assistant_pipeline` | `operator` | `7` | OUI | OUI |
| Specs Phase 1 | `service.voice.miyukinivoice.spec.p1` | `specify_audio_capture_and_wake_word_detection_phase1` | `technical_specification` | `7` | OUI | OUI |
| Choix LLM | `service.voice.miyukinivoice.llm_choice` | `select_optimal_local_llm_for_voice_assistant` | `technical_decision` | `7` | OUI | OUI |

De plus, les sections internes des specs Phase 1 portent leurs propres annotations MSCM (toolkit.voice.miyuvoicecapture, toolkit.voice.miyuwakeword, et sous-modules). Les annotations de phases dans le Document Fondateur (phase.voice.p1 a p4) sont egalement presentes.

**Point d'attention :** Le `@role` "operator" dans le Document Fondateur est semantiquement correct (MiyukiniVoice est un Operateur Strate 7). Le `@layer` a la valeur "7" au lieu d'un nom textuel (comme "domain" ou "infra") -- c'est acceptable car le skill MSCM autorise les deux formes, mais ce n'est pas homogene avec le code source ou `@layer` utilise des noms textuels.

**Verdict : CONFORME**

### 1.4 Coherence structurelle entre documents

Les trois documents suivent une progression logique :
1. Le Document Fondateur pose la vision, les decisions, l'architecture haut niveau
2. Les Specs Phase 1 detaillent l'implementation des deux premiers crates
3. Le Choix LLM isole une decision technique qui sera actionnee en Phase 2

Cette decomposition est pertinente et bien structuree. Le Document Fondateur renvoie vers les Specs Phase 1 et le Choix LLM dans les sections appropriees.

**Verdict : CONFORME**

---

## 2. Conformite architecturale

### 2.1 Respect de la Pyramide COG

**Reference :** `.cursor/skills/miyukini-architecture/SKILL.md`

| Element | Strate attendue | Strate documentee | Conforme |
|---------|----------------|-------------------|----------|
| `miyukinivoice` | 7 (Operateur) | 7 | OUI |
| `miyuvoicecapture` | 6 (Toolkit) | 6 | OUI |
| `miyuwakeword` | 6 (Toolkit) | 6 | OUI |
| `miyustt` | 6 (Toolkit) | 6 | OUI |
| `miyutts` | 6 (Toolkit) | 6 | OUI |
| `miyuvoicerouter` | 6 (Toolkit) | 6 | OUI |

Les regles architecturales sont respectees :
- "Les Cores decident, jamais n'executent" : les Cores (StrongFather, KindMother, etc.) sont documentes en tant qu'autorites de gouvernance, pas en tant qu'executants.
- "Les Outils font, mais ne decident jamais" : les toolkits Strate 6 executent des taches specifiques.
- "Les Operateurs sont gouvernes, jamais autonomes" : `miyukinivoice` orchestre sous mandat.

**Verdict : CONFORME**

### 2.2 Lois d'Autonomie

| Loi | Respect | Justification documentee | Verification |
|-----|---------|--------------------------|-------------|
| **LOI-1** (Pas de dependance externe critique) | OUI | Tous les modeles IA locaux, zero cloud | Correct. Aucune API cloud. |
| **LOI-2** (Isolement = etat normal) | OUI | Fonctionne en isolement total | Correct. |
| **LOI-3** (Etat local souverain) | OUI | KindMother pour la persistance | Correct. |
| **LOI-4** (Pas de temps global) | OUI | Pas de synchronisation temporelle | Correct. |
| **LOI-5** (Cout proportionnel au hardware) | OUI | Modeles dimensionnes pour RTX 5070 12 Go | Correct, bien documente. |
| **LOI-6** (Autonomie n'empeche pas federation) | N/A | Pas de federation en v1 | Correct. |
| **LOI-7** (Strate Cores immuable) | OUI | Aucune modification des Cores | Correct. |
| **LOI-8** (Migration = diplomatie) | N/A | Pas de migration en v1 | Correct. |

**Verdict : CONFORME**

### 2.3 Structure des crates

**Reference :** `.cursor/skills/miyukini-rust-patterns/SKILL.md` -- chaque toolkit doit avoir : lib.rs, admin_cell.rs, context.rs, errors.rs, modules metier.

| Crate | lib.rs | admin_cell.rs | context.rs | errors.rs | Modules metier | Conforme |
|-------|--------|---------------|------------|-----------|---------------|----------|
| miyuvoicecapture | OUI | OUI | OUI | OUI | capture.rs, devices.rs, vad.rs | OUI |
| miyuwakeword | OUI | OUI | OUI | OUI | detector.rs, models.rs | OUI |

Les Cargo.toml documentes dans les specs Phase 1 respectent le pattern standard :
- Metadata heritee du workspace
- `miyukini-kernel` en dependance
- `serde` avec features `["derive"]`
- `unsafe_code = "forbid"` dans `[lints.rust]`
- Clippy pedantic active

**Verdict : CONFORME**

### 2.4 Separation Toolkit (Strate 6) / Operateur (Strate 7)

Le Document Fondateur presente une structure claire :
- L'Operateur `miyukinivoice` (Strate 7) orchestre le pipeline avec un dossier `data/`, `services/`, `domain/`
- Les Toolkits (Strate 6) executent les taches : capture, wake word, STT, TTS, routage

La structure de l'Operateur suit le pattern services (`.cursor/skills/miyukini-services/SKILL.md`) avec `data/mod.rs`, `data/types.rs`, `data/kindmother_db.rs`, `services/`, `domain/`.

**Verdict : CONFORME**

### 2.5 Cores impliques

Les 5 Cores identifies (StrongFather, KindMother, CaringNanny, MasterButler, WorrySentinel) sont documentes avec des roles clairs et pertinents. On note l'absence de 3 Cores sur 8 dans la documentation :

- **BorderGuard** : non mentionne. Pourrait etre pertinent pour le controle d'acces aux peripheriques audio.
- **EverBuddy** : non mentionne. Pourrait etre pertinent pour la gestion de la persistance des sessions.
- **TAMR** : non mentionne.

Cependant, ces absences ne constituent pas une anomalie critique. Les 5 Cores documentes couvrent les besoins identifies de MiyukiniVoice v1. Les Cores non impliques n'ont simplement pas de role dans ce service, ce qui est acceptable.

**Verdict : CONFORME**

---

## 3. Coherence technique

### 3.1 Coherence inter-documents

| Element | Document Fondateur | Specs Phase 1 | Choix LLM | Coherent |
|---------|-------------------|---------------|-----------|----------|
| Build natif Windows WASAPI | Q2 decide | Confirme, detaille | N/A | OUI |
| Signal analogique Cat6 | Q1 decide | Confirme | N/A | OUI |
| Machine A (RTX 5070 12 Go) | Section 2.3 | Confirme | Confirme, base des calculs | OUI |
| `unsafe_code = "forbid"` | Section 2.2 | Present dans Cargo.toml | N/A | OUI |
| rustpotter 3.x | Section 3.5 | Rustpotter 3.0, detaille | N/A | OUI |
| whisper-rs | Section 3.5 : "0.12+" | Section G : "0.15.1 confirmee" | N/A | INCOHERENCE MINEURE (voir A-01) |
| piper-rs | Q7 : "0.1.x" | Section G : confirme | N/A | OUI |
| cpal | Section 3.5 : "0.15+" | Cargo.toml : `cpal = "0.15"` | N/A | INCOHERENCE (voir A-02) |
| VRAM Whisper small | "~2 Go" partout | Confirme | "~2 Go" | OUI |
| VRAM LLM Qwen3 4B | "~3 Go" | N/A | "~3 Go" | OUI |
| Latence cible | "< 2s" (total) | "< 200ms" (wake word seul) | "< 1.5s" (LLM seul) | OUI (coherent car cible differente par composant) |

### 3.2 Version cpal : incoherence factuelle

**Anomalie A-02 :** Le Document Fondateur et les Specs Phase 1 specifient `cpal 0.15` ou `cpal 0.15+`. Or, d'apres crates.io, la derniere version de cpal est **0.17.3** (mise a jour recente). La version 0.15 existe et fonctionnerait, mais specifier `cpal = "0.15"` dans Cargo.toml fixerait la version a la branche 0.15.x et priverait le projet des ameliorations et corrections des versions 0.16 et 0.17.

### 3.3 Version whisper-rs : incoherence documentaire

**Anomalie A-01 :** Le Document Fondateur (section 3.5, tableau Stack logicielle) mentionne `whisper-rs 0.12+`. La section G des Specs Phase 1 mentionne `whisper-rs 0.15.1`. D'apres crates.io, la version actuelle est bien **0.15.1**. Le "0.12+" du Document Fondateur est donc obsolete et n'a pas ete mis a jour apres la recherche de Denis.

### 3.4 Version piper-rs

Le Document Fondateur mentionne `piper-rs 0.1.x`. D'apres crates.io, la version actuelle est **0.1.9**. Specifier `piper-rs = "0.1"` dans le Cargo.toml (comme propose en Q7) resoudra correctement vers 0.1.9. Coherent.

### 3.5 Version ringbuf

Les Specs Phase 1 specifient `ringbuf = "0.4"`. D'apres crates.io, la derniere version est **0.4.8**. La specification est correcte et resoudra vers la derniere 0.4.x.

### 3.6 Choix LLM : Qwen3 4B

Le Choix LLM recommande **Qwen3 4B Instruct (Q4_K_M)** avec fallback sur **Qwen 2.5 7B Instruct (Q4_K_M)**.

Verification factuelle :
- Qwen3 4B existe bien, sorti le 29 avril 2025 (Apache 2.0).
- Le modele Qwen3-4B-Instruct-2507 existe sur HuggingFace (mise a jour juillet 2025).
- Le format GGUF est disponible via unsloth et bartowski (sources mentionnees).
- Les estimations de VRAM (~3 Go pour 4B Q4_K_M) sont realistes.
- Le tool calling Qwen3 est effectivement reconnu comme excellent dans la communaute.

Le Document Fondateur mentionne encore "Mistral/Llama" dans le Resume executif (section 1) et dans Portee/Scope ("Reponse LLM locale (Mistral/Llama)"), alors que la decision finale est Qwen3 4B. C'est une incoherence residuelle due a la mise a jour partielle du document.

**Anomalie A-03 :** Les mentions "Mistral/Llama" dans le Resume executif et Portee/Scope du Document Fondateur n'ont pas ete mises a jour apres la decision Qwen3 4B.

### 3.7 Estimations de latence

| Composant | Doc Fondateur | Specs Phase 1 | Choix LLM | Realiste |
|-----------|--------------|---------------|-----------|----------|
| Wake word detection | "< 100ms" | "< 200ms" | N/A | OUI -- rustpotter < 100ms typique |
| Capture audio | "< 10ms" | "10-30ms WASAPI shared" | N/A | OUI -- coherent |
| STT (Whisper small) | "500-1500ms" | N/A (Phase 2) | N/A | OUI -- realiste sur RTX 5070 |
| LLM (Qwen3 4B) | "500-2000ms" | N/A (Phase 2) | "~450ms (30 tokens)" | INCOHERENCE MINEURE (voir A-04) |
| TTS (Piper) | "200-500ms" | N/A (Phase 2) | N/A | OUI -- realiste CPU ONNX |
| Total pipeline | "< 2s (objectif), < 3s" | N/A | N/A | OUI -- somme coherente |

**Anomalie A-04 :** Le Document Fondateur estime la latence LLM a "500-2000ms", tandis que le Choix LLM estime Qwen3 4B a "~450ms". L'estimation du Choix LLM est plus precise car basee sur un modele specifique (4B au lieu de 7-14B). Le Document Fondateur n'a pas ete mis a jour avec cette estimation plus favorable. Ce n'est pas une contradiction (le choix final est plus rapide que les estimations initiales), mais le Document Fondateur devrait refleter la decision prise.

### 3.8 Archive rhasspy/piper

Le Document Fondateur (Q7) et les Specs Phase 1 (section G) mentionnent correctement que rhasspy/piper a ete archive le 6 octobre 2025. Cela est factuellement verifie. L'analyse des consequences est correcte : les modeles restent sur HuggingFace, et piper-rs est independant du depot archive. Le projet se poursuit sous OHF-Voice/piper1-gpl.

**Verdict : GLOBALEMENT COHERENT avec 4 incoherences mineures identifiees**

---

## 4. Completude

### 4.1 Suffisance pour l'implementation Phase 1

L'objectif principal des specs Phase 1 est de fournir a Francois toutes les informations necessaires pour implementer `miyuvoicecapture` et `miyuwakeword`.

| Element | Present | Suffisant | Notes |
|---------|---------|-----------|-------|
| Architecture crate | OUI | OUI | Structure fichiers detaillee |
| API publique complete | OUI | OUI | Tous les types, traits, fonctions documentes avec signatures Rust |
| Cargo.toml complet | OUI | OUI | Dependances avec versions, lints, metadata |
| Algorithmes (VAD, buffer) | OUI | OUI | Algorithme VAD detaille pas a pas, buffer circulaire dimensionne |
| Strategie de test | OUI | OUI | Tests unitaires, integration, mock, fixtures WAV |
| Checklist de validation | OUI | OUI | 19 etapes detaillees avec criteres pass/fail |
| Metriques a mesurer | OUI | OUI | Tableau complet |
| Distribution taches | OUI | OUI | 12 taches Francois, 6 taches Denis, estimations temps |
| Securite/conformite | OUI | OUI | RGPD, invariants, securite code |
| Notes Phase 2 | OUI | OUI | Preparation de la suite |

**Point fort :** Les specifications sont exceptionnellement detaillees. Le code Rust propose (signatures, types, enums) est directement implementable. La procedure d'entrainement du modele wake word est complete (echantillons, commandes CLI, structure dossiers).

### 4.2 Criteres de succes mesurables

| Critere | Mesurable | Testable | Notes |
|---------|-----------|----------|-------|
| Detection a 2m | OUI | OUI | Test en conditions reelles |
| Faux positifs < 5%/h | OUI | OUI | Compteur sur 1h |
| Detection > 90% | OUI | OUI | 20 tests, >= 18 detections |
| Latence < 200ms | OUI | OUI | Timestamp Instant |
| Build sans erreur | OUI | OUI | `cargo build` |
| Tests passants | OUI | OUI | `cargo test` |
| Clippy 0 warnings | OUI | OUI | `cargo clippy -- -D warnings` |

**Verdict : COMPLET -- Tous les criteres sont mesurables et testables.**

### 4.3 Risques identifies

Le Document Fondateur identifie 10 risques (R1-R10) avec probabilite, impact et mitigation. Les Specs Phase 1 detaillent les risques specifiques a rustpotter.

| Risque non couvert | Gravite |
|--------------------|---------|
| Latence USB hub avec 4 cartes son | Faible -- mentionne dans R8 mais sans detail sur le hub specifique |
| Windows Update desactivant les drivers audio USB | Faible -- risque generique Windows |
| Conflit entre cpal et d'autres applications audio | Faible -- le mode WASAPI shared gere cela |

Les risques sont globalement bien couverts. Aucun risque critique manquant identifie.

### 4.4 Dependances listees avec versions

| Crate | Version specifiee | Version actuelle (crates.io) | Ecart |
|-------|-------------------|------------------------------|-------|
| cpal | 0.15 | 0.17.3 | 2 versions mineures de retard |
| ringbuf | 0.4 | 0.4.8 | Correct (resolue vers 0.4.8) |
| rustpotter | 3.0 | 3.0.2 | Correct (resolue vers 3.0.2) |
| tracing | 0.1 | 0.1.x (stable) | Correct |
| serde | 1.0 | 1.x (stable) | Correct |
| hound (dev) | 3.5 | 3.5.x | Correct |
| whisper-rs | 0.15 (Phase 2) | 0.15.1 | Correct |
| piper-rs | 0.1 (Phase 2) | 0.1.9 | Correct |

**Verdict : COMPLET avec une recommandation de mise a jour pour cpal (voir Recommandations)**

---

## 5. Erreurs et incoherences detectees

### A-01 : Version whisper-rs obsolete dans le Document Fondateur

- **Description :** Le tableau "Stack logicielle detaillee" (section 3.5) du Document Fondateur mentionne `whisper-rs 0.12+`. La recherche de Denis a identifie la version 0.15.1 (documentee en section G des Specs Phase 1 et dans Q7/Q8). Le tableau n'a pas ete mis a jour.
- **Gravite :** Mineur
- **Localisation :** `docs/services/MiyukiniVoice/MiyukiniVoice - Document Fondateur.md`, section 3.5 tableau Stack logicielle, colonne "Crate Rust" ligne STT
- **Recommandation :** Mettre a jour le tableau Stack logicielle : `whisper-rs 0.12+` -> `whisper-rs 0.15+`

### A-02 : Version cpal sous-specifiee

- **Description :** Les documents specifient `cpal = "0.15"` alors que la version actuelle est 0.17.3. La version 0.15 est fonctionnelle mais n'inclut pas les corrections et ameliorations de 0.16 et 0.17. Specifier `cpal = "0.15"` dans Cargo.toml va resoudre vers 0.15.x et exclure les versions majeures 0.16+ (car SemVer pre-1.0 traite le changement de mineur comme breaking).
- **Gravite :** Important
- **Localisation :** `docs/services/MiyukiniVoice/MiyukiniVoice - Specifications Techniques Phase 1.md`, section A.3 (Cargo.toml miyuvoicecapture) et `docs/services/MiyukiniVoice/MiyukiniVoice - Document Fondateur.md`, section 3.5
- **Recommandation :** Tester avec `cpal = "0.15"` en Phase 1 pour validation initiale, mais prevoir une montee de version vers `cpal = "0.17"` si aucune incompatibilite. Documenter cette intention dans les specs. Note : Le changement de 0.15 a 0.17 peut impliquer des changements d'API ; une verification est necessaire avant de mettre a jour la specification.

### A-03 : Mentions "Mistral/Llama" non mises a jour dans le Document Fondateur

- **Description :** Le Resume executif (section 1) mentionne "genere une reponse via LLM local (Mistral/Llama)". La section Portee/Scope mentionne "Reponse LLM locale (Mistral/Llama)". Le tableau de suivi (section 10) confirme pourtant la decision "Qwen3 4B principal + Qwen 2.5 7B fallback". Les sections hautes du document n'ont pas ete rebalayees apres cette decision.
- **Gravite :** Mineur
- **Localisation :** `docs/services/MiyukiniVoice/MiyukiniVoice - Document Fondateur.md`, sections 1 et Portee/Scope
- **Recommandation :** Remplacer "Mistral/Llama" par "Qwen3 4B (Qwen 2.5 7B en fallback)" dans le Resume executif et Portee/Scope. Mettre a jour egalement le tableau "Modeles IA" de la section 5.2 Budget logiciel qui mentionne encore "Whisper, Piper, Mistral/Llama".

### A-04 : Latence LLM non mise a jour dans le Document Fondateur

- **Description :** Le Document Fondateur estime la latence LLM a "500-2000ms" (section 3.5, base sur des modeles 7-14B). Le Choix LLM estime Qwen3 4B a "~450ms". Le Document Fondateur n'a pas ete mis a jour avec cette estimation plus favorable. Ce n'est pas une contradiction (la fourchette initiale etait plus large car le modele n'etait pas encore choisi), mais c'est une information perimee.
- **Gravite :** Mineur
- **Localisation :** `docs/services/MiyukiniVoice/MiyukiniVoice - Document Fondateur.md`, section 3.5 tableau, ligne LLM
- **Recommandation :** Mettre a jour la ligne LLM du tableau Stack logicielle avec l'estimation basee sur Qwen3 4B (~450ms) ou noter "voir MiyukiniVoice - Choix Modele LLM.md pour les estimations finales".

### A-05 : Pipeline audio mentionne "Machine A / B" apres decision serveur unique

- **Description :** Le schema ASCII du pipeline audio (section 3.2) indique "[Machine A / B]" alors que la decision est prise que Machine B n'est PAS utilisee pour MiyukiniVoice (sections 2.3 et 3.6).
- **Gravite :** Mineur
- **Localisation :** `docs/services/MiyukiniVoice/MiyukiniVoice - Document Fondateur.md`, section 3.2 (schema pipeline)
- **Recommandation :** Remplacer "[Machine A / B]" par "[Machine A]" dans le schema.

### A-06 : Mention "Porcupine FFI" dans la structure crate du Document Fondateur

- **Description :** Le Document Fondateur (section 3.4, structure crates) mentionne dans `miyuwakeword/src/detector.rs` le commentaire "Rustpotter ou Porcupine FFI". La decision est de commencer par rustpotter, avec Porcupine en fallback. Les Specs Phase 1 clarifient cela proprement avec un trait `WakeWordDetector` permettant le swap. Le commentaire du Document Fondateur est ambigu et pourrait etre clarifie.
- **Gravite :** Suggestion
- **Localisation :** `docs/services/MiyukiniVoice/MiyukiniVoice - Document Fondateur.md`, section 3.4
- **Recommandation :** Changer le commentaire en "Rustpotter (principal) via trait WakeWordDetector (Porcupine en fallback)".

### A-07 : Section `@layer` utilise des valeurs numeriques dans les docs mais des noms textuels dans les skills MSCM

- **Description :** Les annotations MSCM des documents MiyukiniVoice utilisent `@layer 7` et `@layer 6` (valeurs numeriques referant aux strates de la Pyramide COG). Le skill MSCM montre des exemples avec des valeurs textuelles (`@layer domain`, `@layer infra`). Les deux formes sont techniquement valides mais l'heterogeneite pourrait poser probleme lors de l'indexation MIP.
- **Gravite :** Suggestion
- **Localisation :** Tous les documents MiyukiniVoice
- **Recommandation :** Choisir une convention unique pour `@layer` dans les documents : soit des numeros de strate (coherent avec la Pyramide COG), soit des noms de couche fonctionnelle (coherent avec les exemples MSCM). Documenter la convention choisie.

### A-08 : Manque de mention explicite du crate `blake3` pour la generation de stable_id

- **Description :** Les Specs Phase 1 (section A.2.2) definissent `generate_stable_id` comme utilisant "hash blake3 tronque 16 chars". Cependant, le crate `blake3` n'est pas liste dans les dependances du Cargo.toml de `miyuvoicecapture` (section A.3).
- **Gravite :** Important
- **Localisation :** `docs/services/MiyukiniVoice/MiyukiniVoice - Specifications Techniques Phase 1.md`, section A.3 (Cargo.toml) vs section A.2.2 (algorithme stable_id)
- **Recommandation :** Ajouter `blake3 = "1"` aux dependances du Cargo.toml de miyuvoicecapture, ou bien reviser l'algorithme de stable_id pour utiliser un hash disponible dans les dependances existantes (par exemple, un hash simple avec les APIs standard `std::hash` ou un checksum leger). Blake3 est un choix valide mais doit etre declare explicitement.

---

## 6. Securite et vie privee

### 6.1 Contraintes de vie privee

| Exigence | Documentee | Implementation documentee | Correcte |
|----------|-----------|--------------------------|----------|
| Pas d'enregistrement continu | OUI (Doc Fondateur section 2.2, Specs Phase 1 section F.1) | Buffer circulaire 2s, ecrasement automatique | OUI |
| Traitement post-wake-word uniquement | OUI (Doc Fondateur section 2.1) | En Phase 1, rien n'est traite apres le wake word (log seulement) | OUI |
| Zero cloud / zero transmission reseau | OUI (Doc Fondateur LOI-1) | Aucune API cloud, aucune dependance reseau | OUI |
| Consentement foyer | OUI (Specs Phase 1 section F.1) | Usage domestique, membres du foyer informes | OUI |
| Aucun fichier audio sur disque en production | OUI (Specs Phase 1 section F.1) | Explicit : "Aucun fichier audio n'est ecrit sur disque en production" | OUI |

### 6.2 WorrySentinel

WorrySentinel est correctement identifie comme Core responsable de la surveillance securite et vie privee (Document Fondateur, section 3.3). Son role est decrit : "Surveille la securite (pas d'enregistrement continu, vie privee), etat de confiance". C'est correct et suffisant pour la phase documentaire.

### 6.3 `unsafe_code = "forbid"`

| Document | Mentionne | Cargo.toml specifie | Correct |
|----------|-----------|---------------------|---------|
| Document Fondateur | OUI (section 2.2) | N/A (doc haut niveau) | OUI |
| Specs Phase 1 | OUI (sections A.3, B.7, F.2) | OUI (present dans les 2 Cargo.toml) | OUI |
| Choix LLM | N/A | N/A | N/A |

L'explication que les dependances FFI (whisper-rs, llama-cpp-rs, piper-rs) gerent leur propre unsafe en interne est correcte. Le `forbid` s'applique au code MiyukiniVoice uniquement.

### 6.4 Risques de securite non documentes

| Risque | Gravite | Notes |
|--------|---------|-------|
| Acces physique aux micros (ecoute par un intrus local) | Faible | Le systeme domestique presuppose un environnement de confiance. Acceptable pour v1. |
| Injection de commande via audio adversarial | Tres faible | Attaque theorique (son inaudible declenchant le wake word). Tres peu probable en contexte domestique. Documentable pour v2+. |
| Logs de sessions vocales en clair dans KindMother | Faible | A verifier en Phase 4 quand la persistance des sessions sera implementee. Pas un risque Phase 1. |

**Verdict : CONFORME -- La securite et la vie privee sont correctement documentees pour la Phase 1.**

---

## 7. Recommandations et optimisations

### R-01 : Mettre a jour les references "Mistral/Llama" dans le Document Fondateur

- **Impact :** Eleve (coherence documentaire)
- **Effort :** Faible (5 minutes)
- **Description :** Trois occurrences de "Mistral/Llama" dans le Document Fondateur doivent etre mises a jour avec "Qwen3 4B / Qwen 2.5 7B" : Resume executif (section 1), Portee/Scope, et Budget logiciel (section 5.2).

### R-02 : Mettre a jour la version whisper-rs dans le Document Fondateur

- **Impact :** Faible (documentation correcte dans les Specs Phase 1)
- **Effort :** Faible (1 minute)
- **Description :** Changer `whisper-rs 0.12+` en `whisper-rs 0.15+` dans le tableau Stack logicielle.

### R-03 : Ajouter blake3 aux dependances de miyuvoicecapture

- **Impact :** Eleve (bloquant pour l'implementation de generate_stable_id)
- **Effort :** Faible (1 ligne dans le Cargo.toml)
- **Description :** Ajouter `blake3 = "1"` dans les `[dependencies]` du Cargo.toml de miyuvoicecapture, ou documenter un algorithme alternatif.

### R-04 : Evaluer la montee de version cpal 0.15 -> 0.17

- **Impact :** Moyen (beneficier des corrections et ameliorations)
- **Effort :** Moyen (verification de compatibilite API necessaire)
- **Description :** cpal 0.17 est la version actuelle. Si l'API est compatible, migrer directement. Sinon, documenter la raison de rester sur 0.15.

### R-05 : Corriger le schema pipeline "Machine A / B"

- **Impact :** Faible (clarte documentaire)
- **Effort :** Faible (1 minute)
- **Description :** Remplacer "[Machine A / B]" par "[Machine A]" dans le schema ASCII section 3.2.

### R-06 : Ajouter un glossaire des termes techniques

- **Impact :** Moyen (accessibilite pour les membres de l'equipe non-audio)
- **Effort :** Moyen (30 minutes)
- **Description :** Creer un glossaire des termes audio (PCM, WASAPI, sample rate, RMS, VAD, SPSC, AGC, SNR) dans le Document Fondateur ou en annexe. Cela facilitera la lecture par les non-specialistes.

### R-07 : Documenter la strategie de mise a jour des modeles IA

- **Impact :** Moyen (maintenabilite long terme)
- **Effort :** Faible (section additionnelle)
- **Description :** Le document Q8 definit le stockage des modeles mais ne decrit pas la procedure de mise a jour (nouveau Whisper, nouvelle voix Piper, nouveau LLM). Ajouter une section sur le cycle de vie des modeles (telechargement, validation hash, swap en production, rollback).

---

## Tableau de synthese

### Anomalies par gravite

| Gravite | Nombre | References |
|---------|--------|------------|
| Critique | 0 | -- |
| Important | 2 | A-02, A-08 |
| Mineur | 4 | A-01, A-03, A-04, A-05 |
| Suggestion | 2 | A-06, A-07 |
| **Total** | **8** | |

### Recommandations par impact/effort

| # | Impact | Effort | Description |
|---|--------|--------|-------------|
| R-01 | Eleve | Faible | Mettre a jour "Mistral/Llama" -> "Qwen3 4B / Qwen 2.5 7B" |
| R-03 | Eleve | Faible | Ajouter blake3 aux dependances miyuvoicecapture |
| R-02 | Faible | Faible | Corriger whisper-rs 0.12+ -> 0.15+ |
| R-05 | Faible | Faible | Corriger "Machine A / B" -> "Machine A" |
| R-04 | Moyen | Moyen | Evaluer montee cpal 0.17 |
| R-06 | Moyen | Moyen | Ajouter glossaire |
| R-07 | Moyen | Faible | Documenter mise a jour modeles IA |

---

## Note globale de conformite

### Score detaille

| Axe | Poids | Score | Pondere |
|-----|-------|-------|---------|
| Conformite documentaire (nomenclature, MSCM, structure) | 15% | 95/100 | 14.25 |
| Conformite architecturale (Pyramide, Lois, patterns) | 20% | 98/100 | 19.60 |
| Coherence technique inter-documents | 20% | 85/100 | 17.00 |
| Completude pour implementation | 25% | 95/100 | 23.75 |
| Securite et vie privee | 10% | 97/100 | 9.70 |
| Qualite redactionnelle et clarte | 10% | 93/100 | 9.30 |
| **Total** | **100%** | | **93.60/100** |

### Justification des scores

- **Conformite documentaire (95/100) :** Tous les standards sont respectes. Point retire pour l'heterogeneite des `@layer` (numerique vs textuel).
- **Conformite architecturale (98/100) :** Excellente adherence a la Pyramide COG, aux Lois d'Autonomie, et aux patterns Rust standard. Quasi parfait.
- **Coherence technique (85/100) :** 4 incoherences mineures + 2 importantes (cpal version, blake3 manquant). Le Document Fondateur n'a pas ete completement rafraichi apres les decisions de Denis. Cela reste des points facilement corrigeables.
- **Completude (95/100) :** Les specifications Phase 1 sont exceptionnellement detaillees avec du code Rust directement implementable. Il manque uniquement blake3 dans les dependances et un glossaire.
- **Securite (97/100) :** Tres bien documentee. WorrySentinel implique, `unsafe_code = "forbid"`, vie privee traitee. Seules des suggestions mineures sur le long terme.
- **Qualite redactionnelle (93/100) :** Documents bien structures, clairs, progressifs. Les tableaux sont efficaces. L'information est bien organisee par audience (Francois pour les specs, Denis pour la supervision, George pour l'audit).

---

## Verdict

### APPROUVE AVEC RESERVES

La documentation MiyukiniVoice est de **tres bonne qualite** (93.6/100). Les trois documents forment un ensemble coherent, complet et conforme aux standards Miyukini COG. L'architecture est solide, les decisions sont justifiees, et les specifications Phase 1 sont suffisamment detaillees pour permettre a Francois de demarrer l'implementation.

**Reserves (a traiter avant le lancement de l'implementation) :**

1. **A-08 (Important) :** Ajouter `blake3` aux dependances de miyuvoicecapture ou reviser l'algorithme de stable_id. Sans cette correction, Francois devra prendre une decision technique non specifiee.

2. **A-02 (Important) :** Clarifier la version de cpal a utiliser (0.15 vs 0.17). Documenter le choix et sa justification.

**Corrections recommandees (non bloquantes mais a faire rapidement) :**

3. **A-01, A-03, A-04, A-05 :** Mettre a jour le Document Fondateur avec les decisions finales (Qwen3, whisper-rs 0.15, Machine A seule, latence LLM).

**Une fois les 2 reserves traitees, le statut passera a APPROUVE.**

---

*Rapport redige par George, Audit Expert Analyste Miyukini AI Studio -- 2026-02-28*
*A destination de : Alicia, Utilisateur, Arianne (archivage)*
*Ref : MiyukiniVoice - Document Fondateur, Specifications Techniques Phase 1, Choix Modele LLM (2026-02-28)*

---

## Sources de verification factuelle

- [rustpotter sur crates.io](https://crates.io/crates/rustpotter) -- version 3.0.2 confirmee
- [whisper-rs sur crates.io](https://crates.io/crates/whisper-rs) -- version 0.15.1 confirmee
- [piper-rs sur crates.io](https://crates.io/crates/piper-rs) -- version 0.1.9, par thewh1teagle
- [cpal sur crates.io](https://crates.io/crates/cpal) -- version 0.17.3 confirmee
- [ringbuf sur crates.io](https://crates.io/crates/ringbuf) -- version 0.4.8 confirmee
- [rhasspy/piper sur GitHub](https://github.com/rhasspy/piper) -- archive le 6 octobre 2025, confirme
- [Qwen3 4B sur HuggingFace](https://huggingface.co/Qwen/Qwen3-4B) -- existe, sorti avril 2025
- [Qwen3-4B-Instruct-2507 sur HuggingFace](https://huggingface.co/Qwen/Qwen3-4B-Instruct-2507) -- existe, mise a jour juillet 2025
