# Modèles IA pour petites configurations — base de connaissance datée

> **Gelé v1.0 — 2026-10-24.** Toute modification passe par un amendement : [règles du gel](GEL-DOC-CONCEPTUELLE-v1.md).

<!-- @id arch.ia.modeles-petites-configs -->
<!-- @role model-selection -->
<!-- @layer 8 -->
<!-- @human Choix des modèles locaux de MAIA par configuration matérielle, avec date de coupure des connaissances et licence vérifiées -->
<!-- @do select_local_models_with_dated_knowledge -->

- **Statut** : recherche du 2026-10-24 (sources primaires : fiches Hugging Face, pages éditeurs)
- **Base** : [mémoire MAIA](MEMOIRE-MAIA.md) · [IA fédérée](IA-FEDEREE.md)
- **Légende** : **V** vérifié sur la fiche · **NP** non publiée · **INC** incertain
- Tailles GGUF : dépôts unsloth (Q4_K_M) ; RAM/VRAM = estimation (GGUF + 1 à 2 Go selon le contexte)

## 1. Principe de la base de connaissance datée

<!-- @id arch.ia.modeles.connaissance-datee -->
<!-- @role dated-knowledge -->
<!-- @do date_model_knowledge -->

- Chaque modèle a une **date de coupure** déclarée dans son manifeste (canal officiel)
- MAIA connaît sa date de coupure (ligne N0 de la mémoire) et la **dit** quand une question porte sur un fait récent
- Après la coupure : la réponse s'appuie sur la **mémoire** et les **Fiches** fédérées, sourcées et datées
- Modèle à coupure **NP** :
  - coupure **mesurée** par un jeu de questions datées (événements par trimestre)
  - déclarée « estimée » avec la méthode et la date de mesure
- Manifeste de modèle : nom, éditeur, version, empreinte, quantification, licence, **coupure (publiée ou estimée)**, contexte, langues

## 2. Recommandation

<!-- @id arch.ia.modeles.recommandation -->
<!-- @role recommendation -->
<!-- @do recommend_models_per_config -->

- **P1 · CPU seul, 8–16 Go RAM** → **Gemma 4 E2B-it** (Q4_K_M 3,11 Go)
  - coupure **janvier 2025 (V)** · Apache-2.0 (V) · 128K (V) · 2,3 B effectifs (V) · texte, image, audio
  - 16 Go : **Gemma 4 E4B-it** (4,98 Go)
  - repli licence la plus simple : **Phi-4-mini-instruct** (MIT, coupure juin 2024 V)
- **P2 · GPU 8 Go + 32 Go RAM** (serveur actuel : RTX 3060 Ti) → **Gemma 4 E4B-it** (4,98 Go, tient en VRAM)
  - coupure janvier 2025 (V) · Apache-2.0 (V) · outils natifs, vision
  - option qualité : **Gemma 4 26B-A4B** (MoE 3,8 B actifs, 16,9 Go, déchargement en RAM)
- **P3 · GPU 12–16 Go** → **Gemma 4 26B-A4B-it** (16,9 Go, 256K, coupure janvier 2025 V)
  - repli : **Ministral 3 14B** (8,24 Go, Apache-2.0, très bon français, coupure NP)
- **Embedding** → **Qwen3-Embedding-0.6B** (Apache-2.0, Q8_0 ≈ 639 Mo, dim 1024 réductible, 100+ langues)
  - repli léger : **multilingual-e5-small** (MIT, dim 384)
- **Grand modèle** : Strata (MoE 125 B) à partir de ~40 Go RAM+VRAM et GPU ≥ 12 Go — voir [mémoire MAIA](MEMOIRE-MAIA.md) §8

## 3. Candidats P1 — CPU seul

<!-- @id arch.ia.modeles.p1 -->
<!-- @role model-catalog -->
<!-- @do list_cpu_models -->

| Modèle | Taille Q4 | Contexte | Coupure | Licence | Français |
|---|---|---|---|---|---|
| Gemma 4 E2B-it (avr. 2026) | 3,11 Go | 128K V | **jan. 2025 V** | Apache-2.0 V | 35+ langues V |
| Gemma 4 E4B-it | 4,98 Go | 128K V | **jan. 2025 V** | Apache-2.0 V | 35+ langues V |
| Qwen3.5-2B (mars 2026) | 1,28 Go | 262K | NP | Apache-2.0 | 201 langues annoncées |
| Qwen3.5-4B | 2,74 Go | 262K | NP | Apache-2.0 V | 201 langues annoncées |
| Ministral 3 3B (déc. 2025) | 2,15 Go | 256K | NP | Apache-2.0 V | 40+ langues V |
| SmolLM3-3B | 1,92 Go | 64K (128K YaRN) V | NP | Apache-2.0 V | FR natif V |
| Phi-4-mini-instruct (fév. 2025) | 2,49 Go | 128K V | **juin 2024 V** | MIT V | FR parmi 23 V |
| Granite 4.0-H-Tiny (MoE 1 B actif) | 4,25 Go | 128K V | NP | Apache-2.0 V | FR parmi 12 V |

## 4. Candidats P2 — GPU 8 Go + 32 Go RAM

<!-- @id arch.ia.modeles.p2 -->
<!-- @role model-catalog -->
<!-- @do list_8gb_gpu_models -->

| Modèle | Taille Q4 | Contexte | Coupure | Licence |
|---|---|---|---|---|
| Gemma 4 E4B-it | 4,98 Go | 128K V | **jan. 2025 V** | Apache-2.0 V |
| Qwen3.5-9B | 5,68 Go | 262K V | NP | Apache-2.0 V |
| Ministral 3 8B | 5,2 Go | 256K V | NP | Apache-2.0 V |
| Qwen3-4B-Instruct-2507 | 2,5 Go | 262K V | NP | Apache-2.0 V |
| Gemma 4 26B-A4B (MoE, déchargement) | 16,9 Go | 256K V | **jan. 2025 V** | Apache-2.0 V |
| Qwen3.5-35B-A3B (MoE, déchargement) | 22 Go | 262K V | NP | Apache-2.0 V |

## 5. Candidats P3 — GPU 12–16 Go

<!-- @id arch.ia.modeles.p3 -->
<!-- @role model-catalog -->
<!-- @do list_16gb_gpu_models -->

| Modèle | Taille Q4 | Contexte | Coupure | Licence |
|---|---|---|---|---|
| Gemma 4 26B-A4B-it | 16,9 Go | 256K V | **jan. 2025 V** | Apache-2.0 V |
| Ministral 3 14B | 8,24 Go | 256K V | NP | Apache-2.0 V |
| gpt-oss-20b (MoE 3,6 B actifs) | 11,6 Go | non listé | INC | Apache-2.0 V |
| Qwen3-30B-A3B-Instruct-2507 | ~18 Go (INC) | 262K V | NP | Apache-2.0 V |

## 6. Embeddings

<!-- @id arch.ia.modeles.embeddings -->
<!-- @role embedding-catalog -->
<!-- @do list_embedding_models -->

| Modèle | Dim | Contexte | Taille | Licence |
|---|---|---|---|---|
| Qwen3-Embedding-0.6B | 1024 (MRL 32–1024) V | 32K V | Q8 ≈ 639 Mo V | Apache-2.0 V |
| BGE-M3 | 1024 V | 8192 V | ~600 Mo (INC) | MIT V |
| multilingual-e5-small | 384 V | 512 V | ~120 Mo int8 (INC) | MIT V |
| granite-embedding-278m-multilingual | 768 V | 512 V | INC | Apache-2.0 V |

## 7. Licences

<!-- @id arch.ia.modeles.licences -->
<!-- @role license-review -->
<!-- @do review_model_licenses -->

- **Retenus** : Apache-2.0 ou MIT uniquement (conserver les mentions de licence dans la distribution)
- **Écartés** :
  - Gemma 3 et EmbeddingGemma → *Gemma Terms of Use* (conditions à répercuter en aval)
  - Llama 3.x → licence maison
  - LFM2 → *LFM Open License* (à lire intégralement)
- Gemma 4 est en **Apache-2.0** (V sur les fiches E2B, E4B, 26B-A4B)

## 8. Intégration dans le COG

<!-- @id arch.ia.modeles.integration -->
<!-- @role integration-plan -->
<!-- @do integrate_local_models -->

- Modèles reçus uniquement par le **canal officiel** (signés, EX-26), chargés en sandbox
- Moteur : `maia-llm` (llama.cpp, CUDA) pour P1 à P3 ; Strata au-delà
- Choix automatique du profil selon la machine (RAM, VRAM) relevée par CaringNanny
- Budget mémoire MAIA selon le profil ([mémoire MAIA](MEMOIRE-MAIA.md) §7)

## 9. À vérifier avant décision

<!-- @id arch.ia.modeles.a-verifier -->
<!-- @role open-questions -->
<!-- @do track_model_verifications -->

- Coupure de gpt-oss-20b (model card OpenAI)
- Mesure empirique de la coupure de Qwen3.5 et Ministral 3 (jeu de questions datées)
- Taille et RAM réelles de BGE-M3 et multilingual-e5-small en int8
- Banc d'essai sur le serveur actuel (RTX 3060 Ti 8 Go) : vitesse et qualité en français de Gemma 4 E4B
