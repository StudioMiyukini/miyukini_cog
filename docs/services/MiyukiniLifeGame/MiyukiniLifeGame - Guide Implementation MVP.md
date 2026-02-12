# MiyukiniLifeGame — Guide d'Implémentation MVP

## Contexte

Ce document est le **guide pratique d'implémentation** du MVP (Version 0.1) de Miyukini Life Game. Il définit les phases concrètes, les priorités, et les étapes de développement.

## Portée / Scope

- Objectifs du MVP
- Phases d'implémentation séquentielles
- Fonctionnalités prioritaires
- Tests de validation
- Planning estimé

## Objectifs du MVP (v0.1)

### Vision du MVP

> "Démontrer qu'on peut créer un monde, y placer des Humains, observer leur autonomie, et utiliser quelques pouvoirs divins."

### Fonctionnalités incluses

✅ **Création de monde :**
- Taille fixe : 256×256
- Pinceaux : Terre, Eau, Montagne, Forêt
- Génération procédurale basique (Perlin noise)

✅ **Une race : Humains**
- Placement manuel (1-5 unités à la fois)
- Besoins : Faim, Soif, Énergie
- Rôles : Travailleur uniquement (pas encore de soldats/nobles)
- Comportements : Cherche nourriture, construit maisons

✅ **Bâtiments de base :**
- Maisons (logement)
- Fermes (nourriture)
- Puits (eau)

✅ **10 pouvoirs divins :**
- **Création :** Terre, Eau, Forêt, Humains, Nourriture
- **Destruction :** Éclair, Bombe, Incendie, Tremblement de terre, Inondation

✅ **Simulation simple :**
- Tick 10× par seconde
- Besoins diminuent avec le temps
- Unités meurent si faim = 0 pendant 7 jours
- Construction de bâtiments

✅ **UI minimale :**
- Canvas pixel art (carte du monde)
- Palette de pouvoirs (barre latérale)
- Stats basiques (population, nourriture)
- Contrôles temps (pause, vitesse)

✅ **Sauvegarde locale :**
- 1 slot de sauvegarde
- Fichier `.lifegame` compressé
- Auto-save toutes les 5 minutes

### Fonctionnalités EXCLUES du MVP

❌ **Non inclus dans MVP :**
- Autres races (Orcs, Elfes, Nains) → v1.0
- Royaumes et diplomatie → v1.0
- Guerres → v1.0
- Créatures hostiles → v1.0
- Effets visuels avancés → v1.0
- Multi-slots de sauvegarde → v1.0
- Statistiques détaillées → v1.0
- Timeline historique → v1.0

## Phases d'implémentation

### Phase 0 : Setup (Jour 1-2)

**Objectif :** Créer la structure du projet

**Tâches :**
1. Créer les crates :
   ```bash
   cargo new --lib crates/miyukini-life-game
   cargo new --lib crates/lifegame-simulation
   cargo new --lib crates/lifegame-entities
   cargo new --lib crates/lifegame-world
   cargo new --lib crates/lifegame-powers
   ```

2. Configurer `Cargo.toml` avec dépendances

3. Créer structure de base :
   - `src/lib.rs` dans chaque crate
   - `src/main.rs` dans le service principal
   - Modules de base (vides pour l'instant)

4. Tester compilation :
   ```bash
   cargo build --all
   ```

**Critère de succès :** Projet compile sans erreurs

### Phase 1 : Terrain de base (Jour 3-5)

**Objectif :** Afficher une carte 256×256 modifiable

**Tâches :**
1. Implémenter `TerrainGrid` :
   ```rust
   pub struct TerrainGrid {
       width: u32,
       height: u32,
       tiles: Vec<Vec<Tile>>,
   }
   ```

2. Générer terrain par défaut (plat avec eau)

3. Créer UI Dioxus avec canvas pixel art

4. Afficher le terrain (1 pixel = 1 tuile)

5. Implémenter pinceaux :
   - Clic gauche = dessiner
   - Taille pinceau : 5×5
   - Types : Terre, Eau, Montagne, Forêt

6. Zoom et pan (molette + clic droit glisser)

**Critère de succès :** Peut dessiner un continent jouable

### Phase 2 : Génération procédurale (Jour 6-7)

**Objectif :** Générer des mondes automatiquement

**Tâches :**
1. Implémenter Perlin Noise (ou utiliser crate `noise`)

2. Générer élévation :
   - <0.3 = Eau
   - 0.3-0.5 = Plaines
   - 0.5-0.7 = Collines
   - >0.7 = Montagnes

3. Ajouter forêts (noise secondaire)

4. Bouton "Nouveau Monde" avec seed aléatoire

**Critère de succès :** Génère des mondes variés et jouables

### Phase 3 : Entités de base (Jour 8-10)

**Objectif :** Créer et afficher des Humains

**Tâches :**
1. Implémenter `Unit` :
   ```rust
   pub struct Unit {
       id: EntityId,
       position: Vec2,
       health: u32,
       hunger: u8,
       thirst: u8,
       energy: u8,
       role: UnitRole,
   }
   ```

2. Implémenter `EntityManager`

3. Afficher unités sur la carte (sprite 3×3 pixels)

4. Pouvoir "Placer Humains" (1-3 unités)

5. Déplacement aléatoire (random walk)

**Critère de succès :** Unités se déplacent de façon autonome

### Phase 4 : Besoins et survie (Jour 11-13)

**Objectif :** Unités ont des besoins vitaux

**Tâches :**
1. Tick de simulation :
   ```rust
   fn tick_world(world: &mut World) {
       for unit in world.entities.units.values_mut() {
           unit.hunger -= 1;  // Diminue chaque minute
           unit.thirst -= 2;
           unit.energy -= 1;
       }
   }
   ```

2. Mort par famine :
   - Si hunger = 0 pendant 7 jours → meurt
   - Si health = 0 → meurt

3. Recherche de nourriture :
   - Unité détecte nourriture dans rayon de 20 tuiles
   - Pathfinding simple (ligne droite vers cible)
   - Mange si sur une case avec nourriture

4. Afficher barres de besoins au survol

**Critère de succès :** Unités survivent si nourriture disponible

### Phase 5 : Bâtiments (Jour 14-17)

**Objectif :** Construction autonome

**Tâches :**
1. Implémenter `Building` :
   ```rust
   pub struct Building {
       id: EntityId,
       position: Vec2,
       building_type: BuildingType,
       construction_progress: u8,  // 0-100
   }
   ```

2. Types de bâtiments :
   - **Maison** (20 bois) — Logement
   - **Ferme** (10 bois) — Produit 2 nourriture/jour
   - **Puits** (10 pierre) — Source d'eau

3. Décision de construire :
   - Si pas de maison → construit maison
   - Si manque nourriture → construit ferme
   - Si pas d'eau → construit puits

4. Construction progressive (5-10 minutes de jeu)

5. Afficher bâtiments sur carte (sprites 5×5 pixels)

**Critère de succès :** Village se forme automatiquement

### Phase 6 : Pouvoirs divins (Jour 18-21)

**Objectif :** Joueur peut intervenir

**Tâches :**
1. Implémenter palette de pouvoirs (UI)

2. **Pouvoirs de création :**
   - Terre : Transforme eau en terre
   - Eau : Transforme terre en eau
   - Forêt : Place 5-10 arbres
   - Humains : Place 1-3 unités
   - Nourriture : +100 nourriture

3. **Pouvoirs de destruction :**
   - Éclair : 50 HP dégâts, rayon 3
   - Bombe : 150 HP dégâts, rayon 8
   - Incendie : Se propage dans forêts
   - Tremblement de terre : Détruit bâtiments, rayon 20
   - Inondation : Transforme terre en eau

4. Intégration StrongFather (permissions)

5. Effets visuels basiques (flash, particules simples)

**Critère de succès :** Tous les pouvoirs fonctionnent

### Phase 7 : Contrôle du temps (Jour 22-23)

**Objectif :** Gérer la vitesse de simulation

**Tâches :**
1. Implémenter boutons :
   - Pause (Espace)
   - Vitesse 1× (touche 1)
   - Vitesse 2× (touche 2)
   - Vitesse 5× (touche 3)

2. Ajuster boucle de jeu selon vitesse

3. Afficher temps écoulé (jours, mois, années)

**Critère de succès :** Peut accélérer/ralentir à volonté

### Phase 8 : Sauvegarde (Jour 24-26)

**Objectif :** Persister l'état du monde

**Tâches :**
1. Implémenter sérialisation (serde)

2. Compression (zstd)

3. Sauvegarde manuelle (Ctrl+S)

4. Auto-save toutes les 5 minutes

5. Chargement au démarrage

6. Écran de sélection (1 slot)

**Critère de succès :** Peut sauvegarder et recharger

### Phase 9 : UI et polish (Jour 27-30)

**Objectif :** Interface utilisable et esthétique

**Tâches :**
1. Panel de stats :
   - Population totale
   - Nourriture disponible
   - Nombre de bâtiments
   - Temps écoulé

2. Tooltips informatifs

3. Curseur personnalisé selon pouvoir actif

4. Sons basiques (optionnel)

5. Menu principal :
   - Nouveau monde
   - Charger
   - Quitter

6. Documentation in-game (touches F1)

**Critère de succès :** UI claire et intuitive

### Phase 10 : Tests et debug (Jour 31-35)

**Objectif :** Stabiliser le MVP

**Tâches :**
1. Tests de survie :
   - Laisser tourner 1h sans crash
   - Vérifier pas de memory leak
   - FPS > 60 avec 100 unités

2. Tests de sauvegarde :
   - Sauvegarde/charge 10× de suite
   - Vérifier intégrité des données

3. Tests de pouvoirs :
   - Chaque pouvoir fonctionne
   - Pas de crash avec abus (spam)

4. Correction des bugs majeurs

5. Optimisations si nécessaire

**Critère de succès :** MVP stable et jouable

## APIs des Toolkits requis

### MiyuWorldGen

```rust
/// Génère un terrain procédural
pub fn generate_terrain(
    width: u32,
    height: u32,
    seed: u64,
) -> TerrainGrid {
    // Perlin noise pour élévation
    let noise = PerlinNoise::new(seed);
    
    let mut tiles = vec![vec![Tile::default(); width as usize]; height as usize];
    
    for y in 0..height {
        for x in 0..width {
            let elevation = noise.get([x as f64 / 100.0, y as f64 / 100.0]);
            tiles[y as usize][x as usize] = terrain_from_elevation(elevation);
        }
    }
    
    TerrainGrid { width, height, tiles }
}
```

### MiyuPixelCanvas

```rust
/// Affiche une grille de pixels
pub fn render_canvas(
    canvas: &mut PixelCanvas,
    world: &World,
    camera: &Camera,
) {
    for y in 0..world.terrain.height {
        for x in 0..world.terrain.width {
            let tile = &world.terrain.tiles[y as usize][x as usize];
            let color = tile_color(tile);
            canvas.set_pixel(x, y, color);
        }
    }
    
    // Affiche entités par-dessus
    for unit in world.entities.units.values() {
        canvas.draw_sprite(unit.position, unit_sprite(unit));
    }
}
```

### MiyuEntitySim

```rust
/// Décide l'action d'une unité
pub fn decide_unit_action(
    unit: &Unit,
    world: &World,
) -> Action {
    // Urgences
    if unit.hunger < 10 {
        if let Some(food) = find_nearest_food(unit, world) {
            return Action::MoveTo(food.position);
        }
    }
    
    // Tâches
    if let Some(building) = find_unfinished_building(unit, world) {
        return Action::Build(building.id);
    }
    
    // Par défaut : errer
    Action::Wander
}
```

### MiyuTimeControl

```rust
/// Contrôle la vitesse de simulation
pub struct TimeController {
    speed: TimeSpeed,
    paused: bool,
}

impl TimeController {
    pub fn tick_multiplier(&self) -> u32 {
        if self.paused {
            return 0;
        }
        
        match self.speed {
            TimeSpeed::Normal => 1,
            TimeSpeed::Fast2x => 2,
            TimeSpeed::Fast5x => 5,
            TimeSpeed::Fast10x => 10,
        }
    }
}
```

### MiyuSaveFormat

```rust
/// Sauvegarde un monde
pub fn save_world(world: &World, path: &Path) -> Result<(), Error> {
    // 1. Sérialiser
    let data = bincode::serialize(world)?;
    
    // 2. Compresser
    let compressed = zstd::encode_all(data.as_slice(), 3)?;
    
    // 3. Écrire
    std::fs::write(path, compressed)?;
    
    Ok(())
}

/// Charge un monde
pub fn load_world(path: &Path) -> Result<World, Error> {
    // 1. Lire
    let compressed = std::fs::read(path)?;
    
    // 2. Décompresser
    let data = zstd::decode_all(compressed.as_slice())?;
    
    // 3. Désérialiser
    let world = bincode::deserialize(&data)?;
    
    Ok(world)
}
```

## Tests de validation MVP

### Test 1 : Création de monde

**Scénario :**
1. Lancer l'application
2. Cliquer "Nouveau Monde"
3. Observer la génération procédurale

**Résultat attendu :**
- Monde généré en <3 secondes
- Continents, océans, montagnes visibles
- Pas de crash

### Test 2 : Placement et survie

**Scénario :**
1. Créer un monde
2. Placer 10 Humains sur un continent
3. Accélérer le temps (vitesse 5×)
4. Observer pendant 10 minutes

**Résultat attendu :**
- Unités construisent maisons et fermes
- Unités survivent (ne meurent pas de faim)
- Population stable ou augmente
- Pas de crash

### Test 3 : Pouvoirs divins

**Scénario :**
1. Créer un monde avec population
2. Tester chaque pouvoir 1 par 1
3. Observer les effets

**Résultat attendu :**
- Chaque pouvoir fonctionne comme prévu
- Effets visibles immédiatement
- Pas de crash même avec spam

### Test 4 : Sauvegarde/Chargement

**Scénario :**
1. Créer un monde complexe (50+ unités, 20+ bâtiments)
2. Sauvegarder (Ctrl+S)
3. Quitter l'application
4. Relancer et charger

**Résultat attendu :**
- Sauvegarde en <2 secondes
- Chargement en <5 secondes
- Monde identique (même état)
- Simulation reprend normalement

### Test 5 : Performance

**Scénario :**
1. Créer un monde
2. Placer 100 Humains (en plusieurs groupes)
3. Accélérer au maximum (10×)
4. Laisser tourner 30 minutes

**Résultat attendu :**
- FPS > 60 en permanence
- Pas de ralentissement progressif
- Mémoire stable (<300 MB)
- Pas de crash

## Planning estimé

### Calendrier (1 développeur)

| Phase | Durée | Jours cumulés | Semaine |
|-------|-------|---------------|---------|
| Phase 0 : Setup | 2 jours | 2 | S1 |
| Phase 1 : Terrain | 3 jours | 5 | S1 |
| Phase 2 : Génération | 2 jours | 7 | S1-S2 |
| Phase 3 : Entités | 3 jours | 10 | S2 |
| Phase 4 : Besoins | 3 jours | 13 | S2-S3 |
| Phase 5 : Bâtiments | 4 jours | 17 | S3 |
| Phase 6 : Pouvoirs | 4 jours | 21 | S3-S4 |
| Phase 7 : Temps | 2 jours | 23 | S4 |
| Phase 8 : Sauvegarde | 3 jours | 26 | S4-S5 |
| Phase 9 : UI/Polish | 4 jours | 30 | S5 |
| Phase 10 : Tests | 5 jours | 35 | S5-S6 |

**Total :** ~6 semaines (35 jours ouvrés)

### Livrables par semaine

**Semaine 1 :**
- ✅ Projet compilable
- ✅ Carte 256×256 affichée
- ✅ Pinceaux terrain fonctionnels
- ✅ Génération procédurale

**Semaine 2 :**
- ✅ Humains placés et affichés
- ✅ Déplacement autonome
- ✅ Besoins vitaux implémentés

**Semaine 3 :**
- ✅ Bâtiments construits automatiquement
- ✅ Villages se forment
- ✅ Premiers pouvoirs divins

**Semaine 4 :**
- ✅ Tous les pouvoirs fonctionnels
- ✅ Contrôle du temps
- ✅ Sauvegarde/chargement

**Semaine 5-6 :**
- ✅ UI complète
- ✅ Tests et stabilisation
- ✅ **MVP v0.1 prêt**

## Critères de succès MVP

Le MVP est considéré **réussi** si :

✅ **Technique :**
- Compile sans warnings
- 60 FPS avec 100 unités
- Pas de crash après 1h de jeu
- Sauvegarde <2s, chargement <5s
- Mémoire <300 MB

✅ **Fonctionnel :**
- Peut créer un monde
- Humains survivent de façon autonome
- Villages se forment sans intervention
- 10 pouvoirs divins fonctionnels
- Sauvegarde/chargement fonctionnel

✅ **Jouabilité :**
- UI intuitive
- Commandes réactives
- Effets visuels des pouvoirs
- Partie intéressante pendant >30 min

## Prochaines étapes après MVP

Une fois le MVP validé, la roadmap v1.0 inclut :

**Version 1.0 (Beta) :**
- 4 races (Orcs, Elfes, Nains)
- Royaumes et diplomatie
- Guerres automatiques
- Créatures hostiles
- 50+ pouvoirs divins
- Effets visuels avancés
- Multi-slots de sauvegarde
- Statistiques détaillées

**Durée estimée v1.0 :** +12-16 semaines

## Conclusion

Ce guide d'implémentation fournit un **plan séquentiel clair** pour développer le MVP de Miyukini Life Game en 6 semaines. Chaque phase est indépendante et testable. À la fin du MVP, nous aurons un jeu jouable démontrant le concept core du god simulator.

**Phase suivante :** Lire la Référence WorldBox pour comparer avec le jeu d'inspiration.
