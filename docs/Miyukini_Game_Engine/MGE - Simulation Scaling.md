# MGE — Simulation Scaling

LOD comportemental, gestion de masse, désactivation adaptative et budget CPU pour les simulations à grande échelle.

## Contexte

Le MGE vise à supporter des mondes avec des milliers d'entités : batailles, MMO-lite, simulation sociale. Le scaling repose sur un LOD comportemental, une désactivation adaptative et un budget CPU explicite.

## Portée / Scope

- **Applicable à :** Conception des systèmes, plugins physics/ai, optimisation.
- **Audience :** Développeurs moteur, développeurs de jeux.
- **Statut :** Spécification normative.

---

## 1. LOD comportemental

### 1.1 Trois niveaux

| Niveau | Nom | Description | Coût |
|--------|-----|-------------|------|
| **Full** | Actif complet | Tous les systèmes s'exécutent (physique, IA, scripts). | 100 % |
| **Reduced** | Réduit | Seulement mouvement basique, pas d'IA détaillée, pas de pathfinding. | ~20 % |
| **Sleep** | Dormant | Aucune mise à jour ; entité figée jusqu'à réveil. | 0 % |

### 1.2 Transitions

- **Full → Reduced** : Entité sort de la zone « active » (distance au joueur ou à la caméra).
- **Reduced → Sleep** : Entité trop loin ; hors chunk chargé ; hors zone d'intérêt.
- **Sleep → Reduced** : Entité rentre dans une zone chargée ou un joueur s'approche.
- **Reduced → Full** : Entité rentre dans la zone active (ex. à l'écran, ou proche du joueur).

### 1.3 Composant LOD

```rust
/// Niveau de détail comportemental d'une entité
pub enum BehaviorLod {
    Full,     // Simulation complète
    Reduced,  // Mouvement basique, pas d'IA
    Sleep,    // Aucune mise à jour
}
```

- Le scheduler parcourt les entités par LOD ; les systèmes peuvent skip les entités Sleep ou Reduced selon leurs besoins.

---

## 2. Gestion de masse

### 2.1 Batching

- Les systèmes itèrent par archetype : toutes les entités avec les mêmes composants sont contiguës.
- Pas d'itération entité par entité avec lookups ; accès séquentiel aux tableaux.
- Réduit les cache misses et maximise le throughput.

### 2.2 Spatial partitioning

- **Grille spatiale** : le monde est découpé en cellules (ex. 64×64 px).
- Les entités sont indexées par cellule pour les queries de proximité.
- Mise à jour incrémentale : quand une entité bouge, mise à jour de la cellule.
- Usage : collision broadphase, culling, réveil des entités dormantes.

### 2.3 Cibles de performance

| Cible | Valeur | Condition |
|-------|--------|-----------|
| Entités simulées (Full) | 10 000+ | Sur hardware moyen |
| Entités dormantes (Sleep) | 100 000+ | Présentes dans le World, pas mises à jour |
| Tick rate | 60 ticks/s | Budget tick ~8 ms (le reste pour rendu si 60 FPS) |
| Headless | 1000+ ticks/s | Serveur dédié, sans rendu |

---

## 3. Désactivation adaptative

### 3.1 Zones d'intérêt

- **Zone active** : autour du joueur ou de la caméra (rayon configurable).
- **Zone chargée** : chunks chargés (monde divisé en chunks).
- Les entités hors zone active passent en Reduced puis Sleep.

### 3.2 Réveil

- **Événement** : une entité Sleep peut être réveillée par un événement (ex. projectile qui arrive, joueur qui entre dans le chunk).
- **Périodique** : sweep occasionnel pour réveiller les entités proches des zones actives (coût amorti).

### 3.3 Pool dormant

- Les entités Sleep restent dans le World mais ne sont pas parcourues par les systèmes Full/Reduced.
- Stockage séparé ou marquage ; le scheduler skip les itérations pour ces entités.

---

## 4. Budget CPU

### 4.1 Tick budget

- Temps max par tick de simulation (ex. 8 ms). Le core ne connaît que le tick ; le budget frame (rendu) est dans le plugin rendu.
- Si dépassement : log warning, option de skip tick ou continuer (configurable).
- Le temps est mesuré par le scheduler.

### 4.2 System budget (optionnel)

- Temps max par système (ex. physics < 4 ms).
- Si dépassement : truncation ou warning.
- Utile pour identifier les systèmes coûteux.

### 4.3 Overflow handling

| Mode | Comportement |
|------|--------------|
| **Continue** | Ignorer le dépassement, continuer. Latence possible. |
| **Skip tick** | Abandonner le tick en cours, reprendre au suivant. |
| **Scale down** | Réduire le LOD des entités les plus lointaines pour libérer du temps. |

---

## 5. Intégration avec le Scheduler

### 5.1 Phases et LOD

- En phase Physics : traiter Full + Reduced (mouvement).
- En phase Logic (IA) : traiter Full uniquement.
- En phase PreRender : culling sur Full + Reduced visibles.

### 5.2 Profiling

- Métriques par système : temps d'exécution, nombre d'entités traitées.
- Métriques par LOD : nombre d'entités Full / Reduced / Sleep.
- Exposé via hooks du scheduler.

---

## 6. Exemples de configuration

```rust
/// Configuration du scaling (conceptuel)
pub struct ScalingConfig {
    /// Rayon zone active (Full) en unités monde
    pub active_radius: f32,
    /// Rayon zone chargée (Reduced) en unités monde
    pub loaded_radius: f32,
    /// Taille cellule grille spatiale
    pub spatial_cell_size: u32,
    /// Budget tick en ms (simulation uniquement)
    pub tick_budget_ms: u32,
    /// Comportement en cas de dépassement
    pub overflow_mode: OverflowMode,
}
```

---

## 7. Références

| Document | Rôle |
|----------|------|
| [MGE - Core Specification Technique](./MGE%20-%20Core%20Specification%20Technique.md) | Scheduler, budget, profiling. |
| [MGE - Performance Philosophy](./MGE%20-%20Performance%20Philosophy.md) | SoA, batch, spatial hash, cache. |
| [MGE - Référence Commune](./reference/MGE%20-%20Reference%20Commune.md) | Types, coordonnées. |

---

**Document** : MGE — Simulation Scaling  
**Version** : 1.0  
**Date** : 2026-02-19  
**Statut** : Spécification normative
