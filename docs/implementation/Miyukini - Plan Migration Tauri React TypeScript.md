# Miyukini — Plan de Migration vers Tauri + React/TypeScript + Tailwind

## Contexte

Migration de l'UI eGUI vers **Tauri 2.0 + React/TypeScript + Tailwind CSS** pour obtenir un rendu moderne style Steam/Epic/GoG tout en respectant les LOI du COG.

**Date :** 2026-02-08  
**Statut :** Plan validé, en cours d'exécution

---

## 1. Services concernés

### Services à migrer (suppression eGUI)

| Service | Crate | Logique métier à conserver | UI à supprimer |
|---------|-------|---------------------------|----------------|
| **Miyukini Central** | `miyukini-central` | `auth/`, `services/`, `config.rs` | `app.rs`, dépendances eGUI |
| **JayFestival** | `jayfestival` | `auth/`, `data/`, `services/` | dépendances eGUI |
| **JayKoa** | `jaykoa` | `data/`, `export/`, `services/` | dépendances eGUI |
| **JayKonta** | `jaykonta` | `backend/`, `data/`, `domain/`, `integrations/` | dépendances eGUI |
| **JayXpose** | `jayxpose` | `auth/`, `data/`, `governance.rs` | dépendances eGUI |
| **MiyuClicker** | `miyuclicker` | `carte.rs`, `combat.rs`, `idlesim.rs`, `save.rs`, `state.rs` | `app.rs`, `ui_assets.rs`, dépendances eGUI |

### Service conservé avec eGUI

| Service | Crate | Raison |
|---------|-------|--------|
| **MiyukiniSurvivor** | `lord_of_the_castle` | Jeu conservé en eGUI (demande utilisateur) |

---

## 2. Nouvelle architecture

### Structure proposée

```
Miyukini_COG/
├── crates/                          # Backend Rust (inchangé pour la logique)
│   ├── miyukini-central/            # Backend Central (Tauri commands)
│   ├── jayfestival/                 # Backend JayFestival (lib)
│   ├── jaykoa/                      # Backend JayKoa (lib)
│   ├── jaykonta/                    # Backend JayKonta (lib)
│   ├── jayxpose/                    # Backend JayXpose (lib)
│   ├── miyuclicker/                 # Backend MiyuClicker (lib)
│   └── lord_of_the_castle/          # CONSERVÉ EN EGUI
│
├── apps/                            # Applications Tauri
│   ├── central/                     # Miyukini Central (app principale)
│   │   ├── src-tauri/               # Backend Tauri (Rust)
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── main.rs
│   │   │       ├── commands/        # Commandes exposées au frontend
│   │   │       └── lib.rs
│   │   ├── src/                     # Frontend React
│   │   │   ├── App.tsx
│   │   │   ├── components/
│   │   │   ├── pages/
│   │   │   ├── hooks/
│   │   │   └── styles/
│   │   ├── package.json
│   │   ├── tailwind.config.js
│   │   ├── tsconfig.json
│   │   └── vite.config.ts
│   │
│   └── portal/                      # Miyukini Web Portal (futur)
│       └── ...
│
└── packages/                        # Packages partagés (monorepo pnpm)
    ├── ui/                          # Composants UI partagés
    │   ├── src/
    │   │   ├── components/
    │   │   ├── hooks/
    │   │   └── styles/
    │   └── package.json
    └── types/                       # Types TypeScript partagés
        └── package.json
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

## 3. Conformité LOI

| LOI | Conformité | Justification |
|-----|------------|---------------|
| **LOI-1** (Pas de dép externe critique) | ✅ | WebView natif de l'OS, tout bundlé localement |
| **LOI-2** (Isolation comme état normal) | ✅ | Fonctionne 100% offline, pas de serveur externe |
| **LOI-3** (État local souverain) | ✅ | Backend Rust avec KindMother |
| **LOI-4** (Pas de temps global) | ✅ | Horloge locale (MiyuClock) |
| **LOI-5** (Coût proportionnel hardware) | ✅ | Bundle 2-3 MB, RAM ~30 MB |
| **LOI-6** (Fédération possible) | ✅ | Webway inchangé |
| **LOI-7** (Cores immuables) | ✅ | Aucun impact sur les Cores |
| **LOI-8** (Migration = diplomatie) | ✅ | Aucun impact |

---

## 4. Étapes de migration

### Phase 1 : Préparation ✅ TERMINÉE

1. ✅ Créer ce plan de migration
2. ✅ Supprimer les dépendances eGUI des services concernés
3. ✅ Supprimer les fichiers UI eGUI
4. ✅ Convertir les crates services en bibliothèques (`lib.rs` uniquement)

### Phase 2 : Infrastructure Tauri ✅ TERMINÉE

5. ✅ Créer le dossier `apps/`
6. ✅ Initialiser `apps/central/` avec Tauri 2.0
7. ✅ Configurer le monorepo pnpm avec `pnpm-workspace.yaml`
8. ⏳ Créer le package `packages/ui/` avec Tailwind + shadcn/ui (futur)

### Phase 3 : Miyukini Central ✅ TERMINÉE

9. ✅ Implémenter les commandes Tauri dans `apps/central/src-tauri/`
10. ✅ Créer les écrans React (Dashboard, Catalogue, ServiceDetail, Settings)
11. ✅ Intégrer la navigation et le routage (react-router-dom)
12. ⏳ Tester le build et le packaging

### Phase 4 : Services (itératif)

13. ⏳ JayXpose : commandes + écrans
14. ⏳ JayFestival : commandes + écrans
15. ⏳ JayKoa : commandes + écrans
16. ⏳ JayKonta : commandes + écrans
17. ⏳ MiyuClicker : commandes + écrans

### Phase 5 : Finalisation

18. ⏳ Documentation utilisateur
19. ⏳ Tests d'intégration
20. ⏳ Migration des données existantes

---

## 5. Fichiers à supprimer

### miyukini-central

```
crates/miyukini-central/src/main.rs  (sera remplacé par Tauri)
```

**Dépendances eGUI à retirer :**
- `egui = "0.33"`
- `eframe = { version = "0.33", ... }`
- `egui-phosphor = "0.11"`

### jayfestival

Pas de fichiers UI spécifiques, seulement les dépendances.

### jaykoa

Pas de fichiers UI spécifiques, seulement les dépendances.

### jaykonta

Pas de fichiers UI spécifiques, seulement les dépendances.

### jayxpose

Pas de fichiers UI spécifiques, seulement les dépendances.

### miyuclicker

```
crates/miyuclicker/src/app.rs         (UI eGUI)
crates/miyuclicker/src/ui_assets.rs   (assets UI)
crates/miyuclicker/src/main.rs        (sera remplacé)
```

---

## 6. Conservation de lord_of_the_castle

Le crate `lord_of_the_castle` (MiyukiniSurvivor) reste **inchangé** avec eGUI.

Il continuera à être accessible depuis Central via le mécanisme actuel (spawn de fenêtre séparée ou intégration future).

---

## 7. Références

- [Tauri 2.0 Documentation](https://tauri.app/)
- [shadcn/ui](https://ui.shadcn.com/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Types de Services et Espaces](../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md)

---

---

## 8. Instructions de démarrage

### Prérequis

- Node.js 20+
- pnpm 9+
- Rust 1.75+
- Tauri CLI 2.0

### Installation

```bash
# Installer les dépendances Node.js
pnpm install

# Développement
pnpm tauri:dev

# Build production
pnpm tauri:build
```

### Structure créée

```
apps/central/
├── src-tauri/           # Backend Tauri (Rust)
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       └── commands/mod.rs
├── src/                 # Frontend React
│   ├── main.tsx
│   ├── App.tsx
│   ├── styles/globals.css
│   ├── components/
│   │   ├── Layout.tsx
│   │   └── ServiceCard.tsx
│   └── pages/
│       ├── Dashboard.tsx
│       ├── Catalogue.tsx
│       ├── ServiceDetail.tsx
│       └── Settings.tsx
├── package.json
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.js
└── postcss.config.js
```

---

**Date de création :** 2026-02-08  
**Date de mise à jour :** 2026-02-08  
**Version :** 1.1  
**Statut :** Phase 1-3 TERMINÉES — Phase 4-5 en attente
