# Brief sequence 2026-03-06-mipower-workflow-editor

## Statut

- Etat : APPROUVE
- Mode autonomie : FULL (bascule depuis GUIDED en milieu de E02, decision utilisateur)
- Approuve le : 2026-03-06
- Phase : P0
- Responsable principal : Maria
- Classification : STRATEGIQUE
- Complexite : C5 — Strategique
- Classe tache : T5

## TL;DR

Creer MIPOWER (Miyukini Implementation Protocol Oriented Workflow Editor & Reviewer) : serveur HTTP local Rust (axum) + frontend Vanilla TypeScript. Lance via `cargo run --bin mipower`, accessible dans le navigateur sur 127.0.0.1:9765. 4 modules V1 : Dashboard live, Lecteur rapport graphique riche (Must work #1), Suivi temps reel (SSE + file watcher), Prompt builder. Retrocompatible avec tous les artefacts MIP existants. Stack simplifiee : axum + rusqlite + notify + Vanilla TS + marked.js + mermaid.js. 8 etapes, 37 taches, complexite C5.

---

## 1. Contexte et probleme

MIP a evolue bien au-dela d'un protocole texte : il orchestre des sequences complexes, gere des agents, des metriques, des artefacts. Mais sa base repose encore sur des `.md` statiques, des scripts ps1 manuels et des portails HTML basiques. Cette base ne tient plus la charge ni l'ergonomie necessaire pour un usage intensif et quotidien.

**Probleme central** : Automatisation insuffisante — trop d'actions manuelles repetitives (init sequence, suivi avancement, lecture rapports, recherche/tri sequences).

**Declencheur** : Vision long terme — poser les bases d'un outil durable avant que MIP devienne ingerable.

**HMW retenu** : « Comment pourrions-nous rendre les sequences MIP entierement autonomes, en automatisant init -> suivi -> rapport, et en laissant l'humain n'intervenir qu'aux gates ? »

---

## 2. Solution retenue

**MIPOWER** = serveur HTTP local Rust (axum) ouvert dans le navigateur :

```
Backend Rust (apps/mipower/ dans le workspace Miyukini-COG)
  - axum : serveur HTTP local 127.0.0.1:9765 (connu du projet)
  - rusqlite (bundled) : index sequences + metriques (~APPDATA%/mipower/mipower.db)
  - notify-debouncer-mini : file watcher sequences/
  - SSE (Server-Sent Events) : push events vers le frontend sans websocket
  - Routes : GET /api/sequences, GET /api/artefact?path=..., GET /api/progress/:slug, POST /api/prompt, GET /sse

Frontend Vanilla TypeScript (servi statiquement par axum depuis static/)
  - Dashboard sequences (cards, filtre, tri)
  - Lecteur rapport riche (marked.js + DOMPurify + mermaid.js)
  - ProgressTracker live (SSE client EventSource)
  - Prompt builder (formulaire -> premier prompt MIP + appel init-sequence-base.ps1)
```

**Retrocompatibilite** : MIPOWER lit les artefacts MIP existants sans modification (sequences/index.json, .md, metrics/*.json).

---

## 3. Perimetre V1

### INCLUS

- Bootstrap workspace Tauri v2 (E00)
- Backend SQLite + IPC de base (E01)
- File watcher + indexation (E02)
- Dashboard sequences avec filtre/recherche (E03)
- Lecteur rapport Markdown riche (tableaux + Mermaid) — Must work #1 (E04)
- Suivi temps reel progression par phases (E05)
- Prompt builder + init sequence (E06)
- Polish UI dark mode Miyukini (E07)
- CI/CD minimal GitHub Actions Windows

### EXCLUS

- Edition inline des artefacts .md dans MIPOWER
- Versioning git integre
- Knowledge Base interactive
- Integration LLM runtime
- Cloud sync

---

## 4. Approches evaluees

| Approche | Score | Verdict |
|----------|-------|---------|
| **Rust axum + Vanilla TS** | 9/10 | RETENU — stack connue (axum = miyucloud), zero Tauri, zero framework JS |
| Tauri v2 + Vanilla TS | 7/10 | Ecarte — Tauri setup Windows complexe, inutile pour usage local |
| Tauri v2 + Svelte 5 | 7/10 | Ecarte — complexite triple stack, Svelte 5 runes = nouvel apprentissage |
| Electron + React | 4/10 | Ecarte — 200MB+, non coherent Rust |

---

## 5. Plan d'execution

| Etape | Titre | Agent(s) | Taches |
|-------|-------|----------|--------|
| E00 | Bootstrap Tauri | Francois | 4 |
| E01 | Backend core SQLite + IPC | Francois | 6 |
| E02 | File watcher (|| E03) | Francois | 4 |
| E03 | Dashboard frontend (|| E02) | Lise | 5 |
| E04 | Lecteur rapport — Must work (|| E05) | Lise | 5 |
| E05 | Suivi temps reel (|| E04) | Francois+Lise | 4 |
| E06 | Prompt builder | Lise+Francois | 5 |
| E07 | Polish UI | Lise | 4 |
| BUF | Buffer | Francois+Lise | variable |

DAG : E00 -> E01 -> [E02 || E03] -> [E04 || E05] -> E06 -> E07 -> BUF

---

## 6. Risques

| Risque | Impact | Mitigation |
|--------|--------|-----------|
| Complexite Tauri v2 (setup Windows, WebView2) | Haut | T4 Denis a evalue ; doc officielle Tauri v2 ; plan fallback web local |
| Synchronisation live fichiers (concurrence) | Moyen | notify-debouncer-mini 500ms ; tests unitaires watcher isoles |
| Scope creep | Haut | Perimetre V1 borde strict ; Denis garde le plan |
| Mermaid.js rendu asynchrone | Moyen | Initialisation unique + mermaid.run() sur update |

---

## 7. Securite (RPS Victor)

- Tauri capabilities bornes (.mip/ + %APPDATA%/mipower/ uniquement)
- Chemins FS canonicalises et valides avant toute commande
- SQLite : parameterized queries obligatoires
- Markdown : DOMPurify.sanitize() sur tout HTML inline
- CSP WebView2 : scripts self uniquement
- Score risque pre-impl : 45/100 (app locale, surfaces controlees)

---

## 8. Efficience (Jean)

- Budget P0 : ~74k tokens
- Budget P3 : ~180k tokens (C5, 37 taches)
- Budget P4+P5+P6 : ~38k tokens
- **Total estime : ~292k tokens**
- Subagent frais obligatoire par tache P3
- Checkpoint Denis toutes les 5 taches

---

## 9. Stack confirmee

| Couche | Technologie | Version |
|--------|------------|---------|
| Serveur HTTP | axum (workspace Miyukini-COG) | 0.7.x |
| Backend | Rust + cargo workspace Miyukini-COG | stable |
| DB | rusqlite (bundled) | 0.31 |
| File watcher | notify-debouncer-mini | 0.4 |
| Push live | Server-Sent Events (SSE axum) | — |
| Frontend | Vanilla TypeScript (esbuild) | — |
| CSS | CSS custom (variables dark mode) | — |
| Markdown | marked.js + DOMPurify | 12.x / 3.x |
| Diagrams | mermaid.js | 10.x |
| Build | cargo build (back) + esbuild (TS) | — |
| Tests back | cargo test | — |
| Tests front | deno test ou vitest (minimal) | — |

---

## 10. Artefacts produits par P0

- [x] briefs/2026-03-06-mipower-workflow-editor.md (ce fichier)
- [x] specs/2026-03-06-mipower-workflow-editor-spec.md
- [x] plans_p3/2026-03-06-mipower-workflow-editor-plan.md
- [x] plans_p3/etapes/etape-00.md a etape-07.md + etape-buf.md
- [x] agents/P3_francois.md, P3_lise.md, P4_victor.md
- [x] phases/p0/temps/temps-01-exploration.md (T1 complet)
- [x] phases/p0/temps/temps-02-ideation.md (T2 complet)
- [x] ui/index.html (mini-site sequence)

---

## Gate P0

Ce brief est soumis a approbation utilisateur.

Apres approbation : choix du mode d'autonomie (FULL / BIG_STEPS / GUIDED) puis lancement P3.
