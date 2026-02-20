# Allumina — Caractéristiques, Aptitudes de combat et Compétences

## Contexte

Ce document définit le **système de personnage** d'Allumina : les **caractéristiques** (stats de base), les **aptitudes de combat** (jets, parade, esquive, tirs, magie) et les **compétences** (sauts, fouille, marchandage, etc.). Il complète le [Document Conceptuel](./Allumina%20-%20Document%20Conceptuel.md) pour tout ce qui touche à la progression et aux jets de dés / résolutions d'actions.

## Portée / Scope

- **Applicable à :** Game design des stats, combat, compétences hors combat et prérequis (équipement, monture, machines).
- **Audience :** Game design, équipes produit, implémentation.
- **Statut :** Spécification conceptuelle — formules et plafonds à valider en jeu.

### Hors périmètre

- Liste exhaustive des sorts, armes et équipements (prérequis Force/Agilité/etc. restent définis ici).
- Courbes de régen PV / PM / End (à fixer en tuning).

---

## 1. Nomenclature (codes)

Les **codes** ci-dessous sont utilisés dans les formules et l’implémentation. À utiliser de façon cohérente (scripts, UI tooltips, logs).

### 1.1 Caractéristiques

| Code | Caractéristique |
|------|------------------|
| For | Force |
| Con | Constitution |
| Agi | Agilité |
| Dex | Dextérité |
| Per | Perception |
| Vol | Volonté |
| Int | Intelligence |
| Sag | Sagesse |
| Cha | Charisme |
| Luk | Chance (Luck) |

### 1.2 Statistiques dérivées et combat

| Code | Signification |
|------|----------------|
| PV | Points de vie |
| PM | Points de mana |
| End | Endurance |
| Pds max | Poids max (poids maximal portable sans malus) |
| Aggro | Priorité de ciblage par l’ennemi |
| atk | Attaque (corps à corps) |
| atk speed | Vitesse d’attaque |
| par | Parade |
| esq | Esquive |
| critik | Attaque critique (blessure assurée ; dégâts 150 % + mod moins armure) |
| Cmd | Commandement (compétence) — pool pts troupes, prérequis et coût par type (voir 11.9). |
| tirC | Tir corde (arc, fronde) |
| tirP | Tir poing (pistolet) |
| tirE | Tir épaule (arbalète, arquebuse, fusil) |
| tir speed | Vitesse de tir |
| cast speed | Vitesse d’incantation (réduction du temps d’incantation en %) |

### 1.3 Types de dégâts et armure

| Code | Signification |
|------|----------------|
| Tranc | Tranchant (opposé : ARt) |
| Cont | Contondant (opposé : ARc) |
| Perc | Perforant (opposé : ARp) |
| ARt | Armure tranchante |
| ARc | Armure contondante |
| ARp | Armure perforante |

### 1.4 Échelles et abréviations

| Terme | Plage (base) | Usage |
|-------|----------------|-------|
| **carac** | 1 à 10 | Caractéristiques (For, Con, Agi, etc.) |
| **Apt** | 1 à 100 | Aptitudes de combat (atk, par, esq, tirC, etc.) |
| **Comp** | 1 à 100 | Compétences (Saut, Fouille, Marchandage, etc.) |

---

## 2. Caractéristiques

Les **caractéristiques** sont les stats de base du personnage. Elles déterminent les PV, PM, End, les prérequis d’équipement, les plafonds des compétences et les aptitudes de combat (voir sections 3 et 4).

| Caractéristique | Code | Effets principaux | Prérequis / usage |
|-----------------|------|-------------------|-------------------|
| **Force** | For | Dégâts CàC ; prérequis équipement lourd ; Pds max, End. | Seuils par type d’équipement. |
| **Constitution** | Con | PV ; End ; Aggro. | Base PV, End, priorité ciblage. |
| **Agilité** | Agi | atk speed ; vitesse de déplacement ; esq. | Vitesse d’attaque, esquive, mobilité. |
| **Dextérité** | Dex | Précision des gestes ; visée ; atk (toucher). | Toucher mêlée, crochetage, pièges. |
| **Perception** | Per | Détection ; tirs (visée). | Portée détection, fouille, tirC/tirP/tirE. |
| **Volonté** | Vol | Résistance mentale ; réserve mana ; puissance magique. | Pool mana, résistance effets mentaux. |
| **Intelligence** | Int | Réflexion ; énigmes ; magie (base). | Raisonnement, cast speed. |
| **Sagesse** | Sag | Quantité de sortilèges ; connaissance ; soutien ; cast speed. | Slots sorts, soins, réduction incantation. |
| **Charisme** | Cha | Roleplay ; commerce ; influence sociale ; **cap de troupe**. | Dialogues, marchandage, commandement. Le cap de troupe (groupe, compagnie, etc.) est plafonné par Charisme × multiplicateur selon l'échelle ; une seconde limite (statut social) s'applique au début (voir [Combat et Troupes](./Allumina%20-%20Combat%20et%20Troupes.md)). |
| **Chance** | Luk | Seuil critique (jet ≤ Luk+mod) ; résultat effectif = dé − Luk ; cap non-réussite −1 %/pt Luk. | Critik (blessure assurée, 150 %+mod moins armure) ; critique compétence = test réussi ; max réussite 85 %+Luk (voir section 5.3). |

---

## 3. Statistiques de personnage (à la création)

Les statistiques dérivées sont calculées à la création du personnage à partir des caractéristiques. Les formules ci-dessous sont normatives.

| Statistique | Formule | Description |
|-------------|---------|--------------|
| **PV max** | (For + Con) × 10 | Points de vie maximum. |
| **PM max** | (Int + Sag) × 10 | Points de mana maximum. |
| **End max** | (For + Con × 2) × 10 | Endurance maximum (pool). |
| **Aggro** | Con + For | Priorité de ciblage par l’ennemi (plus élevé = plus ciblé). |
| **Pds max** | (For + Con) × 10 | Poids maximal portable sans malus (unités à définir). |

**Échelles normatives :** les **caractéristiques (carac)** vont de **1 à 10** en base. Les **aptitudes (Apt)** et les **compétences (Comp)** vont de **1 à 100** en base (voir section 1.4).

---

## 4. Aptitudes de combat

Les **aptitudes de combat** représentent la maîtrise dans un type d’action offensive ou défensive. Elles sont utilisées pour les jets de toucher, parade, esquive et dégâts.

### 4.1 Description par aptitude

| Aptitude | Code | Description | Caractéristique(s) typique(s) |
|----------|------|-------------|-------------------------------|
| **Attaque (corps à corps)** | atk | Toucher et dégâts en mêlée. | For (dégâts), Dex (toucher). |
| **Vitesse d’attaque** | atk speed | Nombre d’attaques par unité de temps. | Agi. |
| **Jet** | — | Lancer d’objets (couteaux, haches de jet). | For. |
| **Parade** | par | Bloquer ou dévier une attaque. | For, Con. |
| **Esquive** | esq | Éviter une attaque sans blocage. | Agi. |
| **Tir corde** | tirC | Arc, fronde. | For, Per. |
| **Tir poing** | tirP | Pistolet, arme de poing. | Agi, Per. |
| **Tir épaule** | tirE | Arbalète, arquebuse, fusil. | For, Per. |
| **Vitesse de tir** | tir speed | Cadence de tir (à lier aux aptitudes tirC/tirP/tirE). | Agi ou Dex selon design. |
| **Magie** | — | Lancement de sorts. | Int, Sag. |
| **Vitesse d’incantation** | cast speed | Réduction du temps d’incantation (en %). | Int, Sag. |

### 4.2 Valeurs de base à la création (aptitude minimale)

À la création du personnage, les aptitudes de combat ont une **valeur minimale de base** donnée par les formules ci-dessous (en utilisant les codes des caractéristiques).

| Aptitude | Formule de base |
|----------|------------------|
| **atk** | Dex × 10 |
| **atk speed** | Agi × 10 |
| **Jet** | For × 10 |
| **par** | (For + Con) / 2 × 10 |
| **esq** | Agi × 10 |
| **tirC** | (For/2 + Per) × 10 |
| **tirP** | (Agi + Per) × 10 |
| **tirE** | (For + Per) × 10 |
| **tir speed** | *Idem échelle que atk speed ou dérivé des aptitudes de tir.* |
| **Magie** | (Int + Sag) / 2 |
| **cast speed** | Réduction en % = (Int + Sag) / 2 |

*L’échelle de « Magie » (sans ×10) peut être 1–10 si Int et Sag sont en 1–10 ; à aligner avec le système de sorts.*

---

## 5. Schéma de calcul de base (oppositions)

### 5.1 Règle générale

Lorsqu'une **aptitude ou une compétence** est opposée à une autre (attaquant vs défenseur, détection vs camouflage, etc.), la **chance de succès** de la valeur la plus élevée est :

- **Base 50 %** si les deux valeurs sont **égales**.
- **+1 % par point d'écart** en faveur du plus élevé.  
  *Ex. : A 30 vs B 20 → A a 50 + (30−20) = **60 %** de succès ; B a 40 %.*

Formule : *Chance de succès (celui qui a la valeur la plus haute) = 50 + (Valeur_haute − Valeur_basse)*, en %. Les bornes (0 % et 100 %) sont à appliquer si l'écart dépasse 50.

### 5.2 Opposition explicite vs même vs même

| Cas | Exemple | Résolution |
|-----|---------|------------|
| **Opposition explicite** | Détection vs Discrétion (camouflage / furtivité) | Jet opposé : valeur A vs valeur B avec la règle 50 % + 1 % / point. |
| **Pas d'aptitude/compétence expressément opposée** | Marchandage (vendeur vs acheteur) | Même stat des deux côtés : **Marchandage vs Marchandage** (ou compétence identique). Même formule 50 % + 1 % par point d'écart. |

### 5.3 La Chance (Luk) — synthèse

#### Seuil de réussite critique

- La **Luk** fixe le **seuil de réussite critique**, toujours valable : *Seuil = Luk + modificateur* (équipement, buffs, etc.).
- Sur un **jet d100** (attaque, compétence, etc.), si le **résultat naturel** est **≤ seuil** → **réussite critique**.  
  *Ex. : Luk 5, modificateur 0 → seuil 5 ; un jet naturel de 2 sur une atk = réussite critique.*

#### Effets de la réussite critique

- **Sur une attaque** → **critik** : **blessure assurée** (touche sans opposition esq/par ou règle spécifique). Dégâts = **150 % + modificateur** des dégâts de l'attaque, **moins l'armure** (ARt / ARc / ARp) de l'adversaire.
- **Sur une compétence** : une réussite critique sur un test de **compétence** **fait passer automatiquement le test** (réussite assurée).

#### Cap de non-réussite et échec critique

- **Cap d'échec critique** : fixé à **15 %** de base. Avant tout calcul de chance, une action a donc **au maximum 85 % de chance de réussir** (15 % restent réservés à l'échec / échec critique).
- **Luk diminue le cap de non-réussite** de **1 % par point de Luk**. La part « non-réussite » (échec + échec critique) est donc réduite, ce qui **augmente le plafond de réussite** : *max réussite = 85 % + Luk* (plafonné à 100 %).  
  *Ex. : Luk 10 → cap de non-réussite = 15 − 10 = 5 % → max réussite = 95 %.*

#### Luk sur tous les jets

- La Luk **influe sur tous les jets** en **diminuant le score du dé** du personnage qui fait le jet :  
  **Résultat effectif du jet = résultat naturel du dé − Luk** (+ modificateurs éventuels sur le bonus Luk).  
  Ainsi, plus la Luk est élevée, plus le résultat effectif est favorable (en système « plus bas = mieux » ou après comparaison au seuil).  
  *Ex. : un jet de Détection fait 47 (naturel) ; le personnage a Luk 5 → résultat effectif 47 − 5 = 42.*

- **Les bonus de Luk au jet peuvent avoir des modificateurs** (équipement, sorts, effets) : la valeur soustraite au dé peut être *Luk + modificateur* au lieu de Luk seul (à préciser par type d'action ou d'effet).

---

## 6. Séquence d'attaque (mêlée)

Pour une **attaque au corps à corps** :

1. **Phase 1 — Toucher (atk vs esq)**  
   Jet : **atk (assaillant)** opposé à **esq (défenseur)**.  
   - Si l'attaquant **échoue** (esq l'emporte) → **l'attaque échoue** (pas de dégât, pas de parade).  
   - Si l'attaquant **réussit** → on passe à la phase 2.

2. **Phase 2 — Parade (atk vs par)**  
   Jet : **atk (assaillant)** opposé à **par (défenseur)**.  
   - Si le défenseur **pare** → les dégâts sont appliqués à la **résistance du bouclier ou de l'arme** (parade) ; cette résistance diminue du nombre de points de dégâts ainsi absorbés. La cible ne perd pas de PV.  
   - Si le défenseur **ne pare pas** → l'attaquant inflige les dégâts à la cible (PV), après réduction d'armure (voir section 7).

*Exemple : atk 30 vs esq 20 → 60 % de toucher. Si touché : atk 30 vs par 45 → 65 % de parer l'attaque (défenseur pare dans 65 % des cas).*

### 6.1 Ciblage des ennemis et comportement des alliés

- **Ciblage des ennemis / monstres / animaux agressifs** : ils attaquent le **personnage**, les **troupes** ou les **animaux dressés** qui sont dans leur **champ de vision**, en prenant en priorité la **cible la plus proche**, et en **priorisant les cibles hors combat ou seules** (une cible qui n'a pas encore d'ennemis sur elle, ou qui est isolée). Jusqu'à un **cumul de 4 ennemis** autour d'une même cible ; les ennemis supplémentaires **se dirigent vers une autre cible proche** (répartition selon distance et priorité hors combat / seule).
- **Comportement à l'engagement** :  
  - **Ennemis, monstres et animaux agressifs** : attaquent **à vue** (dès qu'une cible valide est dans leur champ de vision).  
  - **Animaux dressés** : n'attaquent **qu'en cas d'agression** (lorsqu'eux-mêmes ou le personnage / les alliés sont attaqués).  
  - **Troupes** : se comportent **comme les animaux dressés** — attaque en cas d'agression uniquement (pas d'attaque à vue).

### 6.2 Balise et contrôle des suivants

- **Balise (clic droit)** : le joueur peut faire un **clic droit pour baliser un lieu**. Tous les **suivants** (troupes et animaux dressés) **se dirigent vers cette balise** ; ils **peuvent rompre un combat** en cours pour obéir à l'ordre. Cela permet au joueur de **contrôler** le déplacement de ses suivants.
- **À l'arrivée à la balise** : les suivants **reprennent leur comportement normal** : **suivre le joueur** jusqu'à ce qu'un **ennemi entre dans leur champ de vision** (puis application des règles de ciblage et d'engagement : attaque en cas d'agression pour troupes/animaux dressés).
- **Animaux dressés passifs** : les **animaux dressés passifs** (ceux qui ne se battent pas et apportent un bonus passif) **ne sont jamais pris pour cible** — ni par les ennemis, ni par les suivants. Leur **représentation en jeu est purement esthétique** (présence visuelle, pas d'entité de combat).

### 6.3 PNJ alliés

- **Comportement** : les **PNJ alliés** se comportent **comme les suivants** (troupes / animaux dressés) pour le combat et le ciblage — attaque en cas d'agression, pas d'attaque à vue, mêmes règles de cumul (4 ennemis par cible), etc. **Ils ne suivent pas le joueur** sauf si le **scénario** le prévoit explicitement.
- **Déplacement** : ils ont une **balise (invisible)** qui définit leur mission de déplacement :
  - **Garder un lieu** : rester sur place (ou autour de la balise) et engager les ennemis à portée / en champ de vision selon les règles.
  - **Aller d'un point A à un point B** : se déplacer de A vers B et **attaquer les ennemis sur le chemin** (ceux dans leur champ de vision ou qui les agressent).

---

## 7. Types de dégâts et armure

### 7.1 Trois types de dégâts

| Type | Code | Opposé (armure) |
|------|------|------------------|
| **Tranchant** | Tranc | ARt (armure tranchante) |
| **Contondant** | Cont | ARc (armure contondante) |
| **Perforant** | Perc | ARp (armure perforante) |

Chaque attaque (ou arme) est typée (Tranc, Cont, Perc ou combinaison). L'armure réduit les dégâts **en %** selon le type correspondant.

### 7.2 Réduction des dégâts par l'armure

- L'armure (par pièce ou globale) possède un **% de réduction** par type (ex. 10 % Tranc).
- **Dégâts infligés aux PV** = dégâts bruts × (1 − réduction % du type).  
  *Ex. : 10 pts Tranc face à 10 % armure Tranc → 9 pts de dégâts aux PV.*
- **Résistance des pièces** : chaque pièce d'armure (ou bouclier/arme en parade) a une **résistance** (points). À chaque attaque encaissée (touchant la cible ou la parade), la résistance de la pièce concernée **diminue de 1 pt par point de dégât** reçu (ou absorbé). Quand la résistance atteint 0, la pièce ne protège plus (ou plus de parade) jusqu'à réparation / règles spécifiques.

---

## 8. Poids et surcharge (Pds max)

- **Pds max** : poids maximal que le personnage peut porter **sans malus** (formule : (For + Con) × 5).
- **Surcharge** : si le **poids total porté** dépasse **Pds max**, le personnage subit une consommation d’**Endurance** continue : il consomme un montant d’Endurance par seconde égal au **dépassement** (poids porté − Pds max), en unités à définir (ex. End/sec = excédent de poids).
- La marche normale ne consomme pas d’End ; la surcharge seule ajoute cette consommation.

---

## 9. Endurance

- **Pool** : End max = (For + Con × 2) × 10 (voir section 3).
- **Consommation** : certaines actions consomment de l’Endurance (courir, grimper, sauter, nager, acrobatie). La consommation est en **End par seconde** ; le taux peut dépendre de la Con ou être fixe par type d’action (à préciser en tuning). **Marcher ne consomme pas d’End.**
- **Malus à 0 End** : tant que l’Endurance n’est pas revenue à son maximum, **toutes les aptitudes et compétences de type « vitesse / speed »** (atk speed, tir speed, cast speed, vitesses de déplacement, etc.) sont **divisées par 2**. Le malus disparaît lorsque End max est à nouveau complète.
- **Régen et effets** : la vitesse de consommation et de regain peut être modifiée par des effets (buffs, équipement, états). Détails à définir en tuning.

---

## 10. Compétences — Règle de plafond et minimum

Les **compétences** sont des capacités spécifiques (saut, fouille, marchandage, etc.). Chaque compétence est rattachée à **une caractéristique** et obéit aux règles suivantes :

- **Plafond (cap)** : la compétence ne peut pas dépasser **Caractéristique × 10 + 20**.  
  Ex. : Force 1 → compétences Force plafonnées à 30 ; Force 5 → plafond 70.
- **Minimum** : la valeur minimale de la compétence est **Caractéristique / 2*10**.  
  Ex. : Force 2 → « Saut » ne peut pas être en dessous de 10.

En résumé : *[Caractéristique × 10] ≤ Compétence ≤ [Caractéristique × 10 + 20]*.

---

## 11. Compétences par caractéristique

### 11.1 Force

| Compétence | Effet |
|------------|--------|
| **Saut** | Distance de saut en pixels (ou en cases) ; franchissement d'obstacles. |
| **Natation** | Vitesse de déplacement dans l'eau ; réduction de la pénalité en milieu aquatique. |

---

### 11.2 Constitution

| Compétence | Effet |
|------------|--------|
| **Athlétisme** | Vitesse de déplacement au sol (course, marche rapide) ; peut influencer endurance en déplacement. |

---

### 11.3 Agilité

| Compétence | Effet |
|------------|--------|
| **Acrobatie** | Débloque certains lieux et actions (passages étroits, figures) ; équilibre, réception. |
| **Escalade** | Vitesse de grimper ; vitesse de consommation d'endurance en escalade. |
| **Discrétion** | Niveau de furtivité ; opposition aux checks de Détection / Vigilance. |
| **Larcin** | Chance de réussite pour voler une personne (pickpocket). |
| **Danse** | Chance d'attirer l'attention ; bonus en affinité avec des PNJ (social). |
| **Equitation** | Prérequis pour monter certaines montures ; qualité de contrôle en selle. |

---

### 11.4 Dextérité

| Compétence | Effet |
|------------|--------|
| **Crochetage** | Chance d'ouverture des serrures (coffres, portes, etc.). |
| **Pièges** | Chance de détecter les pièges ; prérequis pour en installer et les désarmer. |

---

### 11.5 Perception

| Compétence | Effet |
|------------|--------|
| **Détection** | Permet de détecter les ennemis camouflés ou en discrétion. Opposition explicite : Détection vs Discrétion (voir section 5.2). |
| **Pistage** | Informations sur le passage des PNJ / créatures (traces, direction, récence). |
| **Fouille** | Chance de trouver des choses intéressantes dans un conteneur ou une zone. |
| **Musique** | Prérequis pour jouer de certains instruments ; qualité de jeu (effets buff / social). |
| **Vigilance** | Permet de détecter des ennemis de plus loin ou ceux en furtivité. |

---

### 11.6 Volonté

Aucune **compétence** distincte listée : la Volonté gouverne la **réserve de mana**, la **résistance mentale** et l'**aptitude Magie**. Une compétence optionnelle (ex. **Concentration**, **Résistance mentale**) pourra être ajoutée plus tard si le design le requiert.

---

### 11.7 Intelligence

| Compétence | Effet |
|------------|--------|
| **Orientation** | Mise à jour de la carte en se déplaçant ; moins de zones « brouillard de guerre ». |
| **Raisonnement** | Capacité à résoudre des énigmes ; peut signaler au joueur la bonne réponse ou réduire la difficulté. |

---

### 11.8 Sagesse

| Compétence | Effet |
|------------|--------|
| **Médecine** | Soigner les PNJ ou les autres joueurs ; qualité des soins hors combat ou via objets. |
| **Mécanique** | Connaissance des recettes, catalogue de craft ; taux de réussite modifié par Sag + Int ; composants, produits intermédiaires, outils (voir ci-dessous). |
| **Herboristerie** | Préparation de soins, remèdes et poisons (consommables). |
| **Apprentissage** | Prérequis et vitesse pour lire certains livres ; gain de compétences / aptitudes via lecture. |
| **Science** | Quantité de machines déployables (tours, pièges automatiques, etc.). |

#### Fabrication (craft)

- **Définition** : la fabrication d'un objet est définie par la **connaissance des techniques** de fabrication de cet objet. Le personnage peut **apprendre** une recette auprès d'un **maître**, dans des **livres** ou via des **événements**. Une fois acquise, la recette est **répertoriée dans son catalogue de craft**, classé par **catégorie**.
- **Taux de réussite** : chaque recette a un **taux de réussite de base** (en %), puis **modifié par Sagesse et Intelligence**.  
  *Ex. : une épée — taux de base 35 % ; taux effectif = 35 % + Sag + Int* (formule exacte à préciser : Sag+Int en points, ou en %, selon l'échelle).
- **Composants** : le craft requiert des **composants** (bois, fer, etc.) et/ou des **produits intermédiaires** (hampe, pommeau, pointe de flèche, etc.) selon la recette.
- **Outils** : le craft requiert des **outils appropriés** (forge, marteau de forge, ciseau à bois, tour de menuiserie, etc.) ; l'absence d'outil peut bloquer ou pénaliser la tentative.

---

### 11.9 Charisme

| Compétence | Effet |
|------------|--------|
| **Bluff** | Plus de choix avec les PNJ ; options de dialogue basées sur la tromperie. |
| **Rhétorique** | Meilleurs choix avec les PNJ ; arguments convaincants. |
| **Masque** | Réduit les mauvais choix et leur impact (réputation, conséquences). |
| **Marchandage** | Influence sur prix de vente et d'achat : 1 % par pt d'écart (perso vs marchand), max 50 % ; modificateurs possibles au-delà (voir ci-dessous). |
| **Commandement (Cmd)** | Cap du nombre de troupes (pool en pts Cmd) ; prérequis et coût en pts par type de troupe (voir ci-dessous). |
| **Persuasion** | Plus de chances de réponse favorable ; débloque certains choix de dialogue. |
| **Dressage** | Dresser des animaux (1 tentative/jour) ; cap = dizaine de Dressage ; animaux évoluent, vente possible ; certains bonus passifs (voir ci-dessous). |

#### Marchandage

- **Effet** : le Marchandage permet d'**influer sur les prix de vente et les prix d'achat** en fonction de l'**écart** entre la valeur de Marchandage du **personnage** et celle du **marchand** (ou niveau équivalent du PNJ).
- **Formule** : **1 % par point d'écart**, dans la limite de **50 % d'écart max** (avant modificateurs).  
  - Si **personnage > marchand** → prix plus favorables pour le personnage (achat moins cher, vente plus chère).  
  - Si **personnage < marchand** → prix moins favorables (achat plus cher, vente moins chère).  
  - *Ex. : personnage 30, marchand 45 → écart 15 pts en faveur du marchand → prix **15 % plus chers** pour le personnage à l'achat (et réciproquement 15 % moins bons à la vente). Inversement : personnage 45, marchand 30 → prix 15 % plus avantageux pour le personnage.*
- **Modificateurs** : des **modificateurs** (réputation, quête, objet, etc.) peuvent s'appliquer **après** ce calcul et **permettre de dépasser la limite des 50 %**.

#### Commandement (Cmd) — troupes

- **Cmd** (valeur de la compétence Commandement) forme un **pool de points** : le personnage ne peut pas avoir avec lui des troupes dont le **total des coûts** dépasse **Cmd**.
- Chaque **type de troupe** a :
  - un **prérequis Cmd** : Cmd du personnage ≥ cette valeur pour pouvoir recruter ce type ;
  - un **coût en pts Cmd** : nombre de points que chaque unité de cette troupe occupe dans le pool.
- **Règle** : *Somme (coût × nombre d'unités) pour toutes les troupes accompagnant le personnage ≤ Cmd*, et pour chaque type recruté, *Cmd ≥ prérequis de ce type*.

*Ex. : Miliciens — prérequis 20 Cmd, coût 5 pts par unité. Avec Cmd 25 : 25 pts disponibles → au plus 5 miliciens (5×5 = 25). Ou 3 miliciens (15 pts) + 1 garde (prérequis 25 Cmd, coût 10 pts) = 25 pts, possible si Cmd ≥ 25.*

#### Dressage — animaux

- **Tentative de dressage** : le personnage peut tenter de **dresser un animal 1 fois par jour** (in-game). En cas de **réussite**, l'animal appartient au personnage.
- **Opposition** : le test de Dressage est **opposé à la caractéristique de l'animal × 10** (caractéristique à définir par type d'animal : sauvagerie, volonté, etc.). Règle d'opposition : 50 % + 1 % par point d'écart (section 5.1).
- **Cap du nombre d'animaux** : le personnage peut avoir avec lui **autant d'animaux que sa dizaine de Dressage** (partie entière de Dressage ÷ 10).  
  *Ex. : Dressage 37 → 3 animaux max ; Dressage 9 → 0 animal (il faut au moins 10 pour en avoir 1).*
- **Progression** : les animaux **gagnent de l'XP** et **évoluent** avec le joueur.
- **Vente** : le joueur peut **vendre** ses animaux.
- **Animaux dressés passifs** : certains animaux **ne se battent pas** et apportent un **bonus passif** au personnage (et éventuellement à son équipe). Ils **ne sont jamais pris pour cible** ; leur représentation en jeu est **purement esthétique** (voir section 6.2).

---

### 11.10 Chance

La **Chance (Luk)** n'a pas de compétence dédiée. Elle fixe le **seuil de réussite critique** (jet ≤ Luk+mod), réduit le **cap de non-réussite** (1 % par point de Luk, base 15 % échec) et **soustrait Luk du résultat du dé** sur tous les jets (résultat effectif = dé − Luk, modificateurs possibles). Réussite critique sur attaque = **critik** (blessure assurée, 150 %+mod moins armure) ; sur compétence = **test réussi automatiquement**. Voir section 5.3.

---

## 12. Synthèse des liens caractéristique → compétences

| Caractéristique | Compétences |
|-----------------|--------------|
| Force | Saut, Natation |
| Constitution | Athlétisme |
| Agilité | Acrobatie, Escalade, Discrétion, Larcin, Danse, Equitation |
| Dextérité | Crochetage, Pièges |
| Perception | Détection, Pistage, Fouille, Musique, Vigilance |
| Volonté | — (mana, magie, résistance mentale) |
| Intelligence | Orientation, Raisonnement |
| Sagesse | Médecine, Mécanique, Herboristerie, Apprentissage, Science |
| Charisme | Bluff, Rhétorique, Masque, Marchandage, Commandement, Persuasion, Dressage |
| Chance | Modificateur global ; pas de compétence. |

---

## 13. À préciser ultérieurement

- **Échelle numérique** : valeur min/max des caractéristiques (ex. 1–10) et des compétences (ex. 0–100 ou 10–120 selon cap).
- **Modificateur de critique** : valeur et sources du modificateur ajouté à Luk pour le seuil critik (équipement, buffs).
- **Prérequis équipement** : table For minimale par armure/arme lourde.
- **Coût de progression** : coût en points ou en XP pour monter une caractéristique vs une compétence.
- **Volonté** : ajout éventuel d’une compétence (Concentration, Résistance mentale) si le design le demande.

---

## 14. Références

| Document | Rôle |
|----------|------|
| [Allumina - Document Conceptuel](./Allumina%20-%20Document%20Conceptuel.md) | Vision jeu, personnage, combat, progression. |
| [Allumina - Document Fondateur](../Allumina%20-%20Document%20Fondateur.md) | Vision service, stack technique. |

---

**Document** : Allumina — Caractéristiques, Aptitudes de combat et Compétences  
**Version** : 1.0  
**Date** : 2026-02-17  
**Statut** : Spécification conceptuelle game design
