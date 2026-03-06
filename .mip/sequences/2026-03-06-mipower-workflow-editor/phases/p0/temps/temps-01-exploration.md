# P0 Temps 1 - Exploration et brainstorming

## Statut

- Etat : A completer
- Phase : P0 Temps 1
- Responsable principal : Maria

## TL;DR

Demande T5 : creer MIPOWER, evolution standalone de MIP, avec interface graphique moderne (dashboard, rapports, prompt builder), versioning/indexation des sequences, metriques temps reel, et capitalisation de l'experience MIP. Stack a definir en P0. Complexite estimee C5.

## Section 0 — Orientation (deduire depuis le premier prompt)

> Maria remplit ce tableau SEULE, avant de poser des questions a l'utilisateur.

| Question | Reponse deduite | Confiance |
|----------|----------------|-----------|
| Pourquoi exactement ? Quel probleme est resolu ? | MIP a depasse le scope d'un protocole texte : artefacts .md statiques, scripts ps1 manuels, portails HTML basiques ne tiennent plus la charge. Il faut une vraie application scalable, robuste, ergonomique. | haute |
| Exemple concret d'usage attendu ? | Ouvrir MIPOWER → dashboard sequences actives/archivees → creer une sequence via prompt builder → suivre T1-T11 temps reel → lire rapport P6 avec tableaux/diagrammes. | haute |
| Solution existante proche dans le projet ? | .mip/index.html (portail racine) + .mip/sequences/*/ui/index.html (mini-site sequence) — vues HTML statiques partielles, sans etat persistant. | haute |
| Pour qui ? (utilisateur final, persona) | Dev/architecte solo (Miyukini) + agents IA. Usage local, quotidien, haute frequence. | haute |
| Fonction Online / MWS requise ? | Non — application locale (web local ou desktop) | haute |
| Open-source / forkable ou from scratch ? | From scratch — herite des concepts MIP (phases, sequences, artefacts) | haute |
| Classification estimee (T1-T5) | T5 — Chantier strategique : nouvelle app autonome, stack a definir, architecture complete | haute |

## Brainstorming

### Section 1 — COMPRENDRE
- 1.1 Probleme central : **Automatisation insuffisante** — trop d'actions manuelles repetitives (init, suivi, lecture rapports, recherche)
- 1.2 Declencheur : **Vision long terme** — poser les bases d'un outil durable avant que MIP devienne ingerable
- 1.4 Frictions identifiees (toutes) : suivi avancement temps reel | init sequence manuelle | lecture rapports .md | recherche/tri sequences
- 1.5 Approche : **Unification en un seul outil** (remplacer scripts + portail + mini-sites + metriques JSON)

### Section 2 — CADRER
- 2.1 Contraintes : local uniquement | sans build complex (pas de node_modules 500MB) | independant du LLM
- 2.2 Scope V1 : Dashboard sequences + Lecteur rapports graphique + Suivi temps reel + Prompt builder (perimetre complet)
- 2.3 Priorite : Equilibre Dashboard + Lecteur + Prompt builder
- 2.4 Horizon : Pas de contrainte temporelle — qualite avant vitesse

### Section 3 — IMAGINER
- 3.1 Stack preferee utilisateur : **Tauri (Rust + Web frontend)** — leger, natif, coherent avec le workspace Rust
- 3.3 Reutiliser : sequences/index.json + artefacts .md existants + metrics/*.json
- 3.6 HMW retenu : "Comment pourrions-nous rendre les sequences MIP entierement autonomes ?"

### Section 4 — EVALUER
- 4.1 Must work #1 : **Lecteur rapport avec tableaux et diagrammes** — rendu visuel riche des rapports P6
- 4.2 Risques : Complexite Tauri (triple stack Rust+frontend+Tauri) | Synchronisation live fichiers (file watcher concurrent)
- 4.3 Complexite : **C5 — Strategique** (app autonome, architecture from scratch, UI complete)

## Hypotheses retenues

- MIPOWER est une application desktop locale (Tauri + Rust backend + frontend web)
- Frontend : a definir en T4/T6 (Svelte ou Vanilla TS prefere pour la legerete)
- Elle lit les artefacts MIP existants en retrocompatibilite (sequences/index.json, .md, metrics/*.json)
- Format .md conserve pour rapports/skills/memory ; JSON/SQLite pour indexation/metriques internes
- File watcher Rust pour suivi temps reel (notify crate)
- Pas de dependance cloud ni LLM au runtime

## Hypotheses ecartees

- Electron : trop lourd (Chromium + node_modules), non coherent avec le projet Rust
- Web local pur (serveur HTTP) : moins fluide comme experience desktop
- Amelioration incrementale des scripts ps1 : n'adresse pas les frictions fondamentales

## Classification post-T1

- Classe tache (T1-T5) : **T5** — Chantier strategique : nouvelle application autonome, architecture complete from scratch
- Complexite sequence estimee (C1-C5) : **C5** — App strategique (Tauri+Rust+frontend), UI complete (dashboard+lecteur+prompt builder+suivi live), file watcher, retrocompatibilite artefacts MIP
  > Confirme par utilisateur en T1 Section 4. Sera valide par Denis en T8.

## Statut final T1

- Etat : TERMINE
- Horodatage : 2026-03-06
- Classification retenue : T5 / C5

