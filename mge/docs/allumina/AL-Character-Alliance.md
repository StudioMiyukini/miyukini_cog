<!-- @id: AL-Character-Alliance @do: reference @role: game-designer @layer: 3 @human: miyuk -->

# Allumina — Classes de l'Alliance de Rive

**Statut :** Référence canonique v1.0
**Date :** 2026-02-28
**Scope :** 9 classes jouables de la faction Alliance de Rive — rôles, arbres de compétences, schémas TOML

---

## Table des matières

1. [Vue d'ensemble — L'Alliance et ses combattants](#1-vue-densemble--lalliance-et-ses-combattants)
2. [Corsaire](#2-corsaire--dps-mêlée-agile)
3. [Duelliste](#3-duelliste--dps-précision)
4. [Marchande de Guerre](#4-marchande-de-guerre--support-économique)
5. [Lame des Docks](#5-lame-des-docks--assassin-stealth)
6. [Timonier de Combat](#6-timonier-de-combat--tank-manœuvre)
7. [Tireur d'Élite](#7-tireur-délite--dps-distance)
8. [Barde de Rue](#8-barde-de-rue--support-désorganisation)
9. [Contrebandier](#9-contrebandier--hybride-furtif)
10. [Boucanier](#10-boucanier--dps-mêlée-brutal)
11. [Synergies de faction](#11-synergies-de-faction)
12. [Schémas TOML complets](#12-schémas-toml-complets)

---

## 1. Vue d'ensemble — L'Alliance et ses combattants

L'Alliance de Rive n'a jamais eu d'armée impériale. Elle a des guildes, des contrats et des individus capables de se battre pour ce qui leur appartient ou ce qui peut se monnayer. Ses combattants ne sont pas des soldats — ce sont des spécialistes, des survivants, des entrepreneurs de la violence. Chaque classe reflète un rôle économique ou social précis dans la société marchande de Rive : le corsaire opère sous licence, la marchande de guerre traite la logistique comme un avantage tactique, le barde de rue vend ses services à quiconque paie assez.

La doctrine de combat de l'Alliance peut se résumer ainsi : **éviter la confrontation directe quand c'est possible, l'emporter vite quand c'est inévitable, et facturer le résultat dans les deux cas.**

### Tableau des classes

| # | Classe | Rôle | Armes | Style |
|---|--------|------|-------|-------|
| 1 | Corsaire | DPS Mêlée Agile | Sabre, Crochet, Dague | Mobilité, repositionnement, pression |
| 2 | Duelliste | DPS Précision | Rapière, Main-gauche | Combo parade-riposte, finesse |
| 3 | Marchande de Guerre | Support Économique | Arbalète légère, Fouet | Buffs ressources, logistique, soutien |
| 4 | Lame des Docks | Stealth Burst | Couteau, Dague courte | Invisibilité, poison, exécution |
| 5 | Timonier de Combat | Tank Manœuvre | Bouclier naval, Gaffet | Contrôle de zone, mur humain |
| 6 | Tireur d'Élite | DPS Distance | Arc de composite, Arbalète | Piégeage, positionnement, précision |
| 7 | Barde de Rue | Support Désorganisation | Mandoline, Couteau léger | Auras, CC, sabotage moral |
| 8 | Contrebandier | Hybride Furtif | Pistolet de poing, Dague | Déguisement, pièges, tromperie |
| 9 | Boucanier | DPS Mêlée Brutal | Sabre lourd, Pistolet | Frénésie, rage maritime, gunplay |

### Identité sociale de départ

Tous les joueurs de l'Alliance commencent avec l'identité **Homme libre (Freelander)** — pas de classe de combat imposée, pas de hiérarchie de naissance. Le rang se mérite par les actes et l'argent.

---

## 2. Corsaire — DPS Mêlée Agile

### 2.1 Identité

**Rôle :** DPS Mêlée Agile
**Armes principales :** Sabre courbe, Crochet de bord, Dague de ceinture
**Couleur de classe :** Ambre maritime (#FF8C00)

### 2.2 Style de combat

Le Corsaire combat comme un navire léger sous voile : jamais dans l'axe, toujours en mouvement. Il exploite les cordages, les bordages, les reliefs du terrain comme autant de trampolines vers l'arrière de son adversaire. En mer ou sur un quai, il traite l'environnement comme une arme supplémentaire — une barrique renversée, un mât brisé, une corde de rigging deviennent des outils tactiques. Son crochet lui permet d'accrocher des surfaces, des boucliers, ou des membres adverses pour créer des ouvertures. Il ne gagne pas les combats de force brute ; il gagne les combats d'angle et d'initiative.

### 2.3 Arbre 1 — Lame des Flots

**Thème :** Dommages purs au sabre, combos tranchants, saignements

| Type | Nom | Effet |
|------|-----|-------|
| Active | Taille Oblique | Frappe diagonale rapide infligeant 140% ADPhy + saignement 3 tours |
| Active | Volée de Rive | Série de 4 frappes légères en 1,2s, chaque touche applique une accumulation de "Brèche" |
| Active | Estocade Finale | Dépense toutes les accumulations de Brèche (max 5) pour 80% par accumulation en un coup unique |
| Passive | Tranchant de Sel | Chaque saignement actif sur la cible augmente les dégâts du Corsaire de 4% (cumulable ×5) |
| Passive | Acier Huilé | Réduction du délai de récupération des actives du sabre de 12% |

### 2.4 Arbre 2 — Mobilité Marine

**Thème :** Déplacement, esquive, repositionnement, engagement/désengagement

| Type | Nom | Effet |
|------|-----|-------|
| Active | Abordage | Dash vers la cible (12m), accroche au crochet, immobilise 0,8s, dégâts 90% ADPhy |
| Active | Roulade de Pont | Esquive en tonneau (invulnérabilité 0,4s), reset partiel du cooldown d'Abordage |
| Active | Saut de Beaupré | Bond arrière de 8m immédiat, laisse un leurre de fumée grise au point d'origine |
| Passive | Pieds Marins | Déplacement +15% pendant 4s après chaque esquive |
| Passive | Insaisissable | Les attaques ennemies ont 8% de chance de rater si le Corsaire a bougé dans les 2 dernières secondes |

### 2.5 Arbre 3 — Tactiques de Bord

**Thème :** Utilisation de l'environnement, combat à deux armes, techniques corsaires

| Type | Nom | Effet |
|------|-----|-------|
| Active | Crochet Tournant | Frappe à l'arc avec le crochet (portée 4m), applique Désarmé à l'ennemi pendant 2s |
| Active | Double Dégaine | Attaque simultanée sabre + dague, inflige 110% ADPhy + 80% ADPhy, courte fenêtre de parade |
| Active | Envoi de Grenadine | Lance un flacon d'huile de mer qui ralentit et flambe sur ignition (zone 3m, 4s) |
| Passive | Maître d'Équipage | Augmente les dégâts du groupe de 3% si le Corsaire est en dessous de 50% de vie (pression heroïque) |
| Passive | Réflexes de Rigging | Réduit la durée des effets de contrôle de masse reçus de 20% |

### 2.6 Compétences Signature

**Pavillon Noir** *(Cooldown : 90s)*
Le Corsaire hisse métaphoriquement son pavillon — pendant 8s, chaque frappe qui touche applique automatiquement Saignement sans chance de résistance, la vitesse d'attaque augmente de 25%, et toute esquive réussie génère une accumulation de Brèche gratuite. En fin de durée, une décharge d'Estocade Finale automatique est déclenchée si au moins 3 accumulations de Brèche sont présentes. L'effet est rompu si le Corsaire est immobilisé.

**Arraisonnement** *(Cooldown : 120s)*
Le Corsaire lance son crochet sur la cible (portée 18m) et se propulse vers elle en ligne droite, ignorant les obstacles du terrain sur son chemin. À l'arrivée, il effectue une frappe circulaire touchant tous les ennemis dans un rayon de 4m pour 200% ADPhy. Toute cible touchée subit Ralentissement 40% pendant 6s. Si la cible principale est en dessous de 25% de vie, Arraisonnement inflige un multiplicateur de x2 et ignore 30% des armures.

### 2.7 Synergie de groupe

Le Corsaire génère des accumulations de **Brèche Partagée** : quand il applique 3 saignements distincts sur une même cible, tous les alliés qui frappent cette cible dans les 6s suivantes ignorent 15% de son armure. Synergie naturelle avec le Boucanier (dommages bruts) et la Marchande de Guerre (buffs offensifs).

### 2.8 Lore

Dans la société de l'Alliance, le Corsaire est à la fois héros populaire et fauteur de troubles institutionnel. Opérant sous la Charte des Corsaires (instituée après la Crise de An 220), il est techniquement licencié par l'Assemblée de Rive pour attaquer les navires ennemis — ce qui lui confère une légitimité de surface tout en lui laissant une latitude d'action que l'armée régulière n'aurait jamais. La population de Selmara chante leurs noms dans les tavernes. L'Archonte d'Auranthos les tolère car ils font le travail sale que la diplomatie ne peut pas faire.

---

## 3. Duelliste — DPS Précision

### 3.1 Identité

**Rôle :** DPS Précision
**Armes principales :** Rapière à garde, Main-gauche (dague parade ou cape), Épée courte de réserve
**Couleur de classe :** Or clair (#FFD700)

### 3.2 Style de combat

Le Duelliste est l'antithèse de la brutalité. Là où le Boucanier écrase et le Corsaire virevolte, le Duelliste calcule. Chaque combat est une conversation : il lit les intentions adverses dans les micro-mouvements de leur pied d'appui, anticipe les fenêtres d'attaque, et construit méthodiquement un tableau de vulnérabilités qu'il clôture en une riposte précise et dévastiatrice. Sa parade n'est pas de la défense — c'est le premier mouvement de son attaque suivante. Il accumule des charges de **Précision** par les parades réussies et les frappes non absorbées, puis les libère dans des touches ciblées visant des zones anatomiques précises.

### 3.3 Arbre 1 — École de la Rapière

**Thème :** Maîtrise de l'arme, combos d'estocade, frappes à la pointe

| Type | Nom | Effet |
|------|-----|-------|
| Active | Estocade Directrice | Frappe linéaire rapide 120% ADPhy, génère 1 accumulation de Précision |
| Active | Quarte Haute | Triple estocade vers trois zones du corps (épaule, flanc, cuisse), chaque touche applique -5% armure (cumulable ×3, 10s) |
| Active | Pointe Mortelle | Dépense 5 Précision : frappe unique 350% ADPhy ciblant un emplacement spécifique, ignore le blocage des boucliers |
| Passive | Angle Parfait | Chaque Estocade Directrice consécutive sans autre active intercalée augmente les dégâts de 8% (max ×5) |
| Passive | Fil du Rasoir | Les effets de réduction d'armure de Quarte Haute durent 5s de plus |

### 3.4 Arbre 2 — Art de la Parade

**Thème :** Parades actives, génération de Précision, contre-attaques

| Type | Nom | Effet |
|------|-----|-------|
| Active | Parade de Cercle | Activation manuelle (0,6s de fenêtre) : pare toute attaque physique, génère 2 Précision + riposte automatique 80% ADPhy |
| Active | Déviation Croisée | Avec la main-gauche, déflecte un projectile ou une magie légère (absorbe jusqu'à 200% ADMag), génère 1 Précision |
| Active | Contre-Tempo | Après une parade réussie, attaque immédiate avec désavantage temporel : la cible est stun 0,6s et reçoit 160% ADPhy |
| Passive | Économie de Lame | Les parades réussies réduisent le prochain cooldown d'active de 1s |
| Passive | Instinct du Bretteur | La fenêtre de Parade de Cercle s'étend à 0,9s si le Duelliste a moins de 70% de vie |

### 3.5 Arbre 3 — Finesse Marchande

**Thème :** Techniques de combat issues du milieu des affaires (duels d'honneur, intimidation)

| Type | Nom | Effet |
|------|-----|-------|
| Active | Feinte Commerciale | Faux mouvement d'attaque vers la droite, véritable frappe à gauche : inflige 130% ADPhy si la cible a commencé à parer |
| Active | Défi en Duel | Désigne une cible unique : pendant 8s, cette cible inflige 20% de dégâts en moins à toutes cibles sauf le Duelliste |
| Active | Désarmement Formel | Tente de désarmer l'ennemi (portée mêlée, résistible) : si réussi, l'ennemi ne peut attaquer que de la main secondaire pendant 4s |
| Passive | Sang-Froid de Comptoir | Immunité aux effets de peur et de panique |
| Passive | Réputation de Lame | La présence du Duelliste réduit la précision des ennemis non-ciblés de 5% dans un rayon de 8m |

### 3.6 Compétences Signature

**Sept Passes de Bareth** *(Cooldown : 60s)*
Enchaînement de 7 frappes en 2,5s ciblant des zones anatomiques alternées (gauche/droite/centre). Chaque frappe consomme 1 Précision et inflige 100% ADPhy + 30% par accumulation de Précision dépensée. Si les 7 passes sont complétées sans interruption, la dernière inflige un bonus de 400% ADPhy et applique Saignement Profond (dégâts sur durée ×2 pendant 12s). L'interruption par un stun ou knockback annule les frappes restantes mais conserve les Précisions non dépensées.

**Jugement de Lame** *(Cooldown : 180s — passif déclenché)*
Le Duelliste désigne mentalement un adversaire comme sa "cible de jugement". Si cet adversaire est tué dans les 120s par le Duelliste ou un allié, toutes les accumulations de Précision sont instantanément maximisées (5 charges) et le prochain Pointe Mortelle coûte 0 Précision. Si la cible survit 120s, le Duelliste gagne 3 Précision passifs à la place.

### 3.7 Synergie de groupe

Le Duelliste peut partager ses réductions d'armure via le système de **Lecture Commune** : quand il applique 3 stacks de -armure sur une cible, un indicateur visuel apparaît pour ses alliés. Les dégâts de tous les alliés contre cette cible augmentent de 8% pendant la durée des stacks.

### 3.8 Lore

Dans l'Alliance de Rive, le duel d'honneur est une institution juridique légalement reconnue depuis l'An 89 AO. Les litiges commerciaux, les trahisons de contrat, les disputes de frontières portuaires : tout peut se régler en duel si les deux parties consentent et qu'un arbitre de l'Assemblée supervise. Les Duellistes sont à la fois des combattants et des juristes de terrain — leur art est professionnel, leur réputation vaut une fortune, et les plus réputés sont parfois plus puissants qu'un petit archonte.

---

## 4. Marchande de Guerre — Support Économique

### 4.1 Identité

**Rôle :** Support — Buffs économiques, logistique de combat, gestion de ressources
**Armes principales :** Arbalète légère de poche (attaque à distance secondaire), Fouet de cargaison (contrôle mêlée), Parchemins tactiques (actives de zone)
**Couleur de classe :** Ambre pâle (#FFD580)

### 4.2 Style de combat

La Marchande de Guerre ne tue pas — elle fait en sorte que ses alliés tuent mieux, plus vite, plus longtemps. Elle traite le champ de bataille comme un marché : ressources à distribuer, flux à optimiser, inefficacités à éliminer. En combat, elle tient une position légèrement en retrait, activant des auras d'efficacité qui réduisent les coûts de compétences de ses alliés, améliorent la qualité de leurs consommables en temps réel, et transforment les ressources récupérées sur les ennemis en avantages tactiques immédiats. Son fouet lui permet de contrôler l'espace proche sans prendre de risques. En situation désespérée, elle sait se battre — mais préfère que ce soient les autres.

### 4.3 Arbre 1 — Comptabilité du Sang

**Thème :** Génération et distribution de ressources via les kills alliés

| Type | Nom | Effet |
|------|-----|-------|
| Active | Inventaire de Guerre | Analyse le champ de bataille : identifie 3 ennemis "rentables" (plus de ressources à la mort), marqués pour les alliés |
| Active | Dividende de Victoire | Chaque ennemi marqué tué par un allié génère un Jeton de Rive (monnaie temporaire, max 12) |
| Active | Liquidation | Dépense des Jetons de Rive pour distribuer des soins instantanés à tous les alliés (15% PV max par Jeton, répartis équitablement) |
| Passive | Marge Brute | Les compétences de soin et de buff coûtent 10% de mana en moins |
| Passive | Économie d'Échelle | Chaque allié supplémentaire présent dans un rayon de 20m augmente l'efficacité des actives de 5% (max +20%) |

### 4.4 Arbre 2 — Logistique Maritime

**Thème :** Gestion des consommables, ravitaillement, renforcement des alliés

| Type | Nom | Effet |
|------|-----|-------|
| Active | Caisse de Ravitaillement | Pose une caisse (objet de terrain, 30s) : les alliés qui passent dessus régénèrent 20% de leurs ressources de compétence |
| Active | Potion de Qualité Supérieure | Améliore le prochain consommable utilisé par un allié ciblé (double l'effet, dans les 15s) |
| Active | Chargement Urgent | Téléporte une Caisse de Ravitaillement déjà posée vers un allié ciblé à portée (portée 30m) |
| Passive | Entrepôt Mobile | Le Marchande peut porter 2 fois plus de consommables que la limite de classe normale |
| Passive | Qualité Garantie | Les soins appliqués par les alliés proches de la Marchande (20m) sont 8% plus efficaces |

### 4.5 Arbre 3 — Tactiques Commerciales

**Thème :** Manipulation économique du combat, pression psychologique, aura de commandement

| Type | Nom | Effet |
|------|-----|-------|
| Active | Offre Non Négociable | Ordonne à un allié de charger une cible désignée : l'allié gagne +30% vitesse de déplacement et +15% dégâts pendant 6s |
| Active | Clause Pénale | Marque un ennemi : si cet ennemi attaque un allié autre que le tank dans les 8s, il subit un retour de dégâts 50% ADPhy |
| Active | Accord de Rive | Lie deux alliés pendant 10s : 20% des dégâts subis par l'un sont transférés à l'autre (réduction de pic de dégâts) |
| Passive | Autorité de Guilde | Les alliés dans un rayon de 15m ont leurs coûts de compétences réduits de 5% |
| Passive | Renseignement Commercial | La Marchande voit les valeurs de PV des ennemis (barre précise, pas une estimation) |

### 4.6 Compétences Signature

**Contrat de Sang** *(Cooldown : 120s)*
La Marchande signe un "contrat" avec un allié ciblé. Pendant 20s, cet allié est invulnérable aux one-shot (tout dégât qui le tuerait le laisse à 1 PV au lieu, une fois par activation du contrat). En échange, la Marchande subit 30% des dégâts absorbés par cette protection. Si l'allié tombe quand même, la Marchande récupère un **Grand Jeton de Rive** qui triple l'effet de la prochaine Liquidation.

**Économie de Guerre** *(Cooldown : 90s, aura passive 15s)*
La Marchande active pendant 15s une aura jaune visible : pendant cette durée, toutes les compétences actives des alliés dans un rayon de 25m ont leur cooldown réduit de 30%, et toute ressource consommée (mana, endurance) est remboursée à 20%. En fin d'aura, chaque allié ayant utilisé au moins une compétence pendant l'effet génère automatiquement 2 Jetons de Rive pour la Marchande.

### 4.7 Synergie de groupe

La Marchande est le nœud logistique du groupe. Ses Jetons de Rive sont une ressource partagée visible par tous — un groupe coordonné qui maximise les kills sur les ennemis marqués peut générer un flux de soins et de buffs qui maintient le groupe en combat bien au-delà de sa capacité brute. Elle est particulièrement puissante avec le Corsaire et le Boucanier qui génèrent beaucoup d'interactions offensives sur les ennemis marqués.

### 4.8 Lore

Les Marchandes de Guerre ne sont pas des mercenaires — elles sont des investisseuses. Elles rejoignent une campagne militaire comme on prend une participation dans une entreprise : avec un calcul précis du retour sur engagement. Historiquement, les grandes batailles navales de l'Alliance ont souvent eu une Marchande de Guerre à bord du navire amiral, transformant la gestion des réserves et du moral en art opérationnel. Les plus réputées possèdent leur propre réseau d'intelligence avant même de signer avec une faction.

---

## 5. Lame des Docks — Assassin Stealth

### 5.1 Identité

**Rôle :** Stealth — Burst, Poison, Exécution furtive
**Armes principales :** Couteau de lancer (à distance), Dague courte (mêlée), Aiguille empoisonnée (outil secondaire)
**Couleur de classe :** Gris ardoise (#708090)

### 5.2 Style de combat

La Lame des Docks a grandi dans les ruelles d'Auranthos ou les cales des navires — des endroits où survivre signifie ne jamais être vu avant de frapper. Son style de combat est presque entièrement concentré sur le premier contact : entrer en furtivité, positionner, choisir le bon moment, et éliminer en un minimum d'interventions. Chaque poison est composé avec précision — certains tuent vite, d'autres désorganisent, d'autres simulent une maladie naturelle pour éviter les représailles. Si le premier assaut échoue, elle dispose d'outils de désengagement et peut recommencer depuis l'ombre. Elle n'est pas conçue pour encaisser et elle ne s'y expose jamais volontairement.

### 5.3 Arbre 1 — Art du Poison

**Thème :** Compositions toxiques, effets sur la durée, debuffs progressifs

| Type | Nom | Effet |
|------|-----|-------|
| Active | Onguent Urticant | Applique un poison de contact à la prochaine frappe : 80% ADPhy + poison 3% PV/s pendant 8s (résistible) |
| Active | Extrait de Méduse | Poison neurologique : la cible perd 20% de précision et subit Confusion (chance de frapper un allié) pendant 6s |
| Active | Dose Létale | Cible déjà empoisonnée : triple la durée du poison actif et ajoute un effet de Fragilisation (-15% armure) jusqu'à guérison |
| Passive | Alchimie des Bas-Fonds | Les poisons de la Lame ignorent 25% de la résistance aux poisons des cibles |
| Passive | Accumulation Toxique | Chaque poison distinct actif sur une même cible augmente les dégâts de la Lame de 6% contre cette cible |

### 5.4 Arbre 2 — Ombre des Quais

**Thème :** Furtivité, dissimulation, mobilité en mode invisible

| Type | Nom | Effet |
|------|-----|-------|
| Active | Fond dans l'Ombre | Entre en furtivité (durée illimitée hors combat, 15s max en combat, 0,8s de délai de dissipation si attaqué) |
| Active | Pas Silencieux | En furtivité : déplacement +20%, ne laisse pas de traces audibles, invisibilité maintenue même en mouvement rapide |
| Active | Substitution d'Ombre | Laisse un leurre immobile à sa position et se téléporte jusqu'à 12m dans la direction visée (sort de furtivité après) |
| Passive | Peau de Nuit | La dissipation de la furtivité par des sources de détection ennemies prend 1,5s de plus |
| Passive | Instinct Prédateur | Bonus de dégâts de 40% sur la première attaque depuis la furtivité |

### 5.5 Arbre 3 — Contrats des Docks

**Thème :** Exécution cible unique, burst concentré, techniques d'assassinat

| Type | Nom | Effet |
|------|-----|-------|
| Active | Contrat Ouvert | Désigne une cible (HUD visible pour la Lame seulement) : pendant 30s, toutes ses attaques contre cette cible génèrent 10% de bonus de dégâts cumulatif |
| Active | Frappe du Couteau | Lance le couteau à distance (portée 20m), inflige 130% ADPhy + applique Saignement 4s — peut être utilisée depuis la furtivité sans la briser |
| Active | Jugulaire | Attaque mêlée ciblant la gorge (portée courte, timing précis requis) : inflige 300% ADPhy, applique Silence 3s (empêche les sorts verbaux) |
| Passive | Efficacité Contractuelle | Si la cible de Contrat Ouvert meurt dans les 30s, récupère 40% du coût en mana de toutes les actives utilisées |
| Passive | Spécialisation Anatomique | Les attaques ciblant le dos (cible ne faisant pas face à la Lame) infligent +25% dégâts |

### 5.6 Compétences Signature

**Exécution des Docks** *(Cooldown : 90s)*
Utilisable uniquement depuis la furtivité, sur une cible à moins de 5m. La Lame surgit dans le dos de la cible et plante sa dague en un point vital précis : inflige 500% ADPhy, applique tous les poisons actifs dans son répertoire simultanément, et passe automatiquement en désengagement (Fond dans l'Ombre immédiat). Si la cible meurt dans les 5s suivant l'exécution, le cooldown de Fond dans l'Ombre est annulé.

**Toile de Nuit** *(Cooldown : 60s)*
La Lame projette un filament de résine de calamar traité sur un ennemi ciblé (portée 15m). Ce filament est invisible pour l'ennemi mais visible pour la Lame. Pendant 20s, la Lame voit la position exacte de cet ennemi même à travers les murs et dans le brouillard. Si la Lame attaque depuis la furtivité dans les 20s, le bonus d'Instinct Prédateur est doublé (80% de dégâts supplémentaires). La toile se dissipe si la cible quitte la zone de 50m.

### 5.7 Synergie de groupe

La Lame des Docks est la championne du **burst sur cible prioritaire**. Dans un groupe, elle synchronise ses attaques avec le marquage de la Marchande de Guerre ou le Défi en Duel du Duelliste pour éliminer rapidement les ennemis à forte valeur (healers adverses, artilleurs). Elle peut également utiliser sa Toile de Nuit pour donner la position d'un ennemi fuyard à tout le groupe.

### 5.8 Lore

Les Lames des Docks ne se revendiquent d'aucune guilde officielle — c'est précisément leur valeur. Dans la société de l'Alliance où tout se négocie, les éliminations discrètes sont un service comme un autre. Les Lames opèrent via des intermédiaires, jamais de client direct. La Charte des Corsaires ne les couvre pas, mais l'Assemblée de Rive ferme les yeux tant que les "accidents" surviennent hors des frontières du territoire de l'Alliance.

---

## 6. Timonier de Combat — Tank Manœuvre

### 6.1 Identité

**Rôle :** Tank — Contrôle de zone, résistance, manœuvre tactique
**Armes principales :** Bouclier naval en bois ferrouté (arme et défense), Gaffet de bord (arme d'hast courte), Trident marin (alternative)
**Couleur de classe :** Bleu marine (#1C3A5F)

### 6.2 Style de combat

Le Timonier de Combat gouverne le champ de bataille comme il gouvernerait un navire dans la tempête : anticipation, décision précoce, exploitation des courants et contre-courants. Son bouclier n'est pas un outil de défense passive — c'est une arme de positionnement. Il pousse, déplace, crée des couloirs de passage ou des murs d'interception. En combat de groupe, il détermine qui avance et qui recule en forçant les repositionnements ennemis par la charge et la bousculade. Il absorbe des volumes de dégâts considérables grâce à sa constitution maritime et son expertise dans la lecture des attaques, mais son vrai pouvoir est géographique : il contrôle l'espace.

### 6.3 Arbre 1 — Muraille de Proue

**Thème :** Résistance, absorption de dégâts, protection physique

| Type | Nom | Effet |
|------|-----|-------|
| Active | Posture de Proue | Prend position défensive : +40% résistance physique, -20% vitesse, durée 6s ou jusqu'à annulation |
| Active | Bouclier de Mer | Absorbe les prochains 300% ADPhy en dégâts (bouclier temporaire, 8s) |
| Active | Couverture Maritime | Donne 20% de l'armure du Timonier à un allié ciblé pendant 10s |
| Passive | Coque Renforcée | +15% PV maximum |
| Passive | Lecture des Vagues | Réduit les dégâts reçus des attaques chargées (celles avec des temps d'incantation visibles) de 25% |

### 6.4 Arbre 2 — Manœuvre Navale

**Thème :** Positionnement, déplacements forcés, contrôle de zone

| Type | Nom | Effet |
|------|-----|-------|
| Active | Charge de Bordée | Charge en ligne droite (10m), repousse tous les ennemis sur son chemin de 4m, inflige 80% ADPhy |
| Active | Virage Brusque | Rotation instantanée à 180° avec frappe de bouclier latéral : stun 1s dans un arc de 90° devant lui |
| Active | Encerclement | Se déplace rapidement autour d'une cible (arc de 270°), se repositionne derrière elle — génère un Avantage de Flanc pour le prochain allié qui attaque cette cible (+20% dégâts) |
| Passive | Cap Maintenu | Le Timonier ne peut pas être knockback tant qu'il est en Posture de Proue |
| Passive | Trajectoire Prédictive | Identifie visuellement (pour lui seul) la trajectoire des projectiles entrants 0,3s avant leur arrivée |

### 6.5 Arbre 3 — Commandement de Pont

**Thème :** Leadership, buffs de moral, auras tactiques

| Type | Nom | Effet |
|------|-----|-------|
| Active | Ordre d'Abordage | Cri d'assaut : tous les alliés dans un rayon de 15m gagnent +15% vitesse d'attaque pendant 8s |
| Active | Tenir la Ligne | Crée une ligne imaginaire (6m de long) ancrée à sa position : les alliés derrière la ligne reçoivent +10% armure |
| Active | Point de Ralliement | Pose un marqueur de terrain (30s) : les alliés qui y reviennent récupèrent 5% de PV/s pendant 3s |
| Passive | Présence du Capitaine | Les alliés dans un rayon de 10m sont immunisés à la panique et aux effets de fuite forcée |
| Passive | Répartition de Barre | 10% des dégâts subis par les alliés dans un rayon de 8m sont transférés au Timonier |

### 6.6 Compétences Signature

**Manœuvre du Détroit** *(Cooldown : 120s)*
Le Timonier effectue une rotation complète avec son bouclier sur 360°, repoussant violemment tous les ennemis dans un rayon de 6m de 8m vers l'extérieur, infligeant 120% ADPhy et les immobilisant 2s. Simultanément, un couloir de 8m de large dans la direction de son regard s'ouvre : les alliés qui traversent ce couloir dans les 6s suivantes sont protégés par une réduction de dégâts de 30% et avancent 25% plus vite. Mécanique de percée stratégique pour casser un front ennemi.

**Ancre de Rive** *(Cooldown : 90s)*
Le Timonier plante son gaffet dans le sol et déclare une zone d'ancrage de 8m de rayon. Pendant 12s, aucun ennemi ne peut forcer un allié à quitter cette zone (knockback, charges, déplacements forcés sont absorbés). Tous les ennemis entrant dans la zone subissent un ralentissement de 30%. Le Timonier lui-même ne peut pas quitter la zone pendant la durée, mais ses PV régénèrent de 2% par seconde.

### 6.7 Synergie de groupe

Le Timonier définit la géographie du combat pour son groupe. Ses zones d'ancrage et ses lignes de commandement créent des structures spatiales que les autres classes exploitent : le Tireur d'Élite tire sur les ennemis bloqués par la Charge de Bordée, la Marchande place ses Caisses de Ravitaillement aux Points de Ralliement, le Duelliste profite de l'Avantage de Flanc généré par Encerclement.

### 6.8 Lore

Dans la tradition navale de Selmara, le timonier est l'officier qui prend les décisions tactiques en temps réel — pas le capitaine qui planifie la stratégie, mais celui qui adapte la route aux vents et aux courants de l'instant. Les Timoniers de Combat transposent cette responsabilité au combat terrestre : ils ne commandent pas par grade mais par compétence, et tout bon groupe de combattants de l'Alliance sait reconnaître instinctivement le meilleur Timonier et lui obéir.

---

## 7. Tireur d'Élite — DPS Distance

### 7.1 Identité

**Rôle :** DPS Distance — Pièges, positionnement, tirs de précision
**Armes principales :** Arc composite de Rive (long range), Arbalète de pont (medium range, réchargement lent mais puissant), Pièges mécaniques (déployables)
**Couleur de classe :** Vert olive (#6B8E23)

### 7.2 Style de combat

Le Tireur d'Élite traite le champ de bataille comme une carte marine : il identifie les couloirs de tir, les zones de couverture, et les angles d'approche avant même que le combat ne commence. Il prépare son terrain — pièges mécaniques aux passages obligés, fil de détente aux flancs, plumes de guidage pour corriger sa trajectoire dans le vent marin. En combat actif, il maintient une distance constante et punit tout ennemi qui tente de l'approcher par une salve de tirs ou un piège de repositionnement. Ses dégâts explosent quand la cible est immobile, ralentie ou piégée.

### 7.3 Arbre 1 — Tir de Pont

**Thème :** Précision à distance, bonus contre cibles immobiles, tirs chargés

| Type | Nom | Effet |
|------|-----|-------|
| Active | Tir Tendu | Charge 1,2s, libère une flèche 200% ADPhy — bonus 50% si la cible n'a pas bougé depuis 2s |
| Active | Salve de Rive | Tire 3 flèches rapides en 0,8s, 80% ADPhy chacune, légère dispersion |
| Active | Tir Perforant | Flèche traversant une ligne d'ennemis (jusqu'à 5 cibles), dégâts 120% ADPhy avec -15% par cible traversée |
| Passive | Précision Marine | +10% dégâts par tranche de 5m entre le Tireur et la cible (max +40% à 20m+) |
| Passive | Calme du Tireur | Réduit le temps de charge de Tir Tendu de 0,3s si le Tireur est stationnaire depuis 2s |

### 7.4 Arbre 2 — Ingénierie de Pont

**Thème :** Pièges mécaniques, zones de contrôle, préparation du terrain

| Type | Nom | Effet |
|------|-----|-------|
| Active | Piège à Câble | Dépose un câble de pont (invisible, 30s) : premier ennemi qui passe l'active se prend les pieds, chute 1,5s, prend 60% ADPhy |
| Active | Nasse de Rive | Lance un filet lesté (portée 15m) : immobilise l'ennemi touché 3s, -30% résistance aux dégâts physiques pendant l'immobilisation |
| Active | Bombe Fumigène | Lance un fumigène (zone 6m, 8s) : ennemis dans la zone ont -40% précision et ne peuvent pas cibler à distance |
| Passive | Ingénieur de Fortune | Peut déposer 2 pièges simultanément (normalement 1) |
| Passive | Déclencheur Sensible | Les pièges se déclenchent 0,2s plus rapidement, réduisant le temps de réaction pour les éviter |

### 7.5 Arbre 3 — Chasseur de Bounty

**Thème :** Ciblage prioritaire, marquage, bonus contre des cibles à haute valeur

| Type | Nom | Effet |
|------|-----|-------|
| Active | Marque de Chasse | Marque un ennemi (visible pour tous) : il reçoit +15% dégâts de toutes sources pendant 10s |
| Active | Tir à la Tête | Tir ciblé sur la tête (animation distincte, résistible) : 180% ADPhy + Étourdi 1,5s, raté si la cible bouge pendant le temps de charge |
| Active | Flèche Traçante | Flèche lumineuse qui révèle la position exacte d'un ennemi en furtivité dans une zone de 15m pendant 10s |
| Passive | Instinct de Chasseur | Les cibles avec moins de 30% de PV prennent 20% de dégâts supplémentaires de toutes les attaques du Tireur |
| Passive | Mémoire de Trajectoire | Après avoir touché une cible deux fois consécutives, la troisième frappe est automatiquement un tir parfait (pas de dispersion, pas de réduction de dégâts par la distance) |

### 7.6 Compétences Signature

**Barrage de Rive** *(Cooldown : 90s)*
Le Tireur désigne une zone de 10m de diamètre et tire une salve de 12 flèches en arc balistique sur cette zone en 4s. Chaque flèche inflige 80% ADPhy sur le point d'impact (5m de rayon chacune, zones superposables). Les ennemis qui restent dans la zone reçoivent potentiellement plusieurs impacts. Les ennemis immobiles ou ralentis reçoivent systématiquement 4 impacts minimum. Interrompible par un stun ou knockback.

**Embuscade de Guetteur** *(Passif déclenché, recharge 60s)*
Quand le Tireur est en position stationnaire depuis 5s et hors du champ visuel d'un ennemi, il entre automatiquement en mode Guetteur : les 2s suivantes, son prochain tir est un Tir Tendu à coût zéro (pas de charge visible), infligeant 300% ADPhy avec garantie de toucher. Ce bonus est annulé dès que le Tireur bouge ou est détecté.

### 7.7 Synergie de groupe

Le Tireur synergise avec toute forme de contrôle de masse : les pièges du Contrebandier, l'Ancre de Rive du Timonier, les immobilisations du Corsaire. Quand les ennemis sont cloués sur place, son bonus de dégâts sur cibles statiques et son Barrage de Rive deviennent dévastateurs. La Flèche Traçante est cruciale contre les groupes incluant une Lame des Docks adverse.

### 7.8 Lore

Dans les guerres navales de l'Alliance, les Tireurs d'Élite étaient positionnés dans les hunes — les plateformes en haut des mâts — pour cibler les officiers adverses avant que les navires ne s'abordent. Cette tradition de ciblage prioritaire et de positionnement préparatoire est entrée dans la culture de combat terrestre de l'Alliance. Les meilleurs sont des chasseurs de primes reconnus, engagés pour des "travaux difficiles" que même la Lame des Docks refuserait d'approcher en mêlée.

---

## 8. Barde de Rue — Support Désorganisation

### 8.1 Identité

**Rôle :** Support — Auras actives, désorganisation ennemie, manipulation morale
**Armes principales :** Mandoline de combat (instrument-arme, génère des effets soniques), Couteau léger de ceinture (défense rapprochée), Luth de guerre (version lourde pour effets de zone)
**Couleur de classe :** Jaune vif (#FFE44D)

### 8.2 Style de combat

Le Barde de Rue combat avec les mots, la musique et le rythme — littéralement. Ses instruments sont des armes acoustiques : des accords dissonants brisent la concentration des lanceurs de sorts, des rythmes précis synchronisent les attaques alliées, des mélodies enveloppantes calment le chaos pour permettre à ses alliés de récupérer. En combat, il maintient un flux musical continu, alternant des auras de soutien actif et des dissonances ciblant les ennemis. Il est fragile physiquement et le sait — sa survie dépend de sa mobilité et de ses alliés. Son danger réside dans sa capacité à transformer un groupe de combattants ordinaires en force synergisée.

### 8.3 Arbre 1 — Chants de Mer

**Thème :** Auras actives bénéfiques, synchronisation de groupe

| Type | Nom | Effet |
|------|-----|-------|
| Active | Chant du Vent Arrière | Aura 8m, 12s : alliés +15% vitesse de déplacement et +10% vitesse d'attaque |
| Active | Mélodie du Cap | Aura 12m, 15s : alliés régénèrent 2% de mana/s et leurs compétences coûtent 8% moins cher |
| Active | Chœur de Victoire | Aura 10m, 10s : chaque kill d'un allié dans l'aura soigne tous les alliés de l'aura de 5% PV max |
| Passive | Portée Acoustique | Les auras du Barde ont un rayon étendu de 3m supplémentaires |
| Passive | Résonance Prolongée | La durée de toutes les auras augmente de 20% |

### 8.4 Arbre 2 — Dissonances Marchandes

**Thème :** Effets sonores négatifs sur les ennemis, disruption, CC

| Type | Nom | Effet |
|------|-----|-------|
| Active | Accord Dissonant | Coup de corde brisé vers un ennemi (portée 10m) : interrompt l'incantation en cours, applique Silence 2s |
| Active | Cacophonie de Port | Zone sonore de 8m, 6s : tous les ennemis dans la zone ont -25% précision et subissent Confusion (20% de chance de frapper un allié) |
| Active | Glas de Rive | Note grave de mandoline envoyée à une cible unique : stun 1,5s, elle ne peut pas activer de compétences pendant la durée |
| Passive | Vibration Fracassante | Les effets sonores ennemis (cris de guerre, sorts soniques) dans un rayon de 15m ont 30% de chance d'être annulés |
| Passive | Oreille Absolue | Détecte les unités en furtivité par leur bruit de déplacement dans un rayon de 12m |

### 8.5 Arbre 3 — Histoires de Corsaires

**Thème :** Buffs ciblés narratifs, inspiration individuelle, malédictions verbales

| Type | Nom | Effet |
|------|-----|-------|
| Active | Ballade du Héros | Cible un allié : il reçoit +25% dégâts et immunité à la peur pendant 10s, et chaque kill génère 1 Jeton de Rive (si Marchande présente) |
| Active | Chanson de Honte | Cible un ennemi : il subit -20% dégâts infligés et ses alliés proches ont -10% moral (réduction dégâts 5% supplémentaires) pendant 8s |
| Active | Récit de Nuit | Raconte une histoire courte à voix haute (1,5s d'incantation) : tous les alliés dans 20m régénèrent 10% PV et sont immunisés aux effets de désorientation pendant 6s |
| Passive | Rumeur Propagée | La Chanson de Honte peut se propager à 1 ennemi supplémentaire adjacent à la cible initiale |
| Passive | Mémoire Collective | Si le Barde survit 2 minutes entières en combat sans mourir, tous ses buffs d'aura ont leur efficacité doublée pendant 30s |

### 8.6 Compétences Signature

**Symphonie de Rive** *(Cooldown : 120s)*
Le Barde joue une mélodie de 4s (interrompible par stun seulement) qui affecte simultanément alliés et ennemis : alliés dans 25m gagnent +20% à tous les dégâts et régénèrent 15% PV, ennemis dans 15m subissent -20% dégâts infligés et ont leurs cooldowns augmentés de 3s. Si la mélodie complète est jouée sans interruption, un bonus supplémentaire s'active : les 4 prochaines compétences utilisées par chaque allié présent ont leur cooldown réduit de 50%.

**Dernière Chanson** *(Passif d'urgence, recharge 300s)*
Si le Barde tombe en dessous de 10% de PV, il entonne automatiquement un fragment de "Dernière Chanson" — les 3s suivantes, il est invulnérable et soigne tous ses alliés dans 20m de 20% PV max. À la fin des 3s, il revient à 15% PV. Ce passif peut être déclenché manuellement avec un coût de mana si le Barde choisit de sacrifier sa ressource pour activer l'effet avant d'atteindre le seuil critique.

### 8.7 Synergie de groupe

Le Barde est l'amplificateur universel du groupe de l'Alliance. Sa Symphonie de Rive peut créer une fenêtre de 4s de domination totale si le groupe est coordonné. Il synergise particulièrement avec la Marchande de Guerre (les Jetons de Rive de Ballade du Héros enrichissent le flux économique de combat) et avec le Timonier (les auras de vitesse de Chant du Vent Arrière transforment sa mobilité défensive en pression offensive).

### 8.8 Lore

Les Bardes de Rue d'Auranthos jouent à l'Agora des Quatre Vents depuis des générations — à la fois divertissement et commerce d'information. Les marchands paient pour des chansons flatteuses, les rivaux pour des ballades humiliantes. En temps de guerre, l'Alliance a vite compris que les mêmes talents qui faisaient fuir les clients d'un concurrent pouvaient faire flancher une garde adverse. Les Bardes sont reconnus comme combattants sous la Charte des Corsaires depuis l'An 238 AO, après qu'un Barde nommé Tharven a seul fait déposer les armes d'une garnison impériale en entonnant une chanson sur la honte de leur dernier amiral.

---

## 9. Contrebandier — Hybride Furtif

### 9.1 Identité

**Rôle :** Hybride Furtif — Tromperie, pièges à retardement, déguisement
**Armes principales :** Pistolet de poing à silex (distance courte, bruyant mais dévastateur), Dague de ceinture (mêlée secondaire), Déguisements et accessoires (outils passifs)
**Couleur de classe :** Brun épices (#8B5E3C)

### 9.2 Style de combat

Le Contrebandier n'existe pas — ou plutôt, il existe sous le nom de quelqu'un d'autre. Sa première couche de défense est son identité : il peut adopter une apparence de garde, de marchand, de moine ervan suffisamment longtemps pour passer un checkpoint, placer un piège, ou extraire une information. En combat ouvert, il est moins à l'aise que la Lame des Docks mais bien plus polyvalent : ses pièges à retardement dévient le front ennemi, son pistolet punit les approches imprudentes, et ses nombreux faux-semblants maintiennent l'adversaire dans un état de doute perpétuel.

### 9.3 Arbre 1 — Art de la Tromperie

**Thème :** Déguisements, leurres, identités falsifiées

| Type | Nom | Effet |
|------|-----|-------|
| Active | Changement de Silhouette | Prend l'apparence d'un type d'unité ennemi courant (durée 30s ou jusqu'à attaque) — les ennemis ne l'attaquent pas automatiquement |
| Active | Leurre de Marchande | Pose un mannequin factice à sa position actuelle, se déplace 8m : les ennemis à distance ciblent le leurre pendant 4s |
| Active | Fausse Signature | Applique une marque ennemie sur un allié ciblé (l'ennemi pensera cibler "l'un des siens") pendant 6s — réduit l'aggro de l'allié |
| Passive | Costume sur Mesure | Le Changement de Silhouette dure 15s de plus avant détection automatique |
| Passive | Regard Pénétrant | Le Contrebandier détecte les déguisements adverses dans un rayon de 20m |

### 9.4 Arbre 2 — Réseau de Pièges

**Thème :** Pièges à retardement, zones de déni, déclencheurs logiques

| Type | Nom | Effet |
|------|-----|-------|
| Active | Mine de Cale | Pose une mine invisible (5s d'armement) : explose au contact, 200% ADPhy en zone 4m, applique Ralentissement 6s |
| Active | Piège Empoisonné | Cage mécanique dissimulée : immobilise 4s + applique poison progressif (2% PV/s pendant 12s) |
| Active | Fuse de Retard | Relie deux mines existantes : si l'une explose, l'autre explose 2s plus tard automatiquement (chaîne de détonations) |
| Passive | Artisan Prolifique | Peut déposer 3 pièges simultanément (normalement 1) |
| Passive | Détonateur Synchronisé | Peut déclencher manuellement n'importe quel piège posé dans un rayon de 30m |

### 9.5 Arbre 3 — Commerce Illicite

**Thème :** Marchandises de contrebande, effets chimiques, objets spéciaux

| Type | Nom | Effet |
|------|-----|-------|
| Active | Poudre de Larmes | Lance une fiole (portée 12m) : explosion de poudre aveuglante, -80% visibilité pendant 4s dans une zone 5m |
| Active | Huile de Tempête | Verse une nappe d'huile (zone 6m, 15s) : ennemis dessus ralentis 40%, zone feu si source d'ignition — inflige 50% ADPhy/s |
| Active | Philtre de Confusion | Projette un aérosol sur une cible (portée 8m) : la cible attaque une cible aléatoire adjacente pendant 5s |
| Passive | Réseau d'Approvisionnement | Coût en matériaux des actives réduit de 20% |
| Passive | Immunité Professionnelle | Immunisé aux effets de ses propres pièges et produits chimiques |

### 9.6 Compétences Signature

**Grand Jeu de Rive** *(Cooldown : 150s)*
Le Contrebandier révèle un "Grand Jeu" : pendant 20s, il peut alterner entre deux identités différentes (sa véritable apparence et un déguisement au choix). Ses pièges se déclenchent simultanément tous les 5s plutôt qu'au contact. Son Philtre de Confusion affecte tous les ennemis dans un rayon de 6m au lieu d'une cible. Son pistolet tire deux balles par activation. À la fin des 20s, tous les pièges restants explosent simultanément.

**Fuite Organisée** *(Cooldown : 60s)*
Activation instantanée : le Contrebandier disparaît dans un nuage de fumée, laisse un Leurre de Marchande renforcé (PV réels, 20% des siens), et se téléporte jusqu'à 20m dans une direction choisie. La téléportation traverse les obstacles légers (caisses, balustrades). Le Leurre résiste 5s avant d'être détruit. Si le Leurre est détruit, le cooldown de Fuite Organisée est réduit de 20s.

### 9.7 Synergie de groupe

Le Contrebandier est le spécialiste du chaos contrôlé. Sa Poudre de Larmes combinée avec le Barrage de Rive du Tireur d'Élite sur des ennemis aveuglés crée une zone de destruction sans réponse possible. Ses mines combinées avec les compétences de contrôle du Timonier créent des corridors mortels. Sa capacité à détourner l'aggro avec la Fausse Signature sauve régulièrement la Marchande de Guerre ou le Barde.

### 9.8 Lore

La contrebande est à l'Alliance de Rive ce que la philosophie est à la Fédération Ervan : une tradition culturelle profondément ancrée. L'Alliance taxe tout, donc tout ce qui échappe à la taxe est un acte de résistance autant qu'un business. Les Contrebandiers opèrent à la frontière poreuse entre l'illégal toléré et l'illégal actionnable — tant qu'ils restent discrets et qu'ils paient leurs "honoraires" aux douaniers concernés, l'Assemblée regarde ailleurs. En temps de guerre, leurs réseaux de passage et leurs caches d'équipement font d'eux des acteurs logistiques inestimables.

---

## 10. Boucanier — DPS Mêlée Brutal

### 10.1 Identité

**Rôle :** DPS Mêlée Brutal — Frénésie, rage maritime, gunplay à courte portée
**Armes principales :** Sabre lourd de bord (deux mains ou une main + pistolet), Pistolet de bord à silex (distance très courte, 1-2 tirs avant rechargement), Hache de bord (alternative pour bonus de stagger)
**Couleur de classe :** Rouge brique (#8B1A1A)

### 10.2 Style de combat

Le Boucanier est le marteau de l'Alliance — brutal, direct, excessif dans la bonne mesure. Il n'esquive pas les coups : il les ignore ou les rend avec intérêts. Sa mécanique centrale est la **Fureur de Rive** : une jauge qui monte à chaque coup reçu et à chaque dégât infligé, qui augmente ses dégâts mais réduit sa défense à mesure qu'elle se remplit. Dans l'état de Fureur maximum, il devient un cyclone de violence incontrôlable — vulnérable mais monstrueux en dégâts. L'alternance sabre/pistolet est unique : le pistolet est utilisé à bout portant (moins de 3m) comme une frappe supplémentaire dévastatrice entre deux coups de sabre, créant un rythme de combat irrégulier et difficile à lire pour l'adversaire.

### 10.3 Arbre 1 — Fureur du Large

**Thème :** Gestion de la jauge Fureur, amplification des dégâts, berserk

| Type | Nom | Effet |
|------|-----|-------|
| Active | Déferlante | Frappe puissante chargée 0,8s : 200% ADPhy + génère 20 Fureur, bonus dégâts +5% par 10 Fureur actuels |
| Active | Tempête de Lame | Rotation de 4 frappes larges en 2s : 90% ADPhy chacune, génère 10 Fureur par frappe |
| Active | Déchaînement | Active le mode Berserk si Fureur > 70 : pendant 6s, vitesse d'attaque +50%, dégâts +30%, défense -40% |
| Passive | Rage Maritime | Chaque coup reçu génère 8 Fureur au lieu de 5 |
| Passive | Sang Chaud | La Fureur se décharge 30% moins vite entre les combats |

### 10.4 Arbre 2 — Gunplay des Docks

**Thème :** Pistolat à courte portée, technique mixte sabre+pistolet

| Type | Nom | Effet |
|------|-----|-------|
| Active | Tir de Hanche | Tir de pistolet en combat au corps à corps (portée ≤ 3m) : 250% ADPhy, applique Étourdissement 0,5s, nécessite rechargement de 4s |
| Active | Crosse au Menton | Frappe avec la crosse du pistolet (pas de munition utilisée) : 110% ADPhy + Stun 1s |
| Active | Double Offensive | Enchaînement sabre + pistolet en 1,5s : 130% ADPhy (sabre) + 200% ADPhy (pistolet) si la cible est en mêlée |
| Passive | Rechargement Rapide | Le rechargement du pistolet passe de 4s à 2,5s |
| Passive | Poudrière Vivante | Tir de Hanche génère 15 Fureur supplémentaires si utilisé pendant Déchaînement |

### 10.5 Arbre 3 — Vieil Os de Mer

**Thème :** Résistance au combat prolongé, survie, intimidation

| Type | Nom | Effet |
|------|-----|-------|
| Active | Peau de Calfat | Réduit tous les dégâts reçus de 25% pendant 6s (cooldown 20s) |
| Active | Cri de Bordée | Cri d'intimidation : ennemis dans un rayon de 10m ont -15% dégâts pendant 5s, certains ennemis faibles peuvent fuir |
| Active | Deuxième Souffle | Si le Boucanier tombe en dessous de 20% PV, se soigne de 25% PV une fois (déclenchement manuel ou automatique) |
| Passive | Constitution Maritime | +12% PV maximum, résistance aux effets de poison de 20% |
| Passive | Endurance de Boucanier | Les effets de ralentissement ont une efficacité réduite de 30% sur le Boucanier |

### 10.6 Compétences Signature

**Raz-de-Marée** *(Cooldown : 90s — nécessite Fureur ≥ 60)*
Le Boucanier libère toute sa Fureur accumulée en une attaque dévastrice : charge 1,5s puis frappe en arc de 270° autour de lui, infligeant (Fureur actuelle × 3)% ADPhy à tous les ennemis touchés. Une Fureur de 100 inflige donc 300% ADPhy en zone. Après l'activation, la Fureur tombe à 0 et le Boucanier récupère 10% de PV. Sur le plan sonore, le combat de plusieurs mètres alentour est secoué par l'impact. La zone effective est de 5m de rayon.

**Fils du Large** *(Cooldown : 180s)*
L'état de transe de combat ultime. Le Boucanier brise ses propres limites pendant 15s : immunité aux effets de stun et knockback, la jauge de Fureur ne peut plus descendre en dessous de 50%, chaque frappe inflige un Saignement automatique cumulable, et le pistolet peut être tiré sans temps de rechargement (balles magiquement rechargées). À la fin des 15s, le Boucanier tombe à 1 PV et est Étourdi 2s — la cascade retombée, le corps réclame son dû.

### 10.7 Synergie de groupe

Le Boucanier est la pointe de lance : il brise les formations ennemies, force les réorganisations, et tire parti des ouvertures créées par les autres classes. Son Cri de Bordée combiné avec l'Ordre d'Abordage du Timonier crée une fenêtre de -15% + -15% dégâts ennemis simultanés qui peut sauver un groupe en difficulté. Son Raz-de-Marée est particulièrement dévastateur contre des ennemis regroupés par le Timonier ou piégés par les zones de contrôle du Contrebandier.

### 10.8 Lore

Les Boucaniers descendent des équipages des navires corsaires qui opéraient dans les Archipels de Brume avant la Charte des Corsaires. Sans le cadre légal qui institutionnalisait les corsaires "propres", ils pillaient librement et vivaient vite. La culture de l'excès, de la violence directe et du "je prends ce qui est là" les distingue des corsaires licenciés. Dans l'Alliance moderne, ils sont tolérés — voire recrutés en masse — en temps de guerre, puis "gérés" en temps de paix. Les tavernes de Selmara sont remplies de leurs histoires et de leurs querelles.

---

## 11. Synergies de Faction

### 11.1 Compositions standards

**Composition Commerce (PvE économique)**
Marchande de Guerre + Barde de Rue + 2x Corsaires + Tireur d'Élite
Maximise la génération de Jetons de Rive, les buffs de durée prolongée, et la mobilité pour des runs efficaces de donjons commerciaux. Le Barde maintient les auras, la Marchande gère les ressources, les Corsaires génèrent les kills marqués.

**Composition Abordage (combat naval, siège)**
Timonier de Combat + 2x Boucaniers + Lame des Docks + Marchande de Guerre
Axée sur la percée de ligne : le Timonier ouvre, les Boucaniers déferlent, la Lame élimine les cibles prioritaires pendant que la Marchande maintient le groupe en vie. Efficace pour les prises de forts et les abordages.

**Composition Ombre (reconnaissance, sabotage)**
Contrebandier + Lame des Docks + Barde de Rue + Tireur d'Élite + Duelliste
Composition furtive optimale : aucun membre ne génère de présence forte. Le Contrebandier prépare le terrain, la Lame exécute les cibles prioritaires, le Tireur couvre les retraites, le Barde synchronise les timings d'engagement, le Duelliste sécurise les désengagements.

**Composition Brutalité (RvR frontal)**
Timonier de Combat + 2x Boucaniers + Duelliste + Corsaire + Marchande de Guerre
Six joueurs, front classique. Le Timonier tient la ligne, les Boucaniers pressent les flancs, le Duelliste neutralise les duelistes adverses, le Corsaire harcèle les arrières, la Marchande maintient le tout en vie.

### 11.2 Bonus de faction passif — Liberté des Mers

Tous les personnages de l'Alliance de Rive bénéficient des bonus passifs suivants, indépendants de leur classe :

- **Navigation Naturelle** : Déplacement en zone maritime (bateaux, quais, zones côtières) +10%
- **Réseau de Renseignement** : Révèle la faction des joueurs anonymes dans un rayon de 30m
- **Débrouillard** : Peut utiliser des objets de consommation d'autres factions avec une efficacité réduite de 25% (au lieu d'être bloqué)
- **Marchandage** : Prix d'achat chez les marchands neutres réduit de 5%

### 11.3 Rang et progression sociale

L'avancement dans les rangs de l'Alliance — Matelot, Corsaire, Capitaine, Amiral, Grand Marchand, Consul de Rive, Archonte de Mer — est une progression de réputation partagée entre toutes les classes. Chaque classe contribue différemment à la réputation de faction :

| Classe | Principal vecteur de réputation |
|--------|--------------------------------|
| Corsaire | Victoires navales et abordages |
| Duelliste | Duels officiels gagnés |
| Marchande de Guerre | Valeur commerciale des expéditions soutenues |
| Lame des Docks | Contrats accomplis (anonymes mais comptabilisés) |
| Timonier de Combat | Fortifications défendues, alliés protégés |
| Tireur d'Élite | Bounties capturées, défenses de convois |
| Barde de Rue | Renommée musicale, chansons propagées en faction |
| Contrebandier | Marchandises acheminées, espionnage réussi |
| Boucanier | Ennemis vaincus, sièges brisés |

---

## 12. Schémas TOML complets

```toml
# AL-Character-Alliance — Schémas TOML v1.0
# Faction : Alliance de Rive
# Monde : Véranthas, An 247 AO

[faction.alliance_de_rive]
id = "alliance_de_rive"
color_primary = "#F4C430"
color_secondary = "#FF8C00"
capital = "auranthos"
social_identity = "homme_libre"
rank_progression = [
  "matelot",
  "corsaire_rang",
  "capitaine",
  "amiral",
  "grand_marchand",
  "consul_de_rive",
  "archonte_de_mer"
]
passive_bonuses = [
  "navigation_naturelle",
  "reseau_renseignement",
  "debrouillard",
  "marchandage"
]

# ─────────────────────────────────────────
# 1. CORSAIRE
# ─────────────────────────────────────────
[class.corsaire]
id = "corsaire"
faction = "alliance_de_rive"
role = "dps_melee_agile"
weapon_types = ["sabre", "crochet", "dague"]
skill_trees = ["lame_des_flots", "mobilite_marine", "tactiques_de_bord"]
base_stats = { str = 20, agi = 28, int = 12, con = 15 }
signature_skills = ["pavillon_noir", "arraisonnement"]
group_synergy = "breche_partagee"
social_role = "corsaire_licence"

[class.corsaire.skill_tree.lame_des_flots]
theme = "degats_sabre_saignements"
actives = ["taille_oblique", "volee_de_rive", "estocade_finale"]
passives = ["tranchant_de_sel", "acier_huile"]

[class.corsaire.skill_tree.mobilite_marine]
theme = "deplacement_esquive_repositionnement"
actives = ["abordage", "roulade_de_pont", "saut_de_beaupre"]
passives = ["pieds_marins", "insaisissable"]

[class.corsaire.skill_tree.tactiques_de_bord]
theme = "environnement_double_arme_techniques"
actives = ["crochet_tournant", "double_degaine", "envoi_de_grenadine"]
passives = ["maitre_equipage", "reflexes_de_rigging"]

# ─────────────────────────────────────────
# 2. DUELLISTE
# ─────────────────────────────────────────
[class.duelliste]
id = "duelliste"
faction = "alliance_de_rive"
role = "dps_precision"
weapon_types = ["rapiere", "main_gauche", "epee_courte"]
skill_trees = ["ecole_rapiere", "art_parade", "finesse_marchande"]
base_stats = { str = 16, agi = 24, int = 18, con = 14 }
signature_skills = ["sept_passes_de_bareth", "jugement_de_lame"]
group_synergy = "lecture_commune"
social_role = "breteur_juriste"

[class.duelliste.skill_tree.ecole_rapiere]
theme = "estocades_armure_reduction_combo"
actives = ["estocade_directrice", "quarte_haute", "pointe_mortelle"]
passives = ["angle_parfait", "fil_du_rasoir"]

[class.duelliste.skill_tree.art_parade]
theme = "parades_actives_precision_contre_attaque"
actives = ["parade_de_cercle", "deviation_croisee", "contre_tempo"]
passives = ["economie_de_lame", "instinct_du_bretteur"]

[class.duelliste.skill_tree.finesse_marchande]
theme = "feinte_duel_desarmement"
actives = ["feinte_commerciale", "defi_en_duel", "desarmement_formel"]
passives = ["sang_froid_de_comptoir", "reputation_de_lame"]

# ─────────────────────────────────────────
# 3. MARCHANDE DE GUERRE
# ─────────────────────────────────────────
[class.marchande_de_guerre]
id = "marchande_de_guerre"
faction = "alliance_de_rive"
role = "support_economique"
weapon_types = ["arbalete_legere", "fouet_cargaison", "parchemins_tactiques"]
skill_trees = ["comptabilite_du_sang", "logistique_maritime", "tactiques_commerciales"]
base_stats = { str = 12, agi = 18, int = 26, con = 16 }
signature_skills = ["contrat_de_sang", "economie_de_guerre"]
group_synergy = "flux_de_jetons"
social_role = "investisseuse_militaire"

[class.marchande_de_guerre.skill_tree.comptabilite_du_sang]
theme = "generation_ressources_kills_alliés"
actives = ["inventaire_de_guerre", "dividende_de_victoire", "liquidation"]
passives = ["marge_brute", "economie_echelle"]

[class.marchande_de_guerre.skill_tree.logistique_maritime]
theme = "ravitaillement_consommables_renforcement"
actives = ["caisse_de_ravitaillement", "potion_qualite_superieure", "chargement_urgent"]
passives = ["entrepot_mobile", "qualite_garantie"]

[class.marchande_de_guerre.skill_tree.tactiques_commerciales]
theme = "manipulation_aggro_buffs_commandement"
actives = ["offre_non_negociable", "clause_penale", "accord_de_rive"]
passives = ["autorite_de_guilde", "renseignement_commercial"]

# ─────────────────────────────────────────
# 4. LAME DES DOCKS
# ─────────────────────────────────────────
[class.lame_des_docks]
id = "lame_des_docks"
faction = "alliance_de_rive"
role = "stealth_burst"
weapon_types = ["couteau_lancer", "dague_courte", "aiguille_empoisonnee"]
skill_trees = ["art_du_poison", "ombre_des_quais", "contrats_des_docks"]
base_stats = { str = 14, agi = 30, int = 16, con = 12 }
signature_skills = ["execution_des_docks", "toile_de_nuit"]
group_synergy = "cible_prioritaire"
social_role = "contractuelle_discrete"

[class.lame_des_docks.skill_tree.art_du_poison]
theme = "toxines_effets_duree_debuffs"
actives = ["onguent_urticant", "extrait_de_meduse", "dose_letale"]
passives = ["alchimie_des_bas_fonds", "accumulation_toxique"]

[class.lame_des_docks.skill_tree.ombre_des_quais]
theme = "furtivite_dissimulation_mobilite"
actives = ["fond_dans_lombre", "pas_silencieux", "substitution_dombre"]
passives = ["peau_de_nuit", "instinct_predateur"]

[class.lame_des_docks.skill_tree.contrats_des_docks]
theme = "execution_cible_burst_assassinat"
actives = ["contrat_ouvert", "frappe_du_couteau", "jugulaire"]
passives = ["efficacite_contractuelle", "specialisation_anatomique"]

# ─────────────────────────────────────────
# 5. TIMONIER DE COMBAT
# ─────────────────────────────────────────
[class.timonier_de_combat]
id = "timonier_de_combat"
faction = "alliance_de_rive"
role = "tank_manoeuver"
weapon_types = ["bouclier_naval", "gaffet_de_bord", "trident_marin"]
skill_trees = ["muraille_de_proue", "manoeuvre_navale", "commandement_de_pont"]
base_stats = { str = 22, agi = 14, int = 10, con = 30 }
signature_skills = ["manoeuvre_du_detroit", "ancre_de_rive"]
group_synergy = "geographie_du_combat"
social_role = "officier_tactique"

[class.timonier_de_combat.skill_tree.muraille_de_proue]
theme = "resistance_absorption_protection"
actives = ["posture_de_proue", "bouclier_de_mer", "couverture_maritime"]
passives = ["coque_renforcee", "lecture_des_vagues"]

[class.timonier_de_combat.skill_tree.manoeuvre_navale]
theme = "positionnement_deplacement_force_zone"
actives = ["charge_de_bordee", "virage_brusque", "encerclement"]
passives = ["cap_maintenu", "trajectoire_predictive"]

[class.timonier_de_combat.skill_tree.commandement_de_pont]
theme = "leadership_moral_auras_tactiques"
actives = ["ordre_dabordage", "tenir_la_ligne", "point_de_ralliement"]
passives = ["presence_du_capitaine", "repartition_de_barre"]

# ─────────────────────────────────────────
# 6. TIREUR D'ÉLITE
# ─────────────────────────────────────────
[class.tireur_delite]
id = "tireur_delite"
faction = "alliance_de_rive"
role = "dps_distance"
weapon_types = ["arc_composite", "arbalete_de_pont", "pieges_mecaniques"]
skill_trees = ["tir_de_pont", "ingenierie_de_pont", "chasseur_de_bounty"]
base_stats = { str = 15, agi = 26, int = 20, con = 13 }
signature_skills = ["barrage_de_rive", "embuscade_de_guetteur"]
group_synergy = "synergie_de_controle"
social_role = "chasseur_de_primes"

[class.tireur_delite.skill_tree.tir_de_pont]
theme = "precision_distance_tirs_charges"
actives = ["tir_tendu", "salve_de_rive", "tir_perforant"]
passives = ["precision_marine", "calme_du_tireur"]

[class.tireur_delite.skill_tree.ingenierie_de_pont]
theme = "pieges_zones_controle_preparation"
actives = ["piege_a_cable", "nasse_de_rive", "bombe_fumigene"]
passives = ["ingenieur_de_fortune", "declencheur_sensible"]

[class.tireur_delite.skill_tree.chasseur_de_bounty]
theme = "ciblage_marquage_cibles_prioritaires"
actives = ["marque_de_chasse", "tir_a_la_tete", "fleche_tracante"]
passives = ["instinct_de_chasseur", "memoire_de_trajectoire"]

# ─────────────────────────────────────────
# 7. BARDE DE RUE
# ─────────────────────────────────────────
[class.barde_de_rue]
id = "barde_de_rue"
faction = "alliance_de_rive"
role = "support_desorganisation"
weapon_types = ["mandoline_de_combat", "couteau_leger", "luth_de_guerre"]
skill_trees = ["chants_de_mer", "dissonances_marchandes", "histoires_de_corsaires"]
base_stats = { str = 10, agi = 20, int = 28, con = 14 }
signature_skills = ["symphonie_de_rive", "derniere_chanson"]
group_synergy = "amplificateur_universel"
social_role = "artiste_militaire"

[class.barde_de_rue.skill_tree.chants_de_mer]
theme = "auras_benefiques_synchronisation"
actives = ["chant_du_vent_arriere", "melodie_du_cap", "choeur_de_victoire"]
passives = ["portee_acoustique", "resonance_prolongee"]

[class.barde_de_rue.skill_tree.dissonances_marchandes]
theme = "effets_negatifs_disruption_cc"
actives = ["accord_dissonant", "cacophonie_de_port", "glas_de_rive"]
passives = ["vibration_fracassante", "oreille_absolue"]

[class.barde_de_rue.skill_tree.histoires_de_corsaires]
theme = "buffs_narratifs_inspiration_maledictions"
actives = ["ballade_du_heros", "chanson_de_honte", "recit_de_nuit"]
passives = ["rumeur_propagee", "memoire_collective"]

# ─────────────────────────────────────────
# 8. CONTREBANDIER
# ─────────────────────────────────────────
[class.contrebandier]
id = "contrebandier"
faction = "alliance_de_rive"
role = "hybride_furtif"
weapon_types = ["pistolet_de_poing", "dague_ceinture", "deguisements"]
skill_trees = ["art_de_la_tromperie", "reseau_de_pieges", "commerce_illicite"]
base_stats = { str = 14, agi = 24, int = 22, con = 12 }
signature_skills = ["grand_jeu_de_rive", "fuite_organisee"]
group_synergy = "chaos_controle"
social_role = "passeur_reseau"

[class.contrebandier.skill_tree.art_de_la_tromperie]
theme = "deguisements_leurres_identites"
actives = ["changement_de_silhouette", "leurre_de_marchande", "fausse_signature"]
passives = ["costume_sur_mesure", "regard_penetrant"]

[class.contrebandier.skill_tree.reseau_de_pieges]
theme = "mines_zones_declencheurs_chaines"
actives = ["mine_de_cale", "piege_empoisonne", "fuse_de_retard"]
passives = ["artisan_prolifique", "detonateur_synchronise"]

[class.contrebandier.skill_tree.commerce_illicite]
theme = "chimiques_effets_zones_controle"
actives = ["poudre_de_larmes", "huile_de_tempete", "philtre_de_confusion"]
passives = ["reseau_dapprovisionnement", "immunite_professionnelle"]

# ─────────────────────────────────────────
# 9. BOUCANIER
# ─────────────────────────────────────────
[class.boucanier]
id = "boucanier"
faction = "alliance_de_rive"
role = "dps_melee_brutal"
weapon_types = ["sabre_lourd", "pistolet_de_bord", "hache_de_bord"]
skill_trees = ["fureur_du_large", "gunplay_des_docks", "vieil_os_de_mer"]
base_stats = { str = 28, agi = 16, int = 8, con = 24 }
signature_skills = ["raz_de_maree", "fils_du_large"]
group_synergy = "pointe_de_lance"
mechanic_unique = "fureur_de_rive"
social_role = "ancien_pirate"

[class.boucanier.skill_tree.fureur_du_large]
theme = "jauge_fureur_degats_berserk"
actives = ["deferlante", "tempete_de_lame", "dechainement"]
passives = ["rage_maritime", "sang_chaud"]

[class.boucanier.skill_tree.gunplay_des_docks]
theme = "pistolat_corps_a_corps_mixte"
actives = ["tir_de_hanche", "crosse_au_menton", "double_offensive"]
passives = ["rechargement_rapide", "poudre_de_cale"]

[class.boucanier.skill_tree.vieil_os_de_mer]
theme = "resistance_combat_prolonge_survie"
actives = ["peau_de_calfat", "cri_de_bordee", "deuxieme_souffle"]
passives = ["constitution_maritime", "endurance_de_boucanier"]

# ─────────────────────────────────────────
# MECANIQUES PARTAGEES DE FACTION
# ─────────────────────────────────────────
[faction_mechanics.alliance_de_rive]
jetons_de_rive = { max = 12, generator = "marchande_de_guerre", consumer = "liquidation" }
fureur_de_rive = { max = 100, class_owner = "boucanier", decay_rate = 5.0 }
precision_stacks = { max = 5, class_owner = "duelliste", source = "parades" }
breche_stacks = { max = 5, class_owner = "corsaire", source = "frappe_legere" }
auras_actives_limit = 2

[faction_mechanics.alliance_de_rive.passive_faction_bonuses]
navigation_naturelle = { movement_bonus = 0.10, zones = ["maritime", "quais", "cotes"] }
reseau_renseignement = { reveal_faction = true, range = 30 }
debrouillard = { cross_faction_items = true, efficacy_penalty = 0.25 }
marchandage = { vendor_price_reduction = 0.05 }
```

---

*Document canonique — Alliance de Rive, Classes v1.0*
*Véranthas, An 247 AO — Libre, Lié par le Sel*
