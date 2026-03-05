# Miyukini â€” Plan de Migration vers Tauri + React/TypeScript + Tailwind

## Contexte

Migration de l'UI eGUI vers **Tauri 2.0 + React/TypeScript + Tailwind CSS** pour obtenir un rendu moderne style Steam/Epic/GoG tout en respectant les LOI du COG.

**Date :** 2026-02-08  
**Statut :** Plan validÃ©, en cours d'exÃ©cution

---

## 1. Services concernÃ©s

### Services Ã  migrer (suppression eGUI)

| Service | Crate | Logique mÃ©tier Ã  conserver | UI Ã  supprimer |
|---------|-------|---------------------------|----------------|
| **Miyukini Central** | `miyukini-central` | `auth/`, `services/`, `config.rs` | `app.rs`, dÃ©pendances eGUI |
| **JayFestival** | `jayfestival` | `auth/`, `data/`, `services/` | dÃ©pendances eGUI |
| **JayKoa** | `jaykoa` | `data/`, `export/`, `services/` | dÃ©pendances eGUI |
| **JayKonta** | `jaykonta` | `backend/`, `data/`, `domain/`, `integrations/` | dÃ©pendances eGUI |
| **JayXpose** | `jayxpose` | `auth/`, `data/`, `governance.rs` | dÃ©pendances eGUI |
| **MiyuClicker** | `miyuclicker` | `carte.rs`, `combat.rs`, `idlesim.rs`, `save.rs`, `state.rs` | `app.rs`, `ui_assets.rs`, dÃ©pendances eGUI |

### Service conservÃ© avec eGUI

| Service | Crate | Raison |
|---------|-------|--------|
| **MiyukiniSurvivor** | `lord_of_the_castle` | Jeu conservÃ© en eGUI (demande utilisateur) |

---

## 2. Nouvelle architecture

### Structure proposÃ©e

```
Miyukini_COG/
â”œâ”€â”€ crates/                          # Backend Rust (inchangÃ© pour la logique)
â”‚   â”œâ”€â”€ miyukini-central/            # Backend Central (Tauri commands)
â”‚   â”œâ”€â”€ jayfestival/                 # Backend JayFestival (lib)
â”‚   â”œâ”€â”€ jaykoa/                      # Backend JayKoa (lib)
â”‚   â”œâ”€â”€ jaykonta/                    # Backend JayKonta (lib)
â”‚   â”œâ”€â”€ jayxpose/                    # Backend JayXpose (lib)
â”‚   â”œâ”€â”€ miyuclicker/                 # Backend MiyuClicker (lib)
â”‚   â””â”€â”€ lord_of_the_castle/          # CONSERVÃ‰ EN EGUI
â”‚
â”œâ”€â”€ apps/                            # Applications Tauri
â”‚   â”œâ”€â”€ central/                     # Miyukini Central (app principale)
â”‚   â”‚   â”œâ”€â”€ src-tauri/               # Backend Tauri (Rust)
â”‚   â”‚   â”‚   â”œâ”€â”€ Cargo.toml
â”‚   â”‚   â”‚   â””â”€â”€ src/
â”‚   â”‚   â”‚       â”œâ”€â”€ main.rs
â”‚   â”‚   â”‚       â”œâ”€â”€ commands/        # Commandes exposÃ©es au frontend
â”‚   â”‚   â”‚       â””â”€â”€ lib.rs
â”‚   â”‚   â”œâ”€â”€ src/                     # Frontend React
â”‚   â”‚   â”‚   â”œâ”€â”€ App.tsx
â”‚   â”‚   â”‚   â”œâ”€â”€ components/
â”‚   â”‚   â”‚   â”œâ”€â”€ pages/
â”‚   â”‚   â”‚   â”œâ”€â”€ hooks/
â”‚   â”‚   â”‚   â””â”€â”€ styles/
â”‚   â”‚   â”œâ”€â”€ package.json
â”‚   â”‚   â”œâ”€â”€ tailwind.config.js
â”‚   â”‚   â”œâ”€â”€ tsconfig.json
â”‚   â”‚   â””â”€â”€ vite.config.ts
â”‚   â”‚
â”‚   â””â”€â”€ portal/                      # Miyukini Web Portal (futur)
â”‚       â””â”€â”€ ...
â”‚
â””â”€â”€ packages/                        # Packages partagÃ©s (monorepo pnpm)
    â”œâ”€â”€ ui/                          # Composants UI partagÃ©s
    â”‚   â”œâ”€â”€ src/
    â”‚   â”‚   â”œâ”€â”€ components/
    â”‚   â”‚   â”œâ”€â”€ hooks/
    â”‚   â”‚   â””â”€â”€ styles/
    â”‚   â””â”€â”€ package.json
    â””â”€â”€ types/                       # Types TypeScript partagÃ©s
        â””â”€â”€ package.json
```

### Stack technique

| Composant | Technologie | Version |
|-----------|-------------|---------|
| **Shell desktop** | Tauri | 2.0 |
| **Backend** | Rust | 1.75+ |
| **Frontend framework** | React | 19 |
| **Langage frontend** | TypeScript | 5.x |
| **Styling** | Tailwind CSS | 4.x |
| **Build tool** | Vite | 6.x |
| **Package manager** | pnpm | 9.x |
| **Composants UI** | shadcn/ui | latest |
| **Animations** | Framer Motion | 11.x |

---

## 3. ConformitÃ© LOI

| LOI | ConformitÃ© | Justification |
|-----|------------|---------------|
| **LOI-1** (Pas de dÃ©p externe critique) | âœ… | WebView natif de l'OS, tout bundlÃ© localement |
| **LOI-2** (Isolation comme Ã©tat normal) | âœ… | Fonctionne 100% offline, pas de serveur externe |
| **LOI-3** (Ã‰tat local souverain) | âœ… | Backend Rust avec KindMother |
| **LOI-4** (Pas de temps global) | âœ… | Horloge locale (MiyuClock) |
| **LOI-5** (CoÃ»t proportionnel hardware) | âœ… | Bundle 2-3 MB, RAM ~30 MB |
| **LOI-6** (FÃ©dÃ©ration possible) | âœ… | Webway inchangÃ© |
| **LOI-7** (Cores immuables) | âœ… | Aucun impact sur les Cores |
| **LOI-8** (Migration = diplomatie) | âœ… | Aucun impact |

---

## 4. Ã‰tapes de migration

### Phase 1 : PrÃ©paration âœ… TERMINÃ‰E

1. âœ… CrÃ©er ce plan de migration
2. âœ… Supprimer les dÃ©pendances eGUI des services concernÃ©s
3. âœ… Supprimer les fichiers UI eGUI
4. âœ… Convertir les crates services en bibliothÃ¨ques (`lib.rs` uniquement)

### Phase 2 : Infrastructure Tauri âœ… TERMINÃ‰E

5. âœ… CrÃ©er le dossier `apps/`
6. âœ… Initialiser `apps/central/` avec Tauri 2.0
7. âœ… Configurer le monorepo pnpm avec `pnpm-workspace.yaml`
8. â³ CrÃ©er le package `packages/ui/` avec Tailwind + shadcn/ui (futur)

### Phase 3 : Miyukini Central âœ… TERMINÃ‰E

9. âœ… ImplÃ©menter les commandes Tauri dans `apps/central/src-tauri/`
10. âœ… CrÃ©er les Ã©crans React (Dashboard, Catalogue, ServiceDetail, Settings)
11. âœ… IntÃ©grer la navigation et le routage (react-router-dom)
12. â³ Tester le build et le packaging

### Phase 4 : Services (itÃ©ratif)

13. â³ JayXpose : commandes + Ã©crans
14. â³ JayFestival : commandes + Ã©crans
15. â³ JayKoa : commandes + Ã©crans
16. â³ JayKonta : commandes + Ã©crans
17. â³ MiyuClicker : commandes + Ã©crans

### Phase 5 : Finalisation

18. â³ Documentation utilisateur
19. â³ Tests d'intÃ©gration
20. â³ Migration des donnÃ©es existantes

---

## 5. Fichiers Ã  supprimer

### miyukini-central

```
crates/miyukini-central/src/main.rs  (sera remplacÃ© par Tauri)
```

**DÃ©pendances eGUI Ã  retirer :**
- `egui = "0.33"`
- `eframe = { version = "0.33", ... }`
- `egui-phosphor = "0.11"`

### jayfestival

Pas de fichiers UI spÃ©cifiques, seulement les dÃ©pendances.

### jaykoa

Pas de fichiers UI spÃ©cifiques, seulement les dÃ©pendances.

### jaykonta

Pas de fichiers UI spÃ©cifiques, seulement les dÃ©pendances.

### jayxpose

Pas de fichiers UI spÃ©cifiques, seulement les dÃ©pendances.

### miyuclicker

```
crates/miyuclicker/src/app.rs         (UI eGUI)
crates/miyuclicker/src/ui_assets.rs   (assets UI)
crates/miyuclicker/src/main.rs        (sera remplacÃ©)
```

---

## 6. Conservation de lord_of_the_castle

Le crate `lord_of_the_castle` (MiyukiniSurvivor) reste **inchangÃ©** avec eGUI.

Il continuera Ã  Ãªtre accessible depuis Central via le mÃ©canisme actuel (spawn de fenÃªtre sÃ©parÃ©e ou intÃ©gration future).

---

## 7. RÃ©fÃ©rences

- [Tauri 2.0 Documentation](https://tauri.app/)
- [shadcn/ui](https://ui.shadcn.com/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Types de Services et Espaces](..//miyukini-webway-system//reference//_index.md)

---

---

## 8. Instructions de dÃ©marrage

### PrÃ©requis

- Node.js 20+
- pnpm 9+
- Rust 1.75+
- Tauri CLI 2.0

### Installation

```bash
# Installer les dÃ©pendances Node.js
pnpm install

# DÃ©veloppement
pnpm tauri:dev

# Build production
pnpm tauri:build
```

### Structure crÃ©Ã©e

```
apps/central/
â”œâ”€â”€ src-tauri/           # Backend Tauri (Rust)
â”‚   â”œâ”€â”€ Cargo.toml
â”‚   â”œâ”€â”€ tauri.conf.json
â”‚   â””â”€â”€ src/
â”‚       â”œâ”€â”€ main.rs
â”‚       â”œâ”€â”€ lib.rs
â”‚       â””â”€â”€ commands/mod.rs
â”œâ”€â”€ src/                 # Frontend React
â”‚   â”œâ”€â”€ main.tsx
â”‚   â”œâ”€â”€ App.tsx
â”‚   â”œâ”€â”€ styles/globals.css
â”‚   â”œâ”€â”€ components/
â”‚   â”‚   â”œâ”€â”€ Layout.tsx
â”‚   â”‚   â””â”€â”€ ServiceCard.tsx
â”‚   â””â”€â”€ pages/
â”‚       â”œâ”€â”€ Dashboard.tsx
â”‚       â”œâ”€â”€ Catalogue.tsx
â”‚       â”œâ”€â”€ ServiceDetail.tsx
â”‚       â””â”€â”€ Settings.tsx
â”œâ”€â”€ package.json
â”œâ”€â”€ tsconfig.json
â”œâ”€â”€ vite.config.ts
â”œâ”€â”€ tailwind.config.js
â””â”€â”€ postcss.config.js
```

---

**Date de crÃ©ation :** 2026-02-08  
**Date de mise Ã  jour :** 2026-02-08  
**Version :** 1.1  
**Statut :** Phase 1-3 TERMINÃ‰ES â€” Phase 4-5 en attente

