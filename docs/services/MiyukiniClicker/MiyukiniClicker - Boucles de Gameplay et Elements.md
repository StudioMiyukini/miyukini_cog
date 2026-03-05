# MiyuClicker â€” Boucles de gameplay, elements et assets UI

## Contexte

Ce document decrit les **boucles de gameplay** de MiyuClicker, les **elements necessaires** a l'ecran de gestion (Â« Mon domaine Â»), la **hierarchie de population** (ouvriers, batisseurs, soldats), les **couts de conversion**, et l'utilisation des **packs UI** (Fantasy UI Borders, curseurs Toon) issus du Miyukini UI Builder.

Il est base sur le **mockup GUI de reference** (`references/MiyukiniClicker_GUI.jpg`) qui represente la maquette cible de l'ecran principal.

## Portee / Scope

- **Perimetre :** Boucles de gameplay (production, construction, conversion de population, combat), layout GUI de reference, couts de conversion ouvrier â†’ batisseur / soldat, utilisation des assets Fantasy UI Borders et curseurs Toon.
- **Hors perimetre :** Carte du monde (grande strategie), equilibrage numerique fin, ecrans secondaires (Loading, Landing, Slots).
- **Reference code :** `crates/miyuclicker/` (a implementer), `apps/central/src/services/ui_assets.rs` (packs UI).

---

## 1. Hierarchie de population

La population du joueur se divise en **trois types**, tous issus d'une meme pool de base :

```
Population totale (Pop)
â”œâ”€â”€ Ouvriers    (population de base, unite fondamentale)
â”œâ”€â”€ Batisseurs  (ouvriers transformes, construisent les batiments)
â””â”€â”€ Soldats     (ouvriers transformes, defendent et conquerent)
```

### 1.1 Ouvriers (pop de base)

| Propriete | Description |
|-----------|-------------|
| **Role** | Unite de base de la population. Peuvent etre affectes a la **production** (ferme, scierie, carriere, mine, atelier, forge). |
| **Origine** | Naissent naturellement (fecondite) ou via actions du joueur. |
| **Transformation** | Peuvent etre **convertis** en batisseurs ou en soldats (conversion unidirectionnelle). |
| **Affichage** | Barre du haut : `Ouvriers xxx` |
| **Sprite** | 3Ã—1 px â€” tete blanche, corps **vert** |

### 1.2 Batisseurs

| Propriete | Description |
|-----------|-------------|
| **Role** | Ouvriers specialises dans la **construction**. Alloues aux chantiers (Maisons, Casernes, Guilde des Macons), ils apportent **1 pt de construction / jour** chacun. |
| **Origine** | Transformation d'un ouvrier (voir section 3.1). |
| **Affichage** | Barre du haut : `Batisseurs xxx` |
| **Sprite** | 3Ã—1 px â€” tete blanche, corps **marron fonce** |

### 1.3 Soldats

| Propriete | Description |
|-----------|-------------|
| **Role** | Ouvriers specialises dans le **combat**. Utilises pour defendre le territoire et conquetes sur la Carte du monde. |
| **Origine** | Transformation d'un ouvrier (voir section 3.2). |
| **Affichage** | Barre du haut : `Soldats xxx` |
| **Sprite** | 3Ã—1 px â€” tete blanche, corps **rouge** |

---

## 2. Les trois boucles de gameplay

Le jeu repose sur **trois boucles interconnectees** qui se renforcent mutuellement :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  BOUCLE 1 : PRODUCTION          BOUCLE 2 : CONSTRUCTION        â”‚
â”‚  (idle/automatique)             (investissement long terme)     â”‚
â”‚                                                                 â”‚
â”‚  Ouvriers â†’ Ressources          Batisseurs â†’ Batiments          â”‚
â”‚  (food, bois, pierre,           (Maisons â†’ +pop cap,            â”‚
â”‚   metal, outils, armes)          Casernes â†’ +soldat cap,        â”‚
â”‚                                  Guilde â†’ +batisseur cap)       â”‚
â”‚                                                                 â”‚
â”‚          â””â”€â”€â”€â”€â”€â”€â”€ alimentent â”€â”€â”€â”€â”€â”€â”€â”€â”˜                          â”‚
â”‚                        â”‚                                        â”‚
â”‚                        â–¼                                        â”‚
â”‚              BOUCLE 3 : CONQUETE                                â”‚
â”‚              (grande strategie)                                  â”‚
â”‚              Soldats â†’ Carte du monde                            â”‚
â”‚              â†’ bonus de tribu (ressources)                       â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 2.1 Boucle 1 â€” Production (idle)

La boucle **primaire** du jeu. Les ouvriers sont affectes a des postes de production qui generent des ressources en continu (par tick/seconde).

| Poste de production | Ressource generee | Affichage GUI |
|---------------------|-------------------|---------------|
| **Ferme** | Nourriture (Food/sec) | `Ferme - xxx Food/sec [+] xxx [-]` |
| **Scierie** | Bois (Bois/sec) | `Scierie - xxx Bois/sec [+] xxx [-]` |
| **Carriere** | Pierre (Pierres/sec) | `Carriere - xxx Pierres/sec [+] xxx [-]` |
| **Mine** | Metal (Metal/sec) | `Mine - xxx Metal/sec [+] xxx [-]` |
| **Atelier** | Outils (Outils/sec) | `Atelier - xxx Outils/sec [+] xxx [-]` |
| **Forge** | Armes (Armes/sec) | `Forge - xxx Armes/sec [+] xxx [-]` |

**Mecanisme :**
- Le joueur utilise les boutons **[+]** et **[-]** pour affecter/retirer des ouvriers a chaque poste.
- Le nombre d'ouvriers affectes determine le debit de production (xxx/sec).
- Les ouvriers non affectes restent dans la pool Â« Ouvriers Â» disponibles.

**Formule de production (par tick) :**
```
ressource += nb_ouvriers_affectes Ã— taux_de_base Ã— delta_temps
```

### 2.2 Boucle 2 â€” Construction (long terme)

La boucle de **croissance** du royaume. Les batisseurs construisent des batiments qui augmentent les plafonds de population.

| Batiment | Effet | Cout (B:xx P:xx M:xx) | Pts construction |
|----------|-------|-----------------------|------------------|
| **Maisons** | +4 cap population par maison | 30 bois, 20 pierre, 5 metal | 30 pts (+1%/maison existante) |
| **Casernes** | +10 cap soldats par niveau | 50 bois, 100 pierre, 20 metal | 100 pts (+5%/niveau) |
| **Guilde des Macons** | +cap batisseurs par niveau | A definir | A definir |

**Mecanisme :**
1. Le joueur clique sur **[Construire]** si les ressources sont suffisantes (bouton vert) ou non (bouton blanc/inactif).
2. Le cout (B:xx P:xx M:xx = Bois, Pierre, Metal) est **preleve immediatement**.
3. Les **batisseurs alloues** font progresser la barre (1 pt/jour chacun).
4. Quand la barre atteint 100% â†’ le batiment monte de niveau (ou +1 maison).

**Allocation des batisseurs :**
- Chaque carte batiment a ses propres boutons **[+] xxx [-]** pour affecter des batisseurs.
- Les batisseurs sont pris de la pool globale de batisseurs.

### 2.3 Boucle 3 â€” Conquete (grande strategie)

La boucle d'**objectif a long terme**. Les soldats sont envoyes sur la Carte du monde pour conquÃ©rir des cites-Etats.

| Action | Mecanisme |
|--------|-----------|
| **Envoi de troupes** | Le joueur selectionne une cite adverse et envoie X soldats. |
| **Deplacement** | Temps variable selon la distance (route). |
| **Combat** | Resolution simplifiee (hasard + stats). |
| **Victoire** | La cite donne un **bonus de tribu** (ressources continues). |

*(Detail : voir [MiyuClicker - Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md), sections 3.2 et 3.3.)*

---

## 3. Conversions de population

Les ouvriers sont la **ressource humaine fondamentale**. Ils peuvent etre transformes de maniere **unidirectionnelle** en batisseurs ou soldats. Les couts augmentent avec le nombre deja converti, creant une **courbe de cout croissant** qui force le joueur a equilibrer ses investissements.

### 3.1 Ouvrier â†’ Batisseur

| Propriete | Valeur |
|-----------|--------|
| **Action** | +1 Batisseur |
| **Consomme** | 1 ouvrier |
| **Cout en outils** | **20 + A** outils (ou **A** = nombre de batisseurs deja presents) |
| **Prerequis** | Au moins 1 ouvrier disponible + outils suffisants |

**Exemples de cout :**

| Batisseurs actuels (A) | Cout en outils | Cout total cumule |
|------------------------|----------------|-------------------|
| 0 | 20 | 20 |
| 1 | 21 | 41 |
| 2 | 22 | 63 |
| 5 | 25 | 133 |
| 10 | 30 | 275 |
| 20 | 40 | 630 |
| 50 | 70 | 2 295 |

**Formule :** `cout_outils(A) = 20 + A`

**Justification du scaling :** Les outils deviennent de plus en plus demandes au fur et a mesure que le joueur specialise sa main d'oeuvre. Cela force un equilibre entre production (ouvriers a l'Atelier) et construction (batisseurs).

### 3.2 Ouvrier â†’ Soldat

| Propriete | Valeur |
|-----------|--------|
| **Action** | +1 Soldat |
| **Consomme** | 1 ouvrier |
| **Cout en armes** | **10 + S** armes (ou **S** = nombre de soldats deja presents) |
| **Prerequis** | Au moins 1 ouvrier disponible + armes suffisantes |

**Exemples de cout :**

| Soldats actuels (S) | Cout en armes | Cout total cumule |
|---------------------|---------------|-------------------|
| 0 | 10 | 10 |
| 1 | 11 | 21 |
| 2 | 12 | 33 |
| 5 | 15 | 85 |
| 10 | 20 | 175 |
| 20 | 30 | 430 |
| 50 | 60 | 1 785 |

**Formule :** `cout_armes(S) = 10 + S`

**Justification du scaling :** Les armes sont plus rares que les outils (Forge consomme du metal + bois), ce qui rend l'armee chere a maintenir. Le joueur doit equilibrer son investissement militaire avec ses besoins economiques.

### 3.3 Pas de conversion inverse

Les conversions sont **unidirectionnelles** : un batisseur ou un soldat ne peut pas redevenir ouvrier. Cela rend chaque conversion **strategique** et irreversible â€” le joueur doit planifier ses besoins.

### 3.4 Representation dans la GUI

D'apres le mockup de reference, les fleches entre la zone de production (gauche) et la zone de construction (droite) montrent les chemins de conversion :

```
Production (ouvriers)              Construction (batiments)
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Ferme           â”‚  â”€â”€ +1 Ouvrier â”€â”€â–º  Maisons          â”‚
â”‚ Scierie         â”‚                â”‚                      â”‚
â”‚ Carriere        â”‚  â”€â”€ +1 Batisseur â–º  Casernes         â”‚
â”‚ Mine            â”‚                â”‚                      â”‚
â”‚ Atelier         â”‚  â”€â”€ +1 Soldat â”€â”€â”€â–º  Guilde Macons    â”‚
â”‚ Forge           â”‚                â”‚                      â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜                â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

---

## 4. Boucle d'equilibre â€” Le dilemme du joueur

Le gameplay cree un **triangle d'equilibre** permanent :

```
        Nourriture
        (survie)
           â–²
          / \
         /   \
        /     \
       /  POP  \
      /         \
     â–¼           â–¼
  Outils       Armes
(croissance)  (conquete)
```

| Choix | Avantage | Risque |
|-------|----------|--------|
| **Maximiser la production** | Plus de ressources, croissance rapide | Pas de defense, pas de constructions |
| **Maximiser les batisseurs** | Batiments rapides, plafonds eleves | Moins d'ouvriers en production, armee faible |
| **Maximiser les soldats** | Armee forte, conquetes rapides | Economie affaiblie, constructions lentes |

**Boucle de feedback :**
1. Plus d'ouvriers â†’ plus de ressources â†’ possibilite de convertir â†’ plus de batisseurs/soldats.
2. Plus de batisseurs â†’ batiments plus rapides â†’ cap population plus eleve â†’ plus d'ouvriers potentiels.
3. Plus de soldats â†’ conquetes â†’ bonus de tribu â†’ plus de ressources passives.
4. Trop de conversions â†’ moins d'ouvriers â†’ production en chute â†’ famine possible.

**Contrainte critique :** Le systeme de **bonheur** (voir [MiyuClicker - Systeme Bonheur](MiyukiniClicker%20-%20Systeme%20Bonheur.md)) agit comme regulateur. Si la nourriture tombe en dessous de la population, le moral baisse, la fecondite chute, et apres 7 jours a 0 nourriture â†’ **Game Over**.

---

## 5. Layout GUI de reference

Base sur le mockup `references/MiyukiniClicker_GUI.jpg` :

### 5.1 Structure globale

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ HEADER (2 lignes)                                                   â”‚
â”‚ L1: [pseudo] Food xx/max â”‚ Bois xx/max â”‚ Pierre xx/max â”‚ Metal     â”‚
â”‚     xx/max â”‚ Outils xx/max â”‚ Armes xx/max           [âš™ config]    â”‚
â”‚ L2: Pop xxx/max â”‚ Ouvriers xxx â”‚ Batisseurs xxx â”‚ Soldats xxx      â”‚
â”‚                                    â”‚ Carte du monde â”‚ Mon domaine â”‚ â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ ZONE CITE (20% hauteur, min 200px)                                  â”‚
â”‚ â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ ciel (60%) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”            â”‚
â”‚ â”‚                    [chateau]                          â”‚ [notif]   â”‚
â”‚ â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€ sol (40%) â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤            â”‚
â”‚ â”‚  â– â– â–    â–â–â–â–â–â–â–    â–â–      â–â–â–                   â”‚            â”‚
â”‚ â”‚ vert=ouvr  rouge=sold  violet=bat                     â”‚            â”‚
â”‚ â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜            â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ PANNEAU BAS (production + construction)                              â”‚
â”‚ â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”  conversions  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”          â”‚
â”‚ â”‚ La productionâ”‚               â”‚ Maisons        [LVL]    â”‚          â”‚
â”‚ â”‚              â”‚  +1 Ouvrier   â”‚ â–“â–“â–“â–“â–‘â–‘â–‘â–‘â–‘â–‘â–‘â–‘â–‘â–‘â–‘ 0%      â”‚          â”‚
â”‚ â”‚ Ferme    [+]â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º  â”‚ Batisseurs [+]xxx[-]     â”‚          â”‚
â”‚ â”‚ Scierie  [+]â”‚               â”‚ B:xx P:xx M:xx [Const]   â”‚          â”‚
â”‚ â”‚ Carriere [+]â”‚  +1 Batisseur â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤          â”‚
â”‚ â”‚ Mine     [+]â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º  â”‚ Casernes       [LVL]    â”‚          â”‚
â”‚ â”‚ Atelier  [+]â”‚               â”‚ â–“â–“â–“â–“â–‘â–‘â–‘â–‘â–‘â–‘â–‘â–‘â–‘â–‘â–‘ 0%      â”‚          â”‚
â”‚ â”‚ Forge    [+]â”‚  +1 Soldat    â”‚ Batisseurs [+]xxx[-]     â”‚          â”‚
â”‚ â”‚              â”‚  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º  â”‚ B:xx P:xx M:xx [Const]   â”‚          â”‚
â”‚ â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜               â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤          â”‚
â”‚                                â”‚ Guilde Macons  [LVL]    â”‚          â”‚
â”‚                                â”‚ â–“â–“â–“â–“â–‘â–‘â–‘â–‘â–‘â–‘â–‘â–‘â–‘â–‘â–‘ 0%      â”‚          â”‚
â”‚                                â”‚ Batisseurs [+]xxx[-]     â”‚          â”‚
â”‚                                â”‚ B:xx P:xx M:xx [Const]   â”‚          â”‚
â”‚                                â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜          â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚ BARRE GLOBALE (progression ?)                                        â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.2 Elements du header

**Ligne 1 â€” Ressources materielles :**

| Element | Format | Description |
|---------|--------|-------------|
| Pseudo | `[pseudo]` | Nom du joueur / seigneur |
| Food | `xx/max` | Nourriture courante / cap (Grenier) |
| Bois | `xx/max` | Bois courant / cap (Depot) |
| Pierre | `xx/max` | Pierre courante / cap (Depot) |
| Metal | `xx/max` | Metal courant / cap (Depot) |
| Outils | `xx/max` | Outils courants / cap (Entrepot) |
| Armes | `xx/max` | Armes courantes / cap (Entrepot) |
| Config | `[âš™]` | Roue de configuration (sauvegarde, options) |

**Ligne 2 â€” Population et navigation :**

| Element | Format | Description |
|---------|--------|-------------|
| Pop | `xxx/max` | Population totale / cap (Maisons) |
| Ouvriers | `xxx` | Ouvriers disponibles (non affectes) |
| Batisseurs | `xxx` | Total des batisseurs |
| Soldats | `xxx` | Total des soldats |
| Carte du monde | Bouton | Navigation vers l'ecran strategie |
| Mon domaine | Bouton | Navigation vers l'ecran gestion (actif) |

### 5.3 Zone cite (representation visuelle)

Zone d'**ambiance** entre le header et les panneaux de gestion :
- Hauteur : 20% de l'ecran, minimum 200px
- Ciel (60% du haut) : fond bleu clair / decoratif
- Sol (40% du bas) : fond vert, sprites de population en mouvement aleatoire
- Chateau au centre (batiment principal)
- **Notifications** : glissent depuis la droite (ex. Â« Tu es monte de niveau Â»)

### 5.4 Panneau de production (gauche)

Panneau **Â« La production Â»** avec 6 postes, chacun representant un lieu ou les ouvriers produisent des ressources :

| Poste | Format affichage |
|-------|------------------|
| Ferme | `Ferme - xxx Food/sec [+] xxx [-]` |
| Scierie | `Scierie - xxx Bois/sec [+] xxx [-]` |
| Carriere | `Carriere - xxx Pierres/sec [+] xxx [-]` |
| Mine | `Mine - xxx Metal/sec [+] xxx [-]` |
| Atelier | `Atelier - xxx Outils/sec [+] xxx [-]` |
| Forge | `Forge - xxx Armes/sec [+] xxx [-]` |

- Premier `xxx` = debit de production actuel
- `[+]` = affecter +1 ouvrier
- Deuxieme `xxx` = nombre d'ouvriers affectes
- `[-]` = retirer 1 ouvrier

### 5.5 Cartes batiment (droite)

Chaque batiment est represente par une **carte** avec :
- **Nom** et **description** de l'effet
- Badge **[LVL]** (niveau courant)
- **Barre de progression** (0-100%) de la construction en cours
- **Allocation batisseurs** : `Batisseurs [+] xxx [-]`
- **Cout** : `B:xx P:xx M:xx` (Bois, Pierre, Metal)
- Bouton **[Construire]** (vert si conditions remplies, blanc sinon)

---

## 6. Assets UI â€” Fantasy UI Borders

L'interface de MiyuClicker utilise le pack **Fantasy UI Borders** (Kenney) disponible dans le Miyukini UI Builder pour un look RPG/medieval coherent.

### 6.1 Utilisation par element

| Element GUI | Asset Fantasy UI Borders | Justification |
|-------------|--------------------------|---------------|
| **Header** (barre ressources) | Panel (fond + bordure) â€” ex. PANEL_000 ou PANEL_003 | Cadre solide pour les informations permanentes |
| **Cartes batiment** | Panel avec bordure coloree â€” PANEL_001, PANEL_002 | Chaque type de batiment a une couleur de bordure (vert=Maisons, rouge=Casernes, violet=Guilde) |
| **Panneau production** | Border (centre transparent) â€” BORDER_000 ou BORDER_003 | Cadre sans fond pour laisser voir le background |
| **Bouton [Construire]** | Panel compact ou bouton stylise | Vert (conditions OK) / blanc-gris (inactif) |
| **Barres de progression** | Barres SVG parametriques (bar_track + bar_fill) | Coherence avec le systeme de barres du UI Builder |
| **Separateurs** | Dividers â€” DIVIDER_000 a DIVIDER_005 | Separation entre sections |
| **Zone cite** | Transparent Borders â€” TBORDER_000 | Cadre leger autour de la zone visuelle |

### 6.2 Couleurs de bordure des cartes batiment

D'apres le mockup GUI :

| Batiment | Couleur bordure | Hex suggere |
|----------|----------------|-------------|
| **Maisons** | Vert | `#4CAF50` |
| **Casernes** | Rouge | `#F44336` |
| **Guilde des Macons** | Violet / Bleu | `#7B1FA2` |
| **Grenier** (futur) | Jaune | `#FFC107` |
| **Depot** (futur) | Orange | `#FF9800` |
| **Entrepot** (futur) | Cyan | `#00BCD4` |

### 6.3 Reference assets

Les assets sont disponibles via :
- `apps/central/src/services/ui_assets.rs` â€” module `fantasy_borders`
- Miyukini UI Builder â†’ onglet **Fenetres** â†’ section **Fantasy UI Borders (Kenney)**

---

## 7. Assets UI â€” Curseurs Toon

MiyuClicker utilise les curseurs **Toon** du Cursor Pack (Basic) pour renforcer l'ambiance cartoon/RPG du jeu.

### 7.1 Mapping curseur par contexte

| Contexte de jeu | Curseur Toon | Identifiant asset |
|-----------------|--------------|-------------------|
| **Navigation generale** | Toon A (pointeur) | `cursors::POINTER_TOON_A` |
| **Survol elements cliquables** | Toon B (pointeur alt) | `cursors::POINTER_TOON_B` |
| **Clic sur production** | Hand Point (main) | `cursors::HAND_POINT` |
| **Construction en cours** | Tool Hammer (marteau) | `cursors::TOOL_HAMMER` |
| **Forge / Armes** | Tool Sword A (epee) | `cursors::TOOL_SWORD_A` |
| **Carte du monde** | Nav (directions) | `cursors::NAV_*` |
| **Attente / chargement** | Busy (sablier) | `cursors::BUSY` |
| **Zone non-interactive** | Disabled | `cursors::DISABLED` |
| **Envoi de troupes** | Tool Bow (arc) | `cursors::TOOL_BOW` |

### 7.2 Implementation CSS

Chaque zone de l'interface applique son curseur via la propriete CSS `cursor:url(...)` :

```css
/* Navigation generale */
.game-container { cursor: url("toon_a.png") 0 0, auto; }

/* Elements cliquables */
.clickable:hover { cursor: url("toon_b.png") 0 0, pointer; }

/* Boutons de production */
.production-btn:hover { cursor: url("hand_point.png") 0 0, pointer; }

/* Zone de construction */
.building-card:hover { cursor: url("tool_hammer.png") 0 0, pointer; }
```

### 7.3 Reference assets

Les curseurs sont disponibles via :
- `apps/central/src/services/ui_assets.rs` â€” module `cursors`
- Miyukini UI Builder â†’ onglet **Curseurs** â†’ section **Cursor Pack (Basic)** â†’ categorie **Pointers** (Toon A, Toon B)

---

## 8. Flux de jeu complet (une session type)

### 8.1 Debut de partie

1. Le joueur demarre avec **10 ouvriers**, **20 nourriture**, **3 maisons** (cap pop = 12).
2. Il affecte des ouvriers a la **Ferme** pour produire de la nourriture (survie).
3. Il repartit les ouvriers restants entre **Scierie**, **Carriere**, **Mine** (matieres premieres).

### 8.2 Premiere phase â€” Stabilisation (premiÃ¨res minutes)

1. Accumuler de la nourriture pour nourrir la population croissante.
2. Produire du bois et de la pierre pour construire des **Maisons** (+4 pop cap chacune).
3. Lancer la construction de maisons â†’ affecter des batisseurs (apres conversion).

### 8.3 Deuxieme phase â€” Specialisation (milieu de partie)

1. Convertir des ouvriers en **batisseurs** (cout : 20 + A outils) pour accelerer la construction.
2. Monter les **Casernes** de niveau pour debloquer le cap de soldats.
3. Lancer la production a l'**Atelier** (outils) et a la **Forge** (armes).

### 8.4 Troisieme phase â€” Expansion (fin de partie)

1. Convertir des ouvriers en **soldats** (cout : 10 + S armes) pour constituer une armee.
2. Envoyer des soldats sur la **Carte du monde** pour conquÃ©rir des cites adverses.
3. Les **bonus de tribu** des cites conquises alimentent la boucle de production.
4. Objectif : conquÃ©rir toute la carte.

---

## 9. Synthese des elements a implementer

| Categorie | Elements | Priorite MVP |
|-----------|----------|--------------|
| **Etat du jeu** | Ressources (food, bois, pierre, metal, outils, armes), population (ouvriers, batisseurs, soldats), batiments (niveaux, progression construction), bonheur | Critique |
| **Systeme production** | 6 postes, affectation ouvriers [+][-], calcul debit/sec, tick de production | Critique |
| **Systeme construction** | Cartes batiment, bouton [Construire], barre progression, allocation batisseurs | Critique |
| **Conversion pop** | Ouvrier â†’ Batisseur (20+A outils), Ouvrier â†’ Soldat (10+S armes) | Critique |
| **Systeme bonheur** | Moral, fecondite, Game Over 7j sans nourriture | Important |
| **Zone cite** | Ciel/sol, sprites, mouvement aleatoire, chateau, notifications | Important |
| **Header** | 2 lignes ressources, navigation, config | Critique |
| **Assets UI** | Fantasy UI Borders (panneaux, bordures), curseurs Toon | Important |
| **Carte du monde** | Cites, routes, envoi troupes, combat simplifie | Post-MVP immediat |

---

## 10. References

- [MiyuClicker - Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md) â€” Vision, gameplay, inspirations.
- [MiyuClicker - Ressources et Categories](MiyukiniClicker%20-%20Ressources%20et%20Categories.md) â€” Terminologie ressources.
- [MiyuClicker - Batiments Macons et Construction](MiyukiniClicker%20-%20Batiments%20Macons%20et%20Construction.md) â€” Couts, pts construction.
- [MiyuClicker - Systeme Bonheur](MiyukiniClicker%20-%20Systeme%20Bonheur.md) â€” Moral, fecondite, Game Over.
- [MiyuClicker - Zone Cite et Bouton Construction](MiyukiniClicker%20-%20Zone%20Cite%20et%20Bouton%20Construction.md) â€” Zone visuelle, sprites.
- [MiyuClicker - MVP Ecrans et Mecaniques](MiyukiniClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md) â€” Ecrans et mecaniques MVP.
- [MiyuClicker - Ergonomie Ecran Gestion](MiyukiniClicker%20-%20Ergonomie%20Ecran%20Gestion.md) â€” Layout ecran gestion.
- **Mockup GUI** : `references/MiyukiniClicker_GUI.jpg`

---

**Document cree le :** 2026-02-11
**Derniere mise a jour :** 2026-02-11
**Statut :** Document de reference â€” boucles de gameplay, elements GUI, assets UI, conversions de population

