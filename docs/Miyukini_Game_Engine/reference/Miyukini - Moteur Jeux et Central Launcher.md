# Miyukini — Moteur de Jeu et Central Launcher

Document fondateur de l'architecture jeux Miyukini : moteur maison en Rust, jeux en exécutables séparés, Central comme launcher type Steam, téléchargement à la demande.

## Portée / Scope

- **Applicable à :** Moteur de jeu, jeux (ex. Allumina), Central, distribution des jeux.
- **Audience :** Architecture, game design, distribution.
- **Statut :** Document fondateur normatif.

---

## 1. Vision

> **Central allégé** : le joueur lance les jeux depuis Central (comme sur Steam), mais les jeux ne sont **pas embarqués** dans Central. Chaque jeu a son propre `.exe` et est téléchargé séparément.

| Principe | Description |
|----------|-------------|
| **Central = launcher** | Central affiche le catalogue des jeux, permet de les lancer, mais n'inclut pas les binaires des jeux. |
| **Un exe par jeu** | Chaque jeu (Allumina, etc.) est un exécutable autonome, compilé et distribué indépendamment. |
| **Téléchargement séparé** | Les jeux sont téléchargés à la demande (depuis Origin MWS, dépôt, ou autre source). Central ne les embarque pas. |
| **Moteur maison** | Un moteur de jeu en Rust, développé en interne, sans dépendance à des moteurs tiers lourds. |

---

## 2. Architecture

### 2.1 Moteur de jeu (Rust)

- **Crate(s) dédiés** : moteur 2D (et éventuellement 3D) en Rust, léger, adapté aux besoins Miyukini.
- **Binaire statique** : pas de runtime externe, conformité LOI-1.
- **Intégration Cores** : KindMother (sauvegardes), MWS (multijoueur), StrongFather (autorisation) selon les besoins de chaque jeu.
- **Évolutif** : suffisamment simple pour un MVP (sprite, déplacement, collisions), extensible pour les jeux complexes (Allumina, etc.).

### 2.2 Jeux = exécutables séparés

| Aspect | Détail |
|--------|--------|
| **Compilation** | Chaque jeu est un crate/binaire distinct (ex. `allumina.exe`, future génération). |
| **Lancement** | Central appelle l'exécutable du jeu (comme pour `kindmother-server`) : `Command::new(path_to_game_exe).spawn()`. |
| **Emplacement** | Jeux installés dans un répertoire dédié (ex. `%LOCALAPPDATA%/Miyukini-COG/games/Allumina/`) ou à côté de Central selon la politique d'installation. |
| **Mise à jour** | Chaque jeu peut être mis à jour indépendamment (téléchargement incrémental, gestion de versions via Origin ou dépôt). |

### 2.3 Central = launcher

- **Pas d'embarquement** : Central ne contient pas les binaires des jeux. Il les découvre, les propose au téléchargement, et les lance.
- **Catalogue** : liste des jeux disponibles (installés ou non) ; pour les jeux non installés, bouton « Télécharger ».
- **Lancement** : pour un jeu installé, Central exécute le `.exe` du jeu (chemin connu ou configuré).
- **Allègement** : Central reste léger ; seuls KindMother, MWS client, et l'UI Dioxus sont nécessaires au cœur. Les jeux sont des extensions optionnelles.

### 2.4 Téléchargement des jeux

- **Source** : Origin MWS (serveur Miyukini), dépôt GitHub releases, ou autre CDN.
- **Workflow** : utilisateur clique « Télécharger Allumina » → Central récupère l’archive ou le binaire → extraction dans le répertoire des jeux → jeu prêt à lancer.
- **Versioning** : chaque jeu a une version ; Central peut vérifier les mises à jour et proposer de télécharger une nouvelle version.

---

## 3. Flux utilisateur

```
1. Utilisateur ouvre Central
2. Central affiche : Services (KindMother, MWS, etc.) + Catalogue Jeux
3. Si jeu non installé :
   - Bouton « Télécharger »
   - Central télécharge → extrait → enregistre le chemin
4. Si jeu installé :
   - Bouton « Jouer »
   - Central lance le .exe du jeu (processus fils)
5. Le jeu s’exécute en processus séparé (comme kindmother-server)
```

---

## 4. Structure proposée (workspace)

```
crates/
  miyukini-game-engine/    # Moteur de jeu maison (lib)
  # ou miyu-game-engine, miyugame, etc.
apps/
  central/                 # Launcher (existant, allégé)
  origin/                  # MWS Origin (existant)
  allumina/                # Jeu Allumina (future, exe séparé)
  # ou dans games/allumina/
```

**Alternative** : un dossier `games/` à la racine avec un sous-dossier par jeu, chaque jeu ayant son propre `Cargo.toml` et binaire.

---

## 5. Alignement LOI

- **LOI-1** : Pas de dépendance externe critique — moteur maison, binaires statiques.
- **LOI-2** : Jeux jouables hors-ligne (solo) ; MWS optionnel pour le multijoueur.
- **LOI-3** : État local souverain — sauvegardes via KindMother, données utilisateur maîtrisées.
- **LOI-5** : Coût proportionnel — jeux téléchargés à la demande, pas de surcoût inutile.

---

## 6. Références

| Document | Rôle |
|----------|------|
| [Allumina - Document Fondateur](../services/Allumina/Allumina%20-%20Document%20Fondateur.md) | Vision Allumina (adapter stack : moteur Miyukini) |
| [Miyukini Central - Démarrage](../services/MiyukiniCentral/Miyukini%20Central%20-%20Demarrage%20dependances%20et%20KindMother.md) | Pattern lancement processus (kindmother-server) |
| [kindmother_launcher.rs](../../apps/central/src/kindmother_launcher.rs) | Implémentation actuelle du lancement d'exe par Central |

---

**Document** : Miyukini — Moteur de Jeu et Central Launcher  
**Version** : 1.0  
**Date** : 2026-02-18
