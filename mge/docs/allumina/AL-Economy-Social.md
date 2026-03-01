# AL-Economy-Social — Allumina : Économie, Guildes Civiles & Systèmes Sociaux

<!-- @id: AL-Economy-Social @do: reference @role: game-designer @layer: 3 @human: miyuk -->

## Contexte

Allumina est un MMO-ARPG medieval fantasy développé sur le moteur MGE (Rust, ECS archetype, data-driven TOML). Ce document définit l'intégralité des systèmes économiques, sociaux, de crafting, de commerce, de housing et de guerre (RvR) du jeu.

## Portée / Scope

Ce document couvre :
- L'architecture économique globale (flux d'or, devises, inflation)
- Les 13 guildes civiles avec leur documentation complète
- Le système de crafting (qualité, recettes, ateliers)
- Le système Caravanier (convois, routes, embuscades)
- Le commerce (marchés, AH, enchères, espionnage)
- Le housing (logements, ateliers, clans)
- La guerre RvR style Dark Age of Camelot
- Les systèmes sociaux (groupes, clans, réputation, chat)
- Les taxes et l'économie des Outlaws
- Les schémas TOML complets

---

# PARTIE 1 — ARCHITECTURE ÉCONOMIQUE GLOBALE

## 1.1 Philosophie économique

L'économie d'Allumina repose sur trois principes fondateurs :

**Interdépendance civil/combattant** : Aucun joueur ne peut prospérer seul. Le combattant a besoin des artisans pour ses armes et potions. L'artisan a besoin des combattants pour sécuriser ses convois et lui rapporter des matériaux rares. Cette tension crée une économie vivante où chaque décision individuelle a un impact collectif.

**La guerre comme moteur économique** : Le RvR (Realm vs Realm) n'est pas seulement un système de combat — c'est le principal vecteur de destruction créatrice de l'économie. Les équipements se brisent, les routes sont coupées, les ressources changent de main. La guerre détruit pour que l'artisanat reconstruise.

**Le risque comme valeur** : Un lingot d'acier convoyé depuis une zone de guerre vaut dix fois plus qu'un lingot extrait à proximité de la capitale. Le danger est une composante de la valeur.

## 1.2 Flux économique — Comment l'or entre dans le jeu

### Sources d'injection d'or (gold faucets)

**Récompenses de quêtes** : Les quêtes de faction, de profession et de guilde civile génèrent de l'or. Les quêtes de haut niveau en zone de guerre génèrent davantage.

**Vente aux marchands NPC** : Chaque capitale possède des marchands NPC acheteurs de ressources brutes à prix fixes (prix plancher). Ces prix sont volontairement bas — ils existent pour éviter que les ressources ne disparaissent totalement du marché, mais ils n'enrichissent personne.

**Récompenses RvR** : Capturer un fort, tenir une zone, participer à un siège génère des pièces d'or directement. Le montant dépend du rang du joueur dans la bataille et de la durée du contrôle de zone.

**Pillage de caravanes** : L'or transporté dans un convoi peut être partiellement récupéré lors d'une embuscade réussie (60% de l'or transporté, le reste étant "perdu dans la bagarre").

**Vente d'artefacts archéologiques** : Les Archéologues peuvent vendre des artefacts restaurés à des musées de faction ou à des collectionneurs PJ/NPC pour des sommes importantes.

**Tournois et arènes** : Les arènes PvP sanctionnées génèrent des récompenses d'or pour les vainqueurs, financées par les paris des spectateurs.

**Loyers NPC** : Les maisons de housing non entretenues par leur propriétaire peuvent être "rachetées" par le système municipal NPC, générant un flux d'or vers les caisses de faction.

### Sources d'injection de ressources (resource faucets)

**Extraction** : Mineur (minerais), Bûcheron (bois), Fermier (nourriture), Pêcheur (poisson, matériaux aquatiques).

**Monstres et donjons** : Drops de matériaux bruts, composants alchimiques, gemmes brutes, os et cuirs.

**Événements saisonniers** : Ressources spéciales disponibles uniquement lors d'événements (fête des moissons, solstice de guerre, etc.).

**Zones de guerre** : Les territoires contrôlés par une faction génèrent des ressources passives (mines, forêts, champs) pour les membres de cette faction.

## 1.3 Puits à or — Comment l'or sort du jeu

Le contrôle de l'inflation est une priorité absolue. Sans puits à or efficaces, l'économie se détériore en hyperinflation en quelques mois.

**Taxes de marché** : Toute vente sur l'Hôtel des Ventes (AH) prélève 5% au profit de la trésorerie de faction. Les ventes inter-factions via les Guildes Marchandes neutres prélèvent 8%.

**Entretien du housing** : Chaque logement génère un coût mensuel (en jeu). Non payé, le logement perd ses bonus et finit par être saisi. Coût proportionnel à la taille : chambre (50 po/mois), maison (200 po/mois), manoir (800 po/mois), château de clan (5000 po/mois).

**Réparation d'équipement** : La durabilité s'use. La réparation coûte de l'or aux Forgerons (ou au joueur lui-même avec les compétences adéquates). Plus un objet est rare, plus sa réparation est chère.

**Ressurrection via NPC** : Se faire ressusciter par un clerc NPC coûte de l'or (proportionnel au niveau du joueur). Les joueurs peuvent se ressusciter mutuellement gratuitement, mais le service NPC est une commodité chèrement payée.

**Formation et apprentissage** : Acheter de nouvelles recettes de crafting auprès des maîtres de guilde coûte de l'or. Débloquer des rangs de profession nécessite des frais d'inscription.

**Convois et transport** : Louer des gardes NPC pour un convoi, réserver des emplacements de marché, payer pour des routes sécurisées — tout cela sort de l'or du jeu.

**Crafting de haut tier** : Certaines recettes d'excellence (chef-d'œuvre) nécessitent des "réactifs alchimiques stables" achetables uniquement aux marchands NPC de guilde contre de l'or. C'est un puits à or délibéré pour les économies avancées.

**Taxes de faction** : Les factions prélèvent automatiquement une taxe sur tous les gains d'or des membres (taux configurable par les dirigeants, entre 1% et 15%). Cette taxe alimente la trésorerie militaire de faction.

**Frais d'assurance caravanier** : Les Caravaniers peuvent souscrire une assurance auprès des Banquiers pour couvrir leurs pertes en cas de pillage. La prime est un puits à or régulier.

## 1.4 Les trois devises

### Or standard (pièce d'or, abrégé : po)

La devise universelle. Échangeable partout, acceptée par tous les marchands NPC et joueurs. Subdivisée en pièces d'argent (pa, 1 po = 100 pa) et pièces de cuivre (pc, 1 pa = 100 pc).

Utilisée pour : tout le commerce général, les taxes, le housing, les réparations, les services NPC.

### Monnaie de faction (Éclat, couleur par faction)

- **Empire d'Allumina** : Éclat Doré
- **Confédération des Libres** : Éclat d'Argent
- **Culte de l'Ombre** : Éclat de Sang

La monnaie de faction s'obtient par : participation au RvR, quêtes de faction, contrôle de zones, sièges réussis. Elle ne peut pas être échangée entre factions (sauf par les espions commerciaux, avec un malus de 50%).

Utilisée pour : acheter des équipements de faction uniques, upgrader les forts, financer le "Marchand de Guerre", acheter des titres de noblesse de faction.

**Mécanisme anti-inflation des Éclats** : Les Éclats se périment. Chaque Éclat non dépensé perd 10% de sa valeur tous les 30 jours de jeu. Cela pousse les joueurs à dépenser activement en faveur de leur faction.

### Points de Guilde Civile (PGC)

Chaque guilde civile dispose de sa propre monnaie interne. Les PGC s'obtiennent en accomplissant des missions de guilde, en livrant des commandes de faction, en formant d'autres membres.

Les PGC servent à : monter de rang dans la guilde, débloquer des recettes exclusives, voter lors des décisions de guilde, acheter des outils de maître.

Les PGC ne se transfèrent pas entre guildes et ne sont pas échangeables contre de l'or directement (mais les produits achetés avec des PGC peuvent être revendus).

## 1.5 Interdépendance civil/combattant

Le tableau suivant résume les principales chaînes de dépendance :

**Combattant → Civil** : Rapporte des matériaux bruts (minerais, cuirs, bois rares) extraits en zones dangereuses. Fournit de la sécurité (escorte de convois). Génère de la demande en équipement, potions, munitions.

**Civil → Combattant** : Fournit équipements (armes, armures, bijoux enchantés). Fournit consommables (potions, explosifs, nourriture buff). Construit et répare les fortifications de siège. Fournit des cartes de routes (Cartographe). Gère la trésorerie de clan (Banquier).

**Boucle de guerre** : Un siège consomme massivement des ressources (munitions de siège, potions de soin, réparations d'armes). Ce pic de demande fait monter les prix, enrichit les artisans, qui ont alors davantage à perdre s'ils se font piller, les incitant à financer des escortes militaires plus robustes. Le cycle entretient la guerre.

## 1.6 Économie de guerre — Impact du RvR sur l'économie

**Routes commerciales coupées** : Lorsqu'une zone de guerre est active, les routes qui la traversent deviennent dangereuses. Les Caravaniers doivent faire des détours (coût supplémentaire) ou payer des escortes renforcées. Les prix des marchandises transportées augmentent mécaniquement dans les zones isolées.

**Pic de demande en matériel militaire** : Les sièges consomment : huile bouillante (Alchimiste), boulets de catapulte (Ingénieur de siège), échelles et béliers (Charpentier/Mécanicien), flèches et carreaux (Forgeron), potions de soin massives (Pharmacien). Un siège majeur peut assécher le marché d'une capitale en quelques heures.

**Ressources de zone** : Les zones contrôlées génèrent des ressources spécifiques qui n'existent pas ailleurs (minerai de mithral en zone de guerre nord, bois de chêne noir en forêt disputée, etc.). La faction qui contrôle ces zones a un avantage économique direct.

**Pillage de trésoreries** : Un fort capturé peut être partiellement pillé (30% de la trésorerie de faction stockée dans ce fort). Cela crée un transfert direct de richesse entre factions.

**Inflation de guerre** : Pendant les périodes de conflit intense, un modificateur d'inflation de guerre s'applique sur tous les prix NPC (+15% à +40% selon l'intensité). Cela représente la disruption des chaînes d'approvisionnement.

---

# PARTIE 2 — LES 13 GUILDES CIVILES

## 2.0 Principes communs à toutes les guildes

### Rangs internes (1 à 6)

| Rang | Titre générique | Conditions |
|------|----------------|------------|
| 1 | Apprenti | Rejoindre la guilde, payer frais d'inscription (50 po) |
| 2 | Compagnon | 500 PGC, maîtrise de 10 recettes de rang 1 |
| 3 | Artisan | 2000 PGC, maîtrise de 20 recettes, réaliser une commande de faction |
| 4 | Maître | 5000 PGC, maîtrise de 40 recettes, chef-d'œuvre soumis au jury de guilde |
| 5 | Grand Maître | 15000 PGC, contribution significative à l'économie de faction, vote des pairs |
| 6 | Archonte de Guilde | Élu par les Grand Maîtres, un seul par guilde, mandat de 30 jours de jeu |

### Accès inter-factions

Toutes les guildes civiles sont neutres. Un Forgeron de l'Empire et un Forgeron de la Confédération peuvent en théorie se côtoyer dans la même guilde. En pratique, les bâtiments de guilde se trouvent dans les capitales respectives, mais il existe des loges de guilde neutres dans les villes marchandes libres.

### Reconversion civil ↔ combattant

La reconversion est possible mais longue (30 jours de jeu de transition) et coûteuse (1000 po + abandon temporaire de tous les bonus de rang civil). Avantage : les traces du passé persistent. Un ancien Forgeron devenu combattant sait identifier la qualité des armes ennemies (compétence passive : Connaissance des armures). Un ancien combattant devenu Forgeron sait forger selon les besoins réels du champ de bataille (bonus de 10% en recettes militaires).

---

## 2.1 Guilde des Forgerons

### Description

La Guilde des Forgerons est la plus influente des guildes civiles en temps de guerre. Elle fournit l'armement aux trois factions et, en situation de conflit intense, peut littéralement décider de l'issue d'un siège par sa capacité de production.

### Organisation

- **Siège central** : La Grande Forge, bâtiment monumental dans chaque capitale
- **Loges de campagne** : Forges mobiles pouvant être déployées lors des sièges (construites par les Ingénieurs de siège)
- **Conseil de Maîtres** : 5 Maîtres élus représentent la guilde auprès des factions

### Spécialisations (rang 4+)

**Armurier** : Spécialiste des armures lourdes et boucliers. Accès aux recettes d'armures de plates complètes, armures enchantables, boucliers de siège.

**Ingénieur de siège** : Spécialiste des machines de guerre. Accès aux recettes de catapultes, balistes, béliers, tours d'assaut mobiles, huile bouillante (collaboration avec Alchimiste).

**Forgeur de runes** : Spécialiste de la forge enchantée. Accès aux recettes d'armes runiques, d'inserts de gemmes, de métal ensorcelé. Collaboration obligatoire avec le Bijoutier/Enchanteur.

### Compétences exclusives par rang

**Rang 1** : Fonte basique, Polissage, Aiguisage manuel
**Rang 2** : Trempe contrôlée (+5% durabilité des objets fabriqués), Forge d'alliages simples (bronze, laiton)
**Rang 3** : Forge de précision (+1 slot d'amélioration sur les armes forgées), Identification des métaux (passive : voir la qualité des équipements ennemis)
**Rang 4** : Forge à chaud avancée (recettes de mithral et d'acier draconique), Réparation de terrain (réparer des équipements directement sur le champ de bataille, lent)
**Rang 5** : Maîtrise de l'alliage (créer des alliages uniques non listés dans les recettes standards), Forge en équipe (diriger 3 Forgerons de rang inférieur pour un crafting collaboratif)
**Rang 6 (Archonte)** : Décréter des "Commandes de Guerre" qui offrent des bonus PGC aux forgerons participant à la production militaire

### Recettes représentatives

**Armes (rang 1-2)** :
- Épée courte en fer (rang 1) : 3 lingots fer, 1 poignée cuir, 1 rivet cuivre
- Hache de guerre en acier (rang 2) : 5 lingots acier, 2 rivets acier, 1 bois dur
- Lance de cavalerie (rang 2) : 4 lingots acier, 3 bois de lance, 1 cuir renforcé
- Dague d'assassin (rang 2) : 2 lingots acier, 1 os de wyverne (poids allégé)

**Armures (rang 2-4)** :
- Cotte de mailles partielle (rang 2) : 8 anneaux acier, 2 plaques épaules, cuir doublure
- Plastron de plates (rang 3) : 6 plaques acier épaisses, 3 boulons bronze, 2 sangles cuir
- Armure complète de chevalier (rang 4) : 12 plaques acier haute qualité, 5 rivets mithral, 4 doublures cuir épais, 1 heaume à visière
- Armure de siège (rang 4, Ingénieur) : 15 plaques acier renforcé, 4 plaques mithral, ignifuge alchimique x2

**Machines de siège (rang 4-5, spécialisation Ingénieur)** :
- Catapulte de campagne (rang 4) : 20 bois de chêne, 8 plaques acier, 4 cordes de chanvre renforcé, 2 mécanismes de tension bronze
- Baliste légère (rang 4) : 15 bois de chêne, 6 plaques acier, 3 cordes, 1 mécanisme de visée
- Bélier de siège (rang 4) : 25 bois dur, 10 plaques acier, 8 boulons acier, capuchon de métal
- Tour d'assaut mobile (rang 5) : 40 bois de chêne, 20 plaques acier, 10 boulons, 4 roues renforcées, 2 mécanismes de levage — crafting collaboratif (3 Ingénieurs minimum)

**Armes runiques (rang 4-5, spécialisation Forgeur de runes)** :
- Épée runique de feu (rang 4) : 5 lingots acier, 1 gemme de rubis taillée, 1 rune de feu (fournie par Brodeur de runes ou Enchanteur), parchemin de liaison
- Bouclier runique de glace (rang 5) : 8 plaques acier, 1 saphir taillé, 2 runes de glace, 1 métal ensorcelé (produit par Forgeur avec l'aide d'un Alchimiste)

### Interactions avec d'autres guildes

- **Mineur** : Fournit les minerais bruts (lingots après traitement). Dépendance critique.
- **Alchimiste** : Fournit les produits de trempe, huiles ignifuges, réactifs de métal ensorcelé.
- **Bijoutier/Enchanteur** : Fournit les gemmes taillées pour les inserts runiques.
- **Charpentier** : Fournit le bois pour les machines de siège et les manches d'armes.
- **Tailleur** : Fournit les doublures cuir pour les armures.

### Rôle dans l'économie de guerre

Les Forgerons sont les principaux fournisseurs de l'effort de guerre. En période de siège, la demande explose. Les Ingénieurs de siège peuvent déployer des forges de campagne à proximité des zones de conflit, réduisant les délais de ravitaillement. Un clan qui contrôle plusieurs Ingénieurs de siège de haut rang a un avantage tactique décisif.

### Rôle dans le housing

Les Forgerons peuvent installer une forge dans leur logement (logement de taille maison minimum). Une forge personnelle offre : réduction des coûts de crafting de 10%, accès à des recettes de "forge discrète" (items non tracés par les taxes de guilde — zone grise légale), et stockage de lingots sécurisé.

### Quêtes de profession représentatives

**Quête de rang 2 — "La Commande de l'Armée"** : Livrer 50 épées courtes en acier à l'intendant militaire de la faction en 48h de jeu. Récompense : 500 PGC, recette "Épée de troupe renforcée", 300 po.

**Quête de rang 4 — "Le Chef-d'œuvre du Maître"** : Forger une arme ou armure de qualité Chef-d'œuvre (tirage qualité ≥ 95/100) et la soumettre au jury de guilde. Récompense : Rang 4, titre "Maître Forgeron", accès aux recettes de rang 4.

**Quête de rang 5 — "L'Ingénieur de la Victoire"** (Ingénieur de siège) : Construire et déployer une catapulte de campagne lors d'un siège réel, et qu'elle détruise au moins 3 sections de muraille. Récompense : Rang 5, recette "Tour d'assaut renforcée", 2000 PGC.

### TOML schema

```toml
[profession.forgeron]
id = "forgeron"
guild = "guilde_des_forgerons"
faction_access = "all"
max_rank = 6
requires_tool = "marteau_de_forge"
specializations = ["armurier", "ingenieur_siege", "forgeur_runes"]
pgc_per_level = [0, 500, 2000, 5000, 15000, 50000]
inscription_cost_gold = 50

[profession.forgeron.skills]
rank_1 = ["fonte_basique", "polissage", "aiguisage_manuel"]
rank_2 = ["trempe_controlee", "forge_alliages_simples"]
rank_3 = ["forge_precision", "identification_metaux"]
rank_4 = ["forge_chaud_avancee", "reparation_terrain"]
rank_5 = ["maitrise_alliage", "forge_equipe"]
rank_6 = ["commande_de_guerre"]

[profession.forgeron.specialization.ingenieur_siege]
unlock_rank = 4
bonus_recipes = ["catapulte_campagne", "baliste_legere", "belier_siege", "tour_assaut_mobile"]
passive = "genie_militaire"  # +15% solidité des machines construites

[recipe.epee_guerre]
id = "epee_guerre_standard"
profession = "forgeron"
rank_required = 2
materials = [
  { item = "lingot_acier", quantity = 5 },
  { item = "poignee_cuir", quantity = 1 },
  { item = "rivet_acier", quantity = 2 },
]
output = { item = "epee_guerre", quantity = 1, quality_roll = true }
craft_time_seconds = 120
workshop_bonus = 0.15
```

---

## 2.2 Guilde des Tailleurs

### Description

La Guilde des Tailleurs travaille cuirs, tissus et matériaux souples. En apparence modeste, elle produit les armures légères qui équipent les éclaireurs et assassins, les robes des mages, et les uniformes militaires qui définissent l'identité visuelle des factions.

### Spécialisations

**Couturier** : Spécialiste des vêtements et armures légères en tissu. Produit robes, tuniques, manteaux, armures de cuir. Fort impact sur la personnalisation cosmétique.

**Brodeur de runes** : Spécialiste de l'intégration de runes dans les textiles. Produit robes enchantées, bannières de clan runiques, étendards de guerre qui conferent des auras de buff.

### Compétences exclusives par rang

**Rang 1** : Couture basique, Tannage du cuir, Coloration simple
**Rang 2** : Coupe de précision (+5% résistance des armures légères fabriquées), Teinture avancée (personnalisation de couleur complète)
**Rang 3** : Broderie fine (ajouter des motifs décoratifs avec bonus stats mineurs), Armure de cuir composite (mélanger cuir et métal)
**Rang 4** : Tissage de soie draconique (matériau ultra-léger et résistant), Bannière de guilde/clan (objets spéciaux pour le housing)
**Rang 5** : Manteau de dissimulation (robe qui rend difficile la détection à distance), Armure de sceau (set complet avec bonus d'ensemble pour une faction)
**Rang 6** : Décréter les couleurs officielles de saison (influence cosmétique sur toute la faction)

### Recettes représentatives

- Robe de mage en lin renforcé (rang 1) : 4 lin, 2 fil de soie, 1 broderie simple
- Armure de cuir d'éclaireur (rang 2) : 6 cuir tanné, 2 bandes acier, 1 rivets cuivre x8
- Manteau de voyage du Caravanier (rang 3) : 5 cuir souple, 3 tissu épais, 2 poches cachées, 1 capuche
- Bannière de clan runique (rang 5, Brodeur) : 8 soie d'araignée, 3 fils de mithral, 2 runes de moral, 1 mât de bannière (Charpentier)
- Robe de Grand Mage (rang 5) : 6 soie draconique, 4 fils de mithral, 2 gemmes de clarté (Bijoutier), 1 broderie runique

### Interactions

- **Fermier/Botaniste** : Fournit le lin, le chanvre, le coton
- **Mineur** : Fournit les fils de métal pour les broderies
- **Alchimiste** : Fournit les teintures et fixateurs
- **Bijoutier** : Fournit les gemmes pour les broderies runiques

### Rôle dans l'économie de guerre

Les Brodeurs de runes produisent les bannières de guerre qui confèrent des auras de moral (+5% dégâts, +10% résistance) aux troupes à proximité. En siège, une bannière de clan runique de haute qualité peut changer l'issue d'un assaut. Les Couturiers produisent les uniformes militaires qui permettent l'identification ami/ennemi en RvR.

### TOML schema

```toml
[profession.tailleur]
id = "tailleur"
guild = "guilde_des_tailleurs"
faction_access = "all"
max_rank = 6
requires_tool = "aiguille_et_fil_maitrise"
specializations = ["couturier", "brodeur_runes"]

[recipe.banniere_clan_runique]
id = "banniere_clan_runique"
profession = "tailleur"
specialization = "brodeur_runes"
rank_required = 5
materials = [
  { item = "soie_araignee", quantity = 8 },
  { item = "fil_mithral", quantity = 3 },
  { item = "rune_moral", quantity = 2 },
  { item = "mat_banniere", quantity = 1 },
]
output = { item = "banniere_clan_runique", quantity = 1, quality_roll = true }
craft_time_seconds = 3600
aura_radius_meters = 30.0
aura_effects = ["damage_bonus_0.05", "resistance_bonus_0.10"]
```

---

## 2.3 Guilde des Alchimistes

### Description

La Guilde des Alchimistes est la guilde la plus intellectuellement exigeante. Ses membres jonglent avec des réactions chimiques, des composants biologiques et des principes magiques pour produire potions, explosifs et alcools de guerre. Ils sont indispensables en combat comme en siège.

### Spécialisations

**Pharmacien** : Potions de soin, antidotes, élixirs de buff. La ligne de vie des factions.

**Explosiviste** : Grenades alchimiques, barils explosifs, charges perforantes pour les sièges. Partenaire clé des Ingénieurs de siège.

**Distillateur** : Alcools de guerre (buff de combat), poisons, huiles d'armes, venins. Plus ambigu moralement — les Distillateurs ont mauvaise réputation mais sont très demandés.

### Compétences exclusives par rang

**Rang 1** : Décoction basique, Identification des composants (voir les composants alchimiques dans le monde)
**Rang 2** : Stabilisation des réactifs (réduction du taux d'échec de 15%), Brewing (potions en série)
**Rang 3** : Distillation avancée, Analyse chimique (identifier les effets d'un objet inconnu)
**Rang 4** : Synthèse explosive (accès aux explosifs de siège), Grand Élixir (potions de très longue durée)
**Rang 5** : Formule secrète (créer des recettes uniques par expérimentation), Antidote universel
**Rang 6** : Proclamer une "Semaine de la Grande Pharmacie" (réduction des coûts de potions pour toute la faction)

### Recettes représentatives

**Pharmacien** :
- Potion de soin mineur (rang 1) : 2 herbes de guérison, 1 eau pure, 1 flacon
- Potion de soin majeur (rang 3) : 4 herbes de guérison supérieures, 2 essence de vie, 1 flacon de cristal
- Antidote commun (rang 2) : 3 racines d'antidotaire, 1 charbon actif, 1 flacon
- Élixir de force (rang 3) : 2 sang de troll, 1 extrait de mandragore, 2 minéraux de fer, 1 flacon

**Explosiviste** :
- Grenade de feu (rang 3) : 2 poudre de soufre, 1 huile draconique, 1 détonateur rune, 1 boîtier métal
- Baril explosif (rang 4) : 10 poudre noire, 4 détonateurs, 2 renforts métal, huile stabilisante
- Charge perforante (rang 5) : 5 poudre de mithral, 3 détonateurs de précision, métal concentré — spécialement efficace contre les portes de forts

**Distillateur** :
- Huile de feu pour arme (rang 2) : 2 résine draconique, 1 huile de lin, 1 stabilisant
- Whisky de combat (rang 3) : 3 grains de seigle fermenté, 1 extrait de racine courageuse, épices — buff : +10% dégâts, -5% défense pendant 10 minutes
- Poison de contact (rang 4) : 2 venin d'araignée noire, 1 extrait de morelle, stabilisant alchimique, applicateur

### Rôle dans l'économie de guerre

En temps de siège, les Explosivistes sont aussi précieux que les Ingénieurs. Leur stock d'explosifs peut définir si un assaut réussit ou échoue. La demande en potions de soin pendant un RvR majeur peut être telle que les prix triplent en quelques heures. Les Pharmaciens qui anticipent les sièges en stockant des potions à l'avance réalisent d'énormes profits.

### TOML schema

```toml
[profession.alchimiste]
id = "alchimiste"
guild = "guilde_des_alchimistes"
faction_access = "all"
max_rank = 6
requires_tool = "alambic_de_maitrise"
specializations = ["pharmacien", "explosiviste", "distillateur"]

[recipe.grenade_feu]
id = "grenade_feu"
profession = "alchimiste"
specialization = "explosiviste"
rank_required = 3
materials = [
  { item = "poudre_soufre", quantity = 2 },
  { item = "huile_draconique", quantity = 1 },
  { item = "detonateur_rune", quantity = 1 },
  { item = "boitier_metal", quantity = 1 },
]
output = { item = "grenade_feu", quantity = 3, quality_roll = true }
craft_time_seconds = 60
explosion_radius = 4.0
damage_type = "fire"
```

---

## 2.4 Guilde des Bijoutiers

### Description

La plus précieuse des guildes en termes de valeur d'objet moyen. Les Bijoutiers travaillent les gemmes et métaux précieux pour créer bijoux, enchantements et inserts runiques. Leur collaboration avec les Forgerons et Tailleurs est structurelle.

### Spécialisations

**Tailleur de gemmes** : Transforme les gemmes brutes (extraites par les Mineurs) en gemmes taillées utilisables dans les inserts et enchantements.

**Enchanteur** : Combine gemmes taillées, runes et métaux précieux pour créer des objets d'enchantement permanents ou temporaires.

### Compétences exclusives par rang

**Rang 1** : Taille basique des gemmes, Sertissage simple
**Rang 2** : Taille de précision (+10% efficacité des gemmes taillées), Identification des gemmes brutes
**Rang 3** : Sertissage complexe (2 gemmes sur un objet), Enchantement mineur (ajouter un effet à un équipement existant)
**Rang 4** : Taille parfaite (gemmes Excellent et Chef-d'œuvre), Double enchantement
**Rang 5** : Fusion de gemmes (combiner 2 gemmes de même type en une supérieure), Malédiction inversée (retirer une malédiction d'un objet maudit)
**Rang 6** : Décréter les "Pierres de Faction" (gemmes uniques avec les couleurs et effets de faction)

### Recettes représentatives

- Anneau de vigueur en argent (rang 2) : 2 lingots argent, 1 rubis taillé, 1 serti simple
- Collier de mana en or (rang 3) : 2 lingots or, 1 saphir taillé, 1 maille or
- Diadème de commandement (rang 4) : 3 lingots mithral, 2 diamants taillés, 1 rune de commandement, 1 monture de précision
- Insert de gemme draconique (rang 5) : 1 gemme draconique taillée (fusion de 3 gemmes), 1 liant mithral, 1 rune de liaison

### TOML schema

```toml
[profession.bijoutier]
id = "bijoutier"
guild = "guilde_des_bijoutiers"
faction_access = "all"
max_rank = 6
requires_tool = "loupe_et_taille_diamant"
specializations = ["tailleur_gemmes", "enchanteur"]

[recipe.anneau_vigueur]
id = "anneau_vigueur_argent"
profession = "bijoutier"
rank_required = 2
materials = [
  { item = "lingot_argent", quantity = 2 },
  { item = "rubis_taille", quantity = 1 },
  { item = "serti_simple", quantity = 1 },
]
output = { item = "anneau_vigueur", quantity = 1, quality_roll = true }
craft_time_seconds = 300
stat_bonus = { vitality = 15 }
```

---

## 2.5 Guilde des Charpentiers

### Description

Les Charpentiers sont les bâtisseurs du monde. Ils construisent les logements, fabriquent les meubles, réparent les fortifications et construisent les navires. En temps de guerre, leurs Mécaniciens sont les architectes des défenses de fort.

### Spécialisations

**Constructeur** : Spécialiste du housing. Construit, agrandit et décore les logements. Crée les meubles.

**Mécanicien** : Spécialiste des mécanismes. Crée les pièges de siège (défensifs), les mécanismes de portes secrètes, les ascenseurs de fort.

**Luthier** : Spécialiste des instruments de musique. Les instruments de haut rang produisent des effets de barde (buffs musicaux actifs).

### Compétences exclusives par rang

**Rang 1** : Menuiserie basique, Assemblage de meubles simples
**Rang 2** : Construction de logements (pose des éléments de base), Fabrication de mécanismes simples
**Rang 3** : Architecture (plans de logements complexes), Fabrication de pièges
**Rang 4** : Ingénierie de fort (renforcement des défenses d'un fort de faction), Navires de commerce
**Rang 5** : Architecture de château (extension des châteaux de clan), Machines défensives
**Rang 6** : Grand Projet (lancer un chantier de construction public financé par la guilde)

### Recettes représentatives

**Constructeur** :
- Table de craft en bois de pin (rang 1) : 10 planches pin, 4 chevilles bois, 2 vis
- Forge personnelle en pierre et métal (rang 3) : 20 pierres taillées, 10 briques réfractaires, 8 plaques acier, 4 soufflets cuir — installable en housing maison+
- Atelier d'alchimie (rang 3) : 15 planches de chêne, 8 supports métal, 4 conduites de verre, 2 systèmes de ventilation
- Salle du trésor (rang 5) : 30 pierres renforcées, 15 plaques mithral, 2 serrures de haute sécurité, 1 mécanisme anti-effraction

**Mécanicien** :
- Piège à piques (rang 3) : 8 piques acier, 4 ressorts acier, 2 mécanismes déclencheurs, 1 plaque de dissimulation
- Herse de fort (rang 4) : 20 barres acier épaisses, 8 chaînes renforcées, 1 treuil mécanisé, 2 poulies — installable uniquement sur des forts de faction

**Luthier** :
- Luth de voyage (rang 1) : 6 planches de pin, 2 cordes de soie, 1 chevalet bois
- Harpe runique (rang 5) : 12 planches de bois ancien, 4 cordes de mithral, 2 runes de résonance, 1 cadre de cristal — produit des buffs musicaux actifs dans un rayon de 20m

### TOML schema

```toml
[profession.charpentier]
id = "charpentier"
guild = "guilde_des_charpentiers"
faction_access = "all"
max_rank = 6
requires_tool = "etabli_complet"
specializations = ["constructeur", "mecanicien", "luthier"]

[recipe.forge_personnelle]
id = "forge_personnelle"
profession = "charpentier"
specialization = "constructeur"
rank_required = 3
materials = [
  { item = "pierre_taillee", quantity = 20 },
  { item = "brique_refractaire", quantity = 10 },
  { item = "plaque_acier", quantity = 8 },
  { item = "soufflet_cuir", quantity = 4 },
]
output = { item = "forge_personnelle", quantity = 1, quality_roll = false }
craft_time_seconds = 7200
housing_only = true
min_housing_size = "maison"
installed_bonus = { craft_cost_reduction = 0.10 }
```

---

## 2.6 Guilde des Fermiers

### Description

La Guilde des Fermiers est la base alimentaire de la civilisation d'Allumina. Sans nourriture, les buffs de combat s'effacent, les potions de soin sont moins efficaces, et le moral des troupes chute. Les Fermiers sont discrets mais leur absence se fait cruellement sentir.

### Spécialisations

**Éleveur** : Spécialiste des animaux. Élève des chevaux de guerre, des animaux à laine, des animaux à cuir.

**Botaniste** : Spécialiste des plantes médicinales et des composants alchimiques végétaux. Fournisseur principal des Alchimistes.

**Apiculteur** : Spécialiste du miel et des produits de ruche. Le miel d'abeilles draconiques est un composant de potions exceptionnelles.

### Compétences exclusives par rang

**Rang 1** : Agriculture basique, Cueillette ciblée (trouver des plantes spécifiques dans la nature)
**Rang 2** : Élevage (débloquer les animaux de ferme), Compostage (améliorer les rendements)
**Rang 3** : Culture sélective (améliorer la qualité des récoltes), Herboristerie avancée
**Rang 4** : Élevage de montures de qualité (chevaux de guerre vendables à prix premium), Plantation en zone de guerre (cultiver dans des zones disputées pour profit maximum avec risque)
**Rang 5** : Serre alchimique (produire des plantes rares hors saison), Élevage draconique (animaux rares à haut rendement)
**Rang 6** : "Grande Moisson" (événement de faction qui double les récoltes pour 24h de jeu)

### Recettes représentatives

- Ration de campagne (rang 1) : 2 pain de seigle, 1 fromage, 1 viande séchée — buff : +5% HP régénération pendant 30 min
- Festin du guerrier (rang 3) : 1 rôti de sanglier, 2 légumes racines, 1 sauce aux herbes, 1 pain — buff : +15% dégâts pendant 1 heure
- Herbes médicinales séchées (rang 2) : 3 herbes fraîches + séchage (24h jeu) → 1 herbes médicinales (composant alchimique)
- Miel draconique (rang 5, Apiculteur) : production passive de ruches d'abeilles draconiques — composant de potions d'excellence

### TOML schema

```toml
[profession.fermier]
id = "fermier"
guild = "guilde_des_fermiers"
faction_access = "all"
max_rank = 6
requires_tool = "outils_agricoles"
specializations = ["eleveur", "botaniste", "apiculteur"]

[recipe.festin_guerrier]
id = "festin_guerrier"
profession = "fermier"
rank_required = 3
materials = [
  { item = "roti_sanglier", quantity = 1 },
  { item = "legumes_racines", quantity = 2 },
  { item = "herbes_sauce", quantity = 1 },
  { item = "pain_seigle", quantity = 1 },
]
output = { item = "festin_guerrier", quantity = 6, quality_roll = true }
craft_time_seconds = 1800
buff = { damage_bonus = 0.15, duration_minutes = 60 }
```

---

## 2.7 Guilde des Mineurs

### Description

Les Mineurs sont à la base de toute chaîne de production physique. Sans minerai, pas d'armes. Leur travail est dangereux (les mines sont souvent en zones disputées) et leur expertise est essentielle.

### Spécialisations

**Prospecteur** : Spécialiste de la détection de filons. Peut repérer des mines cachées, évaluer la richesse d'un gisement, débloquer des sites d'extraction nouveaux.

**Géologue** : Spécialiste de l'analyse des pierres. Peut identifier des matériaux rares (gemmes cachées dans la roche, métaux précieux mélangés), analyser la solidité des structures de pierre pour les sièges.

### Compétences exclusives par rang

**Rang 1** : Extraction basique (fer, cuivre), Identification des minerais communs
**Rang 2** : Extraction efficace (+20% rendement), Repérage de filons secondaires
**Rang 3** : Extraction de profondeur (accès aux veines profondes : argent, plomb), Stabilisation de galerie (réduire le risque d'effondrement)
**Rang 4** : Extraction rare (mithral, gemmes brutes), Cartographie de mine (créer des cartes de mines vendables)
**Rang 5** : Exploitation maximale (extraire des filons quasi-épuisés), Analyse structurelle (évaluer les points faibles d'une fortification de pierre)
**Rang 6** : Décréter l'"Ouverture d'un Nouveau Filon" (explorer une zone minière inédite pour toute la faction)

### Rôle dans l'économie de guerre

Les Géologues rang 5 peuvent analyser les murs d'un fort ennemi et identifier les sections les plus faibles, donnant aux Ingénieurs de siège un bonus de 20% sur les dommages de leurs machines à ces points précis. C'est une collaboration civil/militaire unique et très appréciée.

### TOML schema

```toml
[profession.mineur]
id = "mineur"
guild = "guilde_des_mineurs"
faction_access = "all"
max_rank = 6
requires_tool = "pioche_de_maitrise"
specializations = ["prospecteur", "geologue"]

[resource.filon_mithral]
id = "filon_mithral"
location_type = "zone_guerre"
min_rank_required = 4
extraction_time_seconds = 300
yield_range = [1, 3]
item = "minerai_mithral"
danger_level = 5
respawn_hours = 48
```

---

## 2.8 Guilde des Pêcheurs

### Description

Les Pêcheurs approvisionnent en nourriture maritime et en matériaux biologiques aquatiques (os de poisson pour les alchimistes, écailles pour les Tailleurs). L'Aquaculteur est un sous-profil de niche mais les Harponneurs sont de véritables guerriers économiques.

### Spécialisations

**Aquaculteur** : Gère des bassins d'élevage de poissons en housing. Production passive et régulière. Moins risquée, moins profitable.

**Harponneur** : Spécialiste de la chasse aux créatures marines géantes (baleines alchimiques, kraken juvéniles). Produits très rares à très haute valeur.

### Compétences exclusives par rang

**Rang 1** : Pêche à la ligne (zones côtières), Identification des espèces
**Rang 2** : Pêche au filet (rendement x3 mais temps x5), Préparation des poissons (fumage, séchage)
**Rang 3** : Pêche en eau profonde (nouvelles espèces), Aquaculture basique (bassins en housing)
**Rang 4** : Pêche de nuit (espèces nocturnes rares), Harpon léger (chasse aux créatures marines moyennes)
**Rang 5** : Grand Harpon (chasse aux créatures marines géantes), Navigation hauturière (accès aux zones marines lointaines)
**Rang 6** : "La Grande Pêche" (événement de faction : bonus de rendement marin pour 48h)

### Recettes représentatives

- Poisson fumé (rang 1) : 3 poissons frais + fumoir → 3 poissons fumés — conservation longue durée
- Soupe de kraken (rang 5, Harponneur) : 1 tentacule de kraken, 4 herbes marines, 2 sel marin — buff exceptionnel : +30% mana régénération pendant 2h
- Huile de baleine (rang 4) : 1 graisse de baleine alchimique → 10 huiles de baleine — composant alchimique premium

### TOML schema

```toml
[profession.pecheur]
id = "pecheur"
guild = "guilde_des_pecheurs"
faction_access = "all"
max_rank = 6
requires_tool = "canne_a_peche_maitrise"
specializations = ["aquaculteur", "harponneur"]
```

---

## 2.9 Guilde des Caravaniers

### Description

La Guilde des Caravaniers est la colonne vertebrale du commerce a longue distance. Ses membres sont des logisticiens, des aventuriers commerciaux, des diplomates de route. En temps de guerre, ils sont des cibles et des acteurs : leurs convois ravitaillent les armees ou alimentent l'ennemi.

La guilde est strictement neutre. Un Caravanier de l'Empire peut legalement livrer des marchandises a la Confederation via des routes neutres, sous couvert de commerce inter-factions.

### Specialisations

**Convoyeur** : Specialiste des grands convois. Gere des equipes de porteurs, de chariots, de betes de somme. Volume maximum, escorte obligatoire.

**Courrier** : Specialiste de la livraison rapide. Petits colis, grande vitesse, discretion maximale. Peut traverser des zones dangereuses plus facilement (competence Furtivite de route).

**Negociant en route** : Specialiste du commerce opportuniste. Achete et revend sur la route selon les prix locaux. Ne transporte pas de commandes fixes mais surfe sur les fluctuations de prix.

### Competences exclusives par rang

- **Rang 1** : Connaissance des routes (voir niveaux de danger sur la carte), Chargement basique (+20% poids), Marchandage de base
- **Rang 2** : Reseau d'informateurs lvl1 (alertes embuscades NPC), Deguisement simple, Bete de somme (+50% cargo)
- **Rang 3** : Connaissance avancee (embuscades PJ avec delai 5min), Escorte diplomatique (-20% cout gardes), Cheval de convoi (+15% vitesse)
- **Rang 4** : Reseau informateurs lvl2 (temps quasi-reel, 20 contacts NPC), Deguisement avance (passer pour une autre faction), Mule de guerre (+100% cargo)
- **Rang 5** : Route secrete (chemins alternatifs caches), Contrebande (marchandises illegales sans detection NPC), Grand Convoi (10+ chariots, bonus profit)
- **Rang 6** : Treve commerciale (zone neutre declaree 6h, reconnue par les factions)

### TOML schema

```toml
[profession.caravanier]
id = "caravanier"
guild = "guilde_des_caravaniers"
faction_access = "all"
max_rank = 6
requires_tool = "registre_de_route"
specializations = ["convoyeur", "courrier", "negociant_route"]

[profession.caravanier.skills]
rank_1 = ["connaissance_routes", "chargement_basique", "marchandage_base"]
rank_2 = ["reseau_informateurs_1", "deguisement_simple", "bete_de_somme"]
rank_3 = ["connaissance_routes_avancee", "escorte_diplomatique", "cheval_convoy"]
rank_4 = ["reseau_informateurs_2", "deguisement_avance", "mule_guerre"]
rank_5 = ["route_secrete", "contrebande", "grand_convoy"]
rank_6 = ["treve_commerciale"]

[convoy.route_imperiale]
id = "route_cap_vers_forteresse"
origin = "capitale_empire"
destination = "forteresse_frontiere"
distance_km = 45
danger_level = 4
recommended_escort = 3
base_profit_gold = 800
```

---

## 2.10 Guilde des Marchands

### Description

Les Marchands sont les maitres du commerce sedentaire. La ou les Caravaniers bougent, les Marchands restent et organisent le marche. Leur specialisation en espionnage commercial (rang 5+) en fait des acteurs d'intelligence economique redoutes.

### Specialisations

**Teneur de marche** : Gere un stand ou une boutique en ville. Achete bas, vend haut. Acces a des emplacements de marche premium.

**Courtier** : Intermediaire entre vendeurs et acheteurs. Prend une commission. Acces a l'AH inter-factions.

**Espion commercial** : Collecte des informations sur les prix et ressources ennemies. Vend ces informations a sa faction ou aux plus offrants.

### Competences exclusives par rang

- **Rang 1** : Evaluation de marches (prix de toutes les villes connues), Negociation (-5% achats NPC)
- **Rang 2** : Boutique de joueur (ouvrir une boutique en housing ou en ville), Sens des affaires (alerte item sous-evalue +30%)
- **Rang 3** : Reseau de courtage (AH inter-factions frais reduits), Comptabilite avancee (-2% taxes de marche)
- **Rang 4** : Contact ennemi (informateurs dans 1 ville ennemie, delai 30min), Stockage de masse (500 items coffre de guilde)
- **Rang 5** : Espionnage commercial actif (prix ennemi temps reel, risque contre-espionnage 15%), Manipulation de marche (inflation artificielle, risque detection)
- **Rang 6** : Foire commerciale (taxes a 0% dans une ville pour 12h)

### Role dans l'economie de guerre

Les Espions commerciaux constituent un service de renseignement economique. Savoir qu'une faction ennemie manque de potions avant un siege est une information strategique. Certains clans emploient des Marchands rang 5 exclusivement pour ce role.

### TOML schema

```toml
[profession.marchand]
id = "marchand"
guild = "guilde_des_marchands"
faction_access = "all"
max_rank = 6
requires_tool = "registre_commercial"
specializations = ["teneur_marche", "courtier", "espion_commercial"]

[profession.marchand.espionnage]
rank_required = 5
detection_risk = 0.15
intel_delay_minutes = 0
cooldown_hours = 4
```

---

## 2.11 Guilde des Banquiers

### Description

La Guilde des Banquiers gere la richesse. Absente du champ de bataille, elle est omnipresente dans les coulisses : prets aux clans, gestion des tresoreries, assurances pour Caravaniers. Un Banquier de haut rang peut etre l'acteur le plus puissant d'une faction sans jamais tenir une arme.

### Specialisations

**Gestionnaire de tresorerie de clan** : Gere les finances de clans. Produit des rapports financiers, detecte les fuites.

**Preteur** : Accorde des prets a interets. Le non-remboursement declenche des penalites mecaniques (saisie, malus reputation).

### Competences exclusives par rang

- **Rang 1** : Comptabilite basique (historique 30 jours), Coffre-fort personnel (+50% stockage or)
- **Rang 2** : Gestion tresorerie de clan, Taux preferentiel (emprunts NPC reduits)
- **Rang 3** : Assurance caravanier (contrats d'assurance), Analyse de solvabilite
- **Rang 4** : Pret de clan (prets a des clans entiers avec garanties), Transferts inter-factions
- **Rang 5** : Banque de guilde (gerer une banque pour toute une guilde civile), Manipulation de taux (influence taux de change Eclats)
- **Rang 6** : Grande Banque de Faction (tresorerie centrale, rapports economiques globaux)

### Mecanique de pret

Un pret cree un contrat persistant. En cas de defaut de paiement :
- Saisie automatique d'equipements en coffre-fort (jusqu'a concurrence de la dette)
- Malus de reputation (-50 aupres de toutes les guildes civiles)
- Possibilite d'engager des Recouvreurs (mercenaires NPC ou PJ) pour traquer le debiteur

### TOML schema

```toml
[profession.banquier]
id = "banquier"
guild = "guilde_des_banquiers"
faction_access = "all"
max_rank = 6
requires_tool = "ledger_or"
specializations = ["gestionnaire_tresorerie", "preteur"]

[loan_contract]
id = "pret_001"
lender_id = "player_uuid_banquier"
borrower_id = "player_uuid_emprunteur"
amount_gold = 5000
interest_rate = 0.08
duration_days_game = 14
collateral = { item = "armure_mithral", quantity = 1 }
status = "active"
created_at = "2026-03-01T12:00:00Z"
due_at = "2026-03-15T12:00:00Z"
default_penalty = { reputation_loss = 50, auto_seize = true }
```

---

## 2.12 Guilde des Chercheurs

### Description

La Guilde des Chercheurs est la plus intellectuelle. Ses membres debloquent des technologies pour leur faction entiere, forment les nouveaux artisans, et philosophent sur les lois du monde. Leur impact est indirect mais structurel.

### Specialisations

**Inventeur** : Debloque de nouvelles recettes et technologies pour l'ensemble d'une faction via des projets de recherche collectifs.

**Philosophe** : Redige des Traites qui conferent des bonus passifs a tous les membres de faction pendant 7 jours de jeu.

**Enseignant** : Forme d'autres joueurs plus rapidement, reduisant le temps de montee en rang de leurs professions.

### Mecanique de recherche

1. L'Inventeur propose un projet de recherche (ex: "Acier Draconian Ameliore").
2. Le projet necessite contributions de ressources et PGC de plusieurs membres de faction.
3. Apres 48-168h de jeu, si l'objectif est atteint, la nouvelle recette est debloquee pour toute la faction.
4. Si la faction ennemie fait une incursion sur le laboratoire avant la fin, le projet peut etre sabote.

### TOML schema

```toml
[profession.chercheur]
id = "chercheur"
guild = "guilde_des_chercheurs"
faction_access = "all"
max_rank = 6
requires_tool = "bibliotheque_personnelle"
specializations = ["inventeur", "philosophe", "enseignant"]

[research_project.acier_draconian_ameliore]
id = "acier_draconian_ameliore"
proposer_rank_min = 4
required_resources = [
  { item = "minerai_mithral", quantity = 50 },
  { item = "ecailles_draconiques", quantity = 20 },
  { item = "notes_de_recherche", quantity = 10 },
]
pgc_cost = 5000
duration_hours_game = 72
sabotage_window_hours = 48
unlock_recipe = "lingot_acier_draconian_ameliore"
faction_unlock = true
```

---

## 2.13 Guilde des Archeologues

### Description

La Guilde des Archeologues explore les ruines du monde, deterrant des secrets enfouis. Leurs decouvertes peuvent changer l'equilibre d'une faction : un artefact puissant restaure, une carte d'une ancienne mine, un traite diplomatique revelant une alliance oubliee.

### Specialisations

**Fouilleur** : Active des sites de fouille en open world, generant des objets aleatoires : artefacts, materiaux anciens, tresors.

**Restaurateur d'artefacts** : Transforme les artefacts bruts (fragmentes, corrompus) en artefacts complets et fonctionnels.

**Cartographe** : Cree des cartes de zones (routes commerciales, mines cachees, donjons), vendables a prix eleves.

### Competences exclusives par rang

- **Rang 1** : Detection de sites (sites de fouille sur minimap), Analyse d'artefact (identifier un artefact brut)
- **Rang 2** : Excavation rapide (-25% temps de fouille), Conservation (eviter degradation pendant transport)
- **Rang 3** : Restauration basique (artefacts rang 1-2), Cartographie de zone (creer des cartes)
- **Rang 4** : Site rare (fouille en zones de guerre), Restauration avancee (artefacts rang 3-4)
- **Rang 5** : Artefact legendaire (artefacts rang 5, bonus exceptionnels pour clan/faction), Carte strategique (annotations tactiques pour sieges)
- **Rang 6** : Grande Decouverte (expedition archeologique de faction dans une zone inexploree)

### Role dans l'economie

Les Cartographes sont fournisseurs cles des Caravaniers (routes secretes), des strateges de clan (cartes de siege), et des explorateurs. Une bonne carte peut valoir des milliers de po.

### TOML schema

```toml
[profession.archeologue]
id = "archeologue"
guild = "guilde_des_archeologues"
faction_access = "all"
max_rank = 6
requires_tool = "trousse_archeologie"
specializations = ["fouilleur", "restaurateur_artefacts", "cartographe"]

[artefact.epee_ancienne_corrompue]
id = "epee_ancienne_corrompue_01"
state = "corrompue"
restoration_rank_required = 4
restoration_materials = [
  { item = "solvant_alchimique", quantity = 3 },
  { item = "rune_de_purification", quantity = 1 },
]
restoration_time_hours = 24
restored_item = "epee_des_anciens_rois"
restored_item_rarity = "legendaire"

[map.route_secrete_nord]
id = "carte_route_nord_secrete"
creator_profession = "cartographe"
rank_required = 4
reveals = ["route_secrete_001", "mine_cachee_nordique"]
danger_warnings = ["embuscade_col_nord"]
sellable = true
base_value_gold = 500
```

---

# PARTIE 3 — SYSTEME DE CRAFTING

## 3.1 Philosophie du crafting

Le crafting dans Allumina est base sur trois piliers :

**Qualite** : Chaque production peut resultent en une qualite variable. La qualite n'est pas entierement aleatoire — les competences du joueur, la qualite de ses outils, et l'environnement de production (atelier personnel vs. atelier de guilde vs. forge de campagne) influencent directement le tirage de qualite.

**Rareté des meilleures recettes** : Les recettes les plus puissantes ne s'obtiennent pas en montant de rang — elles se decouvrent par experimentation, par quetes rares, ou par achat tres couteux a d'autres joueurs.

**Interdependance** : Aucun artisan ne peut produire les objets les plus puissants seul. Le chef-d'oeuvre necessite la collaboration de plusieurs guildes.

## 3.2 Interface de crafting

L'interface de crafting comporte :

**Panneau de recettes** : Liste filtrable de toutes les recettes connues du joueur, triees par categorie, niveau requis, disponibilite des materiaux. Un indicateur coloré (vert/orange/rouge) signale si les materiaux sont disponibles dans l'inventaire.

**Panneau de materiaux** : Vue de l'inventaire avec mise en evidence des materiaux requis pour la recette selectionnee. Possibilite de puiser dans le coffre de stockage du housing si le joueur est dans son logement.

**Barre de progression** : Affiche le temps de craft, avec un sous-indicateur de "fenetre de precision" (voir ci-dessous).

**Indicateur de qualite** : Affiche la plage de qualite attendue avec les modificateurs en cours. Ne revele pas le resultat exact avant la fin.

**Fenetre de precision** : Pour les crafts de rang 3+, une fenetre de timing apparait. Appuyer au bon moment dans la barre de progression (zone verte) donne un bonus de qualite (+5 a +15 points). Appuyer dans la zone orange donne un leger bonus (+2). Rater donne malus (-5). Ce systeme recompense l'engagement actif du joueur sans punir trop severement la distraction.

## 3.3 Systeme de qualite

Les niveaux de qualite sont : Normal / Bon / Excellent / Chef-d'oeuvre.

**Calcul du tirage de qualite** :

Le score de qualite de base est calcule comme suit :

```
score = base_score + competence_bonus + outil_bonus + atelier_bonus + timing_bonus + rng_modifier
```

- `base_score` : 30 (artisan debutant) a 70 (Grand Maitre)
- `competence_bonus` : +0 a +20 selon le rang de profession
- `outil_bonus` : 0 (outils de base), +5 (outils de qualite), +10 (outils de maitre), +15 (outils legendaires)
- `atelier_bonus` : 0 (craft en plein air), +5 (forge communale), +10 (atelier personnel de housing), +15 (atelier de guilde)
- `timing_bonus` : -5 a +15 selon la fenetre de precision
- `rng_modifier` : -10 a +10 (composante aleatoire residuelle)

**Seuils de qualite** :

| Score | Qualite | Bonus sur l'objet final |
|-------|---------|------------------------|
| 0-39 | Normal | Aucun |
| 40-59 | Bon | +10% stats principales |
| 60-79 | Excellent | +25% stats principales, +1 slot enchantement |
| 80-100 | Chef-d'oeuvre | +50% stats principales, +2 slots enchantement, apparence unique |

**Chef-d'oeuvre** : Un item Chef-d'oeuvre porte le nom de son artisan ("Epee des Anciens de [NomJoueur]"). Il peut etre signe (signature gravee), ce qui augmente sa valeur de revente. Un artisan connu pour ses Chef-d'oeuvres construit une reputation et peut vendre ses creations a des prix bien superieurs au marche.

## 3.4 Craft mobile vs atelier fixe

**Craft mobile** (en deplacement, forge de campagne, etc.) :
- Disponible pour tous les rangs
- Malus atelier_bonus = 0
- Recettes limitees (pas de recettes de rang 4+)
- Necessite d'avoir les outils dans l'inventaire
- Utile pour les reparations de terrain (Forgerons rang 4) et les potions d'urgence

**Atelier personnel (housing)** :
- Bonus atelier : +10
- Acces a toutes les recettes connues
- Possibilite de stocker des materiaux dans l'atelier (non transferables automatiquement)
- Ateliers specialises installes (forge, alambic, etabli) donnent un bonus supplementaire de +5 pour la specialisation correspondante

**Atelier de guilde** :
- Bonus atelier : +15
- Acces aux recettes exclusives de guilde rang 5-6
- Reservation d'ateliers (temps limite par guilde)
- Les crafts realises ici contribuent aux PGC de la guilde

## 3.5 Recettes cachees — Decouverte par experimentation

Les recettes cachees representent une couche de profondeur pour les artisans dedies.

**Mecanique d'experimentation** : Le joueur peut tenter de combiner des materiaux hors des recettes connues. Il choisit jusqu'a 5 materiaux dans son inventaire et les "soumet" a l'atelier. Resultats possibles :
- Echec complet (materiaux consommes, rien produit) : 60% de chance
- Decouverte partielle (indice sur une recette proche, materiaux partiellement consommes) : 25%
- Decouverte de recette : 12%
- Decouverte d'une recette rare unique (lie au personnage, non transmissible) : 3%

Le taux de decouverte augmente avec le rang de profession et les competences de recherche (Chercheurs peuvent booster les experimentations des artisans partenaires).

**Recettes uniques liees au personnage** : Ces recettes sont intransmissibles. Elles representent le secret professionnel de l'artisan. L'objet produit par une telle recette porte le nom de l'artisan et est souvent de qualite Excellent ou Chef-d'oeuvre par defaut.

## 3.6 Crafting collaboratif

Certaines recettes de rang 5-6 necessitent plusieurs artisans simultanes. Le systeme de crafting collaboratif fonctionne ainsi :

1. Le maitre-artisan initie le craft collaboratif dans un atelier de guilde.
2. Il designe les co-artisans (qui doivent etre presents physiquement dans l'atelier).
3. Chaque co-artisan contribue une phase specifique (chacun a sa propre fenetre de precision).
4. Le bonus de qualite est la moyenne des performances individuelles, avec un bonus de +10 pour la collaboration.
5. Le maitre-artisan recoit l'essentiel des PGC, les co-artisans recoivent 30% chacun.

Exemples de crafts collaboratifs :
- Tour d'assaut mobile (Ingenieur de siege x3 + Charpentier x1)
- Armure de faction legendaire (Forgeron Grand Maitre + Tailleur Grand Maitre + Bijoutier Enchanteur rang 5)
- Grand Elixir de faction (Alchimiste Grand Maitre + Botaniste rang 4 + Fermier Apiculteur rang 5)
- Artefact restaure de rang legendaire (Restaurateur Grand Maitre + Alchimiste rang 4 + Enchanteur rang 5)

---

# PARTIE 4 — SYSTEME CARAVANIER

## 4.1 Vue d'ensemble

Le systeme Caravanier est l'un des systemes les plus complexes et les plus risques du jeu. Il connecte les economies de toutes les zones du monde, transporte les marchandises militaires vers les fronts, et cree des opportunites de PvP economique uniques.

Un convoi est un objet du monde : visible, ciblable, pillable. Ce n'est pas une simple transaction de base de donnees — c'est une presence physique dans le monde qui doit etre escortee, protegee, et correctement planifiee.

## 4.2 Interface de convoi

**Tableau de bord du Caravanier** :

- **Onglet Cargo** : Liste des marchandises chargees, poids total, volume total, valeur declaree (pour l'assurance), valeur marche estimee a destination.
- **Onglet Route** : Carte de la route selectionnee avec affichage : distance en km, temps estime, niveau de danger, embuscades signalees (selon competences), postes de gardes NPC, villes intermediaires.
- **Onglet Escorte** : Gestion de l'equipe d'escorte. Gardes NPC (tiers de puissance, cout/heure), joueurs mercenaires (offre/demande), coût total d'escorte.
- **Onglet Profit** : Calcul de profitabilite en temps reel. Profit brut (prix a destination), - frais d'escorte, - frais d'assurance (optionnel), - taxes de faction (automatiques), = profit net estimé.

## 4.3 Systeme de cargo

**Poids et volume** : Chaque item a un poids (en grammes/kg) et un volume (en litres). Un chariot standard supporte 500 kg et 200 litres. Une mule de guerre : 800 kg / 350 litres.

**Types de marchandises** :

| Categorie | Exemples | Specificites |
|-----------|----------|--------------|
| Brutes | Minerais, bois, pierre | Lourd, faible valeur/kg, peu convoitable |
| Transformees | Lingots, planches, potions | Moyen, valeur/kg correcte |
| Rares | Gemmes, artefacts, mithral | Leger, tres haute valeur, tres convoite |
| Militaires | Armes, armures, explosifs | Lourd, haute valeur, cible prioritaire |
| Alimentaires | Rations, festins, alcools | Moyen, valeur variable, perimables |
| Perimables | Potions fraîches, nourriture crue | Duree de validite limitee, valeur chute si livraison tardive |

**Gestion des perissables** : Les potions et la nourriture fraîche ont une duree de validite. Un convoi trop lent (detour, retard) peut faire perdre de la valeur aux perissables (jusqu'a 50% de perte si livraison avec retard de 2x le temps estime).

## 4.4 Routes

**Routes predefinies** : Affichees sur la carte principale. Chaque route a des statistiques fixes :
- Distance (km)
- Danger (1-5)
- Escorte recommandee (nombre de gardes)
- Temps estimé
- Profit de base

**Routes libres** : Un Caravanier peut decider de prendre une route non officielle. Avantages : potentiellement plus rapide ou plus discrete. Inconvenients : pas de donnees de danger, pas de postes de gardes NPC, risque d'embuscade non signalee beaucoup plus eleve.

**Cartes de routes** (Cartographes) : Une carte de route achetee ou trouvee revele une route secrete. Ces routes offrent soit un raccourci (temps -20%, mais danger +1), soit une route alternative evitant des zones de guerre actives.

### Routes principales

```toml
[[convoy_routes]]
id = "route_royale_nord"
origin = "capitale_empire"
destination = "cite_nordique_empire"
distance_km = 120
danger_level = 2
recommended_escort = 2
travel_time_hours = 6
base_profit_modifier = 1.2
notes = "Route principale, bien gardee, peu profitable"

[[convoy_routes]]
id = "col_des_ombres"
origin = "cite_nordique_empire"
destination = "forteresse_confed"
distance_km = 35
danger_level = 5
recommended_escort = 6
travel_time_hours = 2
base_profit_modifier = 3.5
notes = "Zone de guerre active, tres risquee, tres profitable"
faction_restriction = "none"
inter_faction = true

[[convoy_routes]]
id = "route_commerciale_libre"
origin = "ville_neutre_marche"
destination = "capitale_confed"
distance_km = 80
danger_level = 3
recommended_escort = 3
travel_time_hours = 4
base_profit_modifier = 1.8
notes = "Passe par territoire neutre, accessible a toutes factions"
inter_faction = true
```

## 4.5 Embuscades

**Detection** : Un Caravanier avec le reseau d'informateurs (rang 2+) recoit des alertes sur les embuscades signalee sur sa route. La precision de l'information depend du rang :
- Rang 2 : alerte si embuscade active sur la route (sans localisation)
- Rang 3 : localisation approximative (zone de 500m)
- Rang 4 : localisation precise et nombre estimé d'attaquants (avec marge d'erreur)

**Mecanique d'embuscade cote attaquant** :
1. Le joueur Bandit/Mercenaire (ou groupe) se positionne sur une route connue.
2. Il peut "Preparer une embuscade" (temps de preparation 2 minutes, necessite d'etre stationnaire).
3. Une embuscade preparee est plus difficile a detecter (bonus de dissimulation).
4. Quand le convoi entre dans la zone, le systeme detecte la proximite et lance un compteur de 30 secondes avant confrontation.
5. Le Caravanier voit une alerte "DANGER : Mouvement suspect detecte" et peut tenter de fuir ou d'appeler ses escortes.

**Resultat de l'embuscade** :
- Si le convoi est detruit : les attaquants recuperent 60% du cargo, 40% est perdu/detruit dans la bagarre.
- Si le convoi s'echappe : les attaquants ne recuperent rien, penalite de reputation (-10 en zone legale).
- Si le convoi bat les attaquants : les attaquants droppent leurs items selon les regles de full loot PvP.

**Pillage de cargo** : Les items du cargo sont physiquement dropes sur les corps des betes de somme/chariots detruits. Ils doivent etre ramasses manuellement. Un convoi tres chargé peut avoir trop d'items a ramasser pour une petite bande d'attaquants — certains items de faible valeur restent sur le terrain.

## 4.6 Systeme d'escorte

**Gardes NPC** :
- Disponibles dans les villes (tavernes, bureaux de recrutement)
- Trois tiers : Soldat (cout 10 po/heure, equivalent niveau 20), Veteran (25 po/heure, niveau 35), Elite (60 po/heure, niveau 50)
- Les gardes NPC fuient si leur vie descend sous 20% (ils ne sont pas suicidaires)
- Nombre maximum de gardes NPC : 6 (limite technique)

**Mercenaires joueurs** :
- Le Caravanier peut poster une offre de mission d'escorte dans la taverne/AH (categorie Missions)
- L'offre specifie : route, duree estimee, compensation (or + part des profits optionnelle)
- Les joueurs Mercenaires (ou tout combattant cherchant de l'or) peuvent accepter
- Le contrat est lie : si le Mercenaire abandonne le convoi en cours de route, il perd une partie de sa paye et recoit un malus de reputation Mercenaire
- Si le convoi est attaque, les Mercenaires combattent normalement (full loot PvP des attaquants si victoire)

**Contrat d'escorte TOML** :

```toml
[escort_contract]
id = "escorte_001"
convoy_id = "convoi_alphonse_271"
caravanier_id = "player_uuid_caravanier"
route_id = "col_des_ombres"
start_time = "2026-03-01T08:00:00Z"
duration_estimated_hours = 2
compensation_base_gold = 500
compensation_on_success_bonus = 200
npc_guards = [
  { tier = "veteran", quantity = 3, cost_per_hour = 25 },
]
player_escorts = []
status = "recruiting"
```

## 4.7 Marche des prix fluctuants

Les prix des marchandises ne sont pas fixes. Ils dependent de l'offre et de la demande locale.

**Mecanisme** : Chaque zone possede un "stock de demande" pour chaque categorie de marchandises. Lorsque des marchandises arrivent (via convoi ou production locale), le stock augmente et les prix baissent. Lorsque des marchandises sont consommees (achetees par les NPC ou les joueurs), le stock diminue et les prix montent.

**Impact de la guerre** : En zone de guerre, la demande en materiaux militaires explose. Les zones isolees (routes coupees) voient les prix de tout monter (inflation de guerre). Les zones abondamment ravitaillees ont des prix bas.

**Affichage pour le Caravanier** : La carte commerciale (acces via l'interface Caravanier) affiche une heatmap des prix : vert (prix hauts, bonne destination), rouge (prix bas, mauvaise destination), avec une legende par categorie de marchandises.

## 4.8 Impact sur la faction

**Livraison militaire** : Si un convoi livre des marchandises militaires (armes, armures, explosifs, rations) directement a un fort ou a un intendant militaire de faction, le Caravanier recoit :
- Un bonus de profit de 20% sur le prix de la livraison militaire
- Des Eclats de faction (monnaie de faction) en proportion de la valeur militaire livree
- Des PGC de la Guilde des Caravaniers

**Impact tactique** : Un fort bien ravitaille en potions, armes et munitions a ses defenses boosted (+10% resistance des defenses). Un fort non ravitaille voit ses defenses s'affaiblir progressivement (mechanique de moral et d'approvisionnement).

## 4.9 Schema TOML complet d'un convoi

```toml
[convoy]
id = "convoi_alphonse_271"
caravanier_id = "player_alphonse_uuid"
caravanier_rank = 4
specialization = "convoyeur"

route_id = "col_des_ombres"
origin = "cite_nordique_empire"
destination = "forteresse_frontiere_est"
departure_time = "2026-03-01T08:00:00Z"
estimated_arrival = "2026-03-01T10:00:00Z"

[convoy.cargo]
total_weight_kg = 420.5
total_volume_liters = 180.0
items = [
  { item_id = "epee_guerre_bon", quantity = 30, weight_kg = 7.5, unit_value_gold = 85 },
  { item_id = "potion_soin_majeur", quantity = 200, weight_kg = 0.3, unit_value_gold = 45, expires_hours = 24 },
  { item_id = "baril_explosif", quantity = 5, weight_kg = 12.0, unit_value_gold = 350 },
  { item_id = "ration_campagne", quantity = 500, weight_kg = 0.4, unit_value_gold = 3 },
]
declared_value_gold = 14850
insurance_coverage = 0.80
insurance_premium_gold = 594

[convoy.escort]
npc_guards = [
  { tier = "veteran", quantity = 3 },
  { tier = "elite", quantity = 1 },
]
player_escorts = ["player_mercenaire_001", "player_mercenaire_002"]
total_escort_cost_gold = 350

[convoy.economics]
destination_price_modifier = 3.2
gross_revenue_gold = 47520
escort_cost_gold = 350
insurance_premium_gold = 594
faction_tax_rate = 0.05
faction_tax_gold = 2376
net_profit_gold = 44200
military_delivery_bonus = 0.20
```

---

# PARTIE 5 — COMMERCE ET MARCHES

## 5.1 Marches fixes

Chaque ville possede un marche fixe. Son animation varie selon la taille de la ville :

**Marche de village** : Quelques etals NPC (prix fixes), 1 emplacement de boutique joueur.
**Marche de ville** : 3-5 etals NPC, 5-10 emplacements boutiques joueurs, un crieur public (prix courants).
**Grand Marche de capitale** : 10+ etals NPC, 20-30 emplacements boutiques joueurs, acces a l'AH de faction, salle des encheres.
**Marche de la ville neutre** : Emplacements etendus pour toutes factions, acces a l'AH inter-factions (frais reduits pour les membres de la Guilde des Marchands).

**Regles des emplacements de boutiques joueurs** :
- Loyer hebdomadaire (proportionnel a la taille de la ville : 50 po/semaine en village, 500 po/semaine en capitale)
- Le joueur peut fixer ses propres prix
- La boutique reste active meme si le joueur est hors-ligne (NPC gerant)
- Si le loyer n'est pas paye, la boutique est fermee et les items retournes au coffre de guilde

## 5.2 Hotel des Ventes (AH) de faction

L'AH de faction est l'equivalent du marche electronique. Disponible dans toutes les capitales et grandes villes de la faction.

**Fonctionnement** :
1. Le vendeur liste un item avec un prix minimum et optionnellement un prix d'achat immediat.
2. L'item reste en vente pour 48h (renouvellement : 10 po).
3. Les acheteurs peuvent enchérir ou acheter immediatement.
4. En cas d'enchere, la duree restante repart a 10 minutes si une enchere est placee dans les dernieres 10 minutes.
5. Le vendeur recoit le paiement moins 5% de taxe de marche.

**Recherche et filtres** : L'AH possede un systeme de filtres avances : par categorie, qualite, statistiques, niveau requis, prix. Les Marchands rang 3+ ont acces a un filtre "Occasion" (items sous-evalues de 30%+).

## 5.3 AH inter-factions

Les factions en guerre ne peuvent pas commercer directement. Cependant, les **Guildes Marchandes** (guildes civiles) maintiennent des postes de commerce neutres dans les villes libres.

**Acces** : Via la Guilde des Marchands rang 3+ (frais reduits), ou via un Courtier NPC en ville neutre (frais eleves : 12%).

**Restrictions** : Les items militaires (armes de siege, explosifs de siege) ne peuvent pas etre echanges inter-factions via l'AH neutre. Les items standards (materiaux, consommables, equipements generiques) sont librement echangeables.

**L'Espion commercial** : Un Marchand rang 5 peut acceder aux informations de l'AH ennemi (lecture seule, avec risque de contre-espionnage). Il peut identifier les shortages (items manquants) et les surabondances chez l'ennemi.

## 5.4 Systeme d'encheres

Les encheres se tiennent dans les salles d'encheres des capitales. Elles permettent la vente d'items rares (Chef-d'oeuvre, legendaires, artefacts restaures, cartes uniques).

**Format** :
- Annonce 24h a l'avance (crieur public, panneau d'affichage en jeu)
- Mise de depart fixee par le vendeur
- Duree de l'enchere : 1 heure de jeu
- Increment minimum : 5% de la mise courante
- Achat immediat optionnel : 3x la mise de depart estimee
- Frais de salle : 3% de la valeur finale

**Encheres de guerre** : Pendant les periodes de siege, des salles d'encheres temporaires peuvent etre ouvertes pour la vente d'armes de siege et de materiaux rares. Ces encheres sont souvent tres animees.

## 5.5 Espion commercial — Detail mecanique

L'Espion commercial (Marchand rang 5, specialisation) est le seul civil capable d'obtenir des informations economiques sur les factions ennemies.

**Actions disponibles** :
- **Observation de marche** (passif) : Toutes les 4 heures, recoit un rapport sur les 5 items les plus demandes dans la faction ennemie la plus proche. Risque de detection : 5%.
- **Infiltration commerciale** (actif, 24h de cooldown) : Se rend physiquement en territoire ennemi sous couverture (necessite Deguisement avance) et consulte directement l'AH ennemi en lecture. Risque de detection : 30%. Si detecte, declenche un evenement PvP (gardes ennemis en alerte).
- **Vente d'intelligence** : Peut vendre ses rapports d'espionnage a d'autres joueurs de sa faction ou directement a la trésorerie de faction contre des Eclats.

---

# PARTIE 6 — SYSTEME DE HOUSING

## 6.1 Philosophie du housing

Le housing dans Allumina suit le modele FFXIV : des instances separees, pas de housing en open world. Chaque logement est une instance privee accessible via des portes d'entree dans les zones residentielles des capitales et villages.

Ce choix preserve l'esthetique de l'open world (pas de constructions qui encombrent le terrain), tout en offrant une experience de personalisation complete et des espaces fonctionnels.

## 6.2 Types de logements

### Chambre (logement de depart)

- Surface : petite (1 piece)
- Stockage : 1 coffre (50 slots)
- Ateliers possibles : Aucun (espace insuffisant)
- Entretien : 50 po/mois de jeu
- Obtention : Disponible des le niveau 5, achete a l'intendant de faction (500 po)
- Bonus : Lit de repos (+10% regeneration XP pendant 8h apres avoir dormi)

### Appartement

- Surface : moyenne (2-3 pieces)
- Stockage : 2 coffres (75 slots chacun)
- Ateliers possibles : Etabli simple, alambic simple
- Entretien : 200 po/mois
- Obtention : Rang de faction 2+, achete (2000 po) ou loue (200 po/semaine)
- Bonus : Atelier bonus +5 pour la specialisation correspondante

### Maison

- Surface : grande (4-6 pieces + jardin)
- Stockage : 4 coffres (100 slots chacun) + stockage jardin (plantes, animaux)
- Ateliers possibles : Forge personnelle, atelier de tailleur, alambic avance, etabli de bijoutier, bibliothèque
- Entretien : 200 po/mois (base) + 50 po par atelier installe
- Obtention : Rang de faction 3+, achete (8000 po)
- Bonus : Atelier bonus +10, jardin pour culture (Botaniste/Fermier)

### Manoir

- Surface : tres grande (8-12 pieces + grand jardin + sous-sol)
- Stockage : 8 coffres + 2 coffres secrets + salle de tresorie
- Ateliers possibles : Tous les ateliers, plus : forge de mithral, laboratoire alchimique, salle cartographique
- Entretien : 800 po/mois
- Obtention : Rang de faction 4+, achete (30000 po) ou par quete de prestige
- Bonus : Atelier bonus +15, sous-sol = entrepot special Caravanier (point de depart de convois depuis le logement)

### Chateau de clan

- Surface : enorme (donjon, grande salle, armurerie, tresorerie, quartiers)
- Stockage : 20+ coffres de clan + chambre forte
- Ateliers possibles : Tous, plus : forge de siege, laboratoire de recherche, salle de formation (Enseignants)
- Entretien : 5000 po/mois (de la tresorerie de clan)
- Obtention : Rang de clan maximum + mission de fondation + achat terrain de clan (50000 po)
- Capacite : 50 membres avec quartiers individuels

## 6.3 Decoration

Les items de decoration sont craftes par les Charpentiers (meubles), Tailleurs (tentures, tapis), Bijoutiers (chandeliers, vitraux), et Fermiers (plantes decoratives).

Chaque piece a un nombre maximum de slots de decoration (5 pour une chambre, 20 pour une grande salle de manoir, 100 pour la grande salle d'un chateau).

Les items de decoration sont categorises : fonctionnels (coffres, ateliers, lits) et decoratifs purs (statues, tableaux, trophees de monstres). Les items decoratifs purs n'ont pas de limite stricte autre que le nombre de slots.

**Trophees de monstres** : Certains monstres rares droppent des "trophees" (tete de dragon, peau de demon, etc.) qu'on peut accrocher dans son logement. Ces trophees donnent des bonus passifs mineurs (+2% experience en zone similaire) et surtout un prestige social (les visiteurs peuvent les voir).

## 6.4 Ateliers integres — Bonus

Chaque atelier installe dans un logement confere un bonus de crafting :

| Atelier | Installee dans | Bonus |
|---------|---------------|-------|
| Forge simple | Appartement+ | +5 score qualite forge |
| Forge avancee | Maison+ | +10 score qualite forge |
| Forge de mithral | Manoir+ | +15 score, accès recettes mithral |
| Alambic simple | Appartement+ | +5 score qualite alchimie |
| Laboratoire alchimique | Manoir+ | +15 score, acces recettes avancees |
| Etabli de bijoutier | Maison+ | +10 score qualite bijouterie |
| Bibliotheque | Maison+ | +5% PGC gagnes en crafting, acces recettes cachees +5% |
| Salle cartographique | Manoir+ | +10 qualite cartes, revelement bonus sur cartes |
| Salle de formation | Chateau clan | Permet formation groupee (Enseignants) |

## 6.5 Stockage Caravanier — Entrepot special

Le sous-sol d'un Manoir peut etre configure en entrepot de Caravanier. Avantages :
- Stockage de cargo volumique (500 slots contre 100 dans un coffre standard)
- Point de depart de convois (le Caravanier peut lancer un convoi depuis son logement sans se rendre a la taverne)
- Conservation amelioree des perishables (+50% duree de validite pour les items stockes dans cet entrepot)
- Acces direct a l'interface de commande de gardes NPC (sans se deplacer au bureau de recrutement)

## 6.6 Housing de clan — Chateau

Le chateau de clan est un espace de vie collective qui sert aussi de base operationnelle.

**Salles fonctionnelles** :

- **Grande salle** : Lieu de reunion, affichage des annonces de clan, trophees collectifs
- **Salle de guerre** : Carte tactique interactive (Cartographes peuvent y deposer leurs cartes), planification de sieges
- **Tresorerie** : Gestion par le Banquier de clan. Coffres de clan (acces par rang de clan), rapport financier mensuel
- **Armurerie** : Stockage d'equipements de clan (armes de siege, armures de reserve, potions de guerre)
- **Forge de siege** : Permet la construction d'armes de siege (Ingenieur de siege requis) directement dans le chateau de clan
- **Laboratoire de recherche** : Projets de recherche Inventeur, bibliotheque de recettes de clan
- **Quartiers** : Chaque membre a une chambre (bonus de repos)
- **Cachot** : (optionnel, roleplay) Permet de "detenir" un prisonnier de guerre (PvP consenti, max 24h de jeu)

## 6.7 Regles de visite

Chaque logement a un parametre de confidentialite :
- **Public** : N'importe qui peut entrer (en cliquant sur la porte)
- **Amis** : Seuls les joueurs sur la liste d'amis peuvent entrer
- **Clan** : Membres du clan uniquement
- **Invite** : Le proprietaire doit envoyer une invitation active
- **Prive** : Personne ne peut entrer (sauf le proprietaire)

## 6.8 Decay et entretien

Si le loyer mensuel n'est pas paye :
- **Mois 1** : Avertissement, bonus d'atelier reduits de 50%
- **Mois 2** : Bonus d'atelier desactives, ateliers non accessibles
- **Mois 3** : Le logement passe en mode "saisi". Les items restent stockes mais inaccessibles. Le logement est mis en vente par l'administration de faction.
- **Mois 4** : Vente aux encheres force du logement. Les items du joueur sont transferes dans un coffre d'entrepot municipal (accessible pour 6 mois de jeu, puis supprimes).

---

# PARTIE 7 — SYSTEME DE GUERRE RvR

## 7.1 Structure du RvR

Allumina suit le modele Dark Age of Camelot : trois factions en conflit permanent pour le contrôle de zones neutres strategiques (les "Terres de Guerre").

**Les trois factions** :
- Empire d'Allumina (theme : ordre imperial, chevalerie, technologie)
- Confederation des Libres (theme : liberte, magie naturelle, federation de cites)
- Culte de l'Ombre (theme : magie obscure, hierarchie ésoterique, immortalite)

**Les Terres de Guerre** : Zone centrale disputee, contenant des forts, des chateaux, des ressources rares, et les portails vers les donjons de haut niveau. Aucune faction ne "possede" cette zone de facon permanente.

## 7.2 Points de controle

### Forts

Un fort est un ensemble de structures defensives (murs, tours, portes, donjon central) qu'une faction peut capturer et defendre. Il y a 12 forts en Terres de Guerre.

**Capture** : Pour capturer un fort, une faction doit :
1. Detruire ou ouvrir la porte principale (Ingenieur de siege, explosifs, belier)
2. Eliminer le Capitaine Defenseur (boss NPC si le fort est non occupe par des joueurs, ou le commandant PJ)
3. Tenir le Drapeau Central pendant 5 minutes sans interruption

**Defense** : Les defenseurs peuvent : reparer les murs (Charpentier/Ingenieur), resupplier les munitions (Caravanier), utiliser les armes de siege fixes installes sur les murs.

**Benefices du controle de fort** :
- Acces au coffre du fort (ressources accumulees)
- Point de resurrection pour les membres de la faction dans la zone
- Reduction des temps de deplacement vers les zones proches (-20%)
- Bonus de ressources (mine, foret, champ) dans la zone de controle du fort

### Tours de guet

Plus petites que les forts, les tours de guet (24 en Terres de Guerre) offrent :
- Vision etendue (minimap elargie dans la zone)
- Poste de tir (arbaletriers NPC additionnels)
- Point de respawn mineur

**Capture** : Tenir le point central 2 minutes.

### Cols strategiques

Des passages obligatoires entre les zones. Controler un col permet de bloquer les convois ennemis (peage ou blocage complet).

## 7.3 Sieges — Mecanique complete

Un siege est l'evenement RvR le plus complexe et le plus gratifiant du jeu.

### Phase de preparation (avant siege)

1. **Annonce** : Un clan declare un siege sur un fort ennemi (cout : 500 Eclats). L'annonce est visible par toute la faction ennemie (et alliee) 30 minutes a l'avance.
2. **Ravitaillement** : Les deux factions ont 30 minutes pour se positionner et ravitailler. Les Caravaniers militaires apportent potions, munitions, materiaux de reparation.
3. **Construction de siege offensive** : Les Ingenieurs de siege construisent catapultes, balistes, tours d'assaut dans la zone d'approche.
4. **Renforcement defensif** : Les Ingenieurs defenseurs renforcent les murs, installent des pieges (Mecaniciens), versent de l'huile bouillante (Alchimistes) aux positions cles.

### Phase d'assaut

**Artillerie** : Les catapultes et balistes tirent sur les murs (degats a la structure). Chaque tir est resolu par le systeme : position de la machine, competence de l'operateur (Ingenieur de siege), type de projectile.

**Breche** : Quand une section de mur tombe a 0 HP, une breche est ouverte. Les assaillants peuvent passer a travers. Les defenseurs tentent de combler la breche (Charpentiers avec materiaux de reparation).

**Porte principale** : La porte peut etre enfoncee au belier (lent, tres expose au feu) ou exploitee par une charge perforante (Explosiviste, risquee mais rapide).

**Combat interieur** : Une fois la breche ouverte, le siege devient du combat ouvert (PvP avec full loot). Les defenseurs ont l'avantage du terrain (positions elevees, couloirs etroits).

**Capitulation** : Le fort tombe quand le Drapeau Central est tenu 5 minutes, ou quand tous les defenseurs sont morts/fuient.

### Phase de consolidation

1. Le fort change de mains.
2. La faction conquérante peut piller la tresorerie (30% du stock).
3. Les Charpentiers/Ingenieurs commencent les reparations.
4. Les Caravaniers commencent le ravitaillement.
5. La faction perd ses bonus de ressources de zone, la faction gagnante les acquiert.

## 7.4 Systeme du Marchand de Guerre

Le "Marchand de Guerre" est le systeme economique militaire de chaque faction. C'est a la fois un NPC specifique et un systeme de financement de l'effort de guerre.

### Structure

Chaque faction possede un Marchand de Guerre dans sa capitale. C'est un NPC special qui :
- Achete des marchandises militaires (armes, armures, potions) a prix premium (prix marche +15%)
- Vend des consommables de guerre (munitions de siege, huile de feu, renforts de mur) uniquement contre des Eclats de faction
- Gere la distribution des Eclats en recompense des actions militaires

### Financement de la guerre

La richesse economique d'une faction alimente directement sa capacite militaire :

```
budget_militaire = (taxes_commerce × 0.40) + (contributions_volontaires) + (pillage_forts_ennemis) + (recompenses_convois_militaires)
```

Ce budget est gere par les Banquiers de rang 5-6 (Grand Banquier de Faction) et distribue en :
- Paiement des gardes de fort NPC (40% du budget)
- Achat de materiaux de siege stockes dans les forts (30%)
- Recompenses aux joueurs pour actions militaires (30%)

### Culture martiale

La "culture martiale" d'une faction est un score global qui augmente avec les victoires en RvR et diminue avec les defaites. Elle influence :
- Le recrutement de Mercenaires (une faction avec haute culture martiale attire plus de Mercenaires car reputee pour payer bien)
- Le moral des gardes NPC (+5% a +20% selon le score)
- Les bonus passifs des bannières de clan en RvR

### Les Mercenaires comme facteur de bascule

Les Mercenaires sont des joueurs (ou groupes) qui ne sont affilies a aucune faction (ou qui ont choisi ce role). Ils sont recrutes par les factions pour :
- Renforcer les sieges (cote attaque ou defense)
- Escorter des convois militaires
- Effectuer des raids eclairs sur des ressources ennemies

Le prix du Mercenaire est determine par l'offre et la demande. En periode de siege majeur, les tarifs s'envolent. La faction qui peut se permettre de payer plus recrute une force de frappe superieure. C'est intentionnel : la richesse economique d'une faction se traduit directement en puissance militaire via les Mercenaires.

## 7.5 Ressources de zone et impact economique

### Ressources en Terres de Guerre

Les Terres de Guerre contiennent des ressources uniques :
- **Minerai de fer de guerre** : Minerai de qualite superieure, uniquement en Terres de Guerre, zone de danger 4-5
- **Cristaux de mana** : Composant alchimique ultra-rare, uniquement dans les donjons de Terres de Guerre
- **Bois de chene noir** : Materiau de construction exceptionnel, forets disputees
- **Pierre runique** : Materiau pour les Forgeurs de runes et Brodeurs, uniquement en Terres de Guerre

### Impact economique de la guerre

**Routes coupees** : Quand une faction controle un col, elle peut percevoir un peage sur les convois (meme allies). Refus du peage = attaque du convoi par des gardes NPC/PJ.

**Disruption de production** : Les fermes et mines en zone de guerre peuvent etre attaquees, reduisant la production de la faction adverse.

**Prise de tresorerie** : La fraction de tresorerie capturee dans les forts represente un transfert direct de richesse.

**Inflation de guerre** : Le systeme detecte le niveau d'activite militaire et applique un modificateur global sur les prix NPC (+15% a +40%).

---

# PARTIE 8 — SYSTEMES SOCIAUX

## 8.1 Systeme de groupe (Party)

**Taille** : 6 joueurs maximum.

**Formation** : Invitation directe ou recrutement via le tableau de groupe (LFG system).

**Partage des recompenses** :
- XP : Partage egal entre membres du groupe dans un rayon de 50 metres
- Loot : Round-robin (rotation automatique) par defaut, configurable en "Besoin/Convoitise" ou "Maitre du loot"
- Or des monstres : Partage egal automatique

**Roles** : Le groupe peut designer des roles (Tank, Healeur, DPS, Support) mais il n'y a pas de mecanique de role forcee. La designation est indicative pour la coordination.

**Communication** : Canal de chat de groupe (prive), marqueurs sur la minimap, indicateurs de vie/mana des membres.

## 8.2 Systeme de raid

**Taille** : 24 joueurs (4 groupes de 6).

**Formation** : Necessite un commandant de raid (qui doit etre membre de clan ou avoir un rang de faction minimum 2).

**Structure** : 4 groupes. Chaque groupe garde sa cohesion. Le commandant peut donner des ordres a tout le raid.

**Loot de raid** : Geré par le commandant de raid (systeme de points de priorite ou distribution manuelle). Les items legendaires et artefacts sont distribues selon les regles internes du clan/groupe.

**Raid ouvert** : Pour les world bosses et evenements, un "raid ouvert" peut accueillir jusqu'a 100 joueurs. Les recompenses sont distribuees selon la contribution (DPS, soin, support).

## 8.3 Systeme de clan/guilde

### Creation

- Cout : 1000 po + 10 signatures de joueurs fondateurs
- Nom unique (verifie a la creation)
- Symbole de clan (choix parmi des symboles predifinis + couleurs customisables)
- Faction d'appartenance (un clan ne peut etre que d'une faction, sauf clans de Mercenaires neutres)

### Rangs de clan

| Rang | Titre | Permissions |
|------|-------|-------------|
| 1 | Recrue | Acces au chat de clan, coffre de clan lecture seule |
| 2 | Membre | Vote sur les decisions mineures, acces au coffre de clan ecriture |
| 3 | Officier | Recruter, kicke les rangs 1-2, gerer certains coffres |
| 4 | Lieutenant | Declarer des missions de clan, gerer les ressources militaires |
| 5 | Commandant | Declarer des sieges, gerer le housing de clan |
| 6 | Chef de clan | Tous les droits, election/demission |

### Tresorerie de clan

La tresorerie est geree par le Banquier de clan (joueur rank 2+ dans la Guilde des Banquiers qui est aussi membre du clan). Elle contient :
- Tresor commun (or)
- Ressources communes (materiaux de crafting)
- Equipements de reserve
- Eclats de faction

Les depenses sont loggees avec auteur et motif. Le Chef de clan peut definir des limites de depense par rang.

### Alliance de clans

Deux clans ou plus peuvent former une alliance :
- Partage du canal de chat d'alliance
- Coordination de sieges (les membres d'alliance peuvent rejoindre un siege declare par un clan allie)
- Partage de la carte de guerre
- Les alliances ne partagent PAS la tresorerie (sauf accord specifique avec le systeme de "pret inter-clan")

## 8.4 Systeme de reputation

**Reputation de faction** : Gagnee par les actions de faction (RvR, quetes de faction, convois militaires, sieges). Niveaux : Hostile / Neutre / Ami / Honore / Venere / Exalte. Chaque niveau debloque des avantages (prix NPC reduits, acces a des zones speciales, titres).

**Reputation de guilde civile** : Gagnee par les interactions avec la guilde (crafting, commandes, quetes). Independante de la reputation de faction. Hauts niveaux = acces a des recettes exclusives, emplacements de marche premium.

**Reputation de Mercenaire** : Pour les joueurs jouant le role de Mercenaire. Haute reputation = recrutement plus facile, tarifs plus eleves. Trahison d'un contrat = malus majeur.

**Reputation criminelle** : Gagnee par les actes criminels (attaque de civils hors zones PvP, vol, pillage de convois de sa propre faction). Hauts niveaux = gardes NPC hostiles, prix NPC tres eleves, expulsion possible des guildes civiles.

## 8.5 Systeme d'amis et liste noire

**Liste d'amis** :
- Voir le statut en ligne/hors ligne
- Voir la zone actuelle (avec accord de l'autre joueur)
- Message prive prioritaire
- Invitation directe au groupe

**Liste noire** :
- Bloque tous les messages prives
- Le joueur bloque ne peut pas vous inviter
- Invisible sur la liste d'amis de l'autre

**Note de joueur** : Possibilite d'ajouter une note personnelle sur n'importe quel joueur (visible uniquement par vous). Utile pour noter les Mercenaires de confiance, les vendeurs fiables.

## 8.6 Systeme de chat

| Canal | Portee | Conditions |
|-------|--------|------------|
| Local | 50 metres | Tous |
| Zone | Zone entiere | Tous |
| Faction | Toute la faction | Membre de faction |
| Clan | Membres du clan | Membre de clan |
| Alliance | Membres d'alliance | Membre d'un clan en alliance |
| Commerce | Toute la faction | Tous (filtre items, prix) |
| Mercenaire | Tous les Mercenaires | Statut Mercenaire actif |
| Chuchotement | Direct joueur-joueur | Tous |
| RvR | Zone de guerre | Tous les joueurs en Terres de Guerre |

**Filtres automatiques** : Le canal Commerce filtre automatiquement les messages de vente structurees ("[Vente] Epee de guerre Excellent x2 - 1500 po chacune"). Les messages mal formates peuvent etre soumis a un cooldown de 30 secondes.

---

# PARTIE 9 — TAXES ET ECONOMIE DES OUTLAWS

## 9.1 Systeme de taxes de faction

### Prelevement automatique

Toutes les transactions monetaires dans les zones de faction sont soumises a une taxe automatique :
- Ventes en AH de faction : 5% (non modulable)
- Ventes en boutiques joueur en ville de faction : 3%
- Paiements de quetes : 2%
- Gains de RvR : 3%

### Taxe configurable

Les dirigeants de faction (joueurs aux plus hauts rangs de faction ou representants elus) peuvent ajuster la taxe de commerce generale entre 1% et 15%. Les decisions de taxe sont soumises a un vote de la communaute de faction (systeme de gouvernance simplifie).

**Impact d'une taxe elevee** : Les commercants migrent vers les villes neutres. La faction perd du volume commercial mais gagne plus par transaction. Au-dessus de 10%, des mouvements de protestation NPC apparaissent (flavour).

**Impact d'une taxe basse** : Plus de commerce, moins de revenus fiscaux. En periode de guerre, cela peut etre catastrophique pour le budget militaire.

### Distribution des taxes

Les taxes collectees sont reparties :
- 40% : Budget militaire (gardes de forts, armes de siege NPC)
- 30% : Infrastructure (reparation des routes, batiments NPC)
- 20% : Trésorerie de faction (reserve d'urgence)
- 10% : Guilde civile la plus active du mois (bonus de PGC pour ses membres)

## 9.2 Economie des Outlaws

Les Outlaws sont des joueurs qui ont accumule une reputation criminelle suffisante pour etre banni des villes legales de leur faction (ou de toutes les factions). Ils operent dans les zones grises du monde.

### Marche noir

Le marche noir est un AH "clandestin" accessible dans :
- Les tavernes douteuses des villes neutres
- Les repaires de bandits en open world
- Via les Caravaniers avec la competence "Contrebande" (rang 5)
- Via les Marchands Espion commerciaux (en echange d'informations)

**Items disponibles** : Toutes les marchandises standards, plus :
- Armes de siege inter-factions (illegal mais possible)
- Potions interdites (effets overpowered temporaires, risque de malus d'addiction)
- Informations d'espionnage volees
- Items voles (tracables : si un garde NPC detecte l'item, il peut etre confisque)
- Equipements de factions ennemies (butin de pillage)

**Taxe du marche noir** : 0% (pas de taxation formelle), mais une "commission" de 8-12% est prise par le gestionnaire du marche noir (NPC ou joueur Marchand hors-la-loi).

### Recel

Un Forgeron avec le trait "Forgeur Discret" (obtenu via une quete speciale) peut effacer les marques d'identification sur des items voles, les rendant non tracables. Cout : variable selon la valeur de l'item.

### Contrebande

Les Caravaniers rang 5 (competence Contrebande) peuvent transporter des marchandises illegales sans detection automatique par les gardes NPC. Cependant :
- Si un joueur Garde de faction (ou un Marchand Espion ennemi) les detecte, la cargaison peut etre confisquee
- En zone neutre, les gardes NPC ne verifient pas (zone franche)
- En zone de faction alliee, risque de 5% par poste de garde traverse

### Faction des Outlaws

Les joueurs avec haute reputation criminelle peuvent rejoindre une "faction Outlaw" informelle. Cela leur donne :
- Acces au marche noir premium
- Repaires fortifies en open world
- Capacite de "taxer" les convois (legal dans les zones non controlees par une faction)
- Cible des gardes NPC de toutes les factions
- Possibilite de "blanchir" leur reputation via une quete longue et couteuse (Redemption)

---

# PARTIE 10 — SCHEMAS TOML COMPLETS

## 10.1 Schema de profession complet

```toml
# Schema complet d'une profession
[profession.forgeron]
id = "forgeron"
guild_id = "guilde_des_forgerons"
display_name = "Forgeron"
description = "Artisan du metal, fabricant d'armes et d'armures."
faction_access = "all"  # ou liste de factions
max_rank = 6
starting_recipe_ids = ["epee_courte_fer", "tete_hache_fer", "plaque_acier_simple"]
requires_tool = "marteau_de_forge"
requires_workstation = "enclume"
specializations = ["armurier", "ingenieur_siege", "forgeur_runes"]

[profession.forgeron.ranks]
rank_1 = { title = "Apprenti Forgeron", pgc_required = 0, skill_unlocks = ["fonte_basique", "polissage", "aiguisage_manuel"] }
rank_2 = { title = "Compagnon Forgeron", pgc_required = 500, recipe_count_required = 10, skill_unlocks = ["trempe_controlee", "forge_alliages_simples"] }
rank_3 = { title = "Artisan Forgeron", pgc_required = 2000, recipe_count_required = 20, faction_quest_required = true, skill_unlocks = ["forge_precision", "identification_metaux"] }
rank_4 = { title = "Maitre Forgeron", pgc_required = 5000, recipe_count_required = 40, masterwork_required = true, skill_unlocks = ["forge_chaud_avancee", "reparation_terrain"] }
rank_5 = { title = "Grand Maitre Forgeron", pgc_required = 15000, recipe_count_required = 60, peer_vote_required = true, skill_unlocks = ["maitrise_alliage", "forge_equipe"] }
rank_6 = { title = "Archonte des Forgerons", pgc_required = 50000, election_required = true, skill_unlocks = ["commande_de_guerre"] }

[profession.forgeron.specialization.ingenieur_siege]
id = "ingenieur_siege"
unlock_rank = 4
exclusive_skills = ["genie_militaire", "construction_rapide", "armement_de_fort"]
exclusive_recipes = ["catapulte_campagne", "baliste_legere", "belier_siege", "tour_assaut_mobile", "herse_renforce"]
passive_bonus = { siege_machine_durability = 0.15, construction_speed = 0.20 }
```

## 10.2 Schema de recette complet

```toml
[recipe.catapulte_campagne]
id = "catapulte_campagne"
display_name = "Catapulte de campagne"
profession = "forgeron"
specialization = "ingenieur_siege"
rank_required = 4

materials = [
  { item_id = "bois_chene", quantity = 20, quality_min = "bon" },
  { item_id = "plaque_acier", quantity = 8 },
  { item_id = "corde_chanvre_renforce", quantity = 4 },
  { item_id = "mecanisme_tension_bronze", quantity = 2 },
]

output = { item_id = "catapulte_campagne", quantity = 1, quality_roll = true }

craft_time_seconds = 7200  # 2 heures
workshop_required = "forge_de_siege"
collaborative_required = false
min_artisans = 1
max_artisans = 3  # bonus si plus d'artisans

quality_modifiers = { base = 45, rank_bonus = 5, tool_bonus = 10, workshop_bonus = 15 }
timing_window = true

experience_gain = 500
pgc_gain = 200

[recipe.catapulte_campagne.stats]
damage_per_shot = { min = 800, max = 1200 }
reload_time_seconds = 45
range_meters = 150
projectile_types = ["rocher", "baril_explosif", "baril_feu"]
max_hp = 2000
```

## 10.3 Schema de logement complet

```toml
[housing.manoir]
id = "manoir_standard"
type = "manoir"
display_name = "Manoir"
min_faction_rank = 4
purchase_cost_gold = 30000
monthly_upkeep_gold = 800

[housing.manoir.capacity]
rooms = 10
decoration_slots_total = 150
storage_chests = 8
secret_storage_chests = 2

[housing.manoir.workshops]
allowed = ["forge_simple", "forge_avancee", "alambic_simple", "alambic_avance", "etabli_bijoutier", "bibliotheque", "salle_cartographique", "atelier_tailleur", "entrepot_caravanier"]
max_simultaneous = 5
upkeep_per_workshop_gold = 50

[housing.manoir.special.entrepot_caravanier]
requires_profession = "caravanier"
requires_profession_rank = 3
storage_slots = 500
perishable_duration_bonus = 0.50
convoy_launch_point = true
npc_guard_access = true

[housing.manoir.permissions]
default_visibility = "prive"
allowed_settings = ["public", "amis", "clan", "invite", "prive"]
```

## 10.4 Schema de fort RvR complet

```toml
[rvr_fort]
id = "fort_col_des_aigles"
display_name = "Fort du Col des Aigles"
zone = "terres_de_guerre_nord"
position = { x = 1250.0, y = 890.0 }

[rvr_fort.structure]
walls = [
  { id = "mur_nord", hp = 5000, armor = 200 },
  { id = "mur_sud", hp = 5000, armor = 200 },
  { id = "mur_est", hp = 4000, armor = 150 },
  { id = "mur_ouest", hp = 4000, armor = 150 },
]
gates = [
  { id = "porte_principale", hp = 3000, armor = 300, breachable_by = ["belier", "charge_perforante"] },
]
towers = [
  { id = "tour_nord_est", hp = 2000, provides = ["vision_extended", "archer_position"] },
  { id = "tour_nord_ouest", hp = 2000, provides = ["vision_extended", "siege_weapon_slot"] },
]
keep = { id = "donjon_central", hp = 8000, capture_point = true, capture_hold_seconds = 300 }

[rvr_fort.garrison]
npc_guards = [
  { type = "soldat_garde", count = 10, level = 30 },
  { type = "archer_garde", count = 5, level = 28 },
  { type = "capitaine_fort", count = 1, level = 45, is_boss = true },
]
upkeep_per_day_eclats = 100

[rvr_fort.rewards]
on_capture = { treasury_loot_percent = 0.30, eclats_reward = 500 }
on_hold_per_hour = { eclats_per_member = 2, resource_generation = ["minerai_fer_guerre_x5", "pierre_taille_x10"] }

[rvr_fort.treasury]
current_gold = 15000
current_eclats = 2000
resources = [
  { item_id = "minerai_fer_guerre", quantity = 150 },
  { item_id = "potion_soin_majeur", quantity = 300 },
]
```

## 10.5 Schema de contrat de Mercenaire

```toml
[mercenary_contract]
id = "contrat_merc_001"
type = "escorte_convoy"
employer_id = "clan_les_freres_de_fer"
employer_faction = "empire_allumina"

target = "convoi_alphonse_271"
mission = "Escorter le convoi depuis Cite Nordique jusqu'a Forteresse Frontiere Est via le Col des Ombres."

start_time = "2026-03-01T08:00:00Z"
estimated_duration_hours = 3
expiry_time = "2026-03-01T07:30:00Z"  # fermeture du recrutement

compensation = { base_gold = 600, success_bonus_gold = 300, eclats_bonus = 50 }
penalty_abandonment = { reputation_loss = 25, gold_forfeit = 200 }

accepted_mercenaries = [
  { player_id = "player_mercenaire_001", reputation_score = 87, accepted_at = "2026-03-01T05:00:00Z" },
]

status = "in_progress"
faction_restriction = "none"  # ouvert a toutes factions, meme ennemies
max_mercenaries = 4
```

## 10.6 Schema de session de marche commercial

```toml
[market_listing]
id = "listing_001"
seller_id = "player_alphonse"
item_id = "epee_guerre_excellent"
quantity = 3
quality = "excellent"
price_per_unit_gold = 1800
auction = false
listed_at = "2026-03-01T09:00:00Z"
expires_at = "2026-03-03T09:00:00Z"
renewal_cost_gold = 10
market_tax_rate = 0.05
current_bids = []

[market_stats.epee_guerre]
item_id = "epee_guerre"
zone_id = "capitale_empire"
demand_index = 75  # 0-100, 75 = demande elevee
supply_index = 40  # 0-100, 40 = offre moderee
average_price_gold = 1650
price_trend = "hausse"  # hausse, stable, baisse
war_inflation_modifier = 1.20
last_updated = "2026-03-01T08:45:00Z"
```

## 10.7 Schema d'alliance de clans

```toml
[clan_alliance]
id = "alliance_bouclier_du_nord"
display_name = "Le Bouclier du Nord"
founding_date = "2026-02-01T00:00:00Z"
founding_clans = ["clan_freres_fer", "clan_gardiens_collines", "clan_marteaux_nains"]

member_clans = [
  { clan_id = "clan_freres_fer", rank = "fondateur", joined_at = "2026-02-01T00:00:00Z" },
  { clan_id = "clan_gardiens_collines", rank = "fondateur", joined_at = "2026-02-01T00:00:00Z" },
  { clan_id = "clan_marteaux_nains", rank = "membre", joined_at = "2026-02-10T00:00:00Z" },
]

shared_resources = { war_map = true, chat = true, treasury = false }
siege_coordination = true
max_member_clans = 8

[clan_alliance.governance]
voting_threshold = 0.67  # 2/3 des clans fondateurs pour les decisions majeures
expulsion_requires_vote = true
```

---

# ANNEXE — GLOSSAIRE

**AH** : Hotel des Ventes (Auction House).

**Artefact** : Objet ancien decouvrable par les Archeologues, generalement puissant et rare.

**Artisan** : Joueur appartenant a une guilde civile.

**Chef-d'oeuvre** : Plus haute qualite de crafting (score >= 80).

**Civil** : Joueur specialise dans une profession civile (artisan, commercant).

**Combattant** : Joueur specialise dans le combat (guerrier, mage, rodeur, etc.).

**Convoi** : Groupe de chariots/betes de somme transporte des marchandises d'un point a un autre.

**Eclat** : Monnaie de faction, obtenue par actions militaires et civiles de faction.

**Embuscade** : Attaque preparee sur un convoi ou un groupe de joueurs en deplacement.

**Mercenaire** : Joueur vendant ses services militaires aux factions ou aux individus.

**Outlaw** : Joueur banni des villes legales suite a une reputation criminelle excessive.

**PGC** : Points de Guilde Civile, monnaie interne a chaque guilde.

**RvR** : Realm vs Realm, guerre entre les trois factions pour le controle de zones.

**Siege** : Attaque organisee d'un fort ou chateau tenu par une faction adverse.

**Terres de Guerre** : Zone centrale disputee par les trois factions, contenant les ressources les plus rares.

---

*Document AL-Economy-Social v1.0 — Allumina — Miyukini COG*
*@id: AL-Economy-Social @do: reference @role: game-designer @layer: 3 @human: miyuk*
