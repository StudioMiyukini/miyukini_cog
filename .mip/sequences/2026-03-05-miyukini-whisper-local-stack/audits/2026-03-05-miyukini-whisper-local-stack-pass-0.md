# PASS-0 -- Plan d audit securite

## TL;DR

Plan d audit securite P4 pour la sequence Miyukini Whisper.
Objectif: verifier auth, validation, politiques origin, dependances, logs.
Perimetre: `miyustt`, `miyutts`, `miyukini-whisper-app`, `miyualicia`.

## Horodatage

- debut: 2026-03-05T13:51:22Z
- fin cible: 2026-03-05T13:52:29Z

## PASS prevus

1. PASS-01 -- Endpoints et auth (STT/TTS)
2. PASS-02 -- Validation entrees et fallback chain
3. PASS-03 -- Dependances, secrets, logs

## Criteres bloquants

- contournement auth bearer quand active
- acceptation origin distant avec policy locale active
- fuite de secret en clair dans code sequence
- crash sur payload invalide non gere
