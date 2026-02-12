# MiyukiniLifeGame — Mécaniques de Jeu

## Contexte

Ce document détaille les **mécaniques de jeu** de Miyukini Life Game : systèmes de terrain, créatures, civilisations, besoins, diplomatie, guerres, et progression technologique.

## Portée / Scope

- Mécaniques de création de monde
- Systèmes d'entités (créatures, unités, bâtiments)
- Besoins et survie
- Civilisations et royaumes
- Diplomatie et relations
- Guerres et combats
- Technologies et évolution

## 1. Création de monde

### Tailles de monde

| Taille | Dimensions | Tuiles totales | RAM estimée | Recommandé pour |
|--------|------------|----------------|-------------|-----------------|
| Petit | 256 × 256 | 65,536 | ~50 MB | Tests, mondes rapides |
| Moyen | 512 × 512 | 262,144 | ~150 MB | Partie standard |
| Grand | 1024 × 1024 | 1,048,576 | ~500 MB | Campagnes longues |

### Types de terrain

| Terrain | Couleur | Constructible | Navigable | Fertilité |
|---------|---------|---------------|-----------|-----------|
| **Eau profonde** | Bleu foncé | ❌ | 🚢 Bateau | - |
| **Eau peu profonde** | Bleu clair | ❌ | 🚶 Patauger | - |
| **Plage** | Jaune | ⚠️ Limité | ✅ | Faible |
| **Plaines** | Vert | ✅ | ✅ | Élevée |
| **Forêt** | Vert foncé | ⚠️ Après défricher | ✅ | Moyenne |
| **Colline** | Marron clair | ⚠️ Lent | ✅ | Faible |
| **Montagne** | Gris | ❌ | ❌ | - |
| **Neige** | Blanc | ⚠️ | ✅ | Très faible |
| **Désert** | Orange | ✅ | ✅ | Très faible |
| **Marais** | Vert-brun | ⚠️ | ⚠️ Lent | Moyenne |
| **Lave** | Rouge | ❌ | ❌ | - |

### Biomes

Zones géographiques avec caractéristiques spécifiques.

**Liste des biomes :**
1. **Plaines tempérées** — Équilibré, idéal pour toutes les races
2. **Forêt dense** — Favorise les Elfes
3. **Montagnes** — Favorise les Nains
4. **Désert** — Difficile, faible population
5. **Toundra** — Froid, croissance lente
6. **Jungle** — Dense, maladies fréquentes
7. **Marais** — Malsain, mais riche en ressources
8. **Volcanique** — Dangereux, mais minerais précieux
9. **Arctique** — Très froid, survie difficile
10. **Savane** — Sec, faune abondante

### Ressources naturelles

**Ressources de terrain :**
- **Minerai de fer** — Armes et outils
- **Minerai d'or** — Commerce et prestige
- **Charbon** — Énergie
- **Diamant** — Équipements légendaires
- **Pierre** — Construction
- **Bois** — Construction et chauffage

**Ressources biologiques :**
- **Animaux sauvages** — Nourriture (chasse)
- **Plantes sauvages** — Nourriture (cueillette)
- **Poissons** — Nourriture (pêche)

### Outils de création

**Pinceaux de terrain :**
- Taille : Petit (3×3), Moyen (7×7), Grand (15×15)
- Formes : Cercle, Carré, Ligne
- Modes : Dessiner, Remplir, Remplacer

**Outils spéciaux :**
- **Élévation** — Monte/descend le terrain
- **Lissage** — Adoucit les transitions
- **Érosion** — Effet naturel
- **Rivières** — Trace automatiquement
- **Continents** — Génère des masses terrestres

**Génération procédurale :**
- **Perlin Noise** — Terrain réaliste
- **Seed personnalisée** — Mondes reproductibles
- **Présets** — Archipel, Pangée, Deux continents, Chaos

## 2. Entités : Créatures

### Créatures neutres

**Animaux sauvages :**
- **Cerfs** — Herbivores, fuient les prédateurs
- **Loups** — Chassent en meute
- **Ours** — Solitaires, dangereux
- **Moutons** — Domesticables
- **Vaches** — Domesticables
- **Chevaux** — Domesticables, transport

**Comportements :**
- Recherchent nourriture et eau
- Se reproduisent dans environnements favorables
- Migrent si ressources épuisées
- Fuient les dangers (feu, prédateurs)

### Créatures hostiles

| Créature | PV | Dégâts | Spawn | Faiblesse |
|----------|-----|--------|-------|-----------|
| **Zombie** | 50 | 10 | Nuit, tombes | Feu, lumière |
| **Squelette** | 40 | 15 (arc) | Nuit | Feu |
| **Démon** | 200 | 50 | Portails | Eau bénite |
| **Dragon** | 1000 | 100 | Volcans, pondu | Armes lourdes |
| **Tumeur** | 100 | 0 (infecte) | Corruption | Feu divin |
| **Cold One** | 150 | 30 | Arctique | Feu |
| **Crabzilla** | 5000 | 500 | Invoqué | Rien (boss) |

**Comportements spéciaux :**
- **Zombies** — Infectent les morts, créent hordes
- **Tumeurs** — Corrompent le terrain
- **Dragons** — Volent, crachent feu, contrôlables
- **Crabzilla** — Détruit tout, contrôlable par le joueur

### Créatures magiques

- **Licornes** — Guérissent le terrain
- **Treants** — Protègent les forêts
- **Fées** — Accélèrent la croissance des plantes
- **Golems** — Gardiens de montagnes

## 3. Entités : Unités civilisées

### Caractéristiques d'une unité

```rust
struct Unit {
    id: EntityId,
    name: String,
    race: Race,
    kingdom: KingdomId,
    
    // Stats de base
    health: u32,
    max_health: u32,
    attack: u32,
    defense: u32,
    speed: f32,
    
    // Besoins
    hunger: u8,       // 0-100 (meurt si 0 pendant 7 jours)
    thirst: u8,       // 0-100
    energy: u8,       // 0-100 (dort si trop bas)
    happiness: i8,    // -100 à +100
    
    // Rôles
    role: UnitRole,   // Worker, Soldier, Noble, King
    
    // Équipement
    weapon: Option<Weapon>,
    armor: Option<Armor>,
    
    // Traits (génétiques ou acquis)
    traits: Vec<Trait>,
    
    // Relations
    family: FamilyTree,
    friends: Vec<EntityId>,
    enemies: Vec<EntityId>,
    
    // Expérience
    level: u8,
    experience: u32,
    kills: u32,
}
```

### Rôles des unités

| Rôle | Population | Fonction | Équipement |
|------|------------|----------|------------|
| **Paysan** | 70% | Ferme, construit, récolte | Outils |
| **Soldat** | 20% | Défend, attaque | Armes, armures |
| **Noble** | 8% | Gouverne, commerce | Vêtements luxueux |
| **Roi** | 1% | Dirige le royaume | Couronne, sceptre |
| **Prêtre** | 1% | Religion, guérison | Robes |

### Cycle de vie

**Âges :**
1. **Enfant** (0-16 ans) — Ne travaille pas, vulnérable
2. **Adulte** (16-60 ans) — Travaille, se bat, se reproduit
3. **Ancien** (60-80 ans) — Ralenti, sagesse, conseiller

**Reproduction :**
- Nécessite 2 adultes (compatible si même race)
- Bébé naît après 9 mois (accéléré en jeu)
- Hérite de traits parentaux
- Nommé automatiquement avec système de noms

**Mort :**
- Vieillesse (80+ ans)
- Famine (7 jours sans nourriture)
- Soif (3 jours sans eau)
- Combat
- Catastrophes naturelles
- Maladies

## 4. Entités : Bâtiments

### Bâtiments de base

| Bâtiment | Coût | Fonction | Capacité |
|----------|------|----------|----------|
| **Maison** | 20 bois | Logement | 4 unités |
| **Ferme** | 10 bois | Produit nourriture | +2/jour |
| **Mine** | 30 pierre | Extrait minerais | +1 minerai/jour |
| **Caserne** | 50 bois, 20 fer | Entraîne soldats | 10 soldats |
| **Muraille** | 100 pierre | Défense | +50 défense |
| **Tour de garde** | 30 bois, 10 fer | Vision, défense | +20 défense |
| **Marché** | 50 bois, 20 or | Commerce | +10% richesse |
| **Bibliothèque** | 80 bois, 40 or | Recherche | +1 tech/an |
| **Temple** | 100 pierre, 50 or | Religion | +20 bonheur |
| **Château** | 500 pierre, 200 fer | Capitale | Centre royaume |

### Construction

**Processus :**
1. Unité décide de construire (si besoin détecté)
2. Cherche emplacement valide
3. Réserve les ressources nécessaires
4. Construit progressivement (1-10 jours selon taille)
5. Bâtiment devient actif

**Règles :**
- Doit être sur terrain constructible
- Nécessite ressources dans le royaume
- Une seule unité construit à la fois (peut être aidée)
- Peut être interrompu (guerre, catastrophe)

### Routes

- Générées automatiquement entre bâtiments importants
- Augmentent la vitesse de déplacement (+50%)
- Nécessitent entretien
- Détruites par catastrophes ou guerres

## 5. Royaumes et civilisations

### Formation d'un royaume

**Étapes :**
1. **Fondation** — 3+ unités de même race dans un rayon de 50 tuiles
2. **Village** — 5+ maisons, 20+ population
3. **Élection du roi** — Unité avec meilleure réputation devient roi
4. **Établissement capital** — Construction d'un château
5. **Expansion** — Colonisation de nouveaux territoires

### Gouvernance

**Types de gouvernement :**
- **Monarchie** — 1 roi héréditaire
- **Oligarchie** — Conseil de nobles
- **Démocratie** — Élections tous les 10 ans
- **Théocratie** — Prêtres gouvernent

**Lois du royaume :**
- Impôts (10-50% des ressources)
- Service militaire (10-30% de la population)
- Commerce (libre ou régulé)
- Religion (obligatoire ou libre)

### Expansion territoriale

**Méthodes :**
1. **Colonisation pacifique** — Fonde de nouveaux villages
2. **Conquête militaire** — Attaque et annexe
3. **Diplomatie** — Alliance devient vassalisation
4. **Mariage royal** — Union de deux royaumes

**Frontières :**
- Délimitées automatiquement (Voronoi diagram)
- Changent selon contrôle des territoires
- Affichées sur la carte avec couleurs distinctes

## 6. Diplomatie

### Relations entre royaumes

**Échelle de relation (-100 à +100) :**
- **+80 à +100** — Alliés (défense mutuelle)
- **+50 à +79** — Amis (commerce privilégié)
- **+20 à +49** — Neutres positifs
- **-19 à +19** — Neutres
- **-49 à -20** — Méfiants
- **-79 à -50** — Hostiles
- **-100 à -80** — Guerre

### Facteurs influençant les relations

**Positifs :**
- Commerce régulier (+1/an)
- Alliances défensives (+2/an)
- Ennemis communs (+3)
- Même religion (+2)
- Cadeaux de ressources (+5)
- Mariages royaux (+10)

**Négatifs :**
- Frontières contestées (-2/an)
- Raids et pillages (-10)
- Trahisons (-20)
- Guerres passées (-5, diminue avec le temps)
- Religions opposées (-3)
- Différences culturelles (-1)

### Actions diplomatiques

| Action | Coût | Effet | Conditions |
|--------|------|-------|------------|
| **Proposer paix** | 100 or | Fin de guerre | En guerre |
| **Déclarer guerre** | - | Hostilités | Relation < 0 |
| **Alliance** | 200 or | Défense mutuelle | Relation > +50 |
| **Pacte commercial** | 50 or | +20% commerce | Relation > +20 |
| **Vassalisation** | - | Contrôle indirect | Après conquête |
| **Cadeau** | Variable | +5 à +15 relation | Tout moment |

## 7. Guerres

### Déclenchement d'une guerre

**Causes :**
- Relations < -50
- Revendications territoriales
- Insultes diplomatiques
- Alliances forcées
- Interventions divines (joueur force la guerre)

### Mobilisation

1. **Déclaration de guerre** — Annonce publique
2. **Recrutement** — 10-30% paysans → soldats
3. **Production d'armes** — Forges intensives
4. **Rassemblement** — Armées convergent
5. **Marche** — Déplacement vers territoire ennemi

### Batailles

**Calcul de combat :**
```rust
fn battle(army1: &Army, army2: &Army) -> BattleResult {
    let power1 = army1.units.len() * army1.avg_attack * army1.morale;
    let power2 = army2.units.len() * army2.avg_attack * army2.morale;
    
    let ratio = power1 / power2;
    
    if ratio > 1.5 {
        BattleResult::Victory(army1)
    } else if ratio < 0.66 {
        BattleResult::Victory(army2)
    } else {
        BattleResult::Stalemate
    }
}
```

**Facteurs de victoire :**
- Nombre de soldats
- Équipement (armes, armures)
- Moral (+50% si défend capital)
- Terrain (bonus défenseur en montagne)
- Technologie militaire
- Fatigue (malus si longue marche)

### Conséquences de guerre

**Victoire :**
- Annexion de territoires (1-3 provinces)
- Pillage de ressources (10-30%)
- Prestige augmenté (+10)
- Population ennemie peut se rebeller

**Défaite :**
- Perte de territoires
- Perte de ressources
- Moral bas (-30%)
- Risque de rébellion interne

**Traité de paix :**
- Reddition (perd 50% territoire)
- Vassalisation (perd autonomie)
- Indemnités (paie ressources)
- Status quo (retour frontières initiales)

## 8. Technologies

### Âges technologiques

| Âge | Durée moyenne | Bâtiments clés | Capacités |
|-----|---------------|----------------|-----------|
| **Pierre** | 0-50 ans | Huttes, feu | Outils en pierre |
| **Bronze** | 50-150 ans | Forge simple | Armes en bronze |
| **Fer** | 150-300 ans | Forge avancée | Armes en fer |
| **Médiéval** | 300-500 ans | Châteaux, cathédrales | Armures complètes |
| **Renaissance** | 500-700 ans | Universités | Poudre à canon |

### Arbre technologique

**Branches :**
1. **Militaire** — Armes, armures, tactiques
2. **Agriculture** — Irrigation, rotation, engrais
3. **Construction** — Pierre, brique, architecture
4. **Commerce** — Routes, monnaie, marchés
5. **Science** — Écriture, mathématiques, médecine
6. **Religion** — Temples, rituels, miracles

**Exemples de technologies :**
- **Agriculture** : Charrue → Irrigation → Rotation des cultures
- **Militaire** : Arc → Arbalète → Poudre à canon
- **Construction** : Bois → Pierre → Brique

### Progression technologique

**Facteurs d'accélération :**
- Bibliothèques (+1 recherche/an)
- Population éduquée (+0.5/an)
- Commerce avec civilisations avancées (+0.2/an)
- Guerres (vol de technologies)

**Blocages :**
- Guerres constantes (ralentit ×0.5)
- Famine (arrête recherche)
- Isolation (pas d'échanges)

## 9. Religions et cultures

### Systèmes de croyances

**Types de religions :**
- **Monothéisme** — 1 dieu unique
- **Polythéisme** — Plusieurs dieux
- **Animisme** — Esprits de la nature
- **Athéisme** — Pas de religion

**Effets :**
- Bonheur (+10 à +30)
- Unité culturelle (moins de rébellions)
- Guerres saintes (bonus combat contre infidèles)
- Miracles (pouvoirs divins renforcés)

### Cultures

Chaque royaume développe une culture unique :
- Noms de villes spécifiques
- Architecture distinctive
- Drapeaux et symboles
- Langues (affecte diplomatie)

## 10. Événements aléatoires

### Événements positifs

- **Découverte de minerai** — Nouveau filon
- **Récolte abondante** — +50% nourriture
- **Bébé royal** — Héritier né
- **Invention** — Tech gratuite
- **Prophète** — Bonus religieux

### Événements négatifs

- **Famine** — Crop failure
- **Épidémie** — Maladie se propage
- **Tremblement de terre** — Détruit bâtiments
- **Invasion de monstres** — Horde de zombies
- **Rébellion** — Ville se révolte

### Événements neutres

- **Météore** — Crée cratère, donne minerais
- **Migration** — Nouveaux colons arrivent
- **Comète** — Présage (affecte moral)
- **Éclipse** — Événement religieux

## Conclusion

Ces mécaniques forment un système interconnecté où les actions du joueur et les comportements autonomes des entités créent des **récits émergents uniques**. Chaque partie est différente, chaque civilisation suit son propre destin.

**Phase suivante :** Lire le catalogue des Pouvoirs Divins pour comprendre les outils du joueur.
