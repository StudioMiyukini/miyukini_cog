# MiyuClicker — Zone cité et bouton de construction (conceptuel)

## Contexte

Ce document décrit **conceptuellement** deux évolutions de l’écran « Ma citée » : le **bouton de construction** sur chaque carte bâtiment (deuxième ligne) et la **zone de représentation de la cité** (entre le header et la liste des bâtiments / boutons de clic). Il précise les rôles, les états et les comportements attendus, sans détailler l’implémentation technique.

## Portée / Scope

- **Périmètre :** Bouton construction (conditions, états visuel et fonctionnel, paiement, démarrage construction), barre de progression de construction, activation du bloc d’allocation des maçons ; zone cité (dimensions, ciel / sol, sprites personnages, déplacements).
- **Référence :** [MiyuClicker - Batiments Macons et Construction](MiyuClicker%20-%20Batiments%20Macons%20et%20Construction.md).

---

## 1. Bouton de construction (cartes bâtiment)

### 1.1 Rôle

Sur chaque carte bâtiment (Maison, Caserne, Grenier, Dépôt, Entrepôt), un **bouton de construction** permet au joueur de **démarrer** ou de **lancer** la construction du prochain niveau (ou de la prochaine unité, pour les maisons). Le bouton est placé en **deuxième ligne** sur la carte.

### 1.2 Conditions pour « level up »

Les conditions pour qu’un niveau (ou une maison) soit constructible sont :

- **Ressources suffisantes** : le joueur dispose au moins du coût en bois, pierre et fer défini pour ce bâtiment (voir [MiyuClicker - Batiments Macons et Construction](MiyuClicker%20-%20Batiments%20Macons%20et%20Construction.md)).
- Aucune autre condition métier n’est imposée ici (pas de prérequis de niveau d’autre bâtiment, etc.).

Quand ces conditions sont **remplies**, le bouton est **actif** ; sinon il est **inactif**.

### 1.3 États visuels et fonctionnels du bouton

| État | Apparence | Comportement |
|------|-----------|--------------|
| **Conditions remplies** | Bouton **vert** | Clic possible. Au clic : le coût est prélevé et la construction **démarre** (la progression peut commencer à s’accumuler avec les maçons alloués). |
| **Conditions non remplies** | Bouton **blanc** | Clic **sans effet** (bouton désactivé). |

La couleur (vert / blanc) sert de **feedback immédiat** sur la possibilité ou non de lancer la construction.

### 1.4 Action au clic (conditions remplies)

Lorsque le joueur clique sur le bouton alors que les conditions sont remplies :

1. Le **coût** (bois, pierre, fer) est **prélevé** immédiatement.
2. La **construction est considérée comme démarrée** : la progression (pts de construction) pour ce bâtiment peut désormais avancer avec les maçons alloués (1 pt/jour par maçon).
3. Le **bloc d’allocation des maçons** pour ce bâtiment devient **actif** : le joueur peut allouer des maçons depuis la pool de la Guilde des Maçons vers ce bâtiment (boutons +/− déjà présents sur la carte).

En résumé : **paiement → démarrage construction → allocation des maçons possible** pour faire avancer la barre.

### 1.5 Barre de progression de la construction

- Une **barre de chargement** (ou barre de progression) représente le **pourcentage de complétion** de la construction en cours pour ce bâtiment.
- **Largeur recommandée** : 200 px (valeur cible pour la cohérence visuelle).
- La barre affiche : **progression actuelle / pts requis** (ou équivalent en %).
- Quand la progression atteint 100 %, le niveau (ou la maison) est **terminé** : le bâtiment est mis à jour, la barre est réinitialisée pour le prochain niveau, et les ressources ont déjà été payées au moment du clic sur le bouton.

### 1.6 Relation avec l’allocation des maçons

- **Avant** le premier clic sur le bouton de construction (pour ce bâtiment), la construction n’a pas « démarré » au sens interface : les maçons peuvent déjà être alloués métier, mais l’affichage peut considérer qu’il n’y a pas encore de chantier en cours.
- **Après** le clic (coût payé, construction démarrée), le **bloc d’allocation des maçons** est **actif** : le joueur peut augmenter ou diminuer le nombre de maçons alloués à ce bâtiment (depuis la pool de la Guilde). Les maçons alloués font avancer la barre de 1 pt/jour chacun.

Conceptuellement, le bouton vert = « payer et démarrer le chantier » ; la barre 200 px = « avancement du chantier » ; les +/− maçons = « affecter de la main d’œuvre au chantier ».

---

## 2. Zone de représentation de la cité

### 2.1 Emplacement et dimensions

- **Position** : entre le **header** (barre du haut : ressources, horloge, vitesses, etc.) et la **liste des bâtiments + boutons de clic**.
- **Largeur** : toute la **largeur** de la zone d’affichage (responsive).
- **Hauteur** :
  - **20 %** de la hauteur d’affichage ;
  - **Minimum 200 px** pour rester lisible quel que soit la résolution.

La zone est donc **responsive** en largeur et en hauteur (pourcentage avec plancher).

### 2.2 Découpage vertical : ciel et sol

La zone est coupée en **deux** parties verticales :

| Partie | Proportion en hauteur | Rôle |
|--------|------------------------|------|
| **Ciel** | **60 %** (partie haute) | Fond bleu clair ; pas d’éléments interactifs ; décor. |
| **Sol** | **40 %** (partie basse) | Fond marron clair / beige ; **zone de déplacement** des personnages (sprites). |

Limite ciel/sol = une ligne horizontale à 60 % de la hauteur de la zone.

### 2.3 Couleurs de fond

| Zone | Couleur |
|------|---------|
| Ciel | **Bleu clair** |
| Sol | **Marron clair / beige** |

Les valeurs exactes (hex, RGB) seront définies dans le guide d’implémentation ou le design system.

### 2.4 Représentation des personnages (sprites)

- Chaque **personne** (unité de population) est représentée par un **sprite** de **3 px de haut** et **1 px de large** (forme verticale 1×3).
- **Trois types** de personnages (donc trois variantes de sprite) :
  - **Gens** (population civile) ;
  - **Soldat** ;
  - **Maçon**.

Structure commune du sprite (3 pixels de haut) :

| Pixel | Position | Couleur | Signification |
|-------|-----------|---------|----------------|
| 1 | Le plus **haut** | **Blanc** | Tête (commun à tous les types). |
| 2–3 | En **bas** | Selon le type | Corps. |

Couleur du **corps** selon le type :

| Type | Couleur corps |
|------|----------------|
| Gens | **Vert** |
| Soldat | **Rouge** |
| Maçon | **Marron foncé** |

Donc : **1 pixel blanc (tête) + 2 pixels corps (vert / rouge / marron)** = 3 px de haut, 1 px de large.

Le nombre de sprites affichés reflète les effectifs actuels : **gens**, **soldats**, **maçons** (par exemple, un sprite par unité, ou un échantillonnage si le nombre est très grand — à préciser en implémentation).

### 2.5 Déplacement des personnages

- Les personnages (sprites) se **déplacent** dans la zone **sol** uniquement (les 40 % du bas).
- Le déplacement est **aléatoire** : direction et/ou vitesse peuvent être tirées aléatoirement, avec contraintes pour rester dans les limites du sol (pas de sortie de zone).
- Aucune collision obligatoire entre personnages dans ce concept ; l’objectif est un mouvement de vie simple pour donner l’impression d’une cité habitée.

---

## 3. Synthèse des concepts

| Élément | Concept clé |
|--------|-------------|
| **Bouton construction** | Vert = conditions OK, clic = payer + démarrer construction ; blanc = inactif. |
| **Barre construction** | 200 px, % complétion ; avance avec les maçons alloués. |
| **Bloc maçons** | Actif une fois la construction démarrée ; allocation depuis la pool Guilde. |
| **Zone cité** | 20 % hauteur (min 200 px), pleine largeur ; ciel 60 % bleu, sol 40 % beige. |
| **Sprites** | 3×1 px ; tête blanche ; corps vert / rouge / marron (gens / soldat / maçon). |
| **Mouvement** | Aléatoire dans la zone sol uniquement. |

---

## 4. Références

- [MiyuClicker - Batiments Macons et Construction](MiyuClicker%20-%20Batiments%20Macons%20et%20Construction.md) — Coûts, pts de construction, maçons.
- [MiyuClicker - Guide Implementation Zone Cite et Construction](MiyuClicker%20-%20Guide%20Implementation%20Zone%20Cite%20et%20Construction.md) — Guide d’implémentation technique (bouton construction, zone cité, sprites, mouvement).
- [MiyuClicker - MVP Ecrans et Mecaniques](MiyuClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md) — Écran Ma citée.
