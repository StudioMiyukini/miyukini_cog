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
| **Moteur** | Bevy 2D (fork Miyukini COG) |
| **Résolution** | 1280×720 |
| **App** | `apps/allumina` |

---

## 5. Références

| Document | Rôle |
|----------|------|
| [Allumina - Combat et Troupes](./Concept/Allumina%20-%20Combat%20et%20Troupes.md) | Échelles, voies, ordres tactiques. |
| [Allumina - Document Fondateur](./Allumina%20-%20Document%20Fondateur.md) | Vision service, MWS, Lois d'Autonomie. |

---

**Document** : Allumina — Prototype Premier Playable  
**Version** : 1.0  
**Date** : 2026-02-17
