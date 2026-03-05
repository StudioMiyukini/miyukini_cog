# Audit de conformite initial -- MiyuSTT / MiyuTTS / Miyukini Whisper

## TL;DR

Le projet est faisable et coherent avec la direction locale du COG.
Le repo contient deja des briques audio utiles (capture, VAD, wake word, bridge LLM local).
Le principal gap est l absence de toolkits STT/TTS API-first et le decalage contrat Alicia/bridge.
Points de vigilance majeurs: independance STT/TTS vis-a-vis du LLM texte et clarifier les licences TTS.

---

## 1) Conformite architecture COG

| Regle | Etat | Observation |
|---|---|---|
| Local-first | OK | alignement fort avec architecture existante |
| Independance STT/TTS | OK | STT/TTS fonctionnent en local sans LLM requis |
| Degradation gracieuse | PARTIEL | fallback local/host/cloud a formaliser |
| API inter-services | PARTIEL | contrat STT/TTS en place, examples usage cross-COG restants |
| Reuse Alicia | OK | Alicia alignee sur STT `/api/stt` + client TTS feature-flag |
| Isolation composants | OK | design en toolkits separes coherent |
| UI Central test/config | PARTIEL | panneau actif + diagnostics health live, hotkey/e2e restant |

---

## 2) Etat technique actuel (constats)

1. TTS Central existe via `espeak-ng`, mais pas de toolkit TTS COG mutualise.
2. Alicia consomme `POST /api/stt` et `POST /api/nlu` sur un bridge local dedie.
3. Le service `miou-llm-bridge` actuel expose surtout `/v1/*` et inference LLM, pas un bloc STT/TTS complet.
4. Les briques capture/VAD/wake word existent et reduisent le risque implementation.

---

## 3) Ecarts a fermer

| Ecart | Impact | Action P3 |
|---|---|---|
| Pas de `MiyuSTT` | Eleve | creer crate + API `/api/stt` |
| Pas de `MiyuTTS` | Eleve | creer crate + API `/api/tts` |
| Pas de service dictee final | Eleve | creer app `miyukini-whisper` |
| Pas de presets hardware | Eleve | auto-selection + override manuel |
| Contrat Alicia non unifie | Moyen | fixer spec unique v1 |
| STT/TTS dependants d un LLM externe | Eleve | imposer runtime local dedie |
| Licences moteurs heterogenes | Eleve | gate legal P4 obligatoire |

---

## 4) Risques securite et privacy

| Risque | Niveau | Mitigation |
|---|---|---|
| fuite audio locale | Moyen | purge buffers + pas de persistance brute |
| endpoint local expose reseau | Moyen | bind localhost + auth optionnelle |
| prompt injection mode rewrite | Moyen | separation stricte verbatim/clean/rewrite |
| logs contenant donnees sensibles | Moyen | redaction logs + niveaux debug limites |
| fallback cloud active par erreur | Moyen | opt-in explicite + indicateur UI |

---

## 5) Risques licence

| Brique | Risque | Mesure |
|---|---|---|
| moteurs TTS varies | Moyen a Eleve | matrice licence explicite dans ressources |
| modeles open-weights | Moyen | valider conditions redistribution |
| bridge upstream non-open-source eventuel | Faible | non obligatoire, local-only par defaut |

---

## 6) Verdict

Verdict global: **CONFORME SOUS CONDITIONS**

Conditions bloquantes avant P5:
1. Contrat API STT/TTS v1 signe et teste.
2. Choix moteur TTS final valide juridiquement.
3. STT/TTS fonctionnent localement sans LLM texte.
4. Presets hardware valides (compact/balanced/precision).
5. Bench latence/wER documentes.
6. Test humain e2e "dictee dans champ texte" valide.
7. UI Central test/config operationnelle.

---

## 7) Recommandations

1. Implementer d abord MiyuSTT, puis MiyuTTS, puis le service final.
2. Garder le mode `verbatim` sans LLM par defaut.
3. Rendre le mode `rewrite` explicitement opt-in.
4. Fixer un schema JSON stable pour Alicia des V1.
5. Afficher dans Central le backend actif (local/host/cloud) en temps reel.
6. Ajouter un livrable PR + concurrence formel (fait):
   - `.mip/sequences/2026-03-05-miyukini-whisper-local-stack/briefs/2026-03-05-miyukini-whisper-local-stack-pr-concurrence.md`
