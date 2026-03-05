# P0 Travail -- miyukini-whisper-local-stack

## Objet
Lancer une sequence MIP T5 pour construire une stack vocale locale:
`MiyuSTT`, `MiyuTTS`, puis service final `Miyukini Whisper`.

## Decisions de cadrage
- Classe: T5
- Scope: toolkits STT/TTS + service dictee + UI Central + integration Alicia
- Contraintes: local-first, STT/TTS independants d autres LLM, FR/EN, presets hardware
- Fallback: host bridge et cloud uniquement en opt-in
- Recommandation: Approche A (toolkits d abord, service ensuite)
- Mode recommande: BIG_STEPS

## Livrables P0 produits
- Brief principal: `briefs/2026-03-05-miyukini-whisper-local-stack.md`
- Spec technique: `specs/2026-03-05-miyukini-whisper-local-stack-spec.md`
- Spec UI Central: `specs/2026-03-05-miyukini-whisper-local-stack-central-ui-spec.md`
- Plan P3: `plans_p3/2026-03-05-miyukini-whisper-local-stack-plan.md`
- Audit conformite: `audits/2026-03-05-conformite.md`
- Analyse PR + concurrence: `briefs/2026-03-05-miyukini-whisper-local-stack-pr-concurrence.md`

## Statut Gate P0
- Etat: VALIDE
- Approbation: APPROUVE
- Mode autonomie: FULL
- Etape suivante: execution P3 V0 (autopilot)
