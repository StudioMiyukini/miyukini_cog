<!-- @id: AL-World-Events @do: reference @role: game-designer @layer: 3 @human: miyuk -->

# AL-World-Events — Allumina : Evenements Monde, Campagnes et Systemes Sociaux

**Statut :** Reference de design verrouillee
**Version :** 1.0
**Date :** 2026-02-28
**Scope :** Campagnes 1-30, Garum, Aventuriers, Mercenaires, Outlaws, Donjons, Evenements

---

## Table des matieres

1. [Vision d'ensemble](#1-vision-densemble)
2. [Les 5 Campagnes](#2-les-5-campagnes)
   - 2.1 [Campagne Imperiale — Empire Pourpre](#21-campagne-imperiale--empire-pourpre)
   - 2.2 [Campagne de Rive — Alliance de Rive](#22-campagne-de-rive--alliance-de-rive)
   - 2.3 [Campagne Ervan — Federation Ervan](#23-campagne-ervan--federation-ervan)
   - 2.4 [Campagne des Bas-fonds — Outlaws](#24-campagne-des-bas-fonds--outlaws)
   - 2.5 [Convergence — Zone Partagee 30+](#25-convergence--zone-partagee-30)
3. [Systeme Garum](#3-systeme-garum)
4. [Guilde des Aventuriers](#4-guilde-des-aventuriers)
5. [Guilde des Mercenaires](#5-guilde-des-mercenaires)
6. [Systeme Outlaw](#6-systeme-outlaw)
7. [Donjons et Instances](#7-donjons-et-instances)
8. [Evenements Monde Recurrents](#8-evenements-monde-recurrents)
9. [Schemas TOML](#9-schemas-toml)

---

## 1. Vision d'ensemble

Allumina est un **MMO-ARPG open world** dans un univers medieval-fantastique traverse par la menace du **Garum** — une corruption primordiale qui devore les zones, corrompt les creatures et deborde en surface lors des **Stampedes**. Le monde est ouvert des le niveau 1 : tout joueur peut aller n'importe ou, mais les dangers sont reels.

### Tensions fondamentales

| Tension | Description |
|---------|-------------|
| Factions vs Garum | Chaque faction combat le Garum a sa maniere — militairement, commercialement, magiquement, ou de l'ombre |
| Factions entre elles | Guerres de territoire (RvR en Convergence), tensions diplomatiques, mercenaires qui changent de camp |
| Joueur vs Monde | Progression 1-30 dans une campagne, puis fusion en Convergence pour l'endgame |
| Legitimite vs Crime | Outlaws vivent dans les bas-fonds, peuvent se racheter ou rester dans l'ombre |

### Principes de design

- **Pas de serveur "lobby" cache** : le monde est MMO des le debut, d'autres joueurs sont visibles dans toutes les zones
- **Campagnes paralleles** : 5 histoires qui convergent — les joueurs de factions differentes se croisent dans le monde ouvert meme avant la Convergence
- **Le Garum est une horloge** : si les joueurs ignorent la corruption, les zones degenerent jusqu'au Stampede
- **Cooperation forcee lors des Stampedes** : meme les ennemis jures s'allient quand les abysses debordent

---

## 2. Les 5 Campagnes

### Decision architecturale verrouillee

Chaque joueur choisit sa campagne a la creation de personnage. Cette campagne determine :
- Sa faction d'origine
- Les zones qu'il decouvre en 1-30
- Les NPCs avec qui il interagit
- Sa relation de depart avec les autres factions (neutre, alliee, hostile)
- Sa porte d'entree vers la Convergence (niveau 30)

Les joueurs de factions differentes se voient dans le monde ouvert mais n'ont pas acces aux memes quetes de campagne, bases ou batiments de faction.

---

### 2.1 Campagne Imperiale — Empire Pourpre

#### Narrative d'ensemble

L'Empire Pourpre est la puissance militaire dominante du continent. Son armee disciplinee, la **Legion de Cramoisy**, tient des centaines de kilometres de front contre les incursions Garum. La campagne suit un soldat ou un aspirant officier qui monte en grade en defendant les lignes imperiales, decouvrant que certains generaux corrompus alimentent secretement le Garum pour maintenir l'etat de guerre permanent — et donc leur pouvoir.

**Enjeu niveaux 1-30 :** Defendre les avant-postes, purger les traitres dans les rangs, et empecher que la **Forteresse de Varenkor** — derniere grande place forte imperiale proche des terres corrompues — ne tombe.

**Antagoniste principal :** Le General **Draeven Mort-Rouge**, qui a conclu un pacte avec les Chronolites du Garum pour rester immortel tant que la guerre dure.

#### Zones de la Campagne Imperiale

| # | Nom | Type | Niveaux | Ambiance |
|---|-----|------|---------|----------|
| 1 | Plaine de Recrutement | Zone d'introduction | 1-5 | Prairie militarisee, camps d'entrainement, premieres escarmouches |
| 2 | Cite de Varenkor | Hub principal | 1-30 | Grande ville imperiale, marchands, casernes, quartier general |
| 3 | Le Col des Boucliers | Zone de combat | 5-10 | Defilé montagneux, fortifications en ruine, sniper Garum |
| 4 | Marais de Sel | Zone dangereuse | 10-16 | Marais empoisonnes, corruption naissante, mercenaires deserteurs |
| 5 | Ruines de Calveran | Zone d'exploration | 14-20 | Ancienne ville imperiale tombee, fantomes, archives secretes |
| 6 | Front de Vareth | Zone de guerre active | 18-24 | Tranchees, catapultes, batailles NPC vs Garum en temps reel |
| 7 | Tour du Veilleur | Zone isolee | 22-27 | Tour magique, mages imperiaux assieges, Chronolites |
| 8 | Citadelle de Varenkor | Zone finale campagne | 26-30 | Revelation de la trahison, combat final Draeven |

#### Zones — detail

**Zone 1 — Plaine de Recrutement (niveaux 1-5)**

Ambiance : Herbe jaune brulee par les campements militaires, tentes alignees, soldats qui s'entrainent. Ciel couvert, odeur de fer.

Ennemis : Eclaireurs Garum primitifs (Rodeurs des Cendres), Loups corrompus, Deserteurs hostiles

Ressources : Fer brut, Herbes medicinales, Cuir de loup

Waypoint : Camp du Sergent Halvord (structure permanente)

Boss de zone : **Centurion Renegat Brek** — ancien officier corrompu, armor lourde, charge au bouclier, invoque des soldats-spectres. 3 phases de combat, la troisieme declenche une AoE corruption sur toute l'arene.

NPCs notables :
- **Sergent Halvord Pierrefer** : quetes d'introduction, achete equipement basique, mentor militaire implacable
- **Medecin de camp Sora** : soins, vend potions de soin et anti-poisons niveau 1
- **Armurier Ton le Borgne** : forge niveau 1-5, repare l'equipement, propose recettes de base

**Zone 3 — Col des Boucliers (niveaux 5-10)**

Ambiance : Defilé montagneux etroit, neige permanente sur les sommets, fortifications de bois et pierre en etat de siege. Nuit perpetuelle dans les gorges.

Ennemis : Archers Garum (Tireurs de Cendre), Trolls corrompus, Gargoyles de pierre noire, Eclaireurs ailes

Ressources : Pierre de voute (minerai rare), Fourrure de troll, Cristaux de givre

Waypoint : Fort Intermediaire Halvane

Boss de zone : **Grendax le Troue** — troll geant dont le dos porte une bouche secondaire Garum, regeneration rapide, crache acide corrosif. La bouche secondaire doit etre fermee (mecanique d'interruption) pour stopper la regeneration.

Mecanique speciale : Le col se retrecit en certains points. Evenement dynamique toutes les 2 heures : des vagues NPC alliees tentent de traverser — les joueurs doivent tenir des positions defensives pour les proteger. Recompense bonus selon le taux de survie des NPC.

**Zone 4 — Marais de Sel (niveaux 10-16)**

Ambiance : Marais a eau noire et salee, vapeurs toxiques, corruption naissante (stade 1-2). Arbres morts tordus. Sons de creatures invisibles.

Ennemis : Hydres a deux tetes corrompues, Fantomes de soldats perdus, Crabes des profondeurs mutes, Mercenaires deserteurs hostiles

Ressources : Sel noir (alchimie), Vase de corruption (materiau purification), Herbes rares sous-marines

Waypoint : Poste abandonne de Marken

Boss de zone : **La Noyee Eternelle** — officiere imperiale tombee dans le marais, transformee en wight aquatique, controle les eaux stagnantes, drain de mana. Si le joueur a trouve son journal (quete secondaire), la boss peut etre liberee plutot que tuee.

Quete secrete : Retrouver le journal de l'officiere avant que la corruption l'efface definitivement (fenetre de 72h reel apres spawn)

**Zone 5 — Ruines de Calveran (niveaux 14-20)**

Ambiance : Ancienne grande ville imperiale detruite lors d'une invasion Garum il y a 200 ans. Batiments en ruine effondres, bibliotheque partiellement intacte, fantomes errants.

Ennemis : Spectres legionnaires, Golems de pierre animes, Archivistes zombifies, Collecteurs Garum (recuperent les artefacts pour les corrompre)

Ressources : Parchemins anciens (lore + recettes), Pierre calcinee, Reliques imperiales (vente haute valeur)

Waypoint : Place Centrale de Calveran (activable apres avoir nettoye le secteur central)

Boss de zone : **Archonte Valem** — mage imperial mort refusant de lacher ses archives, lance des sorts de niveau 20, peut projeter les joueurs dans un espace d'echo temporel ou les stats sont inversees pendant 15 secondes.

Revelation narrative : Les archives revelent que le premier pacte avec le Garum a ete signe par un ancetre de Draeven Mort-Rouge. La guerre n'est pas accidentelle.

**Zone 6 — Front de Vareth (niveaux 18-24)**

Ambiance : Zone de guerre active en temps reel. Des batailles entre soldats NPC imperiaux et armees Garum se deroulent en permanence. Le terrain evolue selon l'issue des combats — si le front recule, le waypoint se deplace.

Ennemis : Unites Garum de milieu de gamme (Chevaliers de Cendre, Mages corrompus), Deserteurs reanimes, Balistes Garum

Ressources : Acier de guerre, Tissu de banniere, Munitions magiques

Waypoint : Quartier General Mobile (se deplace selon le front — mecanique unique au monde)

Boss de zone : **Commandant Graeus la Lame Noire** — capitaine Garum humanoide, tactiques militaires reelles, appelle des renforts via un signal de cor corrompu. Doit etre interrompu avant le 3e appel sinon 4 renforts d'elite arrivent.

Mecanique front dynamique : Sans participation des joueurs pendant 4h, le front recule. Avec forte participation, il avance et debloque des zones de ressources bonus.

**Zone 7 — Tour du Veilleur (niveaux 22-27)**

Ambiance : Tour magique ancienne, isolee dans une mer de corruption (stade 3). Les mages de l'Ordre du Veilleur y sont assieges depuis 3 semaines. L'acces est par teleportation depuis Varenkor.

Ennemis : Demons Garum superieurs (Fracasseurs de vide), Mages corrompus, Sentinelles spectrales, Chronolites (premiers contacts)

Ressources : Cristaux de memoire (crafting magie), Encres runiques, Essences elementaires purifiees

Waypoint : Plate-forme de teleportation au sommet

Boss de zone : **Chronolite Observateur Zeth** — premier vrai Chronolite rencontre dans la campagne, peut ralentir le temps pour lui-meme uniquement, attaques de desynchronisation temporelle qui decalent les animations de frappe des joueurs.

Revelation narrative : Les Veilleurs ont des preuves du pacte de Draeven — mais ils doivent etre secourus pour les transmettre.

**Zone 8 — Citadelle de Varenkor (niveaux 26-30)**

Ambiance : Coeur militaire de la cite, revele comme corrompu de l'interieur. Pierre pourpre noircie de l'interieur, corridors ou des soldats se retournent contre leurs freres, archives brulees par Draeven.

Ennemis : Garde Pretorienne corrompue, Lieutenants Garum deguises en officiers, Marionnettes de chair (anciens soldats morts reanimes en serviteurs)

Boss final de campagne : **General Draeven Mort-Rouge** — combat en 3 phases :
1. Humain augmente Garum (combat de boss classique, attaques de sabre + aura de corruption)
2. Mi-transformation Chronolite (ailes de cendre, attaques de zone temporelle)
3. Forme corrompue complete (invulnerabilite partielle — 4 cristaux a purifier pendant le combat)

Recompense narrative : Acces au Portail de Convergence Imperial, titre "Defenseur de Varenkor", set d'equipement "Acier Pourpre" niveau 30.

#### Arc narratif Imperial

Le joueur revele la trahison de Draeven devant le Gouverneur et les officiers loyaux. Apres la defaite, l'Empire se retrouve decapite militairement — le Gouverneur doit reorganiser la Legion et reconnait que la menace Garum depasse les capacites d'une seule faction. **Le Portail de Convergence** s'ouvre avec une invitation formelle au niveau 30.

---

### 2.2 Campagne de Rive — Alliance de Rive

#### Narrative d'ensemble

L'Alliance de Rive est une confederation de cites portuaires, guildes marchandes et republiques maritimes unies par l'interet economique. Pas d'armee unifiee permanente : chaque cite contribue selon ses moyens. La campagne suit un aventurier des docks qui devient agent de l'Alliance, decouvrant que les routes commerciales sont sabotees par la **Guilde Noire** — une organisation criminelle qui trafique avec le Garum, echangeant des vivants contre des artefacts de puissance.

**Enjeu niveaux 1-30 :** Securiser les routes maritimes, demasquer la Guilde Noire avant qu'elle ne vende assez d'ames pour invoquer un Seigneur Garum sur le continent.

**Antagoniste principale :** **Marchande Sylvara Eaux-Profondes**, presidente secrete de la Guilde Noire, en apparence philanthrope et presidente du Conseil Commercial de Marenne.

#### Zones de la Campagne de Rive

| # | Nom | Type | Niveaux | Ambiance |
|---|-----|------|---------|----------|
| 1 | Les Docks de Marenne | Zone d'introduction | 1-5 | Ville portuaire animee, marches bruyants, bas-fonds visibles |
| 2 | Marenne | Hub principal | 1-30 | Grande cite portuaire, tension sociale riche/pauvre |
| 3 | Iles Brisees | Zone maritime | 5-10 | Archipel rocheux, grottes marines, pirates |
| 4 | Foret des Courants | Zone terrestre | 10-15 | Foret cotiere dense, routes commerciales, bandits |
| 5 | Cite de Sel | Zone commerciale corrompue | 14-20 | Cite miniere en declin, corruption sociale + Garum naissant |
| 6 | Abysses Cotieres | Zone maritime profonde | 18-24 | Grottes sous-marines, monstres des profondeurs, bases secretes |
| 7 | Le Phare de l'Oubli | Zone isolee | 22-27 | Phare sur ile, observatoire, porte dimensionnelle cachee |
| 8 | Entrepots Centraux de Marenne | Zone finale | 26-30 | Reseau souterrain de la Guilde Noire, combat final Sylvara |

#### NPCs importants — Marenne (hub)

- **Consul Paret Eaux-Vives** : dirigeant officiel de Marenne, deborde, donne quetes politiques
- **Agente Loria** : espionne de l'Alliance, donne les missions d'infiltration, amoureuse de Sylvara (arc tragique)
- **Sylvara Eaux-Profondes** : presente des le debut comme bienfaitrice, antagoniste masquee
- **Vieux Marin Tark** : lore maritime, cartes de zones cachees, a vu trop de choses pour etre naif
- **Crieur Fen** : informations sur les evenements du serveur en temps reel — prix marches, alertes pirates, avis de tempete

#### Boss de zone — Iles Brisees

**Amiral Corven le Naufrageur** — capitaine pirate sur une fregate fantome. Combat naval (premiere phase en barque avec esquive de bordees de canon) puis abordage (combat corps a corps). Invoque une tempete locale reduisant la visibilite.

#### Boss de zone — Abysses Cotieres

**Mere des Profondeurs** — creature Garum ancienne adaptee au milieu marin, masse tentaculaire, attaques d'encre de corruption (aveugle + debuff de vitesse), zone de combat qui s'inonde progressivement. Les joueurs doivent monter sur des plates-formes pour ne pas se noyer.

#### Boss final — Sylvara Eaux-Profondes

Trois phases :
1. Mage de haut niveau avec illusions parfaites (6 copies, il faut identifier la vraie)
2. Invocation partielle d'un Seigneur Garum — elle le controle au depart
3. Perte de controle — le Seigneur se retourne contre elle et les joueurs doivent tuer les deux avant que la fusion soit complete (timer 3 minutes)

Recompense : Titre "Demasqueur", set "Cotte de Brume Marine", Portail de Convergence de Rive.

#### Arc narratif Rive

La Guilde Noire est demantele mais le Consul Paret revele que Sylvara n'etait pas seule — un reseau de marchands a travers tout le continent alimente le Garum. La menace est systemique. Le joueur recoit des lettres d'introduction pour les autres factions et acces a la Convergence.

---

### 2.3 Campagne Ervan — Federation Ervan

#### Narrative d'ensemble

La Federation Ervan est un reseau de villages, sanctuaires et communautes de druides, mages naturels et gardiens forestiers. Pas d'armee centrale — des milices locales et des cercles magiques. La campagne suit un apprenti gardien qui decouvre que les **sanctuaires de Gaïa** sont corrompus de l'interieur par des **Cultistes du Vide** collaborant avec le Garum pour detruire Gaïa et ouvrir un passage permanent pour les Chronolites.

**Enjeu niveaux 1-30 :** Proteger les sanctuaires de Gaïa, purifier les cercles druidiques corrompus, empecher l'activation du **Grand Autel du Vide** au coeur de la Foret Profonde.

**Antagoniste principale :** **Archdruidesse Vaela Cendres-Vives** — gardienne supreme des sanctuaires Ervan, secretement corrompue depuis 10 ans, a l'origine du lent empoisonnement des cercles.

#### Zones de la Campagne Ervan

| # | Nom | Type | Niveaux | Ambiance |
|---|-----|------|---------|----------|
| 1 | Lisiere de l'Eveil | Zone d'introduction | 1-5 | Oree de foret lumineuse, premier sanctuaire, apprentis druides |
| 2 | Village d'Erbael | Hub principal | 1-30 | Village forestier circulaire, grand arbre central, cercle de pierre |
| 3 | Ruines de Kervin | Zone d'exploration | 5-11 | Ancienne cite druidique effondree, gobelins, esprits perdus |
| 4 | Marecage du Crepuscule | Zone sombre | 10-16 | Marecage sous corruption stade 2, plantes carnivores, esprits corrompus |
| 5 | Sanctuaire de la Flamme Froide | Zone mystique | 14-20 | Temple de cristal, gardé par elementaux de lumiere |
| 6 | Foret Profonde | Zone hostile | 18-24 | Foret primordiale impenetrable, corruptions avancees, faune monstrueuse |
| 7 | Cercle des Anciens | Zone de revelation | 22-27 | Cercle de menhirs flottants, archives magiques vivantes |
| 8 | Grand Autel du Vide | Zone finale | 26-30 | Clairiere corrompue au coeur absolu de la Foret Profonde |

#### NPCs importants — Erbael (hub)

- **Archdruidesse Vaela** : presente comme guide bienveillante (masque son role), donne les quetes de campagne principale
- **Apprenti Druide Soren** : allie joueur, seul a avoir des doutes sur Vaela, arc tragique (decouvre que sa mere etait une Cultiste)
- **Ancienne Mira** : trop vieille pour agir, connait les vraies archives, revele des verites par allusions poetiques
- **Forgeron Naturel Bael** : crafting d'equipement druidique, materiaux organiques uniquement
- **Messager des Vents Kivo** : NPC oiseau-messager dote de parole, donne acces a des quetes secondaires distantes

#### Boss de zone — Sanctuaire de la Flamme Froide

**Elemental Brise Arex** — elemental de lumiere dont la flamme interieure a ete corrompue. Mecanique : endommager les 3 orbes de corruption qui tournent autour de lui sans toucher l'elemental lui-meme (attaques precises uniquement). Tuer l'elemental = echec de la quete.

#### Boss de zone — Foret Profonde

**Ent Ancien Gorravel** — ent milleniaire corrompu, geant de bois noir, invoque des racines pieges sur tout le terrain, appelle des lianes-lacets qui immobilisent. Doit etre brule en phase finale — mecanique de fioles incendiaires.

#### Boss final — Vaela Cendres-Vives

Trois phases :
1. Druidesse corrompue avec magie naturelle inversee (guerit les ennemis au lieu des allies, invoque des plantes hostiles)
2. Reception d'un fragment de Chronolite qui s'implante dans son corps — elle gagne des attaques temporelles
3. Forme hybride Garum-Druidesse : controle de vegetation corrompue sur toute l'arene. Mecanique cle : 4 plantes de Gaïa a maintenir vivantes dans les coins de l'arene pour limiter la puissance de Vaela. Si une plante meurt, Vaela gagne +25% de puissance.

Recompense : Titre "Sanctifie", set "Armure de Racines Purifiees", Portail de Convergence Ervan.

#### Arc narratif Ervan

L'Autel est detruit mais le mal qu'il a seme persiste — des zones de la Federation sont corrompues au stade 3-4. Soren, l'apprenti, prend la tete de la reconstruction. La route vers la Convergence s'ouvre avec un message d'Erbael : la menace Garum reclame une unite que la Federation seule ne peut pas offrir.

---

### 2.4 Campagne des Bas-fonds — Outlaws

#### Narrative d'ensemble

Les Outlaws n'ont pas de faction au sens traditionnel — ils sont des individus rejetes, criminalises ou simplement libres qui survivent dans les reseaux souterrains, les egouts et les zones interdites des grandes cites. La campagne suit quelqu'un qui tombe dans les bas-fonds (trahison, ruine, fuite de la justice) et apprend a survivre puis a prosperer dans ce monde parallele. La revelation progressive : les Outlaws ont un code d'honneur propre, et les vrais criminels sans honneur sont tout aussi dangereux que le Garum.

**Enjeu niveaux 1-30 :** Survivre, grimper dans la hierarchie criminelle, et contrecarrer le plan du **Baron de la Cendre** — un Outlaw de haut rang qui a vendu les reseaux souterrains au Garum comme corridors d'invasion.

**Antagoniste principal :** **Baron Veiss la Cendre** — parrain des reseaux souterrains de trois cites, corrompu par le Garum en echange de l'immortalite et de la richesse.

#### Zones de la Campagne des Bas-fonds

| # | Nom | Type | Niveaux | Ambiance |
|---|-----|------|---------|----------|
| 1 | Egouts de la Chute | Zone d'introduction | 1-5 | Egouts d'une grande cite, premier contact avec la pegre |
| 2 | Le Refuge (hub) | Hub principal | 1-30 | Caverne amenagee en ville souterraine, marche noir |
| 3 | Mines Abandonnees de Keth | Zone industrielle | 5-10 | Mines desaffectees, gangs rivaux, ressources precieuses |
| 4 | Les Catacombes | Zone mortuaire | 10-16 | Catacombes anciennes, morts-vivants, artefacts funebres |
| 5 | Sous-Varenkor | Zone urbaine souterraine | 14-20 | Reseau de tunnels sous la ville imperiale, espions, gardes |
| 6 | Cavernes de Cristal Noir | Zone naturelle | 18-24 | Cavernes profondes, cristaux Garum, danger extreme |
| 7 | La Bourse de l'Ombre | Zone commerciale | 22-27 | Marche noir souterrain geant, lieu de pouvoir criminal |
| 8 | Repaire du Baron | Zone finale | 26-30 | Palais souterrain de Veiss, confrontation finale |

#### Zones — detail

**Zone 1 — Egouts de la Chute (niveaux 1-5)**

Ambiance : Egouts d'une grande cite (non specifiquement Varenkor ou Marenne — une ville generique de la zone de convergence des trois factions). Couloirs de pierre humides, eaux usees, rats geants, graffitis de gangs.

Ennemis : Rats geants, Vagabonds hostiles, Gardes de la cite en patrouille (evitement), Serpents des egouts

Ressources : Dechets recycles (crafting improvise), Herbes des fissures (alchimie de base), Pieces perdues

Waypoint : Collecteur Principal (jonction des gros tunnels)

Boss de zone : **Grand Rat Carnassier Vex** — rat mutant de la taille d'un homme, venimeux, chef de colonie. Si tue proprement (sans declencher l'alarme des rats), la colonie devient passive.

Mécanique introductive : Stealth obligatoire pour eviter certaines patrouilles de gardes (apprentissage du systeme Outlaw)

**Zone 2 — Le Refuge (hub 1-30)**

Ambiance : Ancienne mine en partie effondree convertie en ville souterraine. Stalactites eclairees par des lampions colores. Marche, taverne, ateliers clandestins. Ambiance de survie collective avec une touche de liberte.

Services : Marche Noir (equivalents de tous les services officiels mais clandestins), Crocheteur Expert (cle de nombreuses zones secretes), Saigneur (medecin Outlaw), Forgeron de Fortune (repare et ameliore sans poser de questions), Bureau de Contrats Criminels

NPCs importants :
- **Doyenne Haska la Manche Unique** : cheffe du Refuge, gouverne par respect mutuel et pragmatisme, mentor du joueur
- **Informateur Blix** : vend des informations sur les autres factions, sur les gardes, sur les evenements du monde
- **Alchimiste Souterraine Yendra** : potions speciales Outlaw (invisibilite, amortisseur de bruit, resistances aux poisons)
- **Contacte de la Guilde des Aventuriers "Sparks"** : Outlaw reformee, fait le lien avec le systeme Aventurier
- **Baron Veiss la Cendre** (NPC apparence bienveillante jusqu'a la zone 6) : parrain apparent du Refuge, genereux, cache parfaitement sa corruption

**Zone 3 — Mines Abandonnees de Keth (niveaux 5-10)**

Ambiance : Mines desaffectees suite a un eboulement, maintenant disputees entre deux gangs rivaux : les Perceurs (specialises en vol de minerai) et les Ombres Vertes (specialises en contrebande). Tension maximale.

Ennemis : Membres des deux gangs (attaquent si reputation insuffisante), Creatures des profondeurs, Gardes prives corrompus

Ressources : Minerai brut (non transforme, valeur brute), Explosifs de minage abandonnes, Cartes de tunnels

Waypoint : Salle de Direction de Keth (neutralisee entre les deux gangs)

Boss de zone : **Contremaître Mordu** — ancien superviseur devenu chef des Perceurs, bras mecanique de recuperation de mine, attaques de forage

Mecanique de faction : Le joueur choisit de s'allier temporairement a l'un des deux gangs pour cette zone — affecte les dialogues et les ressources disponibles, pas la quete principale

**Zone 4 — Les Catacombes (niveaux 10-16)**

Ambiance : Reseau funeraire colossal sous plusieurs cites, vieux de 500 ans. Catacombes de style ossements disposes en motifs decoratifs, chapelles souterraines abandonnees, fosses communes. Premiere corruption Garum visible.

Ennemis : Squelettes de garde (protegent les tombes de nobles), Zombies recentement reanimes, Necromanciens errants, Premières unites Garum dans les niveaux profonds

Ressources : Reliques funeraires (vente haute valeur), Os renforcis (crafting), Huile des morts (alchimie sombre)

Waypoint : Chapelle du Dernier Repos (accessible apres nettoyage)

Boss de zone : **Roi Mort Elvar** — noble enterre avec ses tresors, reanime par la corruption Garum naissante. Combat royal : garde du corps de squelettes elite, attaque en deux temps (decouronne d'abord les gardes, puis le roi lui-meme)

Revelation : Les tunnels des catacombes sont exactement ceux que le Baron Veiss a vendus au Garum comme corridors. Premiere preuve.

**Zone 5 — Sous-Varenkor (niveaux 14-20)**

Ambiance : Reseau de tunnels sous la cité imperiale. Gardes de patrouille frequents, passages secrets vers des zones de la campagne imperiale, espions de toutes factions, salles de reunion secretes.

Ennemis : Gardes imperiaux de patrouille (evitement/stealth ou combat), Espions d'autres factions (hostiles), Agents de la Guilde Noire, Taupes Garum deguisees

Ressources : Informations volees (monnaie d'echange), Equipement de garde (deguisement), Cles de passages secrets

Waypoint : Sortie de secours Nord (tunnel cache)

Boss de zone : **Chef d'Espionnage Aldren** — espion imperial qui a decouvert le reseau Outlaw, veut le demanteler ET se vendre au baron Veiss. Double traite. Combat d'arene sous Varenkor avec temoins des deux factions (mecanique : ne pas laisser les temoins s'enfuir pour alerter).

Mecanique : Pour la premiere fois, des zones de la campagne imperiale sont visibles de loin dans les tunnels — les deux campagnes se croisent physiquement.

**Zone 6 — Cavernes de Cristal Noir (niveaux 18-24)**

Ambiance : Cavernes naturelles extremement profondes ou des cristaux de corruption Garum poussent comme de la vegetation. Lumiere violette permanente, chaleur oppressante, sons de pulsation. C'est ici que les corridors vendus par Veiss arrivent.

Ennemis : Unites Garum completes (pas seulement des avant-gardes), Cristaux vivants (stationnaires mais attaquent a portee), Guetteurs Garum (alerts, doivent etre elimines silencieusement)

Ressources : Cristal noir brut (artisanat rare, double tranchant), Substance purifiante de fissure, Minerai Garum traitable

Waypoint : Crevasse du Silence (zone hors portee des cristaux)

Boss de zone : **Cristalliseur Harkan** — unité Garum mi-cristal mi-humanoide, gardien du corridor principal. A ete un Outlaw il y a 5 ans — vendu par Veiss. Revelation qui solidifie la motivation du joueur.

**Zone 7 — La Bourse de l'Ombre (niveaux 22-27)**

Ambiance : La plus grande place de marche souterraine du continent — plusieurs centaines de metres carres sous une ancienne carriere. Stalactites couvertes de lampions. Marchands de 12 nationalites differentes. Tension permanente — tout le monde est arme et tout le monde commerce.

Ennemis : Usuriers qui font payer l'entree (combat ou diplomatie), Chasseurs de primes sur le joueur (si reputation Wanted elevee), Agents de Baron Veiss (surveillance)

Ressources : Tout ce qu'il existe dans le jeu (prix du marche noir, 150-300% du prix officiel)

Waypoint : Kiosque Central de la Bourse

Boss de zone : **Trio des Arbitres** — trois marchands qui font la loi a la Bourse. Veiss leur a ordonne de stopper le joueur. Combat de 1 contre 3, chaque arbitre a une specialite (magie, corps a corps, poison)

**Zone 8 — Repaire du Baron (niveaux 26-30)**

Ambiance : Palais souterrain de Veiss — luxe obscene dans les profondeurs. Marbre noir, torches en cristal Garum, gardes d'elite. En apparence calme. Sous les pieces de reception, des laboratoires de transformation Garum ou des Outlaws sont experimentes.

Ennemis : Garde Personnelle de Veiss (élite, equipment endgame), Experientes Garum (Outlaws mi-transformes), Lieutenants Garum en visite de "partenariat"

Boss final — **Baron Veiss la Cendre** :
- Phase 1 : Parrain humain augmente Garum — vitesse surhumaine, poisons de la meilleure qualite, declenche des pieges dans toute la salle
- Phase 2 : Corruption partielle — peau de cristal noir apparait, attaques Garum directes, transformation de la salle (murs couverts de cristaux offensifs)
- Phase 3 : Absorption d'une relique Garum qu'il gardait pour l'ultime recours — forme Seigneur Garum mineur. Durée limitée de la transformation (120 secondes) apres quoi il s'effondre — si le joueur ne l'a pas tue, Veiss explose et la zone devient inaccessible pour 48h

Recompense : Titre "Fantome de l'Ombre", set "Manteau de Cendres", acces au reseau Outlaw endgame, Portail de Convergence des Bas-fonds.

#### Arc narratif Outlaws

Veiss mort, la Bourse de l'Ombre se reorganise sous la direction de Haska. Les Outlaws se retrouvent sans ennemi clairement defini mais avec une dette envers le monde : ils ont vu le Garum de pres, ils savent ce qu'il fait. La Convergence leur offre une chance de se battre pour quelque chose de plus grand que leur survie — sans pour autant devenir "legaux".

---

### 2.5 Convergence — Zone Partagee 30+

#### Narrative d'ensemble

La Convergence n'est pas une cinquieme campagne lineaire — c'est la **zone endgame partagee** ou tous les joueurs de toutes les factions se retrouvent au niveau 30. Les campagnes ont ete des preparatifs ; la Convergence est la vraie guerre. Le Garum pousse ses lignes, les factions maintiennent des zones d'influence mais doivent cooperer, et les Chronolites lancent des offensives majeures.

**Enjeu continu :** Repousser l'invasion Garum, conquerir des territoires, activer des Sanctuaires de Gaïa, tuer des Seigneurs Garum.

#### Zones de la Convergence

| # | Nom | Type | Niveaux | Ambiance |
|---|-----|------|---------|----------|
| 1 | Plaine des Quatre Vents | Zone neutre centrale | 30-33 | Prairie de convergence, postes de toutes factions, premiere rencontre inter-factions |
| 2 | Ruines de la Cite Primordiale | Zone d'exploration | 31-35 | Cite avant les factions, archives de l'histoire du Garum |
| 3 | Front de la Brume | Zone de guerre RvR | 32-36 | Ligne de front permanente entre factions et Garum, changement selon participation |
| 4 | Sanctuaire Central de Gaïa | Zone de purification | 33-37 | Grand temple de Gaïa en etat de corruption partielle, quetes de purification |
| 5 | Labyrinthe de Cendre | Zone de donjon world boss | 34-38 | Zone corrompue stade 4-5, labyrinthe changeant, spawns de Seigneurs Garum |
| 6 | Citeaux des Chronolites | Zone finale narrative | 35-40 | Forteresse Garum semi-dimensionnelle, operations de raid |
| 7 | Abyme Ouverte | Zone de Stampede recurrant | 30-40 | Zone a corruption instable, Stampedes frequents, cooperation obligatoire |
| 8 | Havres de Faction | Zones de base | 30-40 | Base de chaque faction en Convergence, respectivement hostile aux autres |
| 9 | Marchands Nomades | Zone commerciale mobile | 30-40 | Caravane qui tourne entre toutes les zones, service inter-faction |
| 10 | Sanctuaire de l'Oublie | Zone secrete | 38-40 | Zone decouverte uniquement par quete, lore ultime du Garum |

#### Mecanique RvR en Convergence

Le **Front de la Brume** est une zone dont les limites changent en temps reel selon :
- Le nombre de joueurs actifs par faction dans la zone
- Les objectifs de capture (postes, autels, ponts) controles
- La progression de corruption Garum en arriere-plan

Si une faction domine le Front pendant 6 heures consecutives, elle gagne un bonus mondial de 12 heures pour tous ses membres : +10% XP, +15% gold, acces a des ressources rares.

Les Outlaws sont une faction valide en RvR — ils ont leur propre strategie de guerre (guerilla, sabotage, embuscades) plutot que des lignes de front.



---

## 3. Systeme Garum

### 3.1 Philosophie

Le Garum n est pas un mechant statique — c est une force ecologique hostile. Il se comporte comme une moisissure planetaire : il se repand la ou on ne fait pas attention, se contracte la ou on le combat, et explose quand on l ignore trop longtemps. Les joueurs sont collectivement responsables de l etat sanitaire du monde.

### 3.2 Corruption Passive — Les 5 Stades

Chaque zone du monde possede un attribut corruption_level de 0 a 5. Ce niveau monte automatiquement :

- Sans joueurs actifs dans la zone : +0.1 par heure
- Avec joueurs actifs mais sans purification : +0.05 par heure
- Avec quetes de purification completees : -0.2 par quete
- Avec rituel de sanctification majeur : -1.0 instantane (cooldown 24h par zone)

**Stade 0 — Zone Saine**
Couleurs normales, faune et flore naturelles. Aucun debuff, ressources standard disponibles.

**Stade 1 — Contamination Initiale**
Touches de violet/noir aux extremites. Effets : -5% regeneration vie hors combat. 20% des ennemis ont le tag [Corrompu]. Purification : 5 autels a activer (10 minutes). Alerte systeme : "Une etrange corruption commence a envahir [Zone]."

**Stade 2 — Presence Etablie**
La moitie de la zone decoloree, cristaux noirs au sol. Effets : -10% resistance magique, feux de camp a 50% efficacite. 50% des ennemis corrompus. Patrouilles Garum legeres (3-5 unites). Purification : 3 quetes enchainees (30 minutes solo). Alerte serveur : "La corruption s etend a [Zone]."

**Stade 3 — Infestation**
80% de la zone corrompue. Structures Garum emergent du sol. Effets : -20% resistances, vitesse -15%, services PNJs coupes. 80% des ennemis corrompus. Patrouilles lourdes. Purification : raid de zone requis (20 joueurs, 1h minimum). Risque Stampede : 10% par heure.

**Stade 4 — Dominance**
Zone meconnaissable. Architecture Garum. Ciel violace. Effets : -30% toutes les stats, pas de regeneration naturelle, invocations des joueurs corrompues apres 30 secondes. Purification impossible directement — il faut d abord un Stampede pour reduire a stade 3. Risque Stampede : 30% par heure.

**Stade 5 — Zone Perdue**
Zone entierement absorbee. Barrieres de cristal noir. Inaccessible normalement (acces uniquement Outlaw haut rang ou Aventurier Legendaire). Une zone perdue non purifiee en 48h pousse les zones adjacentes de +1 stade.

### 3.3 Armees Garum — 12 Types d Unites

**1. Rodeur des Cendres** — Eclaireur mobile, alerte rayon 20m si non tue silencieusement. Stades 1-2.

**2. Grognard de Vide** — Infanterie de base, charge directe, resist aux CC courts. Stades 1-2.

**3. Tisseuse de Cristal** — Support/piege, pose des pieges de cristal (ralentissent + degats). Tres dangereuse en groupe. Stades 1-2.

**4. Lamenteur** — Aura de debuff : -20% vitesse d attaque pour tous les joueurs dans rayon 15m. Ne combat pas directement. Priorite absolue. Stades 1-2.

**5. Chevalier de Cendre** — Infanterie lourde, bouclier indestructible de face. Attaquer de cote ou derriere. Vie elevee. Stades 2-3.

**6. Mage de la Fracture** — Artillerie magique, boules de corruption longue portee, zone 5m, slow. Vie tres faible. Stades 2-3.

**7. Harceleur Alie** — Creature ailee, plonge-attaque-retire, applique "Marque de Corruption" (attire les Garum proches sur la cible). Stades 2-3.

**8. Devoreur** — Tank quadrupede, avale les joueurs incapacites (5s CC), absorbe les soins de zone. Vie tres elevee. Stades 2-3.

**9. Chronolite Eclaireur** — Elite, deplacements discontinus, attaques desynchronisees (modele visuel 0.5s en avance sur hitbox), copie une competence de joueur recent. Stades 3-4.

**10. Tisseuse de Portail** — Invoque 2 Grognards toutes les 10 secondes, bouclier regenerant. Priorite absolue. Stades 3-4.

**11. Fracasseur de Vide** — Demolisseur de structures, golem 4 metres, AoE tremblement, vulnerabilite magie de lumiere. Stades 3-4.

**12. Seigneur Garum Mineur** — Boss de zone, 3-5 metres, capacites uniques, commande les unites proches (+15% stats rayon 30m). Stades 4-5.

### 3.4 Evenements Stampede

#### Conditions de Declenchement

- corruption_level >= 4 : 30% de chance par heure (automatique)
- Donjon dont la corruption interne atteint le maximum
- Rituel Garum complete par des Cultistes du Vide
- Echec de purification pendant 48h consecutives en stade 3+
- Declenchement manuel administrateur pour evenements speciaux

#### Phases d un Stampede

**Phase 0 — Pre-alerte (5 minutes avant)** : Terrain qui tremble, fissures violettes. Message monde : "Un Stampede approche a [Zone]. Toutes les factions sont appelees." PNJs civils fuient. Compte a rebours visible.

**Phase 1 — Alerte (0-15 minutes)** : 3-5 points d emergence actifs (marquees). Vagues legeres (70% Rodeurs/Grognards, 20% Chevaliers, 10% Mages). Objectif : detruire les points d emergence (5000 PV chacun). Interface commune cross-faction.

**Phase 2 — Escalade (15-35 minutes)** : Vagues lourdes depuis les points non detruits. Tisseuse de Portail au premier point intact. 2-3 points supplementaires s ouvrent. Canal de Guerre cross-faction automatique.

**Phase 3 — Boss Stampede (35-60 minutes)** : 3+ points detruits = boss affaibli (85%). 0-2 points = boss pleine puissance avec suite d elite.

**Phase 4 — Resolution** : Succes : -2 corruption instantane, Coffre Stampede, 500 rep, 50 Cristaux de Purification. Echec : zone stade 5, verrou 6h, corruption adjacente +1.

#### 6 Boss Stampede

**Boss 1 — Fracas-Vide, l Ouvreur** : Gigantesque (8m), zones de vide (physique inversee). A 30% PV : portail vers dimension miroir (5 joueurs y entrent pour son coeur). Recompense : Cristal de Fracas.

**Boss 2 — La Mere Chronolite** : Colossale (6m), annule 10 dernieres secondes d action d un joueur aleatoire toutes les 30s. A 50% PV : ralentit tout a 20% sauf elle-meme pendant 20s. Recompense : Eclat de Chronolite.

**Boss 3 — Warlord Ossueux Krevax** : Collecte les os des joueurs tues pendant le Stampede (+2% puissance par mort). Invoque des formes osseuses de joueurs tombes (40% stats). Recompense : Amulette des Os Heroiques.

**Boss 4 — Sylphide Corrompue Avel** : Controle meteo de l arene (tempete, eclairs, vent). A 60% PV : fenetre de 15s de DPS maximum. Recompense : Pierre de Tempete Apaisee.

**Boss 5 — Demi-Seigneur Narthak** : Resonance de Corruption (joueurs avec meme debuff subissent degats amplifies). A 40% PV : copie 3 joueurs aleatoires en Reflets. Recompense : Cle du Vide (instance 5 joueurs plan Garum).

**Boss 6 — Archiviste Garum Silthar** : Apres 30s d observation, utilise des competences specifiques au joueur. Phase speciale : annonce un sort — 10 secondes pour identifier et interrompre le bon parmi 3. Recompense : Parchemin de Memoire Garum.



---

## 4. Guilde des Aventuriers

### 4.1 Structure Organisationnelle

La Guilde des Aventuriers est une organisation **inter-faction et neutre**. Elle existe independamment de l Empire, de l Alliance, de la Federation et des Outlaws (qui ont leur propre acces via contacts). Son siege est en Convergence — la Tour d Aventure, batiment imposant visible de toutes les zones de convergence.

La Guilde mesure objectivement les exploits : pas de politique, pas de noblesse. Seuls les actes comptent.

**Mission officielle :** Reperencer les menaces du monde, coordonner des equipes capables d y repondre, et honorer ceux qui survivent.

**Mission reelle :** Maintenir un corps d elite independant capable d intervenir partout ou les factions sont trop divisees pour agir.

### 4.2 Les 6 Rangs

#### Rang 1 — Recrue

**Conditions d acces :** Inscription gratuite a tout Hall de Guilde des Aventuriers (disponible dans chaque hub de campagne). Niveau minimum : 1.

**Conditions de progression vers Rang 2 :**
- Tuer 10 monstres champions (nom en or) dans n importe quelle zone
- Completer 5 contrats de Guilde de niveau Basique
- Recolter 100 points d Aventure (PA)

**Privileges :**
- Acces au Hall de Guilde et au tableau de contrats niveau Basique
- Reduction de 5% sur les potions chez les marchands de Guilde
- Insigne de Recrue (cosmetique)

#### Rang 2 — Eclaireur

**Conditions de progression vers Rang 3 :**
- Tuer 5 boss de zone (boss nommes, non-instances)
- Completer 3 donjons groupe de niveau 10+ avec rang B minimum (classement de completion)
- Accumuler 500 PA totaux

**Privileges :**
- Acces aux contrats Intermediaires (meilleure recompense)
- Partage de la carte de Guilde (acces a des zones marquees par d autres Eclaireurs)
- Acces aux donjons exclusifs Guilde niveau 1 (Grotte des Premiers)
- Reduction de 10% sur la reparation d equipement

#### Rang 3 — Chasseur

**Conditions de progression vers Rang 4 :**
- Tuer 15 boss de zone dont au minimum 3 boss de la Convergence
- Completer 2 donjons groupe de niveau 25+ avec rang A minimum
- Participer a la repousse d un Stampede avec succes (au moins en Phase 1)
- Accumuler 2000 PA totaux

**Privileges :**
- Acces aux contrats Avances (dont contrats de chasse de boss)
- Monture exclusive Guilde niveau Chasseur (cheval gris argente avec embleme)
- Acces aux donjons exclusifs Guilde niveau 2 (Ruines Scellees)
- Acces au registre des monstres rares (localisation des champions legendaires)

#### Rang 4 — Vanguard

**Conditions de progression vers Rang 5 :**
- Tuer 5 boss de raid (instances de 10+ joueurs)
- Completer 1 donjon Mythique (difficulte maximale d instance)
- Participer a 3 Stampedes successifs avec succes
- Reussir 10 contrats de rang Avance avec rang S
- Accumuler 10 000 PA totaux

**Privileges :**
- Titre permanent "Vanguard de la Guilde"
- Acces aux contrats Elites (dont contrats de purification de zone)
- Acces aux donjons exclusifs Guilde niveau 3 (Sanctum des Epreuves)
- Armure cosmetique exclusive Vanguard (ensemble non statistique mais tres visible)
- Capacite de poser un Camp de Guilde dans les zones ouvertes (point de resurrection pour les allies)

#### Rang 5 — Heros

**Conditions de progression vers Rang 6 :**
- Tuer au moins 1 Seigneur Garum majeur (boss de monde rare)
- Completer integralite d un donjon Mythique en temps imparti (rang S time)
- Etre reconnu par 50 autres joueurs de rang 3+ via le systeme de Commendation (vote de respect)
- Participer a la purification complete d une zone stade 4 ou 5
- Accumuler 50 000 PA totaux

**Privileges :**
- Titre permanent "Heros de [Nom du joueur]" (visible dans tout le monde)
- Acces aux zones perdues (stade 5) via portail runique de Guilde
- Acces aux donjons exclusifs Guilde niveau 4 (Cryptes Oubliees)
- Bonus de 20% sur toutes les recompenses de Guilde
- Droit de vote au Conseil des Heros (decisions sur les contrats prioritaires de la saison)

#### Rang 6 — Legendaire

**Conditions d acces (rang ultime, tres rare) :**
- Etre classe dans le Top 100 du Hall of Fame de la saison courante
- Avoir participe a l elimination d au moins 3 Seigneurs Garum majeurs differents
- Avoir atteint rang S sur un donjon Mythique en solo (oui, en solo — mode Legendaire special)
- Avoir ete nomme par 5 membres du Conseil des Heros
- Etre en possession du "Sceau Legendaire" (drop ultra-rare de boss de Seigneur Garum majeur, 1% de chance)
- Accumuler 200 000 PA totaux

**Privileges (exclusifs) :**
- Titre permanent "Le Legendaire / La Legendaire [Nom]" (unique par saison — visible monde entier)
- Armure cosmetique unique Legendaire (design different chaque saison)
- Acces au 5e donjon exclusif Guilde (Donjon du Vide — seul endroit ou les Chronolites de haut rang apparaissent)
- Capacite unique : Appel d Urgence (peut convoquer 5 joueurs de rang 3+ a sa position une fois par 24h)
- Hall of Fame permanent avec leur nom grave (meme apres rotation de saison)

### 4.3 Systeme de Points d Aventure (PA)

Les PA sont gagnes par :

| Action | PA accordes |
|--------|-------------|
| Tuer un monstre normal | 1 PA |
| Tuer un champion (nom en or) | 10 PA |
| Tuer un boss de zone | 50 PA |
| Tuer un boss de donjon instance | 100 PA par boss |
| Completer un donjon (rang D) | 50 PA |
| Completer un donjon (rang C) | 100 PA |
| Completer un donjon (rang B) | 200 PA |
| Completer un donjon (rang A) | 350 PA |
| Completer un donjon (rang S) | 600 PA |
| Completer un donjon Mythique (rang S) | 2000 PA |
| Participation a un Stampede (succes) | 500 PA |
| Purification de zone (contribution significative) | 1000 PA |
| Contrat de Guilde complete (Basique) | 20 PA |
| Contrat de Guilde complete (Intermediaire) | 60 PA |
| Contrat de Guilde complete (Avance) | 150 PA |
| Contrat de Guilde complete (Elite) | 400 PA |

### 4.4 Types de Contrats de Guilde

**Contrats Basiques (disponibles des Recrue) :**
- Chasse de routine : Tuer X monstres d un type dans une zone precise
- Collecte : Rapporter X ressources d un type
- Escorte simple : Accompagner un PNJ de A a B (PNJ suit le joueur, pas de combat majeur)
- Exploration : Atteindre et activer un waypoint non decouvert
- Duree : 24h. Recompense : 50-150 gold, 20 PA.

**Contrats Intermediaires (disponibles des Eclaireur) :**
- Chasse de champion : Tuer un champion nomme specifique (spawn en zone ouverte)
- Purification mineure : Reduire la corruption d une zone cible de 1 stade
- Escorte de caravane : Proteger une caravane PNJ sur une route longue (ennemis attacks dynamiques)
- Donjon groupe : Completer un donjon specifique avec un rang minimum
- Duree : 48h. Recompense : 200-500 gold, 60 PA, materiau de crafting rare.

**Contrats Avances (disponibles des Chasseur) :**
- Chasse de boss de zone : Tuer un boss specifique en zone ouverte
- Stampede : Participer a la repousse d un Stampede dans une zone designee
- Collecte d artefact : Recuperer un artefact dans un donjon difficile (PNJ donne la localisation)
- Enquete de corruption : Identifier la source de corruption d une zone (quete d investigation)
- Duree : 72h. Recompense : 1000-3000 gold, 150 PA, equipement de rang zone +2.

**Contrats Elite (disponibles des Vanguard) :**
- Chasse de Seigneur : Localiser et eliminer un Seigneur Garum mineur
- Purification majeure : Mener la purification complete d une zone stade 3+
- Defi de Guilde : Completer un donjon Mythique dans un temps imparti
- Contrat de monde : Evenement unique declenche par la Guilde, implique 20+ joueurs
- Duree : 1 semaine. Recompense : 5000-15 000 gold, 400 PA, equipement de raid, cosmetique exclusif.

### 4.5 Donjons Exclusifs de la Guilde

**Niveau 1 — Grotte des Premiers (rang Eclaireur+)**
- Difficulte : Normale. Groupe de 3-5.
- Contenu : 3 salles + 1 boss final. Thematique "premiere exploration" — mechanique d apprentissage.
- Boss : Hydre Bicephale Gardienne Mirex — deux tetes independantes (tuez la tete gauche d abord sinon la tete droite regenere).
- Temps cible : 30 minutes. Recompense : PA + equipement niveau zone +1.

**Niveau 2 — Ruines Scellees (rang Chasseur+)**
- Difficulte : Difficile. Groupe de 5.
- Contenu : 5 salles, pieges anciens actifs, ennemis de faction Garum + ennemis naturels en conflit.
- Boss : Gardien Automate Arcane Vex-Prime — combat en 3 phases ou le gardien change de protocole (offensif/defensif/aleatoire).
- Temps cible : 45 minutes. Recompense : PA + composants de crafting avances.

**Niveau 3 — Sanctum des Epreuves (rang Vanguard+)**
- Difficulte : Tres difficile. Groupe de 5.
- Contenu : 6 salles, chaque salle a une "epreuve" specifique (combat aveugle, puzzle de deactivation, protection d objectif).
- Boss : Trio des Ombres — trois revenants d aventuriers legendaires morts, chacun avec un style different (guerrier/mage/rodeur). Doivent etre tues dans les 10 secondes les uns des autres sinon ils se ressuscitent mutuellement.
- Temps cible : 60 minutes. Recompense : PA + materiau de crafting legendaire rare.

**Niveau 4 — Cryptes Oubliees (rang Heros+)**
- Difficulte : Extreme. Groupe de 5, sante non regeneree entre les salles.
- Contenu : 8 salles, ennemis scales au niveau maximum des joueurs, objets trouvables dans les cryptes qui modifient les salles suivantes.
- Boss : La Memoire du Fondateur — echo magique du fondateur original de la Guilde, corrompu par le Garum. Combat ou les competences du joueur sont periodiquement copiees et retournees contre lui.
- Temps cible : 90 minutes. Recompense : PA + chance de drop Sceau Legendaire (composant rang 6).

**Niveau 5 — Donjon du Vide (rang Legendaire seulement)**
- Difficulte : Legendaire. Groupe de 5 Legendaires uniquement.
- Contenu : Donjon dimensionnel dans le plan Garum. Physique alteree, pas de waypoints, mort = expulsion du donjon.
- Boss : Chronolite Souverain Arvet — veritable Chronolite de rang superieur, peut reset l etat du donjon aux 20 premieres secondes une fois par combat, frappe dans deux dimensions simultanement.
- Aucun temps cible officiel. Recompense : Equipement cosmetique unique Legendaire, artefact special de saison.

### 4.6 Hall of Fame — Saison

Le Hall of Fame est un classement saisonnier (3 mois par saison) affiche en permanence dans toutes les villes. Il classe les joueurs par PA totaux accumules dans la saison.

**Categories du classement :**
- Classement general (tous PA)
- Classement Chasseur de Boss (PA de boss seulement)
- Classement Donjonneur (PA de donjons seulement)
- Classement Defender (PA de Stampede et purification)
- Classement Solo (PA accumules en solo uniquement)

**Recompenses de fin de saison :**
- Top 1 par categorie : Cosmetique unique "Champion de Saison [Numero]", 100 000 gold, titre permanent
- Top 10 : Cosmetique rare, 20 000 gold, titre de saison
- Top 100 : Acces au rang Legendaire (condition necessaire mais pas suffisante)
- Top 500 : Cosmetique commun de saison, 5 000 gold

---

## 5. Guilde des Mercenaires

### 5.1 Structure et Philosophie

La Guilde des Mercenaires est une organisation professionnelle de combattants a louer. Contrairement aux Aventuriers qui chassent les monstres, les Mercenaires operent au niveau des conflits entre factions. Ils sont embauches par les nations pour renforcer les armees, escorter des personnalites, ou chasser d autres humains.

**Principe fondamental :** Un contrat est sacre. Le briser est considere comme la pire infamie dans le monde du mercenariat — et active immediatement le statut Outlaw.

**Siege :** La Maison des Lames, batiment neutre present dans chaque grande cite. Architecture sobre et militaire.

### 5.2 Comment Rejoindre

- Niveau minimum : 10
- Pas de restriction de faction — n importe qui peut etre Mercenaire en parallele de sa campagne
- Inscription : Se rendre a la Maison des Lames, payer les frais d inscription (500 gold), passer le Test d Aptitude (combat contre un mannequin magique evalue sur : DPS, survivabilite, execution des ordres)
- Les Outlaws peuvent s inscrire sous une identite falsifiee (mecanique speciale de la campagne Outlaws)

### 5.3 Rangs Internes de la Guilde des Mercenaires

**Rang 1 — Lame Libre**
Acces aux contrats d escorte et de chasse basiques. Pas de contrats militaires.

**Rang 2 — Soldart**
Apres 10 contrats completes sans rupture. Acces aux contrats militaires legers (renfort d avant-poste).

**Rang 3 — Veteran**
Apres 30 contrats, dont 5 militaires. Acces aux contrats militaires majeurs (batailles de faction).

**Rang 4 — Commandeur**
Apres 75 contrats, dont 20 militaires, avec reputation de faction embaucheur > 500. Peut accepter des contrats de commandement (diriger un groupe de mercenaires NPC en plus de son equipe de joueurs).

**Rang 5 — Legende de Fer**
Moins de 10 personnes par saison. 200 contrats completes, aucun rompu, dont au moins 3 contrats de "guerre declaree" (duree 7+ jours). Bonus permanents (+15% gold sur tous les contrats) et acces a des contrats secretes de la Guilde.

### 5.4 Types de Contrats Mercenaires

#### Contrats d Escorte

L embaucheur (faction ou PNJ ou joueur) confie une cible a proteger. Le Mercenaire est responsable de la cible jusqu a la destination ou pendant une duree definie.

**Variables :**
- Cible : PNJ (simple) ou joueur VIP (complexe — le joueur cible peut attirer des PK)
- Route : Connue / Inconnue (necessite scouting)
- Duree : Trajet unique / Multi-jours
- Risque : Declare (embuscades connues) / Non-declare

**Mechanique :** Si la cible meurt (ou est volee pour les escortes de caravane), le contrat est echoue. Si le Mercenaire abandonne la cible, le contrat est rompu (statut Outlaw).

**Paiement :** 50% a la signature, 50% a la livraison. En cas d echec non-rupture (la cible est morte malgre les efforts) : 25% de paiement d honneur.

#### Contrats de Chasse

Le Mercenaire est engage pour eliminer une cible specifique. La cible peut etre un monstre, un boss, ou un joueur hostile (en zone PvP uniquement — impossible de contracter un assassinat en zone safe).

**Variables :**
- Type de cible : Monstre nomme / Boss de zone / Joueur (zone PvP only)
- Preuve requise : Tete de la cible (item drop) / Capture / Temoin
- Exclusivite : Contrat exclusif (le Mercenaire est le seul dessus) ou libre (premier arrive, premier servi)

**Paiement :** 100% a la preuve de completion.

#### Contrats Militaires

Le contrat militaire engage le Mercenaire dans un conflit de faction pour une duree determinee (X jours avec minimum 3 jours, maximum 30). Pendant cette duree, le Mercenaire combat sous la banniere de la faction embaucheur.

**Obligations contractuelles :**
- Participer a au moins 1 operation de faction par jour (siege, defense, escarmouche — defini par le contrat)
- Ne pas attaquer les allies de la faction embaucheur
- Ne pas fournir d informations a la faction adverse
- Respecter les ordres tactiques du commandant NPC de la faction (PNJ — les ordres sont simples : "tenir le poste X", "attaquer la position Y")

**Rupture de contrat militaire :**
- Attaquer un allie de la faction : Rupture immediate, statut Outlaw 7 jours, 2000 gold de penalite
- Fournir des infos a l ennemi (detecte via mechanique d espionnage in-game) : Rupture immediate, statut Outlaw 30 jours, bannissement de la Maison des Lames 3 mois
- Abandon (quitter la zone de guerre sans accord de la faction) : Rupture, penalite de 500 gold

**Paiement militaire :**
- Retainer quotidien (verse chaque jour) : varie selon rang Mercenaire et rang du conflit
- Prime de victoire (si la faction gagne l objectif de la semaine) : 200-500% du retainer quotidien
- Prime de survie (bonus si le Mercenaire survit integralite du contrat sans mort) : +25%

**Impact sur les guerres de nations :**
Les Mercenaires comptent comme forces supplementaires dans le systeme RvR. Une faction qui embauche 50+ Mercenaires de rang 3+ peut debloquer des avantages tactiques : acces a un point de capture supplementaire, bonus de +5% aux stats de toutes ses troupes dans la zone concernee. Une faction qui n a pas les fonds pour embaucher des Mercenaires est desavantagee face a une faction riche — mechanique economique deliberee.

### 5.5 Trahison et Consequences

**La Trahison** est l acte de rompre deliberement un contrat militaire pour rejoindre le camp adverse.

Consequences :
- Statut Outlaw immediate (30 jours minimum)
- Bannissement de la Maison des Lames (1 saison complete)
- Reputation de faction embaucheur a zero (ne peut plus etre embauche par cette faction pendant 6 mois)
- Les autres factions sont informees (systeme de diffusion NPC) — certaines refuseront d embaucher un Mercenaire avec historique de trahison

Pour racheter une trahison :
- Payer l amende de trahison (5x le total du contrat restant)
- Completer 5 contrats de rang Veteran sans rupture
- Obtenir une "Lettre de Pardon" de la faction trahie (implique une quete specifique aupres de cette faction)

---

## 6. Systeme Outlaw

### 6.1 Campagne des Bas-fonds — Zones detaillees

(Les 8 zones de la campagne Outlaws sont detaillees en section 2.4.)

### 6.2 Statut Outlaw — Comment on le devient

Un joueur devient Outlaw par :
- Tuer un joueur non-hostile en zone safe ou neutre (PK involontaire ou delibere)
- Voler un joueur (pickpocket reussi mais detecte)
- Crocheter une porte appartenant a un autre joueur
- Rompre un contrat de Mercenaire
- Attaquer un PNJ de faction en zone de cette faction
- Choisir la Campagne des Bas-fonds a la creation de personnage (statut Outlaw natif — mais avec protections supplementaires dans les zones de refuge)

### 6.3 Systeme de Niveau de Recherche (Wanted Level)

Les Outlaws ont un **Niveau de Recherche** de 0 a 5, equivalent d un casier judiciaire visuel.

**Niveau 0 — Propre** : Pas de statut Outlaw. Comportement normal dans toutes les zones.

**Niveau 1 — Suspect** : Resultant d un PK ou vol mineur. Duree : 1h reel. Les gardes NPC dans les villes sont en alerte — si le joueur entre en ville, ils l interrogent (dialogue de confrontation — peut mentir, payer une amende, ou fuir).

**Niveau 2 — Criminel** : 2-3 crimes. Duree : 4h reel. Les gardes attaquent a vue en zone de faction. Les joueurs peuvent signaler un Outlaw niveau 2 pour une prime (100 gold si un autre joueur le tue et rapporte la preuve).

**Niveau 3 — Hors-la-loi** : Crimes serieux ou repetes. Duree : 12h reel. Acces interdit aux villes de faction (portails bloques par barriere magique). Prime de 500 gold sur la tete. Des joueurs de rang Aventurier 2+ recoivent automatiquement les informations sur la localisation approximative.

**Niveau 4 — Traque** : Crimes graves (PK multiples, trahison de Mercenaire). Duree : 48h reel. Des equipes de chasseurs de primes NPC sont envoyees dans la zone du joueur toutes les 30 minutes (groupes de 5 a niveau du joueur +3). Prime de 2000 gold. Acces aux donjons instances interdit (les portails d instance rejettent les Outlaws traqués).

**Niveau 5 — Ennemi Public** : Crimes exceptionnels (massacre de PNJ importants, sabotage de war effort de faction, Stampede deliberement declenche). Duree : jusqu a redemption. Prime de 10 000 gold. Les autres joueurs voient le joueur marque d un symbole visible (crane rouge). Les joueurs qui le tuent gagnent 1000 gold et 200 points de reputation de faction.

### 6.4 Zones de Refuge

Les Outlaws de toutes origines peuvent acceder aux **Zones de Refuge** — espaces souterrains neutres ou le statut Outlaw n active pas les consequences (gardes, primes, rejet des portails).

Zones de Refuge connues :
- **Le Refuge** (hub de la campagne Outlaws en Convergence)
- **Les Egouts de Marenne** (acces par plaque d egout cachee dans le quartier pauvre)
- **Sous-Varenkor** (acces par tunnel secret derriere la taverne des Trois Epees)
- **Sanctuaire d Erbael Inferieur** (acces par arbre creux, zone connue seulement des Outlaws et des Druides corrompus)
- **La Grotte de Sel** (cote des Abysses Cotieres, acces nautique)

Dans les Zones de Refuge, le commerce du Marche Noir est possible, les soins disponibles, et aucun PK n est permis (zone safe pour les Outlaws entre eux).

### 6.5 Competences Exclusives Outlaw

Ces competences ne peuvent etre acquises que par les joueurs de la Campagne des Bas-fonds ou via des quetes secretes specifiques (uniquement pour les joueurs avec statut Outlaw actif de niveau 3+).

**Pickpocket**
- Effet : Voler un item aleatoire de l inventaire d un joueur ou d un PNJ (probabilite variable selon niveau de competence vs niveau cible)
- Niveau 1 : 10% de chance de succes vs PNJ de niveau egal, 1% vs joueur (zone PvP uniquement)
- Niveau 5 : 40% vs PNJ, 8% vs joueur, peut cibler un slot specifique (avec soin)
- Detection : Echec = alerte immediate, +1 Wanted Level si en zone de faction
- Usage PvP : Uniquement en zones PvP declarees

**Crochetage**
- Effet : Ouvrir des serrures sans cle. Applicable aux coffres en zone, portes de donjons secrets, certaines zones cachees dans les villes.
- Niveau 1 : Serrures Simples (duree 10s de minijeu)
- Niveau 5 : Serrures Complexes (duree 30s de minijeu mais acces a 95% des serrures du jeu)
- Minijeu : Representation visuelle de la serrure avec 3-7 goupilles a aligner en temps limite
- Echec : Bruit qui alerte les gardes proches (rayon 10m)

**Dissimulation (Stealth)**
- Effet : Entrer en mode furtif. Le joueur devient invisible pour les gardes NPC et quasi-invisible pour les autres joueurs (ombre legere visible).
- Niveau 1 : 15 secondes de furtivite. Rompu par le combat ou un mouvement trop rapide.
- Niveau 5 : 60 secondes. Peut attaquer une fois depuis la furtivite (attaque sournoise +150% degats). Peut rester furtif en marchant (pas en courant).
- Compteur de furtivite visible uniquement du joueur lui-meme
- Interaction Gardes : Les gardes ont un cone de vision. Attaque depuis le cote/arriere = succes. Attaque de face = detection.

**Contrefacon**
- Effet : Fabriquer de faux papiers d identite qui reduisent temporairement le Wanted Level apparent. Aussi : faux cachets sur des marchandises pour eviter les taxes.
- Niveau 1 : Reduit le Wanted Level affiche de 1 pendant 30 minutes (les gardes vous voient comme un niveau plus bas)
- Niveau 5 : Reduit de 2, dure 2 heures, applicable aux marchandises de contrebande pour le marche normal

**Trafic de Rue**
- Effet : Connaitre les routes de contrebande — chemins alternatifs dans les villes qui evitent les checkpoints de gardes. Affiche sur la minimap des zones Outlaw des passages caches non visibles aux autres joueurs.
- Niveau 1 : Connait les passages de la ville principale de sa campagne
- Niveau 5 : Connait tous les passages de toutes les villes

### 6.6 Economie du Marche Noir

Le Marche Noir est un systeme commercial parallele accessible uniquement aux Outlaws (ou aux joueurs avec une invitation speciale).

**Prix :** 50-200% plus eleves que le marche officiel pour les biens legaux (risque inclus dans le prix). Mais certains items UNIQUEMENT disponibles au Marche Noir :
- Poisons non traçables (utilisables en PvP sans laisser de signature magique)
- Faux papiers d identite (competence Contrefacon passive)
- Cartes de zones interdites
- Runes de teleportation vers les Zones de Refuge
- Objets "chauds" (voles a d autres joueurs, revendu avec un delai de 24h)
- Informations sur d autres joueurs (localisation approximative, niveau, faction) — service payant

**Vendeurs du Marche Noir :**
- Apparaissent aleatoirement dans les Zones de Refuge (spawn toutes les 2-4h)
- Ne stockent pas leurs marchandises entre les sessions — l inventaire change a chaque apparition
- Peuvent etre ruses (vendent des items falsifies avec effets negatifs caches — necessite la competence "Evaluation" pour detecter)

### 6.7 Redemption — Processus Detaille

La Redemption permet a un Outlaw de se racheter aupres d une faction specifique et de lever son statut Outlaw avec cette faction. La Redemption est **faction par faction** — se racheter aupres de l Empire ne rachete pas aupres de l Alliance.

**Conditions generales de debut de Redemption :**
- Avoir un Wanted Level a 0 (toutes les primes actives must be expired ou payees)
- Trouver le PNJ "Confesseur" de la faction cible (NPC cache, localisation connue via rumeur ou quete de lore)
- Payer les "Frais de Redemption" (variable selon historique criminel : 500 a 10 000 gold)

**Processus de Redemption Imperiale :**
1. Trouver le Confesseur Imperial (sous la Tour du Juge a Varenkor — acces via les egouts avec Crochetage niveau 3)
2. Payer 1000 gold + restituer tout item d identification vole a l Empire
3. Accepter une "Quete de Service" : proteger un convoi imperial pendant 3 jours real time sans echouer
4. Etre valide par 3 soldats imperiaux joueurs (ils doivent confirmer via interface que le joueur a bien servi)
5. Statut Outlaw retire aupres de l Empire. Le joueur peut maintenant acceder aux villes imperiales.

**Processus de Redemption de Rive :**
1. Trouver le Confesseur de Rive (bureau cache a Marenne, acces via escalier secret dans la Bourse)
2. Payer 800 gold + rembourser 150% de la valeur des biens voles aux marchands de l Alliance
3. Quete de Service : effectuer 5 livraisons de marchandises legales sous escorte de gardes Alliance (les gardes surveillent — aucun ecart tolere)
4. Statut retire.

**Processus de Redemption Ervan :**
1. Trouver la Druidesse Confesseure (sanctuaire cache dans la Foret des Courants, marque uniquement sur les cartes Outlaw niveau 3+)
2. Payer 600 gold + purifier une zone de corruption (contribution au moins stade 2 vers stade 1)
3. Quete de Service : mediter au Cercle de Pierre d Erbael pendant 48h reel sans interruption (le joueur peut faire d autres activites mais doit revenir toutes les 2h interagir avec le Cercle)
4. Statut retire.

**Redemption Universelle (vers toutes factions simultanement) :**
- Quete epique de niveau 30+
- Implique de traverser les 3 campagnes de faction et completer une action heroique dans chacune
- Necessite de sacrifier l ensemble de l inventaire de Marche Noir (tout item illegal est detruit)
- Recompense : Statut Outlaw completement efface, titre special "Le Rachete", acces complet a toutes les factions
- Note : Les competences Outlaw exclusives sont conservees mais certaines ne fonctionnent plus en zone de faction (Pickpocket et Contrefacon notamment)

---

## 7. Donjons et Instances

### 7.1 Types de Contenu Instancie

**Open World (non instancie)**
- Zones de campagne ouvertes
- Donjons ouverts (accessible sans instance — ennemis partages entre joueurs en zone)
- Spawns de boss de zone (repop 2-6h selon le boss)
- Zones de Stampede et de corruption

**Donjons Instances Groupe (5 joueurs)**
- Instances privees : chaque groupe a sa propre copie de la zone
- Niveaux de difficulte : Normale / Difficile / Tres Difficile / Mythique
- Timer : 30-90 minutes selon le donjon
- Reset : Quotidien (Normale/Difficile), Hebdomadaire (Mythique)
- Rangs de completion : D / C / B / A / S (base sur le temps et le nombre de morts)

**Raids (10-20 joueurs)**
- Instances de grande echelle
- Niveaux de difficulte : Normale Raid / Heroique Raid
- Reset : Hebdomadaire
- Pas de rang de completion — victoire/echec uniquement

**Donjons Exclusifs de la Guilde des Aventuriers**
- Voir section 4.5

**Donjons Secrets**
- Acces non documente, trouve via exploration ou lore
- Pas de niveau de difficulte defini — equilibre sauvage
- Recompense unique (item ou cosmetique non trouvable ailleurs)

### 7.2 Systeme d Instance

| Parametre | Valeur |
|-----------|--------|
| Taille de groupe | 1 (solo, modes specifiques) / 5 (standard) / 10 / 20 (raids) |
| Duree typique | 30-90 minutes |
| Reset Quotidien | Normale, Difficile |
| Reset Hebdomadaire | Mythique, Heroique Raid |
| Corruption de donjon | 0-10 interne, independant de la zone |
| Niveau requis | Variable par donjon, minimum indique a l entree |
| Restrictions | Rang Aventurier pour les donjons exclusifs Guilde |

### 7.3 Corruption de Donjon vers Stampede

Chaque donjon instance a son propre niveau de corruption interne (0-10, distinct de la zone englobante). Ce niveau monte quand :
- Les joueurs echouent les donjons et laissent les boss Garum en vie
- La zone englobante a une corruption elevee (effet de "contamination")
- Personne ne fait le donjon pendant 48h en stade de zone 3+

Quand la corruption de donjon atteint 10 :
- Le portail du donjon explose
- Declenchement immediat d un Stampede dans la zone englobante
- Le contenu du donjon (boss, ennemis, decor) fusionne avec la zone de surface
- Le boss du donjon apparait comme le boss du Stampede (pas necessairement un des 6 boss Stampede standards — peut etre unique selon le donjon)

### 7.4 Donjons Secrets

**Le Couloir de Verre** (Convergence)
- Acces : En brisant le "Miroir de l Oublie" cache dans les Ruines de la Cite Primordiale (objet fragile, aucune indication officielle de son role)
- Contenu : Instance solo uniquement. 7 salles de miroirs ou le joueur affronte des copies de lui-meme a differents moments de sa vie (il voit des versions de son personnage aux niveaux anterieurs). Boss : La Reflection Parfaite (copie du joueur avec stats +50%).
- Recompense : Cosmetique "Armure de Verre", item d equipement unique non-tradable

**Le Puits de la Premiere Corruption** (Zone Outlaw, Bas-fonds)
- Acces : Via un rituel dans les Catacombes (necessite Crochetage niveau 4 + un item rituel rare)
- Contenu : Instance de groupe (3 joueurs exactement — pas plus, pas moins). Plonge dans l histoire du Garum avant la civilisation actuelle. Mecanique unique : le temps s ecoule a l envers dans certaines salles.
- Boss : Precurseur du Garum (entite ancienne sans nom, predatrice, combat sans indication de PV — il faut observer ses animations pour savoir quand l attaquer)
- Recompense : Lore unique (revele l origine du Garum), artefact cosmetique

**La Chambre de l Arbitre** (Empire, sous Varenkor)
- Acces : Decouvert pendant la quete speciale de la Zone 5 Outlaw (Sous-Varenkor)
- Contenu : Instance solo. Combat contre 7 arbitres NPC successifs, chacun representant une loi imperiale (le "crime" du joueur contre cette loi determine les stats de l arbitre)
- Pas de boss final — si le joueur survit aux 7 arbitres, la chambre "accepte" le joueur
- Recompense : Annulation d un crime mineur dans l historique Outlaw de l Empire (util pour la Redemption)

---

## 8. Evenements Monde Recurrents

### 8.1 Saisons et Festivals de Gaïa

**Les 4 Saisons de Gaïa** (chacune dure 3 mois reels, aligne avec les saisons de la Guilde des Aventuriers)

**Saison du Renouveau (Printemps)**
- Thematique : Renaissance, purete, chasse aux corrompus
- Evenement mondial : La Grande Purification — pendant 2 semaines au debut de la saison, tous les Cristaux de Purification depenses comptent double. Les zones purifiees pendant cet evenement ne remontent pas de niveau pendant 7 jours.
- Festival : Festival des Fleurs de Gaïa (village d Erbael est le hub) — quetes de decoration, chasse aux plantes rares, tournoi de cuisine alchimique
- Boss de saison : L Heraut du Renouveau — boss de zone special qui spawn une fois par semaine dans une zone aleatoire de zone saine, gardien de reliques purifiantes

**Saison de la Chaleur (Ete)**
- Thematique : Commerce, exploration, competences artisanales
- Evenement mondial : La Grand-Route — des caravanes marchandes NPC geantes traversent toutes les zones (de chaque hub de campagne vers la Convergence). Proteger les caravanes = recompenses massives. Les attaquer (Outlaw) = risque eleve mais butin colossal.
- Festival : Foire de la Convergence (Plaine des Quatre Vents) — toutes les factions exposent leurs marchandises, tournois d artisanat, competitions de pecheurs
- Boss de saison : Le Marchand Maudit — boss mercantile qui vend des items "trop bons pour etre vrais" puis attaque quand on approche

**Saison des Cendres (Automne)**
- Thematique : Guerre, sacrifice, resistance
- Evenement mondial : Le Grand Sieges — 4 forts en Convergence sont simultanement attaques par des armees Garum massives. Chaque fort doit etre defendu par au moins 20 joueurs. Si tous les 4 forts tiennent, bonus mondial de 48h pour toutes les factions.
- Festival : Commemoration des Tombes (Catacombes) — quetes de respect pour les PNJs importants morts en lore, items cosmetiques funeraires
- Boss de saison : Le Genereux Mort — revenant d un general heroique mort, devenu partiellement Garum, attaque les forts mais peut etre "libere" via un rituel au lieu de tue (deux fins)

**Saison du Vide (Hiver)**
- Thematique : Survie, mysterieux, Chronolites
- Evenement mondial : L Hiver du Garum — la corruption se propage 2x plus vite pendant 3 semaines. Les Stampedes sont 50% plus frequents. Les recompenses de purification sont triplee. "Survivre" a cette periode sans perdre de zone stade 3+ = recompense speciale de saison.
- Festival : Nuit des Veilleurs (Tour du Veilleur) — celebrations astronomiques, observations de constellations in-game, quetes de revelations de lore sur les Chronolites
- Boss de saison : Le Chronolite Nomade — Chronolite erre entre les zones sans pattern fixe, extremement difficile a localiser, recompense unique si tue

### 8.2 World Bosses

Les World Bosses sont des ennemis uniques d une puissance exceptionnelle qui spawnent dans le monde ouvert selon des conditions specifiques. Ils sont visibles de loin et leur apparition genere un message serveur.

**World Boss 1 — Seigneur Garum Majeur Kor-Vel-Than**
- Localisation : Plaine des Quatre Vents (Convergence)
- Conditions de spawn : La corruption totale du serveur (somme de tous les niveaux de corruption de toutes les zones) depasse un seuil critique. Repop : 7 jours si la corruption reste elevee.
- Mecanique : Il augmente activement la corruption des zones proches (rayon 200m) a chaque minute. Doit etre tue en moins de 30 minutes sinon les zones adjacentes passent au stade suivant.
- Stats : 10 millions de PV. Recommande 40+ joueurs. Attaques de zone, invocations de Seigneurs mineurs, frappe temporelle.
- Recompense : Coffre de World Boss (garantit 1 item de qualite maximale de la saison), titre "Pourfendeur de Kor-Vel-Than"

**World Boss 2 — Le Leviathan de la Bais de Sel**
- Localisation : Au large des Abysses Cotieres (Campaign de Rive, mais accessible depuis la Convergence via route maritime)
- Conditions de spawn : 5 joueurs pratiquent la peche en mer simultanement dans la zone pour 10 minutes consecutives (mecanique de declenchement inattendue — rumeur dans le lore)
- Mecanique : Combat naval (les joueurs doivent etre sur des barques ou a la nage). Le Leviathan peut engloutir des barques entieres (joueurs ejectes et perdent leur barque).
- Stats : 5 millions de PV. Recommande 20+ joueurs. Repop : 5 jours.
- Recompense : Materiau "Cuir de Leviathan" (crafting de la meilleure armure legere du jeu), recette unique.

**World Boss 3 — L Ent Primordial Gael-Sorn**
- Localisation : Coeur de la Foret Profonde (Campaign Ervan, accessible via portail en Convergence apres deblocage de quete)
- Conditions de spawn : La corruption de la Foret Profonde atteint le stade 4 ET au moins 3 sanctuaires de Gaïa de la zone Ervan sont tombes.
- Mecanique : Combat en zone boisee. Gael-Sorn invoque des arbres-guerriers. La zone de combat est en perpetuel changement (les arbres poussent et coupent l acces). Vulnerabilite au feu (mais le feu endommage aussi l environnement et peut pieger les joueurs).
- Stats : 8 millions de PV. Recommande 30+ joueurs. Repop : 10 jours.
- Recompense : "Coeur d Ent Primordial" (item de crafting permettant de creer le meilleur staff de nature du jeu).

### 8.3 Tournois Aventuriers

**Tournoi Mensuel de la Guilde**
- Date : Premier weekend de chaque mois (vendredi soir - dimanche soir)
- Lieu : Arene de la Tour d Aventure (Convergence)
- Formats :
  - Solo (1v1 PvP avec equipement normalize — tous les joueurs ont les memes stats de base, seule la technique compte)
  - Duo (2v2)
  - Equipe (5v5)
  - Open (libre — sans normalisation, brute force)

Pour participer : Rang Aventurier minimum 2 (Eclaireur). Inscription payante (50 gold — retourne en prize pool).

**Recompenses par format :**
- 1er : 50% du prize pool + trophee cosmétique de mois + 1000 PA
- 2eme : 25% + 500 PA
- 3eme : 15% + 250 PA
- 4eme-8eme : 10% divise + 100 PA

**Tournoi de Saison**
- Date : Derniere semaine de chaque saison (2 semaines avant la fin)
- Lieu : Grande Arene de Convergence (zone speciale, 500 joueurs max en spectateurs)
- Format : Bracket elimatoire de 128 joueurs (inscription 3 semaines a l avance, qualification sur rang PA de la saison)
- Recompenses du Tournoi de Saison :
  - Champion : Cosmetique "Champion de Saison [Nom]" unique, 50 000 gold, titre permanent, place garantie dans le Top 10 du Hall of Fame
  - Finaliste : 20 000 gold, cosmetique de finaliste, 500 PA de bonus
  - Semi-finalistes (2) : 5 000 gold, 250 PA
  - Quart-finalistes (4) : 1 000 gold, 100 PA

---

## 9. Schemas TOML

### 9.1 Schema Zone

```toml
[zone.foret_corrompue_ervan]
id = "foret_corrompue_ervan"
name = "Foret Profonde d Ervan"
campaign = "ervan"
level_range = [18, 24]
corruption_level = 3
corruption_rate_per_hour = 0.05
corruption_rate_no_players = 0.10
waypoint = true
waypoint_id = "clairiere_memoire"
garum_presence = "active"
stampede_risk_per_hour = 0.10
boss_id = "ent_ancien_gorravel"
resources = ["coeur_dent", "racines_primordiales", "spores_memoire"]
adjacent_zones = ["marecage_crepuscule", "cercle_anciens", "grand_autel_vide"]
ambient_sound = "foret_profonde_corruption"
music_track = "ervan_foret_tendue"

[zone.foret_corrompue_ervan.corruption_effects]
stage_1 = { regen_penalty = 0.05, resource_variant = "corrompue" }
stage_2 = { magic_resist_penalty = 0.10, campfire_efficiency = 0.50 }
stage_3 = { all_resist_penalty = 0.20, speed_penalty = 0.15, services_unavailable = true }
stage_4 = { all_stats_penalty = 0.30, no_natural_regen = true, summons_corrupt_after_seconds = 30 }
stage_5 = { zone_locked = true, adjacent_contamination = true, contamination_delay_hours = 48 }
```

### 9.2 Schema Stampede

```toml
[event.stampede.foret_001]
id = "stampede_foret_ervan_001"
zone_id = "foret_corrompue_ervan"
trigger_condition = "corruption_level >= 4"
trigger_probability_per_hour = 0.30
phases = ["pre_alert", "wave_1", "escalation", "boss", "resolution"]
boss_id = "fracas_vide_ouvreur"
cooperation_required = true
min_players_recommended = 10
max_players = 200
duration_minutes = 90

[event.stampede.foret_001.phase.pre_alert]
duration_minutes = 5
effects = ["terrain_tremor", "violet_fissures", "npc_civilian_flee"]
message_world = "Un Stampede approche a Foret Profonde d Ervan. Toutes les factions sont appelees a defendre."
countdown_visible = true

[event.stampede.foret_001.phase.wave_1]
duration_minutes = 15
emergence_points = 4
composition = { rodeur_cendres = 0.70, chevalier_cendre = 0.20, mage_fracture = 0.10 }
emergence_point_hp = 5000
interface_cross_faction = true

[event.stampede.foret_001.phase.escalation]
duration_minutes = 20
heavy_waves_from_intact_points = true
portal_weaver_spawn = true
additional_emergence_points = 2
war_channel_cross_faction = true
structure_damage_active = true

[event.stampede.foret_001.phase.boss]
duration_minutes = 25
boss_weakened_if_points_destroyed_min = 3
boss_weakened_stat_multiplier = 0.85
waves_continue_during_boss = true

[event.stampede.foret_001.phase.resolution]
success_corruption_reduction = 2
success_rewards = { chest_quality = "stampede", reputation_points = 500, purification_crystals = 50 }
failure_zone_stage = 5
failure_lockout_hours = 6
failure_adjacent_corruption_increase = 1
failure_message = "Le Garum a triomphe a Foret Profonde d Ervan. La corruption se repand."
```

### 9.3 Schema Boss Stampede

```toml
[boss.stampede.fracas_vide_ouvreur]
id = "fracas_vide_ouvreur"
name = "Fracas-Vide, l Ouvreur"
hp = 2500000
scale = 8.0
affinite_zones = ["plaine", "champ_bataille", "convergence"]

[boss.stampede.fracas_vide_ouvreur.abilities]
void_zones = { count_max = 4, radius = 8, physics_inverted = true, duration_seconds = 30, reposition_interval = 15 }
mirror_portal_trigger = { hp_threshold = 0.30, players_required_inside = 5, core_hp = 500000, portal_duration_minutes = 5 }

[boss.stampede.fracas_vide_ouvreur.rewards]
loot_table = "stampede_boss_standard"
unique_drop = "cristal_de_fracas"
unique_drop_chance = 0.15
pa_reward = 1000
```

### 9.4 Schema Aventurier

```toml
[guild.aventurier.ranks]
[[guild.aventurier.ranks.rank]]
id = 1
name = "Recrue"
requirements = { level_min = 1 }
progression_requirements = { champion_kills = 10, contracts_basic = 5, adventure_points = 100 }
privileges = ["hall_access", "contract_basic", "potion_discount_5pct", "badge_recrue"]

[[guild.aventurier.ranks.rank]]
id = 2
name = "Eclaireur"
progression_requirements = { zone_boss_kills = 5, dungeon_rank_b_or_higher = 3, adventure_points = 500 }
privileges = ["contract_intermediate", "guild_map_sharing", "dungeon_grotte_premiers", "repair_discount_10pct"]

[[guild.aventurier.ranks.rank]]
id = 3
name = "Chasseur"
progression_requirements = { zone_boss_kills = 15, zone_boss_convergence_min = 3, dungeon_rank_a_or_higher = 2, stampede_participation = 1, adventure_points = 2000 }
privileges = ["contract_advanced", "mount_silver_horse", "dungeon_ruines_scellees", "rare_monster_registry"]

[[guild.aventurier.ranks.rank]]
id = 4
name = "Vanguard"
progression_requirements = { raid_boss_kills = 5, mythic_dungeon_completions = 1, stampede_successes = 3, contracts_advanced_s_rank = 10, adventure_points = 10000 }
privileges = ["title_vanguard", "contract_elite", "dungeon_sanctum_epreuves", "armor_cosmetic_vanguard", "guild_camp_placement"]

[[guild.aventurier.ranks.rank]]
id = 5
name = "Heros"
progression_requirements = { major_lord_kills = 1, mythic_dungeon_s_timed = 1, commendations_from_rank3_plus = 50, zone_purification_stage4_plus = 1, adventure_points = 50000 }
privileges = ["title_heros_name", "lost_zone_portal_access", "dungeon_cryptes_oubliees", "reward_bonus_20pct", "council_vote_right"]

[[guild.aventurier.ranks.rank]]
id = 6
name = "Legendaire"
requirements = { hall_of_fame_top_100 = true, major_lord_kills_unique_min = 3, mythic_solo_s = true, hero_council_nominations = 5, legendary_seal = true, adventure_points = 200000 }
privileges = ["title_legendaire_unique", "armor_cosmetic_unique_season", "dungeon_du_vide", "emergency_call_5_players_24h"]
```

### 9.5 Schema Mercenaire

```toml
[guild.mercenaire.contract.military]
id = "contract_military_standard"
type = "military"
duration_days_min = 3
duration_days_max = 30
daily_retainer_base = 200
victory_bonus_multiplier = 3.0
survival_bonus_multiplier = 0.25

[guild.mercenaire.contract.military.obligations]
daily_operation_minimum = 1
ally_attack_forbidden = true
intel_leak_forbidden = true
order_compliance_required = true

[guild.mercenaire.contract.military.breach_penalties]
ally_attack = { outlaw_days = 7, gold_penalty = 2000, immediate_termination = true }
intel_leak = { outlaw_days = 30, house_ban_months = 3, immediate_termination = true }
abandonment = { gold_penalty = 500, reputation_loss = 200 }

[guild.mercenaire.contract.military.treason]
outlaw_days_minimum = 30
house_ban_seasons = 1
faction_reputation_reset = true
rehabilitation_contracts = 5
```

### 9.6 Schema Corruption Passive

```toml
[garum.corruption.rates]
no_players = 0.10
players_present_no_action = 0.05
purification_quest_completed = -0.20
sanctification_ritual = -1.00
sanctification_cooldown_hours = 24

[garum.corruption.adjacency]
multiplier_if_neighbor_stage_4_plus = 2.0

[garum.corruption.stampede_probability]
stage_3_per_hour = 0.10
stage_4_per_hour = 0.30
manual_trigger_admin = true
auto_trigger_if_stage4_duration_hours = 12

[garum.corruption.stage_5]
zone_locked = true
access_exceptions = ["outlaw_high_rank", "aventurier_legendaire", "portail_runique"]
adjacent_contamination_delay_hours = 48
purification_requirements = { stampede_max_level = true, gaïa_ritual = true, legendaire_players_min = 10 }
```

### 9.7 Schema Zone de Refuge Outlaw

```toml
[outlaw.refuge.le_refuge]
id = "le_refuge"
name = "Le Refuge"
zone = "convergence_bas_fonds"
access_method = "direct_outlaw_spawn"
safe_zone = true
pvp_forbidden = true
wanted_level_suspended = true
services = ["marche_noir", "soigneur_outlaw", "forgeron_fortune", "contrats_criminels"]

[outlaw.refuge.sous_varenkor]
id = "sous_varenkor"
name = "Sous-Varenkor"
zone = "varenkor_underground"
access_method = { type = "hidden_entrance", location = "taverne_trois_epees", skill_required = "crochetage", skill_level_min = 2 }
safe_zone = true

[outlaw.competence.pickpocket]
id = "pickpocket"
max_level = 5
npc_success_chance = [0.10, 0.18, 0.26, 0.34, 0.40]
player_success_chance = [0.01, 0.03, 0.05, 0.07, 0.08]
detection_on_fail = { wanted_level_increase = 1, alert_radius = 10 }
pvp_only_vs_players = true

[outlaw.competence.stealth]
id = "stealth"
max_level = 5
duration_seconds = [15, 25, 35, 50, 60]
sneak_attack_damage_bonus = 1.50
broken_by = ["combat", "running", "taking_damage"]
level_5_special = { attack_from_stealth = true, walk_while_stealthed = true }

[outlaw.redemption.imperial]
faction = "empire_pourpre"
prerequisite_wanted_level = 0
fee_gold = 1000
steps = [
  { type = "npc_contact", npc_id = "confesseur_imperial", access = "crochetage_3_required" },
  { type = "payment", amount = 1000, items_return = "items_imperiaux_voles" },
  { type = "service_quest", description = "Proteger un convoi imperial 3 jours", duration_days = 3 },
  { type = "validation", required_validators = 3, validator_type = "soldat_imperial_joueur" }
]
result = { outlaw_status_removed = true, city_access_restored = true }
```

### 9.8 Schema Evenement Monde Recurrent

```toml
[event.world.grande_purification]
id = "grande_purification"
season = "renouveau"
duration_days = 14
effects = { purification_crystal_multiplier = 2.0, corruption_regrowth_freeze_days = 7 }
message_world = "La Grande Purification commence. Les Cristaux de Purification sont doublement efficaces."

[event.world.winter_garum]
id = "hiver_garum"
season = "vide"
duration_weeks = 3
effects = { corruption_rate_multiplier = 2.0, stampede_frequency_multiplier = 1.50, purification_reward_multiplier = 3.0 }
special_reward = { condition = "no_zone_lost_to_stage3_during_event", reward = "titre_survivant_hiver" }

[event.world_boss.kor_vel_than]
id = "kor_vel_than"
name = "Seigneur Garum Majeur Kor-Vel-Than"
spawn_location = "plaine_quatre_vents"
spawn_condition = { server_total_corruption_threshold = 250 }
hp = 10000000
respawn_days = 7
mechanic_corruption_aura = { radius = 200, rate_per_minute = 0.10 }
kill_timer_minutes = 30
recommended_players = 40
rewards = { world_boss_chest = true, title = "Pourfendeur de Kor-Vel-Than" }

[event.tournament.mensuel]
id = "tournoi_mensuel"
schedule = "first_weekend_of_month"
formats = ["solo_normalized", "duo_normalized", "team_5v5", "open"]
rank_minimum = 2
entry_fee_gold = 50
prize_pool_distribution = [0.50, 0.25, 0.15, 0.10]
pa_rewards = [1000, 500, 250, 100]
```

---

## Annexe A — Glossaire

| Terme | Definition |
|-------|------------|
| PA | Points d Aventure — monnaie de progression de la Guilde des Aventuriers |
| Garum | Force de corruption primordiale, antagoniste environnemental et militaire |
| Stampede | Debordement de donjon Garum sur la surface, evenement de cooperation force |
| Chronolite | Etre superieur du Garum, manipulation temporelle |
| Corruption Level | Niveau 0-5 indiquant l etat d une zone sous influence Garum |
| Redemption | Processus de rehabilitation des Outlaws aupres d une faction |
| RvR | Realm versus Realm, guerre de territoire entre factions en Convergence |
| Zone Perdue | Zone au stade 5, pratiquement inaccessible, menace les zones adjacentes |
| Wanted Level | Niveau de recherche 0-5, mesure de la notoriete criminelle d un Outlaw |
| Marche Noir | Systeme commercial parallele exclusif aux Outlaws |

## Annexe B — Tableau de Progression Resumee

| Niveau | Activite recommandee | Faction principale | Acces Aventurier |
|--------|---------------------|-------------------|-----------------|
| 1-5 | Zones d introduction de campagne | Propre faction | Rang 1 (inscription) |
| 6-10 | Zones 2-3 de campagne | Propre faction | Rang 1-2 |
| 11-15 | Zones 3-4 de campagne, premiers donjons | Propre faction + contacts cross-faction | Rang 2 |
| 16-20 | Zones 4-5 de campagne, donjons groupe | Propre faction, mercenariat possible | Rang 2-3 |
| 21-25 | Zones 5-6-7 de campagne, Stampedes | Tous contacts faction + guilde | Rang 3 |
| 26-30 | Zone finale campagne, raids basiques | Toutes factions (pre-Convergence) | Rang 3-4 |
| 30+ | Convergence — RvR, raids, world bosses | Convergence (toutes factions) | Rang 4-6 |

