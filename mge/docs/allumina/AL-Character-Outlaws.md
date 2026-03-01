<!-- @id: AL-Character-Outlaws @do: reference @role: game-designer @layer: 3 @human: miyuk -->

# Allumina — Classes des Outlaws

**Statut :** Référence canonique v1.0
**Date :** 2026-02-28
**Scope :** 6 classes jouables des Outlaws — rôles, arbres de compétences, compétences universelles, schémas TOML
**Monde :** Véranthas, An 247 AO

---

## Table des matières

1. [Vue d'ensemble — L'underground criminel de Véranthas](#1-vue-densemble--lunderground-criminel-de-véranthas)
2. [Assassin](#2-assassin--dps-burst-stealth)
3. [Rôdeur Maudit](#3-rôdeur-maudit--dps-contrôle-terrain)
4. [Nécromancien des Rues](#4-nécromancien-des-rues--support-hybride-mort)
5. [Guerrillero](#5-guerrillero--dps-mobilité-embuscade)
6. [Alchimiste Noir](#6-alchimiste-noir--support-hybride-altération)
7. [Berserker Banni](#7-berserker-banni--tank-dps-rage)
8. [Compétences universelles Outlaw](#8-compétences-universelles-outlaw)
9. [Système Wanted Level — Interactions complètes](#9-système-wanted-level--interactions-complètes)
10. [Schémas TOML complets](#10-schémas-toml-complets)

---

## 1. Vue d'ensemble — L'underground criminel de Véranthas

Les Outlaws de Véranthas ne sont pas des héros déchus. Ils sont le produit naturel d'un monde où trois empires rivaux gouvernent par la loi du plus fort, où la Guilde des Mercenaires brise ses propres membres sur la roue du Pacte des Compagnies, et où des générations entières ont été écrasées, trahies, ou simplement oubliées par les structures de faction. Certains ont choisi cette vie. La plupart l'ont vue choisir pour eux.

Leurs combattants ne ressemblent à aucune autre faction. Ils n'ont pas de doctrine de formation, pas d'académies d'armes, pas de maîtres reconnus. Ce qu'ils ont, c'est l'expérience brutale : des années à survivre dans les égouts de Selmara, les tunnels sous Velanthara, les ruelles chaudes de Cendrepas. Leur style de combat est empirique — ce qui tue ce soir, gardé; ce qui tue plus vite demain, adopté.

**La campagne Outlaw (niveaux 1-30)** se déroule dans les bas-fonds : les égouts de la ville d'entrée, les tunnels de contrebande entre Cendrepas et les Îles Grises, le marché noir souterrain de Vel'Drath. Le joueur découvre progressivement la hiérarchie criminelle locale, monte les rangs de la Confrérie de l'Ombre Libre, et s'établit comme une force à compter avant de débloquer l'accès à la carte complète.

### Tableau des classes Outlaw

| # | Classe | Rôle | Armes | Style |
|---|--------|------|-------|-------|
| 1 | Assassin | DPS Burst Stealth | Dague, Épée courte | Invisibilité, burst, poison, exécution |
| 2 | Rôdeur Maudit | DPS Contrôle Terrain | Arc court, Couteau, Pièges | Pièges, terrain, traque, embuscade à distance |
| 3 | Nécromancien des Rues | Support/Hybride Mort | Bâton court, Os rituels | Invocations mineures, malédictions, contrôle de zone |
| 4 | Guerrillero | DPS Mobilité Embuscade | Sabre court, Explosifs, Arbalète légère | Guérilla, mobilité urbaine, explosifs, embuscade |
| 5 | Alchimiste Noir | Support/Hybride Altération | Alambic de combat, Couteau d'analyse | Venins, drogues, bombes chimiques, buffs illégaux |
| 6 | Berserker Banni | Tank/DPS Rage | Hache à deux mains, Chaîne, Corps | Rage pure, absorption des coups, frénésie sans limite |

### Identité sociale de départ

Tous les joueurs Outlaw commencent avec le rang **Banni** — le plus bas de la hiérarchie criminelle. Pas de titre, pas de respect acquis, pas d'accès aux zones sécurisées de la Confrérie. La progression passe par les actes : coups réussis, contrats remplis, primes évitées, et la construction lente d'une réputation dans un milieu où personne ne vous fait confiance par défaut.

### Zones de refuge

Chaque grande ville de Véranthas possède une **Zone de Refuge** — un quartier ou un réseau souterrain où les Outlaws peuvent se déplacer sans être attaqués par les gardes. L'accès à ces zones requiert le rang Renégat minimum ou la connaissance d'un code d'entrée (changé toutes les 72h de jeu). Les zones de refuge sont des zones PvP joueur contre joueur — la Confrérie ne garantit pas la sécurité contre les autres Outlaws.

---

## 2. Assassin — DPS Burst Stealth

### 2.1 Identité

**Rôle :** DPS Burst Stealth
**Armes principales :** Dague principale, Dague de réserve (dual-wield ou main-gauche), Épée courte optionnelle
**Couleur de classe :** Noir de jais (#1A1A1A) / Violet empoisonné (#7B2D8B)

### 2.2 Style de combat

L'Assassin ne se bat pas — il exécute. Chaque engagement est précédé de secondes, parfois de minutes de préparation en invisibilité : analyser les patterns de mouvement de la cible, identifier le bon angle, choisir la dose de poison adaptée. Le combat commence et se termine en quelques secondes. Si ça dure plus longtemps, quelque chose a mal tourné. Sa ressource centrale est la **Tension** — elle monte dans l'ombre et se libère en un pic de violence calculée. Il ne supporte pas les échanges prolongés et le sait : son kit entier est construit pour que le problème soit mort avant d'avoir pu le devenir.

### 2.3 Arbre 1 — Art de l'Ombre

**Thème :** Furtivité, préparation, manipulation de la détection

| Type | Nom | Effet |
|------|-----|-------|
| Active | Voile d'Ombre | Entre en invisibilité pendant 12s (brisée par toute attaque ou sprint). Genère 10 Tension par seconde en invisibilité. |
| Active | Pas Silencieux | Supprime le son de déplacement pendant 8s ; les ennemis ont -40% de rayon de détection. |
| Active | Ombre Portée | Laisse un double illusoire immobile à la position actuelle et se déplace jusqu'à 10m ; le double dure 6s avant de se dissoudre. |
| Passive | Instinct de Proie | Le Wanted Level actuel réduit le délai avant réactivation du Voile d'Ombre de 3s par niveau. |
| Passive | Regard Vide | En invisibilité, régénère 1% de vie par seconde. |

### 2.4 Arbre 2 — Alchimie du Tranchant

**Thème :** Poisons, saignements, applications sur lame

| Type | Nom | Effet |
|------|-----|-------|
| Active | Lame Empoisonnée | Applique Poison Standard sur la dague pendant 30s : chaque frappe inflige +25% ADPhy en dégâts de poison sur 4s (renouvelable). |
| Active | Venom Concentré | Frappe unique qui injecte Poison Profond : 80% ADPhy en poison sur 12s, non-renouvelable, résistance divisée par 2. |
| Active | Saignée Précise | Attaque ciblant une artère : 100% ADPhy + Saignement grave 3% PV max par seconde pendant 6s, interrompu par soin. |
| Passive | Formulation Améliorée | Les durées de tous les poisons personnels augmentent de 30%. |
| Passive | Lame Persistante | L'effet Lame Empoisonnée ne disparaît pas à la mort de l'Assassin — la lame reste toxique si ramassée. |

### 2.5 Arbre 3 — Exécution Parfaite

**Thème :** Dégâts en burst, one-shot, exécution des cibles affaiblies

| Type | Nom | Effet |
|------|-----|-------|
| Active | Backstab | Depuis l'invisibilité ou le dos de la cible : 280% ADPhy, génère 30 Tension bonus, applique Vulnérabilité 10% pendant 6s. |
| Active | Égorgement | Cible à moins de 30% PV : 200% ADPhy + tentative d'exécution instantanée (résistible par les joueurs, non-résistible sur PNJ de rang inférieur). |
| Active | Libération de Tension | Dépense toute la Tension accumulée (max 100) : 15% ADPhy par point de Tension en une frappe unique, portée mêlée. |
| Passive | Lame Froide | Les attaques portées depuis l'invisibilité ignorent 20% de l'armure de la cible. |
| Passive | Instinct du Prédateur | Si la cible est affectée par au moins 2 altérations d'état, toutes les actives de l'Assassin ont -1s de cooldown. |

### 2.6 Compétences Signature

**Contrat Scellé** *(Cooldown : 120s)*
L'Assassin désigne une cible comme son contrat actif. Pendant 20s, toutes ses attaques contre cette cible infligent +40% de dégâts, ses poisons s'appliquent en double dose, et ses cooldowns se réinitialisent entièrement si la cible meurt dans la fenêtre. Si la cible fuit hors du rayon de 30m, la compétence est considérée comme rompue et le cooldown commence. Une seule cible de contrat active à la fois.

**Disparition** *(Cooldown : 45s)*
L'Assassin disparaît instantanément, même en combat, entrant en invisibilité de 5s non brisable par les dégâts (mais brisée par un CC ciblé). Pendant ces 5s, il se déplace à +50% de vitesse. Toute attaque portée dans les 2s suivant la sortie d'invisibilité génère 50 Tension bonus. Utilisable en plein combat pour se réinitialiser.

### 2.7 Interaction avec le système Outlaw

**Wanted Level :** L'Assassin est la classe qui monte le plus vite en Wanted Level (chaque kill PvP +1 niveau, assassinat de PNJ de faction +0,5). En contrepartie, il possède le meilleur outil naturel pour le réduire : Voile d'Ombre et Disparition lui permettent de fuir les zones de danger avant que les gardes arrivent.

**Stealth :** L'Assassin est le seul expert stealth de la faction Outlaw. Il peut utiliser les zones d'ombre urbaines (ruelles non éclairées, égouts) comme bonus de durée sur Voile d'Ombre (+5s).

**Marché Noir :** Accès aux Poisons de Marché Noir (venins de rang 4-5, non disponibles en artisanat légal). Sa compétence Lame Persistante en fait un vecteur de distribution involontaire — une dague empoisonnée revendue sur le marché noir garde ses propriétés.

### 2.8 Chemin de rédemption préférentiel

**Guilde des Aventuriers.** La compétence de traque, la maîtrise du terrain, et la capacité à éliminer des cibles précises font de l'Assassin un excellent éclaireur et chasseur de primes légal. La Guilde des Aventuriers l'intègre dans ses équipes de contrat-délivrance avec relativement peu de friction sociale — un Aventurier qui tue proprement est respecté.

### 2.9 Lore

Personne ne *choisit* de devenir Assassin dans les bas-fonds de Véranthas — on y est formé par nécessité. Les égouts de Selmara ont leurs propres maîtres, des vétérans qui n'ont jamais eu de nom sur une plaque de bronze mais dont les mains ont changé des guerres. Un Assassin Outlaw n'a pas de code d'honneur de duelliste ni de serment de soldat — il a un prix, une méthode, et la discipline froide de quelqu'un qui sait que l'erreur se paye en sang. Le sien en premier.

---

## 3. Rôdeur Maudit — DPS Contrôle Terrain

### 3.1 Identité

**Rôle :** DPS Contrôle de Terrain / Trappeur
**Armes principales :** Arc court de survie, Couteau de chasse, Pièges artisanaux (consommables d'inventaire)
**Couleur de classe :** Marron brûlé (#8B4513) / Vert toxique (#228B22)

### 3.2 Style de combat

Le Rôdeur Maudit contrôle le terrain avant que le combat commence. Sa philosophie : si ton ennemi marche où tu veux qu'il marche, le combat est déjà à moitié gagné. Il passe les 30 premières secondes de chaque engagement à placer ses pièges, à identifier les lignes de fuite, à positionner ses marqueurs de zone. Puis il laisse les autres arriver à lui. Ses capacités de survie en milieu hostile (égouts, ruines, zones corrompues) en font le spécialiste naturel des combats prolongés dans des environnements que ses adversaires trouvent invivables. Il n'est pas le combattant le plus rapide ni le plus puissant — il est celui qui est encore debout quand tous les autres ont épuisé leurs plans.

### 3.3 Arbre 1 — Maîtrise des Pièges

**Thème :** Déploiement, amélioration et déclenchement de pièges au sol

| Type | Nom | Effet |
|------|-----|-------|
| Active | Piège à Mâchoire | Dépose un piège (0,5s d'armement) qui immobilise la première cible pendant 3s et inflige 80% ADPhy. |
| Active | Mine de Clous | Zone de piquants (2m de rayon) active 1s après pose : inflige 40% ADPhy et Saignement aux cibles qui traversent. |
| Active | Détonateur d'Urgence | Déclenche instantanément tous les pièges actifs dans un rayon de 15m, multipliant leurs effets par 1,5 mais les consumant. |
| Passive | Mains Rapides | Le temps d'armement de tous les pièges est réduit de 40%. |
| Passive | Pièges Multiples | Peut maintenir jusqu'à 5 pièges actifs simultanément (base : 3). |

### 3.4 Arbre 2 — Survie en Zone Hostile

**Thème :** Résistances aux environnements dangereux, régénération, adaptation

| Type | Nom | Effet |
|------|-----|-------|
| Active | Peau de Cendres | Applique un enduit de survie sur la peau pendant 15s : résistance aux poisons, aux dégâts de zone, et aux effets de terrain de 30%. |
| Active | Camouflage de Ruines | En milieu urbain dégradé ou souterrain : réduit la détection à 30% pendant 10s (furtivité partielle, non équivalente à l'invisibilité). |
| Active | Herbologie Noire | Consomme une herbe de terrain ramassée pour récupérer 25% PV et purger un effet négatif. |
| Passive | Immunité Progressive | Chaque fois que le Rôdeur subit l'effet d'un poison, sa résistance à ce poison augmente de 5% (cumulable, max 25%). |
| Passive | Traqueur Né | Voit les traces de pas et les marques de pièges adverses dans un rayon de 12m. |

### 3.5 Arbre 3 — Tir de Survie

**Thème :** Combat à l'arc, flèches spéciales, contrôle à distance

| Type | Nom | Effet |
|------|-----|-------|
| Active | Flèche de Ralentissement | Tir infligeant 90% ADPhy + Ralentissement 50% pendant 4s. |
| Active | Tir Tendu | Charge 1,5s : inflige 220% ADPhy et traverse les boucliers (pas les couvertures physiques). |
| Active | Salve Basse | Tir rasant au niveau des jambes : inflige 70% ADPhy et force la cible à s'agenouiller (incapacité partielle) 1,2s. |
| Passive | Viseur de Chasse | Les tirs contre des cibles emprisonnées dans un piège infligent +60% de dégâts. |
| Passive | Économie de Flèches | 15% de chance de récupérer une flèche utilisée après chaque tir. |

### 3.6 Compétences Signature

**Territoire Marqué** *(Cooldown : 90s)*
Le Rôdeur marque une zone circulaire de 12m de rayon pendant 30s. Dans cette zone : tous ses pièges s'arment en 0,1s, tous ses tirs gagnent +25% de dégâts, et tous les ennemis sont révélés (même en invisibilité). Les alliés dans la zone gagnent +10% de vitesse de déplacement. La zone est visible pour les ennemis comme un marquage de terrain (lignes de fumée au sol) — le danger est affiché, l'éviter est possible, mais le territoire est clairement hostile.

**Embuscade Parfaite** *(Cooldown : 180s — préparation requise)*
Nécessite au moins 3 pièges actifs dans un rayon de 20m. Le Rôdeur entre en Camouflage de Ruines gratuit (8s non brisable). Durant ces 8s, le premier tir d'arc qu'il effectue déclenche simultanément tous ses pièges dans un rayon de 20m, inflige 250% ADPhy, et applique Peur (fuite désordonnée) pendant 2s aux cibles touchées par les pièges. Utilisable une fois par zone de combat.

### 3.7 Interaction avec le système Outlaw

**Wanted Level :** Le Rôdeur génère peu de Wanted Level dans les combats tendus parce qu'il laisse rarement de témoins et agit depuis la distance. Cependant, ses pièges laissent des traces physiques identifiables — un garde qui retrouve des pièges à mâchoire dans les égouts peut déclencher une augmentation passive du Wanted Level du Rôdeur dans cette zone.

**Stealth :** Furtivité partielle seulement (Camouflage de Ruines). Il n'est pas un stealth pur — il contourne la détection par la préparation de terrain plutôt que par l'invisibilité.

**Marché Noir :** Accès aux Pièges de rang élevé (pièges à venin, mines à fragmentation artisanales) non disponibles via l'artisanat légal. Peut aussi vendre des herbes rares récoltées dans les zones corrompues — denrée très recherchée.

### 3.8 Chemin de rédemption préférentiel

**Guilde des Mercenaires.** La discipline tactique du Rôdeur, sa maîtrise du terrain et son efficacité en zone hostile en font un mercenaire naturel. La Guilde apprécie les spécialistes qui rendent les missions difficiles possibles — un Rôdeur qui sécurise un couloir ou neutralise une position adverse sans déclencher une guerre ouverte est exactement ce que les contrats complexes requièrent.

### 3.9 Lore

Les Rôdeurs Maudits portent leur nom comme une cicatrice. La plupart ont survécu à quelque chose qui aurait dû les tuer — une zone corrompue, une Stampede, un hiver dans les tunnels — et cette survie les a marqués au niveau cellulaire. Leur peau garde parfois des traces de contact prolongé avec Garum : veines noircies, yeux qui reflètent l'obscurité différemment, une capacité à fonctionner dans des environnements qui rendent les autres malades. Ils ne parlent pas de ça. Les autres Outlaws non plus.

---

## 4. Nécromancien des Rues — Support/Hybride Mort

### 4.1 Identité

**Rôle :** Support/Hybride Mort (invocations mineures, malédictions, contrôle de zone)
**Armes principales :** Bâton rituel court (sculpté en os ou bois carbonisé), Fragments osseux (consommables d'invocation), Dague de sacrifice
**Couleur de classe :** Gris cendres (#696969) / Vert cadavérique (#7FFF00)

### 4.2 Style de combat

Le Nécromancien des Rues n'est pas le seigneur de la mort des grandes tours de sorcellerie — il est le survivant qui a appris à parler aux morts parce que les morts ne mentent pas et ne trahissent pas. Son necromancement est urbain, improvisé : des ossements ramassés dans les catacombes, des formules griffonnées sur des murs de cave, des rituels condensés en gestes économiques par des années de pratique souterraine. En combat, il maintient 2 à 4 serviteurs osseux actifs qui absorbent les dégâts et perturbent les lignes adverses pendant qu'il accumule ses malédictions. Il n'est pas en première ligne — il est la deuxième ligne qui rend la première ligne de ses alliés invivable pour l'ennemi.

### 4.3 Arbre 1 — Invocations Osseuses

**Thème :** Convocation et gestion de serviteurs osseux

| Type | Nom | Effet |
|------|-----|-------|
| Active | Osseux Grouillant | Invoque 1 squelette mineur (100% ADPhy melee, 30% PV base) pour 45s. Consomme 1 Fragment osseux. |
| Active | Chien de Cendres | Invoque un chien osseux rapide (70% ADPhy, 20% PV, vitesse +40%) qui cible la cible désignée et applique Saignement par morsure. |
| Active | Explosion de Serviteur | Détone un serviteur actif au contact d'un ennemi : 150% ADPhy en zone (3m), soin du Nécromancien de 10% PV. |
| Passive | Lien du Maître | Les serviteurs actifs régénèrent 5% de leurs PV par seconde si le Nécromancien est à moins de 10m d'eux. |
| Passive | Os Renforcés | Les serviteurs reçoivent 15% moins de dégâts physiques. |

### 4.4 Arbre 2 — Malédictions Urbaines

**Thème :** Debuffs, altérations d'état, affaiblissement des ennemis

| Type | Nom | Effet |
|------|-----|-------|
| Active | Marque de Corruption | Cible unique : -20% résistances à tous les types de dégâts pendant 10s. Visible pour les alliés. |
| Active | Malédiction de Décrépitude | Zone de 5m : réduit la vitesse d'attaque et de déplacement de 25% pendant 8s. |
| Active | Emprise des Morts | Lie la cible à une chaîne spectrale pendant 3s (immobilisation, résistible). Si le Nécromancien ou un serviteur frappe la cible pendant l'immobilisation, durée étendue de 1,5s. |
| Passive | Voix des Morts | Les malédictions appliquées propagent une aura de peur passive : les ennemis proches de cibles maudites ont -10% de précision. |
| Passive | Accumulation Cadavérique | Chaque mort (PNJ ou joueur) dans un rayon de 20m recharge 1 Fragment osseux (max +3 par combat). |

### 4.5 Arbre 3 — Cendres et Rituels

**Thème :** Rituels de zone, soutien des alliés, manipulations cadavériques

| Type | Nom | Effet |
|------|-----|-------|
| Active | Cercle des Morts | Trace un cercle de 6m (3s de rituel) : pendant 20s, tout ennemi qui entre dans le cercle subit 30% ADPhy par seconde en dégâts nécrotiques. |
| Active | Voile de Cendres | Zone de fumée grise (8m, 8s) : neutralise la vision dans la zone, bloque les tirs à distance, réduit la précision de 50% pour tous ceux à l'intérieur. |
| Active | Transfusion Osseuse | Sacrifie un serviteur actif pour soigner un allié ciblé de 25% de ses PV max. |
| Passive | Ritualiste Économique | Les rituels (Cercle des Morts) sont réduits de 1s de temps de canalisation. |
| Passive | Mémoire des Corps | Les corps de serviteurs détruits au combat redeviennent des fragments osseux ramassables pendant 30s. |

### 4.6 Compétences Signature

**Armée de Fortune** *(Cooldown : 150s)*
Le Nécromancien consomme tous ses Fragments osseux disponibles (max 5) et invoque instantanément autant de serviteurs osseux en même temps. Ces serviteurs ont +50% PV et +30% ADPhy par rapport aux serviteurs normaux, et durent 60s ou jusqu'à leur mort. Pendant les 10 premières secondes d'activation, tous les serviteurs invoqués sont immunisés aux dégâts de zone (ils ne peuvent être éliminés qu'en ciblage direct). La compétence est l'escalade finale — utilisée pour submerger une position ou protéger une retraite.

**Linceul du Désespoir** *(Cooldown : 120s)*
Le Nécromancien enveloppe une cible dans un linceul spectral pendant 6s. Durant ces 6s : la cible est ralentie de 60%, tous les soins reçus sont réduits de 70%, et chaque frappe portée contre elle génère +10% de dégâts supplémentaires pour l'attaquant. Si la cible meurt sous le Linceul, son cadavre se relève automatiquement comme un serviteur osseux de durée 20s sans consommer de Fragment osseux.

### 4.7 Interaction avec le système Outlaw

**Wanted Level :** Les activités nécromantiques sont illégales dans toutes les factions sans exception. Invoquer des serviteurs à vue d'un garde dans une zone neutre déclenche une augmentation automatique de Wanted Level +1. En Zone de Refuge, l'invocation est tolérée (les gardes de faction n'y patrouillent pas).

**Stealth :** Pas de furtivité naturelle — le Nécromancien utilise Voile de Cendres comme équivalent fonctionnel pour se déplacer ou se retirer.

**Marché Noir :** Accès aux Fragments osseux de rang élevé (os de créatures rares, reliques corrompues de Garum). Ses transfusions et malédictions font de lui un prestataire de service recherché parmi les autres Outlaws — il est souvent engagé pour des missions de terrain spécifiques.

### 4.8 Chemin de rédemption préférentiel

**Fédération Ervan.** La Fédération, pragmatique et ouverte à toutes les origines raciales et sociales, est la seule faction qui ne criminalise pas la nécromance en tant que discipline — elle la considère comme une science marginale mal comprise. Un Nécromancien des Rues qui démontre sa maîtrise des zones corrompues (les serviteurs osseux sont naturellement résistants aux effets de Garum) devient un atout de valeur pour les Éclaireurs de la Fédération.

### 4.9 Lore

La nécromance des rues n'a rien à voir avec la grande tradition des nécromanciens académiques de l'Empire Pourpre — ces derniers ne reconnaîtraient même pas ce que pratiquent les Nécromanciens des Rues comme de la vraie magie. Ce sont des autodidactes qui ont appris en touchant des corps dans les catacombes, en écoutant ce que les os ont à dire, en comprenant empiriquement que la mort n'est pas une fin mais une ressource. Ils viennent souvent des districts les plus pauvres des capitales — ceux où les morts ne sont pas enterrés dignement mais simplement entassés dans des fosses collectives. Ils ont grandi avec les morts. Il était naturel de leur parler.

---

## 5. Guerrillero — DPS Mobilité Embuscade

### 5.1 Identité

**Rôle :** DPS Mobilité / Embuscade Urbaine
**Armes principales :** Sabre court de rue, Arbalète légère de poing (une main), Explosifs improvisés (consommables)
**Couleur de classe :** Rouge sang séché (#8B0000) / Orange explosion (#FF4500)

### 5.2 Style de combat

Le Guerrillero est la guerre asymétrique incarnée. Il ne tient pas une ligne — il n'en a aucune. Il entre, frappe, part, entre de l'autre côté, frappe encore, et quand la cible tente de le suivre, elle marche sur quelque chose qu'il a posé quinze secondes avant. Sa mobilité est sa survie : il utilise les structures urbaines (fenêtres, passages étroits, toits, grilles d'égout) comme une géographie tactique que ses adversaires refusent généralement d'exploiter. Ses explosifs improvisés ne sont pas de l'artillerie — ce sont des outils de création d'opportunités : ouvrir une brèche, bloquer une retraite, déclencher la panique dans une formation.

### 5.3 Arbre 1 — Guérilla Urbaine

**Thème :** Mobilité, engagement-désengagement, utilisation du terrain vertical

| Type | Nom | Effet |
|------|-----|-------|
| Active | Ruée de Ruelle | Dash de 8m en ligne droite, ignore les corps adverses, inflige 100% ADPhy à toute cible traversée. |
| Active | Escalade Éclair | Monte verticalement une structure (mur, canalisation, arche) jusqu'à 6m en 0,8s — uniquement en zones urbaines. |
| Active | Replier | Désengagement arrière rapide (5m), applique un effet de fumée au point d'origine (dure 3s, bloque la vision). |
| Passive | Cartographie Criminelle | En zone urbaine connue (visitée une fois), toutes les vitesses de déplacement +10%. |
| Passive | Réflexes de Passage | Les attaques ennemies ont 10% de chance de rater si le Guerrillero est en mouvement. |

### 5.4 Arbre 2 — Artillerie de Fortune

**Thème :** Explosifs improvisés, bombes incendiaires, grenades de confusion

| Type | Nom | Effet |
|------|-----|-------|
| Active | Bombe à Mèche | Lance un explosif (rayon 4m, délai 2s) : 160% ADPhy en zone, knockback 3m sur toutes cibles touchées. |
| Active | Cocktail Enflammé | Bouteille incendiaire : zone de feu (3m, 8s) infligeant 20% ADPhy/s. Les cibles en feu reçoivent +20% de dégâts physiques. |
| Active | Grenade de Fumée | Fumée opaque (6m, 10s) : bloque les projectiles et la vision. Les alliés dans la fumée ont +20% de vitesse. |
| Passive | Synthèse Rapide | Les explosifs se fabriquent deux fois plus vite à l'atelier de campagne. |
| Passive | Charges Renforcées | Le rayon de Bombe à Mèche augmente à 5m et son délai passe à 1,5s. |

### 5.5 Arbre 3 — Lame de Rue

**Thème :** Combat au sabre court, combos rapides, mêlée agressive

| Type | Nom | Effet |
|------|-----|-------|
| Active | Taillade Basse | Frappe aux jambes : 80% ADPhy + Ralentissement 30% pendant 3s. |
| Active | Enchaînement de Ruelle | Série de 3 frappes rapides alternant sabre et poing : 80%/80%/120% ADPhy. La troisième frappe stun 0,5s. |
| Active | Exécution de Proche | Si la cible est à moins de 2m et en dessous de 35% PV : frappe de finisher 250% ADPhy, ignore 25% d'armure. |
| Passive | Maître du Court Espace | Dégâts mêlée +15% si l'ennemi est immobile ou sous effet de CC. |
| Passive | Arme Secondaire | L'arbalète peut tirer depuis la mêlée sans malus de proximité. |

### 5.6 Compétences Signature

**Blocus** *(Cooldown : 100s)*
Le Guerrillero place instantanément 3 bombes à mèche en triangle autour d'une zone cible de 8m de rayon (placement automatique aux angles optimaux). Ces 3 bombes ont un délai d'armement de 3s et détonent simultanément sur commande (ou automatiquement en fin d'armement). Les cibles dans la zone triple reçoivent les dégâts de chaque bombe proportionnellement à leur position — une cible au centre reçoit les 3 explosions. Le Guerrillero est immunisé à ses propres explosions pendant 4s après activation.

**Sprint de Guerre** *(Cooldown : 60s)*
Pendant 8s, le Guerrillero court à +80% de vitesse, ses attaques de mêlée appliquent un effet de Destabilisation (les cibles touchées ratent leur prochaine attaque), et il peut franchir des obstacles physiques (portes verrouillées, barrières basses) en les défoncant. Chaque ennemi touché pendant le Sprint régénère 5% PV au Guerrillero. La compétence simule le mouvement d'assaut d'un combattant de guérilla expérimenté — chaos contrôlé.

### 5.7 Interaction avec le système Outlaw

**Wanted Level :** Chaque explosion dans une zone civile (non-combat) augmente le Wanted Level de +1 et déclenche une alerte locale de 60s. Le Guerrillero doit gérer son Wanted Level activement — ses outils font du bruit.

**Stealth :** Pas de furtivité directe. Grenade de Fumée et Replier sont ses équivalents fonctionnels pour casser la ligne de vue.

**Marché Noir :** Ses explosifs improvisés sont des marchandises précieuses sur le marché noir — il peut fabriquer et revendre des stocks à d'autres Outlaws. Accès aux détonateurs et charges améliorées de rang élevé via les fournisseurs criminels.

### 5.8 Chemin de rédemption préférentiel

**Alliance de Rive.** L'Alliance, pragmatique et mercantile, ne se préoccupe pas de l'origine de ses soldats — elle se préoccupe de leur efficacité. Un Guerrillero qui a survécu aux bas-fonds de Véranthas possède des compétences de terrain que les officiers de l'Alliance ne savent pas enseigner dans leurs académies. La transition vers corsaire ou tireur d'élite de l'Alliance se fait naturellement.

### 5.9 Lore

Les Guerrilleros ne naissent pas dans la violence — ils y sont formés par elle. Chaque Guerrillero Outlaw a une histoire de ville perdue, de quartier rasé, de famille dispersée par une opération militaire ou une purge de faction. Ils ont appris la guerre en la subissant, puis en la retournant contre ceux qui la leur avaient faite. Leurs techniques improvisées sont des innovations issues de la contrainte : une bouteille d'huile de lampe et un chiffon, c'est une arme de siège quand on n'a rien d'autre. Ils savent improviser parce qu'ils n'ont jamais eu les moyens de faire autrement.

---

## 6. Alchimiste Noir — Support/Hybride Altération

### 6.1 Identité

**Rôle :** Support/Hybride Altération (altérations d'état offensives, buffs illégaux, contrôle chimique)
**Armes principales :** Alambic de combat portable (genère des projectiles chimiques), Couteau d'analyse (corps à corps défensif), Fioles et consommables (inventaire dédié de 12 slots)
**Couleur de classe :** Violet acide (#9400D3) / Jaune soufre (#FFD700)

### 6.2 Style de combat

L'Alchimiste Noir est un chimiste de guerre qui a rejeté les contraintes éthiques de sa discipline d'origine. Sa pharmacopée comprend des substances que les factions légales interdisent précisément parce qu'elles fonctionnent trop bien. En combat, il jongle entre les rôles : altérer la cible (venins, acides, paralysants), booster ses alliés (stimulants de combat, analgésiques illégaux), et contrôler la zone (gaz de zone, fumigations). Sa gestion d'inventaire est cruciale — une Fiole d'Acide utilisée au mauvais moment est une ressource perdue. Sa puissance est directement proportionnelle à la qualité de sa préparation d'avant-combat.

### 6.3 Arbre 1 — Pharmacopée de Combat

**Thème :** Venins, drogues de combat, buffs illégaux administrés à soi ou aux alliés

| Type | Nom | Effet |
|------|-----|-------|
| Active | Stimulant de Rang 3 | S'injecte ou injecte un allié : +30% vitesse d'attaque et +15% dégâts pendant 20s. Effets secondaires : -10% défense après expiration pendant 10s. |
| Active | Analgésique Noir | Cible alliée : immunité à la douleur (ignore les effets de slow et de stun d'1 charge, durée 15s). Utilisable sur soi. |
| Active | Antidote Corrosif | Purge un effet négatif d'un allié et renvoie l'effet dissous à la source (dommage de retour 50% de l'effet original). |
| Passive | Métabolisme Adaptatif | Réduit les effets secondaires de toutes les drogues administrées de 50%. |
| Passive | Dosage Expert | Les buffs de drogues alliés durent 30% plus longtemps. |

### 6.4 Arbre 2 — Chimie Offensive

**Thème :** Acides, venins corrosifs, projectiles chimiques

| Type | Nom | Effet |
|------|-----|-------|
| Active | Jet d'Acide | Projette de l'acide (portée 8m, zone 2m) : 100% ADPhy en dégâts acides + réduit l'armure de la cible de 15% pendant 8s. |
| Active | Venin Paralysant | Projectile unique : 60% ADPhy + Paralysie (incapacité totale) 1,5s. Non résistible sur cibles debuffées. |
| Active | Bombe Corrosive | Grenade à acide (zone 4m, 6s) : toute armure exposée se dégrade de 5% par seconde dans la zone. |
| Passive | Réactions Chimiques | Si deux altérations chimiques différentes sont présentes sur une même cible, elles réagissent et infligent 50% ADPhy bonus chaque seconde. |
| Passive | Formules Concentrées | Les effets de durée des venins et acides sont augmentés de 25%. |

### 6.5 Arbre 3 — Contrôle de Zone Chimique

**Thème :** Gaz de zone, nuages chimiques, manipulation de l'environnement

| Type | Nom | Effet |
|------|-----|-------|
| Active | Nuage Soporifique | Nuage de gaz (5m, 10s) : les cibles à l'intérieur sont Ralentis de 40% et ont leurs régénérations réduites à 0. |
| Active | Fumigation Toxique | Zone de gaz empoisonné (4m, 8s) : 30% ADPhy par seconde en poison pour toutes cibles exposées. |
| Active | Brouillard Acide | Brouillard corrosif (6m, 12s) : réduit la vision à 3m et inflige 15% ADPhy/s sur les armures exposées. |
| Passive | Ventilation Maîtrisée | L'Alchimiste est immunisé à tous ses propres gaz et nuages. |
| Passive | Propagation Chimique | Les nuages se propagent de 1m supplémentaire si une compétence explosive (du Guerrillero ou d'un autre Outlaw) les touche. |

### 6.6 Compétences Signature

**Grand Cocktail** *(Cooldown : 90s)*
L'Alchimiste mélange instantanément jusqu'à 3 types de réactifs de son inventaire en une Bombe Composée. Les effets de la bombe dépendent des réactifs choisis (acide + venin = corrosion prolongée ; venin + stimulant = drogue de rage sur alliés ; acide + fumigation = brouillard acide renforcé). La zone d'impact est de 5m, et les effets composés durent 50% plus longtemps que les effets simples. Cela transforme l'Alchimiste en une plateforme de combat adaptative — le joueur doit connaître ses combinaisons.

**Dose Létale** *(Cooldown : 180s)*
L'Alchimiste dose une cible unique avec une injection concentrée au contact (mêlée requise). La cible ne ressent rien pendant 10s — puis subit l'intégralité de 400% ADPhy en dégâts de poison condensés. Durant ces 10s, tous les soins reçus par la cible sont convertis en dégâts supplémentaires au lieu de soigner. La cible ne peut pas détecter l'infection par les interfaces standard (aucun indicateur d'altération visible). Seul un Antidote Alchimique de rang 4+ peut purger l'effet avant qu'il se déclenche.

### 6.7 Interaction avec le système Outlaw

**Wanted Level :** La fabrication et l'utilisation de substances illégales augmentent le Wanted Level passivement si observées par un PNJ de faction. L'Alchimiste doit gérer son laboratoire mobile avec discrétion — se faire surprendre en train de synthétiser déclenche une alerte.

**Stealth :** Nuage Soporifique et Brouillard Acide servent de couverture de mouvement. Pas de furtivité directe mais un excellent contrôle de la ligne de vue.

**Marché Noir :** L'Alchimiste est un fournisseur central du marché noir — ses drogues de combat, venins et antidotes sont les marchandises les plus demandées de la faction criminelle. Il peut établir des contrats de fourniture réguliers avec des Seigneurs Outlaws pour un revenu passif significatif.

### 6.8 Chemin de rédemption préférentiel

**Empire Pourpre.** L'Empire dispose de la plus grande infrastructure de recherche alchimique de Véranthas — les Laboratoires Impériaux de Velanthara sont légendaires. Un Alchimiste Noir qui accepte de légaliser ses formules (en les cédant à l'Empire contre immunité partielle) devient un chercheur sous contrat impérial. Ses connaissances des substances illégales sont précisément ce que l'Empire veut étudier et contrôler.

### 6.9 Lore

Tous les Alchimistes Noirs ont commencé légalement. Quelque part dans leur passé, il y a un laboratoire officiel, un maître reconnu, une licence délivrée par une faction. Et quelque part entre ce début et leur vie actuelle dans les bas-fonds, une ligne a été franchie — une formule trop efficace pour être autorisée, un commanditaire qui a voulu des résultats impossibles à obtenir par des moyens propres, une catastrophe d'expérimentation qu'ils ne pouvaient pas avouer. Ils sont dans les égouts maintenant, et leurs formules marchent mieux que jamais, sans les contraintes institutionnelles qui les ralentissaient.

---

## 7. Berserker Banni — Tank/DPS Rage

### 7.1 Identité

**Rôle :** Tank/DPS Rage (absorption des dégâts, frénésie, survie par l'agression)
**Armes principales :** Hache à deux mains (ou hache + bouclier brisé pour variante défensive), Chaîne de combat (attaque à distance mêlée, outil de CC), Corps (combat à mains nues quand tout le reste est perdu)
**Couleur de classe :** Rouge sang (#DC143C) / Noir de suie (#2F4F4F)

### 7.2 Style de combat

Le Berserker Banni est la chose la plus proche d'une force de la nature que les Outlaws produisent. Il ne combat pas par stratégie — il combat par refus de s'arrêter. Sa mécanique centrale est la **Rage** : une jauge qui monte sous les dégâts reçus et alimente ses capacités offensives. Plus il prend de coups, plus il est dangereux. Cette dynamique perverse fait du Berserker un ennemi qui empire avec le temps — commencer un combat contre lui sans le finir rapidement est une erreur. Il est le Tank de la faction criminelle non par résistance passive mais par agression active : il force les adversaires à se concentrer sur lui parce que ne pas le faire coûte plus cher.

### 7.3 Arbre 1 — Frénésie

**Thème :** Rage pure, escalade de dégâts, états de frénésie active

| Type | Nom | Effet |
|------|-----|-------|
| Active | Déchaînement | Entre en Frénésie pendant 12s : vitesse d'attaque +40%, dégâts +30%, incapable de parer ou d'esquiver activement. |
| Active | Frappe du Désespoir | Inflige 180% ADPhy. Si le Berserker est en dessous de 40% PV, dégâts doublés à 360% ADPhy. |
| Active | Rugissement de Banni | Cri de combat dans un rayon de 8m : réduit la défense des ennemis de 15% pendant 6s et augmente sa propre Rage de 20 points. |
| Passive | Sang-Froid Criminel | Chaque dégât reçu génère 1 point de Rage (max 100). La Rage décroît de 5 points/s hors combat. |
| Passive | Frénésie Prolongée | Déchaînement dure 4s de plus si le Berserker est en dessous de 50% PV lors de l'activation. |

### 7.4 Arbre 2 — Absorption Bestiale

**Thème :** Résistance aux dégâts, survie, régénération au combat

| Type | Nom | Effet |
|------|-----|-------|
| Active | Peau de Pierre | Réduit tous les dégâts reçus de 40% pendant 6s. Accumule 10 Rage par seconde pendant l'activation. |
| Active | Ignorer la Douleur | Active pendant 4s : tous les dégâts reçus sont différés (stockés), puis divisés par 3 à la fin de l'effet. La Rage monte de la totalité des dégâts bruts. |
| Active | Récupération Sauvage | Consomme 30 points de Rage pour régénérer 20% PV instantanément. |
| Passive | Corps Endurci | Résistance aux effets de CC réduite de 30% (stun, knockback, immobilisation durent moins longtemps). |
| Passive | Dernière Limite | En dessous de 15% PV, tous les dégâts reçus sont réduits de 50%. |

### 7.5 Arbre 3 — Chaîne et Corps

**Thème :** Combat à la chaîne, mains nues, contrôle de mêlée

| Type | Nom | Effet |
|------|-----|-------|
| Active | Lancer de Chaîne | Lance la chaîne (portée 10m) : s'accroche à la cible et la tire de force jusqu'à 3m, inflige 120% ADPhy et Étourdit 0,8s. |
| Active | Entrave | Enroule la chaîne autour de la cible (mêlée requise) : Immobilisation 2s, la cible ne peut pas utiliser de compétences de dash. |
| Active | Frappe à Poings Nus | Si l'arme principale est brisée ou sacrifiée : frappe de poing 150% ADPhy + Knockback 2m. Cooldown 3s. |
| Passive | Maître de la Chaîne | Portée du Lancer de Chaîne augmentée à 14m. |
| Passive | Arme de Dernier Recours | Le combat à mains nues inflige +20% de dégâts si la Rage dépasse 70 points. |

### 7.6 Compétences Signature

**Bannissement de Soi** *(Cooldown : 180s)*
Le Berserker entre dans un état de transe de combat pendant 15s. Durant ces 15s : il est immunisé à toute mort (les dégâts qui l'amèneraient à 0 PV le mettent à 1 PV à la place, une seule fois par activation), sa Rage ne peut pas descendre en dessous de 50, toutes ses frappes ignorent 30% de l'armure adverse, et il est immunisé aux effets de peur, charme et confusion. À la fin des 15s, il subit un crash de 5s de fatigue (-50% toutes les stats). Cette compétence est la survie absolue d'un homme qui n'a plus rien à perdre — et qui le sait.

**Colère du Proscrit** *(Cooldown : 90s — déclenché par la Rage)*
Nécessite 80+ points de Rage pour activer. Le Berserker libère toute sa Rage accumulée en une frappe de zone de 5m autour de lui : 15% ADPhy par point de Rage libéré (max 1500% ADPhy à 100 Rage). Tous les ennemis touchés subissent un knockback de 4m. La Rage tombe à 0 après activation. Si au moins 3 ennemis sont touchés, le Berserker récupère 30% PV instantanément. C'est la compétence définitive du Berserker — une libération cathartique de tout ce qu'il a encaissé.

### 7.7 Interaction avec le système Outlaw

**Wanted Level :** Le Berserker est la classe qui préoccupe le moins le Wanted Level mécaniquement — non par discrétion, mais par indifférence. Ses compétences de survie lui permettent de supporter des confrontations directes avec les gardes que les autres classes fuiraient. Cela dit, son niveau de Wanted peut monter très vite (engagements à vue, destructions de terrain).

**Stealth :** Aucune. Le Berserker ne se cache pas. Il attend que les autres viennent.

**Marché Noir :** Client plutôt que fournisseur. Il achète des stimulants de l'Alchimiste Noir, des antidotes pour compenser les effets secondaires de sa Rage, des armures de fortune réparées. Il n'a pas de spécialité marchande mais il est souvent engagé comme protection musclée pour les convois du marché noir.

### 7.8 Chemin de rédemption préférentiel

**Guilde des Mercenaires.** La Guilde des Mercenaires a le cadre institutionnel idéal pour canaliser un Berserker : un contrat, une mission claire, un ennemi désigné. Le Berserker Banni qui cherche la rédemption ne veut pas de discours moraux — il veut quelque chose à faire. La Guilde lui offre ça. Ses anciens crimes sont traités comme de la "pratique de terrain". Son titre de Rédempteur porte moins de stigmate chez les Mercenaires que dans les autres factions.

### 7.9 Lore

Le Berserker Banni est celui que toutes les factions ont rejeté. Pas pour incompétence — souvent pour trop de compétence dans des contextes où la violence débordait de son rôle prévu. Un soldat impérial qui a continué à combattre après l'ordre de cessez-le-feu. Un garde de l'Alliance qui a survécu à un événement dont les autres témoins sont morts. Un mercenaire dont le contrat stipulait "neutraliser la menace" et qui a interprété le terme trop largement. Ils arrivent dans les rangs des Outlaws marqués — des cicatrices physiques, oui, mais surtout une relation avec la douleur que les gens normaux ne comprennent pas. La souffrance ne les arrête pas. Depuis longtemps, elle les alimente.

---

## 8. Compétences universelles Outlaw

Ces compétences sont accessibles à **toutes les classes Outlaw** sans exception, à partir du rang Renégat (niveau 5-10). Elles constituent le socle commun de la vie criminelle de Véranthas.

---

### 8.1 Pickpocket

**Catégorie :** Compétence active passive (disponible en Stealth ou proximité non-détectée)
**Prérequis :** Rang Renégat + au moins 1 niveau dans un arbre Stealth ou Furtivité

**Description mécanique :** Le joueur Outlaw interagit avec un joueur ou PNJ ciblé (doit être dans le dos ou invisible). Une vérification de compétence est effectuée (niveau de Pickpocket vs niveau de Vigilance de la cible). En cas de succès, 1 item aléatoire de l'inventaire ouvert de la cible est transféré dans l'inventaire de l'Outlaw. L'item est sélectionné aléatoirement parmi les emplacements d'inventaire accessibles (hors équipement fixé, hors emplacement verrouillé).

**Règles spécifiques :**
- La tentative génère un Wanted Level +0,5 si témoin PNJ présent
- Sur un joueur : si détecté, déclenche une alerte PvP (le joueur volé est marqué "Primed" — peut attaquer l'Outlaw sans pénalité Wanted)
- Cooldown : 30s par cible (un même joueur ne peut être volé qu'une fois toutes les 30s par le même Outlaw)
- Niveau maximal de la compétence : 5 (niveau 5 = accès aux emplacements verrouillés de catégorie "normale", pas les items équipés)

**Amélioration par classe :**
- Assassin : Pickpocket silencieux (aucune animation visible, difficulté de détection ×2)
- Contrebandier Alliance (rédemption) : peut identifier la valeur marchande de l'item avant de le prendre

---

### 8.2 Crochetage

**Catégorie :** Compétence active de terrain
**Prérequis :** Rang Renégat + kit de crochetage en inventaire (consommable, 10-20 utilisations)

**Description mécanique :** Interagit avec toute serrure dans le monde (coffres verrouillés, portes de service, accès aux zones restreintes). Un mini-jeu de timing apparaît : 3 à 7 goupilles doivent être maintenues dans une zone verte simultanément pendant 1 à 3 secondes selon la difficulté de la serrure.

**Niveaux de serrure :**

| Rang | Type | Difficulté | Wanted Level si raté |
|------|------|-----------|----------------------|
| 1 | Cadenas simple | 1 goupille, 3s | +0 |
| 2 | Serrure standard | 3 goupilles, 2s | +0 |
| 3 | Serrure de faction | 5 goupilles, 1,5s | +0,5 |
| 4 | Coffre de Seigneur | 7 goupilles, 1s | +1 |
| 5 | Vault criminel | 7 goupilles, 0,5s + séquence | +2 |

**Règles spécifiques :**
- Un échec casse une goupille du kit (durée de tentative ultérieure augmentée)
- Un échec critique (toutes goupilles cassées) déclenche une alarme locale
- En Zone de Refuge, crochetage sans risque de Wanted Level

---

### 8.3 Contrefaçon

**Catégorie :** Compétence de métier (atelier requis)
**Prérequis :** Rang Desperado + Atelier de Contrefaçon (installé dans une base Outlaw ou un logement privé)

**Description mécanique :** Permet de fabriquer de faux documents de faction :
- Faux Passeport de faction (accès aux villes de faction ennemie sans Wanted Level pendant 4h de jeu)
- Faux Permis d'Arme (neutralise le Wanted Level lié au port d'arme en zone civile)
- Faux Mandat de Mission (permet de récupérer des coffres de quête assignés à d'autres joueurs)
- Faux Titre de Membre de Guilde (accès partiel aux fonctionnalités des Guildes Aventuriers et Mercenaires)

**Règles de détection :**
- Chaque document a un niveau de qualité (1-5) déterminé par la compétence du fabricant
- Les gardes PNJ vérifient les documents avec une précision liée au rang de la zone
- Un document détecté comme faux = Wanted Level +2 + confiscation + potentiellement arrestation PNJ

**Délai de fabrication :** 10 minutes de jeu par document, matériaux requis (encre, parchemin de faction, sceau de cire spécifique).

---

### 8.4 Trafic de Rue

**Catégorie :** Compétence économique permanente
**Prérequis :** Rang Renégat (accès automatique à l'entrée dans les Zones de Refuge)

**Description mécanique :** Permet d'accéder au Marché Noir parallèle de Véranthas — une interface commerciale distincte du marché public des factions. Le Marché Noir contient :

**Offre permanente du Marché Noir :**
- Items de rang 4-5 non craftables légalement (poisons, explosifs, fragments osseux rares, stimulants illégaux)
- Items de faction ennemie revendus par des intermédiaires criminels (à 150% du prix de faction)
- Équipements volés (prix ×0,5 vs valeur marchande, état dégradé aléatoire)
- Informations sur les convois et les patrouilles (achetables comme données stratégiques)
- Réduction de Wanted Level (service payant : 500 pièces d'or par niveau de Wanted Level réduit)

**Fonctionnement :**
- Chaque Zone de Refuge possède son propre vendeur de Marché Noir avec un stock rotatif (rafraîchi toutes les 6h de jeu)
- Les Outlaws peuvent poster des annonces de vente sur le Marché Noir (commission 10% prélevée par la Confrérie)
- Les achats importants (>1000 pièces d'or) génèrent un Wanted Level +0,5 si un informateur est présent

---

## 9. Système Wanted Level — Interactions complètes

Le Wanted Level (WL) est la mécanique centrale de la vie Outlaw. Il va de 0 à 5 et affecte l'expérience de jeu de façon fondamentale. Voici les interactions par classe et par niveau.

---

### 9.1 Wanted Level 0 — Inconnu

**État :** L'Outlaw est inconnu des autorités locales ou ses crimes sont prescrits.

| Classe | Comportement spécifique |
|--------|------------------------|
| Assassin | Voile d'Ombre dispose de toute sa durée normale. Pickpocket sans risque en zone urbaine. |
| Rôdeur Maudit | Pièges placés en zone neutre sans déclenchement d'alerte. Accès libre aux herboristes légaux. |
| Nécromancien des Rues | Peut traverser les villes sans être interpellé (à condition de ne pas invoquer). |
| Guerrillero | Armes dissimulables en zone civile sans contrôle. Explosifs tolérés si emballés. |
| Alchimiste Noir | Fioles et réactifs passent les contrôles de base non scrutateurs. |
| Berserker Banni | Hache doit être gainée en zone civile. Corps à corps ignoré si non-initié. |

**Conseil général :** Le WL 0 est l'état optimal pour les activités d'infiltration, Pickpocket, Crochetage et Contrefaçon.

---

### 9.2 Wanted Level 1 — Suspect

**État :** Quelques incidents signalés. Les gardes observent mais n'interpellent pas encore.

**Déclencheurs communs :** Un meurtre de PNJ civil, Pickpocket détecté, Crochetage raté en zone publique, explosion en zone civile.

**Effets globaux :** Les gardes regardent l'Outlaw quand il passe. Certains marchands PNJ de faction refusent de traiter avec lui.

| Classe | Comportement spécifique |
|--------|------------------------|
| Assassin | Voile d'Ombre interrompue si un garde regarde dans sa direction dans un rayon de 5m. |
| Rôdeur Maudit | Les traces de pièges déclenchent une patrouille supplémentaire dans la zone. |
| Nécromancien des Rues | Les fragments osseux visibles en inventaire déclenchent une interpellation PNJ. |
| Guerrillero | Port d'arme visible = interpellation. Doit gaîner l'arbalète. |
| Alchimiste Noir | Fioles d'acide = confiscation si fouillé. |
| Berserker Banni | Toute confrontation physique déclenche une alerte même si non-combat. |

---

### 9.3 Wanted Level 2 — Recherché

**État :** Avis de recherche affiché. Les gardes s'approchent pour vérification d'identité.

**Déclencheurs communs :** 3+ kills PvP consécutifs, Crochetage de coffre de faction, utilisation publique de Contrefaçon détectée.

**Effets globaux :** Accès aux zones marchandes des factions restreint (besoin d'un Faux Passeport WL2 pour entrer). Les portails de voyage rapide refusent les Outlaws WL2+.

| Classe | Comportement spécifique |
|--------|------------------------|
| Assassin | Cooldown de Disparition réduit de 15s (bonus — la pression augmente les réflexes). |
| Rôdeur Maudit | Camouflage de Ruines dure 3s de plus en zone urbaine (adaptation à la traque). |
| Nécromancien des Rues | Les serviteurs osseux déclenchent une attaque immédiate des gardes proches si visibles. |
| Guerrillero | Explosifs en inventaire = arrestation immédiate si fouillé. |
| Alchimiste Noir | Peut vendre ses drogues au Marché Noir à +20% prix (sa réputation criminelle augmente sa valeur). |
| Berserker Banni | Sa présence dans une taverne vide la salle en 30s (PNJ civils fuient). |

---

### 9.4 Wanted Level 3 — Prime Active

**État :** Une prime est posée sur la tête de l'Outlaw. Des chasseurs de primes PNJ et joueurs commencent à le pister.

**Déclencheurs communs :** Assassinat d'un officier de faction, pillage d'un convoi officiel, fraude à grande échelle de Contrefaçon.

**Effets globaux :** Les chasseurs de primes PNJ apparaissent dans toutes les zones (niveau moyen du joueur +5). Les joueurs des factions peuvent attaquer l'Outlaw en zone neutre sans pénalité. Montant de la prime : 200-500 pièces d'or selon les crimes.

| Classe | Comportement spécifique |
|--------|------------------------|
| Assassin | Contrat Scellé est libre de cooldown une fois par heure si la cible est un chasseur de primes. |
| Rôdeur Maudit | Territoire Marqué peut être utilisé en zone neutre sans restriction (urgence justifiée). |
| Nécromancien des Rues | Linceul du Désespoir sur un chasseur de primes génère 2 Fragments osseux si la cible meurt. |
| Guerrillero | Sprint de Guerre ne déclenche pas d'alerte de zone si utilisé pour fuir (traçabilité réduite). |
| Alchimiste Noir | Dose Létale sur un chasseur de primes ne déclenche pas de Wanted Level supplémentaire. |
| Berserker Banni | Colère du Proscrit contre des chasseurs de primes génère 50% de la valeur de la prime en pièces d'or. |

---

### 9.5 Wanted Level 4 — Criminel de Guerre

**État :** Le nom de l'Outlaw est dans tous les registres de faction. Il ne peut entrer dans aucune ville de faction sans déguisement.

**Déclencheurs communs :** Massacre de civils PNJ, destruction d'un bâtiment de faction, participation prouvée à une attaque de convoi majeur.

**Effets globaux :** Accès aux villes bloqué même avec Faux Passeport standard (besoin d'un Passeport WL4+, rare). Les gardes de faction attaquent à vue en toute zone semi-neutre. Les chasseurs de primes PNJ sont de niveau moyen +10.

| Classe | Comportement spécifique |
|--------|------------------------|
| Assassin | Instinct du Prédateur s'active en permanence (comme si 2 altérations présentes sur toutes les cibles). |
| Rôdeur Maudit | Traqueur Né voit aussi les chasseurs de primes PNJ sur la carte dans un rayon de 50m. |
| Nécromancien des Rues | Accumulation Cadavérique génère 2 fragments par mort de chasseur de primes. |
| Guerrillero | Tous les explosifs ont +20% de rayon d'effet (désespoir = précision inutile). |
| Alchimiste Noir | Grand Cocktail peut combiner 4 réactifs au lieu de 3 (la maîtrise s'aiguise sous pression). |
| Berserker Banni | Bannissement de Soi se déclenche automatiquement à 5% PV au lieu d'être activé manuellement. |

---

### 9.6 Wanted Level 5 — Seigneur Criminel

**État :** Le plus haut niveau de criminalité. L'Outlaw est une priorité nationale pour toutes les factions simultanément. Sa tête vaut une fortune.

**Déclencheurs communs :** Actes catastrophiques (destruction de bâtiment emblématique, assassinat d'un PNJ de rang Seigneur ou supérieur, perturbation d'un événement de faction majeur).

**Effets globaux :** Prime maximale (5000+ pièces d'or). Toutes les factions coopèrent pour sa capture. Des escouades PNJ d'élite (niveau +15) sont déployées. Un chronomètre de 24h commence — si l'Outlaw survit 24h en WL5, il monte au rang Seigneur Outlaw dans la hiérarchie de la Confrérie. Une cérémonie publique dans Cendrepas reconnaît son statut.

| Classe | Comportement spécifique |
|--------|------------------------|
| Assassin | Voile d'Ombre dure 20s au lieu de 12s (survie maximale). Disparition est à cooldown réduit de 15s. |
| Rôdeur Maudit | Embuscade Parfaite peut être utilisée 2 fois de suite sans recharge (état d'urgence total). |
| Nécromancien des Rues | Armée de Fortune invoque 7 serviteurs maximum au lieu de 5 (résonance des morts avec l'intensité du moment). |
| Guerrillero | Blocus place 5 bombes au lieu de 3 (la panique génère l'inspiration). |
| Alchimiste Noir | Toutes les durées de gaz et venins doublent (formulation à l'adrénaline). |
| Berserker Banni | La Rage monte deux fois plus vite. Colère du Proscrit peut être utilisée à partir de 50 Rage. |

**Rédemption depuis WL5 :** Possible mais d'un coût écrasant. La Quête de Rédemption WL5 requiert un exploit public de rang légendaire (stopper seul une Stampede émergente, livrer un artefact de Garum aux autorités de faction, ou sauver une zone entière d'un désastre). Aucune rédemption WL5 ne s'est conclue en moins de 3 mois de jeu réel dans les archives de la Confrérie.

---

## 10. Schémas TOML complets

### 10.1 Classe Outlaw — Schéma de base

```toml
[class]
id = "assassin"
faction = "outlaws"
display_name = "Assassin"
display_name_short = "Assassin"
role = "dps_burst_stealth"
color_primary = "#1A1A1A"
color_secondary = "#7B2D8B"
weapons_primary = ["dague", "dague_reserve", "epee_courte"]
resource_primary = "tension"
resource_secondary = "wanted_level"
campaign_start_zone = "egouts_velmo"
start_rank = "banni"

[class.stats_base]
ferrath_anth = 8        # Force — Faible (l'Assassin ne mise pas sur le brut)
velthar_sorath = 18     # Agilité — Très élevée (sa survie)
gaiathar_nexis = 14     # Intelligence — Élevée (poisons, préparation)
rhathar_veines = 10     # Constitution — Moyenne

[class.skill_trees]
tree_1 = "art_de_lombre"
tree_2 = "alchimie_du_tranchant"
tree_3 = "execution_parfaite"

[class.signature_skills]
sig_1 = "contrat_scelle"
sig_2 = "disparition"

[class.universal_skills]
pickpocket = true
crochetage = true
contrefacon = true
trafic_de_rue = true

[class.redemption]
preferred_faction = "guilde_aventuriers"
redemption_cost_modifier = 1.0   # Coût de rédemption standard
title_post_redemption = "redempteur_de_lombre"
```

---

### 10.2 Rodeur Maudit — Schéma

```toml
[class]
id = "rodeur_maudit"
faction = "outlaws"
display_name = "Rôdeur Maudit"
display_name_short = "Rôdeur"
role = "dps_controle_terrain"
color_primary = "#8B4513"
color_secondary = "#228B22"
weapons_primary = ["arc_court_survie", "couteau_chasse", "pieges_artisanaux"]
resource_primary = "pieges_actifs"
resource_secondary = "wanted_level"
campaign_start_zone = "tunnels_contrebande_cendrepas"
start_rank = "banni"

[class.stats_base]
ferrath_anth = 10       # Force — Moyenne
velthar_sorath = 15     # Agilité — Élevée
gaiathar_nexis = 13     # Intelligence — Bonne (tactique terrain)
rhathar_veines = 14     # Constitution — Bonne (survie en zone hostile)

[class.skill_trees]
tree_1 = "maitrise_des_pieges"
tree_2 = "survie_en_zone_hostile"
tree_3 = "tir_de_survie"

[class.signature_skills]
sig_1 = "territoire_marque"
sig_2 = "embuscade_parfaite"

[class.universal_skills]
pickpocket = false      # Non spécialisé
crochetage = true       # Expert terrain
contrefacon = false
trafic_de_rue = true

[class.special_passive]
zone_hostile_bonus = true          # Double résistance en zones corrompues
piege_detection_radius = 12        # Voit les pièges adverses à 12m
immune_own_poison_terrain = true   # Immunisé à ses propres zones toxiques

[class.redemption]
preferred_faction = "guilde_mercenaires"
redemption_cost_modifier = 0.9    # Légèrement moins cher (compétences utiles)
title_post_redemption = "traqueur_de_contrat"
```

---

### 10.3 Nécromancien des Rues — Schéma

```toml
[class]
id = "necromancien_des_rues"
faction = "outlaws"
display_name = "Nécromancien des Rues"
display_name_short = "Nécro"
role = "support_hybride_mort"
color_primary = "#696969"
color_secondary = "#7FFF00"
weapons_primary = ["baton_rituel_court", "fragments_osseux", "dague_sacrifice"]
resource_primary = "fragments_osseux"
resource_secondary = "wanted_level"
campaign_start_zone = "catacombes_velmo"
start_rank = "banni"

[class.stats_base]
ferrath_anth = 7        # Force — Faible
velthar_sorath = 10     # Agilité — Faible-moyenne
gaiathar_nexis = 20     # Intelligence — Maximale (magie nécrotique)
rhathar_veines = 13     # Constitution — Moyenne

[class.invocation_cap]
serviteurs_max = 4
fragments_max = 5
mort_passive_trigger = true       # Accumulation Cadavérique activée

[class.wanted_level_modifier]
invocation_en_zone_publique = +1.0  # Par invocation visible
rituel_en_zone_publique = +0.5      # Par rituel visible

[class.redemption]
preferred_faction = "federation_ervan"
redemption_cost_modifier = 1.2   # Plus cher (nécromance mal vue partout)
title_post_redemption = "eclaireur_des_cendres"
```

---

### 10.4 Guerrillero — Schéma

```toml
[class]
id = "guerrillero"
faction = "outlaws"
display_name = "Guerrillero"
display_name_short = "Guerrillero"
role = "dps_mobilite_embuscade"
color_primary = "#8B0000"
color_secondary = "#FF4500"
weapons_primary = ["sabre_court_rue", "arbalete_legere_poing", "explosifs_improvises"]
resource_primary = "explosifs_stock"
resource_secondary = "wanted_level"
campaign_start_zone = "quartier_basse_cendrepas"
start_rank = "banni"

[class.stats_base]
ferrath_anth = 13       # Force — Bonne (corps à corps secondaire)
velthar_sorath = 16     # Agilité — Très élevée (mobilité)
gaiathar_nexis = 11     # Intelligence — Moyenne (improvisation)
rhathar_veines = 12     # Constitution — Moyenne

[class.explosifs_config]
max_stock = 10
craft_time_base = 60              # Secondes
craft_time_reduction = 0.5        # Via Synthèse Rapide
wanted_gain_explosion_civil = 1.0 # Par explosion en zone civile

[class.terrain_bonus]
urban_speed_bonus = 0.10          # +10% en zone urbaine connue
vertical_movement = true          # Escalade Éclair disponible

[class.redemption]
preferred_faction = "alliance_de_rive"
redemption_cost_modifier = 0.85   # Moins cher (compétences directement valorisées)
title_post_redemption = "franchise_de_guerre"
```

---

### 10.5 Alchimiste Noir — Schéma

```toml
[class]
id = "alchimiste_noir"
faction = "outlaws"
display_name = "Alchimiste Noir"
display_name_short = "Alchimiste"
role = "support_hybride_alteration"
color_primary = "#9400D3"
color_secondary = "#FFD700"
weapons_primary = ["alambic_combat", "couteau_analyse"]
resource_primary = "fioles_inventaire"
resource_secondary = "wanted_level"
campaign_start_zone = "laboratoire_souterrain_velmo"
start_rank = "banni"

[class.stats_base]
ferrath_anth = 7        # Force — Faible
velthar_sorath = 11     # Agilité — Faible-moyenne
gaiathar_nexis = 19     # Intelligence — Quasi maximale
rhathar_veines = 11     # Constitution — Faible-moyenne

[class.inventory_config]
fioles_slots = 12
reactifs_categories = ["acide", "venin", "stimulant", "analgésique", "gaz", "smoke"]
grand_cocktail_max_reactifs = 3
grand_cocktail_max_reactifs_wl4 = 4

[class.market_noir_config]
vendor_bonus = 0.20               # +20% prix de vente au marché noir (WL2+)
supply_contract_available = true  # Contrats de fourniture aux Seigneurs Outlaws

[class.wanted_level_modifier]
synthesis_observed = +0.5         # Si observé en train de synthétiser
illegal_substance_found = +1.0    # Si fouillé et substances trouvées

[class.redemption]
preferred_faction = "empire_pourpre"
redemption_cost_modifier = 1.3   # Plus cher (formules à céder)
title_post_redemption = "chercheur_sous_contrat_imperial"
```

---

### 10.6 Berserker Banni — Schéma

```toml
[class]
id = "berserker_banni"
faction = "outlaws"
display_name = "Berserker Banni"
display_name_short = "Berserker"
role = "tank_dps_rage"
color_primary = "#DC143C"
color_secondary = "#2F4F4F"
weapons_primary = ["hache_deux_mains", "chaine_combat", "corps"]
resource_primary = "rage"
resource_secondary = "wanted_level"
campaign_start_zone = "arene_souterraine_cendrepas"
start_rank = "banni"

[class.stats_base]
ferrath_anth = 20       # Force — Maximale
velthar_sorath = 9      # Agilité — Faible
gaiathar_nexis = 7      # Intelligence — Faible (la rage n'a pas besoin de stratégie)
rhathar_veines = 20     # Constitution — Maximale

[class.rage_config]
rage_max = 100
rage_per_damage_taken = 1         # 1 Rage par point de dégât reçu
rage_decay_out_of_combat = 5      # -5 Rage/s hors combat
rage_wl5_multiplier = 2.0         # Double à WL5

[class.survival_config]
bannissement_soi_max_survivals = 1         # Une seule immunité par activation
derniere_limite_threshold = 0.15           # Déclenché en dessous de 15% PV
bannissement_soi_auto_threshold_wl4 = 0.05 # Automatique à WL4 à 5% PV

[class.stealth]
stealth_available = false
stealth_equivalent = "none"       # Aucun équivalent — le Berserker ne se cache pas

[class.redemption]
preferred_faction = "guilde_mercenaires"
redemption_cost_modifier = 0.80   # Le moins cher (la Guilde n'est pas moralisatrice)
title_post_redemption = "banni_sous_contrat"
```

---

### 10.7 Compétences Universelles — Schéma

```toml
[universal_skill.pickpocket]
id = "pickpocket"
faction_restriction = "outlaws"
rank_required = "renégat"
stealth_required = true           # Ou dos de la cible
cooldown_per_target = 30          # Secondes
max_level = 5
level_5_unlock = "emplacements_verrouilles_normaux"
wanted_gain_if_witnessed = 0.5

[universal_skill.crochetage]
id = "crochetage"
faction_restriction = "outlaws"
rank_required = "renégat"
kit_required = "kit_de_crochetage"
kit_uses_range = [10, 20]
mini_jeu_type = "timing_goupilles"
lock_ranks = [1, 2, 3, 4, 5]
wanted_gain_if_failed_public = [0.0, 0.0, 0.5, 1.0, 2.0]
wanted_gain_alarm_critical = 1.0

[universal_skill.contrefacon]
id = "contrefacon"
faction_restriction = "outlaws"
rank_required = "desperado"
atelier_required = "atelier_contrefacon"
craft_time_minutes = 10
documents_available = [
  "faux_passeport_wl1",
  "faux_passeport_wl2",
  "faux_passeport_wl4",
  "faux_permis_arme",
  "faux_mandat_mission",
  "faux_titre_guilde_aventuriers",
  "faux_titre_guilde_mercenaires"
]
detection_system = "qualite_vs_rang_garde"
detected_wanted_gain = 2.0

[universal_skill.trafic_de_rue]
id = "trafic_de_rue"
faction_restriction = "outlaws"
rank_required = "renégat"
access_type = "marche_noir_interface"
stock_refresh_interval_hours = 6
commission_rate = 0.10
large_transaction_threshold = 1000
large_transaction_wanted_gain = 0.5
reduce_wanted_service = true
reduce_wanted_cost_per_level = 500  # Pièces d'or par niveau de WL réduit
```

---

### 10.8 Système Wanted Level — Schéma global

```toml
[wanted_system]
min_level = 0
max_level = 5

[[wanted_system.level]]
level = 0
name = "Inconnu"
guard_behavior = "ignore"
faction_access = "full"
travel_fast = true
bounty_hunters = false
prime_amount = 0

[[wanted_system.level]]
level = 1
name = "Suspect"
guard_behavior = "observation"
faction_access = "full"
travel_fast = true
bounty_hunters = false
prime_amount = 0
npc_merchant_refusal_rate = 0.20

[[wanted_system.level]]
level = 2
name = "Recherché"
guard_behavior = "verification_identite"
faction_access = "restricted"
travel_fast = false
bounty_hunters = false
prime_amount = 0
passport_required = "faux_passeport_wl2"
market_noir_sell_bonus = 0.20

[[wanted_system.level]]
level = 3
name = "Prime Active"
guard_behavior = "attaque_en_zone_neutre"
faction_access = "blocked"
travel_fast = false
bounty_hunters = true
bounty_hunter_level_bonus = 5
prime_amount_range = [200, 500]
players_can_attack_neutral = true

[[wanted_system.level]]
level = 4
name = "Criminel de Guerre"
guard_behavior = "attaque_vue_zone_semi_neutre"
faction_access = "blocked_all"
travel_fast = false
bounty_hunters = true
bounty_hunter_level_bonus = 10
prime_amount_range = [500, 2000]
passport_required = "faux_passeport_wl4"
class_power_bonus = true

[[wanted_system.level]]
level = 5
name = "Seigneur Criminel"
guard_behavior = "escouades_elite"
faction_access = "blocked_all"
travel_fast = false
bounty_hunters = true
bounty_hunter_level_bonus = 15
prime_amount_min = 5000
factions_cooperate = true
survival_timer_hours = 24
survival_reward = "rang_seigneur_outlaw"
survival_ceremony = "cendrepas_public"
redemption_difficulty = "legendaire"
redemption_min_playtime_months = 3
```

---

*Fin du document AL-Character-Outlaws.md*
*Véranthas, An 247 AO — Référence canonique v1.0*
*Auteur : Équipe Game Design Allumina — 2026-02-28*
