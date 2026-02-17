# Bevy COG — Fork Miyukini (2D uniquement)

## Référence du dépôt

- **Fork :** [https://github.com/StudioMiyukini/bevy-COG](https://github.com/StudioMiyukini/bevy-COG)
- **Upstream :** [https://github.com/bevyengine/bevy](https://github.com/bevyengine/bevy)
- **Installation locale :** dossier `bevy/` à la racine du dépôt Miyukini COG.

Ce fork est utilisé pour le jeu **Allumina** et tout projet de jeu 2D de l’écosystème Miyukini COG.

## Usage 2D uniquement

Ce fork est configuré pour **n’utiliser que le rendu 2D**. Les features de rendu 3D ont été retirées des options par défaut et ne sont pas maintenues pour ce dépôt.

- **Default features :** `2d`, `ui` (pas de `3d`).
- Pour la stack officielle desktop : **Dioxus** (apps/central) ; Bevy est réservé au jeu Allumina et aux applications 2D (Bevy 2D).

## Remotes Git

- **origin** : `https://github.com/StudioMiyukini/bevy-COG` (fork Miyukini — push/pull principal)
- **upstream** : `https://github.com/bevyengine/bevy` (Bevy officiel — pour récupérer les mises à jour)

```bash
cd bevy
git fetch origin
git fetch upstream   # pour mettre à jour depuis Bevy officiel
```

---

**Document** : Bevy COG — Référence fork  
**Date** : 2026-02-17
