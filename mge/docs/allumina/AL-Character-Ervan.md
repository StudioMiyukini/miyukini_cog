<!-- @id: AL-Character-Ervan @do: reference @role: game-designer @layer: 3 @human: miyuk -->

# Allumina — Classes de la Fédération Ervan

**Statut :** Référence canonique v1.0
**Date :** 2026-02-28
**Scope :** 9 classes jouables de la faction Fédération Ervan — rôles, arbres de compétences, schémas TOML

---

## Table des matières

1. [Vue d'ensemble — La Fédération et ses combattants](#1-vue-densemble--la-fédération-et-ses-combattants)
2. [Gaïathar — Mage des Forêts](#2-gaïathar--mage-des-forêts-dps-magique)
3. [Ferventhar — Druide](#3-ferventhar--druide-supportheal)
4. [Velsorath — Ranger Arcane](#4-velsorath--ranger-arcane-dps-distance-hybride)
5. [Nytharven — Invocateur](#5-nytharven--invocateur-support-via-créatures)
6. [Leyanthar — Gardien des Ley Lines](#6-leyanthar--gardien-des-ley-lines-tank-magique)
7. [Solthareth — Sorcière de Combat](#7-solthareth--sorcière-de-combat-dps-nukes)
8. [Velthamur — Chamane](#8-velthamur--chamane-supportheal-ancestral)
9. [Rhatharven — Tisserand de Sorts](#9-rhatharven--tisserand-de-sorts-hybride-synergies)
10. [Umbrantal — Veilleur](#10-umbrantal--veilleur-stealth-magique)
11. [Synergies de faction](#11-synergies-de-faction)
12. [Schémas TOML complets](#12-schémas-toml-complets)

---

## 1. Vue d'ensemble — La Fédération et ses combattants

La Fédération Ervan n'a pas d'armée au sens impérial du terme. Elle a des Cercles — des associations de praticiens organisés par spécialité, gouvernés par des conseils de sages régionaux qui délèguent l'autorité selon la compétence démontrée, jamais selon la naissance ou la richesse. Ses combattants ne sont pas des soldats formatés à la discipline de caserne — ce sont des praticiens, des chercheurs, des gardiens de forêts millénaires qui ont choisi d'appliquer leur savoir à la protection du vivant.

La doctrine de combat de la Fédération peut se résumer ainsi : **comprendre avant d'agir, guérir avant de détruire, mais frapper vite et juste quand aucune autre voie ne reste ouverte.**

L'identité sociale de départ pour tous les joueurs Ervan est celle d'**Habitant** (*Anth-Ferrath* en proto-ervan) — membre de la communauté vivante de Véranthas, lié à la terre, aux forêts et aux Ley Lines. Cette identité n'est pas une restriction mais un point d'ancrage : la Fédération considère que tout être conscient est d'abord un habitant du monde avant d'être un citoyen d'une faction.

Les combattants Ervan partagent une caractéristique fondamentale : leur puissance est **contextuelle**. Ils sont plus forts dans les forêts, près des Nexus, le long des Ley Lines actives, et lors des phases lunaires ascendantes. En zone urbaine ou dans les zones corrompues par Garum, certaines de leurs compétences s'affaiblissent — une limite de design voulue, non un bug.

### Statistiques primaires (proto-ervan)

| Stat proto-ervan | Nom commun | Rôle mécanique |
|-----------------|------------|----------------|
| **Ferrath-Anth** | Force | Dégâts physiques, capacité de port, résistance physique brute |
| **Velthar-Sorath** | Agilité | Vitesse d'attaque, esquive, vitesse de déplacement |
| **Gaïathar-Nexis** | Intelligence | Puissance magique, régénération de mana, portée des sorts |
| **Rhathar-Veines** | Constitution | Points de vie, résistance aux états, régénération passive |

### Tableau des classes

| # | Classe | Nom proto-ervan | Rôle | Armes | Style |
|---|--------|----------------|------|-------|-------|
| 1 | Mage des Forêts | Gaïathar | DPS Magique | Bâton, Baguette, Focus ervan | Élémentaire naturel, zones, burst |
| 2 | Druide | Ferventhar | Support/Heal | Sceptre de lierre, Focale gaïenne | Soins, transformations, lien Gaïa |
| 3 | Ranger Arcane | Velsorath | DPS Distance Hybride | Arc runique, Dague de rune | Magie en mouvement, runes mobiles |
| 4 | Invocateur | Nytharven | Support via créatures | Baguette de liaison, Phylactère | Loups, esprits, buff/debuff par procuration |
| 5 | Gardien des Ley | Leyanthar | Tank Magique | Bouclier de cristal, Marteau ley | Absorption, boucliers, contre-flux |
| 6 | Sorcière de Combat | Solthareth | DPS Nukes/Debuffs | Baguette tordue, Grimoire lié | Malédictions, explosions rapides, chaos |
| 7 | Chamane | Velthamur | Support/Heal Ancestral | Hochet d'os, Houlette de brume | Esprits, résurrection, vision du passé |
| 8 | Tisserand de Sorts | Rhatharven | Hybride Synergies | Gants de trame, Orbe tissé | Enchantements combinés, réactivité |
| 9 | Veilleur | Umbrantal | Stealth Magique | Stylet runique, Cape d'effacement | Espionnage, infiltration, sabotage |

---

## 2. Gaïathar — Mage des Forêts (DPS Magique)

### 2.1 Identité

**Rôle :** DPS Magique — élémentaire naturel, zone et burst
**Armes principales :** Bâton ancien de feu de forêt, Baguette de givre, Focus ervan (orbe de pierre vivante)
**Couleur de classe :** Vert forêt profond (#1B5E20)
**Statistique dominante :** Gaïathar-Nexis (Intelligence)

### 2.2 Style de combat

Le Mage des Forêts ne lance pas des sorts comme un technicien calcule des trajectoires — il *exprime* les humeurs de la forêt à travers lui-même. Le feu qu'il invoque n'est pas un feu civil : c'est la foudre d'un incendie contrôlé qui purge, la fièvre que Gaïa met dans une feuille morte. Son givre porte l'immobilité des hivers centenaires des arbres boréaux. Sa foudre de Ley suit les nervures invisibles de l'énergie tellurique sous ses pieds, bifurquant d'ennemi en ennemi comme une crue dans un réseau de ravines.

Sa mécanique de cœur est le **Cycle des Saisons** : il alterne entre trois états élémentaires (Écorce-Feu, Givre-Racine, Foudre-Ley) à chaque activation d'un élément différent. Chaque état modifie les propriétés de ses sorts suivants et génère des *Résonances* accumulables. À 5 Résonances, il entre en **Confluence Gaïenne** pendant 8s : ses sorts ne coûtent plus de mana et chaque ennemi touché subit 20% de dégâts supplémentaires de toutes les sources magiques alliées dans les 4s suivantes.

### 2.3 Arbre 1 — Feu Naturel (*Ferrath-Sorath*)

**Thème :** Dégâts de feu directs et sur zone, brûlures progressives, purge

| Type | Nom | Effet |
|------|-----|-------|
| Active | Embrasement des Sous-Bois | Projectile de feu qui explose à l'impact, 200% ADMag en zone 4m, applique Brûlure (8% ADMag/s, 5s) |
| Active | Colère de la Canopée | Pluie de flamèches sur une zone de 6m pendant 3s, 60% ADMag par seconde, déclenche Écorce-Feu |
| Active | Nova Pyrrhique | Explosion de feu centrée sur le lanceur, rayon 8m, 300% ADMag, repousse de 5m, consomme 3 Résonances |
| Passive | Résine Ardente | Les ennemis brûlants prennent 12% de dégâts de feu supplémentaires de toutes sources |
| Passive | Fièvre des Cendres | Si un ennemi meurt sous Brûlure, une explosion de cendres inflige 80% ADMag aux ennemis adjacents dans 3m |

### 2.4 Arbre 2 — Tempête des Ley (*Gaïathar-Foudre*)

**Thème :** Foudre qui se propage, chaînes d'éclair, décharge des Ley Lines

| Type | Nom | Effet |
|------|-----|-------|
| Active | Arc de Ley | Éclair ciblé, 220% ADMag, se propage à 2 ennemis supplémentaires dans 8m avec 60% de dégâts décroissants |
| Active | Grille Tellurique | Trace une ligne de foudre de 12m de long, 150% ADMag instantané + choc 1s à tous les ennemis sur la ligne |
| Active | Appel du Nexus | Invoque un point de Ley temporaire pendant 10s : toutes les compétences de foudre lancées à moins de 15m ont leur portée +50% et leurs dégâts +30% |
| Passive | Conducteur Naturel | Les ennemis mouillés (pluie, sorts d'eau alliés) reçoivent 40% de dégâts supplémentaires des éclairs |
| Passive | Surcharge Ley | À chaque 3e utilisation d'une compétence de foudre consécutive, la prochaine inflige 50% de dégâts supplémentaires |

### 2.5 Arbre 3 — Métamorphose Élémentaire (*Nexis-Rhathar*)

**Thème :** Givre, contrôle, gel, synergies entre éléments

| Type | Nom | Effet |
|------|-----|-------|
| Active | Souffle d'Hiver Éternel | Cône de givre de 6m, 160% ADMag, ralentit 40% pendant 4s, applique Givre-Racine |
| Active | Prison de Givre | Encapsule une cible en glace gaïenne : immobilisation 2,5s, invulnérabilité aux dégâts mais le sort suivant du lanceur brise la prison et inflige +80% de dégâts |
| Active | Fracture de Grêle | Fait éclater le givre sur un ennemi gelé ou ralenti : 280% ADMag + éclats qui infligent 100% ADMag à tous les ennemis dans 5m |
| Passive | Convergence des Éléments | Si le Mage a utilisé les 3 éléments dans les 15 dernières secondes, son prochain sort de n'importe quel élément inflige +25% de dégâts |
| Passive | Peau de Pierre Froide | Après activation de Givre-Racine, le Mage gagne 15% de résistance magique pendant 6s |

### 2.6 Compétences Signature

**Éveil de la Forêt Ancienne** *(Cooldown : 120s)*
Le Mage des Forêts canalize pendant 1,5s (interruptible), puis libère simultanément les trois éléments dans un rayon de 12m autour de lui. Phase 1 (instantanée) : onde de givre qui ralentit 50% tous les ennemis dans la zone pendant 4s. Phase 2 (0,5s après) : grille de foudre qui traverse la zone, 280% ADMag à chaque ennemi touché. Phase 3 (1s après) : torrent de feu qui embrase le sol de la zone pendant 6s, infligeant 80% ADMag/s. Les trois éléments s'activent dans cet ordre immuable et génèrent chacun 2 Résonances. Si le Mage était en **Confluence Gaïenne** au moment de l'activation, la durée de la phase de feu passe à 12s.

**Voix du Nexus Vide** *(Cooldown : 90s — actif uniquement en Confluence Gaïenne)*
Exige d'être en état de Confluence Gaïenne pour s'activer. Le Mage projette son essence dans le réseau de Ley Lines environnant pendant 6s : pendant cette durée il est en **Forme de Flux** — intangible (immunité aux dégâts physiques), se déplace 80% plus vite, et chaque mètre parcouru laisse une traîne de foudre qui inflige 30% ADMag/s aux ennemis qui la traversent. Au terme des 6s, une explosion centrée sur sa position inflige 400% ADMag en zone 10m et réinitialise le compteur de Résonances à 0.

### 2.7 Synergie de groupe

Le Mage des Forêts apporte le **Marquage Élémentaire** : quand un ennemi cumule deux états élémentaires distincts (ex : Brûlure + Givre-Racine), tous les alliés qui l'attaquent voient leurs dégâts augmenter de 10% pendant la durée du second état. Ce bonus est indépendant de la classe des alliés. Synergie naturelle avec le Velsorath (flèches runiques qui appliquent des états) et le Solthareth (malédictions qui amplifient les états).

### 2.8 Lore

Dans la hiérarchie des Cercles Ervans, les Gaïathars occupent une position ambiguë : vénérés pour leur maîtrise de l'expression élémentaire de Gaïa, craints pour la destruction qu'ils peuvent déchaîner, marginalement tolérés par les druides plus contemplatifs qui estiment qu'ils confondent puissance avec compréhension. La tradition des Gaïathars remonte à l'Ère de la Pierre Montante, quand les premiers mages ervan apprirent que les Ley Lines ne transportent pas un élément unique mais une *harmonie* d'énergies — et que les rompre en faveur d'un seul élément produit une force brute dont la violence intérieure consume le praticien en quelques décennies si mal maîtrisée.

---

## 3. Ferventhar — Druide (Support/Heal)

### 3.1 Identité

**Rôle :** Support principal / Heal — soins, transformations animales limitées, lien avec Gaïa
**Armes principales :** Sceptre de lierre vivant, Focale gaïenne (gemme de nexus enchâssée), Houlette de croissance
**Couleur de classe :** Vert tendre (#4CAF50)
**Statistique dominante :** Gaïathar-Nexis (Intelligence) + Rhathar-Veines (Constitution)

### 3.2 Style de combat

Le Ferventhar n'est pas un soignant passif qui regarde les autres se battre depuis l'arrière-ligne. Il est la **présence vivante de Gaïa dans le groupe** — il voit simultanément les flux de vie de ses alliés, l'état de corruption du sol sous ses pieds, et les Ley Lines que ses ennemis piétinent sans le savoir. Il soigne en renforçant le lien entre ses alliés et Gaïa, amplifiant leur régénération naturelle plutôt qu'en injectant de la magie externalisée. Ses transformations animales sont limitées et coûteuses en énergie, mais offrent des capacités uniques pour des situations spécifiques : la forme d'Ours Gaïen absorbe les dégâts à sa place pendant quelques secondes, la forme de Cerf Vif lui permet une mobilité de support inégalée.

Sa mécanique de cœur est le **Lien Gaïen** : il peut maintenir un lien actif avec jusqu'à 5 alliés simultanément. Ces alliés liés partagent 15% de la régénération de vie que le Ferventhar génère lui-même, voient leurs soins reçus améliorés de 8%, et reçoivent une alerte visuelle 1,5s avant d'être la cible d'une compétence ennemie critique.

### 3.3 Arbre 1 — Racines Guérisseuses (*Rhathar-Gaïa*)

**Thème :** Soins directs, régénérations, purification de corruption

| Type | Nom | Effet |
|------|-----|-------|
| Active | Communion des Racines | Soin ciblé instantané : restaure 180% de Gaïathar-Nexis en PV, génère 1 charge de Lien Gaïen si la cible est liée |
| Active | Réseau de Sève | Aura de zone (rayon 10m, 8s) : tous les alliés dans la zone régénèrent 25% de Gaïathar-Nexis en PV par seconde |
| Active | Purge Gaïenne | Supprime jusqu'à 3 effets négatifs (poison, brûlure, malédiction) sur la cible, chaque purge restaure 60% de Gaïathar-Nexis en PV |
| Passive | Sève de Printemps | Les soins directs du Ferventhar ont 20% de chances de déclencher un soin supplémentaire à 50% de la valeur sur l'allié lié avec le moins de PV |
| Passive | Terre Fertile | Les alliés sur un sol non-corrompu régénèrent 2% de PV/s supplémentaires tant que le Ferventhar est dans un rayon de 15m |

### 3.4 Arbre 2 — Formes Animales (*Ferrath-Gaïathar*)

**Thème :** Transformations limitées, adaptations situationnelles, mobilité de support

| Type | Nom | Effet |
|------|-----|-------|
| Active | Forme de l'Ours Gaïen | Transformation 6s : gagne 200% de Rhathar-Veines en armure temporaire, taunts passif les ennemis à moins de 6m, immunité aux knockbacks |
| Active | Forme du Cerf Vif | Transformation 4s : vitesse +120%, peut traverser les alliés sans collision, les soins appliqués pendant cette forme ont leur portée doublée |
| Active | Appel de la Louve | Invoque une louve spirituelle pendant 12s qui se place sur l'allié le plus blessé et lui transfère 5% de sa vie max par seconde (ne dépasse pas le max) |
| Passive | Instinct Bestial | En forme animale, le Ferventhar est immunisé aux effets de silence et de dispel magique |
| Passive | Mémoire du Troupeau | Chaque transformation réussie réduit le cooldown des soins de 1,5s |

### 3.5 Arbre 3 — Voix de Gaïa (*Gaïa-Nexis-Profond*)

**Thème :** Connexion directe au réseau gaïen, renforcement des liens, vision des flux

| Type | Nom | Effet |
|------|-----|-------|
| Active | Tisser le Lien | Établit ou transfère un Lien Gaïen vers une cible alliée (portée 25m), instante, peut empiler jusqu'à 5 liens actifs simultanés |
| Active | Chant des Anciens | Canalisation 2s : insuffle 40% de PV max instantanément à tous les alliés liés, ignore les effets d'immunité aux soins |
| Active | Vision du Sol Vivant | Passif activable : pendant 8s, le Ferventhar voit les lignes de Ley Lines, les zones de corruption, et les auras ennemies à travers les murs (portée 20m) |
| Passive | Résonance Gaïenne | Les alliés liés qui tombent en dessous de 20% de PV déclenchent automatiquement un Communion des Racines à valeur réduite (60%), 1 fois par combat par cible |
| Passive | Permanence des Liens | Les Liens Gaïens ne se brisent pas si le Ferventhar prend des dégâts ; seule la mort ou une Dispel magique peut les rompre |

### 3.6 Compétences Signature

**Tissage des Cinq Voix** *(Cooldown : 180s — référence au rituel légendaire de l'An 7 AO)*
Le Ferventhar active tous ses Liens Gaïens actifs simultanément dans un rituel de 3s de canalisation (interruptible). Si complété, pendant 15s : tous les alliés liés reçoivent un bouclier gaïen égal à 30% de leurs PV max, régénèrent 5% de PV max par seconde, sont immunisés aux états négatifs de corruption, et toute mort d'un allié lié pendant cette durée est *annulée* — l'allié revient à 1 PV au lieu de mourir (une seule fois par allié par activation). Le Ferventhar lui-même est vulnérable pendant la canalisation : s'il est interrompu, le cooldown est consommé à 50%.

**Communion avec le Nexus** *(Cooldown : 60s — requiert un Nexus naturel actif dans un rayon de 30m)*
Le Ferventhar fusionne temporairement son essence avec le Nexus le plus proche pendant 10s. Pendant cette durée : il ne peut pas se déplacer mais est intangible aux attaques physiques, tous ses soins ont leur valeur doublée, et le Nexus émet une aura de 15m qui réduit les dégâts subis par les alliés de 20%. Si aucun Nexus n'est disponible, la compétence est grisée. En zone corrompue, cette compétence est inutilisable.

### 3.7 Synergie de groupe

Le Ferventhar amplifie les soins et régénérations de toutes les autres classes de support via ses **Liens Gaïens** : le Velthamur (Chamane) peut cibler les alliés liés pour des résurrections instantanées sans canalisation ; le Nytharven (Invocateur) voit ses créatures régénérer automatiquement si elles sont dans l'aura du Ferventhar. En groupe de 5, il est le pivot central — aucune autre classe n'est aussi dépendante de la cohésion spatiale du groupe.

### 3.8 Lore

Les Ferventhars sont les dépositaires de la mémoire vivante de la Fédération. Ce sont eux qui transmettent les Mémoires Vertes de génération en génération, qui supervisent les rituels d'inauguration des conseils de sages, et qui décident — parfois contre l'avis politique des conseils — si une zone est trop corrompue pour être habitée. Leur autorité est douce mais absolue sur les questions de santé gaïenne : un Ferventhar qui déclare un territoire "corrompu au-delà du soin" provoque le déplacement de villages entiers sans appel possible. C'est un pouvoir qu'ils exercent rarement et toujours avec une douleur visible.

---

## 4. Velsorath — Ranger Arcane (DPS Distance Hybride)

### 4.1 Identité

**Rôle :** DPS Distance Hybride — magie + arc, runes tracées en mouvement
**Armes principales :** Arc runique de feu de Ley (gravé de runes actives), Dague de rune secondaire, Carquois de focus runiques
**Couleur de classe :** Argent lumineux (#C0D0FF)
**Statistique dominante :** Velthar-Sorath (Agilité) + Gaïathar-Nexis (Intelligence)

### 4.2 Style de combat

Le Velsorath est la réponse ervan à une question que les autres factions ne se posent jamais : *comment maintenir une puissance magique maximale sans jamais s'arrêter ?* Là où les mages classiques stationnent pour canaliser, lui trace des runes dans l'air avec ses mouvements — chaque pas laisse une empreinte runique éphémère qui amplifie ses tirs suivants ou piège les ennemis qui foulent son chemin. Son arc n'est pas une arme de précision froide : les flèches runiques qu'il tire interagissent avec l'environnement, se bifurquant sur les surfaces de cristal de Ley, explosant au contact des zones chargées, se multipliant si elles traversent ses propres traînes runiques.

Sa mécanique de cœur est la **Trame Runique** : chaque mètre de déplacement génère 1 point de Trame (max 30). Il dépense la Trame pour augmenter la puissance de ses tirs (5 Trame = +15% dégâts) ou activer des runes de terrain (pièges, amplificateurs, zones de ralentissement). S'il reste immobile plus de 2s, sa Trame commence à se dissiper au rythme de 5 points par seconde.

### 4.3 Arbre 1 — Flèches de Ley (*Velthar-Nexis*)

**Thème :** Tirs puissants, effets élémentaires sur flèches, pénétration

| Type | Nom | Effet |
|------|-----|-------|
| Active | Flèche Runique Directe | Tir unique, 240% ADMag + 80% ADPhy, portée 30m, traverse le premier ennemi touché |
| Active | Salve des Trois Voies | Tire 3 flèches simultanément en éventail de 20°, chacune inflige 160% ADMag + applique un état aléatoire (brûlure/givre/choc) |
| Active | Flèche de Nexus | Flèche chargée (1s de canalisation), 400% ADMag, à l'impact crée un mini-Nexus pendant 6s qui amplifie les tirs suivants de 20% dans un rayon de 5m |
| Passive | Veine Tracée | Les flèches tirées depuis une zone de Trame active ont 15% de pénétration d'armure magique supplémentaire |
| Passive | Résonance de Carquois | Chaque tir consécutif sur la même cible augmente les dégâts de 5% (max ×6, reset si changement de cible) |

### 4.4 Arbre 2 — Tracé en Mouvement (*Velthar-Rhathar*)

**Thème :** Génération de Trame, runes de terrain, pièges runiques mobiles

| Type | Nom | Effet |
|------|-----|-------|
| Active | Dash Runique | Dash de 8m en ligne droite, laisse une traîne de runes de 8m qui dure 6s et ralentit 30% les ennemis qui la traversent, génère 15 Trame |
| Active | Pose du Sceau | Dépose une rune de piège à ses pieds (invisible aux ennemis) : le premier ennemi à marcher dessus est raciné 2s et subit 200% ADMag |
| Active | Rune d'Amplification | Trace un cercle runique de 5m de rayon (2s de tracé en marchant) : toutes les flèches du Velsorath qui traversent ce cercle gagnent +40% de dégâts pendant 8s |
| Passive | Sillage Lumineux | Les traînes runiques de Dash Runique infligent maintenant 30% ADMag par seconde au lieu de ralentir seulement |
| Passive | Mémoire Kinétique | Si le Velsorath dépasse 20 points de Trame, sa vitesse de déplacement augmente de 10% |

### 4.5 Arbre 3 — Magie Combinatoire (*Nexis-Synthesis*)

**Thème :** Synergies entre états, flèches à effets multiples, explosions combinatoires

| Type | Nom | Effet |
|------|-----|-------|
| Active | Flèche Convergente | Tire deux flèches simultanées (feu + glace) qui convergent sur le même point de 10m : si elles touchent le même ennemi, explosion de 320% ADMag et application de Fracture Élémentaire (vulnérabilité magique +25%, 8s) |
| Active | Tir de Rupture | Flèche unique qui *consomme* tous les états actifs sur la cible et inflige 120% ADMag par état consommé (max 5 états, soit 600% ADMag maximum) |
| Active | Pluie de Runes | Tire 8 flèches runiques en l'air qui retombent en zone de 8m de diamètre après 1s, chacune inflige 80% ADMag et applique un état aléatoire |
| Passive | Lecture des Symbioses | Si 3 types d'états différents sont actifs sur une cible, les prochains dégâts du Velsorath contre cette cible ignorent 20% de résistance magique |
| Passive | Trame Conductrice | Chaque état élémentaire appliqué par une flèche génère 3 points de Trame supplémentaires |

### 4.6 Compétences Signature

**Danse des Ley Lines** *(Cooldown : 100s)*
Le Velsorath entre en état de **Flux Runique** pendant 10s : sa vitesse de déplacement augmente de 50%, chaque pas génère 3 Trame au lieu de 1, et toutes ses flèches tirées pendant cet état laissent des traînes runiques persistantes de 3m de rayon autour du point d'impact (durée 4s, ralentissement 50%). En fin d'état, toute la Trame accumulée est convertie en une décharge unique : 20% ADMag par point de Trame libéré en zone autour du Velsorath (rayon 8m). Le maximum de Trame pendant cet état est porté à 60.

**Flèche du Premier Nexus** *(Cooldown : 150s)*
Canalisation de 2s pendant laquelle le Velsorath trace manuellement une rune géante dans l'air. Au terme de la canalisation, une flèche de la taille d'un pilier de Ley traverse l'écran en ligne droite (portée illimitée jusqu'à un mur ou bord de zone), infligeant 600% ADMag à chaque ennemi traversé et appliquant simultanément les trois états élémentaires (brûlure, givre, choc) pendant 6s chacun. La flèche laisse une traîne de foudre de Ley sur sa trajectoire qui persiste 8s et inflige 40% ADMag/s. Interruptible pendant la canalisation.

### 4.7 Synergie de groupe

Le Velsorath est le **poseur d'états** par excellence du groupe Ervan. Ses runes de terrain bénéficient à tous les alliés qui les traversent (gain de Trame partagé, 50% de la valeur). Le Gaïathar (Mage des Forêts) et le Solthareth (Sorcière de Combat) profitent directement des états élémentaires empilés par ses flèches pour amplifier leurs propres dégâts.

### 4.8 Lore

Les Velsoraths sont issus d'une tradition paradoxale : ils ont appris à concilier deux philosophies ervan longtemps opposées — celle des **Observateurs du Mouvement** (contemplatifs qui étudient les flux naturels) et celle des **Praticiens de Défense** (guerriers qui protègent les forêts par la force). Le résultat est une classe qui ne correspond pas tout à fait aux cases de la société ervan : trop actifs pour être des sages, trop subtils pour être de simples rangers. Les conseils les utilisent pour les missions qui requièrent à la fois discrétion et puissance — explorations de zones corrompues, contre-espionnage inter-factions, sécurisation de Nexus menacés.

---

## 5. Nytharven — Invocateur (Support via Créatures)

### 5.1 Identité

**Rôle :** Support via créatures de Gaïa — buff/debuff par procuration, contrôle de zone
**Armes principales :** Baguette de liaison (cristal de Ley enchâssé dans du bois vivant), Phylactère d'âmes gaïennes, Anneau de lien
**Couleur de classe :** Vert émeraude + or naturel (#00695C / #C8A000)
**Statistique dominante :** Gaïathar-Nexis (Intelligence) + Velthar-Sorath (Agilité)

### 5.2 Style de combat

L'Invocateur ne combat pas directement — il *dirige*. Sur un champ de bataille, il est l'œil du groupe qui voit simultanément les flux d'énergie dans ses créatures liées, les intentions de ses ennemis à travers les sens animaux de ses invocations, et les opportunités de synergie que personne d'autre ne perçoit. Ses créatures ne sont pas des familiers dociles : ce sont des fragments temporaires de la conscience de Gaïa, prêtés avec réticence et rappelés au repos quand leur mission est accomplie. Un loup invoqué n'obéit pas aveuglément : il a ses propres instincts que le Nytharven doit guider plutôt que contrôler.

Sa mécanique de cœur est la **Résonance de Meute** : chaque créature active génère un niveau de Résonance. À 1 créature = +5% de dégâts de groupe. À 3 créatures = +15% et les créatures partagent leurs sens (vision à 360°, détection de furtivité). À 5 créatures (maximum absolu) = +25% de dégâts de groupe, les créatures se coordonnent automatiquement pour focus la cible du Nytharven.

### 5.3 Arbre 1 — Meute de Gaïa (*Ferrath-Nexis*)

**Thème :** Invocation de loups, coordination de meute, dégâts physiques des créatures

| Type | Nom | Effet |
|------|-----|-------|
| Active | Appel du Loup Gaïen | Invoque 1 Loup de Gaïa (PV : 60% du max du Nytharven, dégâts : 80% ADPhy du Nytharven, durée : jusqu'à mort ou renvoi) |
| Active | Hurlement de Coordination | Les loups actifs se repositionnent simultanément derrière la cible désignée du Nytharven et attaquent ensemble dans les 2s suivantes pour +60% de dégâts |
| Active | Morsure de Ley | Désigne un loup actif pour une attaque spéciale : il charge la cible, inflige 200% ADPhy + immobilise 2s, se retire ensuite (cooldown 12s par loup) |
| Passive | Alpha de Gaïa | Chaque loup actif augmente la vitesse d'attaque des autres loups de 5% (cumulable) |
| Passive | Sang de Forêt | Les loups régénèrent 3% de leurs PV par seconde s'ils ne sont pas en combat actif depuis 3s |

### 5.4 Arbre 2 — Esprits de Gaïa (*Gaïathar-Nexis-Esprit*)

**Thème :** Invocation d'esprits elementaux, buffs/debuffs à distance, support invisible

| Type | Nom | Effet |
|------|-----|-------|
| Active | Esprit de Racine | Invoque un Esprit de Racine (intangible) sur un allié : cet allié régénère 3% de PV/s et est immunisé aux ralentissements pendant 8s |
| Active | Esprit de Brume | Invoque un Esprit de Brume qui enveloppe un ennemi : réduit sa précision de 25% et ses résistances de 10% pendant 10s |
| Active | Esprit de Tonnerre | Invoque un Esprit de Tonnerre (flottant) pendant 12s : chaque 3s, il frappe aléatoirement un ennemi dans un rayon de 8m pour 120% ADMag + choc 0,5s |
| Passive | Symbiose Spirituelle | Les esprits actifs génèrent 1 Résonance chacun (comptent dans le calcul de Résonance de Meute) |
| Passive | Lien Éthéré | Les esprits peuvent traverser les murs et les obstacles ; les loups restent contraints à la géographie normale |

### 5.5 Arbre 3 — Maître des Liens (*Nexis-Lien-Profond*)

**Thème :** Renforcement des invocations, liaisons entre créatures et alliés, sacrifice tactique

| Type | Nom | Effet |
|------|-----|-------|
| Active | Lien de Vie Partagée | Lie deux créatures actives ou une créature et un allié : ils partagent leurs PV (les dégâts subis sont répartis à 50/50) pendant 12s |
| Active | Rappel d'Urgence | Renvoie instantanément toutes les créatures actives : chaque créature rappelée restaure 8% de PV max au Nytharven et génère 2 Résonances stockées |
| Active | Invocation Suprême | Invoque temporairement une **Chimère de Gaïa** (fusion loup + esprit) pendant 20s : PV 150% du Nytharven, dégâts 200% ADMag + ADPhy, immunisée aux debuffs, compte pour 3 créatures dans la Résonance de Meute |
| Passive | Réseau Conscient | Si 3+ créatures sont actives, le Nytharven reçoit passivement les informations sensorielles de toutes (mini-radar de groupe, 25m) |
| Passive | Dernier Souffle | Quand une créature meurt, elle libère une explosion d'énergie gaïenne : 150% ADMag en zone 4m autour d'elle |

### 5.6 Compétences Signature

**Éveil de la Forêt** *(Cooldown : 140s)*
Le Nytharven rugit (0,5s d'activation) et toutes ses invocations actives entrent simultanément en **état Gaïen Éveillé** pendant 12s : leurs PV sont restaurés à 100%, leurs dégâts doublent, et elles deviennent immunisées aux dégâts pendant les 3 premières secondes de l'éveil. De plus, pendant ces 12s, chaque fois qu'une invocation inflige des dégâts, le Nytharven restaure 2% de ses PV max. Si 5 créatures sont actives au moment de l'activation, les effets durent 18s au lieu de 12s.

**Dissolution dans Gaïa** *(Cooldown : 200s)*
Le Nytharven dissout sa propre forme corporelle pendant 6s en fusionnant avec le réseau gaïen. Pendant cet état : il est intangible (immunité complète aux dégâts), toutes ses créatures reçoivent +100% de PV max temporaires, et il peut invoquer n'importe quelle créature de son répertoire sans coût de mana (max 2 invocations par dissolutions). À la fin des 6s, sa réintégration corporelle envoie une onde de soin qui restaure 25% des PV max de tous les alliés dans un rayon de 12m.

### 5.7 Synergie de groupe

Le Nytharven est le **multiplicateur silencieux** du groupe. La Résonance de Meute qu'il génère profite à tous les alliés sans qu'ils aient besoin de faire quoi que ce soit. Le Ferventhar (Druide) peut soigner ses créatures via l'aura de Terre Fertile. L'Umbrantal (Veilleur) peut se déplacer dans les zones que les créatures du Nytharven ont marquées comme sûres.

### 5.8 Lore

Les Nytharven sont peut-être la classe la plus théologiquement chargée de la Fédération. Ils ne prient pas Gaïa — ils lui *empruntent*. Chaque invocation est une négociation tacite avec le principe vivant du monde : "je prends une part de ton attention, je te promets de la rendre." Les conseils de sages ont codifié ce protocole dans les **Accords de Lierre** (An 112 AO), qui limitent le nombre d'invocations actives simultanées à 5 et interdisent l'invocation de certaines créatures (les Gardiens de Nexus notamment) sauf en cas de Stampede avéré. Les Nytharven qui violent ces accords sont rares — non par peur de la sanction sociale, mais parce qu'ils ressentent physiquement le "mécontentement" de Gaïa comme une douleur sourde qui croît avec chaque emprunt abusif.

---

## 6. Leyanthar — Gardien des Ley Lines (Tank Magique)

### 6.1 Identité

**Rôle :** Tank Magique — absorption d'énergie des Ley, boucliers réactifs, contre-flux
**Armes principales :** Bouclier de cristal de Ley (vivant, pulsant), Marteau à rune de Nexus, Brassards conducteurs
**Couleur de classe :** Bleu Ley (#1565C0)
**Statistique dominante :** Rhathar-Veines (Constitution) + Gaïathar-Nexis (Intelligence)

### 6.2 Style de combat

Le Leyanthar est une forteresse mobile. Là où un tank impérial absorbe les coups par la masse et le métal, lui absorbe par la *canalisation* — chaque frappe ennemie qui le touche est partiellement convertie en énergie de Ley qu'il stocke dans son bouclier cristallin, redistribuée plus tard sous forme de boucliers alliés ou de contre-décharges. Il est fondamentalement *réactif* : plus ses ennemis frappent fort, plus il devient dangereux. Les mages adverses qui l'affrontent découvrent rapidement que leurs sorts les plus puissants sont partiellement absorbés et renvoyés sous forme de décharges de Ley.

Sa mécanique de cœur est le **Capacitor de Ley** : un compteur d'énergie de 0 à 100 qui se charge à chaque dégât reçu (1 charge par 1% de PV max perdu) et à chaque activation de compétence de canal. Cette énergie stockée peut être dépensée pour des boucliers actifs, des décharges offensives, ou transférée vers des alliés.

### 6.3 Arbre 1 — Absorption de Nexus (*Rhathar-Ley*)

**Thème :** Réduction des dégâts reçus, conversion des coups en Capacitor, mitigation

| Type | Nom | Effet |
|------|-----|-------|
| Active | Bouclier de Cristal Actif | Génère un bouclier personnel de 250% de Rhathar-Veines en PV pendant 8s, chaque dégât absorbé par ce bouclier charge 2 Capacitor |
| Active | Absorption de Ley | Pendant 4s, réduit tous les dégâts magiques reçus de 60%, convertit 30% des dégâts absorbés en Capacitor |
| Active | Mur de Nexus | Crée un mur de cristal de Ley de 6m de large pendant 5s : bloque les projectiles physiques, réduit les sorts de 40%, génère 1 Capacitor par sort bloqué |
| Passive | Corps Conducteur | Chaque point de Capacitor donne +0.4% de résistance magique (max +40 Capacitor = +16%) |
| Passive | Peau de Pierre de Ley | Les dégâts physiques reçus sont réduits de 8% par défaut (non cumulable avec les résistances d'armure standards) |

### 6.4 Arbre 2 — Boucliers Alliés (*Nexis-Partage*)

**Thème :** Protection des alliés, transfert d'énergie, auras de résistance

| Type | Nom | Effet |
|------|-----|-------|
| Active | Transfert de Nexus | Cible un allié (portée 15m) : lui transfère 30 Capacitor sous forme de bouclier égal à 30% de ses PV max, durée 8s |
| Active | Aura de Cohésion | Active une aura de 10m de rayon pendant 10s : tous les alliés dans la zone reçoivent 15% de réduction de dégâts, charge 1 Capacitor par coup absorbé par les alliés dans la zone |
| Active | Rempart Partagé | Lie le Leyanthar à un allié pendant 6s : 30% des dégâts que l'allié subit sont redirigés vers le Leyanthar (qui les absorbe normalement dans son Capacitor) |
| Passive | Généreux Conducteur | Les boucliers transférés vers des alliés durent 2s de plus que la durée normale |
| Passive | Réseau de Cristaux | Si 3 alliés ou plus ont un bouclier actif du Leyanthar simultanément, sa régénération de Capacitor augmente de 50% |

### 6.5 Arbre 3 — Contre-Flux (*Ley-Rupture*)

**Thème :** Décharges offensives depuis le Capacitor, contre-attaques magiques, interruption

| Type | Nom | Effet |
|------|-----|-------|
| Active | Décharge de Nexus | Libère 25 Capacitor en une décharge de foudre de Ley : 300% ADMag sur la cible, choc 1s |
| Active | Nova de Cristal | Libère 50 Capacitor en une explosion sphérique de 8m de rayon : 400% ADMag + repousse de 6m, casse les effets de concentration ennemis |
| Active | Contre-Flux Absolu | Passif déclenché : si le Leyanthar reçoit un sort de plus de 30% de ses PV max en un seul coup, il renvoie automatiquement 50% des dégâts absorbés en dégâts de Ley à l'attaquant (cooldown 8s) |
| Passive | Économie de Flux | Les décharges offensives depuis le Capacitor ont leur coût réduit de 20% |
| Passive | Sur-saturation | Si le Capacitor atteint 100, le prochain sort ou attaque physique reçu est *entièrement absorbé* (annulation complète des dégâts), le Capacitor se vide à 0 après cela |

### 6.6 Compétences Signature

**Ancrage au Nexus** *(Cooldown : 90s)*
Le Leyanthar plante son bouclier dans le sol pendant 10s et entre en état d'**Ancrage** : il ne peut plus se déplacer mais gagne 40% de résistance à toutes les formes de dégâts, charge 5 Capacitor par seconde, et génère automatiquement des boucliers sur les alliés dans un rayon de 8m (bouclier de 15% de leurs PV max, mis à jour chaque 2s). Tout sort de contrôle de masse tentant de le déplacer (knockback, lévitation, pull) est annulé et converti en +10 Capacitor. L'état peut être annulé manuellement pour déclencher immédiatement une Nova de Cristal sans coût.

**Éclat du Ley Primordial** *(Cooldown : 160s)*
Nécessite 80+ Capacitor. Le Leyanthar libère toute l'énergie stockée dans son bouclier en un seul éclat directionnel de 20m de long et 4m de large. Chaque point de Capacitor converti inflige 8% ADMag (à 100 Capacitor = 800% ADMag total). Les ennemis à moins de 8m de l'éclat reçoivent les dégâts complets ; entre 8m et 20m, les dégâts décroissent linéairement à 40%. L'éclat traverse les boucliers adverses magiques. Après activation, le Leyanthar est à 0 Capacitor et sa résistance magique est réduite de 20% pendant 6s (vulnérabilité post-décharge).

### 6.7 Synergie de groupe

Le Leyanthar est le **fondateur de la ligne de front** du groupe Ervan. Ses boucliers actifs partagés permettent au Ferventhar (Druide) de concentrer ses soins sur la récupération plutôt que la mitigation d'urgence. Le Gaïathar (Mage des Forêts) bénéficie de l'Aura de Cohésion pour canaliser ses sorts les plus longs sans interruption.

### 6.8 Lore

Les Leyanthars sont les gardiens physiques des Nexus — une fonction à la fois militaire, religieuse et scientifique dans la société ervan. Ils passent des années à étudier le flux d'un Nexus spécifique avant d'être certifiés comme Gardiens de ce lieu, et beaucoup développent une relation quasi-personnelle avec le réseau de Ley de leur territoire. La corruption de Garum les affecte différemment des autres : là où un druide ressent la corruption comme une blessure morale, un Leyanthar la ressent comme un *bruit* — un parasite dans les fréquences des Ley qu'il capte constamment. Les Leyanthars les plus anciens développent parfois une capacité à *sentir* l'avancée des Stampedes jusqu'à 48h avant leur émergence.

---

## 7. Solthareth — Sorcière de Combat (DPS Nukes/Debuffs)

### 7.1 Identité

**Rôle :** DPS — nukes rapides, malédictions, debuffs, chaos contrôlé
**Armes principales :** Baguette tordue (bois de chêne-éclair), Grimoire de malédictions lié (volant en combat), Anneaux de mana
**Couleur de classe :** Violet-vert (#6A1B9A / #33691E)
**Statistique dominante :** Gaïathar-Nexis (Intelligence) + Velthar-Sorath (Agilité)

### 7.2 Style de combat

Le Solthareth est la classe la plus *agressive* de la Fédération — une contradiction culturelle que les conseils de sages tolèrent car la fonction existe, même si elle dérange. Elle ne soigne pas, ne protège pas, ne guide pas : elle *maudit*. Ses sorts frappent vite, successivement, superposant des debuffs qui s'alimentent mutuellement jusqu'à ce que la cible soit dans un état de délabrement magique si profond que n'importe quelle compétence alliée la tue en une fraction de seconde. Sa particularité est le **Grimoire Volant** : son grimoire de malédictions est en lévitation constante et peut être dirigé indépendamment de sa position, lançant automatiquement des malédictions mineures pendant qu'elle concentre ses actives sur des nukes.

Sa mécanique de cœur est la **Jauge de Chaos** : chaque debuff actif sur n'importe quel ennemi visible charge la Jauge (1 charge par debuff actif). À 10 charges, elle entre en **Frénésie de Solthar** pendant 6s : ses sorts coûtent 50% de mana en moins, leur temps de recharge est réduit de 30%, et tout soin reçu par une cible maudite est converti en dégâts supplémentaires.

### 7.3 Arbre 1 — Nukes Fulgurants (*Gaïathar-Rupture*)

**Thème :** Dégâts instantanés élevés, rapidité de cast, succession de sorts

| Type | Nom | Effet |
|------|-----|-------|
| Active | Éclair Maudit | Sort instantané, 180% ADMag + applique Malédiction Mineure (réduit résistances de 5%, 8s, cumulable ×3) |
| Active | Noirceur de Nexus | Projectile de ténèbres-Ley, 260% ADMag, ignore 15% de résistance magique, temps de cast 0,8s |
| Active | Détonation de Malédiction | Fait exploser toutes les Malédictions Mineures actives sur la cible : 80% ADMag par malédiction + étourdit 0,5s par malédiction |
| Passive | Immédiateté Chaotique | Les sorts de la Solthareth ont leur animation de cast réduite de 20% (elle gesticule plus vite) |
| Passive | Économie du Mal | Chaque Malédiction Mineure active sur une cible réduit le coût des sorts suivants de 3% contre cette cible (max -15%) |

### 7.4 Arbre 2 — Malédictions Tissées (*Solthar-Corruption*)

**Thème :** Debuffs lourds, malédictions de longue durée, affaiblissement progressif

| Type | Nom | Effet |
|------|-----|-------|
| Active | Malédiction de Lenteur | Réduit la vitesse de déplacement et d'attaque de la cible de 30% pendant 12s, résistible |
| Active | Fléau de Visions | Inflige une hallucination gaïenne : la cible voit ses alliés comme des ennemis pendant 3s (attaque les plus proches), immunité de 30s après |
| Active | Malédiction d'Épuisement | Réduit le mana/énergie max de la cible de 40% pendant 15s (si cible joueur, bloque les sorts à coût élevé) |
| Passive | Persistance du Mal | Toutes les malédictions de la Solthareth durent 25% plus longtemps |
| Passive | Propagation | Si une cible maudite touche un allié non-maudit dans un combat, 30% de chance que la malédiction se propage |

### 7.5 Arbre 3 — Chaos de Gaïa (*Nexis-Entropie*)

**Thème :** Effets aléatoires contrôlés, interaction avec la Jauge de Chaos, sorts de panique

| Type | Nom | Effet |
|------|-----|-------|
| Active | Sort du Destin | Effet aléatoire parmi 4 : dégâts (300% ADMag), soin allié (150% ADMag), malédiction de zone (8m), ou silence 2s — la probabilité change selon le niveau de Jauge |
| Active | Éclat de Panique | Envoie une onde de peur gaïenne dans un cône de 10m : les ennemis fuient aléatoirement pendant 2,5s (résistible) |
| Active | Instabilité Gaïenne | Crée une zone de 6m de diamètre pendant 8s où toute magie (alliée et ennemie) a 25% de chances d'être amplifiée ou réduite aléatoirement de 50% |
| Passive | Maîtrise du Désordre | Le Sort du Destin n'est jamais entièrement aléatoire : la Solthareth voit les 3 prochains effets possibles et peut en éliminer un |
| Passive | Frénésie Prolongée | Chaque ennemi tué pendant la Frénésie de Solthar prolonge son état de 2s (max +8s) |

### 7.6 Compétences Signature

**Sabbat de Solthar** *(Cooldown : 120s)*
La Solthareth libère son Grimoire Volant en mode autonome pendant 15s. Le Grimoire cible aléatoirement les ennemis dans un rayon de 20m, appliquant une Malédiction Mineure par seconde à des cibles différentes (ne dépasse jamais 3 fois la même cible). Pendant ces 15s, la Solthareth a ses deux mains libres et tous ses sorts ont leur vitesse de cast réduite à 0 (instantané). En fin d'état, le Grimoire revient et déclenche une Détonation de Malédiction automatique sur la cible la plus chargée en malédictions.

**Malédiction de Garum** *(Cooldown : 200s — nom délibérément provocateur dans la société ervan)*
La Solthareth inscrit sur une cible la malédiction la plus puissante de son répertoire : pendant 20s, la cible ne peut pas régénérer de PV ni de mana/énergie d'aucune source (soins, régénération, potions, capacités actives). De plus, 20% de tous les dégâts reçus par la cible pendant cette durée sont convertis en soins pour la Solthareth. La malédiction est visible par tous (aura rouge sombre) et peut être purifiée par le Ferventhar ou le Velthamur. Son nom est une transgression culturelle délibérée — nommer un sort d'après Garum est tabou en territoire ervan.

### 7.7 Synergie de groupe

La Solthareth est le **démultiplicateur de dégâts** du groupe via ses réductions de résistances. Ses Malédictions Mineures cumulées créent des fenêtres d'opportunité que le Gaïathar (Mage des Forêts) et le Velsorath (Ranger Arcane) exploitent pour des burst dévastateurs. Le Rhatharven (Tisserand de Sorts) peut tresser ses malédictions dans ses enchantements pour créer des combos automatiques.

### 7.8 Lore

Le Solthareth est une profession mal-aimée dans la Fédération — pas interdite, mais marginalisée. Les malédictions sont une forme de magie que Gaïa *tolère* sans *approuver* : elles travaillent avec le principe d'entropie naturelle de la vie (tout décline, tout se corrompt) plutôt qu'avec le principe de cohésion. Les praticiens ne l'ignorent pas : beaucoup de Solthareth développent une relation philosophique complexe avec leur propre pratique, oscillant entre une justification fonctionnelle ("je combats Garum avec ses propres outils") et une culpabilité sourde. Les conseils de sages les autorisent parce qu'ils reconnaissent leur efficacité contre les Stampedes — mais aucun Solthareth n'a jamais siégé dans un conseil.

---

## 8. Velthamur — Chamane (Support/Heal Ancestral)

### 8.1 Identité

**Rôle :** Support / Heal via esprits ancestraux — résurrection, vision du passé, soins indirects
**Armes principales :** Hochet d'os et de cristal de Ley, Houlette de brume ancestrale, Masque de l'Ancien (couvre le visage, amplifie la communication avec les esprits)
**Couleur de classe :** Gris-brume et or pâle (#607D8B / #FFF9C4)
**Statistique dominante :** Gaïathar-Nexis (Intelligence) + Rhathar-Veines (Constitution)

### 8.2 Style de combat

Le Velthamur ne combat pas dans le présent — il combat depuis le passé. Ses esprits ancestraux sont les fragments de conscience de druides, de gardiens et de sages ervan morts depuis des siècles, maintenus dans un état de présence partielle par le réseau gaïen. Il les convoque, les guide, les écoute, et parfois les laisse parler à travers lui. En combat, il est à mi-chemin entre le chamane classique et le médecin de terrain : il voit les blessures *avant* qu'elles soient infligées (via les murmures ancestraux), soigne via des rituels que les esprits exécutent à sa place, et peut — dans les moments les plus graves — demander à un esprit de *remplacer* temporairement un allié mort le temps que la vraie résurrection soit préparée.

Sa mécanique de cœur est l'**Écho Ancestral** : il maintient un réseau de 1 à 3 esprits actifs autour de lui à tout moment. Chaque esprit est une ancienne personnalité avec des spécialités propres. Le Chamane "parle" avec eux via ses actives et leur donne des instructions ; eux exécutent avec une certaine autonomie interprétative (source de surprises occasionnelles, voulues mécaniquement).

### 8.3 Arbre 1 — Esprits Soigneurs (*Rhathar-Esprit*)

**Thème :** Soins via les esprits ancestraux, régénération, protection spirituelle

| Type | Nom | Effet |
|------|-----|-------|
| Active | Appel de l'Esprit Guérisseur | Invoque l'Esprit Guérisseur pendant 15s : il se positionne automatiquement sur l'allié le plus blessé et le soigne de 6% de PV max par seconde |
| Active | Bénédiction Ancestrale | L'Esprit Guérisseur actif touche une cible désignée : soin immédiat de 200% Gaïathar-Nexis + bouclier spirituel de 100% Gaïathar-Nexis pendant 6s |
| Active | Cercle de Purification | L'Esprit Guérisseur crée un cercle de 8m : tous les alliés dedans sont purifiés de 2 effets négatifs par seconde pendant 5s |
| Passive | Mémoire de Soin | L'Esprit Guérisseur se souvient des 3 dernières blessures graves de chaque allié soigné et applique un soin préventif léger (2% PV/s) si ces conditions semblent se répéter |
| Passive | Présence Apaisante | La simple présence d'un Esprit Guérisseur actif réduit le délai de récupération des consommables de soin de tous les alliés dans 12m de 20% |

### 8.4 Arbre 2 — Résurrection et Passage (*Velthar-Mort*)

**Thème :** Résurrection d'alliés, remplacement temporaire par un esprit, interaction avec la mort

| Type | Nom | Effet |
|------|-----|-------|
| Active | Rite de Retour | Canalisation 3s sur un allié mort : le ressuscite à 40% de ses PV max, réduit ses cooldowns de 50% pendant 10s, coût 60% de mana |
| Active | Présence de l'Esprit Gardien | Quand un allié dans 15m descend à 0 PV, un Esprit Gardien prend possession de son corps pendant 5s : l'allié est maintenus en vie avec 1 PV, immunisé aux dégâts, mais ne peut agir. Le Velthamur a 5s pour lancer Rite de Retour en priorité sur cet allié |
| Active | Traversée des Voiles | Le Velthamur lui-même peut traverser momentanément l'état de mort : pendant 3s, il est en "Forme Spectrale" (intangible, invisible, ne peut pas agir), puis revient avec 30% de PV. Utilisable uniquement si ses PV descendent sous 10% |
| Passive | Voile Mince | Dans un rayon de 20m autour du Velthamur, les délais avant qu'un allié mort puisse être résurrectionné sont réduits de 2s |
| Passive | Souvenir des Morts | Chaque allié résurrectionné génère 1 Écho Ancestral stocké (max 3) que le Velthamur peut dépenser pour accélérer son prochain Rite de Retour de 1,5s |

### 8.5 Arbre 3 — Vision du Passé (*Gaïathar-Temporel*)

**Thème :** Informations sur le passé, anticipation des combats, auras de connaissance ancestrale

| Type | Nom | Effet |
|------|-----|-------|
| Active | Regard dans le Passé | Pendant 5s, le Velthamur voit les 3 dernières actions effectuées par l'ennemi ciblé (sous forme de flash visuels) + reçoit un indicateur de sa prochaine compétence probable |
| Active | Murmures des Anciens | Aura de groupe pendant 10s : tous les alliés dans 15m reçoivent un indicateur visuel 1s avant qu'un ennemi les cible (réduction des dégâts surprises de 15%) |
| Active | Écho de Bataille | Rejoue un événement passé du combat (max 8s dans le passé) : un allié mort il y a moins de 8s revient à la vie à la position et aux PV qu'il avait à ce moment précis — durée 6s puis l'écho se dissout (l'allié doit être ressuscité normalement ensuite ou il meurt à nouveau) |
| Passive | Mémoire Vivante | Les informations données par Regard dans le Passé sont partagées à tous les alliés liés au Velthamur dans un rayon de 20m |
| Passive | Prescience Gaïenne | Le Velthamur reçoit automatiquement une alerte si une Stampede est en cours à moins de 500m (avant les systèmes de détection normaux) |

### 8.6 Compétences Signature

**Concile des Anciens** *(Cooldown : 180s)*
Le Velthamur invoque simultanément 3 esprits ancestraux (Guérisseur + Gardien + Guerrier) pendant 20s en un rituel de 1,5s. Le Guerrier ancestral est une nouveauté : il se fixe sur la cible la plus dangereuse et lui inflige 80% ADMag par seconde tout en absorbant 20% des dégâts que cette cible inflige aux alliés proches. Pendant les 20s, le Velthamur bénéficie des effets passifs de ses trois arbres simultanément. En fin de durée, chaque esprit libère un soin sur l'allié le plus proche (200% Gaïathar-Nexis chacun).

**Testament du Premier Druide** *(Cooldown : 240s — compétence la plus emblématique du Chamane)*
Canalisation de 4s pendant laquelle le Velthamur communique avec un esprit ancestral d'avant les Trois Maux. Au terme de la canalisation : tous les alliés dans un rayon de 20m sont soignés à 100% de leurs PV max, toutes leurs malédictions et états négatifs sont purgés, et ils reçoivent une **Bénédiction Primordiale** pendant 30s (résistance +20% à tous les dégâts, immunité aux effets de corruption de Garum, régénération de 3% PV/s). Le Velthamur lui-même est épuisé après l'activation : ses capacités de soin sont réduites de 50% pendant 30s. Interruptible.

### 8.7 Synergie de groupe

Le Velthamur est le **filet de sécurité ultime** du groupe. Sa capacité de résurrection en combat (Rite de Retour, Présence de l'Esprit Gardien) est la plus puissante de toutes les classes Ervan. Le Ferventhar (Druide) peut cibler les alliés sous Présence de l'Esprit Gardien pour des soins prioritaires. L'Écho de Bataille peut annuler des situations catastrophiques et constitue la compétence de "remontée après désastre" la plus puissante du jeu.

### 8.8 Lore

Les Velthamur sont les archivistes vivants de la Fédération — non pas par des livres ou des parchemins, mais par les esprits eux-mêmes. Chaque Chamane construit au fil de sa vie un **Cercle Personnel** d'esprits avec lesquels il développe une relation : il connaît leurs noms d'avant la mort, leurs regrets, leurs sagesses. Cette relation est exigeante : un esprit peut refuser d'agir s'il juge la situation contraire à ses valeurs, obligeant le Chamane à négocier en temps réel — ce que les concepteurs des mécaniques appellent "l'autonomie interprétative" des invocations. Les Velthamur les plus anciens ont souvent un air distant et légèrement décalé du présent : ils entendent trop de voix du passé pour être pleinement dans l'instant.

---

## 9. Rhatharven — Tisserand de Sorts (Hybride Synergies)

### 9.1 Identité

**Rôle :** Hybride — enchantements combinés sur alliés et ennemis, réactivité, synergies automatiques
**Armes principales :** Gants de trame (fils de Ley tissés dans du cuir vivant), Orbe tissé (sphère de fils d'énergie entrelacés), Aiguilles runiques (projectiles secondaires)
**Couleur de classe :** Or-argent entrelacé (#FFD700 / #C0C0C0)
**Statistique dominante :** Gaïathar-Nexis (Intelligence) + Velthar-Sorath (Agilité) en proportions égales

### 9.2 Style de combat

Le Rhatharven est la classe la plus *systémique* de la Fédération — celui qui comprend que la somme des compétences alliées peut être supérieure à leur simple addition si elles sont correctement *entrelacées*. Ses enchantements ne font pas de dégâts directs importants : ils modifient les conditions des sorts et des effets déjà en cours, créant des réactions en chaîne. Une flèche de Velsorath qui traverse un enchantement de Rhatharven peut soudainement se démultiplier en cinq flèches. Un soin de Ferventhar tissé dans une aura de Rhatharven soigne simultanément trois cibles au lieu d'une. Ses gants de trame lui permettent de saisir des fils d'énergie magique dans l'air ambiant et de les retravailler en temps réel.

Sa mécanique de cœur est la **Trame Active** : il maintient jusqu'à 6 fils d'enchantement simultanément — 3 sur des alliés (amplificateurs), 3 sur des ennemis (affaiblisseurs). Ces fils sont visibles à l'œil dans le jeu (effets visuels de fils d'or-argent) et peuvent interagir entre eux si deux fils connectent le même événement de combat.

### 9.3 Arbre 1 — Enchantements Offensifs (*Gaïathar-Trame*)

**Thème :** Amplification des dégâts alliés, multiplication des effets existants

| Type | Nom | Effet |
|------|-----|-------|
| Active | Fil d'Amplification | Tisse un fil sur un allié pendant 12s : son prochain sort inflige 50% de dégâts supplémentaires et génère 1 Trame Active |
| Active | Tresse Explosive | Tisse 3 fils simultanément sur la même cible ennemie : la prochaine source de dégâts magiques à toucher cette cible déclenche une explosion supplémentaire de 180% ADMag en zone 4m |
| Active | Résonance Tissée | Tisse un fil entre deux alliés : pendant 8s, 30% des dégâts que l'un inflige sont également infligés par l'autre (sans coût de mana/énergie pour le second) |
| Passive | Maîtrise de Trame | Chaque fil d'enchantement actif sur un allié augmente sa pénétration magique de 3% (cumulable par allié) |
| Passive | Écho de Tissu | Quand un fil expire naturellement (sans être dissipé), il libère un soin de 60% Gaïathar-Nexis sur l'allié qu'il enchantait |

### 9.4 Arbre 2 — Enchantements Défensifs (*Rhathar-Trame*)

**Thème :** Réduction des dégâts via tresse défensive, absorption, redistribution

| Type | Nom | Effet |
|------|-----|-------|
| Active | Fil de Déviation | Tisse un fil défensif sur un allié pendant 10s : 25% des prochains dégâts reçus sont déviés vers le sol (annulés) |
| Active | Cocon de Trame | Enveloppe un allié dans un cocon de fils de Ley pendant 3s : immunité complète aux dégâts, immobilisation, puis libération avec +40% de vitesse d'attaque pendant 5s |
| Active | Redistribution | Tisse un réseau entre 3 alliés pendant 8s : les dégâts reçus par l'un sont répartis à 33% entre les trois |
| Passive | Trame Renforcée | Les fils défensifs du Rhatharven résistent à 1 dispel magique ennemi avant de se briser |
| Passive | Contrepoint | Si un Cocon de Trame est brisé prématurément par un ennemi, cet ennemi reçoit 200% ADMag en retour |

### 9.5 Arbre 3 — Synergies Autonomes (*Nexis-Synthesis-Profond*)

**Thème :** Réactions automatiques entre fils, création de systèmes auto-entretenu, combos passifs

| Type | Nom | Effet |
|------|-----|-------|
| Active | Trame Vivante | Tisse un réseau de fils autonome pendant 15s : chaque fois qu'une compétence alliée affecte une cible dans le réseau, 1 fil supplémentaire se crée automatiquement (max +3 fils auto) |
| Active | Nœud de Convergence | Crée un point fixe de 4m de rayon pendant 10s : tous les enchantements actifs des alliés passant dans ce point voient leur durée prolongée de 4s |
| Active | Démontage | Retire violemment tous les fils actifs sur une cible ennemie : chaque fil retiré inflige 150% ADMag et rend la cible vulnérable à la magie +15% pendant 5s |
| Passive | Intelligence du Tissu | Le Rhatharven reçoit une notification visuelle chaque fois qu'un fil enchantement sur un allié est sur le point d'expirer (3s d'avance) |
| Passive | Perpétuation | Si le même fil est réappliqué sur la même cible avant expiration, sa durée est prolongée de 50% au lieu de recommencer |

### 9.6 Compétences Signature

**Grand Tissu de Gaïa** *(Cooldown : 150s)*
Le Rhatharven tisse pendant 2s un réseau de fils de Ley qui connecte tous les alliés présents dans un rayon de 20m (maximum 8 connexions). Pendant 20s, ce réseau a les effets suivants : 15% de tous les soins reçus par un allié connecté se propagent aux autres ; 20% de tous les dégâts infligés par un allié connecté s'additionnent pour une frappe partagée sur la cible la plus menaçante toutes les 5s ; si un allié connecté subit un effet de mort (PV à 0), le réseau absorbe 30% des dégâts létaux (peut prolonger la survie). En fin de durée, les fils se dissolvent en une explosion de soin de 120% Gaïathar-Nexis sur chaque allié connecté.

**Contre-Tissu** *(Cooldown : 80s)*
Analyse en temps réel un enchantement ou une aura ennemie active dans un rayon de 20m et tisse un **contre-enchantement** en 0,5s. Le contre-enchantement annule l'effet ennemi et le retourne : une aura ennemie de buff-dégâts devient une aura de debuff-dégâts de même intensité pendant la durée restante. Une malédiction ennemie active sur un allié est renvoyée à son lanceur. Un sort de contrôle de masse en cours d'incantation est annulé et transformé en 3s de ralentissement sur le lanceur. Cette compétence requiert que le Rhatharven *voie* (ligne de vue) la source de l'enchantement ennemi.

### 9.7 Synergie de groupe

Le Rhatharven est le **multiplicateur invisible** du groupe — ses effets sont rarement spectaculaires seuls, mais transforment radicalement l'efficacité de toutes les autres classes. Ses fils de Résonance Tissée entre un Gaïathar (Mage des Forêts) et un Velsorath (Ranger Arcane) créent des chaînes d'explosions élémentaires qu'aucune des deux classes ne pourrait produire seule. En groupe optimisé, il est la colle qui double l'efficacité d'un groupe de 5 par rapport à la même composition sans lui.

### 9.8 Lore

Les Rhatharven sont les descendants directs des techniciens magiques de l'Ère de l'Empire Uni — ceux qui travaillaient dans les **Académies Mixtes** de Velanthor à combiner magie et ingénierie. Après l'effondrement de l'Empire, leurs successeurs ervans ont réorienté ces techniques vers la cohésion communautaire plutôt que la puissance militaire. Aujourd'hui, ils sont les praticiens les plus demandés dans les conseils de sages pour des projets complexes : restauration de Ley Lines endommagées, renforcement des Nexus, études des patterns de Garum. Leur rapport à la magie est fondamentalement *industrieux* — ils voient le mana comme un matériau à travailler plutôt qu'une force à canaliser.

---

## 10. Umbrantal — Veilleur (Stealth Magique)

### 10.1 Identité

**Rôle :** Stealth Magique — espionnage entre factions, sabotage de Ley corrompues, infiltration
**Armes principales :** Stylet runique (lame courte gravée d'un réseau de runes, active au toucher), Cape d'effacement (tissu de Ley stabilisé, supprime la signature magique), Médaillon de Ley (outil d'interface avec les réseaux de Ley corrompus)
**Couleur de classe :** Vert sombre et argent pâle (#1A3A1A / #B0BEC5)
**Statistique dominante :** Velthar-Sorath (Agilité) + Gaïathar-Nexis (Intelligence)

### 10.2 Style de combat

L'Umbrantal est la réponse ervan à une question pragmatique que la philosophie gaïenne rend politiquement sensible : *comment protéger la Fédération des menaces qui ne peuvent pas être affrontées directement ?* Il n'est pas un assassin — tuer est son dernier recours. Il est un **chirurgien de l'information et des Ley Lines** : il s'infiltre pour observer, pour cartographier les réseaux corrompus, pour démanteler les altars de Garum que l'Empire ou les Outlaws établissent parfois sur des Nexus volés. Son invisibilité n'est pas physique — elle est magique : il supprime sa signature énergétique dans le réseau gaïen, devenant invisible aux sens magiques et aux Ley Lines elles-mêmes.

Sa mécanique de cœur est l'**Empreinte de Gaïa** : chaque action qu'il effectue génère une Empreinte (de 0 à 100). À 0, il est parfaitement invisible aux détections magiques. Chaque attaque, chaque compétence active, chaque interaction avec un objet ajoute des Empreintes. À 60+, les mages ennemis peuvent le détecter. À 100, son invisibilité est brisée et il doit attendre 3s pour commencer à l'effacer (réduction naturelle de 10/s en stationnaire, 5/s en mouvement lent).

### 10.3 Arbre 1 — Silence Gaïen (*Velthar-Efface*)

**Thème :** Maintien de l'invisibilité, réduction d'Empreinte, mouvements silencieux

| Type | Nom | Effet |
|------|-----|-------|
| Active | Effacement de Signature | Réduction immédiate de 30 Empreintes, cooldown 8s — seule façon de réduire activement l'Empreinte |
| Active | Pas de Brume | Pendant 6s, déplacements sans génération d'Empreinte, vitesse +30%, les pièges au sol sont ignorés |
| Active | Impulsion d'Oubli | Supprime la mémoire immédiate de 1 ennemi non-joueur (PNJ) : l'ennemi oublie avoir vu l'Umbrantal dans les 4 dernières secondes (uniquement PNJ, pas PvP) |
| Passive | Silence Intérieur | En dessous de 30 Empreintes, la régénération de mana est doublée |
| Passive | Ombre de Gaïa | La Cape d'effacement réduit passivement la génération d'Empreinte de 20% sur toutes les actions |

### 10.4 Arbre 2 — Sabotage des Ley (*Gaïathar-Rupture-Corrompue*)

**Thème :** Interaction avec les Ley Lines corrompues, démantèlement, sabotage de structures

| Type | Nom | Effet |
|------|-----|-------|
| Active | Purge de Ley | Canalisation 4s sur une Ley Line ou un Nexus corrompu : réduit son niveau de corruption de 25% et génère 50 Empreintes — ne peut être effectuée qu'une fois par Nexus par heure |
| Active | Court-Circuit de Ley | Interrompt temporairement un flux de Ley corrompu dans une zone de 6m : les ennemis dépendants des Ley perdent 30% de leur puissance magique pendant 8s, génère 30 Empreintes |
| Active | Pose de Rune de Sabotage | Installe une rune invisible (30m de portée visible uniquement par le Veilleur) : se déclenche quand un ennemi interagit avec la Ley ciblée, infligeant 300% ADMag et réduisant la puissance de son prochain sort de 50% |
| Passive | Lecteur de Flux | L'Umbrantal voit les Ley Lines à travers les murs et surfaces (rayon 30m), y compris les corrompues |
| Passive | Mains Légères | La Pose de Rune de Sabotage ne génère que 10 Empreintes au lieu de 25 |

### 10.5 Arbre 3 — Frappe dans l'Ombre (*Velthar-Ferrath*)

**Thème :** Attaques depuis l'ombre, application de malédictions runiques, extraction d'informations

| Type | Nom | Effet |
|------|-----|-------|
| Active | Piqûre de Rune | Attaque de mêlée avec le stylet, 180% ADPhy + applique une Rune Runique sur la cible (invisible à la cible) : révèle ses prochaines 2 actives au Veilleur, génère 20 Empreintes |
| Active | Sabotage de Mana | Via le stylet, injecte une rune disruptrice : réduit le mana max de la cible de 35% pendant 15s, annule les régénérations en cours, génère 25 Empreintes |
| Active | Extraction | Depuis l'invisibilité (0-29 Empreintes) uniquement : attaque de mêlée silencieuse, 400% ADPhy, applique Silence 3s, ne brise pas l'invisibilité si réussie (génère quand même 35 Empreintes) |
| Passive | Lame Silencieuse | Les attaques du Stylet en dessous de 40 Empreintes génèrent 5 Empreintes de moins que la normale |
| Passive | Chirurgien de Ley | Les compétences de sabotage de Ley ont leur durée d'effet augmentée de 30% |

### 10.6 Compétences Signature

**Disparition Gaïenne** *(Cooldown : 45s)*
L'Umbrantal se fusionne instantanément avec le réseau de Ley local pendant 5s : invisibilité totale (même aux détections magiques), déplacement +50%, génération d'Empreinte à 0 pendant toute la durée. Il peut traverser les parois fines (jusqu'à 1m d'épaisseur) pendant cet état. Toute attaque lancée dans les 2 premières secondes de Disparition est considérée comme lancée depuis l'invisibilité parfaite (effet de surprise automatique, dégâts doublés). La disparition se termine immédiatement si l'Umbrantal attaque après la 2e seconde ou si ses PV tombent sous 20%.

**Démantèlement du Nœud Corrompu** *(Cooldown : 300s — compétence d'objectif majeure)*
Requiert d'être adjacent (2m) à un nœud de Ley corrompue de niveau moyen ou supérieur. Canalisation de 8s (interruptible) : l'Umbrantal démantèle le nœud en injectant une contre-signature gaïenne. Résultat : le nœud est purifié à 100%, tous les ennemis qui tiraient de l'énergie du nœud perdent 40% de leur puissance magique pendant 30s, et une décharge de Ley pure éclate dans un rayon de 15m pour 300% ADMag à tous les ennemis corrompus présents. Cette compétence ne peut être utilisée qu'une fois par nœud et est enregistrée dans les archives des Cercles Ervans (impact narratif permanent sur la session de jeu).

### 10.7 Synergie de groupe

L'Umbrantal est le **renseignement** du groupe. Les informations qu'il récolte via Piqûre de Rune (révélation des actives ennemies) sont partagées visuellement avec tous les alliés dans un rayon de 20m. Son Sabotage de Ley sur les nœuds corrompus affaiblit les ennemis d'une zone entière avant même que le groupe n'y entre. En coordination avec le Leyanthar (Gardien des Ley), il peut identifier les nœuds à purifier pendant que le Gardien protège la canalisation.

### 10.8 Lore

Les Umbrantal sont le secret le moins bien gardé de la Fédération. Tout le monde sait qu'ils existent — les autres factions en particulier. Mais la Fédération maintient officiellement qu'ils sont de simples "Explorateurs de Ley", des chercheurs indépendants qui cartographient les réseaux énergétiques pour le bénéfice de tous. Ce déni plausible est une tradition centenaire, et personne n'a vraiment intérêt à la remettre en question. Les Umbrantal eux-mêmes entretiennent l'ambiguïté : ils ne se définissent jamais comme des espions ou des agents, mais comme des **Veilleurs** — ceux qui observent sans être vus, pour que les autres puissent dormir en paix. La plupart d'entre eux ont une relation sincère avec cette autodéfinition. Quelques-uns ont développé une philosophie plus sombre : que l'invisibilité parfaite n'est pas un outil, mais une condition existentielle, et qu'un être que personne ne voit n'existe peut-être pas vraiment.

---

## 11. Synergies de Faction

### 11.1 Doctrine de Groupe Ervan

La Fédération Ervan est la faction dont les classes ont le plus fort potentiel de synergie intra-groupe — et le plus faible potentiel en solo pur. Un Gaïathar seul est un mage compétent. Un Gaïathar avec un Rhatharven qui amplifie ses fils, un Velsorath qui empile des états élémentaires, et un Ferventhar qui maintient son Confluence Gaïenne via des soins continus est une force de destruction élémentaire sans équivalent dans le jeu.

### 11.2 Compositions Canoniques

**La Triade de Guérison** *(3 joueurs minimum)*
- Ferventhar (Druide) + Velthamur (Chamane) + Nytharven (Invocateur)
- Zéro mort permanente possible en combat standard. Idéal pour progression de contenu difficile sans DPS élevé.

**La Lance Élémentaire** *(5 joueurs, burst PvP et boss)*
- Gaïathar + Velsorath + Rhatharven + Solthareth + Leyanthar
- Le Rhatharven amplifie tous les états du Gaïathar et du Velsorath. Le Solthareth réduit les résistances. Le Leyanthar absorbe les contre-offensives. Burst total en 8-10s sur cible unique.

**La Forteresse Vivante** *(défense de territoire, RvR)*
- Leyanthar + Ferventhar + Velthamur + 2× Nytharven
- 10 créatures actives, boucliers multi-couches, résurrection rapide. Impossible à percer en affrontement direct.

**L'Ombre et la Tempête** *(groupe hybride furtif/offensif)*
- Umbrantal + Solthareth + Velsorath + Velthamur + Rhatharven
- L'Umbrantal prépare le terrain (sabotage Ley, informations), Velsorath et Solthareth déclenchent en explosif, Rhatharven amplifie et Velthamur maintient.

### 11.3 Bonus de Faction Passive

Tous les joueurs Ervan bénéficient de :
- **Affinité Gaïenne** : régénération de mana +15% dans les forêts, à moins de 50m d'un Nexus actif, ou sur une Ley Line non-corrompue.
- **Lecture du Vivant** : les effets de corruption de Garum sont visibles 2s avant d'affecter le joueur (aura d'avertissement).
- **Lien Communautaire** : si 3 joueurs Ervan ou plus sont dans un rayon de 20m, tous bénéficient de +5% de résistance à tous les dégâts.
- **Mémoire de Gaïa** : les consommables de soin ont leur efficacité augmentée de 10% pour les joueurs Ervan.

---

## 12. Schémas TOML Complets

```toml
# =============================================================================
# FÉDÉRATION ERVAN — Classes jouables
# Véranthas, An 247 AO
# =============================================================================

[faction.federation_ervan]
id = "federation_ervan"
name = "Fédération Ervan"
identity_social = "habitant"  # anth-ferrath en proto-ervan
colors = ["#1B5E20", "#C0C0C0"]  # vert forêt + argent
philosophy = "gaïa_comprehension"
garum_stance = "heal_not_fight"
governance = "conseil_des_sages"
bonus_passive = [
  "affinite_gaienne",
  "lecture_du_vivant",
  "lien_communautaire",
  "memoire_de_gaia"
]

# =============================================================================
# CLASSE 1 — GAÏATHAR (Mage des Forêts)
# =============================================================================

[class.gaiathar]
id = "gaiathar"
display_name = "Gaïathar"
lore_name = "Mage des Forêts"
faction = "federation_ervan"
role = "dps_magic"
weapon_types = ["staff_ancestral", "wand_ley", "focus_ervan"]
skill_trees = ["feu_naturel", "tempete_des_ley", "metamorphose_elementaire"]
base_stats = { ferrath = 10, velthar = 15, gaiathar = 40, rhathar = 12 }
core_mechanic = "cycle_des_saisons"
resource_type = "mana"
color = "#1B5E20"
signature_skills = ["eveil_de_la_foret_ancienne", "voix_du_nexus_vide"]

[class.gaiathar.skill_tree.feu_naturel]
id = "feu_naturel"
theme = "feu_elementaire_zone_burst"
actives = ["embrasement_sous_bois", "colere_canopee", "nova_pyrrhique"]
passives = ["resine_ardente", "fievre_des_cendres"]

[class.gaiathar.skill_tree.tempete_des_ley]
id = "tempete_des_ley"
theme = "foudre_propagation_chaines"
actives = ["arc_de_ley", "grille_tellurique", "appel_du_nexus"]
passives = ["conducteur_naturel", "surcharge_ley"]

[class.gaiathar.skill_tree.metamorphose_elementaire]
id = "metamorphose_elementaire"
theme = "givre_controle_synergies"
actives = ["souffle_hiver_eternel", "prison_de_givre", "fracture_de_grele"]
passives = ["convergence_elements", "peau_pierre_froide"]

# =============================================================================
# CLASSE 2 — FERVENTHAR (Druide)
# =============================================================================

[class.ferventhar]
id = "ferventhar"
display_name = "Ferventhar"
lore_name = "Druide"
faction = "federation_ervan"
role = "support_heal"
weapon_types = ["sceptre_lierre", "focale_gaienne", "houlette_croissance"]
skill_trees = ["racines_guerisseuses", "formes_animales", "voix_de_gaia"]
base_stats = { ferrath = 12, velthar = 14, gaiathar = 32, rhathar = 28 }
core_mechanic = "lien_gaïen"
resource_type = "mana"
color = "#4CAF50"
max_liens = 5
signature_skills = ["tissage_cinq_voix", "communion_avec_le_nexus"]

[class.ferventhar.skill_tree.racines_guerisseuses]
id = "racines_guerisseuses"
theme = "soins_directs_purification"
actives = ["communion_des_racines", "reseau_de_seve", "purge_gaienne"]
passives = ["seve_de_printemps", "terre_fertile"]

[class.ferventhar.skill_tree.formes_animales]
id = "formes_animales"
theme = "transformations_limitees_situationnelles"
actives = ["forme_ours_gaïen", "forme_cerf_vif", "appel_de_la_louve"]
passives = ["instinct_bestial", "memoire_du_troupeau"]

[class.ferventhar.skill_tree.voix_de_gaia]
id = "voix_de_gaia"
theme = "connexion_directe_gaïa_liens_vision"
actives = ["tisser_le_lien", "chant_des_anciens", "vision_sol_vivant"]
passives = ["resonance_gaienne", "permanence_des_liens"]

# =============================================================================
# CLASSE 3 — VELSORATH (Ranger Arcane)
# =============================================================================

[class.velsorath]
id = "velsorath"
display_name = "Velsorath"
lore_name = "Ranger Arcane"
faction = "federation_ervan"
role = "dps_ranged_hybrid"
weapon_types = ["arc_runique", "dague_de_rune", "carquois_focus_runiques"]
skill_trees = ["fleches_de_ley", "trace_en_mouvement", "magie_combinatoire"]
base_stats = { ferrath = 14, velthar = 30, gaiathar = 28, rhathar = 14 }
core_mechanic = "trame_runique"
resource_type = "mana_trame"  # double resource : mana + Trame (0-30)
color = "#C0D0FF"
trame_max = 30
trame_decay_stationary = 5  # par seconde après 2s d'immobilité
signature_skills = ["danse_des_ley_lines", "fleche_du_premier_nexus"]

[class.velsorath.skill_tree.fleches_de_ley]
id = "fleches_de_ley"
theme = "tirs_puissants_effets_elementaires"
actives = ["fleche_runique_directe", "salve_des_trois_voies", "fleche_de_nexus"]
passives = ["veine_tracee", "resonance_de_carquois"]

[class.velsorath.skill_tree.trace_en_mouvement]
id = "trace_en_mouvement"
theme = "trame_terrain_pieges_runiques"
actives = ["dash_runique", "pose_du_sceau", "rune_amplification"]
passives = ["sillage_lumineux", "memoire_kinetique"]

[class.velsorath.skill_tree.magie_combinatoire]
id = "magie_combinatoire"
theme = "synergies_etats_fleches_complexes"
actives = ["fleche_convergente", "tir_de_rupture", "pluie_de_runes"]
passives = ["lecture_des_symbioses", "trame_conductrice"]

# =============================================================================
# CLASSE 4 — NYTHARVEN (Invocateur)
# =============================================================================

[class.nytharven]
id = "nytharven"
display_name = "Nytharven"
lore_name = "Invocateur"
faction = "federation_ervan"
role = "support_summon"
weapon_types = ["baguette_liaison_ley", "phylactere_ames_gaïennes", "anneau_de_lien"]
skill_trees = ["meute_de_gaia", "esprits_de_gaia", "maitre_des_liens"]
base_stats = { ferrath = 10, velthar = 20, gaiathar = 35, rhathar = 12 }
core_mechanic = "resonance_de_meute"
resource_type = "mana"
color = "#00695C"
max_creatures = 5
resonance_per_creature = 5  # % dégâts groupe par créature active
signature_skills = ["eveil_de_la_foret_nythar", "dissolution_dans_gaia"]

[class.nytharven.creatures]
loup_gaïen = { hp_ratio = 0.60, dmg_ratio = 0.80, type = "physical" }
esprit_guerisseur = { hp = 0, dmg = 0, type = "support_invisible" }
esprit_de_brume = { hp = 0, dmg = 0, type = "debuff_invisible" }
esprit_de_tonnerre = { hp = 0, dmg_ratio = 1.20, type = "magic_aoe" }
chimere_de_gaia = { hp_ratio = 1.50, dmg_ratio = 2.00, counts_as = 3, type = "hybrid" }

[class.nytharven.skill_tree.meute_de_gaia]
id = "meute_de_gaia"
theme = "loups_coordination_meute"
actives = ["appel_loup_gaïen", "hurlement_coordination", "morsure_de_ley"]
passives = ["alpha_de_gaia", "sang_de_foret"]

[class.nytharven.skill_tree.esprits_de_gaia]
id = "esprits_de_gaia"
theme = "esprits_elementaux_support_debuff"
actives = ["esprit_de_racine", "esprit_de_brume", "esprit_de_tonnerre"]
passives = ["symbiose_spirituelle", "lien_etheree"]

[class.nytharven.skill_tree.maitre_des_liens]
id = "maitre_des_liens"
theme = "renforcement_liens_sacrifice_tactique"
actives = ["lien_de_vie_partagee", "rappel_urgence", "invocation_supreme"]
passives = ["reseau_conscient", "dernier_souffle"]

# =============================================================================
# CLASSE 5 — LEYANTHAR (Gardien des Ley Lines)
# =============================================================================

[class.leyanthar]
id = "leyanthar"
display_name = "Leyanthar"
lore_name = "Gardien des Ley Lines"
faction = "federation_ervan"
role = "tank_magic"
weapon_types = ["bouclier_cristal_ley", "marteau_rune_nexus", "brassards_conducteurs"]
skill_trees = ["absorption_de_nexus", "boucliers_allies", "contre_flux"]
base_stats = { ferrath = 18, velthar = 12, gaiathar = 22, rhathar = 40 }
core_mechanic = "capacitor_de_ley"
resource_type = "mana_capacitor"  # mana + Capacitor (0-100)
color = "#1565C0"
capacitor_max = 100
capacitor_charge_rate = 1  # par 1% PV max perdu
signature_skills = ["ancrage_au_nexus", "eclat_du_ley_primordial"]

[class.leyanthar.skill_tree.absorption_de_nexus]
id = "absorption_de_nexus"
theme = "reduction_degats_conversion_capacitor"
actives = ["bouclier_cristal_actif", "absorption_de_ley", "mur_de_nexus"]
passives = ["corps_conducteur", "peau_pierre_ley"]

[class.leyanthar.skill_tree.boucliers_allies]
id = "boucliers_allies"
theme = "protection_allies_transfert_energie"
actives = ["transfert_de_nexus", "aura_de_cohesion", "rempart_partage"]
passives = ["genereux_conducteur", "reseau_de_cristaux"]

[class.leyanthar.skill_tree.contre_flux]
id = "contre_flux"
theme = "decharges_offensives_contre_attaques"
actives = ["decharge_de_nexus", "nova_de_cristal", "contre_flux_absolu"]
passives = ["economie_de_flux", "sur_saturation"]

# =============================================================================
# CLASSE 6 — SOLTHARETH (Sorcière de Combat)
# =============================================================================

[class.solthareth]
id = "solthareth"
display_name = "Solthareth"
lore_name = "Sorcière de Combat"
faction = "federation_ervan"
role = "dps_nuke_debuff"
weapon_types = ["baguette_tordue_chene_eclair", "grimoire_maledictions_lié", "anneaux_de_mana"]
skill_trees = ["nukes_fulgurants", "maledictions_tissees", "chaos_de_gaia"]
base_stats = { ferrath = 8, velthar = 22, gaiathar = 38, rhathar = 12 }
core_mechanic = "jauge_de_chaos"
resource_type = "mana"
color = "#6A1B9A"
chaos_max = 10
frenzee_threshold = 10  # charges pour Frénésie de Solthar
grimoire_flying = true  # mécanique de Grimoire Volant autonome
signature_skills = ["sabbat_de_solthar", "malediction_de_garum"]

[class.solthareth.skill_tree.nukes_fulgurants]
id = "nukes_fulgurants"
theme = "degats_instantanes_rapides_succession"
actives = ["eclair_maudit", "noirceur_de_nexus", "detonation_de_malediction"]
passives = ["immediateté_chaotique", "economie_du_mal"]

[class.solthareth.skill_tree.maledictions_tissees]
id = "maledictions_tissees"
theme = "debuffs_lourds_longue_duree"
actives = ["malediction_de_lenteur", "fleau_de_visions", "malediction_epuisement"]
passives = ["persistance_du_mal", "propagation"]

[class.solthareth.skill_tree.chaos_de_gaia]
id = "chaos_de_gaia"
theme = "effets_aleatoires_controles_panique"
actives = ["sort_du_destin", "eclat_de_panique", "instabilite_gaienne"]
passives = ["maitrise_du_desordre", "frenzee_prolongee"]

# =============================================================================
# CLASSE 7 — VELTHAMUR (Chamane)
# =============================================================================

[class.velthamur]
id = "velthamur"
display_name = "Velthamur"
lore_name = "Chamane"
faction = "federation_ervan"
role = "support_heal_ancestral"
weapon_types = ["hochet_os_cristal_ley", "houlette_de_brume_ancestrale", "masque_de_lancien"]
skill_trees = ["esprits_soigneurs", "resurrection_et_passage", "vision_du_passe"]
base_stats = { ferrath = 10, velthar = 16, gaiathar = 34, rhathar = 26 }
core_mechanic = "echo_ancestral"
resource_type = "mana"
color = "#607D8B"
max_esprits_actifs = 3
echo_ancestral_max = 3
signature_skills = ["concile_des_anciens", "testament_du_premier_druide"]

[class.velthamur.spirits]
guerisseur = { specialite = "soin_auto_cible_critique", autonomie = "haute" }
gardien = { specialite = "prevention_mort_temporaire", autonomie = "moyenne" }
guerrier = { specialite = "dps_et_absorption_partielle", autonomie = "faible" }

[class.velthamur.skill_tree.esprits_soigneurs]
id = "esprits_soigneurs"
theme = "soins_via_esprits_purification"
actives = ["appel_esprit_guerisseur", "benediction_ancestrale", "cercle_de_purification"]
passives = ["memoire_de_soin", "presence_apaisante"]

[class.velthamur.skill_tree.resurrection_et_passage]
id = "resurrection_et_passage"
theme = "resurrection_forme_spectrale_mort"
actives = ["rite_de_retour", "presence_esprit_gardien", "traversee_des_voiles"]
passives = ["voile_mince", "souvenir_des_morts"]

[class.velthamur.skill_tree.vision_du_passe]
id = "vision_du_passe"
theme = "information_passee_anticipation_connaissance"
actives = ["regard_dans_le_passe", "murmures_des_anciens", "echo_de_bataille"]
passives = ["memoire_vivante", "prescience_gaienne"]

# =============================================================================
# CLASSE 8 — RHATHARVEN (Tisserand de Sorts)
# =============================================================================

[class.rhatharven]
id = "rhatharven"
display_name = "Rhatharven"
lore_name = "Tisserand de Sorts"
faction = "federation_ervan"
role = "hybrid_synergy"
weapon_types = ["gants_de_trame_ley", "orbe_tisse", "aiguilles_runiques"]
skill_trees = ["enchantements_offensifs", "enchantements_defensifs", "synergies_autonomes"]
base_stats = { ferrath = 12, velthar = 22, gaiathar = 30, rhathar = 18 }
core_mechanic = "trame_active"
resource_type = "mana"
color = "#FFD700"
fils_max_alliés = 3
fils_max_ennemis = 3
signature_skills = ["grand_tissu_de_gaia", "contre_tissu"]

[class.rhatharven.fil_interaction]
# Deux fils qui touchent le même événement de combat créent une réaction de Trame
reaction_threshold = 2
reaction_bonus_dmg = 0.25

[class.rhatharven.skill_tree.enchantements_offensifs]
id = "enchantements_offensifs"
theme = "amplification_degats_multiplication_effets"
actives = ["fil_amplification", "tresse_explosive", "resonance_tissee"]
passives = ["maitrise_de_trame", "echo_de_tissu"]

[class.rhatharven.skill_tree.enchantements_defensifs]
id = "enchantements_defensifs"
theme = "protection_boucliers_redistribution"
actives = ["fil_de_deviation", "cocon_de_trame", "redistribution"]
passives = ["trame_renforcee", "contrepoint"]

[class.rhatharven.skill_tree.synergies_autonomes]
id = "synergies_autonomes"
theme = "systemes_auto_entretenu_combos_passifs"
actives = ["trame_vivante", "noeud_de_convergence", "demontage"]
passives = ["intelligence_du_tissu", "perpetuation"]

# =============================================================================
# CLASSE 9 — UMBRANTAL (Veilleur)
# =============================================================================

[class.umbrantal]
id = "umbrantal"
display_name = "Umbrantal"
lore_name = "Veilleur"
faction = "federation_ervan"
role = "stealth_magic_sabotage"
weapon_types = ["stylet_runique", "cape_effacement_ley", "medaillon_de_ley"]
skill_trees = ["silence_gaïen", "sabotage_des_ley", "frappe_dans_l_ombre"]
base_stats = { ferrath = 16, velthar = 32, gaiathar = 26, rhathar = 14 }
core_mechanic = "empreinte_de_gaia"
resource_type = "mana"
color = "#1A3A1A"
empreinte_max = 100
empreinte_decay_stationary = 10  # par seconde
empreinte_decay_moving_slow = 5  # par seconde en mouvement lent
detection_threshold_magic = 60  # les mages détectent au-dessus de 60
detection_threshold_visual = 100  # visibilité visuelle complète à 100
signature_skills = ["disparition_gaïenne", "demantelement_noeud_corrompu"]

[class.umbrantal.skill_tree.silence_gaïen]
id = "silence_gaïen"
theme = "invisibilite_reduction_empreinte_silence"
actives = ["effacement_de_signature", "pas_de_brume", "impulsion_d_oubli"]
passives = ["silence_interieur", "ombre_de_gaia"]

[class.umbrantal.skill_tree.sabotage_des_ley]
id = "sabotage_des_ley"
theme = "ley_corrompues_demantelement_sabotage"
actives = ["purge_de_ley", "court_circuit_de_ley", "pose_rune_sabotage"]
passives = ["lecteur_de_flux", "mains_legeres"]

[class.umbrantal.skill_tree.frappe_dans_l_ombre]
id = "frappe_dans_l_ombre"
theme = "attaques_furtives_malefices_runiques_extraction"
actives = ["piqure_de_rune", "sabotage_de_mana", "extraction"]
passives = ["lame_silencieuse", "chirurgien_de_ley"]

# =============================================================================
# SYNERGIES DE FACTION
# =============================================================================

[faction.federation_ervan.bonus_passive.affinite_gaienne]
mana_regen_bonus = 0.15
conditions = ["forest_biome", "nexus_50m", "ley_line_active"]

[faction.federation_ervan.bonus_passive.lecture_du_vivant]
garum_warning_advance = 2.0  # secondes d'avance avant effet

[faction.federation_ervan.bonus_passive.lien_communautaire]
players_required = 3
radius = 20
all_damage_resist_bonus = 0.05

[faction.federation_ervan.bonus_passive.memoire_de_gaia]
healing_consumable_bonus = 0.10

[faction.federation_ervan.compositions]
triade_de_guerison = ["ferventhar", "velthamur", "nytharven"]
lance_elementaire = ["gaiathar", "velsorath", "rhatharven", "solthareth", "leyanthar"]
forteresse_vivante = ["leyanthar", "ferventhar", "velthamur", "nytharven", "nytharven"]
ombre_et_tempete = ["umbrantal", "solthareth", "velsorath", "velthamur", "rhatharven"]
```

---

*Document généré pour Allumina — MMO-ARPG sur MGE. Véranthas, An 247 AO.*
*Référence canonique v1.0 — Fédération Ervan — 9 classes jouables.*
