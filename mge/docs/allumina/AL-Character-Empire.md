<!-- @id: AL-Character-Empire @do: reference @role: game-designer @layer: 3 @human: miyuk -->

# Allumina — Classes de l'Empire Pourpre

**Statut :** Référence canonique v1.0
**Date :** 2026-02-28
**Monde :** Véranthas, An 247 AO
**Scope :** 9 classes jouables de l'Empire Pourpre — arbres de compétences, mécaniques, lore, schémas TOML

---

## Table des matières

1. [Philosophie des classes impériales](#1-philosophie-des-classes-impériales)
2. [Légionnaire de Fer](#2-légionnaire-de-fer) — Tank infanterie lourde
3. [Chevalier Velanthor](#3-chevalier-velanthor) — Tank/DPS cavalerie lourde
4. [Ingénieur de Siège](#4-ingénieur-de-siège) — Support/DPS distance mécanique
5. [Stratège Impérial](#5-stratège-impérial) — Support commandement
6. [Agent du Bureau](#6-agent-du-bureau) — Stealth assassinat espionnage
7. [Canonnier Impérial](#7-canonnier-impérial) — DPS artillerie AoE
8. [Paladin de l'Ordre Solaire](#8-paladin-de-lordre-solaire) — Tank/Heal divin
9. [Centurion de Ligne](#9-centurion-de-ligne) — DPS mêlée agile commandement
10. [Alchimiste Militaire](#10-alchimiste-militaire) — Hybride explosifs science de guerre
11. [Matrice de synergie de groupe](#11-matrice-de-synergie-de-groupe)
12. [Schémas TOML complets](#12-schémas-toml-complets)

---

## 1. Philosophie des classes impériales

L'Empire Pourpre ne forme pas des héros. Il forge des instruments de l'État.

Chaque classe impériale reflète une institution réelle de la société velantharienne : le Légionnaire est le produit de vingt ans d'entraînement dans les casernes d'État ; l'Ingénieur de Siège porte la tradition de Loran Vaex ; l'Agent du Bureau incarne le bras invisible de l'Inquisiteur Général. Jouer un personnage impérial, c'est incarner un rouage — puissant, précis, interchangeable si nécessaire.

**Mécaniques transversales impériales :**

- **Discipline (DIS)** : ressource secondaire universelle (0-100). Générée par les actions disciplinées (formations tenues, ordres exécutés, coups portés sous commandement). Consommée par les compétences les plus puissantes. Se régénère lentement au repos ou rapidement sous l'effet d'un Stratège. La Discipline remplace la "rage" ou le "focus" des autres factions — elle est collective et militaire, pas individuelle et émotionnelle.

- **Moral d'unité** : statut de groupe (Excellent / Bon / Ébranlé / Brisé). Affecté par les morts alliées proches, les débuffs de peur, et les Stampedes. Certaines compétences de Stratège et de Centurion maintiennent ou restaurent le Moral. Un moral Brisé réduit tous les dégâts alliés de 20% et accélère le recul de formation.

- **Formation** : certaines compétences imposent ou bénéficient d'une "formation" — une disposition géométrique précise des alliés (ligne, carré, échelon). La mécanique est simple en solo, stratégique en raid. Un Légionnaire en formation avec deux autres Légionnaires adjacents reçoit un bonus défensif passif automatique.

- **Cristaux de Veine** : les machines et engins des classes techniques (Ingénieur, Canonnier, Alchimiste) consomment des **Charges de Cristal** — une ressource stackable en inventaire, craftable ou lootable. Cela crée une micro-économie de front : les combattants techniques dépendent des lignes d'approvisionnement que les Légionnaires et Centurions protègent.

---

## 2. Légionnaire de Fer

**Rôle :** Tank
**Armes principales :** Épée courte (gladius) + bouclier ovale militaire (scutum) ; Lance longue (hasta) + bouclier en option secondaire
**Ressource :** Discipline

### 2.1 Style de combat

Le Légionnaire n'attaque pas — il absorbe, contrôle, et punit. Son bouclier est une extension de lui-même : il frappe avec, esquive avec, et ancre sa ligne avec. En formation, le Légionnaire devient un mur mobile que les ennemis brisent sur leurs propres corps. Sa gestion de l'espace est millimétrée : reculer d'un pas est une décision tactique, jamais une fuite. La puissance du Légionnaire monte avec le nombre d'alliés adjacents — seul il est solide, en trio il est imprenable, en formation complète il est une forteresse.

### 2.2 Arbres de compétences

#### Arbre A — Muraille de Fer (défense absolute)

*Thème : réduction des dégâts, résistance aux crowd controls, protection passive des alliés adjacents.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Bloc Impérial** | Active | Bloque la prochaine attaque physique, génère 15 DIS si réussi, cooldown 6s |
| **Scutum Vivant** | Active | Pendant 4s, redirige 30% des dégâts subis par un allié adjacent vers soi-même |
| **Charge de Rempart** | Active | Pousse tous les ennemis dans un cône de 3m, étourdit 1,5s, génère Formation : Ligne automatiquement |
| **Cuirasse Disciplinée** (passive) | Passive | Chaque point de DIS au-dessus de 50 réduit les dégâts physiques reçus de 0,3% |
| **Mur de Boucliers** (passive) | Passive | +12% de réduction de dégâts pour chaque Légionnaire adjacent en formation |

#### Arbre B — Assaut Impérial (pression offensive)

*Thème : dégâts de pression, debuffs de résistance ennemie, punition des ennemis qui tentent de percer.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Gladius Impitoyable** | Active | Frappe rapide qui ignore 20% de l'armure cible, cooldown 4s |
| **Pression de Ligne** | Active | Pousse la ligne ennemie de 2m en avançant soi-même de 2m, inflige "Écrasé" (−15% vitesse de déplacement, 6s) |
| **Estoc Discipliné** | Active | Consomme 30 DIS, inflige ×2,5 dégâts en pénétration d'armure, ne peut être paré |
| **Acier Pur** (passive) | Passive | +8% dégâts physiques quand la cible est Écrasée ou Étourdie |
| **Vengeance Impériale** (passive) | Passive | Chaque attaque bloquée avec succès ajoute une charge (max 5) ; la prochaine attaque consume toutes les charges pour +12% dégâts par charge |

#### Arbre C — Commandement de Ligne (leadership tactique)

*Thème : auras de formation, coordination d'équipe, buffs de moral, commandement à courte portée.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Cri de Ralliement** | Active | Restaure 15 Moral aux alliés à 10m, annule un debuff de peur, cooldown 45s |
| **Formation Serrée** | Active | Ancre la position pendant 6s ; les alliés adjacents gagnent +20% défense mais ne peuvent pas bouger |
| **Ordre de Tenue** | Active | Pendant 8s, tout allié dans un rayon de 5m génère 5 DIS/s supplémentaire |
| **Dos Couvert** (passive) | Passive | Les alliés adjacents en Formation reçoivent −10% de dégâts des attaques de flanc |
| **Officier de Rang** (passive) | Passive | Le Légionnaire compte comme "commandant de section" pour les mécaniques de Stratège ; ses buffs de Formation ont 15% de durée supplémentaire |

### 2.3 Compétences signature

**[ULTIMA] Bastille Impériale** *(coût : 80 DIS, cooldown : 3 min)*
Le Légionnaire plante son bouclier en terre et ouvre les bras. Pendant 8 secondes, il devient immobile mais absorbe 60% des dégâts subis par tous les alliés dans un rayon de 6 mètres. Son armure pendant cet effet est doublée. À expiration ou si le Légionnaire tombe sous 15% de PV, une onde de choc éjecte tous les ennemis adjacents de 4m et les étourdit 2s. Mécaniquement, c'est le pivot d'un push de raid — l'équipe s'abrite derrière un homme pendant que le reste frappe.

**[SIGNATURE] Coin Velanthorien** *(coût : 50 DIS, cooldown : 90s)*
Exige au moins deux alliés dans un rayon de 4m. Le Légionnaire coordonne une manœuvre d'encerclement : tous les alliés désignés avancent simultanément de 3m, coinçant les ennemis dans la zone centrale. Les ennemis dans la zone reçoivent "Encerclé" (−25% résistances, −30% esquive) pendant 10s. Si la zone contient au moins 3 alliés, l'effet passe à 15s. Un classique de la légion velantharienne — et une compétence qui récompense la coordination réelle.

### 2.4 Synergie de groupe

Le Légionnaire est l'épine dorsale de toute composition impériale. En doublette ou trio, les Murs de Boucliers s'enchaînent. En raid, Formation Serrée crée des fenêtres d'invulnérabilité que le Canonnier exploite pour ses bombardements. Le Centurion exploite les debuffs "Écrasé" du Légionnaire pour ses compétences de finisseur. Le Stratège amplifie le Commandement de Ligne pour des buffs de zone massifs.

### 2.5 Lore

Le Légionnaire est le pilier social de l'Empire. Son enrôlement commence à 16 ans dans les Casernes d'État, son service minimal est de dix ans. Un Légionnaire honorablement libéré reçoit la **Bague de Bronze** — le signe visible d'un Citoyen de premier rang. Dans la société velantharienne, porter la Bague de Bronze ouvre plus de portes qu'une fortune commerciale. La plupart des sénateurs ont été Légionnaires. L'Empire sait que son armée est son identité.

---

## 3. Chevalier Velanthor

**Rôle :** Tank / DPS hybride
**Armes principales :** Lance de cavalerie (kontos), épée longue (spatha) ; peut utiliser bouclier cavalier léger en secondaire
**Ressource :** Discipline + Élan (0-100, généré par le mouvement et les charges)

### 3.1 Style de combat

Le Chevalier Velanthor combat monté ou démontré — deux états distincts avec des compétences différentes. Monté, il est une force de frappe mobile d'une puissance dévastatrice : la charge est son arme absolue, capable de briser une ligne entière. Démonté, il devient un DPS lourd et résilient, sa lance remplacée par la spatha. La gestion de l'Élan est centrale : chaque mètre parcouru au galop génère de l'Élan qui amplifie la prochaine frappe. Ralentir ou être stoppé réinitialise l'Élan. Le Chevalier est un prédateur de momentum — terrifiant en terrain ouvert, vulnérable dans les espaces confinés.

### 3.2 Arbres de compétences

#### Arbre A — Charge de Fer (cavalerie de choc)

*Thème : maximiser l'impact de la charge, dégâts de percée, renversement de lignes.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Charge Impériale** | Active | Sprint de 6m en ligne droite, inflige ×(1 + Élan/50) dégâts, repousse tout ennemi sur la trajectoire |
| **Lance Brisée** | Active | Consomme toute l'Élan, frappe unique qui inflige 1% des PV max de la cible par point d'Élan consommé (max 100 dégâts % PV) |
| **Piétinement** | Active | L'animal renverse les ennemis dans un rayon de 3m, "Renversé" pendant 2s — uniquement monté |
| **Cavalier Aguerri** (passive) | Passive | L'Élan se génère 25% plus vite ; les effets de ralentissement réduisent l'Élan de 30% moins vite |
| **Kontos Tranchant** (passive) | Passive | Les charges traversent les boucliers et ignorent 25% de la résistance physique des cibles bloquantes |

#### Arbre B — Lame Velanthorienne (combat démontré)

*Thème : DPS lourd démontré, résistance pendant les transitions, finisseurs.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Spatha Impériale** | Active | Enchaînement de 3 frappes rapides, le 3e coup inflige toujours un critique si les 2 premiers touchent |
| **Descente de Cheval** | Active | Se démonte en infligeant une frappe de zone (3m) qui étourdit 1,5s ; génère 40 DIS à l'atterrissage |
| **Duel d'Honneur** | Active | Verrouille une cible en "Duel" pendant 8s ; les deux combattants reçoivent +20% dégâts l'un contre l'autre, ni l'un ni l'autre ne peut être ciblé par des tiers |
| **Armure Cavalière** (passive) | Passive | −15% dégâts reçus pendant 3s après chaque Descente de Cheval ou démontage forcé |
| **Veteran de Ligne** (passive) | Passive | Si démontré depuis plus de 20s, +12% dégâts et +8% résistance physique |

#### Arbre C — Bannière de l'Aigle (leadership de cavalerie)

*Thème : buffs mobiles, coordination de la cavalerie alliée, présence de commandement.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Bannière Impériale** | Active | Déploie une bannière sur sa position pendant 15s ; tous les alliés à 8m gagnent +15% vitesse de déplacement et +10% dégâts |
| **Ordre de Charge** | Active | Tous les alliés montés ou Chevaliers à 20m reçoivent "En Avant" : prochain déplacement génère ×2 Élan, cooldown 60s |
| **Présence de l'Aigle** | Active | Aura active 10s : les ennemis à 5m subissent "Démoralisé" (−20% moral, −10% dégâts) |
| **Flanc Couvert** (passive) | Passive | Quand le Chevalier est adjacent à un Légionnaire en Formation, les deux reçoivent −10% dégâts subis |
| **Cavalerie d'Élite** (passive) | Passive | Les compétences de l'Arbre A coûtent 10 DIS de moins si 2 Chevaliers ou plus sont dans le groupe |

### 3.3 Compétences signature

**[ULTIMA] Charge des Mille** *(coût : 100 Élan + 60 DIS, cooldown : 4 min)*
Le Chevalier effectue une charge de 12m en ligne droite à une vitesse doublée. Tous les ennemis sur la trajectoire et dans un cône de 4m sont renversés et prennent des dégâts massifs en fonction de l'Élan consommé. Si d'autres Chevaliers Velanthor alliés activent la même compétence dans les 3s, leurs trajectoires se combinent : chaque Chevalier supplémentaire ajoute +50% dégâts à toutes les charges. Une seule Charge des Mille coordonnée par quatre Chevaliers peut traverser une ligne de siège entière.

**[SIGNATURE] Montée Fantôme** *(coût : 40 DIS, cooldown : 75s)*
Invoque le "Cheval d'Urgence" — une monture spectrale convoquée des stables impériaux par cristal-signal. Apparaît en 1,5s, permet un remontage immédiat même en combat. Le cheval fantôme a 40% des PV de la monture réelle et disparaît après 20s ou quand il atteint 0 PV. Utilisé tactiquement pour restaurer la mobilité après un Piétinement ou une retraite.

### 3.4 Synergie de groupe

Le Chevalier est le briseur de ligne que le Légionnaire ne peut pas être. Tandis que le Légionnaire tient, le Chevalier perce. En composition classique "Marteau-Enclume", le Légionnaire fixe la ligne et le Chevalier frappe le flanc. L'Ordre de Charge permet au Stratège de déclencher des rushes coordonnés de toute la cavalerie alliée en raid.

### 3.5 Lore

L'Ordre Équestre Velanthorien est l'aristocratie militaire de l'Empire — ses membres sont des fils (et depuis Marta Velassian, des filles) de familles qui ont offert un Chevalier à l'Empire pendant au moins trois générations consécutives. Le rang de Chevalier Velanthor donne accès au Sénat Militaire inférieur. En dehors de la guerre, les Chevaliers sont diplomates, gouverneurs et administrateurs. Leur monture est registrée à l'État et assurée par le Trésor Imperial.

---

## 4. Ingénieur de Siège

**Rôle :** Support / DPS distance
**Armes principales :** Arbalète à répétition (arme personnelle) ; engins mécaniques déployables (tourelles, catapultes de poche, mines de cristal)
**Ressource :** Charges de Cristal (0-10 stockées) + Énergie Mécanique (0-100, générée par les engins actifs)

### 4.1 Style de combat

L'Ingénieur transforme le champ de bataille en laboratoire. Il déploie, repositionne, optimise. Sa puissance est asymptotique : faible au premier cycle, redoutable au troisième. Ses engins ont une durée de vie limitée et doivent être remplacés ou réparés — gestion constante. Seul, il est vulnérable et sa DPS personnelle est médiocre. Avec 30 secondes de préparation et une ligne protectrice, il devient le contributeur de dégâts le plus efficace sur la durée. Son rôle en raid est de transformer le terrain en avantage tactique permanent.

### 4.2 Arbres de compétences

#### Arbre A — Arsenal Déployable (engins de terrain)

*Thème : tourelles automatiques, mines, barricades mécaniques.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Tourelle Velanthor MkI** | Active | Déploie une tourelle automatique (portée 10m, 15 dégâts/s, 45s de durée), consomme 2 Charges |
| **Mine de Cristal** | Active | Pose une mine invisible (détection ennemie à 0,5m), explose pour 180% dégâts de zone 3m, consomme 1 Charge |
| **Barricade Mécanique** | Active | Déploie un obstacle physique de 2m de large et 1,5m de haut pendant 30s, bloquant le passage ennemi |
| **Surcharge Mécanique** (passive) | Passive | L'Énergie Mécanique générée par les engins actifs réduit le cooldown de déploiement de 2s par 10 points |
| **Ingénierie de Précision** (passive) | Passive | Les tourelles ont +20% de portée et ignorent 15% de l'armure cible |

#### Arbre B — Artillerie Individuelle (DPS distance personnel)

*Thème : arbalète améliorée, munitions spéciales, tirs de précision.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Tir Perforant** | Active | Bolt d'arbalète qui traverse jusqu'à 3 ennemis en ligne, infligeant 100%/75%/50% dégâts successifs |
| **Munition Incendiaire** | Active | Prochain tir applique "Brûlure" (25 dégâts/s pendant 8s), consomme 1 Charge |
| **Arbalète à Répétition** | Active | Mode automatique pendant 5s : tir toutes les 0,8s, mais −30% dégâts par tir |
| **Ciblage Optique** (passive) | Passive | +15% dégâts contre les cibles à plus de 15m de distance |
| **Rechargeur Magnétique** (passive) | Passive | Les Charges de Cristal se reconstituent 20% plus vite ; cap porté à 12 |

#### Arbre C — Génie de Soutien (support tactique)

*Thème : buff d'équipement allié, réparation d'armure, optimisation du terrain.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Maintenance de Combat** | Active | Répare l'armure d'un allié (+15% durabilité effective, supprime un effet de "Corrosion") en 2s de canal |
| **Amplificateur de Cristal** | Active | Installe un boost de cristal sur un engin allié ou sa propre tourelle : +40% dégâts et portée pendant 12s |
| **Fumigène Tactique** | Active | Lance une grenade fumigène (rayon 5m, durée 10s) : les ennemis dans la fumée ont −35% précision, les alliés peuvent se déplacer à travers sans pénalité |
| **Ingénieur Résilient** (passive) | Passive | L'Ingénieur peut agir normalement pendant 4s après avoir été frappé par une compétence de stun ou de silence |
| **Réseau d'Engins** (passive) | Passive | Les engins déployés dans un rayon de 8m les uns des autres se partagent les Charges : si l'un est détruit, les autres récupèrent ses charges restantes |

### 4.3 Compétences signature

**[ULTIMA] Presse à Veine Portable** *(coût : 8 Charges + 100 Énergie, cooldown : 5 min)*
Déploie une station d'extraction de cristal miniaturisée pendant 30s. Génère 1 Charge de Cristal toutes les 3s pour l'Ingénieur et 1 Charge toutes les 6s pour les alliés proches (rayon 10m). Pendant son activation, toutes les tourelles existantes passent en mode "Surpuissance" : dégâts ×2 mais durée de vie réduite de 50%. La station peut être détruite par les ennemis (200 PV). Référence directe à la Presse à Veine de Loran Vaex.

**[SIGNATURE] Canon de Siège d'Urgence** *(coût : 5 Charges, cooldown : 2 min)*
Assemble en 4s de canal un canon léger de siège pointé dans une direction fixe. Le canon tire automatiquement toutes les 2s pendant 20s, infligeant 250% des dégâts de l'arbalète de base avec pénétration d'armure totale. Si l'Ingénieur est frappé pendant le canal d'assemblage, le canon est raté et les Charges sont perdues. Mécanique à double risque : l'Ingénieur doit être protégé pendant 4 secondes critiques.

### 4.4 Synergie de groupe

L'Ingénieur définit les couloirs de combat. Ses barricades forcent les ennemis vers des angles où ses mines explosent et ses tourelles visent. En raid de siège, il est positionné derrière les Légionnaires — son Canon de Siège d'Urgence est l'arme que les tanks protègent et que les ennemis veulent à tout prix interrompre. Le Stratège peut doubler la durée de ses engins via des compétences de Commandement.

### 4.5 Lore

Les Ingénieurs de Siège sont membres du **Corps des Machines Velanthor** — une branche militaire à part, ni pure armée ni pure académie. Ils répondent directement au Grand Trésorier pour les ressources et au Légat Suprême pour les opérations. Leur insigne est le rouage d'or sur fond pourpre — le même symbole que l'Aigle de Fer tient dans ses serres. Loran Vaex est leur saint patron officiel, célébré le jour de sa mort le 15e jour du mois des Machines.

---

## 5. Stratège Impérial

**Rôle :** Support commandement
**Armes principales :** Bâton de commandement (arme symbolique, dégâts mineurs), parchemins tactiques (cooldowns déclencheurs), pistollet d'officier (à courte portée, utilitaire)
**Ressource :** Points de Commandement (0-200, générés par les alliés qui exécutent des actions pendant ses auras actives)

### 5.1 Style de combat

Le Stratège ne combat pas — il dirige. Ses dégâts personnels sont négligeables ; sa contribution au groupe est multiplicatrice. Il lit le champ de bataille, identifie les moments critiques, et déploie des cooldowns tactiques au millimètre. Sa complexité est cognitive, pas réflexive : les bons Stratèges décident deux actions à l'avance. Il ne peut pas être absent du combat (ses auras sont de portée limitée) mais ne doit jamais être en première ligne (il est fragile). Le Stratège est la classe la plus difficile à jouer correctement et la plus récompensante en raid organisé.

### 5.2 Arbres de compétences

#### Arbre A — Commandement de Bataille (buffs d'attaque de masse)

*Thème : augmenter la puissance offensive du groupe, cooldowns d'assaut synchronisés.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Ordre d'Assaut** | Active | Tous les alliés à 15m gagnent +20% dégâts pendant 12s, génère 40 PC |
| **Percée Coordonnée** | Active | Désigne un ennemi "Cible Prioritaire" pendant 15s : tous les dégâts alliés sur cette cible +30% |
| **Rythme de Bataille** | Active | Aura de 10m pendant 20s : réduit tous les cooldowns alliés de 2s/cycle (toutes les 3s) |
| **Lecture du Champ** (passive) | Passive | Après 5s en combat, le Stratège révèle passivemnt les cooldowns restants des 3 ennemis les plus proches (affichés pour tout le groupe) |
| **Synchronisation** (passive) | Passive | Quand 3 alliés ou plus activent une compétence dans le même cycle de 1s, génère 30 PC bonus |

#### Arbre B — Commandement de Défense (buffs défensifs, repositionnement)

*Thème : absorber la pression, permettre les repositionnements, protéger les alliés fragiles.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Position de Retraite** | Active | Tous les alliés à 12m peuvent se déplacer de 3m dans la direction de leur choix sans déclencher d'attaques d'opportunité pendant 2s |
| **Bouclier Tactique** | Active | Crée un bouclier absorbant 40% des dégâts sur un allié ciblé pendant 6s |
| **Contre-Offensive** | Active | Pendant 8s, chaque attaque ennemie évitée par un allié à 10m génère une riposte automatique de 80% des dégâts de l'allié concerné |
| **Anticipation** (passive) | Passive | +15% de réduction de dégâts pour les alliés qui viennent d'exécuter une compétence active dans les 2s précédentes |
| **Mémoire Tactique** (passive) | Passive | Les cooldowns des compétences de l'Arbre B sont réduits de 20% pour chaque ennemi Élite ou Boss en combat |

#### Arbre C — Logistique de Front (soutien de ressources, moral, formation)

*Thème : régénération de Discipline, gestion du Moral, buffs de Formation.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Ravitaillement Éclair** | Active | Distribue 2 Charges de Cristal à chaque allié technique (Ingénieur, Canonnier, Alchimiste) à 15m |
| **Moral d'Acier** | Active | Remet le Moral d'unité à "Bon" immédiatement, supprime tout debuff de peur, cooldown 90s |
| **Parchemin de Formation** | Active | Force tous les alliés à 10m à adopter la Formation désignée (Ligne/Carré/Échelon) instantanément, +15% bonus de Formation pendant 20s |
| **Intendant du Corps** (passive) | Passive | Tous les alliés à 15m régénèrent +3 DIS/s en plus de leur régénération normale |
| **Officier Confirmé** (passive) | Passive | Les Points de Commandement se génèrent 25% plus vite ; cap augmenté à 250 |

### 5.3 Compétences signature

**[ULTIMA] Manœuvre Velanthor** *(coût : 200 PC, cooldown : 5 min)*
Le Stratège déploie un plan de bataille qui dure 25s. Pendant cette durée : tous les alliés à 20m reçoivent +25% dégâts, +20% résistance, −15% cooldowns. Chaque ennemi tué pendant la Manœuvre génère un surge d'Élan pour les Chevaliers alliés et 10 DIS pour tous les autres. C'est la compétence la plus puissante du jeu en termes de DPS de groupe — un bon Stratège choisit le moment de son utilisation avec une précision de chirurgien.

**[SIGNATURE] Exécution Tactique** *(coût : 80 PC, cooldown : 2 min)*
Désigne deux alliés qui reçoivent "Synchronisés" pendant 12s. Quand les deux alliés frappent la même cible dans la même fenêtre de 0,5s, les deux coups sont comptés comme des critiques. Si un troisième allié frappe dans le même 0,5s, tous les trois obtiennent également +50% dégâts sur ce coup. Une mécanique qui force la coordination vocale — et récompense massivement les groupes organisés.

### 5.4 Synergie de groupe

Le Stratège est le multiplicateur de tout le groupe. Sans lui, une composition impériale est solide. Avec lui, elle est redoutable. Sa Manœuvre Velanthor est typiquement synchronisée avec la Charge des Mille du Chevalier, le Canon de Siège de l'Ingénieur, et la Bastille du Légionnaire pour un burst de 25 secondes capable de vider n'importe quelle phase de boss.

### 5.5 Lore

Le Stratège Impérial est issu de l'**Académie de Tactique de Velanthara** — une école militaire d'élite dont l'admission est parmi les plus compétitives de l'Empire. Seuls 40 diplômés par an rejoignent le Corps des Stratèges. En temps de paix, ils sont gouverneurs, diplomates et directeurs administratifs. Marta Velassian elle-même était Stratège avant de devenir commandante de terrain, puis Imperator.

---

## 6. Agent du Bureau

**Rôle :** DPS (Stealth)
**Armes principales :** Dague à double lame (combos rapides), arbalète de poing (distance, silence), fioles d'acide et de poison (consommables d'artisanat)
**Ressource :** Ombre (0-100, générée en stealth et par actions furtives) + Exposition (0-100, croît quand l'Agent agit ou est repéré — réduit au repos)

### 6.1 Style de combat

L'Agent du Bureau est le secret honteux de l'Empire — le bras de l'Inquisiteur Général que le Sénat Martial préfère ne pas voir. Son combat est un ballet de positionnement : entrer en stealth, analyser les cibles, frapper depuis l'angle optimal avec des dégâts garantis criiques, se désengager avant que l'Exposition ne monte trop. L'Exposition est son ennemi intime : à 100, il est "Exposé" et ne peut plus entrer en stealth pendant 10s. Gérer l'Exposition, c'est survivre. Un Agent bien joué n'est jamais vu que deux fois : quand il veut l'être et quand sa cible s'effondre.

### 6.2 Arbres de compétences

#### Arbre A — Dossier Noir (assassinat et dégâts de burst)

*Thème : dégâts garantis depuis l'ombre, finisseurs, exécutions.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Frappe Exécutoire** | Active | Depuis le stealth, dégâts ×3,5 sur la prochaine attaque ; si la cible est sous 25% PV, ×5. Rompt le stealth |
| **Double Jugement** | Active | Deux frappes simultanées de dagues, la seconde infligeant 125% de la première si la première touche |
| **Sentence du Bureau** | Active | Applique "Marqué" (visible uniquement pour l'Agent et ses alliés) pendant 30s ; les dégâts contre une cible Marquée +25% et ignorent la défense d'esquive |
| **Lame Fantôme** (passive) | Passive | Les attaques depuis le stealth ajoutent "Saignement" automatiquement (30 dégâts/s, 8s, non-annulable) |
| **Exécuteur d'État** (passive) | Passive | Contre les cibles sous 30% PV, +20% dégâts et −5 Exposition par coup |

#### Arbre B — Protocole d'Ombre (stealth, mobilité, contre-mesures)

*Thème : stealth avancé, réduction d'Exposition, déplacements tactiques.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Camouflage Impérial** | Active | Entre en stealth immédiatement (même en combat, coût 30 Ombre), se maintient 8s ou jusqu'à une action |
| **Repli de Bureau** | Active | Téléportation de 5m dans une direction, ajoute 1s d'immunité aux dégâts, réduit l'Exposition de 20 |
| **Brouillard d'Identité** | Active | Crée une illusion statique de soi-même pendant 6s ; les ennemis cibleront l'illusion en priorité |
| **Ombre Active** (passive) | Passive | En stealth, l'Agent se déplace à 80% de sa vitesse normale (au lieu de 50%) |
| **Profil Bas** (passive) | Passive | L'Exposition diminue 40% plus vite au repos et 20% plus vite même pendant les combats |

#### Arbre C — Chimie Militaire (poison, acide, debuffs)

*Thème : dégâts dans le temps, réduction de stats ennemies, perturbation.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Enduit Neurotoxique** | Active | Enrobe les dagues en poison pendant 15s : chaque frappe applique "Neurotoxine" (−20% vitesse d'attaque, 10s, stackable 3x) |
| **Acide de Bureau** | Active | Lance une fiole d'acide (portée 8m) qui réduit l'armure physique de la cible de 35% pendant 20s |
| **Gaz Paralysant** | Active | Grenade à gaz (rayon 4m) : les ennemis dans la zone sont "Ralentis" (−50% déplacement) pendant 5s et ne peuvent pas utiliser de compétences actives |
| **Alchimie de Terrain** (passive) | Passive | Les fioles d'acide et de poison durent 25% plus longtemps et ont un rayon d'effet de 0,5m supplémentaire |
| **Immunité Chimique** (passive) | Passive | L'Agent est immunisé à ses propres poisons et acides, et résiste à 50% des effets de poison ennemis |

### 6.3 Compétences signature

**[ULTIMA] Dossier Scellé** *(coût : 80 Ombre + 0 Exposition requis, cooldown : 4 min)*
L'Agent disparaît totalement pendant 12s (Exposition bloquée à 0, aucune révélation possible sauf magie ervan spécifique). Pendant cette invisibilité absolue, il peut se déplacer à vitesse normale, placer des mines de poison (3 maximum), marquer jusqu'à 3 cibles différentes avec "Sentence du Bureau", et préparer une Frappe Exécutoire améliorée (×5 dégâts de base). À l'expiration des 12s ou à sa première attaque, toutes les mines s'activent simultanément. La compétence ultimate d'élimination multiple la plus redoutée du jeu en PvP impérial.

**[SIGNATURE] Rapport au Bureau** *(coût : 40 Ombre, cooldown : 90s)*
L'Agent transmet un rapport d'observation : révèle à tous ses alliés les PV, les cooldowns restants et les buffs actifs de toutes les cibles ennemies dans un rayon de 20m pendant 20s. En PvP de faction, c'est un outil de renseignement inestimable. En PvE, il permet au Stratège de synchroniser parfaitement ses cooldowns avec la fenêtre d'attaque d'un boss.

### 6.4 Synergie de groupe

L'Agent est le couteau dans la fissure. Tandis que le Légionnaire absorbe et le Chevalier charge, l'Agent cible le healer ou le Stratège ennemi. Rapport au Bureau transforme le groupe entier en équipe de renseignement. En raid, deux Agents peuvent coordonner des Dossiers Scellés en alternance pour une pression invisible permanente.

### 6.5 Lore

Le Bureau des Phénomènes Anormaux recrute dans l'anonymat total. Les Agents n'ont officiellement pas de grade dans la hiérarchie militaire — ils ont un numéro de dossier. Leur existence est connue mais inavouée : chaque sénateur sait que l'Inquisiteur Général a ses Agents, personne ne dit leur nom en public. Un Agent libéré du service (chose rare) reçoit une nouvelle identité complète et un "effacement" de son dossier. La moitié des marchands les plus prospères de Velanthara ont un passé qu'ils ne mentionneront jamais.

---

## 7. Canonnier Impérial

**Rôle :** DPS (artillerie AoE longue portée)
**Armes principales :** Canon portatif à épaule (lourd, lent, dévastateur), mortier de campagne (déployable, zone)
**Ressource :** Charges de Cristal (0-8) + Cycle de Rechargement (barre linéaire, 0-100% — les compétences ne sont disponibles qu'à 100%)

### 7.1 Style de combat

Le Canonnier est un investissement différé. Chaque tir coûte du temps de rechargement — pendant ce temps, il est vulnérable et presque sans défense. Ses dégâts quand il tire sont parmi les plus élevés du jeu, mais la gestion du cycle de rechargement est l'art de sa classe. Il ne peut pas spammer : il doit choisir le bon moment, viser le bon point d'impact, et prévoir sa prochaine couverture. En PvE de masse (Stampedes), le Canonnier est roi — ses AoE nettoient des vagues entières. En PvP, il est une cible prioritaire que ses alliés doivent protéger à tout prix.

### 7.2 Arbres de compétences

#### Arbre A — Artillerie Lourde (dégâts massifs de zone)

*Thème : maximiser les dégâts d'impact, portée, pénétration d'armure.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Obus Standard** | Active | Tir direct (portée 25m), dégâts élevés en point d'impact, zone 2m, consomme 1 Charge et 100% Rechargement |
| **Obus Perforant** | Active | Tir qui traverse les obstacles (murs de taille ≤2m), pénètre 50% de l'armure, zone réduite 1m, consomme 2 Charges |
| **Salve d'Artillerie** | Active | Tire 3 obus en 1,5s en arc de cercle couvrant 6m de largeur (consomme 3 Charges, reload 150%) |
| **Calibrage Maître** (passive) | Passive | +20% dégâts si le Canonnier ne s'est pas déplacé depuis 4s avant le tir |
| **Obus Blindé** (passive) | Passive | Les obus standards ignorent 20% de l'armure physique de toutes les cibles |

#### Arbre B — Mortier de Campagne (zone déployable, siège indirect)

*Thème : tir indirect, zone de déni, siège de positions.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Déploiement Mortier** | Active | Assemble le mortier en 3s ; pendant 30s, peut effectuer des tirs indirects jusqu'à 40m, zone 4m, mais ne peut pas se déplacer |
| **Frappe de Suppression** | Active | Tir de mortier qui applique "Supprimé" (−40% déplacement, −25% vitesse d'attaque) dans la zone touchée pendant 6s |
| **Barrage Continu** | Active | 5 obus de mortier tombent en 5s sur une zone de 8m de diamètre désignée (consomme 4 Charges) |
| **Cartographie de Tir** (passive) | Passive | Après 2 tirs de mortier sur la même zone, les suivants ont +15% dégâts dans cette zone (mémoire de 60s) |
| **Mobilité de Siège** (passive) | Passive | Le temps d'assemblage et démontage du mortier est réduit de 1s ; peut se déplacer de 2m sans démonter |

#### Arbre C — Munitions Spéciales (effets de statut, contrôle de zone)

*Thème : obus à effets spéciaux, contrôle de terrain, debuffs.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Obus Incendiaire** | Active | Zone de feu (rayon 3m) pendant 10s ; les ennemis entrant dans la zone reçoivent 40 dégâts feu/s |
| **Obus Fumigène** | Active | Zone de fumée opaque (rayon 5m, 15s) bloquant la ligne de mire et causant "Aveuglé" (−60% précision) aux ennemis |
| **Obus de Concussion** | Active | Impact qui étourdit tous les ennemis dans un rayon de 3m pendant 2,5s (résistance aux CC réduit à 1,8s/1,2s) |
| **Charge Maximale** (passive) | Passive | Si le Rechargement est à 100% depuis plus de 5s (overcharge), le prochain tir inflige +35% dégâts |
| **Expert en Explosifs** (passive) | Passive | Les zones d'effet des obus ont un rayon +0,5m supplémentaire |

### 7.3 Compétences signature

**[ULTIMA] Bombardement Impérial** *(coût : 8 Charges + 200% Rechargement, cooldown : 6 min)*
Le Canonnier désigne une zone de 12m de diamètre. Après 2s de marquage (visible pour les ennemis — ils peuvent fuir), 8 obus tombent en 4s sur toute la zone dans un ordre pseudo-aléatoire. Chaque obus inflige 200% des dégâts standards et applique des effets d'incendie. Si la zone est dans un rayon de 5m d'une Mine de Cristal d'Ingénieur, la mine explose également. Le combo Ingénieur/Canonnier pour le Bombardement + explosion de mines simultanée est le push de siège le plus efficace de l'Empire.

**[SIGNATURE] Tir Rasant** *(coût : 2 Charges + 100% Rechargement, cooldown : 45s)*
Tir à très haute trajectoire qui tombe presque verticalement sur une position à 30-40m. Ignore totalement les couverts, les obstacles et les boucliers levés. Les cibles derrière un Légionnaire en Formation Serrée ou derrière des barricades n'ont aucune protection contre ce tir. Dégâts légèrement inférieurs (−20%) à l'Obus Standard mais son utilité tactique en contournement est invaluable.

### 7.4 Synergie de groupe

Le Canonnier est protégé par les tanks, dirigé par le Stratège, et synergise avec l'Ingénieur sur les zones de mines. La véritable synergie à trois : Légionnaire ancre la Formation, Stratège lance Rythme de Bataille (réduisant le Rechargement), Canonnier enchaîne les tirs deux fois plus vite. En Stampede, le Canonnier est priorité absolue de protection pour maximiser le nettoyage de vagues.

### 7.5 Lore

Le Corps de l'Artillerie Impériale est séparé du Corps des Machines — un choix politique. Les Ingénieurs font des engins, les Canonniers tirent. La distinction évite qu'un seul Corps ne contrôle trop de puissance de feu. Les Canonniers viennent typiquement des régions minières du nord, où les enfants apprennent à doser les charges d'extraction dès 10 ans. Leur insigne est un obus d'or sur fond pourpre.

---

## 8. Paladin de l'Ordre Solaire

**Rôle :** Tank / Heal
**Armes principales :** Marteau de guerre solaire (lourd, lent, dégâts sacrés), bouclier gravé de l'Aigle de Sorath
**Ressource :** Foi Solaire (0-100, générée par les auras actives et les soins effectués)

### 8.1 Style de combat

Le Paladin est la seule classe impériale qui utilise explicitement de la magie — la magie divine de Sorath, soigneusement distinguée de la "sorcellerie primitive" ervan et de la magie de cristal technique. C'est une frontière politique fragile que le Paladin incarne quotidiennement. Son combat combine la solidité d'un Légionnaire (armure lourde, bouclier) avec des auras de soutien passives et des soins limités mais précieux. Sa Foi Solaire détermine la puissance de ses soins et de ses auras — plus il protège des alliés, plus Sorath lui accorde de pouvoir. Un Paladin seul est un bon tank. Un Paladin entouré d'alliés est presque indestuctible et les maintient en vie.

### 8.2 Arbres de compétences

#### Arbre A — Lumière de Sorath (soins et protections divines)

*Thème : soins progressifs, boucliers divins, résistance aux statuts.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Bénédiction Solaire** | Active | Soin d'un allié ciblé : restaure 18% de ses PV max, consomme 30 Foi |
| **Lumière Gardienne** | Active | Bouclier divin sur un allié (absorbe les dégâts équivalents à 25% de ses PV max pendant 8s), consomme 40 Foi |
| **Purification Solaire** | Active | Supprime 2 effets négatifs sur un allié ciblé ; si les effets supprimés étaient des poisons ou malédictions, soin bonus de 8% PV |
| **Résonance de Sorath** (passive) | Passive | Chaque soin effectué génère 10 Foi ; les soins au-delà des PV max se convertissent en bouclier temporaire (durée 6s) |
| **Lumière Partagée** (passive) | Passive | Quand le Paladin reçoit des dégâts, 15% des dégâts sont redistribués comme soin à l'allié avec les PV les plus bas à 8m |

#### Arbre B — Bouclier de l'Ordre (défense et résistance)

*Thème : absorber et punir, résistance magique, protection de zone.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Sceau Sacré** | Active | Le prochain bloc du bouclier libère une explosion de lumière : 100% des dégâts bloqués sont reflétés sur l'attaquant |
| **Bastion Solaire** | Active | Pendant 6s, le Paladin et les alliés adjacents sont immunisés aux effets de déplacement forcé (knockback, pull) |
| **Jugement de l'Ordre** | Active | Frappe au marteau qui inflige "Jugé" (la cible perd 20% de sa résistance magique et physique pendant 12s) |
| **Armure de Foi** (passive) | Passive | +1% résistance magique pour chaque 5 points de Foi ; maximum +20% à 100 Foi |
| **Indéfectible** (passive) | Passive | Si les PV du Paladin tombent sous 20%, il génère instantanément 50 Foi et son prochain soin sur soi-même est ×2 |

#### Arbre C — Auras de l'Aigle (buffs de zone passifs)

*Thème : auras permanentes ou semi-permanentes bénéficiant au groupe entier.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Aura de Dévotion** | Active | Active une aura (rayon 8m, durée libre) : tous les alliés régénèrent 1,5% PV max/s — coût 2 Foi/s |
| **Aura de Martyre** | Active | Active une aura (rayon 8m) : le Paladin absorbe 20% des dégâts des alliés dans la zone — coût 3 Foi/s |
| **Aura de Rétribution** | Active | Active une aura (rayon 6m) : +15% dégâts physiques pour tous les alliés — coût 4 Foi/s |
| **Maître des Auras** (passive) | Passive | Peut maintenir 2 auras simultanément (au lieu d'1) ; coût total des auras actives −20% |
| **Ferveur Solaire** (passive) | Passive | Chaque ennemi tué dans un rayon de 10m génère 5 Foi immédiatement |

### 8.3 Compétences signature

**[ULTIMA] Lumière d'Aeternum** *(coût : 100 Foi, cooldown : 5 min)*
Le Paladin lève son bouclier et une colonne de lumière solaire descend sur sa position (rayon 10m). Pendant 10s : tous les alliés dans la zone régénèrent 5% PV max/s, sont immunisés aux poisons et malédictions, et reçoivent +20% résistance physique. Les ennemis entrant dans la zone reçoivent 80 dégâts sacrés/s et "Aveuglés" (−50% précision). La zone est visible de très loin — en raid, c'est un signal de ralliement autant qu'un outil de combat.

**[SIGNATURE] Sacrifice Consacré** *(coût : 60 Foi, cooldown : 2 min)*
Le Paladin désigne un allié et l'imprègne d'une marque divine pendant 12s. Si l'allié marqué devrait mourir (0 PV), la mort est absorbée par la marque : l'allié reste à 1 PV et est invulnérable 2s. Le Paladin perd 40% de ses PV propres en contrepartie. Une seule utilisation par alliance (la marque ne peut pas être appliquée à deux alliés simultanément). La mécanique de "réserve de vie" la plus dramatique du jeu — et la plus risquée pour celui qui la donne.

### 8.4 Synergie de groupe

Le Paladin est le sustain de la composition. Là où le Stratège amplifie les dégâts, le Paladin garantit la survie. L'Aura de Dévotion combinée à la régénération de DIS du Stratège crée une ligne quasi-imprenable. En raid, le Paladin se positionne au centre du groupe, ses auras couvrant un maximum d'alliés. La compétence Sacrifice Consacré est typiquement réservée au Stratège — perdre le Stratège en plein Manœuvre Velanthor serait catastrophique.

### 8.5 Lore

L'Ordre Solaire est la seule institution religieuse officiellement reconnue par l'Empire. Ses Paladins sont formés dans le Temple de la Flamme Éternelle à Velanthara — un bâtiment dont la flamme au sommet brûle depuis la fondation de l'Empire (alimentée en cristal de Veine, bien qu'aucun prêtre ne l'admette officiellement). L'Ordre marche la frontière politique délicate entre magie acceptée et "sorcellerie" — ses membres passent la moitié de leur formation à apprendre comment articuler philosophiquement la différence entre la Lumière de Sorath et la magie ervan.

---

## 9. Centurion de Ligne

**Rôle :** DPS mêlée agile
**Armes principales :** Deux épées courtes (gladii jumelés) ou épée + armure légère (style plus défensif), optionnellement une lance courte (pilum)
**Ressource :** Élan de Guerre (0-100, généré par les frappes consécutives — se réinitialise si 2s sans frapper)

### 9.1 Style de combat

Le Centurion est la vitesse là où le Légionnaire est la masse. Il ne bloque pas — il esquive, contre-attaque, rebondit. Son combat est un enchaînement de frappes rythmiques dont la puissance monte avec l'Élan de Guerre : les premières frappes sont ordinaires, mais à 80+ Élan les compétences deviennent dévastatrices. Il commande aussi sa "section" — il peut coordonner des alliés proches via des cris de guerre qui buffent leur prochain coup. La dualité Centurion/Légionnaire est le duo le plus classique de l'Empire : l'un fixe, l'autre lacère.

### 9.2 Arbres de compétences

#### Arbre A — Lame Jumelle (DPS pur, combos)

*Thème : enchaînements de frappes, vitesse d'attaque, finisseurs d'Élan.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Estoc-Riposte** | Active | Deux frappes rapides (main droite puis gauche) qui génèrent 15 Élan chacune si elles touchent |
| **Tempête de Gladii** | Active | Enchaînement de 5 frappes en 2,5s à une cible (ou réparties sur plusieurs cibles adjacentes) — chaque frappe génère 10 Élan |
| **Finisseur d'Élan** | Active | Consomme tout l'Élan : inflige (Élan consommé × 2,2)% des dégâts de base comme frappe unique |
| **Rythme de Combat** (passive) | Passive | +8% vitesse d'attaque pour chaque 25 points d'Élan actifs |
| **Gladius Affûté** (passive) | Passive | Chaque frappe consécutive sur la même cible ajoute "Entaille" stackable (max 5, +4% dégâts par stack) |

#### Arbre B — Mobilité de Section (esquive, repositionnement, harcèlement)

*Thème : déplacements rapides, esquive active, pression de flanc.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Pas du Centurion** | Active | Bond de 4m dans n'importe quelle direction, ne rompt pas l'Élan (comptabilisé comme continuation du combo) |
| **Manœuvre de Flanc** | Active | Téléporte derrière la cible en 0,3s, la prochaine frappe inflige +50% dégâts et applique "Déstabilisé" (−20% résistance physique, 5s) |
| **Harcèlement de Ligne** | Active | Attaque 3 cibles différentes adjacentes dans un rayon de 4m en 1,5s, maintenant l'Élan entre chaque frappe |
| **Esquive Instinctive** (passive) | Passive | +15% de chance d'esquive passive quand l'Élan est supérieur à 50 |
| **Marathon de Guerre** (passive) | Passive | La vitesse de déplacement est augmentée de 12% quand l'Élan est supérieur à 75 |

#### Arbre C — Commandement de Section (buffs de meute, leadership de mêlée)

*Thème : buff des alliés proches, cris de guerre, coordination de ligne.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Cri du Centurion** | Active | Tous les alliés à 10m gagnent +20% vitesse d'attaque pendant 8s, génère 20 Élan pour chaque allié dans la zone |
| **Offensive Coordonnée** | Active | Désigne une cible ; tous les alliés à 8m effectuent automatiquement leur prochaine attaque sur cette cible dans les 2s |
| **Marche Forcée** | Active | Tous les alliés à 10m gagnent +30% vitesse de déplacement pendant 6s et ignorent les effets de ralentissement |
| **Entraînement de Meute** (passive) | Passive | Chaque allié à 6m qui frappe ajoute 5 Élan au Centurion (max 20/s depuis cette source) |
| **Autorité de Section** (passive) | Passive | Les compétences de l'Arbre C ont des cooldowns réduits de 25% si 3 alliés ou plus sont dans un rayon de 8m |

### 9.3 Compétences signature

**[ULTIMA] Décision de Centurion** *(coût : 80 Élan, cooldown : 3 min)*
Le Centurion effectue une série de 8 frappes ultra-rapides en 3s, chaque frappe ciblant l'ennemi le plus proche (ou la cible principale si aucun autre ennemi n'est adjacent). Les 4 premières frappes sont des attaques normales. Les 4 dernières sont automatiquement des critiques. L'Élan est maintenu et augmente de 10 par frappe pendant la séquence. Si l'Élan atteint 100 avant la fin de la séquence, la dernière frappe devient Finisseur d'Élan intégré (sans coût supplémentaire).

**[SIGNATURE] Exemple du Rang** *(coût : 30 Élan, cooldown : 60s)*
Le Centurion effectue une attaque spectaculaire visible de loin (effets visuels marqués). Tous les alliés qui voient l'attaque (ligne de vue, rayon 20m) reçoivent "Inspiré" pendant 12s : +15% dégâts et l'Élan se génère 30% plus vite. Mécaniquement, c'est un buff à déclenchement visuel — en raid, le Centurion peut se positionner pour qu'un maximum d'alliés le voient. Thématiquement, c'est le centurion qui mène par l'exemple devant ses troupes.

### 9.4 Synergie de groupe

Le Centurion est le DPS de soutien mobile. Il nettoie les flancs que le Légionnaire ne peut pas couvrir, harcèle les archers ennemis qui ciblent l'Ingénieur ou le Canonnier, et maintient le Moral d'unité par son Exemple du Rang. Sa Offensive Coordonnée peut transformer une mêlée dispersée en focus-fire instantané — à condition que le Stratège soit là pour en amplifier la fenêtre.

### 9.5 Lore

Les Centurions sont les officiers de terrain de l'infanterie impériale — ils commandent les sections de 80 soldats qui forment les légions. Dans le jeu, un joueur Centurion incarne l'élite de cette catégorie : les Centurions de Premier Rang, ceux qui ont survécu assez longtemps pour développer un style de combat personnel reconnaissable. Historiquement, plusieurs Centurions de Premier Rang sont montés jusqu'au rang de Légat et de Préfet.

---

## 10. Alchimiste Militaire

**Rôle :** DPS hybride (explosifs / science de guerre)
**Armes principales :** Lance-grenades de campagne (distance courte-moyenne), fioles alchimiques (consommables craftés), fouet d'acide (mêlée de zone)
**Ressource :** Pression Alchimique (0-100, générée par les explosions déclenchées) + Réactifs (ressource d'inventaire stackable)

### 10.1 Style de combat

L'Alchimiste Militaire est le chaos contrôlé de l'Empire. Ses dégâts sont imprévisibles dans leur distribution mais redoutables dans leur cumul. Il jongle avec des effets chimiques multiples — acide, feu alchimique, gaz corrosif, grenades à percussion — et les combine pour des réactions en chaîne. Sa Pression Alchimique monte avec chaque explosion, amplifiant ses dégâts successifs mais risquant une "Surchauffe" (Pression à 100 sans décharge = explosion incontrôlée qui blesse aussi les alliés). La gestion de la Pression est l'art de sa classe : maintenir la pression haute pour amplifier les dégâts sans atteindre la limite fatale.

### 10.2 Arbres de compétences

#### Arbre A — Chimie Explosive (grenades, projectiles, dégâts de zone)

*Thème : explosions multiples, enchaînements de zones, dégâts de burst.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Grenade à Percussion** | Active | Lance une grenade (portée 10m, zone 3m), inflige dégâts physiques, génère 15 Pression |
| **Grenade Incendiaire** | Active | Zone de feu alchimique (rayon 2,5m, 8s) — plus intense que le feu normal : 60 dégâts/s, consomme 1 Réactif |
| **Charge en Chaîne** | Active | Deux grenades reliées par un câble de détonation : lancer l'une, l'autre explose automatiquement 1,5s après si à moins de 8m — zone combinée |
| **Expert en Détonation** (passive) | Passive | Les explosions dans un rayon de 3m d'une autre explosion génèrent +10 Pression bonus |
| **Composition Renforcée** (passive) | Passive | +15% dégâts de toutes les grenades et projectiles alchimiques |

#### Arbre B — Chimie Corrosive (acide, gaz, debuffs persistants)

*Thème : réduction d'armure, dégâts dans le temps, contrôle de terrain par les gaz.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Jet d'Acide** | Active | Jet liquide (portée 6m, ligne) qui réduit l'armure physique de toutes les cibles touchées de 30% pendant 15s, consomme 1 Réactif |
| **Bombe à Gaz Corrosif** | Active | Zone de gaz corrosif (rayon 4m, 12s) : −20% résistances à tous les dégâts pour les ennemis dans la zone |
| **Fouet d'Acide** | Active | Frappe de mêlée (portée 3m) en arc, applique "Dissolution" sur tous les ennemis touchés (25 dégâts acide/s, 10s, stackable 2x) |
| **Corrosion Catalytique** (passive) | Passive | Les effets d'acide amplifient les dégâts des explosions alliées de 15% contre les cibles affectées |
| **Résistance Chimique** (passive) | Passive | L'Alchimiste est immunisé à ses propres effets chimiques et résiste à 40% de tout dégât chimique ennemi |

#### Arbre C — Alchimie de Soutien (potions offensives, buffs de groupe, science défensive)

*Thème : potions offensives distribuées, buffs de combat, protection alchimique.*

| Compétence | Type | Effet |
|-----------|------|-------|
| **Potion de Fureur** | Active | Lance une fiole sur un allié ciblé : +25% dégâts et +15% vitesse d'attaque pendant 10s — mais l'allié prend 5% de ses PV max en dégâts par seconde pendant la durée |
| **Écran de Fumée Alchimique** | Active | Fumigène dense (rayon 6m, 12s) — les ennemis dans la fumée sont Aveuglés et leur Pression (Élan, Foi, etc.) se régénère 50% plus lentement |
| **Bombe de Soin d'Urgence** | Active | Lance une fiole alchimique verte qui éclate sur un allié : restaure 20% de ses PV max instantanément, consomme 2 Réactifs |
| **Laboratoire Mobile** (passive) | Passive | L'Alchimiste peut crafter des Réactifs de base en combat (action de 2s, pas de matériaux requis, max 2 fois par combat, cooldown 30s) |
| **Surchauffe Contrôlée** (passive) | Passive | Quand la Pression atteint 100, au lieu d'une explosion incontrôlée, l'Alchimiste peut déclencher une **Décharge Dirigée** : libère toute la Pression comme projectile concentré ciblé (une seule cible, dégâts massifs). Cooldown de cette Décharge : 60s |

### 10.3 Compétences signature

**[ULTIMA] Grand Œuvre Militaire** *(coût : 80 Pression + 5 Réactifs, cooldown : 6 min)*
L'Alchimiste prépare pendant 3s un mélange alchimique ultime et lance une sphère instable de 8m de diamètre sur une zone désignée (portée max 15m). À l'impact : premier anneau (2m) — explosion massive. Second anneau (5m) — acide qui réduit toutes les armures de 45%. Troisième anneau (8m) — gaz corrosif qui persiste 15s. Les ennemis traversant les différentes zones accumulent les effets. Le Grand Œuvre est techniquement la compétence avec le plus d'états simultanés du jeu — et la plus dangereuse à utiliser en espace confiné pour les alliés.

**[SIGNATURE] Injection de Combat** *(coût : 2 Réactifs, cooldown : 45s)*
S'injecte directement un stimulant alchimique (animation rapide de 0,5s). Pendant 8s : vitesse d'attaque +30%, génération de Pression +50%, immunité aux étourdissements et silences. À l'expiration : l'Alchimiste subit "Crash Alchimique" (−20% dégâts, −15% vitesse de déplacement, 6s). La compétence du joueur agressif qui choisit d'accepter la vulnérabilité post-burst pour maximiser le burst lui-même.

### 10.4 Synergie de groupe

L'Alchimiste est le spécialiste des phases chaotiques — Stampede, percée d'ennemi, engagement de masse. Ses gaz corrosifs amplifient les dégâts de tout le groupe (Corrosion Catalytique est l'une des meilleures synergies passives du jeu). La Potion de Fureur sur le Centurion combinée au Cri du Centurion crée un pic de DPS à courte durée considérable — au prix de la santé du Centurion si la situation se prolonge.

### 10.5 Lore

Les Alchimistes Militaires du **Laboratoire de Campagne Impérial** (LCI) sont peut-être la branche la plus controversée de l'armée impériale. Officiellement, leur science est "chimie mécanique appliquée" — aucune magie n'entre en jeu. Officieusement, certains Réactifs qu'ils utilisent proviennent directement des Veines Grises, et le Bureau des Phénomènes Anormaux surveille le LCI de très près. Les Alchimistes eux-mêmes savent que la ligne entre "chimie des cristaux" et "magie de Garum" est philosophique, pas technique.

---

## 11. Matrice de synergie de groupe

### 11.1 Composition standard (5 joueurs)

| Slot | Classe | Rôle dans la comp |
|------|--------|------------------|
| Tank | Légionnaire de Fer | Absorbe, ancre la formation |
| Tank/DPS | Chevalier Velanthor | Perce, flanque, mobilité |
| Heal/Tank | Paladin de l'Ordre Solaire | Sustain, auras, survie critique |
| Support | Stratège Impérial | Amplification, cooldowns, Moral |
| DPS | Canonnier Impérial | Dégâts de masse, zone |

### 11.2 Composition raid (25 joueurs — recommandation impériale)

| Rôle | Classes | Quantité recommandée |
|------|---------|---------------------|
| Tanks de ligne | Légionnaire | 4-6 |
| Tanks mobiles | Chevalier | 2-3 |
| Heals | Paladin | 3-4 |
| Commandement | Stratège | 2 |
| Artillerie | Canonnier | 3-4 |
| DPS mêlée | Centurion | 4-5 |
| Technique | Ingénieur de Siège | 2-3 |
| Furtif | Agent du Bureau | 1-2 |
| Explosifs | Alchimiste | 1-2 |

### 11.3 Tableau de synergie inter-classes

| Classe | Meilleure synergie avec | Raison mécanique |
|--------|------------------------|-----------------|
| Légionnaire | Stratège, Centurion | Formation Serrée + Ordre d'Assaut ; Offensive Coordonnée derrière le mur |
| Chevalier | Stratège, Légionnaire | Ordre de Charge déclenché par Stratège ; Flanc Couvert passif |
| Ingénieur | Canonnier, Stratège | Mines + Bombardement Impérial ; Ravitaillement Éclair |
| Stratège | Tout le groupe | Multiplicateur universel |
| Agent | Stratège, Centurion | Rapport au Bureau + Exécution Tactique ; Sentence partagée |
| Canonnier | Ingénieur, Stratège | Mines + obus ; Rythme de Bataille réduit rechargement |
| Paladin | Stratège, Légionnaire | Sacrifice Consacré sur Stratège ; Aura + Formation |
| Centurion | Paladin, Légionnaire | Potion de Fureur du Alchimiste ; mêlée derrière mur de boucliers |
| Alchimiste | Centurion, Canonnier | Corrosion Catalytique amplifie tous les dégâts explosifs |

---

## 12. Schémas TOML complets

```toml
# AL-Character-Empire — Schémas de classes de l'Empire Pourpre
# Allumina MMO-ARPG — An 247 AO — MGE v1.0

[class.legionnaire_de_fer]
id = "legionnaire_de_fer"
faction = "empire_pourpre"
role = "tank"
weapon_types = ["sword_shield", "spear_shield"]
resource_primary = "discipline"
resource_secondary = "formation_bonus"
skill_trees = ["muraille_de_fer", "assaut_imperial", "commandement_de_ligne"]
signature_skills = ["bastille_imperiale", "coin_velanthorien"]
base_stats = { str = 28, agi = 12, int = 8, con = 30, dex = 14 }
armor_type = "heavy"
mobility = "low"
group_role = "front_line_anchor"
difficulty = "medium"
lore_summary = "Produit de vingt ans de caserne. Incarne la discipline de l'Empire. Pilier social et militaire."

[class.chevalier_velanthor]
id = "chevalier_velanthor"
faction = "empire_pourpre"
role = "tank_dps"
weapon_types = ["cavalry_lance", "longsword", "cavalry_lance_shield"]
resource_primary = "discipline"
resource_secondary = "elan"
skill_trees = ["charge_de_fer", "lame_velanthorienne", "banniere_de_l_aigle"]
signature_skills = ["charge_des_mille", "montee_fantome"]
base_stats = { str = 26, agi = 20, int = 10, con = 22, dex = 18 }
armor_type = "heavy"
mobility = "very_high_mounted"
group_role = "flanker_line_breaker"
difficulty = "high"
lore_summary = "Aristocratie militaire de l'Empire. Trois générations d'Ordre Équestre. Monté ou démonté, toujours à l'avant."

[class.ingenieur_de_siege]
id = "ingenieur_de_siege"
faction = "empire_pourpre"
role = "support_dps"
weapon_types = ["crossbow", "mechanical_devices", "wrench"]
resource_primary = "crystal_charges"
resource_secondary = "mechanical_energy"
skill_trees = ["arsenal_deployable", "artillerie_individuelle", "genie_de_soutien"]
signature_skills = ["presse_a_veine_portable", "canon_de_siege_urgence"]
base_stats = { str = 14, agi = 18, int = 28, con = 16, dex = 22 }
armor_type = "medium"
mobility = "medium"
group_role = "terrain_control_dps"
difficulty = "very_high"
lore_summary = "Corps des Machines Velanthor. Héritier de Loran Vaex. Transforme le terrain en avantage."

[class.stratege_imperial]
id = "stratege_imperial"
faction = "empire_pourpre"
role = "support"
weapon_types = ["command_staff", "tactical_scrolls", "officer_pistol"]
resource_primary = "command_points"
resource_secondary = "none"
skill_trees = ["commandement_de_bataille", "commandement_de_defense", "logistique_de_front"]
signature_skills = ["manoeuvre_velanthor", "execution_tactique"]
base_stats = { str = 10, agi = 16, int = 32, con = 14, dex = 18 }
armor_type = "light_medium"
mobility = "medium"
group_role = "universal_multiplier_commander"
difficulty = "extreme"
lore_summary = "Académie de Tactique de Velanthara. 40 diplômés par an. Commande en amplifiant, jamais en frappant."

[class.agent_du_bureau]
id = "agent_du_bureau"
faction = "empire_pourpre"
role = "dps"
weapon_types = ["dual_dagger", "wrist_crossbow", "poison_vials"]
resource_primary = "shadow"
resource_secondary = "exposure"
skill_trees = ["dossier_noir", "protocole_d_ombre", "chimie_militaire"]
signature_skills = ["dossier_scelle", "rapport_au_bureau"]
base_stats = { str = 16, agi = 30, int = 20, con = 12, dex = 28 }
armor_type = "light"
mobility = "very_high"
group_role = "assassin_intelligence"
difficulty = "high"
lore_summary = "Bras de l'Inquisiteur Général. Numéro de dossier, pas de grade. Le secret honteux de l'Empire."

[class.canonnier_imperial]
id = "canonnier_imperial"
faction = "empire_pourpre"
role = "dps"
weapon_types = ["shoulder_cannon", "field_mortar"]
resource_primary = "crystal_charges"
resource_secondary = "reload_cycle"
skill_trees = ["artillerie_lourde", "mortier_de_campagne", "munitions_speciales"]
signature_skills = ["bombardement_imperial", "tir_rasant"]
base_stats = { str = 22, agi = 14, int = 22, con = 18, dex = 20 }
armor_type = "medium_heavy"
mobility = "low"
group_role = "aoe_dps_siege"
difficulty = "high"
lore_summary = "Corps de l'Artillerie Impériale. Issu des régions minières. Tir décisif, vulnérabilité assumée."

[class.paladin_de_l_ordre_solaire]
id = "paladin_de_l_ordre_solaire"
faction = "empire_pourpre"
role = "tank_heal"
weapon_types = ["war_hammer_solar", "solar_shield"]
resource_primary = "solar_faith"
resource_secondary = "none"
skill_trees = ["lumiere_de_sorath", "bouclier_de_l_ordre", "auras_de_l_aigle"]
signature_skills = ["lumiere_d_aeternum", "sacrifice_consacre"]
base_stats = { str = 24, agi = 12, int = 24, con = 26, dex = 12 }
armor_type = "heavy"
mobility = "low"
group_role = "sustain_heal_tank"
difficulty = "medium"
lore_summary = "Ordre Solaire de Velanthara. Seule magie reconnue par l'Empire. Flamme éternelle et frontière philosophique."

[class.centurion_de_ligne]
id = "centurion_de_ligne"
faction = "empire_pourpre"
role = "dps"
weapon_types = ["dual_gladii", "gladius_light_armor", "pilum"]
resource_primary = "war_momentum"
resource_secondary = "none"
skill_trees = ["lame_jumelle", "mobilite_de_section", "commandement_de_section"]
signature_skills = ["decision_de_centurion", "exemple_du_rang"]
base_stats = { str = 22, agi = 26, int = 14, con = 18, dex = 24 }
armor_type = "medium"
mobility = "high"
group_role = "agile_dps_melee_leader"
difficulty = "medium_high"
lore_summary = "Officier de terrain. Commande 80 soldats. Elite : Centurion de Premier Rang à style reconnaissable."

[class.alchimiste_militaire]
id = "alchimiste_militaire"
faction = "empire_pourpre"
role = "dps_hybrid"
weapon_types = ["grenade_launcher", "alchemy_vials", "acid_whip"]
resource_primary = "alchemic_pressure"
resource_secondary = "reagents"
skill_trees = ["chimie_explosive", "chimie_corrosive", "alchimie_de_soutien"]
signature_skills = ["grand_oeuvre_militaire", "injection_de_combat"]
base_stats = { str = 16, agi = 22, int = 28, con = 16, dex = 18 }
armor_type = "medium_light"
mobility = "medium"
group_role = "chaos_aoe_debuffer"
difficulty = "very_high"
lore_summary = "Laboratoire de Campagne Impérial. Ligne floue entre chimie et magie. Surveillé par le Bureau."

# Ressources de faction partagées
[faction.empire_pourpre]
id = "empire_pourpre"
player_title = "citoyen"
rank_ladder = [
  "civis",
  "legionnaire",
  "centurion",
  "tribun",
  "legat",
  "prefet",
  "consul",
  "senateur",
  "grand_consul"
]
colors = { primary = "#8B0000", secondary = "#FFD700" }
symbol = "aigle_de_fer"
motto = "Velanthas Aeternum"
capital = "velanthara"
shared_resource = "discipline"
formation_mechanic = true
moral_mechanic = true
crystal_charges_classes = ["ingenieur_de_siege", "canonnier_imperial", "alchimiste_militaire"]
```

---

*AL-Character-Empire v1.0 — Référence canonique Allumina, An 247 AO*
*Corps des Machines Velanthor, Corps de l'Artillerie, Ordre Solaire, Académie de Tactique, Bureau des Phénomènes Anormaux, Ordre Équestre Velanthorien — propriété de l'Empire Pourpre.*
