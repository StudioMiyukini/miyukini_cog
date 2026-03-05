# Specification UI Central -- Miyukini Whisper

## TL;DR

Cette UI s integre dans Central pour tester et configurer Miyukini Whisper.
Elle doit permettre de verifier rapidement STT/TTS, presets hardware, et fallback.
Aucun mode cloud n est actif sans action explicite utilisateur.

---

## 1) Emplacement dans Central

- Nouveau service visible dans la grille services:
  - id: `miyukini-whisper`
  - type: `InterneCog`
  - description: "Dictee locale STT/TTS avec presets hardware"

- Nouveau module UI:
  - `apps/central/src/services/miyukini_whisper/mod.rs`

---

## 2) Structure de l ecran

1. Header
- statut global (online/degraded/offline)
- backend actif (`local`, `host-bridge`, `cloud`)
- bouton restart service

2. Onglets
- `STT Test`
- `TTS Test`
- `Presets`
- `Fallback`
- `Diagnostics`

---

## 3) Details onglets

## 3.1 STT Test

- select langue: `auto | fr | en`
- bouton `Push to talk`
- affichage transcript partiel
- affichage transcript final
- metriques:
  - latence partielle
  - latence finale
  - modele STT utilise

## 3.2 TTS Test

- textarea input
- select voix FR/EN
- select vitesse
- bouton `Play`
- metriques:
  - latence synthese
  - moteur TTS utilise

## 3.3 Presets

- option `Auto (recommande)`
- options manuelles:
  - `compact`
  - `balanced`
  - `precision`
- resume hardware detecte:
  - CPU
  - RAM
  - GPU/VRAM
- preview des modeles qui seront charges

## 3.4 Fallback

- `Local only` (defaut)
- `Host bridge` (URL + ping test)
- `Cloud` (toggle + provider + cle)
- affichage ordre de routage effectif
- avertissement privacy visible

## 3.5 Diagnostics

- `/api/health` MiyuSTT
- `/api/health` MiyuTTS
- historique erreurs recentes
- bouton export logs local

---

## 4) Donnees persistees

Profil utilisateur Central:
- preset selectionne
- langue par defaut
- voix TTS par defaut
- mode fallback selectionne
- endpoint host bridge

---

## 5) Critere d acceptance

1. Un utilisateur peut lancer un test STT et obtenir un transcript dans l UI.
2. Un utilisateur peut lancer un test TTS et entendre l audio local.
3. Le preset hardware auto est visible et modifiable.
4. Le mode fallback actif est lisible en permanence.
5. Aucun appel cloud sans activation explicite.
