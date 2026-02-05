# Miyukini Survivor — Loot, préfixes, suffixes et équipement

## Contexte

Ce document définit le **système de loot** du service Miyukini Survivor : **drop rate**, **préfixes** et **suffixes** à la manière de Diablo ou Path of Exile, et structure de la **table exhaustive d'équipement et d'objets**. Le loot est **primordial pour maintenir l'engagement** du joueur ; une liste d'objets dépassant largement la concurrence est visée.

## Portée / Scope

- **Périmètre :** Règles de génération d'objets (préfixe + base + suffixe), catégories d'équipement, types d'objets, structure des tables.
- **Hors périmètre :** Valeurs numériques exactes (équilibrage, drop rates par ennemi), implémentation technique.

---

## 1. Principes du loot

### 1.1 Drop au sol

- Les **ennemis tués** font apparaître au sol : **or**, **objets** (équipement, consommables), **orbes d'XP**.
- Les drops restent au sol un **temps limité** (par défaut 4 s) ; **clignotement** si < 2 s restantes — voir [Gameplay et Mecaniques](../Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md).

### 1.2 Drop rate

- Les **objets** ont un **drop rate** (probabilité ou taux par ennemi / par vague / par type d'ennemi).
- Le drop rate peut être modifié par : **mode Hardcore** (×2), **lune rouge**, **équipement**, **compétences**, **quêtes journalières**.

### 1.3 Objets avec préfixe et suffixe

- Les objets (équipement) peuvent avoir **un préfixe** et **un suffixe**, à la manière de **Diablo** ou **Path of Exile**.
- **Nom affiché** = `[Préfixe] [Base] [Suffixe]` (ex. *Épée de fer brutale*, *Arc ancien du chasseur*).
- Chaque **préfixe** et **suffixe** apporte des **modificateurs** (stats, effets) ; la **base** définit le type d'objet et ses stats de base.

---

## 2. Structure des objets

### 2.1 Base (type d'objet)

- **Base** = type d'équipement ou d'objet (arme, armure, gants, bottes, casque, consommable, etc.).
- Chaque base a des **stats de base** (dégâts, armure, emplacement, rareté de base).

### 2.2 Préfixe

- **Préfixe** = modificateur appliqué au **nom** et aux **stats** (ex. *Brutal*, *Ancien*, *Enflammé*).
- Un objet peut avoir **zéro ou un préfixe** (selon règles de génération).
- Les préfixes sont regroupés par **thème** (dégâts, défense, vitesse, élément, etc.) et par **niveau / rareté**.

### 2.3 Suffixe

- **Suffixe** = modificateur appliqué au **nom** et aux **stats** (ex. *du Berserker*, *du Chasseur*, *de la Vitesse*).
- Un objet peut avoir **zéro ou un suffixe** (selon règles de génération).
- Les suffixes sont regroupés par **thème** et par **niveau / rareté**.

### 2.4 Rareté et combinaisons

- **Rareté** : commun (blanc), magique (préfixe ou suffixe), rare (préfixe + suffixe), légendaire / unique (à définir).
- Les **tables de préfixes et suffixes** sont **exhaustives** : un grand nombre de combinaisons permet une liste d'objets dépassant largement la concurrence.

---

## 3. Catégories d'équipement et d'objets

Les slots d'équipement du joueur sont définis dans [Miyukini Survivor - Gameplay et Mecaniques](../Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md) (section 9). Résumé :

### 3.1 Équipement (slots du joueur)

| Slot(s) | Type d'objet |
|---------|--------------|
| **Tête** | Casque, chapeau |
| **Collier** | Amulette, collier |
| **Épaules** | Épaulières |
| **Brassard** | Brassard |
| **Gants** | Gants |
| **2× Bagues** | Anneaux |
| **Torse** | Plastron, armure de torse |
| **Ceinture** | Ceinture |
| **Jambes** | Jambières |
| **Pieds** | Bottes |
| **Main gauche** / **Main droite** | Armes, bouclier, focus |
| **Dos** | Sac, carquois, cape (un type à la fois) |
| **Monture** | Monture |
| **5 objets divers** | Talismans, artefacts, etc. |
| **1 slot potion vie** | Potion de vie (combat) |
| **1 slot potion mana** | Potion de mana (combat) |

### 3.2 Consommables

- Potions (PV, mana) — slots dédiés ; buffs temporaires (à définir).
- Parchemins, objets à usage unique (à définir).

### 3.3 Objets spéciaux

- Objets de quête, matériaux de craft, clés, etc. (à définir).

---

## 4. Tables préfixes et suffixes (structure)

Les tables ci-dessous donnent la **structure** et des **exemples** ; la liste exhaustive est à maintenir en annexe ou dans un fichier de données (JSON, etc.).

### 4.1 Exemples de préfixes (thèmes)

| Thème | Exemples de préfixes | Effet type |
|-------|----------------------|------------|
| Dégâts | Brutal, Sanglant, Perçant | + dégâts physiques |
| Élément | Enflammé, Givré, Foudroyant | + dégâts élémentaires |
| Défense | Renforcé, Cuirassé, Gardien | + armure / PV |
| Vitesse | Rapide, Agile, Vif | + vitesse attaque / déplacement |
| Rareté / ancien | Ancien, Antique, Légendaire | + stats, rareté |

### 4.2 Exemples de suffixes (thèmes)

| Thème | Exemples de suffixes | Effet type |
|-------|----------------------|------------|
| Classe / style | du Berserker, du Chasseur, du Mage | + stats orientées classe |
| Propriété | de la Vitesse, de la Force, de la Précision | + stat secondaire |
| Effet | de la Vie, du Vampire, de l'Éclair | + vie, vol de vie, élément |

### 4.3 Table exhaustive

- Une **table exhaustive** regroupe **tous les préfixes** et **tous les suffixes** autorisés par base (ou par catégorie).
- Chaque entrée associe : **id**, **nom**, **effets (modificateurs)**, **rareté min**, **niveau requis**, **tags** (pour filtres, achievements).
- La combinaison **base + préfixe + suffixe** est générée selon le **drop rate** et les **règles de compatibilité** (ex. certains préfixes uniquement sur armes).

---

## 5. Références

- [Miyukini Survivor - Gameplay et Mecaniques](../Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md) — drops, inventaire, modes.
- [Miyukini Survivor - Document Fondateur](../Miyukini%20Survivor%20-%20Document%20Fondateur.md)

---

**Document créé le :** 2026-02-04  
**Dernière mise à jour :** 2026-02-04  
**Statut :** Référence ; tables exhaustives à compléter (liste complète des préfixes/suffixes et bases).
