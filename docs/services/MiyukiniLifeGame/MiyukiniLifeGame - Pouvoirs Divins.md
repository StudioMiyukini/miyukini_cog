# MiyukiniLifeGame — Pouvoirs Divins

## Contexte

Ce document catalogue les **pouvoirs divins** que le joueur peut utiliser pour interagir avec le monde. Inspiré des 230+ pouvoirs de WorldBox, Miyukini Life Game propose 50+ pouvoirs organisés en 7 catégories.

## Portée / Scope

- Catalogue complet des pouvoirs
- Organisation en catégories
- Coûts et limitations
- Effets visuels
- Intégration avec la gouvernance (StrongFather)

## Organisation des pouvoirs

### 7 catégories principales

1. **🌍 Création** — Créer terrain, ressources, vie
2. **💥 Destruction** — Détruire, catastrophes naturelles
3. **🐉 Créatures** — Invoquer créatures et monstres
4. **✨ Magiques** — Bénédictions, malédictions, enchantements
5. **⏱️ Contrôle du temps** — Vitesse, pause, avance rapide
6. **🎯 Spéciaux** — Interactions directes (aimant, inspection)
7. **🎨 Effets** — Purement esthétiques

## Philosophie des pouvoirs

> "Un dieu n'a pas de mana. Il décide."

**Principes :**
- ✅ **Pas de mana** — Aucune ressource à gérer
- ✅ **Cooldowns courts** — 0-5 secondes entre utilisations
- ✅ **Sensation de puissance** — Effets visibles et impactants
- ✅ **Conséquences réalistes** — Chaque pouvoir affecte l'écosystème
- ⚠️ **Limitations de sécurité** — WorrySentinel empêche les abus

## 1. 🌍 Pouvoirs de Création

### Terrain

| Pouvoir | Description | Effet | Visuel |
|---------|-------------|-------|--------|
| **Terre** | Crée du terrain solide | Transforme eau en terre | Particules brunes |
| **Eau** | Crée de l'eau | Transforme terre en eau | Éclaboussures bleues |
| **Montagne** | Élève le terrain | +10 élévation | Rochers qui sortent |
| **Forêt** | Plante des arbres | Place 5-10 arbres | Pousses vertes |
| **Sable** | Crée du désert | Transforme en sable | Nuage jaune |
| **Neige** | Couvre de neige | Abaisse température | Flocons blancs |
| **Plage** | Crée une côte | Terrain plat près de l'eau | Sable doré |

### Ressources

| Pouvoir | Description | Effet | Durée |
|---------|-------------|-------|-------|
| **Minerai de fer** | Place du fer | Gisement extractible | Permanent |
| **Minerai d'or** | Place de l'or | Gisement précieux | Permanent |
| **Diamant** | Place des diamants | Gisement rare | Permanent |
| **Charbon** | Place du charbon | Énergie | Permanent |
| **Pierre** | Place de la pierre | Construction | Permanent |
| **Forêt dense** | Place 20+ arbres | Bois abondant | Permanent |

### Vie

| Pouvoir | Description | Effet | Spawn |
|---------|-------------|-------|-------|
| **Humains** | Place 1-5 Humains | Nouvelle unité | Aléatoire |
| **Orcs** | Place 1-5 Orcs | Nouvelle unité | Aléatoire |
| **Elfes** | Place 1-5 Elfes | Nouvelle unité | Aléatoire |
| **Nains** | Place 1-5 Nains | Nouvelle unité | Aléatoire |
| **Animaux** | Place des animaux | Cerfs, moutons | 3-10 |
| **Poissons** | Remplit l'eau | Nourriture aquatique | Dans l'eau |

### Autres

| Pouvoir | Description | Effet | Visuel |
|---------|-------------|-------|--------|
| **Nourriture** | Donne de la nourriture | +100 nourriture au royaume | Pain, fruits |
| **Ressources** | Donne ressources diverses | +50 bois, pierre, fer | Caisses |
| **Armes** | Équipe des soldats | Armes aléatoires | Épées brillantes |
| **Soigner** | Guérit toutes blessures | HP = max | Aura verte |
| **Fertiliser** | Rend le sol fertile | +50% rendement fermes | Paillettes dorées |

## 2. 💥 Pouvoirs de Destruction

### Catastrophes naturelles

| Pouvoir | Description | Dégâts | Rayon | Visuel |
|---------|-------------|--------|-------|--------|
| **⚡ Éclair** | Foudre divine | 50 HP | 3 tuiles | Flash blanc |
| **🌪️ Tornade** | Tourbillon destructeur | 20 HP/sec | 7 tuiles | Spirale grise |
| **🌋 Volcan** | Éruption volcanique | 100 HP | 15 tuiles | Lave rouge |
| **🗿 Tremblement de terre** | Secoue le sol | 30 HP, détruit bâtiments | 20 tuiles | Fissures |
| **🌊 Tsunami** | Vague géante | 80 HP | 30 tuiles (côtier) | Vague bleue |
| **💧 Pluie acide** | Pluie toxique | 10 HP/sec pendant 10s | 20 tuiles | Gouttelettes vertes |
| **❄️ Blizzard** | Tempête de neige | 5 HP/sec, ralentit | 15 tuiles | Flocons épais |
| **🔥 Incendie de forêt** | Feu qui se propage | 30 HP/sec | Se propage | Flammes orange |

### Armes divines

| Pouvoir | Description | Dégâts | Rayon | Cooldown |
|---------|-------------|--------|-------|----------|
| **☄️ Météorite** | Chute de rocher spatial | 200 HP | 10 tuiles | 5s |
| **💣 Bombe** | Explosion classique | 150 HP | 8 tuiles | 2s |
| **☢️ Bombe atomique** | Arme ultime | 1000 HP | 50 tuiles | 30s |
| **🔥 Rayon laser** | Faisceau de feu | 100 HP continu | Ligne de 20 tuiles | 1s |
| **⚡ Rayon de foudre** | Chaîne d'éclairs | 50 HP × 5 cibles | Chaîne de 10 tuiles | 3s |

### Fléaux

| Pouvoir | Description | Effet | Durée |
|---------|-------------|-------|-------|
| **☠️ Peste** | Maladie contagieuse | -5 HP/jour, contagieux | 30 jours |
| **🦗 Invasion de sauterelles** | Détruit récoltes | -80% nourriture | 7 jours |
| **🌑 Sécheresse** | Pas de pluie | -50% rendement fermes | 90 jours |
| **🌊 Inondation** | Eau monte | Détruit bâtiments bas | 10 jours |
| **🌪️ Tempête de sable** | Ensevelit tout | Réduit vision, -20 HP/jour | 5 jours |

## 3. 🐉 Pouvoirs de Créatures

### Créatures hostiles

| Pouvoir | Créature | Quantité | PV | Comportement |
|---------|----------|----------|-----|--------------|
| **🧟 Zombies** | Mort-vivants | 5-10 | 50 | Attaquent vivants, infectent |
| **💀 Squelettes** | Archers morts | 3-7 | 40 | Attaquent à distance |
| **👹 Démons** | Démons de feu | 1-3 | 200 | Très agressifs, feu |
| **🐉 Dragon** | Dragon adulte | 1 | 1000 | Vole, crache feu, contrôlable |
| **🦀 Crabzilla** | Crabe géant | 1 | 5000 | Boss, contrôlable |
| **🐺 Loups enragés** | Meute hostile | 8-15 | 60 | Attaquent en meute |
| **🧟 Horde zombie** | Invasion massive | 50+ | 50 | Déferlent sur royaume |

### Créatures magiques

| Pouvoir | Créature | Effet | Durée |
|---------|----------|-------|-------|
| **🦄 Licornes** | Guérisseuses | Soignent terrain corrompu | Permanent |
| **🌳 Treants** | Gardiens de forêt | Protègent arbres | Permanent |
| **🧚 Fées** | Esprits lumineux | +50% croissance plantes | Permanent |
| **🗿 Golems** | Gardiens de pierre | Défendent zone | Permanent |
| **🐉 Dragon de glace** | Dragon glacial | Gèle ennemis | Permanent |
| **🔥 Élémentaire de feu** | Esprit de feu | Attaque avec feu | Permanent |

### Infestations

| Pouvoir | Description | Effet | Propagation |
|---------|-------------|-------|-------------|
| **🦠 Tumeur** | Masse corrompue | Corrompt terrain | 1 tuile/jour |
| **🕷️ Araignées géantes** | Nid d'araignées | Attaquent habitants | Se reproduisent |
| **🐀 Rats** | Invasion de rongeurs | Mangent nourriture | Rapide |
| **👻 Fantômes** | Esprits hantés | Effraient population | Immobiles |

## 4. ✨ Pouvoirs Magiques

### Bénédictions

| Pouvoir | Description | Effet | Cible | Durée |
|---------|-------------|-------|-------|-------|
| **🛡️ Bouclier divin** | Protection magique | Invincibilité | 1 unité | 60s |
| **⚡ Force divine** | Décuple la puissance | Attaque ×5 | 1 unité | 30s |
| **⚡ Vitesse divine** | Course surhumaine | Vitesse ×3 | 1 unité | 60s |
| **💎 Immortalité** | Ne vieillit plus | Âge figé | 1 unité | Permanent |
| **✨ Régénération** | Guérison continue | +10 HP/sec | 1 unité | 30s |
| **🌟 Charisme** | Adoré de tous | +50 diplomatie | 1 roi | Permanent |
| **🧠 Intelligence** | Génie scientifique | Tech ×2 | 1 royaume | Permanent |

### Malédictions

| Pouvoir | Description | Effet | Cible | Durée |
|---------|-------------|-------|-------|-------|
| **☠️ Malédiction de faiblesse** | Affaiblit | Attaque ÷2 | 1 unité | 60s |
| **🐌 Malédiction de lenteur** | Ralentit | Vitesse ÷3 | 1 unité | 60s |
| **🤢 Malédiction de maladie** | Malade | -5 HP/sec | 1 unité | 30s |
| **😱 Malédiction de folie** | Devient fou | Attaque alliés | 1 unité | 60s |
| **👻 Malédiction de mort** | Mort assurée | Meurt dans 24h | 1 unité | 24h |
| **💔 Malédiction de haine** | Haï de tous | -50 diplomatie | 1 roi | Permanent |

### Enchantements de zone

| Pouvoir | Description | Effet | Rayon | Durée |
|---------|-------------|-------|-------|-------|
| **🌈 Terre sacrée** | Zone bénie | +20 bonheur, guérit | 15 tuiles | Permanent |
| **💀 Terre corrompue** | Zone maudite | -20 bonheur, maladie | 15 tuiles | Permanent |
| **🔥 Terre enflammée** | Sol brûlant | 10 HP/sec | 10 tuiles | 60s |
| **❄️ Terre gelée** | Sol glacé | Ralentit ×2 | 10 tuiles | 60s |
| **✨ Terre magique** | Magie amplifiée | Pouvoirs ×2 | 10 tuiles | Permanent |

## 5. ⏱️ Contrôle du temps

| Pouvoir | Description | Effet | Touche |
|---------|-------------|-------|--------|
| **⏸️ Pause** | Arrête le temps | Simulation = 0 | Espace |
| **▶️ Vitesse normale** | 1× | Temps réel | 1 |
| **⏩ Vitesse ×2** | 2× | Accéléré | 2 |
| **⏩ Vitesse ×5** | 5× | Rapide | 3 |
| **⏩ Vitesse ×10** | 10× | Très rapide | 4 |
| **🚀 Avance rapide** | Saute dans le temps | +1 jour instantané | Shift+Espace |

**Notes :**
- Pas de limite d'utilisation
- Peut changer à tout moment
- Pause ne consomme rien
- Stats continuent d'être calculées

## 6. 🎯 Pouvoirs Spéciaux

### Interaction directe

| Pouvoir | Description | Effet | Usage |
|---------|-------------|-------|-------|
| **🧲 Aimant divin** | Attrape et déplace | Déplace 1 entité | Clic+glisser |
| **🔍 Inspection** | Infos détaillées | Affiche stats | Clic droit |
| **🎯 Sélection** | Sélectionne zone | Sélectionne entités | Clic+rectangle |
| **📷 Screenshot** | Capture d'écran | Sauvegarde image | F12 |
| **💾 Sauvegarde rapide** | Sauvegarde instantanée | Sauvegarde monde | Ctrl+S |
| **🗑️ Effacer** | Supprime entité | Supprime 1 entité | Suppr |

### Contrôle de créatures

| Pouvoir | Description | Effet | Contrôle |
|---------|-------------|-------|----------|
| **🐉 Contrôler dragon** | Prend contrôle | WASD pour déplacer, Espace cracher feu | Jusqu'à lâcher |
| **🦀 Contrôler Crabzilla** | Prend contrôle | WASD pour déplacer, Espace attaquer | Jusqu'à lâcher |

### Diplomatie forcée

| Pouvoir | Description | Effet | Durée |
|---------|-------------|-------|-------|
| **🕊️ Forcer la paix** | Arrête une guerre | Paix immédiate | Permanent |
| **⚔️ Forcer la guerre** | Déclenche une guerre | Guerre immédiate | Jusqu'à fin guerre |
| **🤝 Forcer l'alliance** | Allie 2 royaumes | Alliance forcée | 10 ans |
| **💔 Forcer la trahison** | Rompt alliance | Trahison immédiate | Permanent |

## 7. 🎨 Pouvoirs d'Effets

### Effets esthétiques

| Pouvoir | Description | Effet | Durée |
|---------|-------------|-------|-------|
| **🎆 Feux d'artifice** | Spectacle lumineux | Particules colorées | 10s |
| **🌈 Arc-en-ciel** | Arc lumineux | Ligne colorée | 30s |
| **☄️ Comète** | Comète décorative | Trainée lumineuse | 20s |
| **✨ Étoiles filantes** | Pluie d'étoiles | Particules blanches | 15s |
| **🎇 Aurore boréale** | Lueurs nordiques | Ondulations vertes | 60s |
| **💫 Scintillement** | Fait briller | Paillettes dorées | 10s |

**Notes :**
- Aucun impact sur le gameplay
- Purement visuels
- Pas de cooldown
- Pour célébrer ou décorer

## Gouvernance des pouvoirs

### Rôle de StrongFather

Chaque pouvoir nécessite une **autorisation du Core StrongFather** :

```rust
// Exemple de flux
async fn use_power(power: PowerType, target: Target) -> Result<(), Error> {
    // 1. Demande d'autorisation
    let permission = strongfather::request_permission(
        PowerUseRequest {
            power: power,
            target: target,
            user: current_user(),
        }
    ).await?;
    
    // 2. Vérification des limites (WorrySentinel)
    worrysentinel::check_limits(power, target)?;
    
    // 3. Exécution via MasterButler
    masterbutler::orchestrate(power, target).await?;
    
    // 4. Effets via Toolkits
    execute_power_effects(power, target).await?;
    
    // 5. Sauvegarde via KindMother
    kindmother::save_world_state().await?;
    
    Ok(())
}
```

### Limites de sécurité (WorrySentinel)

| Limite | Valeur | Raison |
|--------|--------|--------|
| **Bombes atomiques/min** | 5 | Éviter lag |
| **Créatures hostiles max** | 500 | Performance |
| **Dragons max** | 10 | Équilibrage |
| **Crabzilla max** | 1 | Boss unique |
| **Taille explosion max** | 100 tuiles | Stabilité |

### Cooldowns

La plupart des pouvoirs ont des cooldowns courts :
- **0s** — Pouvoirs de création basiques
- **1-5s** — Pouvoirs de destruction moyens
- **10-30s** — Pouvoirs ultimes (nuke, Crabzilla)
- **Aucun** — Contrôle du temps, inspection, aimant

## Interface utilisateur

### Organisation de la palette

**Barre latérale (7 onglets) :**
1. 🌍 **Création** (20 pouvoirs)
2. 💥 **Destruction** (15 pouvoirs)
3. 🐉 **Créatures** (12 pouvoirs)
4. ✨ **Magiques** (18 pouvoirs)
5. ⏱️ **Temps** (6 options)
6. 🎯 **Spéciaux** (10 outils)
7. 🎨 **Effets** (6 effets)

**Affichage d'un pouvoir :**
- Icône distinctive
- Nom clair
- Description courte (tooltip)
- Cooldown affiché (si actif)
- Coût (aucun sauf exceptions)

### Raccourcis clavier

| Touche | Pouvoir |
|--------|---------|
| **1-7** | Sélectionne onglet |
| **Q-R** | Pouvoirs rapides (personnalisables) |
| **Espace** | Pause/Play |
| **Shift+Espace** | Avance rapide |
| **Ctrl+S** | Sauvegarde |
| **F12** | Screenshot |
| **Suppr** | Effacer sélection |
| **Échap** | Annuler pouvoir en cours |

## Exemples de combinaisons

### Scénarios créatifs

**Jardin d'Éden :**
1. Créer un continent isolé
2. Placer forêts et lacs
3. Remplir d'animaux pacifiques
4. Placer quelques Humains
5. Bénir la terre (terre sacrée)
6. Observer l'évolution pacifique

**Armageddon :**
1. Attendre que 4 royaumes se forment
2. Forcer la guerre entre tous
3. Invoquer dragons sur chaque capitale
4. Déclencher volcans sur frontières
5. Pluie de météorites aléatoire
6. Observer le chaos

**Tour de mage :**
1. Créer une île montagneuse
2. Placer des Elfes
3. Bénir le roi avec immortalité + intelligence
4. Donner régulièrement des ressources magiques
5. Observer la civilisation magique prospérer

## Conclusion

Les 50+ pouvoirs divins de Miyukini Life Game offrent une **liberté totale d'expérimentation**. Organisés en 7 catégories claires, gouvernés par StrongFather, et limités par WorrySentinel pour la stabilité, ils permettent au joueur d'incarner véritablement un dieu omnipotent.

**Phase suivante :** Lire le document sur le Système de Civilisations pour comprendre comment les royaumes réagissent aux interventions divines.
