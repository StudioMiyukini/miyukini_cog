# Allumina — Prototype Premier Playable

## Contexte

Ce document définit le **premier playable** multijoueur d'Allumina : objectif, périmètre minimal et priorités de développement.

## Portée / Scope

- **Applicable à :** Plan de développement, MVP, sprint 0.
- **Audience :** Équipe produit, développement Bevy.
- **Statut :** Spécification du prototype.

---

## 1. Objectif

**Base multijoueur fonctionnelle** : démontrer que deux joueurs peuvent jouer ensemble, avec troupes et monstres synchronisés, en coopération PvE.

---

## 2. Périmètre minimal

| Élément | Inclus |
|---------|--------|
| **Joueurs** | 2 joueurs |
| **Carte** | Même carte partagée |
| **Synchronisation** | Position, déplacement, troupes, monstres |
| **Combat** | PvE partagé (co-op) |
| **Mode** | Co-op (pas de PvP pour ce prototype) |

---

## 3. Priorité de développement

Par quoi commencer : **le combat avec troupe** (sensation de bataille).

Une proto-fonction existe dans le service **MiyukiniSurvivor** (IA, recrutement, combat) ; elle sert de référence. Le code Allumina sera **réécrit en Bevy**.

---

## 4. Stack technique

| Couche | Choix |
|--------|-------|
| **Moteur** | MGE (Miyukini Game Engine) — wgpu, ECS SoA |
| **Résolution** | 1280×720 |
| **App** | `mge/examples/allumina_prototype` |

---

## 5. Contrôles

| Action | Touches |
|--------|---------|
| **Déplacement joueur** | `Z Q S D` (AZERTY) ou `↑ ← ↓ →` (flèches) |
| **Sélectionner entité** | Clic gauche (rayon ≤ 1.5 tiles) |
| **Balise de ralliement** | Clic droit → déplace le joueur par A* |
| **Zoom** | Molette souris (0.5× – 4×) |
| **Grille isométrique** | `G` |
| **Overlay stats** | `F3` |
| **Quitter** | `Échap` |

### Mapping isométrique ZQSD

```
Touches → Direction monde
  Z / ↑   =  NW  (-1, -1)   (haut-gauche écran)
  S / ↓   =  SE  (+1, +1)   (bas-droite écran)
  Q / ←   =  SW  (-1, +1)   (bas-gauche écran)
  D / →   =  NE  (+1, -1)   (haut-droite écran)
  Diagonales → normalisées automatiquement
```

### Comportement sélection

- Clic gauche sans entité à proximité → désélectionne.
- Entité sélectionnée affiche un ovale cyan semi-transparent sous les pieds.
- La sélection se vide automatiquement si le mob meurt.
- Le joueur peut être sélectionné pour voir ses stats en détail.

---

## 6. Références

| Document | Rôle |
|----------|------|
| [Allumina - Combat et Troupes](./Concept/Allumina%20-%20Combat%20et%20Troupes.md) | Échelles, voies, ordres tactiques. |
| [Allumina - Document Fondateur](./Allumina%20-%20Document%20Fondateur.md) | Vision service, MWS, Lois d'Autonomie. |

---

**Document** : Allumina — Prototype Premier Playable
**Version** : 1.1
**Date** : 2026-02-23
