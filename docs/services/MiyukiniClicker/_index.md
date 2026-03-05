# MiyuClicker â€” Premier jeu officiel Miyukini

## PrÃ©sentation

**MiyuClicker** est le premier jeu officiel de l'Ã©cosystÃ¨me Miyukini. Il sert de **dÃ©mo** pour montrer la coexistence de plusieurs services (OpÃ©rateurs, Toolkits) dans un mÃªme environnement COG.

- **Genre :** Idle / Clicker (gestion) + Grande stratÃ©gie (carte, conquÃªte type Risk).
- **Stack :** Rust, Dioxus, pack UI open-source Ã  licence permissive.
- **Cibles :** Desktop, Web (WASM), Ã©ventuellement Android.

## Ordre de lecture recommandÃ©

1. **[Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md)** â€” Vision, marchÃ©, gameplay, stack.
2. **[Boucles de Gameplay et Elements](MiyukiniClicker%20-%20Boucles%20de%20Gameplay%20et%20Elements.md)** â€” Boucles de gameplay, hiÃ©rarchie population, coÃ»ts conversion, layout GUI, assets UI.
3. **[Ã‰tat des lieux](MiyukiniClicker%20-%20Etat%20des%20Lieux.md)** â€” Ce qui est fait / partiel / non fait (code vs spec).
4. **[Architecture technique actuelle](MiyukiniClicker%20-%20Architecture%20Technique%20Actuelle.md)** â€” Modules, flux d'appel, oÃ¹ se trouve la logique.
5. **[Guide Implementation MVP](MiyukiniClicker%20-%20Guide%20Implementation%20MVP.md)** â€” ModÃ¨le d'Ã©tat, specs Ã©crans, APIs, phases.
6. Autres docs selon le besoin (Zone CitÃ©, BÃ¢timents, Audit, Packs UI, etc.).

## Documents

| Document | Description |
|----------|-------------|
| [MiyuClicker - Document Fondateur](MiyukiniClicker%20-%20Document%20Fondateur.md) | Raison d'Ãªtre, analyse marchÃ©, besoins/Toolkits, gameplay, versions 0.1 et beta v1.0, stack, intÃ©gration COG, inspirations, packs UI. |
| [MiyuClicker - Etat des Lieux](MiyukiniClicker%20-%20Etat%20des%20Lieux.md) | Ã‰tat des lieux code vs spec : Ã©crans, modÃ¨le d'Ã©tat, mÃ©caniques (Fait / Partiel / Non fait). Ã€ lire aprÃ¨s le Document Fondateur pour reprendre le dev. |
| [MiyuClicker - Architecture Technique Actuelle](MiyukiniClicker%20-%20Architecture%20Technique%20Actuelle.md) | Architecture technique actuelle : modules (app, state, idlesim, save, carte, combat), flux d'appel, schÃ©ma. |
| [MiyuClicker - Analyse Marche et Besoins Toolkits](MiyukiniClicker%20-%20Analyse%20Marche%20et%20Besoins%20Toolkits.md) | Analyse PR du marchÃ© Idle/RPG/gestion ; dÃ©tail des besoins mÃ©tier et Toolkits (UI, animations, sprites, carte, sauvegarde). |
| [MiyuClicker - Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md) | Inventaire des packs dans `ui/game_ui_pack` (Cute_Fantasy, Cute_Fantasy_UI, modernuserinterface-win, Tiny RPG, ui-icn_fantasy-weapons_01, CatUIFree) ; licences, contenu, mapping besoin MiyuClicker. |
| [MiyuClicker - Parcours Utilisateur](MiyukiniClicker%20-%20Parcours%20Utilisateur.md) | Parcours utilisateur : Ã©cran de chargement, Ã©cran d'accueil [Jouer] + roue config, sÃ©lection des 3 slots, lancement du jeu. |
| [MiyuClicker - Ergonomie Ecran Gestion](MiyukiniClicker%20-%20Ergonomie%20Ecran%20Gestion.md) | Ergonomie Ã©cran de gestion : 4 gros boutons gauche (Champs, Ateliers, ChÃ¢teau, Village) ; liste dÃ©roulante droite pour affectation des gens (Cookie Clickerâ€“like). |
| [MiyuClicker - Layout Lord of Click Blocs Organiques et Molecules](MiyukiniClicker%20-%20Layout%20Lord%20of%20Click%20Blocs%20Organiques%20et%20Molecules.md) | Layout cible Ã©cran Ma citÃ©e : blocs organiques (marron) = zones structurelles ; molÃ©cules/atomes (noir) = panneaux ressources, liste bÃ¢timents, recherche, boutons. |
| [MiyuClicker - MVP Ecrans et Mecaniques](MiyukiniClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md) | MVP : Ã©crans (Loading, Landing, Slots, Ma citÃ©e, Carte du monde), mÃ©caniques de jeu (clic, allocation, tick, sauvegarde), mapping Toolkits / OpÃ©rateurs / Service. |
| [MiyuClicker - Operateurs et Toolkits](MiyukiniClicker%20-%20Operateurs%20et%20Toolkits.md) | Mapping OpÃ©rateurs et Toolkits pour le MVP : Toolkits Ã  utiliser (Dioxus) ou crÃ©er (Sprites, IdleSim, Save, Combat, Carte) ; OpÃ©rateurs (UI, Sim, Save, Combat, Carte) ; Service MiyuClicker. |
| [MiyuClicker - Guide Implementation MVP](MiyukiniClicker%20-%20Guide%20Implementation%20MVP.md) | Guide d'implÃ©mentation MVP : modÃ¨le d'Ã©tat, spÃ©cifications par Ã©cran, APIs Toolkits/OpÃ©rateurs, format de sauvegarde, phases d'implÃ©mentation. |
| [MiyuClicker - Systeme Bonheur](MiyukiniClicker%20-%20Systeme%20Bonheur.md) | SystÃ¨me de bonheur (moral) : rÃ¨gles nourriture/moral, Game Over 7j Ã  0 nourriture, affichage % aprÃ¨s Recherche. |
| [MiyuClicker - Batiments Macons et Construction](MiyukiniClicker%20-%20Batiments%20Macons%20et%20Construction.md) | BÃ¢timents (Maison, Caserne, Grenier, DÃ©pÃ´t, EntrepÃ´t), Guilde des MaÃ§ons, maÃ§ons, construction, clics, UI cartes. |
| [MiyuClicker - Zone Cite et Bouton Construction](MiyukiniClicker%20-%20Zone%20Cite%20et%20Bouton%20Construction.md) | Concept : bouton construction (vert/blanc), barre 200 px, zone citÃ© (ciel/sol), sprites 3Ã—1 px (tÃªte blanche, corps vert/rouge/marron), dÃ©placement alÃ©atoire. |
| [MiyuClicker - Guide Implementation Zone Cite et Construction](MiyukiniClicker%20-%20Guide%20Implementation%20Zone%20Cite%20et%20Construction.md) | Guide d'implÃ©mentation : bouton construction (conditions, paiement au clic), zone citÃ© (20 % hauteur min 200 px, ciel 60 % / sol 40 %), sprites et mouvement alÃ©atoire, MIP. |
| [MiyuClicker - Boucles de Gameplay et Elements](MiyukiniClicker%20-%20Boucles%20de%20Gameplay%20et%20Elements.md) | Boucles de gameplay (production, construction, conquÃªte), hiÃ©rarchie de population (ouvriers â†’ bÃ¢tisseurs / soldats), coÃ»ts de conversion, layout GUI de rÃ©fÃ©rence, assets Fantasy UI Borders et curseurs Toon. |
| [MiyuClicker - Audit Qualite et Optimisations](MiyukiniClicker%20-%20Audit%20Qualite%20et%20Optimisations.md) | Audit du code : mÃ©trique de qualitÃ© (score / 10), points forts/faibles, optimisations proposÃ©es (performance, DRY, robustesse, testabilitÃ©). |

## Liens utiles

- [Stack UI Dioxus](..//..//_index.md) â€” Stack UI officielle Miyukini.
- **Assets :** `ui/game_ui_pack` â€” voir [Reference Packs UI Jeux](MiyukiniClicker%20-%20Reference%20Packs%20UI%20Jeux.md).


