# MiyuClicker — Premier jeu officiel Miyukini

## Présentation

**MiyuClicker** est le premier jeu officiel de l'écosystème Miyukini. Il sert de **démo** pour montrer la coexistence de plusieurs services (Opérateurs, Toolkits) dans un même environnement COG.

- **Genre :** Idle / Clicker (gestion) + Grande stratégie (carte, conquête type Risk).
- **Stack :** Rust, Dioxus, pack UI open-source à licence permissive.
- **Cibles :** Desktop, Web (WASM), éventuellement Android.

## Ordre de lecture recommandé

1. **[Document Fondateur](MiyuClicker%20-%20Document%20Fondateur.md)** — Vision, marché, gameplay, stack.
2. **[Boucles de Gameplay et Elements](MiyuClicker%20-%20Boucles%20de%20Gameplay%20et%20Elements.md)** — Boucles de gameplay, hiérarchie population, coûts conversion, layout GUI, assets UI.
3. **[État des lieux](MiyuClicker%20-%20Etat%20des%20Lieux.md)** — Ce qui est fait / partiel / non fait (code vs spec).
4. **[Architecture technique actuelle](MiyuClicker%20-%20Architecture%20Technique%20Actuelle.md)** — Modules, flux d'appel, où se trouve la logique.
5. **[Guide Implementation MVP](MiyuClicker%20-%20Guide%20Implementation%20MVP.md)** — Modèle d'état, specs écrans, APIs, phases.
6. Autres docs selon le besoin (Zone Cité, Bâtiments, Audit, Packs UI, etc.).

## Documents

| Document | Description |
|----------|-------------|
| [MiyuClicker - Document Fondateur](MiyuClicker%20-%20Document%20Fondateur.md) | Raison d'être, analyse marché, besoins/Toolkits, gameplay, versions 0.1 et beta v1.0, stack, intégration COG, inspirations, packs UI. |
| [MiyuClicker - Etat des Lieux](MiyuClicker%20-%20Etat%20des%20Lieux.md) | État des lieux code vs spec : écrans, modèle d'état, mécaniques (Fait / Partiel / Non fait). À lire après le Document Fondateur pour reprendre le dev. |
| [MiyuClicker - Architecture Technique Actuelle](MiyuClicker%20-%20Architecture%20Technique%20Actuelle.md) | Architecture technique actuelle : modules (app, state, idlesim, save, carte, combat), flux d'appel, schéma. |
| [MiyuClicker - Analyse Marche et Besoins Toolkits](MiyuClicker%20-%20Analyse%20Marche%20et%20Besoins%20Toolkits.md) | Analyse PR du marché Idle/RPG/gestion ; détail des besoins métier et Toolkits (UI, animations, sprites, carte, sauvegarde). |
| [MiyuClicker - Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md) | Inventaire des packs dans `ui/game_ui_pack` (Cute_Fantasy, Cute_Fantasy_UI, modernuserinterface-win, Tiny RPG, ui-icn_fantasy-weapons_01, CatUIFree) ; licences, contenu, mapping besoin MiyuClicker. |
| [MiyuClicker - Parcours Utilisateur](MiyuClicker%20-%20Parcours%20Utilisateur.md) | Parcours utilisateur : écran de chargement, écran d'accueil [Jouer] + roue config, sélection des 3 slots, lancement du jeu. |
| [MiyuClicker - Ergonomie Ecran Gestion](MiyuClicker%20-%20Ergonomie%20Ecran%20Gestion.md) | Ergonomie écran de gestion : 4 gros boutons gauche (Champs, Ateliers, Château, Village) ; liste déroulante droite pour affectation des gens (Cookie Clicker–like). |
| [MiyuClicker - Layout Lord of Click Blocs Organiques et Molecules](MiyuClicker%20-%20Layout%20Lord%20of%20Click%20Blocs%20Organiques%20et%20Molecules.md) | Layout cible écran Ma citée : blocs organiques (marron) = zones structurelles ; molécules/atomes (noir) = panneaux ressources, liste bâtiments, recherche, boutons. |
| [MiyuClicker - MVP Ecrans et Mecaniques](MiyuClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md) | MVP : écrans (Loading, Landing, Slots, Ma citée, Carte du monde), mécaniques de jeu (clic, allocation, tick, sauvegarde), mapping Toolkits / Opérateurs / Service. |
| [MiyuClicker - Operateurs et Toolkits](MiyuClicker%20-%20Operateurs%20et%20Toolkits.md) | Mapping Opérateurs et Toolkits pour le MVP : Toolkits à utiliser (Dioxus) ou créer (Sprites, IdleSim, Save, Combat, Carte) ; Opérateurs (UI, Sim, Save, Combat, Carte) ; Service MiyuClicker. |
| [MiyuClicker - Guide Implementation MVP](MiyuClicker%20-%20Guide%20Implementation%20MVP.md) | Guide d'implémentation MVP : modèle d'état, spécifications par écran, APIs Toolkits/Opérateurs, format de sauvegarde, phases d'implémentation. |
| [MiyuClicker - Systeme Bonheur](MiyuClicker%20-%20Systeme%20Bonheur.md) | Système de bonheur (moral) : règles nourriture/moral, Game Over 7j à 0 nourriture, affichage % après Recherche. |
| [MiyuClicker - Batiments Macons et Construction](MiyuClicker%20-%20Batiments%20Macons%20et%20Construction.md) | Bâtiments (Maison, Caserne, Grenier, Dépôt, Entrepôt), Guilde des Maçons, maçons, construction, clics, UI cartes. |
| [MiyuClicker - Zone Cite et Bouton Construction](MiyuClicker%20-%20Zone%20Cite%20et%20Bouton%20Construction.md) | Concept : bouton construction (vert/blanc), barre 200 px, zone cité (ciel/sol), sprites 3×1 px (tête blanche, corps vert/rouge/marron), déplacement aléatoire. |
| [MiyuClicker - Guide Implementation Zone Cite et Construction](MiyuClicker%20-%20Guide%20Implementation%20Zone%20Cite%20et%20Construction.md) | Guide d'implémentation : bouton construction (conditions, paiement au clic), zone cité (20 % hauteur min 200 px, ciel 60 % / sol 40 %), sprites et mouvement aléatoire, MIP. |
| [MiyuClicker - Boucles de Gameplay et Elements](MiyuClicker%20-%20Boucles%20de%20Gameplay%20et%20Elements.md) | Boucles de gameplay (production, construction, conquête), hiérarchie de population (ouvriers → bâtisseurs / soldats), coûts de conversion, layout GUI de référence, assets Fantasy UI Borders et curseurs Toon. |
| [MiyuClicker - Audit Qualite et Optimisations](MiyuClicker%20-%20Audit%20Qualite%20et%20Optimisations.md) | Audit du code : métrique de qualité (score / 10), points forts/faibles, optimisations proposées (performance, DRY, robustesse, testabilité). |

## Liens utiles

- [Stack UI Dioxus](../../ux_ui/Miyukini%20-%20Stack%20UI%20Dioxus.md) — Stack UI officielle Miyukini.
- **Assets :** `ui/game_ui_pack` — voir [Reference Packs UI Jeux](MiyuClicker%20-%20Reference%20Packs%20UI%20Jeux.md).
