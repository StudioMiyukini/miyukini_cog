# Miyukini Survivor — Bestiaire et métriques ennemis

## Contexte

Ce document décrit le **bestiaire** du jeu Lord of the Castle (Miyukini Survivor) : les types d’ennemis, leurs statistiques et les règles d’apparition. Il sert de référence pour le design, l’équilibrage et l’implémentation.

**Source de vérité code :** `crates/lord_of_the_castle/src/enemies.rs` et `crates/lord_of_the_castle/src/constants.rs`.

---

## Portée / Scope

- **Applicable à :** Lord of the Castle (Miyukini Survivor)
- **Contenu :** Types d’ennemis, tableaux de stats, formules de PV/vitesse, règles de spawn
- **Statut :** Référence alignée sur le code

---

## 1. Vue d’ensemble des types d’ennemis

Le jeu distingue **trois types** d’ennemis :

| Type        | Rôle principal              | Apparition                          |
|------------|-----------------------------|-------------------------------------|
| **Normal** | Foule, pression continue    | Par défaut à chaque spawn           |
| **MiniBoss** | Unité renforcée, plus résistante | ~15 % de chance par ennemi spawné   |
| **Boss**   | Élite de vague, très résistant | 1 par vague tous les 10 niveaux (vague 10, 20, 30…) |

---

## 2. Tableau récapitulatif des métriques

Toutes les métriques par type, telles que définies dans le code.

| Métrique            | Unité   | Normal | MiniBoss | Boss |
|---------------------|--------|--------|----------|------|
| **Taille (côté)**   | px     | 10     | 20       | 30   |
| **Vitesse de base** | px/s   | 16     | 8        | 6    |
| **Vitesse max**     | px/s   | 50     | 40       | 30   |
| **Dégâts au contact** | dégâts/collision | 1 | 3 | 10 |
| **PV max (formule)** | —      | ½×C    | 2,5×C    | 10×C |

**Légende :** C = Constitution du joueur au moment du spawn (`player_constitution()`).  
Formule PV de base : `normal_hp = (C / 2).max(1)` ; Normal = normal_hp, MiniBoss = 5×normal_hp, Boss = 20×normal_hp.

---

## 3. Détail par type

### 3.1 Ennemi Normal

| Propriété   | Valeur / Formule |
|------------|-------------------|
| **Taille** | 10×10 px (`size::MOBILE`) |
| **Vitesse** | `16 × (1 + vague/100)` px/s, plafonnée à **50** px/s |
| **Dégâts contact** | 1 par collision |
| **PV max** | `(Constitution / 2).max(1)` |

**Description :** Unité standard. Vitesse la plus élevée, faible résistance et dégâts au contact. Représente la majorité des spawns.

---

### 3.2 MiniBoss

| Propriété   | Valeur / Formule |
|------------|-------------------|
| **Taille** | 20×20 px (`size::MINI_BOSS`) |
| **Vitesse** | `8 × (1 + vague/100)` px/s, plafonnée à **40** px/s |
| **Dégâts contact** | 3 par collision |
| **PV max** | `5 × (Constitution / 2).max(1)` |

**Description :** Ennemi renforcé, plus lent mais plus résistant et plus dangereux au contact. Environ 15 % de chance d’apparition à chaque spawn (hors vague « boss »).

---

### 3.3 Boss

| Propriété   | Valeur / Formule |
|------------|-------------------|
| **Taille** | 30×30 px (`size::BOSS`) |
| **Vitesse** | `6 × (1 + vague/100)` px/s, plafonnée à **30** px/s |
| **Dégâts contact** | 10 par collision |
| **PV max** | `20 × (Constitution / 2).max(1)` |

**Description :** Élite de vague. Apparaît **une fois par vague** lorsque `vague % 10 == 0` (vagues 10, 20, 30…), en premier spawn de la vague. Très résistant et très dangereux au contact.

---

## 4. Constantes globales (ennemis)

| Constante                 | Valeur | Description |
|---------------------------|--------|-------------|
| **Champ de vision**       | 60 px  | Rayon pour cibler joueur / tour / château |
| **Durée clignotement dégâts** | 0,2 s | Durée du flash visuel après un coup reçu |

---

## 5. Règles de spawn

| Règle | Détail |
|-------|--------|
| **Qui décide du type** | Pour chaque ennemi créé : si vague multiple de 10 et premier de la vague → Boss ; sinon si `rand() < 0.15` → MiniBoss ; sinon → Normal. |
| **PV au spawn** | PV max = `kind.hp_max_from_constitution(player_constitution())`. |
| **Constitution joueur** | `(10 + Con).max(1)` (stats effectives). |

---

## 6. Tableau des constantes code (référence)

Aligné sur `constants.rs` et `enemies.rs`.

| Constante | Valeur |
|-----------|--------|
| `size::MOBILE` | 10 |
| `size::MINI_BOSS` | 20 |
| `size::BOSS` | 30 |
| `speed::ENEMY_NORMAL_BASE` | 16 |
| `speed::ENEMY_NORMAL_MAX` | 50 |
| `speed::ENEMY_MINI_BOSS_BASE` | 8 |
| `speed::ENEMY_MINI_BOSS_MAX` | 40 |
| `speed::ENEMY_BOSS_BASE` | 6 |
| `speed::ENEMY_BOSS_MAX` | 30 |
| `combat::ENEMY_CONTACT_NORMAL` | 1 |
| `combat::ENEMY_CONTACT_MINI_BOSS` | 3 |
| `combat::ENEMY_CONTACT_BOSS` | 10 |
| `ENEMY_VISION_RADIUS` | 60 |
| `DAMAGE_FLASH_DURATION_S` (enemies.rs) | 0.2 |

---

## 7. Mécaniques annexes (résumé)

- **Vitesse dynamique :** `vitesse = base × (1 + vague/100)`, plafonnée par type (voir tableau).
- **Cible :** Ennemi choisit joueur, tour ou château selon distance / règles du game loop.
- **Dégâts reçus :** `take_damage(damage)` ; optionnel `push_back_from` et `set_damage_flash` pour feedback.
- **Hitbox :** Demi-taille = `kind.size() / 2` pour collisions.

---

**Date :** 2026-02-06  
**Version :** 1.0  
**Alignement :** `enemies.rs`, `constants.rs`, `game_state.rs` (spawn_enemies, player_constitution)
