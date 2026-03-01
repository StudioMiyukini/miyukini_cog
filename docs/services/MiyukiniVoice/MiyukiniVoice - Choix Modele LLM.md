# MiyukiniVoice - Choix Modele LLM Vocal

> @id service.voice.miyukinivoice.llm_choice
> @role technical_decision
> @layer 7
> @do select_optimal_local_llm_for_voice_assistant
> @human Benchmark et recommandation du modele LLM local pour le pipeline vocal MiyukiniVoice

---

## Contexte

MiyukiniVoice necessite un modele LLM local qui doit cohabiter avec Whisper small (~2 Go VRAM) sur une RTX 5070 12 Go. Le modele doit repondre a des commandes vocales courtes en francais, supporter le tool calling (agenda, rappels, intercom), et offrir une latence suffisamment basse pour une experience vocale fluide.

Ce document presente l'evaluation des candidats et la recommandation finale, basee sur des recherches factuelles (crates.io, HuggingFace, benchmarks publics, documentation officielle).

## Portee / Scope

### Inclus
- Evaluation de 7 modeles candidats sur 5 criteres
- Recommandation finale avec justification
- Configuration LM Studio recommandee
- Notes sur Qwen3 (generation suivante)

### Exclus
- Tests d'inference en conditions reelles (a realiser en Phase 2)
- Fine-tuning du modele
- Benchmark des modeles > 14B (exclus par contrainte VRAM)

---

## 1. Contraintes (du Document Fondateur)

| Contrainte | Valeur | Source |
|-----------|--------|--------|
| GPU | RTX 5070 12 Go VRAM | Hardware Machine A |
| VRAM reservee Whisper small | ~2 Go | Estimation standard |
| VRAM disponible LLM | **9-10 Go maximum** | 12 - 2 = 10 Go |
| Marge de securite VRAM | **1-2 Go minimum** | Eviter OOM sous charge |
| VRAM utilisable LLM en pratique | **7-9 Go** | 10 - (1 a 2) marge |
| Latence cible | < 1.5 s pour reponses courtes | Critere Maria |
| Qualite francais | Fluide et naturelle | Critere utilisateur |
| Tool calling | Fiable et structure (JSON) | Scope v1 (agenda, rappels, intercom) |
| Format | GGUF (LM Studio) | Infrastructure existante miou-llm-bridge |
| Quantification | Q4_K_M recommandee | Meilleur ratio qualite/taille |

---

## 2. Candidats evalues

### 2.1 Tableau comparatif

| Modele | Parametres | Taille Q4_K_M | VRAM estimee | Marge 12 Go | Tool calling | Qualite FR | Latence estimee |
|--------|-----------|---------------|-------------|-------------|-------------|-----------|----------------|
| **Qwen 2.5 7B Instruct** | 7.6B | ~4.7 Go | ~5 Go | **5 Go** | Excellent (natif) | Tres bonne (29+ langues) | ~800 ms |
| **Llama 3.1 8B Instruct** | 8.0B | ~5.0 Go | ~5.5 Go | **4.5 Go** | Bon (template dedie) | Bonne | ~900 ms |
| **Mistral 7B v0.3 Instruct** | 7.2B | ~4.4 Go | ~5 Go | **5 Go** | Moyen (tokens speciaux, IDs 9 digits) | Tres bonne (origine FR) | ~800 ms |
| **Phi-4 mini 3.8B** | 3.8B | ~2.5 Go | ~3 Go | **7 Go** | Bon (post-training) | Bonne (supporte FR officiel) | ~400 ms |
| **Qwen3 4B Instruct** | 4.0B | ~2.5 Go | ~3 Go | **7 Go** | Excellent (Hermes-style natif) | Tres bonne (multilingue natif) | ~450 ms |
| **Qwen3 8B Instruct** | 8.2B | ~4.7 Go | ~5.5 Go | **4.5 Go** | Excellent (Hermes-style natif) | Tres bonne | ~850 ms |
| **Gemma 2 9B Instruct** | 9.2B | ~5.5 Go | ~6 Go | **4 Go** | Moyen (pas natif) | Moyenne (anglais-centre) | ~1000 ms |

### 2.2 Analyse detaillee par critere

#### A. Tool calling / Function calling

| Modele | Qualite | Details |
|--------|---------|---------|
| **Qwen 2.5 7B** | **Excellent** | Support natif Hermes-style. Chat template integre. Compatible LM Studio. Genere du JSON structure fiable. |
| **Qwen3 4B/8B** | **Excellent** | Amelioration significative du tool calling par rapport a Qwen 2.5. Support natif via Qwen-Agent, compatible llama.cpp, Ollama, LM Studio. Modele specialement entraine pour le tool usage. |
| **Llama 3.1 8B** | **Bon** | Support via template dedie. Fiable mais moins elegant que Qwen. |
| **Mistral 7B v0.3** | **Moyen** | Tokens speciaux (TOOL_CALLS, AVAILABLE_TOOLS). Problemes de generation de tool_call IDs (exactement 9 digits). Difficile avec les appels paralleles. |
| **Phi-4 mini 3.8B** | **Bon** | Post-training ameliore pour le function calling. Fonctionne mais qualite variable sur des schemas complexes. Problemes rapportes avec Ollama. |
| **Gemma 2 9B** | **Moyen** | Pas de support natif tool calling. Necessiterait du prompt engineering custom. |

**Verdict tool calling : Qwen 2.5 7B ou Qwen3 4B/8B.**

#### B. Qualite en francais

| Modele | Qualite | Details |
|--------|---------|---------|
| **Mistral 7B v0.3** | **Tres bonne** | Equipe Mistral AI (Paris). Le francais est une priorite. Vocabulaire riche, conjugaisons correctes, expressions naturelles. |
| **Qwen 2.5 7B** | **Tres bonne** | Entraine sur 29+ langues dont le francais. Bonne comprehension contextuelle. |
| **Qwen3 4B/8B** | **Tres bonne** | Multilingue ameliore, couverture linguistique etendue. |
| **Llama 3.1 8B** | **Bonne** | Multilingue (8 langues principales). Francais correct mais moins naturel que Mistral/Qwen. |
| **Phi-4 mini 3.8B** | **Bonne** | Supporte officiellement le francais (parmi 24 langues). Qualite acceptable mais inferieure aux 7B+. |
| **Gemma 2 9B** | **Moyenne** | Principalement anglais-centre. Francais fonctionnel mais avec des artefacts. |

**Verdict francais : Mistral 7B ou Qwen 2.5 7B, suivis par Qwen3.**

#### C. VRAM et cohabitation avec Whisper

| Modele | VRAM LLM | VRAM Whisper | Total | Marge sur 12 Go | Verdict |
|--------|----------|-------------|-------|-----------------|---------|
| **Phi-4 mini 3.8B** | ~3 Go | ~2 Go | ~5 Go | **7 Go** | Excellent |
| **Qwen3 4B** | ~3 Go | ~2 Go | ~5 Go | **7 Go** | Excellent |
| **Mistral 7B** | ~5 Go | ~2 Go | ~7 Go | **5 Go** | Tres bon |
| **Qwen 2.5 7B** | ~5 Go | ~2 Go | ~7 Go | **5 Go** | Tres bon |
| **Llama 3.1 8B** | ~5.5 Go | ~2 Go | ~7.5 Go | **4.5 Go** | Bon |
| **Qwen3 8B** | ~5.5 Go | ~2 Go | ~7.5 Go | **4.5 Go** | Bon |
| **Gemma 2 9B** | ~6 Go | ~2 Go | ~8 Go | **4 Go** | Limite |

**Verdict VRAM : Phi-4 mini et Qwen3 4B offrent la meilleure marge. Les 7B sont confortables. Les 8-9B sont faisables mais sans grande marge.**

#### D. Latence pour le vocal

Pour une experience vocale fluide, on cible des reponses de 30-80 tokens (1-3 phrases courtes). La latence comprend le time-to-first-token (TTFT) et la generation.

| Modele | TTFT estime | Generation (30 tokens) | Total estime |
|--------|-------------|----------------------|-------------|
| **Phi-4 mini 3.8B** | ~100 ms | ~300 ms | **~400 ms** |
| **Qwen3 4B** | ~120 ms | ~330 ms | **~450 ms** |
| **Mistral 7B** | ~200 ms | ~600 ms | **~800 ms** |
| **Qwen 2.5 7B** | ~200 ms | ~600 ms | **~800 ms** |
| **Llama 3.1 8B** | ~250 ms | ~650 ms | **~900 ms** |
| **Qwen3 8B** | ~250 ms | ~600 ms | **~850 ms** |
| **Gemma 2 9B** | ~300 ms | ~700 ms | **~1000 ms** |

*Estimations basees sur RTX 5070, Q4_K_M, contexte 2048 tokens. Les valeurs reelles dependent de la charge GPU et du batch size.*

**Verdict latence : Phi-4 mini et Qwen3 4B sont les plus rapides. Les 7B restent sous 1s. Tous les candidats sont dans la cible < 1.5s.**

#### E. Score synthetique

| Modele | Tool calling (/25) | FR (/25) | VRAM (/20) | Latence (/15) | Qualite generale (/15) | **Total (/100)** |
|--------|-------------------|---------|-----------|--------------|----------------------|-----------------|
| **Qwen 2.5 7B** | 24 | 23 | 16 | 12 | 14 | **89** |
| **Qwen3 8B** | 25 | 23 | 14 | 11 | 15 | **88** |
| **Qwen3 4B** | 24 | 22 | 20 | 15 | 12 | **93** |
| **Mistral 7B** | 15 | 25 | 16 | 12 | 13 | **81** |
| **Llama 3.1 8B** | 20 | 20 | 14 | 11 | 13 | **78** |
| **Phi-4 mini 3.8B** | 18 | 19 | 20 | 15 | 10 | **82** |
| **Gemma 2 9B** | 10 | 15 | 12 | 10 | 12 | **59** |

---

## 3. Recommandation finale

### Recommandation principale : Qwen3 4B Instruct (Q4_K_M)

**Justification :**

1. **Meilleur ratio performance/ressources** : avec seulement ~3 Go de VRAM, il laisse 7 Go de marge sur 12 Go, ce qui garantit une cohabitation stable avec Whisper small et elimine tout risque d'OOM.

2. **Tool calling excellent** : Qwen3 a ete specifiquement entraine pour le tool usage, avec un support natif Hermes-style. C'est la fonctionnalite la plus critique pour Alicia v1 (agenda, rappels, intercom).

3. **Latence optimale pour le vocal** : ~450 ms pour une reponse de 30 tokens sur RTX 5070. C'est 2x plus rapide que les modeles 7B, ce qui rend la conversation vocale fluide.

4. **Francais de bonne qualite** : multilingue de naissance (Qwen), couverture linguistique etendue incluant le francais. Legerement en dessous de Qwen 2.5 7B pour la richesse du vocabulaire, mais suffisant pour un assistant vocal domestique.

5. **Modernite** : generation Qwen3 (2025), post-training ameliore par rapport a Qwen 2.5.

### Recommandation secondaire (fallback) : Qwen 2.5 7B Instruct (Q4_K_M)

**Si Qwen3 4B ne satisfait pas en termes de qualite de reponse** (raisonnement trop superficiel, reponses trop courtes ou imprecises), basculer vers Qwen 2.5 7B Instruct :

- VRAM ~5 Go, total ~7 Go avec Whisper, marge de 5 Go = confortable
- Tool calling excellent (meme architecture Hermes)
- Francais tres bon (29+ langues, benchmarks solides)
- Latence ~800 ms = acceptable pour le vocal
- Ecosysteme mature (documentation, communaute, GGUF disponibles)

### Modeles exclus et raisons

| Modele | Raison d'exclusion |
|--------|-------------------|
| **Mistral 7B v0.3** | Tool calling problematique (IDs 9 digits, appels paralleles defaillants). Malgre un excellent francais, le tool calling est critique pour Alicia. |
| **Llama 3.1 8B** | Francais moins naturel que Qwen/Mistral. Pas d'avantage decisif sur les autres criteres. |
| **Phi-4 mini 3.8B** | VRAM et latence excellentes mais qualite de raisonnement insuffisante pour 3.8B. Problemes de tool calling rapportes avec Ollama. |
| **Gemma 2 9B** | Pas de support tool calling natif. Anglais-centre. VRAM elevee. |
| **Qwen3 8B** | Bon modele mais pas d'avantage decisif sur Qwen 2.5 7B. Plus de VRAM pour un gain marginal. |
| **gpt-oss-20b** | Trop gros (~10-12 Go VRAM), ne cohabite pas avec Whisper sur 12 Go. Reste disponible pour les usages non-vocaux via LM Studio. |

---

## 4. Configuration LM Studio recommandee

### 4.1 Modele principal : Qwen3 4B Instruct

```
Modele : Qwen3-4B-Instruct-2507 (GGUF Q4_K_M)
Source : unsloth/Qwen3-4B-Instruct-2507-GGUF ou bartowski/
Taille fichier : ~2.5 Go
VRAM : ~3 Go
```

**Parametres LM Studio :**

| Parametre | Valeur | Justification |
|-----------|--------|---------------|
| Context Length | 4096 tokens | Suffisant pour une conversation vocale courte + historique |
| Temperature | 0.6 | Plus deterministe que le defaut (0.7-0.8) pour des reponses vocales coherentes |
| Top P | 0.9 | Standard |
| Max Tokens | 256 | Reponses vocales courtes (pas de pavés) |
| Repeat Penalty | 1.1 | Evite les repetitions |
| GPU Layers | All (toutes les couches sur GPU) | Maximise la vitesse |
| Batch Size | 512 | Standard pour un 4B |
| Flash Attention | true (si supporte) | Reduit la VRAM et accelere |

### 4.2 Modele fallback : Qwen 2.5 7B Instruct

```
Modele : Qwen2.5-7B-Instruct (GGUF Q4_K_M)
Source : bartowski/Qwen2.5-7B-Instruct-GGUF
Taille fichier : ~4.7 Go
VRAM : ~5 Go
```

**Parametres identiques** sauf :
- Batch Size : 512
- Context Length : 4096 (peut aller jusqu'a 8192 si la marge VRAM le permet)

### 4.3 Configuration du fork miou-voice-bridge

Le fork du bridge devra implementer :

1. **System prompt Alicia** : personnalite bienveillante, reponses courtes et vocales, francais naturel
2. **Tools schema** : definitions JSON des fonctions agenda, rappels, intercom
3. **Timeout agressifs** : 10s max par requete (vs 120s pour le bridge standard)
4. **Streaming** : activer le streaming pour commencer le TTS des le premier token utile
5. **Context window glissant** : garder les 5 derniers echanges max (economie de tokens)

---

## 5. Plan de test (Phase 2)

Avant de valider definitivement le choix, les tests suivants seront realises en Phase 2 :

| Test | Methode | Critere de succes |
|------|---------|-------------------|
| VRAM cohabitation | Charger Whisper small + LLM simultanement, monitorer VRAM | < 10 Go total, pas d'OOM |
| Latence reelle | 20 requetes vocales courtes, mesurer TTFT + generation | < 1.5s en P50, < 2s en P95 |
| Qualite FR | 20 questions variees en francais, evaluation subjective | > 80% reponses jugees "naturelles" |
| Tool calling | 10 scenarios d'appels d'outils (agenda, rappels) | > 90% appels correctement structures |
| Stabilite | 30 minutes d'utilisation continue | 0 crash, 0 OOM, 0 timeout |

---

*Document produit par Denis, Chef Dev Senior Miyukini AI Studio -- 2026-02-28*
*Basé sur : recherches web (crates.io, HuggingFace, benchmarks publics, documentation officielle Qwen/Mistral/Meta/Microsoft)*
*A destination de : Maria (validation), Francois (implementation Phase 2), Arianne (archivage)*
