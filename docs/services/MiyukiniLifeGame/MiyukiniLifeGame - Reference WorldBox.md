# MiyukiniLifeGame — Référence WorldBox

## Contexte

Ce document analyse en détail **WorldBox**, le jeu qui inspire Miyukini Life Game. Il catalogue ses mécaniques, fonctionnalités, forces et faiblesses, et définit ce que nous en reprenons ou améliorons.

## Portée / Scope

- Présentation générale de WorldBox
- Catalogue des fonctionnalités
- Mécaniques de jeu détaillées
- Analyse des forces et faiblesses
- Comparaison avec Miyukini Life Game

## Présentation de WorldBox

### Identité

**WorldBox: God Simulator**
- **Développeur :** Maxim Karpenko (solo dev)
- **Éditeur :** Maxim Karpenko
- **Date de sortie :**
  - Prototype Flash : 2012
  - Android : 2018
  - Steam Early Access : 2 décembre 2021
- **Plateformes :** Steam (Windows, macOS, Linux), iOS, Android
- **Genre :** God Simulator / Sandbox / Simulation de civilisations
- **Prix :** 19,50€ (Steam)
- **Statut :** Early Access (prévu 2 ans, toujours actif en 2026)

### Description officielle

> "WorldBox est le simulateur de dieu ULTIME et jeu sandbox. Une boîte de Petri pour vos civilisations fantastiques. Créez votre propre monde ou détruisez-le en utilisant de nombreux pouvoirs. Observez les civilisations grandir, former des royaumes, coloniser de nouvelles terres, naviguer vers des continents lointains, se faire la guerre, voir des empires tomber et des villes brûler !"

### Philosophie

**Trois piliers :**
1. **Liberté totale** — Aucun objectif imposé, aucune contrainte
2. **Émergence narrative** — Chaque partie raconte une histoire unique
3. **Simplicité puissante** — Règles simples → comportements complexes

## Fonctionnalités principales

### 1. Création de monde

**Outils de terrain :**
- **Pinceaux** — Formes et tailles variées (cercle, carré)
- **Types de terrain :**
  - Eau (profonde, peu profonde)
  - Terre (herbe, sable, neige)
  - Montagne
  - Forêt
  - Lave
  - Marais
  - Désert
  - Toundra

**Génération procédurale :**
- Seed personnalisable
- Tailles : Petit, Moyen, Grand, Géant
- Présets : Continents, Archipel, Pangée

**Biomes :**
- 10+ biomes distincts
- Températures et précipitations
- Flore et faune spécifiques

### 2. Races et civilisations

**4 races jouables :**

| Race | Traits | Biome préféré | Style |
|------|--------|---------------|-------|
| **Humains** | Équilibrés, adaptables | Plaines | Expansion rapide |
| **Orcs** | Forts, agressifs | Montagnes | Guerriers |
| **Elfes** | Longue vie, pacifiques | Forêts | Magie et nature |
| **Nains** | Technologie, défense | Montagnes | Artisans |

**Comportements autonomes :**
- Construisent villages, routes, ports
- Forment royaumes avec frontières
- Élisent des rois
- Développent cultures (noms, drapeaux, langues)
- Progressent technologiquement (pierre → bronze → fer → médiéval)

### 3. Diplomatie et guerres

**Systèmes diplomatiques :**
- Relations dynamiques (-100 à +100)
- Alliances et ennemis
- Traités commerciaux
- Guerres et conquêtes
- Rébellions

**Guerres :**
- Déclarations automatiques (relations < -50)
- Batailles en temps réel
- Sièges de villes
- Conquêtes territoriales
- Traités de paix

### 4. Pouvoirs divins

**230+ pouvoirs organisés en 7 onglets :**

#### Tab 1 : Créatures
- Humains, Orcs, Elfes, Nains
- Animaux (moutons, loups, vaches)
- Monstres (zombies, squelettes, démons)
- Dragons, UFOs
- **Crabzilla** (boss contrôlable)

#### Tab 2 : Destruction
- Éclairs ⚡
- Tornades 🌪️
- Volcans 🌋
- Tremblements de terre
- Météorites ☄️
- Bombes (classiques, atomiques) 💣
- Pluie acide
- Rayons laser

#### Tab 3 : Nature
- Arbres
- Plantes
- Fleurs
- Animaux sauvages
- Ressources naturelles

#### Tab 4 : Civilisation
- Ressources (bois, pierre, or)
- Nourriture
- Armes et armures
- Bâtiments

#### Tab 5 : Effets
- Bénédictions (force, vitesse, immortalité)
- Malédictions (faiblesse, folie, mort)
- Soins
- Zombification

#### Tab 6 : Outils
- Aimant divin (déplace entités)
- Inspection (stats détaillées)
- Sélection de zone
- Effacer

#### Tab 7 : Temps et paramètres
- Pause / Play
- Vitesses (1×, 2×, 5×, 10×)
- Options de simulation

### 5. Créatures spéciales

**Créatures hostiles :**
- **Zombies** — Infectent morts, créent hordes
- **Squelettes** — Archers rapides
- **Démons** — Très puissants, portails infernaux
- **Dragons** — Volent, crachent feu, 1000 HP
- **Tumeurs** — Corrompent terrain
- **Cold Ones** — Créatures de glace
- **UFOs** — Enlèvent unités

**Boss :**
- **Crabzilla** — Crabe géant de 5000 HP, contrôlable, détruit tout

**Créatures magiques :**
- Licornes
- Treants
- Fées
- Golems

### 6. Systèmes avancés

**Génétique :**
- Traits héréditaires
- Mutations
- Sélection naturelle

**Religions :**
- Formation spontanée
- Temples et prêtres
- Guerres saintes

**Familles :**
- Arbre généalogique
- Dynasties royales
- Héritages

**Langues :**
- Chaque royaume développe sa langue
- Alphabets générés procéduralement
- Noms de villes et personnes uniques

**Histoire :**
- Timeline complète
- Événements marquants enregistrés
- Statistiques détaillées

### 7. Interface et outils

**UI :**
- Palette de pouvoirs (barre latérale)
- Panneau de statistiques
- Sélecteur de royaume
- Graphiques de population
- Timeline historique

**Outils avancés :**
- Éditeur de cartes
- Partage de mondes (Steam Workshop)
- Mods (communauté très active)

## Métriques et performance

### Statistiques Steam

**Reviews :**
- **Nombre total :** 45,669 avis
- **Score :** Overwhelmingly Positive (95-96%)
- **Avis récents (30 jours) :** 928, 95% positifs
- **Avis anglais :** 26,541, 96% positifs

**Popularité :**
- Top 100 jeux Sandbox sur Steam
- Communauté très active (100,000+ membres Discord/Reddit)

**Langues :**
- 27 langues supportées (traduites par la communauté)

### Configuration requise

**Minimale :**
- OS : Windows 10
- Processeur : 2.0 Ghz
- RAM : 2.5 GB
- Carte graphique : 128 MB, Shader Model 2.0+
- Stockage : 300 MB

**Performance réelle :**
- Très léger et performant
- Fonctionne sur machines modestes
- Pixel art = peu de ressources

## Forces de WorldBox

### 1. Simplicité et accessibilité

✅ **UI intuitive** — Tout est clair et facile à trouver
✅ **Pas de tutoriel obligatoire** — On apprend en jouant
✅ **Pas de game over** — Liberté totale
✅ **Pas de mana** — Tous les pouvoirs toujours disponibles

### 2. Profondeur émergente

✅ **Systèmes interconnectés** — Tout affecte tout
✅ **Récits uniques** — Chaque partie est différente
✅ **Dynasties et histoire** — Attachement aux personnages
✅ **Guerres épiques** — Batailles imprévisibles

### 3. Rejouabilité infinie

✅ **Mondes infinis** — Seeds aléatoires
✅ **Scénarios variés** — Jardin d'Éden vs Armageddon
✅ **Modding** — Steam Workshop très actif
✅ **Défis communautaires** — Joueurs créent leurs propres objectifs

### 4. Développement actif

✅ **Updates régulières** — Nouvelles fonctionnalités fréquentes
✅ **Écoute de la communauté** — Suggestions implémentées
✅ **Solo dev passionné** — Maxim Karpenko très impliqué
✅ **Roadmap claire** — Objectifs annoncés

### 5. Performance

✅ **Très optimisé** — Fonctionne sur vieux PC
✅ **Pixel art** — Léger et rapide
✅ **Sauvegarde rapide** — Fichiers compressés
✅ **Pas de bugs majeurs** — Très stable

## Faiblesses de WorldBox

### 1. Limitations techniques

❌ **Pas de multithreading** — Ralentit avec 1000+ entités
❌ **Pathfinding simple** — Unités parfois bloquées
❌ **Pas de physique avancée** — Interactions limitées
❌ **Sauvegarde locale uniquement** — Pas de cloud sync

### 2. UI et UX

❌ **Trop de pouvoirs** — 230+ = difficile de tout trouver
❌ **Pas de recherche** — Faut scrolller dans les menus
❌ **Manque de tooltips** — Certains pouvoirs peu clairs
❌ **Pas de raccourcis clavier personnalisables**

### 3. Gameplay

❌ **Pas d'objectifs** — Certains joueurs se sentent perdus
❌ **Répétitivité** — Après 50h, on a tout vu
❌ **Diplomatie limitée** — Pas assez de profondeur
❌ **IA parfois stupide** — Décisions illogiques

### 4. Technique

❌ **Early Access depuis 5 ans** — Pas de date de sortie v1.0
❌ **Code fermé** — Pas de moddabilité profonde
❌ **Pas de multiplayer** — Uniquement solo
❌ **Pas de mobile web** — Seulement apps natives

## Comparaison : WorldBox vs Miyukini Life Game

### Ce que nous reprenons

✅ **Philosophie god simulator** — Liberté totale, pas d'objectifs
✅ **4 races civilisées** — Humains, Orcs, Elfes, Nains
✅ **Autonomie des civilisations** — Comportements émergents
✅ **Diversité des pouvoirs** — Création ET destruction
✅ **Pixel art** — Style simple et performant
✅ **Pas de mana** — Pouvoirs toujours disponibles
✅ **Génération procédurale** — Seeds, biomes
✅ **Diplomatie et guerres** — Relations dynamiques

### Ce que nous améliorons

🚀 **Architecture Miyukini COG**
- Gouvernance intégrée (Cores)
- Extensibilité via Toolkits
- Sécurité (WorrySentinel)
- Permissions (StrongFather)

🚀 **Sauvegarde synchronisée**
- Via KindMother
- Cloud sync automatique
- Multi-device (Desktop + Web)

🚀 **Performance Rust**
- Multithreading (rayon)
- Spatial hashing
- Optimisations mémoire

🚀 **UI moderne**
- Dioxus (réactif)
- Recherche de pouvoirs
- Tooltips détaillés
- Raccourcis personnalisables

🚀 **Statistiques avancées**
- CaringNanny pour métriques
- Graphiques détaillés
- Timeline interactive
- Comparaisons entre royaumes

🚀 **Web natif**
- WASM full support
- Pas besoin d'installation
- Partage facile de mondes

🚀 **Moddabilité**
- Nouveaux pouvoirs via Toolkits
- Nouvelles races extensibles
- API publique documentée

### Ce que nous simplifions pour le MVP

⚠️ **Moins de pouvoirs** — 50 au lieu de 230 (pour commencer)
⚠️ **Pas de génétique avancée** — Traits simples
⚠️ **Pas de religions** — Dans v1.0 seulement
⚠️ **Pas de langues générées** — Noms prédéfinis
⚠️ **Pas de Steam Workshop** — Intégration future

## Tableau comparatif détaillé

| Aspect | WorldBox | Miyukini Life Game |
|--------|----------|-------------------|
| **Développement** | Solo dev (5 ans) | Équipe Miyukini |
| **Architecture** | Standalone monolithique | Miyukini COG (modulaire) |
| **Plateforme** | Steam, iOS, Android | Desktop, Web (WASM) |
| **Langage** | ? (probablement C#/Unity) | Rust |
| **UI** | Custom engine | Dioxus |
| **Sauvegarde** | Local uniquement | KindMother (sync cloud) |
| **Pouvoirs** | 230+ | MVP: 50+, extensible |
| **Races** | 4 (fixes) | 4 + extensible |
| **Performance** | Bonne (1000 entités) | Excellente (2000+ entités) |
| **Modding** | Steam Workshop (limité) | Toolkits Miyukini (profond) |
| **Prix** | 19,50€ | Gratuit (inclus dans COG) |
| **Open source** | Non | Miyukini COG (licence à définir) |
| **Multiplayer** | Non | Futur (Inter-COG) |
| **Statut** | Early Access | En développement |

## Inspirations des fonctionnalités

### Pouvoirs repris directement

| WorldBox | Miyukini Life Game | Priorité |
|----------|-------------------|----------|
| ⚡ Éclair | ⚡ Éclair | MVP |
| 🌪️ Tornade | 🌪️ Tornade | MVP |
| 🌋 Volcan | 🌋 Volcan | v1.0 |
| ☄️ Météorite | ☄️ Météorite | v1.0 |
| 💣 Bombe | 💣 Bombe | MVP |
| ☢️ Bombe atomique | ☢️ Bombe atomique | v1.0 |
| 🧟 Zombies | 🧟 Zombies | v1.0 |
| 🐉 Dragon | 🐉 Dragon | v1.0 |
| 🦀 Crabzilla | 🦀 Crabzilla | v1.0 |
| 🌳 Arbres | 🌳 Forêt | MVP |
| 💧 Eau | 💧 Eau | MVP |
| 🌍 Terre | 🌍 Terre | MVP |

### Mécaniques reprises

✅ **Formation de villages** — Automatique à partir de 3+ unités
✅ **Élection de rois** — Basée sur prestige
✅ **Guerres automatiques** — Déclenchées par relations basses
✅ **Batailles en temps réel** — Calcul de puissance
✅ **Dynasties** — Arbres généalogiques
✅ **Technologies** — Âges progressifs
✅ **Biomes** — Affectent gameplay
✅ **Effets météo** — Catastrophes naturelles

## Leçons apprises de WorldBox

### Ce qui fonctionne bien

1. **Simplicité des règles** — Règles simples → émergence complexe
2. **Pas de tutoriel forcé** — Découverte naturelle
3. **Pixel art efficace** — Léger, rapide, charme rétro
4. **Timeline historique** — Crée attachement émotionnel
5. **Contrôle du temps** — Essentiel (pause, accélération)
6. **Aimant divin** — Interaction directe satisfaisante
7. **Effets visuels** — Explosions, particules = sensation de pouvoir

### Ce qu'on évitera

1. **Trop de pouvoirs d'un coup** — Progression graduelle
2. **UI surchargée** — Organisation claire dès le début
3. **Pathfinding naïf** — Utiliser A* ou flow fields
4. **Pas de multithreading** — Rust + rayon dès le départ
5. **Sauvegarde locale seulement** — KindMother sync
6. **Pas de metrics** — CaringNanny pour observer

## Conclusion

WorldBox est un **excellent jeu** qui a démontré la viabilité et l'attrait du genre god simulator sandbox. Avec 45,000+ avis positifs sur Steam, une communauté active, et un développement continu depuis 2012, c'est une référence incontournable.

Miyukini Life Game s'en inspire profondément tout en apportant des **innovations significatives** :
- Architecture gouvernée (Miyukini COG)
- Performance supérieure (Rust, multithreading)
- Sauvegarde synchronisée (KindMother)
- Extensibilité (Toolkits)
- Web natif (WASM)

Notre objectif n'est **pas de remplacer WorldBox**, mais de créer un god simulator **intégré dans l'écosystème Miyukini**, avec une architecture solide permettant des évolutions futures (Inter-COG, nouveaux pouvoirs, nouvelles races).

## Sources

- [WorldBox sur Steam](https://store.steampowered.com/app/1206560)
- [WorldBox sur l'App Store](https://apps.apple.com/us/app/worldbox-god-sandbox/id1450941371)
- Recherches web (février 2026)
- Analyse communautaire Steam
