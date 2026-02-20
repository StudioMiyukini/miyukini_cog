# MGE — Performance Philosophy

Principes de performance : SoA vs AoS, batch processing, spatial hashing, cache locality, pas de dynamic dispatch inutile, no hidden allocations, profiling.

## Contexte

Le MGE est conçu pour des simulations à grande échelle (10k+ entités simulées, 100k+ dormantes). La philosophie de performance est explicite : maîtrise du coût, pas de surprises, données contiguës, itération efficace.

## Portée / Scope

- **Applicable à :** Conception du World, des systèmes, des plugins, optimisations.
- **Audience :** Développeurs moteur, développeurs de jeux.
- **Statut** : Spécification normative.

---

## 1. SoA vs AoS

### 1.1 Choix : Structure of Arrays (SoA)

- Les composants sont stockés en **tableaux séparés** : un tableau par type.
- Ex. : `positions: Vec<Vec2>`, `velocities: Vec<Vec2>` au lieu de `Vec<Entity { pos, vel }>`.
- Avantage : itération séquentielle sur un seul type (Position) sans charger les autres (Velocity) ; meilleure localité de cache pour les systèmes qui ne touchent qu'un sous-ensemble.

### 1.2 Archetypes

- Les entités avec le même ensemble de composants sont groupées en **archetypes**.
- Chaque archetype a ses propres tableaux SoA.
- Itération : parcourir un archetype = parcourir des slices contiguës.

### 1.3 Éviter AoS dans le hot path

- **Array of Structures** : `Vec<Entity>` où chaque Entity contient plusieurs composants.
- Problème : cache pollution, mauvais prédicteur de branchement si les systèmes n'utilisent qu'une partie des champs.
- Le MGE n'utilise pas AoS pour le stockage des composants.

---

## 2. Batch processing

### 2.1 Itération par archetype

- Les systèmes itèrent sur tous les archetypes pertinents.
- Pour chaque archetype : boucle séquentielle sur les tableaux.
- Pas de lookup entité par entité ; pas de HashMap dans le hot path.

### 2.2 Réduction des branches

- Les boucles sont simples : peu de conditions dans l'itération interne.
- Les filtres (ex. LOD) sont appliqués en amont (séparation Full/Reduced/Sleep) ou par chunk.

### 2.3 Taille des batches

- Les archetypes avec peu d'entités ont un coût fixe (une itération) mais un coût par entité plus élevé si le système fait du travail par entité.
- Les archetypes avec beaucoup d'entités profitent du cache et du pipeline CPU.

---

## 3. Spatial hashing

### 3.1 Grille spatiale

- Le monde est découpé en cellules (ex. 64×64 px ou 128×128 px).
- Chaque entité avec une Position est indexée dans la cellule correspondante.
- Structure : `HashMap<IVec2, Vec<EntityId>>` ou équivalent compact.

### 3.2 Usage

- **Broadphase collision** : ne tester que les paires dans la même cellule ou cellules adjacentes.
- **Culling** : ne considérer que les entités dans les cellules visibles.
- **Réveil LOD** : quand une entité entre dans une cellule proche du joueur, réveiller les Sleep.

### 3.3 Mise à jour incrémentale

- Quand une entité bouge, recalculer sa cellule.
- Si cellule différente : retirer de l'ancienne, ajouter à la nouvelle.
- Éviter un rebuild complet à chaque frame.

---

## 4. Cache locality

### 4.1 Données contiguës

- Les tableaux SoA sont alloués en blocs contigus.
- Pas de pointeurs indirects vers des allocations dispersées.
- `Vec<T>` plutôt que `Vec<Box<T>>` pour les composants petits.

### 4.2 Ordre d'itération

- Parcourir les données dans l'ordre de stockage (séquentiel).
- Éviter les sauts aléatoires (ex. suivre des pointeurs vers des entités éparpillées).

### 4.3 Taille des composants

- Composants compacts : préférer `f32` à `f64` si suffisant, éviter les types larges dans le hot path.
- Composants volumineux (ex. mesh, texture) : stocker un handle (ID) plutôt que la donnée brute.

---

## 5. Pas de dynamic dispatch inutile

### 5.1 Génériques + monomorphisation

- Les systèmes et les queries sont génériques ; le compilateur génère du code spécialisé (monomorphisation).
- Pas de `dyn Trait` dans le hot path pour les itérations.
- Les plugins peuvent utiliser des traits object pour l'extension, mais le scheduler appelle des fonctions concrètes.

### 5.2 Éviter Box<dyn>

- Pas de `Vec<Box<dyn System>>` si un enum ou un tableau de fonctions concrètes suffit.
- Réduire les indirections et les cache misses.

---

## 6. No hidden allocations

### 6.1 Hot path

- Le hot path (boucle principale, systèmes appelés chaque frame) ne doit pas allouer.
- Pas de `Vec::new()` dans un système qui tourne 60 fois/s.
- Pas de String format! dans une boucle serrée.

### 6.2 Pools et arenas

- Pour les entités temporaires (projectiles, particules) : **object pool**.
- Réutilisation des EntityId et des composants désactivés.
- Arenas pour les allocations batch (ex. création de multiples entités en une fois).

### 6.3 Pre-allocation

- Pré-allouer les capacités des Vec au build ou au chargement de niveau.
- `Vec::with_capacity(n)` pour éviter les reallocations pendant le jeu.

---

## 7. Profiling

### 7.1 Hooks dans le Scheduler

- Avant/après chaque système : mesure du temps écoulé.
- Métriques : temps par système, par phase, total frame.
- Exposition : logs, métriques exportables (Prometheus, etc.), overlay debug.

### 7.2 Métriques par système

- Temps moyen, max, p95.
- Nombre d'entités traitées.
- Permet d'identifier les goulots d'étranglement.

### 7.3 Intégration

- Le scheduler appelle des hooks (optionnels) pour le profiling.
- Pas de coût si désactivé (feature flag ou no-op).

---

## 8. Résumé des principes

| Principe | Application |
|----------|-------------|
| **SoA** | Composants en tableaux séparés par type, groupés par archetype. |
| **Batch** | Itération séquentielle sur les archetypes ; pas de lookup par entité. |
| **Spatial hash** | Grille pour broadphase, culling, LOD. |
| **Cache locality** | Données contiguës, ordre séquentiel, composants compacts. |
| **Pas de dyn** | Génériques, monomorphisation, pas de Box<dyn> dans le hot path. |
| **No hidden alloc** | Pools, arenas, pre-allocation ; pas d'alloc dans les systèmes. |
| **Profiling** | Hooks scheduler, métriques par système, overlay debug. |

---

## 9. Références

| Document | Rôle |
|----------|------|
| [MGE - Core Specification Technique](./MGE%20-%20Core%20Specification%20Technique.md) | World SoA, Scheduler, profiling. |
| [MGE - Simulation Scaling](./MGE%20-%20Simulation%20Scaling.md) | LOD, budget CPU. |
| [MGE - Plugin Contract](./MGE%20-%20Plugin%20Contract.md) | Systèmes, itération. |

---

**Document** : MGE — Performance Philosophy  
**Version** : 1.0  
**Date** : 2026-02-19  
**Statut** : Spécification normative
