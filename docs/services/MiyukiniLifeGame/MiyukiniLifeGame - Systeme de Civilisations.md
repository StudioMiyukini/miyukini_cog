# MiyukiniLifeGame — Système de Civilisations

## Contexte

Ce document détaille le **système de civilisations autonomes** de Miyukini Life Game : comportements des races, formation de royaumes, diplomatie, guerres, et évolution culturelle.

## Portée / Scope

- Traits raciaux des 4 races civilisées
- Comportements émergents
- Formation et gestion de royaumes
- Systèmes de diplomatie
- Mécaniques de guerre
- Évolution technologique et culturelle
- Intelligence artificielle des entités

## Les 4 Races Civilisées

### Vue d'ensemble

| Race | Philosophie | Force | Faiblesse | Biome préféré |
|------|-------------|-------|-----------|---------------|
| **Humains** | Équilibre et expansion | Adaptabilité | Aucune spécialité | Plaines |
| **Orcs** | Force et guerre | Combat | Technologie lente | Steppes, montagnes |
| **Elfes** | Harmonie et magie | Longévité, magie | Fragiles | Forêts |
| **Nains** | Craft et défense | Technologie, défense | Lents, rares | Montagnes |

### Humains

**Identité :**
> "Les adaptables. Ils prospèrent partout, se multiplient vite, et conquièrent par le nombre."

**Traits raciaux :**
- **Adaptabilité** — +20% croissance dans tous biomes
- **Expansion rapide** — Colonisent 2× plus vite
- **Polyvalents** — Pas de bonus/malus spécialisé
- **Diplomates** — Relations neutres +10 de base
- **Durée de vie** — 60-80 ans

**Style de jeu :**
- Expansion agressive
- Équilibre armée/économie
- Commerce actif
- Adaptation aux menaces

**Architecture typique :**
- Villages étalés
- Routes nombreuses
- Fermes extensives
- Châteaux en pierre grise

**Noms typiques :**
- Villes : Aldaria, Brenwick, Covenham
- Rois : Alaric, Baldwin, Conrad
- Reines : Elara, Beatrix, Isolde

### Orcs

**Identité :**
> "Les guerriers. Ils vivent pour le combat, méprisent la faiblesse, et honorent la force brute."

**Traits raciaux :**
- **Force brutale** — +50% dégâts au combat
- **Agressifs** — -20 diplomatie avec toutes races
- **Technologie lente** — -30% vitesse de recherche
- **Résistants** — +50 HP max
- **Durée de vie** — 40-60 ans (vie dure)

**Style de jeu :**
- Guerre constante
- Raids fréquents
- Peu de bâtiments
- Expansion militaire

**Architecture typique :**
- Campements fortifiés
- Palissades en bois
- Tours de guerre
- Fosses de combat

**Noms typiques :**
- Villes : Grom'kar, Drak'zul, Kor'gath
- Chefs : Thrall, Grom, Durotan
- Matrones : Garona, Draka, Aggra

### Elfes

**Identité :**
> "Les éternels. Ils vivent des siècles, protègent la nature, et maîtrisent la magie."

**Traits raciaux :**
- **Longue vie** — 200-400 ans
- **Affinité magique** — Pouvoirs divins 2× plus efficaces sur eux
- **Fragiles** — -30% HP max
- **Protecteurs de nature** — -10 bonheur si forêt détruite
- **Croissance lente** — -50% taux de natalité

**Style de jeu :**
- Défense des forêts
- Diplomatie pacifique
- Technologies magiques
- Expansion très lente

**Architecture typique :**
- Arbres habités
- Ponts suspendus
- Tours blanches élancées
- Jardins enchantés

**Noms typiques :**
- Villes : Silverwood, Moonhaven, Starfall
- Rois : Elrohir, Fëanor, Celeborn
- Reines : Galadriel, Arwen, Elenia

### Nains

**Identité :**
> "Les bâtisseurs. Ils creusent profond, forgent l'acier le plus pur, et défendent jusqu'à la mort."

**Traits raciaux :**
- **Maîtres artisans** — +100% qualité équipements
- **Technologie avancée** — +50% vitesse de recherche (tech)
- **Défenseurs** — +100% défense en montagne
- **Lents** — -20% vitesse de déplacement
- **Durée de vie** — 120-180 ans

**Style de jeu :**
- Fortifications massives
- Extraction minière
- Technologies militaires
- Défense territoriale

**Architecture typique :**
- Citadelles souterraines
- Forges géantes
- Murailles de pierre
- Halls majestueux

**Noms typiques :**
- Villes : Ironforge, Khazad-dûm, Thorinshold
- Rois : Thorin, Gimli, Dwalin
- Reines : Disa, Brunhilde, Freya

## Comportements émergents

### Intelligence artificielle des unités

**Hiérarchie de besoins (Maslow appliqué) :**
```
1. Survie      → Cherche nourriture, eau, fuit danger
2. Sécurité    → Rejoint groupe, construit abri
3. Social      → Forme famille, se marie, a des enfants
4. Estime      → Devient soldat/noble, gagne prestige
5. Réalisation → Devient roi, construit merveilles
```

**Arbre de décision simplifié :**
```rust
fn decide_action(unit: &Unit, world: &World) -> Action {
    // Urgences (survie)
    if unit.health < 20 {
        return Action::Flee;
    }
    if unit.hunger < 10 {
        return Action::FindFood;
    }
    if unit.thirst < 10 {
        return Action::FindWater;
    }
    
    // Rôle
    match unit.role {
        UnitRole::Worker => decide_worker_action(unit, world),
        UnitRole::Soldier => decide_soldier_action(unit, world),
        UnitRole::Noble => decide_noble_action(unit, world),
        UnitRole::King => decide_king_action(unit, world),
    }
}
```

### Comportement des travailleurs

**Tâches quotidiennes :**
- Matin (6h-12h) : Travail (ferme, mine, construction)
- Midi (12h-13h) : Repas
- Après-midi (13h-18h) : Travail
- Soir (18h-20h) : Socialisation
- Nuit (20h-6h) : Sommeil

**Décisions autonomes :**
- Construit maison si pas de logement
- Construit ferme si manque nourriture
- Construit muraille si menace proche
- Se marie si célibataire + bonheur > 50
- A des enfants si marié + nourriture abondante

### Comportement des soldats

**États :**
- **Patrouille** — Surveille frontières
- **Défense** — Garde capitale/murailles
- **Attaque** — Marche vers ennemi
- **Retraite** — Fuit si surnombre
- **Pillage** — Vole ressources ennemies

**Tactiques de combat :**
- Formation en ligne (défense)
- Charge en masse (attaque)
- Encerclement (surnombre)
- Embuscade (forêt/montagne)
- Siège (autour de ville)

### Comportement des rois

**Responsabilités :**
- Décider des guerres (évalue chances victoire)
- Gérer diplomatie (accepte/refuse alliances)
- Distribuer ressources (impôts, constructions)
- Nommer nobles (promeut soldats valeureux)
- Gérer lois (taxes, service militaire)

**Personnalité (traits aléatoires) :**
- **Belliqueux** — Déclare guerres facilement
- **Pacifique** — Évite conflits
- **Avide** — Taxes élevées
- **Généreux** — Taxes basses, populaire
- **Paranoïaque** — Armée surdimensionnée
- **Visionnaire** — Investit dans technologie

## Formation de royaumes

### Étape 1 : Fondation (Jour 0-10)

**Conditions :**
- 3+ unités de même race
- Rayon de 50 tuiles max
- Terrain favorable

**Événement :**
```
"Un groupe d'Humains a fondé le village de Aldaria."
```

### Étape 2 : Village (Jour 10-100)

**Développement :**
- Construction de 5+ maisons
- 1-2 fermes
- Population : 20-50
- Pas encore de roi

**Gouvernance temporaire :**
- Conseil d'anciens (3 plus âgés)
- Décisions consensuelles

### Étape 3 : Bourg (Jour 100-300)

**Développement :**
- 10+ maisons
- Marché
- Caserne
- Population : 50-150

**Élection du roi :**
- Candidat = Unité avec meilleur prestige
- Prestige = Kills + Âge + Richesse
- Événement :
```
"Alaric a été couronné Roi d'Aldaria."
```

### Étape 4 : Royaume (Jour 300+)

**Développement :**
- Château (capitale)
- Murailles
- 3+ villages vassaux
- Population : 150-1000

**Expansion :**
- Colonise nouveaux territoires
- Fonde villes secondaires
- Établit frontières

### Étape 5 : Empire (Jour 1000+)

**Développement :**
- 5+ villes
- Plusieurs continents
- Population : 1000+
- Technologies avancées

**Caractéristiques :**
- Flotte navale
- Armée permanente (500+ soldats)
- Commerce international
- Influence diplomatique

## Système de diplomatie

### Relations dynamiques

**Calcul de relation :**
```rust
fn calculate_relation(k1: &Kingdom, k2: &Kingdom, history: &History) -> i32 {
    let mut score = 0;
    
    // Facteurs positifs
    if k1.race == k2.race { score += 10; }
    if k1.religion == k2.religion { score += 5; }
    score += history.trades(k1, k2) / 10;
    score += history.gifts(k1, k2) * 2;
    
    // Facteurs négatifs
    score -= history.wars(k1, k2) * 20;
    score -= history.border_conflicts(k1, k2) * 5;
    if k1.territory.overlaps(k2.territory) { score -= 10; }
    
    // Personnalité du roi
    if k1.king.trait == Trait::Aggressive { score -= 10; }
    if k1.king.trait == Trait::Peaceful { score += 10; }
    
    score.clamp(-100, 100)
}
```

### Actions diplomatiques autonomes

**Le roi décide seul (simulation) :**

```rust
fn king_diplomacy_turn(kingdom: &mut Kingdom, world: &World) {
    for neighbor in world.get_neighbors(kingdom) {
        let relation = world.get_relation(kingdom, neighbor);
        
        match relation {
            -100..=-80 => {
                // Très hostile → Guerre
                if kingdom.army_strength > neighbor.army_strength * 0.7 {
                    declare_war(kingdom, neighbor);
                }
            }
            -79..=-50 => {
                // Hostile → Raids
                if rand::random::<f32>() < 0.2 {
                    launch_raid(kingdom, neighbor);
                }
            }
            -49..=19 => {
                // Neutre → Rien (ou commerce si proche de positif)
                if relation > 0 && rand::random::<f32>() < 0.1 {
                    propose_trade(kingdom, neighbor);
                }
            }
            20..=49 => {
                // Positif → Commerce
                if !has_trade_pact(kingdom, neighbor) {
                    propose_trade(kingdom, neighbor);
                }
            }
            50..=100 => {
                // Amis → Alliance
                if !has_alliance(kingdom, neighbor) {
                    propose_alliance(kingdom, neighbor);
                }
            }
        }
    }
}
```

### Événements diplomatiques

**Types d'événements :**
- **Mariage royal** — Roi épouse princesse d'un autre royaume
- **Traité commercial** — +20% commerce entre royaumes
- **Alliance défensive** — Se défendent mutuellement
- **Non-agression** — Promesse de paix (10 ans)
- **Vassalisation** — Royaume devient vassal
- **Indépendance** — Vassal se libère
- **Trahison** — Alliance rompue brutalement

## Système de guerre

### Déclenchement

**Causes automatiques :**
1. Relation < -80
2. Revendications territoriales (frontières contestées)
3. Insulte diplomatique (rejet humiliant)
4. Alliance forcée (allié attaqué)
5. Intervention divine (joueur force guerre)

### Phases de guerre

#### Phase 1 : Mobilisation (Jours 1-10)

```
"Le Royaume d'Aldaria a déclaré la guerre au Royaume de Brenwick!"
```

- Recrutement de soldats (20% population)
- Production d'armes (forges 2× vitesse)
- Stockage de nourriture
- Moral +30% (défense de la patrie)

#### Phase 2 : Invasion (Jours 10-50)

- Armée marche vers frontière
- Batailles de frontière
- Sièges de villes
- Raids sur villages

#### Phase 3 : Occupation (Jours 50-100)

- Territoire conquis occupé
- Résistance locale possible
- Rébellions si occupation brutale
- Intégration progressive

#### Phase 4 : Résolution (Jour 100+)

**Victoire totale :**
- Capitale ennemie capturée
- Roi tué ou en fuite
- Royaume annexé ou vassalisé

**Paix négociée :**
- Statu quo (frontières initiales)
- Concessions territoriales (1-3 provinces)
- Indemnités de guerre (ressources)

**Défaite :**
- Perd territoire
- Moral bas (-50%)
- Risque de rébellion

### Tactiques de combat

**Facteurs de victoire :**
```rust
fn battle_outcome(army1: &Army, army2: &Army, terrain: Terrain) -> Outcome {
    let mut power1 = army1.size as f32 * army1.avg_attack * army1.morale;
    let mut power2 = army2.size as f32 * army2.avg_attack * army2.morale;
    
    // Bonus défenseur
    if army2.is_defending {
        power2 *= 1.2;
    }
    
    // Bonus terrain
    match terrain {
        Terrain::Mountain => {
            if army2.is_defending { power2 *= 1.5; }
        }
        Terrain::Forest => {
            if army1.race == Race::Elf { power1 *= 1.3; }
        }
        Terrain::Plains => {
            // Aucun bonus
        }
        _ => {}
    }
    
    // Bonus racial
    if army1.race == Race::Orc {
        power1 *= 1.3; // Orcs = guerriers nés
    }
    if army2.race == Race::Dwarf && terrain == Terrain::Mountain {
        power2 *= 1.5; // Nains en montagne
    }
    
    let ratio = power1 / power2;
    
    if ratio > 2.0 {
        Outcome::CrushingVictory(army1)
    } else if ratio > 1.3 {
        Outcome::Victory(army1)
    } else if ratio < 0.5 {
        Outcome::CrushingVictory(army2)
    } else if ratio < 0.77 {
        Outcome::Victory(army2)
    } else {
        Outcome::Stalemate
    }
}
```

### Rébellions

**Causes :**
- Impôts trop élevés (>40%)
- Famine prolongée (>30 jours)
- Occupation étrangère
- Roi impopulaire (charisme <20)
- Différence culturelle (conquête récente)

**Processus :**
1. **Mécontentement monte** — Bonheur <-50
2. **Manifestations** — Unités arrêtent de travailler
3. **Rébellion armée** — 20-40% population se rebelle
4. **Guerre civile** — Combat contre armée loyale
5. **Résolution** — Nouveau roi ou répression

## Évolution culturelle

### Langues

Chaque royaume développe une langue unique (générée) :
- Affecte noms de villes
- Affecte noms de personnes
- -10 diplomatie si langues différentes
- Traducteurs peuvent être formés (bibliothèque)

### Drapeaux et symboles

Générés procéduralement :
- Couleurs primaires (rouge, bleu, vert, or)
- Symboles (lion, aigle, arbre, marteau)
- Motifs (rayures, diagonales, quadrants)

### Architectures

Évoluent avec la technologie :
- **Âge de pierre** — Huttes en bois
- **Âge de bronze** — Maisons en pierre simple
- **Âge de fer** — Bâtiments en pierre taillée
- **Médiéval** — Châteaux majestueux

### Religions

Se forment spontanément :
- **Déisme** — Culte du Joueur (dieu créateur)
- **Animisme** — Esprits de la nature
- **Monothéisme** — Dieu unique
- **Polythéisme** — Panthéon de dieux

**Effets :**
- Temples construits
- Prêtres apparaissent
- Pèlerinages
- Guerres saintes possibles

## Statistiques et observation

### Statistiques par royaume

**Démographie :**
- Population totale
- Répartition par âge
- Taux de natalité/mortalité
- Espérance de vie

**Économie :**
- Ressources stockées
- Production/jour
- Commerce (import/export)
- Richesse moyenne par habitant

**Militaire :**
- Nombre de soldats
- Qualité de l'équipement
- Victoires/défaites
- Territoires contrôlés

**Technologies :**
- Âge actuel
- Technologies découvertes
- Vitesse de recherche

**Diplomatie :**
- Alliés / Ennemis
- Guerres actives
- Traités signés

### Timeline historique

Tous les événements sont enregistrés :
- Fondation de royaumes
- Couronnements de rois
- Déclarations de guerre
- Batailles majeures
- Traités de paix
- Catastrophes naturelles
- Interventions divines

**Exemple de timeline :**
```
Jour 1    : Monde créé
Jour 15   : Village d'Aldaria fondé (Humains)
Jour 42   : Village de Silverwood fondé (Elfes)
Jour 120  : Alaric couronné Roi d'Aldaria
Jour 185  : Elrohir couronné Roi de Silverwood
Jour 240  : Traité commercial Aldaria-Silverwood
Jour 315  : Aldaria déclare la guerre à Brenwick
Jour 340  : Bataille de Redford - Victoire d'Aldaria
Jour 380  : Paix signée - Brenwick cède 2 provinces
Jour 420  : Éclair divin frappe Aldaria (intervention divine)
Jour 450  : Reconstruction d'Aldaria
```

## Conclusion

Le système de civilisations de Miyukini Life Game repose sur des **comportements émergents** issus de règles simples. Les unités ont des besoins, des désirs, et des traits de personnalité. Les royaumes se forment, prospèrent, entrent en guerre, signent des traités. Chaque partie raconte une histoire unique.

**Phase suivante :** Lire le document sur l'Architecture Technique pour comprendre l'implémentation concrète de ces systèmes.
