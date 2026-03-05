# MiyuClicker â€” Zone citÃ© et bouton de construction (conceptuel)

## Contexte

Ce document dÃ©crit **conceptuellement** deux Ã©volutions de lâ€™Ã©cran Â« Ma citÃ©e Â» : le **bouton de construction** sur chaque carte bÃ¢timent (deuxiÃ¨me ligne) et la **zone de reprÃ©sentation de la citÃ©** (entre le header et la liste des bÃ¢timents / boutons de clic). Il prÃ©cise les rÃ´les, les Ã©tats et les comportements attendus, sans dÃ©tailler lâ€™implÃ©mentation technique.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** Bouton construction (conditions, Ã©tats visuel et fonctionnel, paiement, dÃ©marrage construction), barre de progression de construction, activation du bloc dâ€™allocation des maÃ§ons ; zone citÃ© (dimensions, ciel / sol, sprites personnages, dÃ©placements).
- **RÃ©fÃ©rence :** [MiyuClicker - Batiments Macons et Construction](MiyukiniClicker%20-%20Batiments%20Macons%20et%20Construction.md).

---

## 1. Bouton de construction (cartes bÃ¢timent)

### 1.1 RÃ´le

Sur chaque carte bÃ¢timent (Maison, Caserne, Grenier, DÃ©pÃ´t, EntrepÃ´t), un **bouton de construction** permet au joueur de **dÃ©marrer** ou de **lancer** la construction du prochain niveau (ou de la prochaine unitÃ©, pour les maisons). Le bouton est placÃ© en **deuxiÃ¨me ligne** sur la carte.

### 1.2 Conditions pour Â« level up Â»

Les conditions pour quâ€™un niveau (ou une maison) soit constructible sont :

- **Ressources suffisantes** : le joueur dispose au moins du coÃ»t en bois, pierre et fer dÃ©fini pour ce bÃ¢timent (voir [MiyuClicker - Batiments Macons et Construction](MiyukiniClicker%20-%20Batiments%20Macons%20et%20Construction.md)).
- Aucune autre condition mÃ©tier nâ€™est imposÃ©e ici (pas de prÃ©requis de niveau dâ€™autre bÃ¢timent, etc.).

Quand ces conditions sont **remplies**, le bouton est **actif** ; sinon il est **inactif**.

### 1.3 Ã‰tats visuels et fonctionnels du bouton

| Ã‰tat | Apparence | Comportement |
|------|-----------|--------------|
| **Conditions remplies** | Bouton **vert** | Clic possible. Au clic : le coÃ»t est prÃ©levÃ© et la construction **dÃ©marre** (la progression peut commencer Ã  sâ€™accumuler avec les maÃ§ons allouÃ©s). |
| **Conditions non remplies** | Bouton **blanc** | Clic **sans effet** (bouton dÃ©sactivÃ©). |

La couleur (vert / blanc) sert de **feedback immÃ©diat** sur la possibilitÃ© ou non de lancer la construction.

### 1.4 Action au clic (conditions remplies)

Lorsque le joueur clique sur le bouton alors que les conditions sont remplies :

1. Le **coÃ»t** (bois, pierre, fer) est **prÃ©levÃ©** immÃ©diatement.
2. La **construction est considÃ©rÃ©e comme dÃ©marrÃ©e** : la progression (pts de construction) pour ce bÃ¢timent peut dÃ©sormais avancer avec les maÃ§ons allouÃ©s (1 pt/jour par maÃ§on).
3. Le **bloc dâ€™allocation des maÃ§ons** pour ce bÃ¢timent devient **actif** : le joueur peut allouer des maÃ§ons depuis la pool de la Guilde des MaÃ§ons vers ce bÃ¢timent (boutons +/âˆ’ dÃ©jÃ  prÃ©sents sur la carte).

En rÃ©sumÃ© : **paiement â†’ dÃ©marrage construction â†’ allocation des maÃ§ons possible** pour faire avancer la barre.

### 1.5 Barre de progression de la construction

- Une **barre de chargement** (ou barre de progression) reprÃ©sente le **pourcentage de complÃ©tion** de la construction en cours pour ce bÃ¢timent.
- **Largeur recommandÃ©e** : 200 px (valeur cible pour la cohÃ©rence visuelle).
- La barre affiche : **progression actuelle / pts requis** (ou Ã©quivalent en %).
- Quand la progression atteint 100 %, le niveau (ou la maison) est **terminÃ©** : le bÃ¢timent est mis Ã  jour, la barre est rÃ©initialisÃ©e pour le prochain niveau, et les ressources ont dÃ©jÃ  Ã©tÃ© payÃ©es au moment du clic sur le bouton.

### 1.6 Relation avec lâ€™allocation des maÃ§ons

- **Avant** le premier clic sur le bouton de construction (pour ce bÃ¢timent), la construction nâ€™a pas Â« dÃ©marrÃ© Â» au sens interface : les maÃ§ons peuvent dÃ©jÃ  Ãªtre allouÃ©s mÃ©tier, mais lâ€™affichage peut considÃ©rer quâ€™il nâ€™y a pas encore de chantier en cours.
- **AprÃ¨s** le clic (coÃ»t payÃ©, construction dÃ©marrÃ©e), le **bloc dâ€™allocation des maÃ§ons** est **actif** : le joueur peut augmenter ou diminuer le nombre de maÃ§ons allouÃ©s Ã  ce bÃ¢timent (depuis la pool de la Guilde). Les maÃ§ons allouÃ©s font avancer la barre de 1 pt/jour chacun.

Conceptuellement, le bouton vert = Â« payer et dÃ©marrer le chantier Â» ; la barre 200 px = Â« avancement du chantier Â» ; les +/âˆ’ maÃ§ons = Â« affecter de la main dâ€™Å“uvre au chantier Â».

---

## 2. Zone de reprÃ©sentation de la citÃ©

### 2.1 Emplacement et dimensions

- **Position** : entre le **header** (barre du haut : ressources, horloge, vitesses, etc.) et la **liste des bÃ¢timents + boutons de clic**.
- **Largeur** : toute la **largeur** de la zone dâ€™affichage (responsive).
- **Hauteur** :
  - **20 %** de la hauteur dâ€™affichage ;
  - **Minimum 200 px** pour rester lisible quel que soit la rÃ©solution.

La zone est donc **responsive** en largeur et en hauteur (pourcentage avec plancher).

### 2.2 DÃ©coupage vertical : ciel et sol

La zone est coupÃ©e en **deux** parties verticales :

| Partie | Proportion en hauteur | RÃ´le |
|--------|------------------------|------|
| **Ciel** | **60 %** (partie haute) | Fond bleu clair ; pas dâ€™Ã©lÃ©ments interactifs ; dÃ©cor. |
| **Sol** | **40 %** (partie basse) | Fond marron clair / beige ; **zone de dÃ©placement** des personnages (sprites). |

Limite ciel/sol = une ligne horizontale Ã  60 % de la hauteur de la zone.

### 2.3 Couleurs de fond

| Zone | Couleur |
|------|---------|
| Ciel | **Bleu clair** |
| Sol | **Marron clair / beige** |

Les valeurs exactes (hex, RGB) seront dÃ©finies dans le guide dâ€™implÃ©mentation ou le design system.

### 2.4 ReprÃ©sentation des personnages (sprites)

- Chaque **personne** (unitÃ© de population) est reprÃ©sentÃ©e par un **sprite** de **3 px de haut** et **1 px de large** (forme verticale 1Ã—3).
- **Trois types** de personnages (donc trois variantes de sprite) :
  - **Gens** (population civile) ;
  - **Soldat** ;
  - **MaÃ§on**.

Structure commune du sprite (3 pixels de haut) :

| Pixel | Position | Couleur | Signification |
|-------|-----------|---------|----------------|
| 1 | Le plus **haut** | **Blanc** | TÃªte (commun Ã  tous les types). |
| 2â€“3 | En **bas** | Selon le type | Corps. |

Couleur du **corps** selon le type :

| Type | Couleur corps |
|------|----------------|
| Gens | **Vert** |
| Soldat | **Rouge** |
| MaÃ§on | **Marron foncÃ©** |

Donc : **1 pixel blanc (tÃªte) + 2 pixels corps (vert / rouge / marron)** = 3 px de haut, 1 px de large.

Le nombre de sprites affichÃ©s reflÃ¨te les effectifs actuels : **gens**, **soldats**, **maÃ§ons** (par exemple, un sprite par unitÃ©, ou un Ã©chantillonnage si le nombre est trÃ¨s grand â€” Ã  prÃ©ciser en implÃ©mentation).

### 2.5 DÃ©placement des personnages

- Les personnages (sprites) se **dÃ©placent** dans la zone **sol** uniquement (les 40 % du bas).
- Le dÃ©placement est **alÃ©atoire** : direction et/ou vitesse peuvent Ãªtre tirÃ©es alÃ©atoirement, avec contraintes pour rester dans les limites du sol (pas de sortie de zone).
- Aucune collision obligatoire entre personnages dans ce concept ; lâ€™objectif est un mouvement de vie simple pour donner lâ€™impression dâ€™une citÃ© habitÃ©e.

---

## 3. SynthÃ¨se des concepts

| Ã‰lÃ©ment | Concept clÃ© |
|--------|-------------|
| **Bouton construction** | Vert = conditions OK, clic = payer + dÃ©marrer construction ; blanc = inactif. |
| **Barre construction** | 200 px, % complÃ©tion ; avance avec les maÃ§ons allouÃ©s. |
| **Bloc maÃ§ons** | Actif une fois la construction dÃ©marrÃ©e ; allocation depuis la pool Guilde. |
| **Zone citÃ©** | 20 % hauteur (min 200 px), pleine largeur ; ciel 60 % bleu, sol 40 % beige. |
| **Sprites** | 3Ã—1 px ; tÃªte blanche ; corps vert / rouge / marron (gens / soldat / maÃ§on). |
| **Mouvement** | AlÃ©atoire dans la zone sol uniquement. |

---

## 4. RÃ©fÃ©rences

- [MiyuClicker - Batiments Macons et Construction](MiyukiniClicker%20-%20Batiments%20Macons%20et%20Construction.md) â€” CoÃ»ts, pts de construction, maÃ§ons.
- [MiyuClicker - Guide Implementation Zone Cite et Construction](MiyukiniClicker%20-%20Guide%20Implementation%20Zone%20Cite%20et%20Construction.md) â€” Guide dâ€™implÃ©mentation technique (bouton construction, zone citÃ©, sprites, mouvement).
- [MiyuClicker - MVP Ecrans et Mecaniques](MiyukiniClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md) â€” Ã‰cran Ma citÃ©e.

