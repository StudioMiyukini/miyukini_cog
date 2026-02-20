# MGE — Miyukini Game Engine — Référence technique

Spécification des capacités du moteur de jeu Miyukini (MGE). Liste structurée des modules et fonctionnalités à aborder.

## Portée / Scope

- **Applicable à :** Développement du moteur MGE, jeux (Allumina, etc.).
- **Audience :** Développement moteur, game design, intégration jeux.
- **Statut :** Référence technique normative.

---

## 1. Affichage et rendu

| Point | Description |
|-------|-------------|
| **Affichage et résolution** | Fenêtre, résolution logique/physique, scale factor, fullscreen, vsync. |
| **Coordonnées** | Système de coordonnées 2D (monde, écran, UI) ; origine ; unités (px, tiles). |
| **Gestion des sprites** | Chargement textures, sprite sheets, atlas ; taille ; anchor/pivot. |
| **Animations de sprites** | Frames, boucles, transitions ; flip horizontal/vertical ; directions multiples. |
| **Caméra** | Suivi joueur ; zoom ; limites ; couches (parallax) ; shake. |
| **Z-order / couches** | Ordre d'affichage ; calques (arrière-plan, monde, avant-plan, UI). |
| **Particules et effets** | Système de particules ; effets visuels (traces, impacts). |
| **Monde tile-based** | Grille 2D isométrique ; tuiles terrain, objets, murs. |

---

## 2. Physique et collisions

| Point | Description |
|-------|-------------|
| **Hitbox** | Forme et taille des hitbox ; alignement sur la taille finale du sprite. |
| **Collision** | Détection collision (AABB, cercle, polygone) ; réponse (rebond, blocage). |
| **Collision layers** | Masques de collision ; qui collisionne avec qui. |

---

## 3. Déplacement et locomotion

| Point | Description |
|-------|-------------|
| **Déplacement 8 directions** | Haut, bas, gauche, droite + 4 diagonales ; input normalisé. |
| **Orientation / rotation** | Orientation des PNJ ; vitesse de rotation ; axes ; sources (mouvement, cible, waypoint). |
| **Accélération / décélération** | Interpolation de vitesse ; friction ; inertie. |
| **Vitesse max** | Limite de vitesse ; clamp par axe ou par norme. |
| **Pathfinding** | Recherche de chemin (A*, Dijkstra) ; obstacles. |
| **Navmesh** | Graphe de navigation ; zones navigables ; ports. |
| **Run / walk** | Mode course vs marche ; impact sur le bruit / aggro. |
| **Stamina** | Jauge d'endurance ; course la consomme ; régénération. |
| **Shift-clic (stand and attack)** | Rester sur place et attaquer dans une direction. |
| **Dash / esquive** | Déplacement rapide ; invincibilité ; cancel. |
| **Bateaux** | Navigation ; déplacement sur l'eau ; multi-passagers ; ancrage. |
| **Combat naval** | Cannons ; abordage ; dégâts au navire. |
| **Continents** | Plusieurs cartes ; traversée entre continents ; attente ; horaires. |
| **PNJ de téléportation** | Téléport vers des zones connues ; coût. |
| **Runes et atlas** | Marquer des lieux ; Recall et Portail vers eux. |

---

## 4. Entités et monde

| Point | Description |
|-------|-------------|
| **Unicité des entités** | ID unique ; registre ; lifecycle. |
| **Spawn** | Création d'entités ; position ; préfab ; pool d'objets. |
| **Despawn** | Destruction ; nettoyage références. |
| **Gestion des chunks** | Monde divisé en chunks ; chargement/déchargement ; culling. |
| **Monde persistant vs instancié** | Zones partagées vs privées par instance. |
| **Instances / donjons** | Zones isolées par groupe ; entrée/sortie ; durée ; difficultés ; clefs. |
| **Raids** | Instances pour grand groupe ; bosses ; phases. |
| **Respawn dynamique** | Points de spawn ; timers ; tables de spawn. |
| **World bosses / événements** | Spawns mondiaux ; participation multi-joueurs ; loot. |
| **Facets / shards** | Mondes parallèles ; miroirs ou variantes. |
| **Culling agressif** | Ne pas traiter les entités hors écran ou loin. |
| **Grands effectifs à l'écran** | Centaines ou milliers d'unités ; optimisation du rendu. |
| **Comportement en foule** | Mouvements de masse ; évitement ; boids simplifiés. |

---

## 5. Joueur et personnage

| Point | Description |
|-------|-------------|
| **Données du joueur** | Caractéristiques ; état ; persistance (KindMother). |
| **Multi-personnages** | Plusieurs personnages par compte ; sélection au login. |
| **Slots d'équipement** | Armure, armes ; stats ; apparence (transmog optionnel). |
| **Customisation** | Visage, corps ; coiffures ; teinture ; costumes ; skins ; apparence persistante. |
| **Stats** | Attaque, défense, vitesse, précision ; impact sur dégâts, esquive, etc. |
| **Moveset par personnage** | Chaque personnage jouable possède son propre arsenal d'attaques. |
| **Arme unique / signature** | Arme ou attaque spécifique au personnage. |
| **Relation / affinité** | Liens entre personnages ; dialogues ; bonus. |

---

## 6. Progression

| Point | Description |
|-------|-------------|
| **Système de niveau** | XP ; paliers ; récompenses par niveau. |
| **Gain de compétences / aptitudes** | Progression ; arbres de compétences. |
| **Arbres de talents** | Points de talent ; branches ; reset. |
| **Skills par usage** | Les compétences montent à l'utilisation ; elles peuvent baisser si non utilisées. |
| **Cap total de skills** | Plafond global réparti entre toutes les compétences. |
| **Répartition libre** | Le joueur choisit quelles compétences faire évoluer. |
| **Skill gains dégressifs** | Plus une skill est haute, plus les gains sont lents. |
| **Achievements / succès** | Objectifs ; récompenses ; progression affichée. |
| **Titres** | Succès débloqués ; affichage sous le nom. |
| **Reset quotidien / hebdo** | Limites par jour/semaine ; réinitialisation. |
| **Saisons / battle pass** | Contenu limité dans le temps ; progression saisonnière. |
| **Jobs et changement de classe** | Quêtes de changement ; évolution de carrière. |

---

## 7. Combat

| Point | Description |
|-------|-------------|
| **Action** | Actions de base ; compétences ; cooldowns ; cast time ; after-cast delay ; ressources (mana, endurance). |
| **Auto-attaque de base** | Attaque automatique ; portée ; cadence ; dégâts. |
| **Projectiles** | Création ; trajectoire ; collision ; dégâts ; durée de vie. |
| **Gestion du parcours de combat** | Ciblage ; verrouillage cible ; priorité ; ordres tactiques. |
| **Click-to-attack** | Ciblage au clic ; attaque la cible ou le lieu. |
| **Chance de toucher** | Formule attaque vs défense ; précision vs esquive. |
| **Esquive / flee** | Formule vs précision ; réduction des dégâts. |
| **Parade / block** | Parade au bouclier ; % de blocage ; contre-attaque après block. |
| **Modificateurs de taille** | Petit, Moyen, Grand ; bonus/malus de dégâts. |
| **Modificateurs de race** | Bonus/malus selon la race de la cible. |
| **Vitesse d'attaque (ASPD)** | Vitesse d'attaque ; cap ; bonus. |
| **Changement d'arme** | Swap d'arme en combat ; armes secondaires. |
| **Zone d'effet (AOE)** | Cercle, cône ; dégâts multiples ; indicateurs. |
| **Barre de cast** | Canalisation ; interruption ; feedback visuel ; progression ; annulation. |
| **Aggro / menace** | Gestion de la menace ; priorité de ciblage des ennemis. |
| **Rôles** | Tank, DPS, soigneur ; synergies. |
| **Officiers vs mooks** | Ennemis uniques vs soldats standards. |
| **Officiers alliés** | PNJ contrôlables ; ordres ; perte possible. |

---

## 8. Dégâts, résistances et effets

| Point | Description |
|-------|-------------|
| **Résistances** | Feu, froid, éclair, poison, physique, magique ; cap ; immunités. |
| **Éléments** | Feu, Eau, Vent, Terre, Saint, Obscur, Neutre. |
| **Immunités** | Monstres ou joueurs immunes à un élément ou plusieurs. |
| **Critical strike** | Coup critique ; dégâts amplifiés. |
| **Dégâts en pourcentage** | Pourcentage de vie enlevé en un coup. |
| **Vol de vie / mana** | Sur les coups. |
| **Knockback** | Repousser l'ennemi. |
| **Effets de statut** | Buffs, debuffs ; stack ; durée ; dispel. |
| **Crowd control (CC)** | Stun, root, silence, freeze, slow ; résistances. |
| **Poison over time** | Dégâts de poison dans le temps. |
| **Autres effets** | Curse, stone, aveuglement, sleep. |

---

## 9. Mort et résurrection

| Point | Description |
|-------|-------------|
| **Mort joueur / NPC** | États (mort, en récupération). |
| **Respawn** | Lieu de réapparition ; run back. |
| **Corps (corpse)** | Corps au sol avec équipement ; lootable ; décomposition ; récupération. |
| **Récupération du corps** | Retour au corpse ; risque si plusieurs corps. |
| **Résurrection** | Skill, objet, NPC ; coût ; Rez in-combat ; cooldown. |
| **Perte d'XP** | À la mort ; possibilité de perdre un niveau. |
| **Drop à la mort** | Objets et monnaie tombent au sol ; ou équipement conservé par défaut. |
| **Cimetière / point de réapparition** | Lieu de respawn. |

---

## 10. Loot et tables

| Point | Description |
|-------|-------------|
| **Tables de loot** | Par type de monstre ; rareté. |
| **Ramassage** | Automatique dans zone de pick-up du joueur. |
| **Bosses** | Spawn fixe ou long ; loot spécial ; annonce de spawn et de kill. |
| **Droits de loot** | Premier coup, dernier coup ; partage. |
| **Champion spawns** | Vagues progressives de mobs ; boss final ; artefacts. |
| **Super uniques / mini-boss** | Boss nommés ; groupe modifié ; bonus ; loot amélioré. |
| **Niveau de zone** | Niveau des monstres selon la zone ; scaling. |
| **Cartes au trésor** | Trouver une carte ; se rendre au point ; creuser ; spawn + loot. |

---

## 11. Inventaire et objets

| Point | Description |
|-------|-------------|
| **Inventaire** | Slots + poids max ; pas de drag and drop ; empilable ; tri. |
| **Stockage persistant** | Stash, coffre, stockage de guilde ; partagé entre personnages. |
| **Poids / encumbrance** | Limite de charge ; ralentissement ; surcharge. |
| **Limite de stack** | Plafond par pile. |
| **Menus contextuels** | Clic droit ; actions selon le type d'objet. |
| **Gumps / fenêtres** | Fenêtres/dialogs réutilisables ; sérialisables pour le réseau. |
| **Slots rapides** | Ceinture à potions ; raccourcis ; empilable. |
| **Prérequis** | Niveau ; stats requises pour équiper. |
| **Durabilité** | Usure ; réparation ; casse définitive. |
| **Renforcement** | Enchantement / raffinement (+N) ; risque d'échec ; over-upgrade. |
| **Affixes** | Préfixe + suffixe ; génération procédurale. |
| **Objets Set** | Pièces d'ensemble ; bonus pour 2, 3, 5+ pièces. |
| **Objets uniques** | Stats fixes ; noms et histoires. |
| **Slots et insertions** | Emplacements dans équipement ; gemmes ; cartes ; combinaisons. |
| **Objets maudits** | Ne peuvent pas être droppés ; effets négatifs. |
| **Objets bénis** | Ne tombent pas à la mort ; protection. |
| **Charrette** | Inventaire supplémentaire ; suit le personnage. |
| **Vente (vending)** | S'asseoir ; ouvrir boutique ; vendre hors-ligne. |
| **Overcharge / Bargain** | Compétences pour vendre plus cher ou acheter moins cher. |

---

## 12. Économie et commerce

| Point | Description |
|-------|-------------|
| **Commerce** | Vente / achat ; marchands ; prix. |
| **Échange entre joueurs** | Trade ; offre ; validation ; fenêtre d'échange. |
| **Devises** | Or, argent ; monnaie premium ; monnaie avec poids ; taux de change. |
| **Vendeurs joueur** | PNJ vendeurs gérés par un joueur ; vente hors-ligne. |
| **Crafting / artisanat** | Recettes ; matériaux ; niveaux d'artisanat. |
| **Récolte** | Ressources dans le monde ; outils ; respawn. |
| **Mining** | Minage de minerais ; types ; veines. |
| **Lumberjacking** | Abattage d'arbres ; types de bois. |
| **Fishing** | Pêche ; types de poissons ; trésors. |
| **Recettes et matériaux** | Combinatoire ; ressources ; qualité variable. |

---

## 13. Social et groupes

| Point | Description |
|-------|-------------|
| **Liste d'amis** | Ajout, suppression ; statut en ligne ; invite. |
| **clans** | Création ; membres ; rangs ; banque de clan ; log ; emblème. |
| **Stockage de Clan** | Coffre partagé ; dépôt/retrait par rangs. |
| **Compétences de Clan** | Skills financées par la clan ; effets de groupe. |
| **Groupe / party / raid** | Invitation ; composition ; partage de loot ; cadres de groupe. |
| **Chat** | Canaux (global, clan, groupe, trade) ; whisper ; modération. |
| **Boîte aux lettres** | Envoi d'objets/monnaie entre joueurs ; expiration ; limite. |
| **Liste noire** | Ignorer un joueur ; blocage des messages. |
| **Réputation / factions** | Rang par faction ; récompenses ; hostilité. |
| **Guerre de factions** | Ordre vs Chaos ; contrôle de villes ; bonus. |
| **Clan wars** | Guerre clan vs clan ; déclaration ; objectifs. |

---

## 14. Siège et territoire

| Point | Description |
|-------|-------------|
| **Organisation hiérarchique** | Seigneur, vassaux ; serment ; chaîne de loyauté. |
| **Roi et royaume** | Couronne ; déclaration de guerre ; tributs ; protection. |
| **Possession de château** | Une guilde contrôle un château. |
| **Siège de château** | Événement périodique ; attaque vs défense. |
| **Portes et murailles** | Destructibles ; points de contrôle ; prise et perte. |
| **Zones de contrôle** | Bases, postes, fortifications. |
| **Points de capture** | Objectifs ; temps de capture ; résistance. |
| **Armes de siège** | Catapultes ; engins ; dégâts aux structures ; siège de maisons. |
| **Gardes NPC** | Défenseurs automatiques. |
| **Taxes** | Perception sur le territoire contrôlé. |
| **Bannière** | Drapeau de guilde ; affichage. |
| **Morale des troupes** | Dépend du combat ; fuite ; renforcement. |
| **Territoire** | Contrôle de zones ; influence sur le flux. |

---

## 15. Réputation et criminalité

| Point | Description |
|-------|-------------|
| **Karma et Fame** | Réputation bien/mal ; influence les PNJ et le statut PvP. |
| **Alignement** | Loyal, Neutre, Chaotique ; affichage (couleur du nom). |
| **Système de meurtrier** | Noms rouges ; perte de stats au respawn ; statut persistant. |
| **PK (Player Kill)** | Tuer un loyal = devenir chaotique. |
| **Rédemption** | Retour au loyal ; temps ; quêtes ; expiation. |
| **Full loot PvP** | En zones hostiles, tout est droppé à la mort. |
| **Zones PK** | Zones où le PvP est autorisé ; zones sûres. |
| **Actions criminelles** | Vol, meurtre, intrusion ; flag criminel ; gardes. |
| **Lockpicking** | Crochetage ; conteneurs verrouillés. |
| **Stealing / pickpocket** | Vol dans les sacs ; flag criminel ; échec possible. |
| **Snooping** | Consulter le contenu du sac d'un autre. |

---

## 16. Magie et sorts

| Point | Description |
|-------|-------------|
| **Mana** | Ressource pour lancer les sorts. |
| **Coût en mana** | Coût par sort ; régénération. |
| **Cercles de magie** | Organisation des sorts ; puissance progressive. |
| **Composants / réagents** | Consommation de ressources pour lancer les sorts. |
| **Livres de sorts** | Sorts connus ; grimoires ; apprentissage via objet consumable. |
| **Compétences passives** | Auras ; maîtrises ; bonus permanents. |
| **Sorts de zone** | AOE ; dégâts de zone. |
| **Hotkeys** | Raccourcis ; clic gauche, clic droit. |
| **Barding / provocation** | Compétences pour faire combattre des créatures entre elles. |

---

## 17. Montures et familiers

| Point | Description |
|-------|-------------|
| **Montures** | Acquisition ; vitesses ; animations ; combat monté. |
| **Combat à cheval** | Attaques montées ; désarçonnement. |
| **Montures spéciales** | Capacités uniques. |
| **Familiers / pets** | Suivi ; buffs passifs ; cosmétiques. |
| **Capture** | Oeufs ; éclosion ; évolution. |
| **Faim / intimité** | Nourrir ; lien affectif ; évolution. |
| **Compétences de pet** | Capacités actives ou passives. |
| **Taming** | Dompter des créatures ; difficulté par type. |
| **Bonding** | Lien sur la durée ; devient permanent. |
| **Mercenaires** | Recrutement ; suit le joueur ; équipement ; évolution ; résurrection. |
| **Homunculus** | Création à partir d'ingrédients ; IA ; compétences ; évolution ; nourriture. |

---

## 18. Logement et monde

| Point | Description |
|-------|-------------|
| **Logement / housing** | Maison de joueur ; placement d'objets ; visite. |
| **Maisons dans le monde** | Placées dans le monde persistant, pas en instance. |
| **Decay des maisons** | Abandon = dégradation puis suppression après délai. |
| **Placement d'objets** | Décor ; coffres ; objets posés. |
| **Cycle jour / nuit** | Heure de jeu ; luminosité ; événements liés. |
| **Météo** | Pluie, neige, brouillard ; effets visuels/audio. |
| **Zones PvP** | Conflit ouvert ; récompenses PvP. |
| **Zones de faction** | Territoire ; contrôle ; bonus. |
| **Zones sûres** | Pas de PvP ; villes ; hubs. |
| **Kafra / services centralisés** | Sauvegarde ; téléport ; stockage ; courrier ; location d'objets. |

---

## 19. Quêtes et missions

| Point | Description |
|-------|-------------|
| **Quêtes** | Journal ; suivi ; objectifs ; récompenses ; chaînes. |
| **Objectifs dynamiques** | Changements d'objectif selon l'état du monde. |
| **Branchements** | Succès / échec selon les actions ; chemins alternatifs. |
| **Pression temporelle** | Timer ; objectifs à réaliser à temps. |
| **Sauvetage d'alliés** | Alliés en danger ; mission de secours. |
| **Défaite d'alliés** | Mort d'alliés ; impact sur la mission. |
| **Quêtes de changement de rang** | Épreuves ; objectifs ; récompense = nouveau rang. |

---

## 20. Interface

| Point | Description |
|-------|-------------|
| **GUI** | Menus ; HUD ; boutons ; inventaire visuel (slots, pas de drag and drop) ; tooltips. |
| **Carte du monde** | Vue globale ; waypoints ; découverte. |
| **Suivi de quêtes** | Objectifs actuels ; distance ; indicateurs. |
| **Unit frames** | Barres vie/mana party/raid ; cibles ; buffs. |
| **Barre de cast** | Progression du sort ; annulation. |
| **Indicateurs de cooldown** | Temps restant sur les compétences. |
| **Notifications** | Alertes ; loot ; invitations ; événements. |

---

## 21. IA et bots

| Point | Description |
|-------|-------------|
| **IA bot** | Comportements ; états (Idle, Chase, Attack, Flee) ; machine à états. |
| **Pathfinding** | Recherche de chemin ; obstacles. |
| **Navmesh** | Graphe de navigation. |

---

## 22. Réseau et serveurs

| Point | Description |
|-------|-------------|
| **Réseau / MWS** | Connexion multijoueur ; synchronisation ; Lobbys. |
| **Sélection de serveur** | Liste de mondes ; population ; latence. |
| **Sharding** | Fragmentation du monde ; répartition des joueurs. |
| **Cross-server / cross-realm** | Groupes entre serveurs ; files d'attente. |
| **Compensation de latence** | Prédiction ; réconciliation ; lag compensation. |
| **Anti-cheat** | Détection ; validation côté serveur. |
| **Tick serveur** | Fréquence de simulation ; synchronisation. |
| **Trading** | Échange direct ; fenêtre de trade. |
| **PvP** | Duel ; zones hostiles. |
| **Co-op partagé** | Plusieurs joueurs sur le même champ de bataille. |
| **Écran partagé** | Split screen ou vue partagée. |

---

## 23. Système (indispensables)

| Point | Description |
|-------|-------------|
| **Boucle de jeu** | Game loop ; delta time ; frame rate ; pause. |
| **Entrées utilisateur** | Clavier (ZQSD, flèches) ; souris ; manette ; mapping. |
| **Gestion du temps** | Delta time ; temps de jeu ; timers ; cooldowns. |
| **Chargement des assets** | Textures ; sons ; fonts ; chemins ; hot reload (dev). |
| **Sauvegarde / chargement** | Persistance ; KindMother ; sérialisation ; slots. |
| **Audio** | Musique ; SFX ; volume ; canaux ; spatial (optionnel). |
| **Musique adaptative** | Intensité selon le contexte ; transitions. |
| **États du jeu** | Menu ; jeu ; pause ; game over ; transitions. |
| **Système d'événements** | Événements internes ; signaux ; délégués. |
| **Règles du jeu** | Système de règles ; conditions ; validations ; mods. |
| **Debug et outils dev** | Overlays ; logs ; inspector ; raccourcis debug. |
| **Configuration** | Options ; paramètres ; préférences ; persistance. |
| **Localisation (i18n)** | Textes traduisibles ; clés ; langues. |
| **Optimisation** | Culling ; LOD ; pooling ; mise en cache. |

---

## 24. Meta et modération

| Point | Description |
|-------|-------------|
| **Classements / leaderboards** | PvP ; PvE ; économie ; saison. |
| **Outils de modération** | Ban ; mute ; logs ; outils GM. |
| **Système de signalement** | Report joueur ; catégories ; suivi. |
| **Actualités in-game** | Patch notes ; annonces ; événements. |

---

## Synthèse par priorité

### MVP (phase 1)
- Affichage, résolution, coordonnées
- Sprites, animations, caméra
- Hitbox, collision
- Déplacement 8 directions, accélération, vitesse max
- Entités, spawn, unicité
- Données joueur, inventaire (slots + poids)
- Boucle de jeu, entrées, temps
- Chargement assets, sauvegarde basique

### Phase 2
- Combat (action, auto-attaque, projectiles)
- Loot (ramassage auto dans zone pick-up), gestion mort
- Pathfinding, Navmesh
- IA bot
- GUI avancée

### Phase 3
- Chunks, monde ouvert
- Commerce, échange joueurs
- Compétences, progression
- Audio, particules
- Réseau / MWS

### Phase 4+
- Social, guildes, instances, quêtes, crafting, aggro, buffs
- Progression alternative (skills par usage)
- Karma, alignement, criminalité
- Logement, récolte, artisanat

### Phase 5+
- Siège territorial, châteaux, guild wars
- Magie, runes, livres de sorts
- Bateaux, navigation
- Cartes au trésor, champion spawns

### Phase 6+
- Grande densité d'ennemis, champ de bataille dynamique
- Zones de contrôle, objectifs dynamiques
- Montures au combat, mercenaires

### Phase 7+
- Résistances, affixes, objets Set/uniques
- Tables de loot avancées
- Jobs, changement de classe

### Phase 8+
- Effets de statut avancés
- Vending, charrette
- Homunculus, familiers évolués

---

## Références

| Document | Rôle |
|----------|------|
| [MGE - Référence commune](./MGE%20-%20Reference%20Commune.md) | Types, coordonnées, glossaire et conventions partagés |
| [MGE - Hitbox et collisions - Référence](./MGE%20-%20Hitbox%20et%20Collisions%20-%20Reference.md) | Hitbox, collision, broad/narrow phase, MTV, formules |
| [MGE - Paramètres déplacement entité](./MGE%20-%20Parametres%20Deplacement%20Entite.md) | Tous les paramètres pour qu'une entité puisse se déplacer |
| [Index déplacement et orientation](./deplacement-orientation/_index.md) | Index documentaire déplacement/orientation (pratique pour IA) |
| [MGE - Pathfinding Collisions - Guide Entités Groupes](./MGE%20-%20Pathfinding%20Collisions%20-%20Guide%20Entites%20Groupes.md) | Pathfinding, coût déplacement, hitbox, collisions — spectre entités à groupes (RTS, musou) |
| [Plan démo pathfinding labyrinthe](../implementation/MGE%20-%20Plan%20Demo%20Pathfinding%20Labyrinthe.md) | Implémentation démo A* dans labyrinthe |
| [Miyukini - Moteur Jeux et Central Launcher](./Miyukini%20-%20Moteur%20Jeux%20et%20Central%20Launcher.md) | Architecture globale MGE |
| [Allumina - Document Fondateur](../services/Allumina/Allumina%20-%20Document%20Fondateur.md) | Cas d'usage : Action RPG |
| [Index des points de développement](points/_index.md) | Un fichier par point pour développer les spécifications |

---

**Document** : MGE — Miyukini Game Engine — Référence technique  
**Version** : 2.0  
**Date** : 2026-02-18
