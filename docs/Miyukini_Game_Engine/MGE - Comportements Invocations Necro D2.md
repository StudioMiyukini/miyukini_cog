# MGE — Comportements Invocations (inspiration Necromancien D2)

Référence et spécifications pour les invocations type Skeleton Warriors du Necromancien (Diablo II) : formation idle, chase, anti-stack, états de comportement.

---

## 1. Vue d'ensemble

Les invocations (skeletons, golems, etc.) présentent des comportements caractéristiques :

| Comportement | Description | Triggers |
|--------------|-------------|----------|
| **Idle** | Formation autour du maître, légère errance | Pas d'ennemi en vision |
| **Chase** | Pathfinding vers l'ennemi | Ennemi détecté dans champ de vision |
| **Combat** | Attaque au contact | Intersection AABB avec ennemi |
| **Return** | Retour au maître / spawn | Ennemi mort ou hors vision |

---

## 2. Formation Idle

### 2.1 Principe

En absence d'ennemi, les invocations maintiennent une **formation circulaire** autour du maître (joueur/Necromancien).

- **Rayon** : ~50 px (paramétrable)
- **Répartition** : Un slot par invocation, angle uniforme sur 360°
- **Position cible** : `master_pos + (radius * cos(angle), radius * sin(angle))`
- **Errance** : Optionnel — léger offset aléatoire ou déplacement lent pour effet organique

### 2.2 Calcul des slots

```
angle_slot_i = (2 * PI * i) / n  +  offset_temps
position_cible_i = master_pos + Vec2(radius * cos(angle_slot_i), radius * sin(angle_slot_i))
```

- `i` : index de l'invocation (0..n-1)
- `n` : nombre total d'invocations
- `offset_temps` : légère rotation dans le temps (ex. 0.1 rad/s) pour éviter positions figées

### 2.3 Pathfinding

Chaque invocation calcule un chemin A* vers sa **position cible** (slot sur le cercle). Recalcul périodique (ex. 0,5 s) ou lorsque le maître a bougé significativement.

---

## 3. Chase (poursuite de l'ennemi)

### 3.1 Détection

- **Champ de vision** : Rayon autour de l'invocation (ex. 150 px)
- **Condition** : `distance(invocation, ennemi) <= VISION_RANGE`
- **Comportement global** : Si **au moins une** invocation voit l'ennemi, toutes les invocations passent en état Chase (ou chaque invoquation peut décider individuellement)

### 3.2 Objectif

- **Cible** : Position de l'ennemi (ou cellule la plus proche)
- **Pathfinding** : A* de la position de l'invocation vers l'ennemi
- **Recalcul** : Toutes les 0,3–0,5 s ou lorsque l'ennemi change de cellule

### 3.3 Sortie Chase

- Transition vers **Combat** si contact (AABB intersect)
- Transition vers **Return** si ennemi mort ou hors vision pendant un délai

---

## 4. Anti-stack (séparation douce)

### 4.1 Problème

Sans contrainte, les invocations tendent à se **superposer** (stacking) sur la même cellule, rendant le groupe illisible et peu réaliste.

### 4.2 Solution : Force de séparation (pas de MTV)

**Principe** : Appliquer une **force de répulsion** entre invocations trop proches, sans collision physique (pas de résolution MTV).

| Paramètre | Valeur typique | Description |
|-----------|----------------|-------------|
| `SEPARATION_DIST` | 15 px | Distance en dessous de laquelle la force s'applique |
| `SEPARATION_STRENGTH` | 1.0–2.0 | Intensité de la poussée |
| `MAX_DISPLACEMENT` | 3–5 px | Déplacement max par frame pour éviter saccades |

### 4.3 Formule

Pour chaque paire (invocation A, invocation B) :

```
d = distance(A, B)
si d < SEPARATION_DIST && d > 0 :
    direction = (A.pos - B.pos).normalize()
    force = direction * (SEPARATION_STRENGTH * (1 - d/SEPARATION_DIST))
    A.velocity_offset += force
```

La force est **proportionnelle** à la proximité : plus on est proche, plus la répulsion est forte.

### 4.4 Intégration au mouvement

- Calculer le vecteur de séparation **avant** ou **après** le déplacement selon waypoints
- Appliquer le déplacement de séparation comme un **offset** limité par frame
- Ne pas remplacer le pathfinding, seulement **corriger** la position pour éviter le chevauchement visuel

---

## 5. États et machine d'états

### 5.1 Diagramme

```
     [Idle] <------------+ 
         |               |
         | ennemi vu     | ennemi mort / hors vision
         v               |
     [Chase] ------------+
         |
         | contact AABB
         v
     [Combat]
```

### 5.2 Transitions

| État   | Condition entrante       | Condition sortante           |
|--------|--------------------------|------------------------------|
| Idle   | Début, Return terminé     | Ennemi dans vision            |
| Chase  | Ennemi vu                | Contact AABB → Combat ; hors vision → Return |
| Combat | Contact avec ennemi      | Ennemi mort ; hors contact   |
| Return | Ennemi mort / hors vision| Atteint formation idle       |

### 5.3 Return

- **Objectif** : Retourner vers le maître (position de formation)
- Identique à Idle : chemin vers `master_pos + offset_slot`
- Une fois position atteinte → état Idle

---

## 6. Paramètres recommandés (démo)

| Paramètre            | Valeur   | Unité |
|----------------------|----------|-------|
| Nombre d'invocations | 5        | —     |
| Rayon formation idle| 50       | px    |
| Champ de vision      | 150      | px    |
| Distance séparation  | 15       | px    |
| Force séparation     | 1.5      | —     |
| Recalcul path idle  | 0.5      | s     |
| Recalcul path chase | 0.4      | s     |
| Vitesse invocations | 0.85× joueur | — |

---

## 7. Références

- [MGE - Pathfinding Collisions - Guide Entites Groupes](MGE%20-%20Pathfinding%20Collisions%20-%20Guide%20Entites%20Groupes.md) — Formations, flow fields, évitement
- [MGE - Hitbox et Collisions - Reference](MGE%20-%20Hitbox%20et%20Collisions%20-%20Reference.md) — AABB, MTV
- [points/21-ia-bots/pathfinding](points/21-ia-bots/pathfinding.md) — IA et navigation

---

**Document** : MGE — Comportements Invocations Necro D2  
**Version** : 1.0  
**Date** : 2026-02-18
