---
name: Enrichissement documentation MGE
overview: Enrichir chaque document de point MGE pour atteindre au moins 500 lignes, en priorisant les catégories MVP (01–05), en centralisant les définitions, et en respectant le glossaire Miyukini.
todos:
  - id: P0-1
    content: Phase 0 - Créer MGE - Reference Commune.md (~600-800 lignes)
    status: completed
  - id: P1-01
    content: Phase 1 - Enrichir 01-affichage-rendu (8 points)
    status: completed
  - id: P1-02
    content: Phase 1 - Enrichir 02-physique-collisions (3 points)
    status: completed
  - id: P1-03
    content: Phase 1 - Enrichir 03-deplacement-locomotion (14 points)
    status: completed
  - id: P1-04
    content: Phase 1 - Enrichir 04-entites-monde (13 points)
    status: completed
  - id: P1-05
    content: Phase 1 - Enrichir 05-joueur-personnage (8 points)
    status: completed
  - id: P2-1
    content: Phase 2 - Enrichir 5 points système MVP
    status: completed
  - id: P3-1
    content: Phase 3 - Enrichir catégories 06→24 (~210 points)
    status: completed
  - id: TX-1
    content: Transversal - Lien Reference Commune dans Reference Technique
    status: completed
isProject: false
---

# Plan d'enrichissement de la documentation MGE

## Contexte

- **Objectif :** Chaque document de point atteint au moins 500 lignes
- **Priorité :** MVP (catégories 01–05) puis tous les points
- **Centralisation :** Définitions partagées dans un document de référence unique ; les points y font référence
- **Langue :** Français + anglicismes pertinents (frame rate, hitbox, culling, etc.) ; respect du [glossaire Miyukini](.cursor/skills/miyukini-glossary/SKILL.md) (KindMother, MWS, Opérateur, etc.)
- **Pertinence :** Contenu adapté à chaque point (specs, diagrammes, API, exemples)

---

## Phase 0 : Document de référence centralisé

Créer [docs/Miyukini_Game_Engine/MGE - Reference Commune.md](docs/Miyukini_Game_Engine/MGE%20-%20Reference%20Commune.md) (~600–800 lignes) contenant :

- **Types et structures communs** : `Vec2`, `Resolution`, `Rect`, `ScaleFactor`, `LayerId`
- **Coordonnées** : systèmes monde/écran/UI (référencé par coordonnées, caméra, hitbox)
- **Cycle de rendu** : pipeline (référencé par affichage, boucle)
- **Glossaire MGE** : termes moteur (sprite, chunk, entity, prefab, etc.) avec liens vers le glossaire Miyukini (KindMother, MWS) quand pertinent
- **Conventions** : nommage, unités (px, tiles), formats

Ce document évite la duplication entre les ~270 points.

---

## Phase 1 : Enrichissement MVP (catégories 01–05)

**46 points** à enrichir en premier.

### Structure type pour chaque point (500+ lignes)


| Section                   | Lignes cible | Contenu                                                                                                                                          |
| ------------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| En-tête et contexte       | 30–50        | Titre, catégorie, description ; rôle dans le moteur ; liens vers [Reference Commune](docs/Miyukini_Game_Engine/MGE%20-%20Reference%20Commune.md) |
| Spécifications techniques | 100–150      | Contraintes, formules, paramètres ; références croisées (ex. hitbox → collision)                                                                 |
| Modèle de données / API   | 80–120       | Structures Rust ou pseudo-code ; signatures principales ; liens vers central                                                                     |
| Diagrammes                | 50–100       | Mermaid (flux, états, séquences) selon le point                                                                                                  |
| Exemples et cas d'usage   | 80–120       | Allumina ou cas générique ; scénarios                                                                                                            |
| Cas limites et tests      | 50–80        | Edge cases ; critères de validation                                                                                                              |
| Références                | 20–40        | Liens vers index, autres points, doc externe                                                                                                     |


### Ordre par catégorie MVP

1. **01-affichage-rendu** (8 points) : affichage-resolution, coordonnees, gestion-sprites, animations-sprites, camera, z-order-couches, particules-effets, monde-tile-based
2. **02-physique-collisions** (3 points) : hitbox, collision, collision-layers
3. **03-deplacement-locomotion** (14 points)
4. **04-entites-monde** (13 points)
5. **05-joueur-personnage** (8 points)

### Exemple de contenu pertinent par type

- **affichage-resolution** : Résolution logique vs physique, scale factor, DPI, fullscreen (exclusif, borderless), VSync, intégration wgpu/SDL
- **hitbox** : Formes (AABB, cercle), alignement sprite, références [Reference Commune](docs/Miyukini_Game_Engine/MGE%20-%20Reference%20Commune.md) pour `Rect`
- **coordonnees** : Définition détaillée dans Reference Commune ; ce point = usage et conversion
- **donnees-joueur** : Persistance KindMother (lien glossaire), structures, sérialisation

---

## Phase 2 : Points système MVP

Enrichir les points de [23-systeme](docs/Miyukini_Game_Engine/points/23-systeme/_index.md) nécessaires au MVP :

- boucle-jeu
- entrees-utilisateur
- chargement-assets
- sauvegarde-chargement (KindMother)
- gestion-temps

---

## Phase 3 : Catégories restantes (06–24)

Enrichir les **~210 points** restants dans l’ordre des catégories :

06-progression → 07-combat → … → 24-meta-moderation

Même structure type ; contenu adapté (ex. combat : formules, CC ; inventaire : slots, poids ; réseau : MWS).

---

## Contraintes techniques

- **Liens relatifs** : `../MGE%20-%20Reference%20Commune.md` depuis les points
- **Diagrammes Mermaid** : Respecter la syntaxe (pas d’espaces dans les IDs, pas de couleurs explicites)
- **Pas de duplication** : Définitions canoniques dans Reference Commune uniquement

---

## Estimation


| Phase     | Points | Lignes/doc | Total lignes |
| --------- | ------ | ---------- | ------------ |
| 0         | 1 doc  | ~700       | 700          |
| 1         | 46     | ~550       | ~25 300      |
| 2         | 5      | ~550       | ~2 750       |
| 3         | ~210   | ~550       | ~115 500     |
| **Total** | ~262   | —          | ~144 250     |


---

## Fichiers clés

- [MGE - Reference Commune.md](docs/Miyukini_Game_Engine/MGE%20-%20Reference%20Commune.md) (à créer)
- [points/01-affichage-rendu/affichage-resolution.md](docs/Miyukini_Game_Engine/points/01-affichage-rendu/affichage-resolution.md) (premier point à enrichir)
- [MGE - Miyukini Game Engine - Reference Technique.md](docs/Miyukini_Game_Engine/MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md) : ajouter un lien vers Reference Commune dans les Références

