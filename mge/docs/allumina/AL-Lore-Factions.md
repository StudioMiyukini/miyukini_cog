<!-- @id: AL-Lore-Factions @do: reference @role: game-designer @layer: 3 @human: miyuk -->

# Allumina — Lore & Factions

**Statut :** Référence canonique v1.0
**Date :** 2026-02-28
**Scope :** Cosmologie, histoire du monde, six factions, géographie, schémas TOML

---

## Table des matières

1. [Vision & Pitch](#1-vision--pitch)
2. [Histoire du Monde — Ères](#2-histoire-du-monde--ères)
3. [Géographie — Carte textuelle](#3-géographie--carte-textuelle)
4. [Cosmologie — Gaïa & Garum](#4-cosmologie--gaïa--garum)
5. [Empire Pourpre](#5-empire-pourpre)
6. [Alliance de Rive](#6-alliance-de-rive)
7. [Fédération Ervan](#7-fédération-ervan)
8. [Guilde des Aventuriers](#8-guilde-des-aventuriers)
9. [Guilde des Mercenaires](#9-guilde-des-mercenaires)
10. [Outlaws — La Confrérie de l'Ombre Libre](#10-outlaws--la-confrérie-de-lombre-libre)
11. [Relations inter-factions](#11-relations-inter-factions)
12. [Schémas TOML](#12-schémas-toml)

---

## 1. Vision & Pitch

### 1.1 Pitch court

Allumina est un MMO-ARPG médiéval-fantasy isométrique où trois empires rivaux se disputent un monde lentement dévoré par une corruption divine, où les guildes neutres basculent les guerres, et où même un criminel peut se racheter — si le prix est suffisamment élevé.

### 1.2 Pitch long

Dans le monde de **Véranthas**, la déesse-terre Gaïa tisse la vie dans chaque racine, chaque source et chaque soufle de vent. Mais depuis les Trois Maux — trois catastrophes consécutives qui ont fracturé l'ancien empire unifié — une force opposée s'est éveillée dans les profondeurs : **Garum**, l'Érosion, un principe cosmique de dissolution et de corruption.

Allumina n'est pas l'histoire d'un héros solitaire sauvant le monde. C'est l'histoire de **civilisations entières** qui survivent, s'affrontent, trahissent et s'allient dans un monde où la corruption progresse depuis les failles souterraines, où les donjons "débordent" en surface sous forme de **Stampedes** cataclysmiques, et où la paix n'est jamais qu'un armistice précaire entre des peuples aux valeurs irréconciliables.

Le joueur choisit une faction, forge son identité sociale (Citoyen, Homme libre, Habitant, Aventurier, Mercenaire, ou Outlaw), et participe à une guerre vivante dont l'issue change chaque saison. La progression est horizontale autant que verticale : un marchand expert vaut autant qu'un général de guerre. La mort a du sens. La rédemption est possible — mais publique et coûteuse.

**Allumina, c'est DAoC + Ragnarok Online + Ultima Online + Diablo II — fusionnés dans un moteur entièrement Rust, data-driven, sans dette technique.**

### 1.3 Positionnement concurrentiel

| Inspirateur | Ce qu'Allumina emprunte |
|-------------|------------------------|
| **Dark Age of Camelot** | RvR à trois factions, zones de guerre dynamiques, importance de la guilde |
| **Ragnarok Online** | Rangs de guilde, Hall of Fame saisonnier, feel isométrique 2D |
| **Ultima Online** | Full-loot PvP, économie joueur, criminalité et rédemption, housing |
| **Diablo II** | ARPG de fond, Stampedes = donjons qui débordent, loot tables riches |

### 1.4 Piliers de game design

1. **La guerre est permanente mais non totale.** Les trois factions principales sont en guerre chronique, mais des zones neutres commerciales persistent. Les mercenaires sont le thermomètre des alliances.
2. **Chaque identité sociale a une valeur.** Marchand, soldat, diplomate, criminel : chaque rôle est viable et impactant sur le monde.
3. **La corruption est une menace partagée.** Garum est l'ennemi commun — les Stampedes forcent la coopération temporaire même entre factions ennemies.
4. **La réputation se construit publiquement.** Trahison, rédemption, montée en rang : tout est visible dans les capitales et les Hall of Fame.
5. **Le monde vieillit.** Les zones corrompues progressent si ignorées. Les bâtiments peuvent être détruits, reconquis, reconstruits.
6. **La mort est un coût, pas un inconvénient.** Full-loot en zones PvP, perte d'XP en zones corrompues, corps récupérable.
7. **Chaque saison raconte une histoire.** Les guerres ont des arcs narratifs saisonniers, avec des vainqueurs, des traîtres célèbres, et un Hall of Fame qui entre dans la mémoire collective du jeu.

### 1.5 Public cible

- Joueurs MMO vétérans (25-40 ans) nostalgiques de l'âge d'or UO/DAoC/RO
- Joueurs ARPG hardcore cherchant une profondeur sociale
- Joueurs RvR cherchant des guerres à enjeux réels
- Joueurs sandbox cherchant une économie vivante et un système criminel cohérent

### 1.6 Différenciateurs clés

- **Stampedes** : événement unique où un donjon "déborde" en surface — menace PvE majeure qui suspend temporairement les conflits PvP
- **Guilde des Mercenaires** : facteur de bascule narratif — une faction qui perd trop de mercenaires peut perdre une guerre entière
- **Rédemption publique** : devenir Outlaw est facile, en sortir est une cérémonie publique humiliante et coûteuse dans la capitale de la faction choisie
- **Moteur Rust natif** : pas de GC, performances déterministes, tick serveur stable même sous charge massive

---

## 2. Histoire du Monde — Ères

### 2.1 Le monde : Véranthas

**Véranthas** (du proto-ervan *véra* = vérité, *anthas* = terre) est un monde de taille continentale, à dominante terrestre avec deux grandes mers intérieures. Sa géologie est marquée par un réseau de **failles telluriques** appelées les **Veines Grises** — des fractures profondes d'où remontent parfois des énergies corrompues. Les anciens savants pensaient que ces veines étaient les artères d'un être vivant plus grand. Ils n'avaient pas tout à fait tort.

Véranthas tourne autour d'une étoile jaune, **Sorath**, avec une inclinaison axiale marquée qui génère quatre saisons prononcées. Les nuits d'hiver voient apparaître **l'Aurore de Cendre** — un phénomène lumineux vert-gris qui, selon la théologie gaïenne, est le souffle de Garum cherchant à éteindre les étoiles une à une.

---

### 2.2 Ère de la Lumière Première (Avant l'écriture)

Avant que les mortels ne tracent la moindre rune, Véranthas était une symbiose parfaite. Gaïa — ni dieu ni déesse au sens strict, mais **principe vivant de cohésion** — maintenait l'équilibre des écosystèmes, guidait les migrations des créatures, faisait croître les forêts et purifiait les eaux. Les premiers humains vivaient en clans nomades, cueilleurs et chasseurs, qui vénéraient les **Nexus** : des lieux naturels d'une puissance exceptionnelle — cascades, volcans dormants, forêts millénaires — où la présence de Gaïa se concentrait physiquement.

Ces Nexus sont les **Sanctuaires Naturels** actuels. Ils n'ont jamais été construits : ils ont été *reconnus*.

Les traditions orales de la Fédération Ervan préservent des récits de cette époque sous forme de chants appelés les **Mémoires Vertes**. Leur authenticité est débattue, mais leur cohérence interne sur plusieurs millénaires de transmission est troublante.

---

### 2.3 Ère de la Pierre Montante (Années -2000 à -500 AO)

*AO = Avant l'Obscurcissement, le calendrier commun adopté après les Trois Maux.*

Les clans nomades se sédentarisent. Les premières cités émergent autour des Nexus : Ervanath (future capitale de la Fédération), Auranthos (future Alliance de Rive), et Veranthas-Majeur (future capitale impériale). Ces trois cités forment un triangle commercial et culturel informel connu sous le nom de **Triumvirat de Pierre**.

Durant cette ère, un phénomène nouveau apparaît : des créatures monstrueuses commencent à remonter des Veines Grises. Elles sont rares, confuses, comme des animaux malades. Les druides ervan les nomment les **Éraillés** — des êtres dont l'essence vitale a été corrompue à la source. Ce sont les premiers indices de Garum.

L'Ère de la Pierre Montante voit aussi la naissance des premiers **mages** : humains capables d'interagir directement avec l'énergie des Nexus. La Fédération les accueille. Les proto-impériaux les craignent. Les marchands d'Auranthos les emploient.

---

### 2.4 Ère de l'Empire Uni (Années -500 à -1 AO)

Un conquérant charismatique nommé **Velanthor le Pourpre** unifie militairement les trois cités et fonde le **Premier Empire de Véranthas**. Son titre complet : *Imperator Solaire, Gardien des Trois Cités, Pont entre Gaïa et les Hommes*.

Velanthor n'est pas un tyran. C'est un administrateur brillant qui crée :
- Le premier réseau de routes pavées reliant les trois cités
- Un code de lois commun (le **Codex Velanthor**, toujours en vigueur partiel dans l'Empire Pourpre actuel)
- Un système de taxation équitable basé sur le commerce plutôt que sur la propriété foncière
- Les premières **Académies Mixtes** où mages et ingénieurs militaires travaillent ensemble

Cette coopération entre magie et technologie militaire produit les premières **armes de siège animées** — préfiguration du steampunk martial de l'Empire Pourpre actuel.

L'Empire Uni dure 499 ans. Il s'effondre en trois catastrophes successives : les **Trois Maux**.

---

### 2.5 Les Trois Maux (Années -1 à +47 AO)

Les Trois Maux sont la fracture fondatrice de l'histoire moderne de Véranthas. Chaque Mal est une catastrophe distincte, mais les historiens débattent encore si elles sont causalement liées ou simplement consécutives.

#### Premier Mal : La Peste de Cendre (An 0-7 AO)

Une maladie inconnue se propage depuis les Veines Grises du centre du continent. Elle ne tue pas immédiatement : elle **corrompue** progressivement, transformant les victimes en créatures grises, lentes, sans volonté propre — les **Cendrés**. Ils ne souffrent pas. Ils errent. Et ils contaminent au toucher.

La Peste de Cendre tue un tiers de la population de l'Empire Uni en sept ans. La cité de Veranthas-Majeur est la plus touchée — elle doit être partiellement abandonnée et reconstruite plus au nord, fondant ce qui deviendra **Velanthara**, la capitale actuelle de l'Empire Pourpre.

Les archives impériales attribuent la guérison finale à un rituel de masse effectué par les druides ervan à partir de cinq Nexus simultanément — ce que les Ervans appellent le **Tissage des Cinq Voix**. L'Empire Uni, fier de sa rationalité, efface cet événement de ses chroniques officielles. La Fédération Ervan ne l'oubliera jamais.

#### Deuxième Mal : La Guerre des Mages Noirs (An 12-31 AO)

Parmi les mages formés dans les Académies Mixtes, un courant philosophique radical émerge : les **Nécroliciens**, qui théorisent que l'énergie des Cendrés n'est pas corrompue mais *libérée*, et qu'elle peut être maîtrisée et réorientée. Leur chef, **Ardath Vorak**, parvient à construire une armée de Cendrés conscients — les **Volontaires** — et revendique l'indépendance d'une zone côtière qu'il nomme **Vorakis**.

La guerre qui s'ensuit dure dix-neuf ans. Ardath Vorak est finalement défait par une coalition des trois cités, mais Vorakis est détruite et son emplacement est aujourd'hui la **Mer de Cendre** — une étendue d'eau grise et stérile à l'est du continent, où rien ne croît et où les poissons naissent déformés. C'est la plus grande zone corrompue permanente de Véranthas.

La Guerre des Mages Noirs laisse un traumatisme durable dans la culture impériale : toute magie jugée "non naturelle" est désormais suspecte. C'est l'origine profonde du **méfiance envers la magie** de l'Empire Pourpre.

#### Troisième Mal : Le Grand Silence (An 38-47 AO)

Le plus mystérieux des Trois Maux. Pendant neuf ans, les Nexus s'éteignent un à un. Les druides perdent leur connexion à Gaïa. Les mages ressentent un vide dans les lignes d'énergie. Les sanctuaires naturels se taisent.

Personne ne comprend ce qui se passe. Pas de monstre à combattre, pas de maladie à soigner. Juste... le silence.

Les récoltes échouent. Les rivières réduisent leur débit. Les naissances d'animaux diminuent. Ce n'est pas une catastrophe brutale — c'est une **érosion lente**, comme si le monde retenait son souffle.

Au bout de neuf ans, les Nexus se rallument — mais différemment. Plus intenses dans certains lieux, éteints à jamais dans d'autres. Et dans les Veines Grises, quelque chose a changé. Les créatures qui en remontent maintenant sont organisées. Coordonnées. Dirigées.

Les théologiens ervan ont une explication : **Garum s'est éveillé**. Le Grand Silence n'était pas une absence de Gaïa, mais le premier souffle conscient de son opposé.

Après le Grand Silence, l'Empire Uni est trop affaibli pour maintenir la cohésion. Les trois cités reprennent leur autonomie. Les trois factions actuelles naissent sur ces cendres.

---

### 2.6 Ère de la Fracture (An 47-200 AO)

Les trois siècles qui suivent voient l'émergence et la consolidation des trois factions principales. Ce n'est pas une période de guerre constante — plutôt une période de définition identitaire, de frontalisation progressive, d'incidents diplomatiques qui s'accumulent.

Les grandes étapes de cette ère :

- **An 47** : Proclamation de l'Empire Pourpre par Velanthas II, arrière-petit-fils de Velanthor. Adoption du pourpre impérial comme couleur dynastique.
- **An 52** : Fondation de l'Alliance de Rive par le Pacte des Trois Ports — Auranthos, Selmara, et Caraveth signent un traité de libre-échange et de défense mutuelle.
- **An 63** : Le Grand Concile d'Ervanath réunit les cent-vingt clans druides et vote la création de la Fédération Ervan. Premier Conseil des Voix élu.
- **An 89** : Première Guerre des Frontières entre l'Empire Pourpre et la Fédération Ervan. Durée : quatre ans. Résolution par le **Traité des Brumes**, qui définit encore aujourd'hui les frontières contestées du plateau de Velharris.
- **An 134** : Fondation de la Guilde des Aventuriers à Auranthos, à l'initiative d'un marchand-explorateur nommé Karath Soleine, qui théorise que les Stampedes (alors appelés "Percées") nécessitent des spécialistes indépendants des factions.
- **An 156** : Premier Stampede majeur documenté — la Percée de Korreth, où le donjon sous la ville minière de Korreth déborde pendant quarante-deux jours, détruisant la moitié de la ville. Les trois factions envoient des forces conjointement pour la première fois depuis le Grand Silence.
- **An 178** : Fondation officielle de la Guilde des Mercenaires à partir d'un regroupement de compagnies de guerre indépendantes qui refusent de s'affilier à une faction après la Deuxième Guerre des Frontières.

---

### 2.7 Ère Actuelle — L'Ère des Failles (An 200+ AO)

Le monde tel que le joueur le découvre au lancement d'Allumina. L'an actuel de jeu est **l'An 247 AO**.

Les Veines Grises sont de plus en plus actives. Les Stampedes se produisent toutes les deux à quatre semaines (en temps réel, correspondant à des cycles saisonniers de jeu). Les trois factions sont en état de guerre chronique de basse intensité, avec des flambées régulières en saison de guerre. La Guilde des Mercenaires est au summum de son influence — leur neutralité est un équilibre précaire que toutes les factions cherchent à briser discrètement. Les Outlaws prolifèrent dans les zones non gouvernées, certains en agents de Garum à leur insu.

L'enjeu narratif de fond, qui se dévoile sur plusieurs saisons : **Garum prépare quelque chose de plus grand que des Stampedes ordinaires**. Des lieutenants de Garum — les **Voix de l'Érosion** — s'infiltrent dans les structures de pouvoir des trois factions, cherchant à déclencher une guerre totale qui affaiblirait suffisamment Véranthas pour permettre une **Grande Percée** : un Stampede à l'échelle d'un continent entier.

---

## 3. Géographie — Carte textuelle

### 3.1 Vue d'ensemble de Véranthas

Véranthas est composé de **deux grandes masses continentales** séparées par la **Mer du Milieu** (appelée *Mer Centrale* par les impériaux, *Mer des Libertés* par l'Alliance de Rive, *Mer-Mère* par la Fédération Ervan) :

- **Continent Occidental — Velanthas** : plus grand, à dominante montagneuse au centre, plaines côtières à l'ouest et au sud. Territoire principal de l'Empire Pourpre et de la Fédération Ervan.
- **Continent Oriental — Aranthos** : plus petit, archipelago côtier complexe à l'est, plaines fertiles au centre. Territoire principal de l'Alliance de Rive.

Entre les deux continents, plusieurs **îles majeures** forment des zones de commerce et de conflit :
- **Île de Selmar** : ville-État indépendante, principale base de la Guilde des Mercenaires
- **Îles Grises** (anciennes Îles Vorakis) : zones corrompues, habitations Outlaws
- **Île de la Convergence** : île neutre où se trouve le QG principal de la Guilde des Aventuriers

---

### 3.2 Continent Occidental — Velanthas

#### Région Nord : Le Plateau de Velharris

Zone montagneuse et froide, forêts de conifères denses. Territoire **contesté** entre Empire Pourpre et Fédération Ervan. Riches en minerais (fer, cuivre, cristaux de Veine). Population mixte — impériaux dans les forts et forteresses, ervans dans les villages forestiers. Les Veines Grises y sont nombreuses et affleurent parfois en surface sous forme de **Pierres Grises** — des blocs de roche d'un gris métallique qui tuent toute végétation dans un rayon de trente mètres. Sites fréquents de Stampedes.

**Villes notables :**
- **Fort Velharris** (Empire Pourpre) : forteresse militaire, garnison de 4000 soldats, siège du Commandement Nord
- **Sylvareth** (Fédération Ervan) : village-nexus, sanctuaire de la Grande Racine, population 2000

#### Région Centre-Ouest : Le Cœur Impérial

Plaines légèrement vallonnées, climat tempéré, sol fertile. Cœur économique et démographique de l'Empire Pourpre. Bien drainé, nombreuses rivières navigables, réseau routier le plus dense de Véranthas.

**Villes notables :**
- **Velanthara** (Capitale de l'Empire Pourpre) : population ~400 000, la plus grande ville du continent
- **Korreth** (reconstruit post-Stampede) : ville industrielle et minière, cité ouvrière
- **Aurum-Vest** : ville de garnison et école militaire impériale

#### Région Sud-Ouest : Les Marches Brûlées

Zone semi-aride, collines rocheuses, végétation de garrigue. Anciennement fertile, les Marches Brûlées ont souffert d'un Stampede long qui a duré deux ans (la **Percée des Cendres**, An 198). Partiellement récupérées, mais encore marquées de zones grises. Territoire de friction entre l'Alliance de Rive (qui convoite les ports naturels) et l'Empire Pourpre (qui revendique historiquement la région).

**Villes notables :**
- **Selmaris-Sur** : port impérial, base navale, population 80 000
- **Cendrepas** : ville reconstruite sur les ruines post-Stampede, ambiance sombre, marché noir florissant — base principale des Outlaws dans l'Ouest

#### Région Centre-Est (Vaste Forêt d'Ervan) : Le Manteau Vert

La plus grande forêt tempérée de Véranthas, s'étendant sur 800 km du nord au sud. Territoire cœur de la Fédération Ervan. Forêts si denses et si anciennes que certains arbres sont larges de vingt mètres. Les chemins sont organiques, tortueux, connus des seuls guides ervans. Les Nexus y sont les plus nombreux et les plus puissants.

**Villes notables :**
- **Ervanath** (Capitale de la Fédération Ervan) : population ~120 000, construite *dans* les arbres et entre leurs racines, jamais au sol
- **Nexar** : ville-Nexus de la Source Mère, lieu de pèlerinage, population 15 000 permanente + 50 000 pèlerins en saison
- **Verdantel** : ville-frontière est, porte d'entrée du Manteau Vert, commerce avec l'Alliance de Rive

---

### 3.3 Continent Oriental — Aranthos

#### Région Nord : La Côte des Épices

Climat méditerranéen chaud, végétation d'oliviers et de citronniers sauvages, vents réguliers favorables à la navigation. Le berceau de l'Alliance de Rive. Littoral très découpé avec des centaines de baies naturelles formant des ports. Population de pêcheurs, navigateurs, marchands. Les Veines Grises sont rares ici — l'Alliance attribue cela à la "bénédiction des eaux" mais les érudits ervans pensent que la salinité de l'eau côtière ralentit la corruption.

**Villes notables :**
- **Auranthos** (Capitale de l'Alliance de Rive) : population ~280 000, ville-port, le plus grand marché de Véranthas
- **Caraveth** : deuxième port, spécialisé dans le commerce avec les îles
- **Selmara** : ville-forteresse maritime, siège de la flotte de guerre de l'Alliance

#### Région Centre : Les Plaines d'Or

Agricoles, fertiles, parsemées de cités moyennes prospères. Grenier de l'Alliance. Routes commerciales terrestres vers l'intérieur. Ambiance plus calme et prospère que les côtes — mais aussi moins défendue. Cible fréquente des razzias des Outlaws et, occasionnellement, des expéditions impériales.

**Villes notables :**
- **Auranthos-Centre** (nommée familièrement "la Centrale") : nœud logistique, entrepôts gigantesques
- **Goldfield** : ville agricole, marché aux céréales, siège de la guilde des marchands de blé

#### Région Est : Les Archipels de Brume

Côte est complexe, îles innombrables, brumes marines permanentes. Navigation difficile pour les non-initiés. La flotte de l'Alliance y maîtrise des routes que personne d'autre ne connaît. Havre de pirates (recrutés comme corsaires en temps de guerre par l'Alliance). Les Îles Grises se trouvent à l'extrémité est de cette zone — techniquement hors du territoire de l'Alliance mais sous son "influence dissuasive".

---

### 3.4 Zones de guerre et zones neutres

| Zone | Statut | Contrôle actuel |
|------|--------|-----------------|
| Plateau de Velharris | Zone de guerre permanente | Contesté Empire/Fédération |
| Marches Brûlées | Zone de friction | Nominalement Empire, contesté Alliance |
| Île de Selmar | Zone neutre | Guilde des Mercenaires |
| Île de la Convergence | Zone neutre | Guilde des Aventuriers |
| Îles Grises | Zone corrompue / Outlaw | Sans gouvernement |
| Détroit de la Mer Centrale | Zone de tension navale | Alliance (suprématie navale) |
| Zones de Veine Grise | Zones corrompues | Garum (de facto) |
| Cendrepas | Zone grise | Outlaws + population civile mixte |

### 3.5 Routes commerciales majeures

1. **La Route Pourpre** : Velanthara → Fort Velharris → Verdantel (terrestre, contrôlée par l'Empire)
2. **La Route des Épices** : Auranthos → Caraveth → Île de Selmar → Selmaris-Sur (maritime, contrôlée par l'Alliance)
3. **La Voie Verte** : Ervanath → Nexar → Verdantel → côte est (forestière, contrôlée par la Fédération)
4. **La Route Grise** : réseau informel reliant Cendrepas, les Îles Grises et des ports de contrebande (Outlaws)
5. **La Route Centrale** : Île de la Convergence → les trois capitales (neutre, sous protection de la Guilde des Aventuriers)

---

## 4. Cosmologie — Gaïa & Garum

### 4.1 Nature de Gaïa

Gaïa n'est pas un être pensant au sens humain du terme. Elle est un **principe de cohésion cosmique** — la force qui maintient les systèmes vivants en équilibre dynamique. Elle est consciente, mais pas *personnelle* : elle ne choisit pas de favoriser un humain plutôt qu'un autre. Elle répond à des patterns, à des états d'équilibre ou de déséquilibre.

Les druides ervans la comparent à un système nerveux planétaire : chaque plante, animal, rivière et vent est un signal dans ce réseau. Les Nexus sont les **ganglions** — des nœuds de densité informative exceptionnelle où la conscience de Gaïa est la plus accessible.

Gaïa ne parle pas directement aux mortels. Elle *résonne*. Les druides formés passent des décennies à apprendre à distinguer une résonance de Gaïa d'une simple intuition humaine. Les mages de la Fédération utilisent des instruments de mesure (les **Harmoniques**) pour quantifier les signaux des Nexus. L'Empire Pourpre considère tout cela comme de la mystification.

**Ce que Gaïa peut faire :**
- Accélérer ou ralentir la croissance végétale dans une zone
- Modifier les comportements migratoires des animaux
- Envoyer des "alertes" sous forme de phénomènes naturels inhabituels (tempêtes, séismes locaux, floraisons hors-saison)
- En cas de menace extrême : concentrer l'énergie d'un Nexus dans un manifestation physique temporaire (appelée **Veilleuse**) — créature lumineuse, sans langage, qui guide ou protège puis disparaît

**Ce que Gaïa ne peut pas faire :**
- Intervenir directement contre Garum avec une force comparable
- Contraindre un mortel à agir contre sa volonté
- Régénérer une zone complètement corrompue sans aide externe massive

### 4.2 Nature de Garum

Garum est l'opposé de Gaïa mais pas son "ennemi" au sens intentionnel premier. La relation est plus complexe.

**Théologie ervan (théorie dominante) :** Gaïa et Garum sont deux aspects d'un même principe cosmique originel — l'**Équilibre Primordial**. Gaïa est la cohésion, Garum est la dissolution. Sans dissolution, pas de renouvellement. Sans cohésion, pas de structure. Pendant l'Ère de la Lumière Première, les deux forces coexistaient naturellement : Garum dissolvait les choses mortes, Gaïa en tissait de nouvelles. L'équilibre était parfait.

Ce qui a déréglé cet équilibre, c'est l'activité humaine intensive : les Veines Grises, naturellement drainées vers Garum, ont été **exploitées** par les mineurs de l'Empire Uni pour en extraire des cristaux d'énergie (les **Cristaux de Veine**, très utilisés dans le steampunk impérial). Cette extraction massive a **asymétrisé** le flux : trop d'énergie prise du côté Garum, pas assez retournée. Garum a commencé à "compenser" en absorbant de l'énergie vitale directement — c'est le début de la corruption.

**Ce que Garum est devenu :** Un principe de dissolution devenu **vorace**. Il n'a pas de plan narratif humain (pas de vengeur, pas de conquérant). Il ne veut rien au sens où un humain veut. Il *dissout* parce que c'est sa nature, mais cette nature est maintenant déchaînée et non régulée. Cependant, au cours des siècles, l'absorption d'esprits corrompus et de volontés brisées lui a fourni quelque chose de nouveau : une **proto-intelligence collective**, un agrégat de tous les désespoirs et rancœurs absorbés.

C'est de là que viennent les **Voix de l'Érosion** — des entités qui portent une fraction de cette intelligence agrégée et peuvent agir de façon semi-autonome dans le monde physique.

**Forme physique de Garum :** Garum n'a pas de corps. Il est présent dans les Veines Grises comme une pression, une densité. Dans les zones de forte corruption, on peut percevoir une "présence" sous forme de sons très graves inaudibles mais ressentis physiquement, de froid sec, de désorientation. Les Pierres Grises sont ses excroissances physiques les plus directes.

### 4.3 Les Voix de l'Érosion (Lieutenants de Garum)

Quatre entités semi-conscientes, nées de la proto-intelligence de Garum. Chacune est associée à un type de corruption différent.

| Voix | Domaine | Manifestation physique | Localisation actuelle |
|------|---------|------------------------|----------------------|
| **Vreth l'Érodeur** | Corruption minérale | Forme de rocher mouvant, taille d'une maison | Profondeurs sous Fort Velharris |
| **Silana la Murmurante** | Corruption mentale | Aucune forme physique — voix dans les rêves | Infiltrée dans les cercles politiques |
| **Korrakh le Submergé** | Corruption des eaux | Forme de vague géante de liquide gris | Mer de Cendre |
| **Verdeth la Tordante** | Corruption végétale | Arbre mort de 40m qui se déplace lentement | Frontière nord du Manteau Vert |

### 4.4 Mécanique narrative des Stampedes

Un **Stampede** est déclenché quand une concentration de corruption dans une zone souterraine (un donjon) dépasse un seuil critique. La pression devient si forte que les parois entre le souterrain et la surface cèdent — physiquement. Des failles s'ouvrent dans le sol, des créatures corrompues se déversent, la zone autour du point de percée se corrompt rapidement.

**Causes d'un Stampede :**
1. **Progression naturelle** : Garum accumule passivement de l'énergie corrompue dans une veine souterraine jusqu'au point de rupture (processus lent, ~4-8 semaines en temps réel)
2. **Déclenchement actif par une Voix** : une Voix de l'Érosion peut accélérer le processus dans une zone cible — utilisé comme arme tactique contre des villes spécifiques
3. **Exploitation minière excessive** : creuser trop près d'une Veine Grise peut déclencher un Stampede prématuré (risque de gameplay pour les zones minières)
4. **Rituel Nécrolicide** : certains membres de cultes secrets (agents involontaires de Garum) effectuent des rituels qui alimentent les veines

**Déroulement d'un Stampede (en gameplay) :**
- Phase 1 (Alerte, 24h in-game) : tremblements légers, faune fuyant la zone, Pierres Grises qui apparaissent
- Phase 2 (Percée, Jour 1-3) : failles ouvertes, premières vagues de créatures corrompues, zones PvP suspendues dans le secteur
- Phase 3 (Submersion, Jour 3-10) : pic d'intensité, boss de Stampede actifs, villes sous siège possible
- Phase 4 (Reflux, Jour 10-14) : les créatures se retirent ou sont vaincues, purification possible par les druides ervans
- Phase 5 (Cicatrisation, Semaines 2-4) : la zone reste instable, loot spécial, craft unique lié à la corruption

**Suspension du PvP pendant un Stampede :** Les règles de guerre entre factions sont *officiellement* suspendues dans un rayon de 5 km autour du point de percée. En pratique, des incidents se produisent — et sont la source de scandales diplomatiques retentissants chaque saison.

### 4.5 Autres forces cosmiques

**Les Échos de l'Équilibre** : Quelques rares mortels naissent avec une sensibilité exceptionnelle aux deux forces. Ni purs serviteurs de Gaïa, ni corrompus par Garum — ils perçoivent les deux comme un dialogue. La Fédération Ervan les appelle **Équilibristes** et les traite avec une déférence mêlée d'inquiétude. L'Empire Pourpre les classe comme "anomalies à surveiller". En gameplay, ce sont des PNJ de classe narrative rare, sources de quêtes exceptionnelles.

**Sorath, l'Étoile** : Le soleil de Véranthas est vénéré par l'Empire Pourpre comme une divinité solaire — **Sorath l'Illuminateur**. Les druides ervans savent que Sorath n'est pas une divinité consciente mais une source d'énergie pure qui renforce les Nexus. L'ambiguïté théologique est une source de tension diplomatique constante.

**Les Spectres Verts** : Âmes de druides morts particulièrement liées aux Nexus qui refusent de quitter le plan physique. Ni dangereux ni bienveillants — informatifs. Les joueurs de la Fédération Ervan peuvent parfois interagir avec eux pour obtenir des informations historiques ou des quêtes de tradition.

---

## 5. Empire Pourpre

### 5.1 Identité visuelle et symbolique

**Couleurs :** Pourpre profond (#8B0000) et or impérial (#FFD700)
**Symbole :** L'Aigle de Fer — un aigle aux ailes déployées tenant dans ses serres un rouage et une lance, superposé sur un soleil stylisé
**Devise :** *"Velanthas Aeternum"* — Velanthas pour l'éternité
**Titre de joueur :** Citoyen (Civis) → Légionnaire → Centurion → Tribun → Légat → Préfet → Consul → Sénateur → Grand Consul
**Starter class joueur :** Citoyen (Civis) — classe sociale, pas classe de combat

### 5.2 Lore — Histoire complète

L'Empire Pourpre se considère comme le **légataire direct de l'Empire Uni de Velanthor**. Sa mythologie fondatrice est simple et puissante : Velanthor le Pourpre a uni les hommes, les Trois Maux ont brisé cette unité, et l'Empire Pourpre œuvre à la **réunification** — par la diplomatie si possible, par la force si nécessaire.

**Fondation (An 47 AO) :** Velanthas II, arrière-petit-fils de Velanthor, proclame l'Empire Pourpre dans les ruines partiellement reconstruites de Veranthas-Majeur — désormais baptisée **Velanthara**. Il adopte la constitution militaire qui régit encore l'Empire : le **Codex Martial**, un code combinant droit romain, discipline prussienne et pragmatisme économique.

**L'Ère de la Construction (An 47-120 AO) :** Les cent premières années de l'Empire sont consacrées à reconstruire l'infrastructure de l'Empire Uni — routes, aqueducs, fortifications. C'est durant cette période que les ingénieurs impériaux développent les premières **Machines de Guerre** — armes de siège mécanisées utilisant des cristaux de Veine comme source d'énergie. L'ironie que ces cristaux soient liés à Garum n'échappe à personne, mais l'Empire choisit d'ignorer la théologie ervan et de traiter les cristaux comme une ressource naturelle neutre.

**La Purge des Mages (An 89 AO) :** Suite à la Première Guerre des Frontières contre la Fédération Ervan, l'Empereur Claudas III promulgue le **Décret de la Raison Pure** : toute pratique magique non sanctionnée par l'État est interdite. Les mages doivent s'enregistrer, se soumettre à des tests, et leurs pouvoirs doivent être "utiles à l'Empire". En pratique, seule la **magie technique** — celle qui alimente les machines — est tolérée. La magie naturelle ervan et toute magie non-technologique sont officiellement classées comme "sorcellerie primitive".

**La Guerre des Trois Côtiers (An 145-163 AO) :** L'Empire tente de s'emparer des ports naturels des Marches Brûlées, alors contrôlés par l'Alliance de Rive. La guerre dure dix-huit ans et se termine sans vainqueur clair — l'Empire conserve deux ports mais perd 40 000 soldats et épuise ses réserves de cristaux. Ce conflit laisse une rancœur profonde et un appétit de revanche dans la culture militaire impériale.

**L'Ère des Machines (An 170-200 AO) :** Le génie militaire impérial Loran Vaex invente la **Presse à Veine** — un procédé qui extrait les cristaux avec dix fois l'efficacité des méthodes précédentes. L'Empire entre dans une révolution industrielle-militaire : armures renforcées mécaniquement, balistes à répétition, premiers prototypes d'automates de guerre. C'est le pic du "steampunk martial" impérial. C'est aussi, selon la théologie ervan, la période qui a le plus alimenté la montée de Garum — une coïncidence que l'Empire Pourpre refuse catégoriquement d'examiner.

### 5.3 Structure politique

L'Empire Pourpre est une **oligarchie militaire républicaine** — un oxymore assumé. L'Empereur est élu, mais uniquement parmi les membres du **Sénat Martial** (les généraux en chef et les Grands Consuls des provinces). Le pouvoir se transmet rarement par hérédité — plutôt par mérite militaire et politique.

**Structure du pouvoir :**

```
Sénat Martial (12 membres)
  └── Imperator (élu parmi les Sénateurs, mandat de 10 ans)
        ├── Légat Suprême (commandant militaire unifié)
        ├── Grand Trésorier (économie et cristaux)
        ├── Préfet des Frontières (3 préfets régionaux)
        └── Inquisiteur Général (surveillance magique et contre-espionnage)
```

**L'Imperator actuel (An 247 AO) : Marta Velassian**
Première femme Imperator de l'histoire, élue à l'issue d'une campagne militaire exceptionnelle contre un Stampede majeur (la Percée de Korreth II, An 243). Pragmatique, froide, génialement stratégique. Elle est secrètement en contact avec des membres modérés de la Fédération Ervan pour discuter d'une alliance anti-Garum — ce que le Sénat Martial n'approuverait pas.

### 5.4 Géographie et capitale

**Capitale : Velanthara**
Population : ~400 000 habitants permanents, jusqu'à 600 000 en période de mobilisation militaire.
Architecture : pierre blanche et granit gris, colonnes monumentales, aqueducs visibles depuis n'importe quel point de la ville, forum central gigantesque (le **Forum d'Or**), rues en damier parfaitement droites. Au centre : la **Citadelle Pourpre**, résidence de l'Imperator, forteresse-palais dont les murs sont incrustés de cristaux de Veine qui brillent la nuit d'un rouge sombre.

**Territoire impérial :**
- Le Cœur Impérial (plaines centrales, ~60% du territoire)
- Le Plateau de Velharris (nord, contesté)
- Les Marches Brûlées (sud-ouest, partiellement)
- Selmaris-Sur (port principal, pointe sud)

### 5.5 Rapport aux autres factions

| Faction | Relation | Intensité | Notes |
|---------|----------|-----------|-------|
| Alliance de Rive | Rivalité économique et militaire | Haute | Guerre des Trois Côtiers non oubliée |
| Fédération Ervan | Mépris idéologique + guerre froide | Très haute | Désaccord fondamental sur la magie |
| Guilde des Aventuriers | Respect utilitaire | Moyenne | Utilisés pour les Stampedes |
| Guilde des Mercenaires | Dépendance masquée | Haute | Grands acheteurs de contrats |
| Outlaws | Criminalisation totale | Très haute | Chasse aux primes actives |

### 5.6 Rapport à Garum / Gaïa

L'Empire Pourpre ne reconnaît ni Gaïa ni Garum comme des entités religieuses légitimes. Officiellement, les Veines Grises sont un phénomène géologique naturel, les Stampedes sont des "éruptions telluriques d'énergie brute", et les Cendrés sont des "victimes d'une maladie compréhensible et maîtrisable scientifiquement".

Officieusement, l'Inquisiteur Général maintient un département secret — le **Bureau des Phénomènes Anormaux** (BPA) — qui étudie la corruption avec une rigueur scientifique terrifiante. Le BPA sait que quelque chose d'intelligent dirige les Stampedes non-aléatoires. Il ne partage pas cette information avec le Sénat.

L'Empire vénère **Sorath** comme divinité solaire — un culte monothéiste organisé, avec temples, prêtres et fêtes nationales. C'est le seul cadre religieux officiellement reconnu.

### 5.7 Culture impériale

**Nourriture :** Cuisine substantielle et ordonnée — viandes grillées, légumes racines, pain de seigle noir, vins rouges épais. Les repas ont une structure militaire : heure fixe, service en ordre. Le banquet sénatorial est un art politique — les alliances se forgent à table autant qu'au Sénat.

**Festivals :**
- **Dies Velanthoris** (jour de fondation, début du printemps) : grand défilé militaire à Velanthara, promotions publiques, remise des distinctions militaires
- **Nuit de Sorath** (solstice d'été) : feux sur toutes les citadelles, prières au soleil, concours de machines mécaniques
- **Jour des Morts Honorables** (automne) : commémoration des soldats tombés, lecture publique du **Livre des Braves** (registre de tous les soldats morts au combat depuis la fondation)

**Vêtements :** Tunique pourpre liseré d'or pour les Citoyens, armure complète pour les militaires actifs, toge bordeaux pour les sénateurs. Le port de couleurs "ennemies" (vert ervan, jaune de l'Alliance) est mal vu en public.

**Musique :** Percussions et cuivres militaires. Les marches de guerre sont composées par des musiciens d'État — la musique est considérée comme un outil de cohésion et de moral, pas d'expression personnelle. Les musiciens indépendants existent mais travaillent dans les tavernes, jamais dans les lieux officiels.

**Architecture unique :** Les **Tours de Cristal** — des tours d'observation dont les sommets sont surmontés de cristaux de Veine traités pour émettre un signal lumineux de portée variable. Utilisées pour les communications militaires entre garnisons. Réseau s'étendant sur tout le territoire impérial — la première infrastructure de communication longue distance de Véranthas.

### 5.8 Figures emblématiques

**Velanthor le Pourpre** (fondateur légendaire, -500 à -400 AO) : Conquérant et unificateur, figure mythique. Son portrait est reproduit dans chaque bâtiment officiel. La réalité historique est que Velanthor était un administrateur brilliant mais un général médiocre — mais la mémoire collective a inversé cette proportion.

**Claudas III "le Purificateur"** (An 85-132 AO) : Auteur de la Purge des Mages. Héros pour la majorité impériale, figure haïe pour la Fédération Ervan. Son sarcophage au mausolée impérial est régulièrement vandalisé (et régulièrement restauré).

**Loran Vaex "l'Architecte de Fer"** (An 142-201 AO) : Génie ingénieur qui a révolutionné la technologie militaire impériale. Sa statue dans le Forum d'Or est la plus grande de la ville — plus grande que celle de l'Imperator fondateur.

**Marta Velassian "la Ferrière"** (An 215 AO - présent) : Imperator actuelle. Née dans une famille de forgerons militaires, elle a gravi tous les rangs par le mérite de combat. Sa campagne contre la Percée de Korreth II, où elle a commandé une armée mixte de 30 000 soldats pendant quarante jours de siège de Stampede, est étudiée dans toutes les écoles militaires.

**Dorak Vaessen "le Traître de Velharris"** (An 198 AO) : Général impérial qui a vendu les plans de défense du Plateau de Velharris à la Fédération Ervan en échange d'une promesse d'asile. L'asile lui a été accordé mais la Fédération l'a immédiatement remis à l'Empire pour "geste de bonne foi". Il a été exécuté publiquement. Son nom est utilisé comme insulte suprême dans l'armée : "faire un Vaessen" = trahir.

### 5.9 Forces et faiblesses

**Forces militaires :**
- Infanterie lourde la plus entraînée de Véranthas
- Machines de siège inégalées
- Réseau logistique (routes + Tours de Cristal) extrêmement efficace
- Discipline et moral élevés grâce à la structure sociale

**Faiblesses militaires :**
- Marine inférieure à l'Alliance de Rive
- Dépendance aux cristaux de Veine (ressource dont l'approvisionnement est une cible)
- Inflexibilité tactique — excellent en batailles rangées, moins en guérilla
- Zéro capacité magique officielle (un désavantage contre les mages ervans)

**Forces sociales :**
- Cohésion nationale forte
- Bureaucratie efficace
- Infrastructure la plus développée
- Système de méritocratie militaire qui génère une loyauté réelle

**Faiblesses sociales :**
- Xénophobie institutionnalisée vers la magie et les cultures "non-rationnelles"
- Inégalités économiques croissantes entre Citoyens militaires et population civile non-enrôlée
- Le BPA est une bombe à retardement — si ses découvertes sur Garum deviennent publiques, la legitimité de l'État scientifique s'effondre

### 5.10 Motivations de guerre

L'Empire Pourpre veut :
1. **Réunification** : contrôle des trois régions historiques de l'Empire Uni (objectif générationnel)
2. **Accès aux ports des Marches Brûlées** : pour briser la dépendance navale à l'Alliance
3. **Neutralisation de la Fédération Ervan** : pas nécessairement la destruction, mais l'assimilation ou la réduction à une région autonome sous tutelle impériale
4. **Monopole sur les Veines Grises** : toutes les veines accessibles doivent être sous contrôle impérial

---

## 6. Alliance de Rive

### 6.1 Identité visuelle et symbolique

**Couleurs :** Jaune soleil (#F4C430) et ambre maritime (#FF8C00)
**Symbole :** La Balance et la Vague — une balance de commerce dont les plateaux portent une pièce d'or et un coquillage, superposée sur une vague stylisée
**Devise :** *"Libres, Liés par le Sel"* — libre en individu, uni par la mer
**Titre de joueur :** Homme libre (Freelander) → Matelot → Corsaire → Capitaine → Amiral → Grand Marchand → Consul de Rive → Archonte de Mer
**Starter class joueur :** Homme libre — classe sociale, pas de classe de combat imposée

### 6.2 Lore — Histoire complète

L'Alliance de Rive n'a pas été fondée par des conquérants ou des druides. Elle a été fondée par des **marchands pratiques** qui ont réalisé qu'un traité commercial vaut mieux qu'une guerre. Son ADN est profondément pragmatique : la liberté est une valeur cardinale, mais pas au détriment des affaires.

**Les Trois Ports (avant la fondation) :** Auranthos, Selmara et Caraveth étaient des cités-États indépendantes, parfois en compétition féroce pour les mêmes routes commerciales. Les Trois Maux ont changé la donne : la Peste de Cendre a tué des marins, la Guerre des Mages Noirs a coupé les routes commerciales terrestres, et le Grand Silence a réduit les pêches de moitié. Les trois cités ont réalisé qu'elles mouraient ensemble. Elles ont décidé de survivre ensemble.

**Le Pacte des Trois Ports (An 52 AO) :** Signé par les trois Archontes sur un bateau ancré à mi-chemin entre les trois ports — territoire de personne, donc territoire de tous. Le Pacte établit la libre circulation des marchands entre les trois cités, un tarif douanier commun pour les tiers, et un fonds de défense mutuelle. Ce n'est pas un empire — chaque cité garde son gouvernement interne. C'est une **alliance fonctionnelle**.

**L'Expansion Maritime (An 52-130 AO) :** Libérée des guerres inter-cités, l'Alliance se tourne vers la mer. Les navigateurs de Caraveth découvrent des routes vers des îles inconnues. Les marchands d'Auranthos financent des flottes d'exploration. Les corsaires de Selmara sécurisent (et parfois "taxent") les routes. En soixante-dix ans, l'Alliance contrôle la totalité de la Mer Centrale et établit des comptoirs commerciaux sur le continent occidental.

**Le Grand Conflit Commercial (An 95-145 AO) :** L'enrichissement de l'Alliance attire l'hostilité de l'Empire Pourpre, qui voit dans cette prospérité maritime une menace à son monopole sur les routes terrestres. La tension monte sur cinquante ans, débouchant sur la Guerre des Trois Côtiers (An 145-163) — une guerre que l'Alliance gagne tactiquement (sa flotte est supérieure) mais finit par abandonner diplomatiquement (elle préfère un accord commercial à une guerre d'attrition).

**L'Âge d'Or Marchand (An 163-220 AO) :** Après le traité qui met fin à la Guerre des Trois Côtiers, l'Alliance connaît son apogée économique. Auranthos devient la plus grande ville du continent oriental. La Banque de Rive, fondée en An 168, devient la première institution financière supranationale de Véranthas — prêtant de l'argent aux trois factions, maintenant ainsi une influence économique qui dépasse de loin la puissance militaire de l'Alliance.

**La Crise des Corsaires (An 220-235 AO) :** Une faction de corsaires de Selmara refuse un accord de paix commerciale signé avec l'Empire Pourpre, continuant à attaquer les navires impériaux. L'Alliance se retrouve dans l'embarras diplomatique et doit militairement réprimer "ses propres" corsaires — une guerre interne humiliante qui révèle les tensions entre la liberté individuelle et les engagements collectifs de l'Alliance. La crise se résout par la création de la **Charte des Corsaires** : un cadre légal qui permet l'activité corsaire sous licence officielle, régulée et taxée.

### 6.3 Structure politique

L'Alliance est une **oligarchie marchande décentralisée**. Chaque cité-membre gouverne ses affaires internes, mais délègue la politique étrangère et commerciale à l'**Assemblée de Rive**.

```
Assemblée de Rive (représentants des cités-membres + guildes marchandes)
  └── Grand Archonte (élu pour 5 ans par l'Assemblée)
        ├── Archonte d'Auranthos (capitale, voix prépondérante)
        ├── Archonte de Selmara (flotte de guerre)
        ├── Archonte de Caraveth (commerce maritime)
        ├── Guilde des Marchands (lobbying puissant, voix non-officielle mais réelle)
        └── Capitaine des Corsaires sous Charte (liaison militaire navale)
```

**Le Grand Archonte actuel (An 247 AO) : Pelvan Sorel**
Un marchand d'épices devenu politique, connu pour sa capacité à trouver des compromis là où tout le monde voit des impasses. Il maintient secrètement la Banque de Rive comme outil de pression diplomatique — en refusant d'accorder des prêts à une faction, il peut la pousser à la table des négociations sans un seul soldat. Personnellement : il s'ennuie mortellement et rêve de reprendre la mer.

### 6.4 Géographie et capitale

**Capitale : Auranthos**
Population : ~280 000 habitants permanents, jusqu'à 400 000 en haute saison commerciale (afflux de marchands).
Architecture : blanc calcaire et bois de teck, maisons à colonnes orientées vers la mer, rues larges et ombragées, marché central permanent (l'**Agora des Quatre Vents**) ouvert 24h/24, port en eau profonde pouvant accueillir 800 navires simultanément. Les bâtiments publics sont décorés de mosaïques représentant des scènes de commerce et de navigation. L'odeur de sel, d'épices et de poisson frais caractérise la ville.

**Territoire de l'Alliance :**
- La Côte des Épices (nord du continent oriental)
- Les Plaines d'Or (centre du continent oriental)
- Les Archipels de Brume (côte est)
- Comptoirs commerciaux dans les Marches Brûlées (contestés)
- Île de Selmar (partagée avec la Guilde des Mercenaires)

### 6.5 Rapport aux autres factions

| Faction | Relation | Intensité | Notes |
|---------|----------|-----------|-------|
| Empire Pourpre | Rivalité commerciale + rancœur militaire | Haute | Guerre des Trois Côtiers non digérée |
| Fédération Ervan | Relation commerciale respectueuse | Basse | Commerce des ressources forestières |
| Guilde des Aventuriers | Partenariat stratégique | Haute | Co-fondateurs, lien historique |
| Guilde des Mercenaires | Clients réguliers mais méfiants | Moyenne | Trop dépendants des mercenaires en guerre |
| Outlaws | Tolérance pragmatique | Basse | Certains ports de l'Alliance "ne voient pas" le commerce Outlaw |

### 6.6 Rapport à Garum / Gaïa

L'Alliance a une relation **pragmatique et commerciale** avec les forces cosmiques. Gaïa est respectée car la pêche, les vents et les routes maritimes en dépendent — les navigateurs ervan paient des druides pour bénir leurs flottes, et l'Alliance n'y voit aucune objection tant que le service est rendu et payé.

Garum est traité comme un **risque commercial** : les zones corrompues rendent les routes dangereuses, les Stampedes ferment des ports, et les cristaux de Veine utilisés par l'Empire sont une ressource dont l'Alliance essaie de contrôler le commerce. Pas de théologie — de la comptabilité.

La **Banque de Rive** finance secrètement des expéditions de purification des zones corrompues qui bloquent des routes commerciales. Ces expéditions sont cataloguées comme "sécurisation de routes" dans les registres.

### 6.7 Culture de l'Alliance

**Nourriture :** La cuisine est le marqueur culturel le plus fort de l'Alliance. Poissons en mille préparations, huile d'olive, herbes méditerranéennes, pain plat, fruits de mer, vins blancs légers et rosés. Les repas sont sociaux, longs, bruyants. La table d'un marchand de Rive est l'équivalent d'une salle de réunion.

**Festivals :**
- **Fête des Quatre Vents** (équinoxe de printemps) : bénédiction de la flotte, courses de bateaux, grand marché d'ouverture de saison, feux d'artifice sur le port
- **Jour de la Balance** (mi-été) : règlement des dettes annuelles, reconciliation des comptes, tradition de "brûler les mauvaises dettes" (symboliquement — les Livres de Comptes abîmés sont brûlés)
- **Nuit des Corsaires** (automne) : commémoration des marins perdus en mer, bateaux miniatures envoyés sur l'eau avec des bougies, chants nautiques, pardon des crimes mineurs commis en mer

**Vêtements :** Tissus légers, amples, en lin et coton de couleurs vives (jaune, orange, bleu marine). Bijoux marins — coquillages, perles, ambre. Les marchands portent des ceintures à multiples poches et porte-monnaie ostensibles — afficher sa richesse est une normalité culturelle, pas de la vanité.

**Musique :** Instruments à cordes et flûtes — la musique de Rive est mélodique, rythmée, dansante. Les chants de marins sont transmis de génération en génération. La musique est un commerce comme un autre : les musiciens se vendent et se louent librement.

### 6.8 Figures emblématiques

**Karath Soleine** (An 134 AO) : Marchande-exploratrice d'Auranthos, fondatrice de la Guilde des Aventuriers. Son histoire — une femme marchande qui a perdu sa cargaison dans un Stampede et a décidé de fonder une guilde pour ne jamais revivre ça — est le récit fondateur le plus populaire de Rive. Ironiquement, la Guilde des Aventuriers est indépendante de l'Alliance, ce que Karath voulait.

**Aldran le Corsaire** (An 148-189 AO) : Plus grand amiral de l'histoire de l'Alliance, héros de la Guerre des Trois Côtiers. Sa victoire navale de la Baie de Selmara (An 157) — où il a détruit une flotte impériale deux fois supérieure en nombre — est étudiée dans toutes les académies navales du monde. Personnage ambigu : brillant stratège mais violent et peu scrupuleux sur les méthodes.

**Pelvan Sorel** (An 247 AO) : Grand Archonte actuel. Voir section 6.3.

**Liria "la Trahison de Caraveth"** (An 223 AO) : Archonte de Caraveth qui a secrètement signé un accord commercial avec l'Empire Pourpre garantissant à l'Empire l'accès au port de Caraveth contre une garantie de non-attaque. L'accord a été découvert. Liria a été destituée et exilée. Elle vit maintenant à... Velanthara.

### 6.9 Forces et faiblesses

**Forces :**
- Marine de guerre et de commerce incomparable
- Puissance économique et financière (Banque de Rive)
- Réseau de renseignement commercial étendu
- Capacité à acheter des mercenaires et à influencer les guerres par l'économie
- Flexibilité politique — l'Alliance s'adapte, négocie, pivote

**Faiblesses :**
- Armée terrestre faible (sous-investissement chronique)
- Cohésion politique fragile — chaque cité défend ses intérêts propres
- La liberté individuelle peut tourner en anarchie (crise des corsaires)
- Dépendance aux routes maritimes — vulnérable si la mer est contrôlée

### 6.10 Motivations de guerre

L'Alliance veut :
1. **Liberté des mers** : aucune puissance ne doit contrôler les routes maritimes
2. **Accès aux ressources de Velharris** : les minerais et cristaux de Veine sont essentiels aux échanges
3. **Déstabilisation de l'hégémonie impériale** : pas pour conquérir l'Empire, mais pour que personne n'ait assez de puissance pour fermer les routes commerciales
4. **Expansion des comptoirs** dans les Marches Brûlées pour réduire la dépendance aux seuls ports de la Côte des Épices

---

## 7. Fédération Ervan

### 7.1 Identité visuelle et symbolique

**Couleurs :** Vert forêt (#228B22) et argent lunaire (#C0C0C0)
**Symbole :** L'Arbre aux Cinq Branches — un arbre stylisé dont les cinq branches portent les symboles des cinq Nexus fondateurs, entouré d'un cercle de runes
**Devise :** *"Ervan Véranthas"* — Ervan est Véranthas (sous-entendu : la Fédération protège l'essence du monde)
**Titre de joueur :** Habitant (Dweller) → Veilleur → Gardien → Tisseur → Archidruide → Voix du Nexus → Porte-Parole → Première Voix
**Starter class joueur :** Habitant — classe sociale, forte implication communautaire

### 7.2 Lore — Histoire complète

La Fédération Ervan est la faction la plus ancienne dans ses racines, mais la plus récente dans sa forme politique. Les clans druides ervan existent depuis l'Ère de la Lumière Première — ils ne se sont fédérés politiquement qu'à la suite des Trois Maux.

**Les Cent-Vingt Clans (avant An 47 AO) :** Les druides ervan vivaient en clans nomades semi-sédentaires dans ce qui est aujourd'hui le Manteau Vert. Chaque clan gardait un Nexus ou un ensemble de Nexus. Il y avait une culture commune (les Mémoires Vertes, la langue proto-ervan, les rituels saisonniers) mais aucune structure politique unifiée. Les conflits inter-clans étaient réglés par des assemblées ad hoc.

**Le Traumatisme des Trois Maux :** La Peste de Cendre a décimé des clans entiers. La Guerre des Mages Noirs a menacé les Nexus (Ardath Vorak cherchait à capturer des Nexus pour alimenter son armée). Le Grand Silence a failli briser la connexion entre tous les druides et Gaïa simultanément. Ces trois catastrophes ont convaincu les clans qu'ils ne pouvaient pas survivre séparément.

**Le Grand Concile d'Ervanath (An 63 AO) :** Le plus grand rassemblement de l'histoire ervan. Cent-vingt clans représentés, débats pendant quarante jours sous les arbres millénaires d'Ervanath. Résultat : la création du **Conseil des Voix** et la rédaction de la **Charte Verte** — constitution de la Fédération. Points clés de la Charte Verte :
- Chaque clan garde son autonomie interne
- Les Nexus sont la propriété collective de la Fédération (pas des clans individuels)
- Le Conseil des Voix décide à la majorité des deux-tiers pour les questions de guerre et d'alliance
- L'accès des "extérieurs" aux Nexus est régulé par le Conseil
- La magie naturelle est un droit protégé, jamais une marchandise

**La Première Voix fondatrice : Sylvara Erven** (An 63-112 AO) : Première Première Voix élue, figure fondatrice. Une femme d'une soixantaine d'années lors de l'élection, druide de rang exceptionnel, réputée pour pouvoir converser directement avec les Spectres Verts. Son mandat de cinquante ans (renouvelé six fois de suite — les mandats de la Fédération n'ont pas de limite) a posé les bases de toutes les institutions actuelles.

**La Guerre des Frontières et ses conséquences (An 89-93 AO) :** L'attaque de l'Empire Pourpre sur les zones contestées du Plateau de Velharris est vécue comme une agression existentielle. La Fédération riposte avec la première démonstration d'une **magie de masse coordonnée** — cinq Nexus activés simultanément créent une tempête magique qui dévaste l'avance impériale. L'Empire signe le Traité des Brumes mais ne l'oublie pas — et la Purge des Mages qui suit (An 89) est directement liée à cette démonstration de puissance magique.

**L'Ère de la Croissance Intérieure (An 93-200 AO) :** Convaincue que la guerre frontale avec l'Empire est contre-productive, la Fédération se concentre sur le développement interne : cartographie complète du réseau de Nexus, développement de l'Académie de Nexar, formation de nouvelles générations de mages-druides, et — plus controversé — développement discret d'une **magie militaire défensive**.

**La Crise de Verdeth (An 201 AO) :** L'apparition de la Voix de l'Érosion Verdeth à la frontière nord du Manteau Vert provoque une crise politique interne. Une faction de druides radicaux propose de tenter une communion avec Verdeth pour "re-équilibrer" la corruption — théorie héritée de la philosophie de l'Équilibre Primordial. L'autre faction veut une réponse militaire immédiate. Le débat dure trois ans. Finalement, une expédition de purification est lancée : Verdeth est... repoussée, pas détruite. Elle reste à la frontière, lentement. La crise révèle des fractures idéologiques profondes dans la Fédération.

### 7.3 Structure politique

La Fédération est une **démocratie directe des Nexus** — une structure unique où le pouvoir est lié à la connexion aux Nexus plutôt qu'à la propriété ou à la force militaire.

```
Conseil des Voix (120 représentants de clans, 1 par clan)
  └── Première Voix (élue par le Conseil, mandat illimité révocable)
        ├── Tisseuses des Nexus (5 Grandes Tisseuses, une par Nexus fondateur)
        ├── Gardiens des Frontières (représentants militaires des zones contestées)
        ├── Voix des Mémoires (archives, histoire, anti-propagande)
        └── Académie de Nexar (recherche magique, formation)
```

**La Première Voix actuelle (An 247 AO) : Verath Silvane**
Un homme de 89 ans (âge considérable, maintenu par sa connexion aux Nexus). Conservateur dans ses méthodes, progressiste dans ses idées. Il est convaincu que Garum est une urgence existentielle qui dépasse les querelles avec l'Empire, mais il sait que son Conseil n'est pas prêt à voter une alliance avec Velanthara. Il joue sur le long terme. Contact secret avec Marta Velassian — les deux se respectent sans s'aimer.

### 7.4 Géographie et capitale

**Capitale : Ervanath**
Population : ~120 000 permanents. La ville est construite *dans* et *entre* les arbres millénaires — passerelles de bois vivant entre les cimes, maisons creusées dans des troncs de cinquante mètres de circonférence, escaliers en spirale autour des racines géantes. Le sol de la ville est une forêt continue — il n'y a pas de sol nu, pas de pierre pavée. Les structures sont posées sur les racines ou suspendues entre les branches. La lumière est toujours filtrée par la canopée — des lanternes de bioluminescence (champignons traités) éclairent les passages la nuit.

La **Salle des Voix** — le parlement de la Fédération — est construite entre les branches des sept plus anciens arbres d'Ervanath, à 60 mètres du sol. Y accéder nécessite de gravir des rampes et passerelles qui peuvent prendre une heure depuis le bas de la ville.

**Territoire de la Fédération :**
- Le Manteau Vert (cœur)
- La moitié sud du Plateau de Velharris (contesté)
- Des zones de forêt clairsemée en contact avec les Plaines d'Or de l'Alliance

### 7.5 Rapport aux autres factions

| Faction | Relation | Intensité | Notes |
|---------|----------|-----------|-------|
| Empire Pourpre | Hostilité idéologique profonde + guerre froide | Très haute | Purge des Mages jamais pardonnée |
| Alliance de Rive | Commerce respectueux, méfiance légère | Basse-Moyenne | L'Alliance est pragmatique, pas un pair |
| Guilde des Aventuriers | Collaboration active | Haute | Druides coopèrent souvent avec les Aventuriers |
| Guilde des Mercenaires | Utilisation rare, méfiance | Basse | La Fédération préfère ses propres guerriers |
| Outlaws | Compassion prudente | Basse | Certains Outlaws cherchent la rédemption via la Fédération |

### 7.6 Rapport à Garum / Gaïa

La Fédération Ervan est la faction qui a la relation la plus directe et la plus intime avec Gaïa et la compréhension la plus nuancée de Garum.

**Sur Gaïa :** Les druides sont les "traducteurs" de Gaïa — ils passent des années à apprendre à lire ses résonances dans les Nexus. Gaïa n'est pas vénérée comme une déesse personnelle (ce malentendu agace profondément les théologiens ervans), mais comme une force cosmique dont ils sont les intendants. La Fédération se considère comme gardienne de l'équilibre de Gaïa.

**Sur Garum :** La Fédération adopte la **Théorie de l'Équilibre Brisé** : Garum n'est pas mauvais par nature, mais déséquilibré par l'activité humaine (principalement l'extraction minière impériale des cristaux de Veine). Réparer l'équilibre est la mission de la Fédération — ce que l'Empire interprète comme une tentative de monopole sur les ressources naturelles.

**En pratique :** Les druides sont les seuls en Véranthas capables de **purifier** une zone corrompue par Garum — un processus lent, coûteux en énergie, et qui nécessite plusieurs Nexus actifs simultanément. Cette capacité unique est leur plus grand atout géopolitique. Lors des Stampedes, les trois factions ont besoin des druides ervans pour la purification post-percée.

### 7.7 Culture de la Fédération

**Nourriture :** Végétarienne par tradition, mais pas par obligation (les chasseurs de certains clans consomment de la viande dans le cadre de rituels spécifiques). Fruits sauvages, champignons, graines, herbes, miel des ruches géantes. Cuisine complexe en saveurs mais simple en présentation. L'**Assemblée de Table** — manger ensemble sans hiérarchie — est un rituel social fondamental.

**Festivals :**
- **Chant des Mémoires** (solstice d'hiver) : récitation collective des Mémoires Vertes pendant sept nuits, chaque nuit dédiée à une ère historique
- **Réveil des Nexus** (équinoxe de printemps) : rituels aux cinq Nexus fondateurs simultanément, participation ouverte à tous (y compris les non-ervans, sous supervision)
- **Fête de l'Équilibre** (équinoxe d'automne) : commémoration du Grand Silence, moment de réflexion, pas de célébration — une journée de silence collectif dans toute la Fédération

**Vêtements :** Robes amples en fibres naturelles (lin, ortie, chanvre), teintes végétales (verts, ocres, bruns), broderies représentant les motifs des Nexus du clan d'appartenance. Les Grandes Tisseuses portent des robes d'argent — la seule couleur qui ne vient pas des plantes mais des pigments de minerai lunaire.

**Musique :** Voix, flûtes de bois, harpes aux cordes végétales. La musique ervan est polyphonique — conçue pour être chantée à plusieurs voix simultanément, représentant la pluralité des Nexus. Les compositions sont transmises oralement et varient selon les clans.

**Langue :** Le proto-ervan (langue des Mémoires) est une langue morte utilisée uniquement dans les rituels. La langue commune est partagée avec le reste de Véranthas, mais les Ervans ont un accent distinct et utilisent des expressions idiomatiques issues des Mémoires Vertes.

### 7.8 Figures emblématiques

**Sylvara Erven** (An 63-112 AO) : Première Première Voix, fondatrice. Voir section 7.2.

**Doran le Tisseur de Feu** (An 112-156 AO) : Archidruide qui a développé la magie militaire défensive ervan, utilisée pour la première fois lors de la Première Guerre des Frontières. Personnage controversé — pour une faction qui se revendique de la paix, avoir un "génie militaire" dans sa galerie de héros est inconfortable. Sa statue à Ervanath est trois fois plus petite que celle de Sylvara.

**Aelindra "la Passeuse"** (An 156 AO) : Druide qui a guidé les druides de cinq Nexus simultanément lors du Tissage contre la Percée de Korreth. Elle a survécu à l'effort mais a perdu la parole pour les douze années suivantes — la connexion avait été si intense qu'elle ne pouvait plus distinguer sa voix de celle de Gaïa. Icône de la dévotion ervan.

**Verath Silvane** (An 158 AO - présent) : Première Voix actuelle. Voir section 7.3.

**Kael Vorak** (An 202 AO) : Le druide qui a proposé la communion avec Verdeth. Expulsé du Conseil des Voix, il vit maintenant en ermite à la frontière de la zone corrompue, continuant ses recherches seul. Certains disent qu'il parle à Verdeth. D'autres qu'il a été partiellement corrompu. Il est le point de départ de quêtes exceptionnelles.

### 7.9 Forces et faiblesses

**Forces :**
- Seule faction capable de purifier la corruption de Garum
- Magie la plus puissante de Véranthas pour les opérations défensives
- Connaissance unique du réseau de Nexus
- Cohésion culturelle et communautaire
- Terrain (le Manteau Vert) presque impossible à conquérir pour une armée conventionnelle

**Faiblesses :**
- Structure démocratique lente à décider (les deux-tiers requis)
- Magie offensive moins développée que la magie défensive
- Population faible comparée aux deux autres factions
- Dépendance aux Nexus (si un Nexus est capturé ou corrompu, la faction perd des capacités)
- Tendance à l'isolationnisme qui peut mener à des retards stratégiques

### 7.10 Motivations de guerre

La Fédération Ervan veut :
1. **Protection des Nexus** : aucune force étrangère ne doit accéder aux Nexus sans autorisation du Conseil
2. **Arrêt de l'extraction impériale des cristaux de Veine** : conviction que cette extraction alimente Garum
3. **Reconnaissance de la souveraineté du Manteau Vert** : end définitif du contentieux sur le Plateau de Velharris
4. **Rôle officiel dans la gestion des Stampedes** : la Fédération veut être reconnue comme autorité internationale de purification, pas juste un service payable

---

## 8. Guilde des Aventuriers

### 8.1 Identité visuelle et symbolique

**Couleurs :** Bleu ciel (#4169E1) et cuivre (#B87333)
**Symbole :** La Boussole et l'Épée — une boussole dont l'aiguille nord est une épée stylisée
**Devise :** *"Toujours plus loin, toujours plus profond"*
**Structure :** Guilde neutre PvE — aucune affiliation factionnelle
**Rangs :** Recrue → Explorateur → Chasseur → Briseur → Maître-Briseur → Champion → Légende
*La progression est uniquement basée sur les boss tués et les donjons complétés — pas sur l'affiliation politique*

### 8.2 Lore

La Guilde des Aventuriers est née d'un constat pratique fait par **Karath Soleine** (marchande d'Auranthos) après avoir perdu sa cargaison dans un Stampede : les trois factions se battent entre elles pendant que les Stampedes détruisent les infrastructures dont tout le monde dépend. Il fallait des spécialistes — neutres, mobiles, motivés par quelque chose d'autre que la politique.

La Guilde est fondée en **An 134 AO** à Auranthos. Son acte fondateur est pragmatique : aucune allégeance politique, financement par contrats (les factions payent pour avoir des équipes de Briseurs lors des Stampedes), et un système de mérite pur (les rangs basés sur les accomplissements objectifs).

**Le Hall of Fame** est l'invention la plus influente de la Guilde : chaque saison de jeu, les meilleurs Aventuriers (par faction, par classe, et au niveau global) sont inscrits dans un registre permanent disponible dans chaque QG de guilde. Les Légendes d'une saison reçoivent des titres permanents, des équipements uniques, et — le plus précieux — leur nom dans le registre physique qui est conservé à l'éternité dans le QG central de l'Île de la Convergence.

**Expansion et neutralité maintenue :** La Guilde a su rester neutre pendant deux siècles parce qu'elle offre quelque chose que chaque faction veut mais ne peut pas reproduire seule : des équipes de Briseurs entraînés à travailler hors des structures militaires classiques. Tenter de contrôler la Guilde, c'est risquer qu'elle signe des contrats exclusifs avec l'ennemi.

### 8.3 Structure

```
Conseil des Maîtres-Briseurs (élu parmi les Maîtres-Briseurs de chaque région)
  └── Grand Maître (élu pour 3 ans, renouvelable une fois)
        ├── QG Central — Île de la Convergence
        ├── QG Velanthara (Empire Pourpre)
        ├── QG Auranthos (Alliance de Rive)
        ├── QG Ervanath (Fédération Ervan)
        └── QG Selmar (Île neutre, Mercenaires adjacents)
```

**Le Grand Maître actuel (An 247 AO) : Rethkar "l'Immortel" Dune**
Aventurier légendaire qui a survécu à six Stampedes majeurs et tué soixante-dix-huit boss répertoriés. Physiquement impressionnant mais diplomatiquement habile. Il est la garantie vivante que la Guilde n'est pas un club de politiciens — elle est dirigée par quelqu'un qui fait encore des donjons le week-end.

### 8.4 Rapport aux factions et à la cosmologie

La Guilde n'a pas de position officielle sur Gaïa ou Garum, mais en pratique, ses membres connaissent mieux la corruption que quiconque — ils la traversent dans les donjons. Beaucoup de Briseurs développent des croyances personnelles diverses. Certains sont devenus des dévots de Gaïa après des expériences dans des Nexus souterrains. Quelques-uns ont été corrompus et expulsés.

La Guilde maintient sa neutralité par un principe simple : **elle accepte des contrats des trois factions simultanément**. Si une faction essaie d'obtenir l'exclusivité, la Guilde annule tous les contrats de cette faction et en signe d'urgence avec ses deux ennemis. Ce mécanisme s'est produit une fois (An 189, tentative impériale d'exclusivité) et la leçon n'a pas été oubliée.

### 8.5 Système de rang en gameplay

| Rang | Exigence | Avantages |
|------|----------|-----------|
| Recrue | Inscription + 1 donjon complété | Accès au QG, stocks de base |
| Explorateur | 5 boss différents tués | Réductions marchands Guilde |
| Chasseur | 20 boss, dont 3 boss de Stamped | Équipement Guilde niveau 2, quêtes spéciales |
| Briseur | 50 boss, dont 5 boss de Stamped + 1 boss de saison | Titre permanent, équipement rare |
| Maître-Briseur | Top 10% de la saison + 100 boss | Vote au Conseil, équipement épique, logement à la Guilde |
| Champion | Top 1% de la saison | Titre de Champion de saison, Hall of Fame |
| Légende | Première place d'une catégorie de saison | Inscription permanente au registre, équipement légendaire unique |

---

## 9. Guilde des Mercenaires

### 9.1 Identité visuelle et symbolique

**Couleurs :** Gris acier (#708090) et rouge sang (#DC143C)
**Symbole :** Le Poing et la Pièce — un poing fermé tenant une pièce d'or
**Devise :** *"La loyauté a un prix. Le prix est juste."*
**Structure :** Guilde neutre — contrats militaires temporaires avec n'importe quelle faction principale

### 9.2 Lore

La Guilde des Mercenaires naît en **An 178 AO** de la fusion de sept compagnies de guerre indépendantes qui refusent l'affiliation après la Deuxième Guerre des Frontières. Ces compagnies ont combattu des deux côtés et ont constaté que leur valeur stratégique était précisément leur absence d'allégeance permanente.

L'acte fondateur de la Guilde est le **Pacte des Compagnies** — un traité interne régissant les règles d'engagement, les tarifs minimaux, et surtout : **la Clause de Trahison**. Un mercenaire qui brise un contrat en cours sans cause justifiée est déclaré **Outlaw** par la Guilde elle-même — et la Guilde coopère avec toutes les factions pour le traquer.

**Île de Selmar :** La Guilde a choisi l'Île de Selmar comme base principale — île neutre entre les deux continents, accessible à toutes les factions, suffisamment isolée pour ne pas être envahie sans un engagement militaire massif. Selmar est devenue une ville-État de facto, gouvernée par la Guilde, avec ses propres lois (très simples : honore ton contrat, paye tes dettes, ne tue pas en ville).

**Rôle de "facteur de bascule" :** L'histoire militaire de Véranthas depuis An 178 montre que dans chaque guerre majeure, la faction qui obtient le plus grand nombre de mercenaires de la Guilde a invariablement gagné. Pas parce que les mercenaires sont les meilleurs soldats (ils sont excellents, mais pas magiquement supérieurs) — mais parce qu'ils ajoutent de la flexibilité, de la compétence spécialisée (génie, marine, siège, assassinat) et du volume à une armée qui en a besoin au moment critique.

Les trois factions le savent. Elles dépensent des fortunes pour les contrats. Et elles tentent régulièrement, discrètement, de corrompre des officiers de la Guilde pour obtenir un avantage exclusif — avec des résultats toujours catastrophiques pour ceux qui se font attraper.

### 9.3 Structure

```
Conseil des Compagnies (un représentant par compagnie membre)
  └── Commandant Suprême (élu pour 5 ans)
        ├── Bureau des Contrats (négociation, suivi)
        ├── Bureau de la Réputation (blacklist, Clause de Trahison)
        ├── École de Selmar (formation des nouveaux membres)
        └── Compagnies spécialisées (Génie, Marine, Siège, Renseignement...)
```

**Le Commandant Suprême actuel (An 247 AO) : Alrath "le Peseur" Korn**
Ancien officier de l'Empire Pourpre qui a démissionné après que ses supérieurs ont refusé d'évacuer des civils pour protéger une position tactique. Il est devenu mercenaire, puis Commandant Suprême. Il déteste l'Empire Pourpre avec une passion professionnellement maîtrisée — et il est le premier à empêcher que cette haine ne biaise les contrats de la Guilde.

### 9.4 Système de contrats en gameplay

**Un contrat mercenaire fonctionne ainsi :**
- Durée fixe : X jours de jeu (minimum 7 jours, maximum 30 jours)
- Faction cliente : une des trois factions principales
- Obligations : participation aux batailles de la faction cliente, non-agression envers les alliés de cette faction
- Interdictions : attaquer des civils, violer les règles des zones neutres, trahir le contrat en cours
- Récompense : pièces d'or + points de réputation Guilde + potentiellement rang spéciaux

**Trahison du contrat :**
- Attaquer la faction cliente pendant le contrat = statut Outlaw immédiat
- Vente d'informations à une faction ennemie pendant le contrat = statut Outlaw + blacklist permanente de la Guilde

**Avantages d'être mercenaire :**
- Accès à des équipements de Selmar non disponibles ailleurs
- Bonus d'XP pendant les contrats militaires
- Possibilité de servir n'importe quelle faction selon les saisons (flexibilité maximale)
- Réseau de contacts dans les trois factions

**Contraintes :**
- Impossible d'être propriétaire dans une capitale factionnelle
- Pas d'accès aux titres de rang factionnels
- Ciblé en priorité si la faction cliente perd la guerre (pas de protection de faction propre)

### 9.5 Rapport à Garum / Gaïa

La Guilde est officiellement athée — ou plus précisément, elle ne prend pas de position institutionnelle. En pratique, les mercenaires sont superstitieux comme tous les soldats : quelques rituels avant la bataille, des amulettes personnelles, et une peur viscérale des zones corrompues (expérience professionnelle).

La Guilde a une règle informelle : **ne pas accepter de contrats qui impliquent d'entrer en zone corrompue** sauf tarif triple et équipement de purification fourni par le client. Cette règle a sauvé beaucoup de vies et constitue un marché de niche très lucratif pour les mercenaires spécialisés en zones corrompues.

---

## 10. Outlaws — La Confrérie de l'Ombre Libre

### 10.1 Identité visuelle et symbolique

**Couleurs :** Noir (#1C1C1C) et rouge carmin (#8B0015)
**Symbole :** Le Crâne Couronné — un crâne humain portant une couronne brisée
**Devise :** *"Libres parce que bannis"*
**Structure :** Non-faction — regroupement d'individus hors-la-loi
**Rangs internes :** Banni → Renégat → Desperado → Criminel de Guerre → Seigneur Outlaw

### 10.2 Lore

Les Outlaws ne sont pas une faction fondée — ils sont une **accumulation**. Chaque individu qui trahit un contrat mercenaire, commet un crime irréparable dans une faction, ou choisit délibérément la vie hors-la-loi rejoint ce groupe diffus qui n'a pas d'histoire fondatrice mais une culture commune.

La **Confrérie de l'Ombre Libre** est une organisation informelle qui s'est progressivement structurée parmi les Outlaws de longue date. Elle n'est pas hiérarchique au sens classique — elle est plutôt un réseau de **Seigneurs Outlaws** qui contrôlent chacun un territoire (une île des Îles Grises, une zone de Cendrepas, un réseau de tunnels sous une ville) et coopèrent quand leurs intérêts convergent.

**La Campagne Criminelle (niveaux 1-30 en gameplay) :** L'histoire personnelle d'un joueur Outlaw commence dans Cendrepas — ville semi-légale, mi-ruines, mi-camp de réfugiés. Les premiers niveaux consistent à survivre, monter dans la hiérarchie criminelle locale, et comprendre le monde des Outlaws. A niveau 30, le joueur a accès à toute la carte, mais sa réputation de criminel est connue partout.

**La Rédemption publique :** C'est le système le plus unique des Outlaws. À tout moment après niveau 10, un Outlaw peut demander la rédemption vers **n'importe laquelle des trois factions principales**. La procédure est :
1. Se présenter seul (sans escorte) à la capitale de la faction choisie
2. Payer une amende en or proportionnelle au niveau de criminalité et aux actes commis (calculée par le système de réputation)
3. Effectuer une **Quête de Rédemption** publique — une mission connue de tous les joueurs de la faction, difficilement réalisable, qui prouve la valeur du repenti
4. Cérémonie publique dans la capitale : le nom de l'Outlaw est affiché pendant 24h réelles, et les joueurs de la faction peuvent voter (symboliquement) pour ou contre l'acceptation

La rédemption donne accès à toutes les fonctionnalités de la faction choisie — mais un titre "Rédempteur" reste visible dans le profil, et certains PNJ méfiants en tiennent compte. La trahison après rédemption est irrémédiable : le statut Outlaw devient permanent, aucune deuxième rédemption possible.

### 10.3 Mécanique full-loot

**Zones PvP Outlaw :**
- Les Îles Grises : full-loot permanent
- Cendrepas : full-loot la nuit (18h-6h en temps réel)
- Zones de friction (Marches Brûlées, Plateau de Velharris) : full-loot si les deux joueurs sont en mode PvP

**Attaque d'un non-PvP :**
- Possible dans les zones sans gouvernement
- Donne automatiquement des points de criminalité
- Un joueur non-PvP peut toujours riposter sans pénalité

**Mort en zone full-loot :**
- L'inventaire non-protégé est accessible au tueur pendant 5 minutes
- Un slot "équipement de cœur" est toujours protégé (l'arme principale ou l'armure principale)
- L'XP perdue est récupérable en tuant des créatures dans les 24h (système de "dette d'âme")

### 10.4 Zones et territoire Outlaw

**Cendrepas :** Ville principale des Outlaws. Ancienne ville minière détruite par un Stampede, reconstruite par des réfugiés et des criminels en fuite. Architecture chaotique — bâtiments en ruine réhabilités avec des matériaux de récupération, rues impraticables pour des armées conventionnelles. Population : ~30 000 permanents, très variable. Pas de gouvernement officiel — les Seigneurs Outlaws locaux se disputent les blocs de la ville.

**Îles Grises :** Archipel de sept îles à l'est du continent. Anciens Vorakis — le site de la défaite d'Ardath Vorak. Partiellement corrompues par la Mer de Cendre adjacente. Population d'environ 15 000 Outlaws permanents. Les structures sont en bois traité (la corruption de Garum dissout la pierre). Lieu de nombreuses légendes sur les trésors d'Ardath Vorak.

### 10.5 Culture des Outlaws

**"Libres parce que bannis"** : La philosophie outlaw n'est pas romanesque — c'est une rationalisation pratique. Personne ne choisit d'être Outlaw comme premier choix. Mais une fois hors-la-loi, certains trouvent dans cette liberté contrainte quelque chose qu'ils n't auraient jamais trouvé dans les structures des factions : une absence totale de hiérarchie imposée.

**La règle d'or informelle des Outlaws** : ne trahis pas ceux qui partagent ta camp. En dehors de ce camp, tout est permis. Cette règle est respectée... à peu près. Les trahisons inter-Outlaws existent, mais elles sont sévèrement pénalisées par la communauté (bannissement de Cendrepas, chasse collective).

**Nourriture et survie :** Cuisine de fortune — gibier, conserves, boulangerie improvisée. Les Seigneurs Outlaws qui contrôlent des territoires stables ont parfois de meilleures tables que des nobles de faction, financées par leurs activités criminelles.

**Musique :** Chants de taverne, ballades criminelles, histoires chantées des grands coups. La musique outlaw est la plus vivante et la plus spontanée de Véranthas.

### 10.6 Figures emblématiques

**Ardath Vorak** (An 12-31 AO) : Pas considéré comme un Outlaw par les Outlaws modernes (il était bien plus qu'un criminel) — mais les Îles Grises portent son héritage et sa légende est récupérée par la Confrérie. Certains Seigneurs Outlaws se disent héritiers de sa vision.

**Mire "la Couronne Brisée"** (An 220 AO - présent) : Fondatrice de fait de la Confrérie de l'Ombre Libre. Ancienne marchande de l'Alliance, ruinée par une manipulation politique, devenue Outlaw par nécessité et Seigneur Outlaw par force. Elle n'est pas une idéologiste — elle est une organisatrice pragmatique. Elle a transformé une collection chaotique de criminels en un réseau fonctionnel.

**Torrath "le Repenti de Velanthara"** (An 235 AO) : L'Outlaw qui a effectué la première rédemption de l'histoire du jeu vers l'Empire Pourpre. Processus qui a duré six mois de jeu, coûté une fortune, et impliqué une quête de rédemption où il a seul stoppé un Stampede émergent à la frontière impériale. Sa cérémonie de rédemption à Velanthara a attiré 12 000 joueurs présents. Il est maintenant Centurion de l'armée impériale.

---

## 11. Relations inter-factions

### 11.1 Tableau des relations actuelles (An 247 AO)

| | Empire Pourpre | Alliance de Rive | Fédération Ervan | G. Aventuriers | G. Mercenaires | Outlaws |
|---|---|---|---|---|---|---|
| **Empire Pourpre** | — | Guerre froide/chaude | Hostilité idéologique | Respect utilitaire | Client régulier | Chasse active |
| **Alliance de Rive** | Rivalité commerciale | — | Commerce respectueux | Partenaire historique | Client régulier | Tolérance pragmatique |
| **Fédération Ervan** | Hostilité profonde | Commerce respectueux | — | Collaboration active | Utilisation rare | Compassion prudente |
| **G. Aventuriers** | Contrats actifs | Contrats actifs | Collaboration | — | Coordination sur Stampedes | Acceptation si rédemption |
| **G. Mercenaires** | Contrats actifs | Contrats actifs | Contrats rares | Coordination | — | Clause de Trahison appliquée |
| **Outlaws** | Ennemi déclaré | Ennemi déclaré (officiel) | Tension | Cas par cas | Ennemis déclarés | Solidarité de survie |

### 11.2 Histoire des conflits majeurs

| Conflit | Années | Parties | Issue |
|---------|--------|---------|-------|
| Première Guerre des Frontières | An 89-93 AO | Empire vs Fédération | Traité des Brumes (statu quo) |
| Guerre des Mages Noirs | An 12-31 AO | Empire+Alliance+Fédération vs Vorak | Victoire coalition |
| Guerre des Trois Côtiers | An 145-163 AO | Empire vs Alliance | Traité ambigu (Empire gagne 2 ports, Alliance garde la suprématie navale) |
| Deuxième Guerre des Frontières | An 165-172 AO | Empire vs Fédération | Statu quo, Fédération gagne des points diplomatiques |
| Crise des Corsaires | An 220-235 AO | Alliance interne | Alliance maintient sa cohésion par la Charte des Corsaires |
| Guerre Actuelle (en cours) | An 240-présent | Empire vs Alliance (Fédération observatrice armée) | Non résolue, Stampedes de Garum changent les calculs |

### 11.3 Neutralité des guildes — Comment ça fonctionne

**Guilde des Aventuriers — Mécanisme de neutralité :**
La Guilde des Aventuriers maintient sa neutralité par un principe économique simple : elle a besoin des trois factions pour fonctionner. Les Stampedes sont répartis sur tout le territoire — une Guilde qui s'aliène une faction perd l'accès aux donjons de ce territoire. De plus, le Hall of Fame est inter-factionnel : des joueurs de toutes factions compétissent dans les mêmes classements, ce qui crée une communauté de mérite qui transcende les allégeances.

En pratique, la Guilde a des **Accords de Zone** avec chaque faction : dans chaque capitale, le QG de la Guilde est zone neutre — aucun acte d'hostilité inter-faction n'y est autorisé. Les Aventuriers qui violent cette neutralité sont expulsés de la Guilde.

**Guilde des Mercenaires — Mécanisme de neutralité :**
La Guilde des Mercenaires maintient sa neutralité par la **Règle des Contrats Ouverts** : tant qu'une faction paie, elle peut recruter. Mais si une faction tente d'obtenir l'exclusivité ou de corrompre des officiers de la Guilde, la Guilde annule tous les contrats de cette faction et offre des tarifs préférentiels à ses ennemis pendant la saison suivante.

L'Île de Selmar est défendue par la Guilde elle-même — et les trois factions savent que tenter de s'emparer de Selmar déclencherait une coalition des deux autres factions pour défendre la neutralité (chacune ayant autant à perdre de la perte de Selmar). C'est un équilibre de terreur mutuelle.

---

## 12. Schémas TOML

### 12.1 Factions principales

```toml
[faction.empire_pourpre]
id = "empire_pourpre"
display_name = "Empire Pourpre"
display_name_short = "Empire"
color_primary = "#8B0000"
color_secondary = "#FFD700"
starter_title = "citoyen"
starter_title_display = "Civis"
capital = "velanthara"
continent = "velanthas_west"
magic_access = "limited"
magic_style = "techno_crystalline"
combat_style = "heavy_infantry_siege"
naval_strength = "weak"
economy_model = "military_industrial"
religion = "sorath_solar"
garum_stance = "denial_scientific"
gaia_stance = "ignore"
hostile_to = ["outlaws"]
at_war_with = ["alliance_de_rive", "federation_ervan"]
neutral_with = ["guilde_aventuriers", "guilde_mercenaires"]
rank_titles = [
  "civis",
  "legionnaire",
  "centurion",
  "tribun",
  "legat",
  "prefet",
  "consul",
  "senateur",
  "grand_consul",
]

[faction.alliance_de_rive]
id = "alliance_de_rive"
display_name = "Alliance de Rive"
display_name_short = "Alliance"
color_primary = "#F4C430"
color_secondary = "#FF8C00"
starter_title = "homme_libre"
starter_title_display = "Freelander"
capital = "auranthos"
continent = "aranthos_east"
magic_access = "moderate"
magic_style = "utility_mercantile"
combat_style = "naval_marine_mercenary"
naval_strength = "dominant"
economy_model = "free_market_mercantile"
religion = "pragmatic_polytheist"
garum_stance = "commercial_risk"
gaia_stance = "transactional"
hostile_to = ["outlaws"]
at_war_with = ["empire_pourpre"]
neutral_with = ["federation_ervan", "guilde_aventuriers", "guilde_mercenaires"]
rank_titles = [
  "homme_libre",
  "matelot",
  "corsaire",
  "capitaine",
  "amiral",
  "grand_marchand",
  "consul_de_rive",
  "archonte_de_mer",
]

[faction.federation_ervan]
id = "federation_ervan"
display_name = "Fédération Ervan"
display_name_short = "Fédération"
color_primary = "#228B22"
color_secondary = "#C0C0C0"
starter_title = "habitant"
starter_title_display = "Dweller"
capital = "ervanath"
continent = "velanthas_center_east"
magic_access = "full"
magic_style = "nature_nexus_druidic"
combat_style = "magic_defensive_guerrilla"
naval_strength = "minimal"
economy_model = "communal_sustainable"
religion = "gaia_stewardship"
garum_stance = "equilibrium_theory_active"
gaia_stance = "devoted_stewards"
hostile_to = ["empire_pourpre", "outlaws"]
at_war_with = ["empire_pourpre"]
neutral_with = ["alliance_de_rive", "guilde_aventuriers", "guilde_mercenaires"]
rank_titles = [
  "habitant",
  "veilleur",
  "gardien",
  "tisseur",
  "archidruide",
  "voix_du_nexus",
  "porte_parole",
  "premiere_voix",
]

[faction.guilde_aventuriers]
id = "guilde_aventuriers"
display_name = "Guilde des Aventuriers"
display_name_short = "G. Aventuriers"
color_primary = "#4169E1"
color_secondary = "#B87333"
starter_title = "recrue"
starter_title_display = "Recrue"
capital = "ile_convergence"
continent = "neutral_islands"
magic_access = "unrestricted"
magic_style = "mixed"
combat_style = "dungeon_specialist"
naval_strength = "moderate"
economy_model = "contract_meritocratic"
religion = "none_official"
garum_stance = "active_opposition_practical"
gaia_stance = "opportunistic_respect"
hostile_to = []
at_war_with = []
neutral_with = ["empire_pourpre", "alliance_de_rive", "federation_ervan", "guilde_mercenaires"]
pve_only = true
hall_of_fame = true
seasonal_reset = true
rank_titles = [
  "recrue",
  "explorateur",
  "chasseur",
  "briseur",
  "maitre_briseur",
  "champion",
  "legende",
]

[faction.guilde_mercenaires]
id = "guilde_mercenaires"
display_name = "Guilde des Mercenaires"
display_name_short = "G. Mercenaires"
color_primary = "#708090"
color_secondary = "#DC143C"
starter_title = "recrue_mercenaire"
starter_title_display = "Recrue"
capital = "selmar"
continent = "neutral_islands"
magic_access = "unrestricted"
magic_style = "mixed"
combat_style = "specialist_adaptive"
naval_strength = "moderate"
economy_model = "contract_military"
religion = "none_official"
garum_stance = "commercial_avoidance"
gaia_stance = "indifferent"
hostile_to = ["outlaws"]
at_war_with = []
neutral_with = ["empire_pourpre", "alliance_de_rive", "federation_ervan", "guilde_aventuriers"]
contract_system = true
betrayal_clause = true
rank_titles = [
  "recrue_mercenaire",
  "soldat",
  "sergent",
  "lieutenant",
  "capitaine_mercenaire",
  "commandant",
  "grand_commandant",
]

[faction.outlaws]
id = "outlaws"
display_name = "Outlaws"
display_name_short = "Outlaws"
color_primary = "#1C1C1C"
color_secondary = "#8B0015"
starter_title = "banni"
starter_title_display = "Banni"
capital = "cendrepas"
continent = "velanthas_southwest"
magic_access = "unrestricted"
magic_style = "mixed_forbidden"
combat_style = "asymmetric_criminal"
naval_strength = "weak"
economy_model = "criminal_opportunistic"
religion = "none_official"
garum_stance = "ignorant_or_complicit"
gaia_stance = "indifferent"
hostile_to = ["empire_pourpre", "alliance_de_rive", "federation_ervan", "guilde_mercenaires"]
at_war_with = []
pvp_full_loot = true
redemption_available = true
redemption_targets = ["empire_pourpre", "alliance_de_rive", "federation_ervan"]
campaign_levels = [1, 30]
rank_titles = [
  "banni",
  "renegat",
  "desperado",
  "criminel_de_guerre",
  "seigneur_outlaw",
]
```

### 12.2 Capitales

```toml
[city.velanthara]
id = "velanthara"
display_name = "Velanthara"
faction = "empire_pourpre"
continent = "velanthas_west"
population = 400000
architecture_style = "imperial_roman_steampunk"
landmark = "citadelle_pourpre"
landmark_description = "Forteresse-palais dont les murs sont incrustés de cristaux de Veine rougeoyants"
port = false
nexus_proximity = "none"
crystal_vein_proximity = "high"
safe_zone = true
pvp_zone = false

[city.auranthos]
id = "auranthos"
display_name = "Auranthos"
faction = "alliance_de_rive"
continent = "aranthos_east"
population = 280000
architecture_style = "mediterranean_mercantile"
landmark = "agora_quatre_vents"
landmark_description = "Le plus grand marché permanent de Véranthas, ouvert 24h/24"
port = true
port_capacity = 800
nexus_proximity = "none"
crystal_vein_proximity = "low"
safe_zone = true
pvp_zone = false

[city.ervanath]
id = "ervanath"
display_name = "Ervanath"
faction = "federation_ervan"
continent = "velanthas_center_east"
population = 120000
architecture_style = "treehouse_organic"
landmark = "salle_des_voix"
landmark_description = "Parlement de la Fédération suspendu à 60m dans les branches des sept arbres millénaires"
port = false
nexus_proximity = "extreme"
crystal_vein_proximity = "none"
safe_zone = true
pvp_zone = false

[city.cendrepas]
id = "cendrepas"
display_name = "Cendrepas"
faction = "outlaws"
continent = "velanthas_southwest"
population = 30000
architecture_style = "ruin_salvage"
landmark = "tour_des_cendres"
landmark_description = "Tour de guet improvisée sur les ruines du bâtiment municipal, toujours debout malgré les apparences"
port = false
nexus_proximity = "corrupted"
crystal_vein_proximity = "high"
safe_zone = false
pvp_zone = true
pvp_full_loot_night_only = true
pvp_night_start = "18:00"
pvp_night_end = "06:00"

[city.selmar]
id = "selmar"
display_name = "Selmar"
faction = "guilde_mercenaires"
continent = "neutral_islands"
population = 45000
architecture_style = "port_military_neutral"
landmark = "palais_des_contrats"
landmark_description = "Le bâtiment où tous les contrats militaires de la Guilde sont signés — les murs sont tapissés de parchemins de contrats historiques"
port = true
port_capacity = 200
nexus_proximity = "none"
crystal_vein_proximity = "none"
safe_zone = true
pvp_zone = false

[city.ile_convergence]
id = "ile_convergence"
display_name = "Île de la Convergence"
faction = "guilde_aventuriers"
continent = "neutral_islands"
population = 25000
architecture_style = "mixed_adventurer"
landmark = "hall_of_fame"
landmark_description = "Salle monumentale où les registres de tous les Légendes de chaque saison sont conservés depuis la fondation"
port = true
port_capacity = 150
nexus_proximity = "moderate"
crystal_vein_proximity = "low"
safe_zone = true
pvp_zone = false
```

### 12.3 Cosmologie et entités

```toml
[cosmic_entity.gaia]
id = "gaia"
display_name = "Gaïa"
nature = "cohesion_principle"
conscious = true
personal = false
communicates_directly = false
communicates_via = ["nexus_resonance", "veilleuse_manifestation", "natural_phenomena"]
hostile = false
alignment = "equilibrium"
worship_style = "stewardship"
worshipped_by = ["federation_ervan"]
partially_respected_by = ["alliance_de_rive"]
denied_by = ["empire_pourpre"]

[cosmic_entity.garum]
id = "garum"
display_name = "Garum"
nature = "dissolution_principle"
conscious = "proto_aggregate"
personal = false
communicates_directly = false
communicates_via = ["corruption_spread", "stampede_direction", "voices_erosion"]
hostile = true
alignment = "unbalanced_dissolution"
origin = "cosmic_balance_broken_by_human_extraction"
lieutenants = ["vreth_erodeur", "silana_murmurante", "korrakh_submerge", "verdeth_tordante"]
diplo_possible = false
ai_full_npc = true

[stampede]
trigger_natural_weeks = [4, 8]
trigger_voice_override = true
trigger_mining_risk = true
alert_phase_hours = 24
breach_phase_days = [1, 3]
surge_phase_days = [3, 10]
reflux_phase_days = [10, 14]
scar_phase_weeks = [2, 4]
pvp_suspend_radius_km = 5
pvp_suspend_during_breach = true
loot_special_during_scar = true
```

### 12.4 Système de rédemption Outlaw

```toml
[outlaw.redemption]
minimum_level = 10
available_targets = ["empire_pourpre", "alliance_de_rive", "federation_ervan"]
requires_solo_arrival = true
requires_gold_payment = true
gold_formula = "base_fine + (crime_level * crime_multiplier) + (pvp_kills * kill_tax)"
requires_redemption_quest = true
quest_difficulty = "elite"
ceremony_public = true
ceremony_duration_real_hours = 24
ceremony_display = "capital_plaza_announcement"
second_redemption = false
betrayal_after_redemption = "permanent_outlaw"
title_after_redemption = "redempteur"
npc_memory_duration_days = 30
```

### 12.5 Contrats mercenaires

```toml
[mercenary.contract]
minimum_duration_days = 7
maximum_duration_days = 30
eligible_clients = ["empire_pourpre", "alliance_de_rive", "federation_ervan"]
exclusive_contract = false
simultaneous_contracts = false
betrayal_consequence = "outlaw_status_immediate"
information_sale_consequence = "outlaw_status_permanent_blacklist"
bonus_xp_during_contract = true
bonus_xp_multiplier = 1.25
property_restriction = true
property_allowed_in = ["selmar", "ile_convergence"]
rank_access_restriction = true
```

---

## 13. Événements saisonniers et arcs narratifs

### 13.1 Structure d'une saison de jeu

Une saison Allumina dure **douze semaines réelles**. Chaque saison a :
- Un arc narratif principal impliquant une ou plusieurs factions
- Un ou plusieurs Stampedes majeurs scénarisés (en plus des Stampedes aléatoires)
- Un Hall of Fame saisonnier pour la Guilde des Aventuriers
- Une Quête de Rédemption publique disponible (si au moins un Outlaw remplit les conditions)
- Des conséquences permanentes sur la carte (zones qui restent corrompues, villes qui restent détruites ou reconstruites)

Les résultats des guerres inter-factions durant une saison impactent la saison suivante : une faction qui gagne des territoires commence la saison suivante avec un avantage logistique dans ces zones. Les territoires perdus restent perdus — jusqu'à ce qu'une nouvelle guerre les reconquière.

**Réinitialisation partielle entre saisons :**
- Réinitialisé : niveaux personnels des joueurs (remise à zéro partielle selon le système de progression horizontal), contrats mercenaires actifs, réputation de combat
- Conservé : titres gagnés, inscriptions Hall of Fame, changements géographiques, réputations sociales, propriétés et logements

### 13.2 Saison Zéro — "L'Éveil des Failles" (saison de lancement)

La saison de lancement est conçue pour introduire progressivement tous les systèmes :

**Semaines 1-3 : Mise en place**
Les joueurs découvrent leur faction, progressent dans les premiers rangs, explorent la carte. Pas de guerre inter-faction déclarée — period de "paix armée" scénarisée. Les Stampedes sont de taille modérée.

**Semaines 4-6 : Première Friction**
Un incident diplomatique — un convoi impérial attaqué dans les Marches Brûlées, origine incertaine (Outlaws ? Alliance ? Provocation interne ?) — enflamme les tensions. Les mercenaires commencent à recevoir des offres. La Guilde des Aventuriers enregistre un pic de recrutement. Les premières zones PvP s'ouvrent.

**Semaines 7-9 : Le Grand Stampede de l'Éveil**
Le premier Stampede majeur scénarisé frappe simultanément deux zones — une en territoire impérial (frontière nord), une en territoire de l'Alliance (port de Caraveth). L'ampleur suggère une coordination — une Voix de l'Érosion en est responsable. Pour la première fois, des forces des trois factions doivent coopérer ou regarder des zones stratégiques tomber.

**Semaines 10-12 : Résolution et Conséquences**
La saison se termine avec une bataille rangée dans le Plateau de Velharris — zone de guerre permanente. Le résultat (victoire Empire, victoire Fédération, ou statu quo selon les actions collectives des joueurs) détermine les conditions de départ de la Saison 1. Premier Hall of Fame révélé. Premières rédemptions Outlaw possibles.

### 13.3 Arcs narratifs planifiés (Saisons 1-5)

**Saison 1 : "Les Murmures de Silana"**
La Voix de l'Érosion Silana la Murmurante s'infiltre dans les cercles politiques des trois factions. Des joueurs reçoivent des rêves qui semblent des révélations stratégiques — certaines vraies, certaines empoisonnées. Des PNJ importants prennent des décisions étranges. La Fédération Ervan est la première à identifier la source. Le combat contre Silana est une quête de raid pour la Guilde des Aventuriers — mais trouver son point de manifestation nécessite des informations que seule la Fédération possède.

**Saison 2 : "La Banque Brisée"**
Un scandale financier ébranle l'Alliance de Rive : la Banque de Rive a secrètement financé des opérations militaires impériales en échange de garanties commerciales. L'Archonte de Caraveth est impliqué (variation de l'histoire de Liria — ou sa fille ?). L'Alliance est politiquement fracturée. Les mercenaires voient leurs tarifs monter de 50% alors que toutes les factions cherchent à profiter de la faiblesse temporaire de l'Alliance.

**Saison 3 : "Le Silence de Velharris"**
Les Tours de Cristal du réseau de communication impérial sur le Plateau de Velharris s'éteignent une à une. Personne ne comprend pourquoi — ni sabotage Fédéral (ils le nient), ni défaillance technique (les ingénieurs sont perdus). La réponse : Vreth l'Érodeur creuse depuis les dessous et dissout les fondations des tours. La Fédération doit choisir : aider l'Empire à sauver ses tours (et donc renforcer une infrastructure militaire impériale) ou laisser Garum progresser dans une zone stratégique.

**Saison 4 : "Les Enfants de Vorak"**
Un culte secret — les **Fils d'Ardath** — émerge dans les Îles Grises. Ils prétendent pouvoir "communiquer" avec Garum et contrôler les Stampedes. Ce n'est pas tout à fait faux — ils peuvent retarder un Stampede de quelques jours en échange de rituels qui alimentent les Veines Grises. Ils vendent ce service aux trois factions sans discrimination. La Guilde des Aventuriers découvre la vérité en premier. Que font-ils avec l'information ?

**Saison 5 : "La Grande Fracture"**
Kael Vorak — le druide ervan qui communique avec Verdeth depuis des années — revient à Ervanath avec une révélation : Verdeth lui a transmis une image de ce que prépare Garum. La Grande Percée est réelle et imminente — dans deux saisons. Mais Verdeth a mis une condition : pour révéler la localisation exacte de la Percée planifiée, elle exige que la Fédération Ervan lui permette d'entrer dans le Manteau Vert. Le Conseil des Voix doit voter. Le monde entier attend.

---

## 14. Bestaire de Garum — Créatures corrompues

Les créatures de Garum suivent une logique de corruption progressive. Les animaux et humains ordinaires exposés longtemps aux Veines Grises se transforment en versions corrompues d'eux-mêmes — reconnaissables mais profondément altérées.

### 14.1 Catégories de corruption

**Niveau 1 — Éraillés (corruption légère)**
Animaux ordinaires (loups, cerfs, rats) dont la fourrure ou la peau est marquée de veines grises. Comportement agressif. Dangereux en groupe mais vaincus facilement. Apparaissent dans les premières zones corrompues. Loot : Cristaux de Veine bruts (grade bas), fourrure corrompue (matériau d'artisanat).

**Niveau 2 — Fondus (corruption moyenne)**
La forme originale est encore visible mais déformée — membres supplémentaires, corps allongé ou compressé, yeux multiples. Une certaine intelligence rudimentaire. Capables de stratégies de base (encerclement, utilisation du terrain). Apparaissent dans les zones corrompues stables. Loot : Cristaux de Veine (grade moyen), essence de corruption (ingrédient alchimique).

**Niveau 3 — Vrais Corrompus (corruption profonde)**
La forme originale n'est plus reconnaissable. Créatures spécifiques à Garum, sans équivalent naturel. Intelligence variable — certains sont des berserkers aveugles, d'autres sont des stratèges. Apparaissent dans les zones de forte corruption et pendant les Stampedes. Loot : Cristaux de Veine purs, artefacts corrompus, parfois des fragments de Veine.

**Niveau 4 — Champions de Garum**
Créatures uniques, nommées, avec des capacités spéciales. Gardiens de zones corrompues ou chefs d'armées de Stampede. Boss de raid ou de donjon. Loot : équipements épiques, artefacts rares, matériaux de purification de haute qualité.

### 14.2 Créatures emblématiques

**Le Marcheur de Cendre (Éraillé, zones de Marches Brûlées)**
Ancien humain, nomade des Marches, transformé par la corruption de Veine. Silhouette humaine grise, mouvements saccadés, incapable de parler mais capable d'imiter des gestes humains de façon troublante. Se déplace seul ou en groupes de 3 à 7. Faible mais psychologiquement perturbant — les joueurs qui l'affrontent pour la première fois réagissent souvent mal à la ressemblance humaine.

**La Vague Grise (Fondu, zones côtières corrompues)**
Ancien banc de poissons fusionné en une masse fluide gris métal qui se déplace comme un liquide sur terre et dans l'eau. Attaque en englobant les ennemis. Immunisé aux dommages physiques normaux — nécessite magie ou armes traitées avec de l'essence de purification ervan. Apparaît dans la Mer de Cendre et les côtes adjacentes.

**L'Arbre Creux (Fondu, Manteau Vert)**
Ancien arbre vivant progressivement corrompu par Verdeth. Ressemble à un arbre normal jusqu'à ce qu'une créature s'en approche à moins de 10 mètres — puis il "s'ouvre" et attaque avec des branches-tentacules. Stationnaire mais avec un rayon d'attaque de 15 mètres. Particulièrement présent à la frontière nord du Manteau Vert.

**Le Chevalier de Cendre (Vrai Corrompu)**
Ancien soldat — de n'importe quelle faction — entièrement corrompu. Garde son armure originale mais déformée, ses armes transformées en extensions de la corruption. Combat avec une brutalité organisée qui rappelle une formation militaire. Détient parfois des fragments de mémoire de son ancienne vie — les joueurs peuvent parfois les déclencher avec certains mots ou objets, créant des quêtes contextuelles.

**Korragh-Vague (Champion de Garum, Mer de Cendre)**
Manifestation partielle de Korragh le Submergé dans une forme physique combat. Masse de liquide gris de la taille d'un bâtiment, capable de se fragmenter en dizaines de petites vagues indépendantes et de se reformer. Boss de raid de la Mer de Cendre. Faiblesse : la chaleur et les sorts de feu élèvent sa viscosité jusqu'à l'immobilisation temporaire. Résistance : toute attaque physique directe est absorbée.

**Verdeth-Branche (Champion de Garum, frontière Manteau Vert)**
L'arbre corrompu de 40 mètres qui se déplace lentement. N'attaque pas les joueurs individuels — il attaque les Nexus. Se dirige passivement vers le Nexus le plus proche. Si personne ne l'en empêche, il peut corrompre un Nexus en 72 heures (temps réel), le rendant inutilisable pendant toute une saison. Boss de raid ouvert — pas de salle de donjon, combat en terrain ouvert dans la forêt.

---

## 15. Économie et ressources — Vue d'ensemble lore

### 15.1 Monnaies de Véranthas

Véranthas utilise trois systèmes monétaires principaux, partiellement convertibles :

**Solidus Impérial (Empire Pourpre)**
Pièce d'or moulée avec le profil de l'Imperator régnant. Standard de référence pour la valeur depuis l'Empire Uni. La valeur est garantie par l'Empire — ce qui signifie qu'en temps de crise impériale, le Solidus peut se dévaluer. Très accepté dans toutes les zones du continent occidental.

**Jeton de Rive (Alliance)**
Pièce circulaire en électrum (alliage or-argent) avec la Balance et la Vague. Valeur fixée par le taux de change de la Banque de Rive — très stable car l'Alliance gère activement la valeur. Principal moyen d'échange dans les transactions commerciales inter-factions. La Banque de Rive offre des services de change (avec commission).

**Feuille d'Argent (Fédération Ervan)**
Non une pièce mais une feuille d'argent pur pressée et gravée d'une rune de clan. Utilisée principalement en interne — les clans n'ont pas de système monétaire traditionnel, et la Feuille est une concession au commerce avec l'extérieur. Valeur perçue comme très haute par les collectionneurs — chaque Feuille est unique (rune différente par clan et par émission).

**Monnaie de Guerre (Guilde des Mercenaires)**
La Guilde accepte toutes les monnaies mais fixe ses tarifs en **Poids d'Or** — une unité abstraite correspondant à un poids standardisé d'or pur. Les clients paient dans leur monnaie locale au taux de change du jour fixé par le Bureau des Contrats de Selmar.

### 15.2 Ressources stratégiques

**Cristaux de Veine**
Ressource la plus stratégique de Véranthas. Extraits des Veines Grises, ils concentrent de l'énergie brute utilisable pour alimenter les machines de guerre impériales, certains sorts puissants, et diverses applications industrielles. La controverse sur leur origine (lien avec Garum selon la théologie ervan) est la cause profonde de nombreux conflits. Commerce international interdit officiellement — mais florissant en pratique.

**Bois du Manteau Vert**
Les arbres du Manteau Vert ont des propriétés uniques : flexibilité exceptionnelle, résistance à l'eau, et légèreté. Idéaux pour la construction navale. L'Alliance de Rive est le premier acheteur. La Fédération contrôle strictement l'abattage — seule une petite quantité est vendue chaque saison, ce qui maintient un prix très élevé.

**Épices de la Côte**
Les épices de la Côte des Épices sont le produit commercial le plus rentable de l'Alliance. Saveurs inimitables, propriétés médicinales légères, durée de conservation longue. Commerce trans-continental. L'Empire Pourpre consomme énormément d'épices de Rive et déteste cette dépendance.

**Fourrures des Hautes Terres**
Le Plateau de Velharris produit des fourrures d'une qualité exceptionnelle — animaux adaptés aux grands froids du nord. Ressource pour laquelle Empire et Fédération se disputent l'accès depuis des générations. Les Outlaws des Marches ont développé un commerce parallèle de fourrures volées.

**Essence de Purification**
Produite uniquement par les druides ervans à partir d'énergie de Nexus concentrée. Permet de neutraliser la corruption de Garum dans des zones ou des objets. Ressource rare, très coûteuse, sans équivalent. La Fédération la vend au compte-gouttes et uniquement à des conditions politiques qui lui sont favorables. Lors des Stampedes, la demande explose et la Fédération peut faire levier sur cette ressource.

### 15.3 Commerce inter-faction

Le commerce inter-faction ne s'arrête jamais complètement même en temps de guerre — c'est une caractéristique qui distingue Allumina de nombreux MMO. Les marchands bénéficient d'un **Statut de Commerce Protégé** dans toutes les zones neutres (Selmar, Île de la Convergence, certains ports de l'Alliance) qui leur confère une immunité aux attaques de faction.

En zones de guerre, le commerce continue via des réseaux de contrebande (opportunité économique pour les joueurs aventuriers et outlaws). Des **routes commerciales grises** parallèles aux routes officielles permettent le passage de marchandises moyennant des taxes informelles (en pratique : payer les gardes de frontière pour fermer les yeux).

---

## 16. Religion, rites et spiritualité comparés

### 16.1 Le culte de Sorath (Empire Pourpre)

Le culte solaire impérial est une religion d'État organisée, hiérarchique et fonctionnelle. **Sorath l'Illuminateur** est le soleil divinisé — source de lumière, de raison, et de légitimité impériale. L'Imperator est *Pont de Sorath* — représentant divin sur Véranthas.

Structure du clergé sorathin :
- Flammes Primaires (prêtres de base, un par bâtiment officiel)
- Torches Vivantes (prêtres itinérants, armée et zones de frontière)
- Gardiens de la Forge Sacrée (prêtres-ingénieurs, maintien des Tours de Cristal)
- Grand Solaire (chef du clergé, membre non-votant du Sénat Martial)

Les rites sorathin : prières à l'aube et au coucher du soleil, jeûne lors des éclipses (vue comme attaque de Garum contre Sorath), offrandes d'huile dans des braseros permanents. Les funérailles impériales sont des crémations à ciel ouvert — "retourner la lumière à Sorath".

**Incompatibilité théologique :** La théologie sorathin affirme que Gaïa est une force mineure — une sorte d'esprit de la terre, non une entité cosmique. Que les Nexus sont des anomalies géologiques, pas des manifestations divines. Cette position enrage les druides ervans qui y voient une désacralisation délibérée.

### 16.2 L'Intendance de Gaïa (Fédération Ervan)

Pas une religion au sens traditionnel — pas de clergé à proprement parler, pas de temples, pas de dogme écrit. Les druides sont des praticiens, pas des prêtres. Leur relation avec Gaïa est fonctionnelle : ils écoutent, ils traduisent, ils agissent en conséquence.

Les **Grandes Tisseuses** (une par Nexus fondateur) sont les figures d'autorité spirituelle — mais leur autorité vient de leur compétence pratique à lire les résonances de Gaïa, pas d'une nomination hiérarchique.

Rites ervans :
- Le Tissage Quotidien : chaque matin, les druides d'un clan se rassemblent en cercle et "écoutent" ensemble le Nexus le plus proche — quelques minutes de silence partagé
- Les Mémoires Chantées : une à deux fois par semaine, récitation collective d'une section des Mémoires Vertes
- Le Retour à la Terre : les morts ne sont pas inhumés ni brûlés — ils sont déposés dans des clairières spécifiques et l'énergie de leur décomposition est "guidée" vers le Nexus local par un rituel de trois jours

**Incompatibilité théologique :** Les druides refusent catégoriquement la conception sorathin de Gaïa comme force mineure. Pour eux, Sorath *lui-même* n'est qu'une source d'énergie extérieure — importante, mais pas divine au sens où Gaïa est cosmiquement fondamentale.

### 16.3 Le Pragmatisme des Dieux (Alliance de Rive)

L'Alliance n'a pas de religion d'État. Elle a un **panthéon de confort** — un ensemble de petites divinités fonctionnelles que les navigateurs, marchands et marins vénèrent selon leurs besoins immédiats. Ces divinités n'ont pas de clergé organisé, pas de temples permanents — juste des autels de quartier, des statuettes portées sur soi, et des rites personnels.

Quelques déités populaires de Rive :
- **Auros** : dieu des vents favorables, prières avant chaque départ en mer
- **Pelvaneth** : déesse des contrats honnêtes, invoquée lors de la signature de tout accord commercial important
- **Caritha** : déesse des naufrages, priée pour être épargnée — et pour trouver les épaves utiles
- **Vorak** (tabou) : le nom d'Ardath Vorak est utilisé comme figure repoussoir — "Que Vorak ne vous trouve pas" = souhait que tout aille mal pour quelqu'un

La Banque de Rive interdit toute activité religieuse dans ses locaux — espace de "neutralité rationnelle". C'est la seule règle religieuse de l'Alliance.

### 16.4 Le Credo des Outlaws

Les Outlaws n'ont pas de religion mais ont un **code moral non-écrit** transmis oralement :

*"Prends ce que tu peux défendre. Paye ce que tu as promis. Tue proprement ou laisse vivre. Ne trahis pas le camp qui t'a nourri. Meurs sans genoux à terre."*

Ce code est respecté de façon variable mais son existence est universellement reconnue. Un Outlaw qui viole une de ces règles peut être chassé de sa communauté — pire que la mort pour quelqu'un qui n'a que sa communauté.

La seule "vénération" outlaw est celle des **Grands Bannis** — des figures historiques d'Outlaws exceptionnels dont les histoires sont racontées comme des épopées. Ardath Vorak y occupe une place ambiguë — le plus grand Banni de l'histoire, mais aussi celui qui a le plus souffert pour ses ambitions.

---

## 17. Lexique — Termes canoniques de Véranthas

Ce lexique établit les termes officiels à utiliser dans tous les documents du jeu. La cohérence terminologique est obligatoire pour maintenir l'immersion.

### 17.1 Termes cosmologiques

| Terme | Définition |
|-------|-----------|
| **Gaïa** | Principe de cohésion cosmique, force vitale du monde |
| **Garum** | Principe de dissolution cosmique, force corrompue/déséquilibrée |
| **Veine Grise** | Fracture tellurique traversée par l'énergie de Garum |
| **Pierre Grise** | Excroissance physique de Veine Grise à la surface |
| **Nexus** | Lieu de concentration de l'énergie de Gaïa |
| **Stampede** | Débordement d'un donjon sur la surface (percée de Garum) |
| **Veilleuse** | Manifestation physique temporaire de Gaïa |
| **Voix de l'Érosion** | Lieutenant semi-conscient de Garum |
| **Équilibre Primordial** | État originel avant le déséquilibre Gaïa/Garum |
| **Mémoires Vertes** | Archives orales ervanes de l'Ère de la Lumière Première |

### 17.2 Termes politiques

| Terme | Définition |
|-------|-----------|
| **Codex Velanthor** | Code de lois de l'ancien Empire Uni, encore en vigueur partiel |
| **Codex Martial** | Constitution militaire de l'Empire Pourpre |
| **Charte Verte** | Constitution de la Fédération Ervan |
| **Pacte des Trois Ports** | Traité fondateur de l'Alliance de Rive |
| **Pacte des Compagnies** | Traité fondateur de la Guilde des Mercenaires |
| **Clause de Trahison** | Règle interne de la Guilde des Mercenaires sur les contrats brisés |
| **Traité des Brumes** | Traité post-Première Guerre des Frontières (Empire/Fédération) |
| **Charte des Corsaires** | Cadre légal de l'activité corsaire sous licence de l'Alliance |
| **Assemblée de Rive** | Parlement fédéral de l'Alliance de Rive |
| **Conseil des Voix** | Parlement de la Fédération Ervan |
| **Sénat Martial** | Organe législatif suprême de l'Empire Pourpre |
| **Bureau des Phénomènes Anormaux** | Département secret impérial étudiant la corruption |

### 17.3 Termes de jeu (game terms)

| Terme | Définition |
|-------|-----------|
| **Civis** | Titre de départ pour les joueurs Empire Pourpre |
| **Freelander** | Titre de départ pour les joueurs Alliance de Rive |
| **Dweller** | Titre de départ pour les joueurs Fédération Ervan |
| **Banni** | Titre de départ pour les joueurs Outlaw |
| **Hall of Fame** | Registre saisonnier de la Guilde des Aventuriers |
| **Légende** | Rang maximum de la Guilde des Aventuriers — inscription permanente |
| **Rédempteur** | Titre permanent d'un Outlaw ayant accompli la rédemption publique |
| **Poids d'Or** | Unité de tarification de la Guilde des Mercenaires |
| **Quête de Rédemption** | Mission publique obligatoire dans le parcours de rédemption Outlaw |
| **Contrat de Sang** | Contrat mercenaire de durée maximale (30 jours) avec bonus et risques accrus |
| **Zone de Cicatrice** | Zone post-Stampede, instable, avec loot spécial et craft unique |
| **Grande Percée** | Hypothétique Stampede continental — menace narrative de fond |
| **Tissage des Cinq Voix** | Rituel ervan simultané sur cinq Nexus — puissance maximale |
| **Harmonique** | Instrument de mesure des résonances de Gaïa utilisé par les mages ervans |

### 17.4 Noms propres géographiques — Index

| Lieu | Type | Faction | Continent |
|------|------|---------|-----------|
| Véranthas | Monde | — | — |
| Velanthas | Continent occidental | Mixte | Ouest |
| Aranthos | Continent oriental | Alliance | Est |
| Velanthara | Capitale | Empire Pourpre | Velanthas |
| Auranthos | Capitale | Alliance de Rive | Aranthos |
| Ervanath | Capitale | Fédération Ervan | Velanthas |
| Cendrepas | Capitale Outlaws | Outlaws | Velanthas |
| Selmar | QG Mercenaires | G. Mercenaires | Îles |
| Île de la Convergence | QG Aventuriers | G. Aventuriers | Îles |
| Îles Grises | Zone Outlaw | Outlaws | Îles |
| Mer de Cendre | Zone corrompue | Garum (de facto) | Mer |
| Plateau de Velharris | Zone de guerre | Contesté | Velanthas |
| Manteau Vert | Forêt Fédération | Fédération | Velanthas |
| Marches Brûlées | Zone de friction | Contesté | Velanthas |
| Côte des Épices | Territoire Alliance | Alliance | Aranthos |
| Plaines d'Or | Territoire Alliance | Alliance | Aranthos |
| Archipels de Brume | Territoire Alliance | Alliance | Aranthos |
| Fort Velharris | Forteresse Empire | Empire | Velanthas |
| Sylvareth | Village-Nexus | Fédération | Velanthas |
| Korreth | Ville industrielle | Empire | Velanthas |
| Aurum-Vest | École militaire | Empire | Velanthas |
| Nexar | Ville-Nexus | Fédération | Velanthas |
| Verdantel | Ville-frontière | Fédération | Velanthas |
| Caraveth | Port Alliance | Alliance | Aranthos |
| Selmara | Port de guerre | Alliance | Aranthos |

---

## 18. Notes de game design — Décisions et justifications

### 18.1 Pourquoi six factions et pas trois ou dix

Trois factions "principales" garantissent le RvR triangulaire de DAoC — aucune coalition permanente 2 contre 1, toujours une dynamique à trois qui force la diplomatie. Les deux guildes neutres (Aventuriers et Mercenaires) ajoutent des couches sans fragmenter le RvR. Les Outlaws ajoutent le sandbox criminel sans casser l'équilibre des trois factions. Six est le nombre maximal gérable sans diluer l'identité.

### 18.2 Pourquoi Garum ne peut pas être diplomatiquement neutralisé

Garum comme menace partagée non-négociable est le seul mécanisme qui peut forcer une coopération temporaire entre factions ennemies. Si Garum pouvait être "apprivoisé" par une faction, cela créerait un déséquilibre compétitif que l'autre faction chercherait à neutraliser — et l'ennemi commun deviendrait un outil stratégique factionnelle. En restant IA full-NPC sans diplomatie possible, Garum reste une pression externe constante et équitable.

### 18.3 Le paradoxe de la rédemption Outlaw

La rédemption est coûteuse, publique et irrémédiable en cas de trahison post-rédemption — par design. Une rédemption trop facile rendrait la faction Outlaw sans conséquence (tout le monde jouerait Outlaw puis se redirait facilement). Une rédemption impossible empêcherait les arcs narratifs personnels de longue durée et découragerait les joueurs qui veulent changer de trajectoire. La solution actuelle crée un risque réel, une récompense narrative forte, et une mémoire collective (les autres joueurs savent qui s'est racheté).

### 18.4 Les Mercenaires comme thermomètre de jeu

En observant le flux des contrats mercenaires sur une saison, les game masters peuvent diagnostiquer l'état de santé du jeu : si 90% des contrats vont à une faction, cette faction est trop dominante et des ajustements sont nécessaires (événements narratifs, spawn de ressources stratégiques). Les mercenaires ne sont pas qu'une mécanique de jeu — ils sont un outil de monitoring et d'équilibrage en temps réel.

### 18.5 Pourquoi les Stampedes suspendent le PvP

Suspendre le PvP dans un rayon de 5 km autour d'un Stampede crée deux dynamiques positives. D'abord, une coopération forcée qui génère des histoires mémorables ("on a dû combattre côte à côte avec nos ennemis"). Ensuite, une ambiguïté narrative — dans les 5 km on ne sait pas si son voisin de combat est ami ou ennemi *dès que la suspension se lève*. Cette transition de PvE à PvP peut produire des moments dramatiques forts.

### 18.6 La carte qui vieillit

Les changements géographiques permanents (zones corrompues qui progressent, villes détruites, fortifications reconstruites) répondent à une frustration commune des joueurs MMO : "rien de ce qu'on fait ne compte". Dans Allumina, les actions collectives modifient la carte de façon durable et visible. Cela crée de la fierté (on a sauvé cette ville), des regrets (on aurait dû défendre ce port), et une identité historique partagée par les joueurs d'une même communauté.

---

## 19. PNJ notables par région — Fiches de référence

Les PNJ suivants sont des personnages clés dans leurs régions respectives. Ils ont des dialogues, des quêtes et des réactions dynamiques basées sur la réputation du joueur et l'état de la guerre.

### 19.1 Velanthara (Empire Pourpre)

**Seris Kovan — Maître des Archives Impériales**
Rôle : Gardien des chroniques de l'Empire, source d'informations historiques, donne accès aux quêtes d'histoire impériale.
Personnalité : Vieille femme de 70 ans, mémoire photographique, légère agoraphobie (sort rarement des Archives). Passionnée d'histoire, frustrée que les victoires navales soient sous-documentées comparées aux victoires terrestres.
Relation Garum : Elle sait, par ses archives, que les cristaux de Veine ont un lien avec la corruption. Elle le tait. Ce secret est le cœur d'une quête majeure.
Réaction aux joueurs : Neutre par défaut. Devient chaleureuse si le joueur lui apporte des documents historiques (item de loot rare dans les donjons). Hostile si le joueur appartient à la Fédération Ervan.

**Drak Velhen — Commandant de la Garde Impériale de Velanthara**
Rôle : PNJ de mission militaire, donne des contrats de guerre PvE et des quêtes de défense de ville.
Personnalité : Homme de 45 ans, ancien légionnaire, cicatrice de Stampede sur la joue gauche. Pragmatique, peu théorique. Déteste la politique mais la subit avec discipline.
Relation Garum : Il a survécu à la Percée de Korreth II. Il sait que c'est une intelligence qui dirige les Stampedes — il ne comprend pas comment, mais son instinct militaire lui a sauvé la vie plusieurs fois.
Réaction aux joueurs : Respect immédiat pour les joueurs qui ont des rangs militaires élevés. Méfiance envers les aventuriers "non affiliés". Il peut être convaincu de coopérer avec n'importe qui si la menace de Garum est présentée correctement.

**Miratha "la Petite Forge" — Artisane de la Citadelle**
Rôle : Marchand d'armes et armures, forge les équipements militaires les plus avancés disponibles dans la capitale.
Personnalité : Jeune femme de 28 ans, fille de forgeron, fierté professionnelle absolue, légère tendance à refuser les commandes qu'elle juge "indignes de son talent". Vénère Loran Vaex comme modèle.
Relation Garum : Elle utilise des cristaux de Veine dans ses alliages et ignore complètement la théologie — pour elle c'est juste une ressource.
Réaction aux joueurs : Neutre financièrement. Devient amie si le joueur lui apporte des matériaux rares. Peut fabriquer des équipements uniques pour les joueurs Rang Légat+.

### 19.2 Auranthos (Alliance de Rive)

**Pelvan Sorel — Grand Archonte (voir section 6.3)**
Disponible pour des quêtes de diplomatie de haut rang (Rang Grand Marchand+). Dialogue dynamique qui reflète l'état des relations inter-factions.

**Kaena de la Mer — Capitaine Corsaire sous Charte**
Rôle : Donne des quêtes navales, commerce de matériaux maritimes rares, source d'informations sur les Archipels de Brume.
Personnalité : Femme de 38 ans, peau brûlée par le sel, accent des îles extrême. Rit de tout. A coulé trois fois et s'est reconstruite chaque fois. Connaît les routes secrètes des Archipels mieux que quiconque.
Relation Garum : Elle a navigué près de la Mer de Cendre. Elle sait qu'il y a quelque chose sous les eaux grises — elle l'a vu. Elle n'en parle jamais.
Réaction aux joueurs : Chaleureuse avec tout le monde sauf l'Empire (rancune de la Guerre des Trois Côtiers). Adore les Outlaws qui ont une bonne histoire derrière eux.

**Orlan Tessev — Directeur de la Banque de Rive, Bureau d'Auranthos**
Rôle : Services financiers (change, prêts, investissements), source de quêtes économiques, informateur discret sur les transactions inter-factions suspectes.
Personnalité : Homme de 55 ans, toujours en costume impeccable malgré la chaleur, sourire professionnel permanent. Plus intelligent qu'il ne le montre. Joue aux échecs contre lui-même.
Relation Garum : La Banque surveille les flux d'or vers les zones corrompues — argent qui disparaît dans des zones grises, transactions avec des entités non identifiées. Orlan a des dossiers. Des dossiers très intéressants.
Réaction aux joueurs : Traite tout le monde de façon identique (c'est une posture professionnelle). Peut être corrompu — mais à un prix très élevé et avec des conséquences dramatiques.

### 19.3 Ervanath (Fédération Ervan)

**Verath Silvane — Première Voix (voir section 7.3)**
Disponible pour des quêtes de rang Voix du Nexus+. Dialogue qui reflète l'état des Nexus et la progression de la corruption.

**Aelindra la Silencieuse — Retraitée de la Grande Tisseuse**
Rôle : La druide qui a dirigé le Tissage de Korreth (section 7.8). Maintenant vieille femme de 90 ans, elle parle peu mais chaque mot compte. Source de quêtes de tradition ervan et de connaissance des Nexus profonds.
Personnalité : Elle ne parle que lorsqu'elle a quelque chose de précis à dire. Ses silences durent parfois des minutes entières. Les joueurs inexpérimentés la croient inattentive — les joueurs expérimentés savent qu'elle évalue.
Relation Garum : Elle a senti Garum lors du Tissage. Elle décrit l'expérience comme "toucher quelque chose de très vieux et très fatigué". Elle pense que Garum souffre autant qu'il fait souffrir.
Réaction aux joueurs : Accorde du temps à quiconque se présente avec une question sincère, quelle que soit la faction.

**Thorn Gaelven — Gardien des Frontières du Plateau de Velharris**
Rôle : Commande militaire des forces ervanes dans la zone contestée. Donne des quêtes de combat défensif, de patrouille de zone et de neutralisation de Pierres Grises.
Personnalité : Homme de 32 ans, le plus jeune Gardien jamais nommé, légèrement intimidé par sa propre responsabilité mais le masque avec un stoïcisme appris. Très compétent en guérilla forestière, beaucoup moins en diplomatie.
Relation Garum : Les Pierres Grises progressent dans sa zone. Il a perdu deux éclaireurs dans un Stampede il y a trois mois. Il a une haine très pratique et très personnelle de Garum.
Réaction aux joueurs : Chaleureux avec les joueurs Fédération. Prudent mais coopératif avec la Guilde des Aventuriers. Méfiance envers l'Empire. Refus de parler aux Outlaws.

### 19.4 Cendrepas (Outlaws)

**Mire "la Couronne Brisée" — Fondatrice de la Confrérie (voir section 10.6)**
Disponible pour des quêtes de rang Desperado+. Son dialogue reflète les tensions internes de la Confrérie et les opportunités criminelles de la saison.

**Petyr l'Apothicaire — Médecin de Cendrepas**
Rôle : Soins, vente de potions, quêtes de récolte de plantes médicinales dans les zones dangereuses.
Personnalité : Homme de 50 ans, ancien médecin de l'Alliance, exilé pour avoir soigné des Outlaws pendant une épidémie sans autorisation officielle. Rancœur calme contre l'Alliance. Soigne tout le monde sans distinction.
Relation Garum : Il traite les effets de la corruption sur les humains — une corruption légère peut être ralentie par ses remèdes, mais pas stoppée. Il a des notes médicales sur la progression de la corruption qui intéresseraient beaucoup la Fédération Ervan.
Réaction aux joueurs : Traite tout le monde. Pas de jugement. Le payer à sa juste valeur augmente sa réputation envers le joueur.

### 19.5 Selmar (Guilde des Mercenaires)

**Alrath "le Peseur" Korn — Commandant Suprême (voir section 9.3)**
Disponible pour des contrats de haut rang et des quêtes narratives sur l'histoire de la Guilde.

**Sera Vane — Chef du Bureau des Contrats**
Rôle : Gère les contrats mercenaires en pratique. Chaque joueur mercenaire passe par elle pour les signatures.
Personnalité : Femme de 40 ans, née à Selmar, a grandi dans les couloirs du Palais des Contrats. Connaît les règles dans tous leurs détails et leurs nuances. Légèrement cynique mais professionnellement intègre.
Relation Garum : Elle a vu des contrats avec des clients dont les ressources provenaient clairement de la corruption. Elle les a refusés en invoquant le "risque de réputation" — elle n'a pas besoin de l'expliquer autrement.
Réaction aux joueurs : Strictement professionnelle avec tous. Légèrement plus chaleureuse avec les mercenaires qui ont un long historique de contrats honorés.

---

## 20. Schémas TOML supplémentaires

### 20.1 Créatures corrompues de Garum

```toml
[creature.marcheur_cendre]
id = "marcheur_cendre"
display_name = "Marcheur de Cendre"
corruption_level = 1
base_zone = ["marches_brulees", "velharris_contested"]
behavior = "passive_aggressive_grouped"
group_size = [3, 7]
threat = "low"
loot_table = ["cristal_veine_bas", "fourrure_corrompue"]
special_trait = "humanoid_resemblance_psychological"
purification_required = false

[creature.vague_grise]
id = "vague_grise"
display_name = "Vague Grise"
corruption_level = 2
base_zone = ["mer_cendre_coast", "archipels_brume_corrupted"]
behavior = "engulf_flow"
group_size = [1, 1]
threat = "medium"
loot_table = ["cristal_veine_moyen", "essence_corruption"]
immunity = ["physical_standard"]
weakness = ["magic_fire", "purification_essence"]

[creature.chevalier_cendre]
id = "chevalier_cendre"
display_name = "Chevalier de Cendre"
corruption_level = 3
base_zone = ["stampede_zones", "deep_corrupted"]
behavior = "organized_military"
group_size = [1, 4]
threat = "high"
loot_table = ["cristal_veine_pur", "artefact_corrompu", "fragment_veine"]
special_trait = "memory_fragments_quest_trigger"
memory_trigger_items = ["faction_insignia", "military_medals"]

[creature_champion.korragh_vague]
id = "korragh_vague"
display_name = "Korragh-Vague"
type = "champion_garum"
voice = "korrakh_submerge"
base_zone = ["mer_cendre_deep"]
encounter_type = "raid"
min_players = 20
max_players = 40
threat = "extreme"
loot_table = ["equipement_epique_eau", "artefact_rare_marine", "materiau_purification_haute"]
immunity = ["physical_all"]
weakness = ["fire_magic", "heat_aoe"]
mechanics = ["fragment_split", "reform_after_split", "tide_wave"]
```

### 20.2 Nexus et zones de Gaïa

```toml
[nexus.nexus_ervanath]
id = "nexus_ervanath"
display_name = "Nexus d'Ervanath"
rank = "fondateur"
location = "ervanath_center"
faction_control = "federation_ervan"
energy_level = "extreme"
corruption_resistance = "extreme"
purification_power = "continent_scale"
accessible_to = ["federation_ervan", "guilde_aventuriers"]
restricted_to_outsiders = true
corruption_status = "clean"

[nexus.nexus_nexar]
id = "nexus_nexar"
display_name = "Nexus de la Source Mère"
rank = "fondateur"
location = "nexar_center"
faction_control = "federation_ervan"
energy_level = "very_high"
corruption_resistance = "high"
purification_power = "regional_scale"
accessible_to = ["all_pilgrim"]
restricted_to_outsiders = false
corruption_status = "clean"
pilgrimage_destination = true

[nexus.nexus_velharris_south]
id = "nexus_velharris_south"
display_name = "Nexus du Plateau Sud"
rank = "secondary"
location = "velharris_south_plateau"
faction_control = "contested"
energy_level = "medium"
corruption_resistance = "medium"
purification_power = "local_scale"
accessible_to = ["federation_ervan", "guilde_aventuriers"]
restricted_to_outsiders = false
corruption_status = "threatened"
verdeth_proximity = "high"
```

### 20.3 Événements saisonniers

```toml
[seasonal_event.stampede_major]
id = "stampede_major"
type = "stampede_scripted"
trigger = "seasonal_narrative"
duration_days = 14
alert_phase_hours = 24
pvp_suspend = true
pvp_suspend_radius_km = 5
cross_faction_cooperation = true
reward_purification_loot = true
reward_unique_craft = true
hall_of_fame_contribution = true

[seasonal_event.guerre_rvr]
id = "guerre_rvr"
type = "faction_war"
trigger = "seasonal_automatic"
duration_weeks = 6
eligible_factions = ["empire_pourpre", "alliance_de_rive", "federation_ervan"]
mercenary_factor = true
territory_stakes = true
permanent_map_change = true
hall_of_fame_contribution = false

[seasonal_event.ceremonie_redemption]
id = "ceremonie_redemption"
type = "social_redemption"
trigger = "player_action"
visibility = "server_wide_announcement"
duration_real_hours = 24
public_vote = true
vote_weight = "symbolic"
permanent_title_grant = "redempteur"
```

### 20.4 PNJ clés — Données de base

```toml
[npc.seris_kovan]
id = "seris_kovan"
display_name = "Seris Kovan"
title = "Maître des Archives Impériales"
location = "velanthara_archives"
faction = "empire_pourpre"
role = ["quest_giver", "merchant_information", "historian"]
minimum_rank_required = "civis"
faction_reputation_gate = "neutral"
hostile_to_factions = ["federation_ervan"]
secret_knowledge = "cristal_veine_garum_link"
secret_quest_unlock_condition = "rank_tribun_plus AND historical_doc_gift_5"

[npc.kaena_mer]
id = "kaena_mer"
display_name = "Kaena de la Mer"
title = "Capitaine Corsaire sous Charte"
location = "auranthos_harbor"
faction = "alliance_de_rive"
role = ["quest_giver", "merchant_naval", "route_guide"]
minimum_rank_required = "homme_libre"
faction_reputation_gate = "friendly"
hostile_to_factions = ["empire_pourpre"]
special_routes = ["archipels_brume_secret", "mer_cendre_approach"]

[npc.petyr_apothicaire]
id = "petyr_apothicaire"
display_name = "Petyr l'Apothicaire"
title = "Médecin de Cendrepas"
location = "cendrepas_clinic"
faction = "neutral"
role = ["healer", "merchant_consumable", "quest_giver"]
minimum_rank_required = "none"
faction_reputation_gate = "none"
hostile_to_factions = []
special_knowledge = "corruption_progression_medical"
special_quest = "medical_notes_for_federation"
```

---

## 21. Appendice — Chronologie condensée de Véranthas

| Date | Événement |
|------|-----------|
| Ère de la Lumière Première | Gaïa en équilibre avec Garum. Premiers humains. Nexus reconnus. |
| -2000 AO | Sédentarisation des clans. Premières cités autour des Nexus. |
| -1500 AO | Premiers Éraillés remontent des Veines Grises. |
| -500 AO | Velanthor le Pourpre fonde l'Empire Uni. |
| -400 AO | Premières Académies Mixtes. Développement magie+technologie. |
| An 0-7 AO | **Première Mal : Peste de Cendre.** Un tiers de la population. Tissage des Cinq Voix. |
| An 12-31 AO | **Deuxième Mal : Guerre des Mages Noirs.** Ardath Vorak et Vorakis. Création de la Mer de Cendre. |
| An 38-47 AO | **Troisième Mal : Grand Silence.** Nexus éteints. Éveil de Garum. |
| An 47 AO | Proclamation de l'Empire Pourpre par Velanthas II. Fondation de Velanthara. |
| An 52 AO | Pacte des Trois Ports. Fondation de l'Alliance de Rive. |
| An 63 AO | Grand Concile d'Ervanath. Fondation de la Fédération Ervan. Sylvara Erven première Première Voix. |
| An 89-93 AO | Première Guerre des Frontières (Empire vs Fédération). Traité des Brumes. |
| An 89 AO | Purge des Mages par Claudas III. Traumatisme culturel impérial. |
| An 134 AO | Karath Soleine fonde la Guilde des Aventuriers à Auranthos. |
| An 142 AO | Loran Vaex invente la Presse à Veine. Révolution industrielle-militaire impériale. |
| An 145-163 AO | Guerre des Trois Côtiers (Empire vs Alliance). Traité ambigu. |
| An 156 AO | Percée de Korreth — premier Stampede majeur documenté. Première coopération des trois factions. |
| An 165-172 AO | Deuxième Guerre des Frontières (Empire vs Fédération). Statu quo. |
| An 178 AO | Fondation de la Guilde des Mercenaires par le Pacte des Compagnies. |
| An 168 AO | Fondation de la Banque de Rive. |
| An 198 AO | Percée des Cendres — Stampede de deux ans dans les Marches Brûlées. Naissance de Cendrepas. |
| An 201 AO | Apparition de Verdeth à la frontière nord du Manteau Vert. Crise politique ervan. |
| An 215 AO | Naissance de Marta Velassian. |
| An 220-235 AO | Crise des Corsaires dans l'Alliance. Charte des Corsaires créée. |
| An 223 AO | Trahison de Liria, Archonte de Caraveth. Exilée à Velanthara. |
| An 235 AO | Torrath "le Repenti de Velanthara" — première rédemption Outlaw publique. |
| An 240 AO | Début de la guerre actuelle (Empire vs Alliance, Fédération observatrice armée). |
| An 243 AO | Percée de Korreth II. Campagne de Marta Velassian, 40 jours. |
| An 244 AO | Élection de Marta Velassian comme Imperator. |
| An 247 AO | **Date de lancement d'Allumina.** État actuel du monde. |

---

---

## 22. Checklist de cohérence canonique

Cette checklist doit être vérifiée avant chaque ajout de contenu à Allumina pour garantir la cohérence avec ce document fondateur.

- [ ] Le nom du monde est **Véranthas** (avec accent). Le continent occidental est **Velanthas**. Le continent oriental est **Aranthos**.
- [ ] Gaïa ne parle jamais directement aux mortels. Elle *résonne*. Jamais de dialogue direct Gaïa → joueur.
- [ ] Garum est IA full NPC. Aucune mécanique de diplomatie avec Garum n'est possible. Les Voix de l'Érosion peuvent interagir avec des PNJ mais pas être négociées.
- [ ] Les Stampedes suivent les cinq phases : Alerte → Percée → Submersion → Reflux → Cicatrisation.
- [ ] Le PvP est suspendu dans un rayon de 5 km autour d'un point de percée de Stampede actif.
- [ ] La rédemption Outlaw est toujours publique, jamais privée. Jamais de deuxième rédemption.
- [ ] Les cristaux de Veine sont une ressource de l'Empire Pourpre, liés cosmologiquement à Garum selon la théologie ervan — mais l'Empire nie ce lien officiellement.
- [ ] La Guilde des Aventuriers est neutre PvE uniquement. Aucun combat inter-faction dans ses QG.
- [ ] Un mercenaire qui trahit son contrat devient immédiatement Outlaw. Pas de procédure intermédiaire.
- [ ] Le Hall of Fame est saisonnier. Les inscriptions de Légende sont permanentes et inter-saisonnières.
- [ ] L'an de référence est **An 247 AO** au lancement du jeu.
- [ ] Toute nouvelle faction ou sous-faction doit être approuvée par le Conseil des Voix (game design) et ne peut pas modifier les six factions lockées.

*Fin du document AL-Lore-Factions v1.0*
*Auteur : game-designer (workflow standard Miyukini COG)*
*Prochains documents suggérés : AL-Systems-Combat.md, AL-Systems-Economy.md, AL-World-Map.md*
