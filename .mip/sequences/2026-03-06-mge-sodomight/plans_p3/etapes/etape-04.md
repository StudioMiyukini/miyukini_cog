# Etape 04 - UI, HUD, navigation et feedback

## Objectif

Construire toute la surface utilisateur indispensable a un ARPG de complexite D2.

## Taches

1. Poser le shell UI global: ecrans, overlays, modalites et focus.
2. Implementer le HUD combat de base: vie, ressource, barre de raccourcis, buffs.
3. Implementer les tooltips d'items, skills, stats et etats.
4. Implementer l'inventaire avec grille, equipement et drag and drop.
5. Implementer le stash avec persistence et transfert d'objets.
6. Implementer les surfaces vendors: achat, vente, repair, identify, gamble.
7. Implementer le journal de quetes et l'affichage de progression.
8. Implementer la feuille personnage: stats de base, derivees, resistances, breakpoints utiles.
9. Implementer les menus systeme: pause, options, video, audio, gameplay.
10. Poser navigation clavier/souris, clic contextuel, raccourcis et conflits d'input.
11. Poser les etats visuels critiques: survol, selection, indisponible, erreur, cooldown, manque ressource.
12. Ajouter feedback visuel, sonore et options d'accessibilite critiques.
13. Prevoir les surfaces debug minimales reservees au dev sans contaminer l'UX joueur.

## Documentation de soutien

1. Documenter l'arborescence UI et les ecrans requis.
2. Documenter la map d'inputs et les conventions d'interaction.
3. Documenter les feedbacks critiques lisibilite/combat/accessibilite.

## Criteres de sortie

1. Toutes les interactions coeur passent par une UI exploitable.
2. L'utilisateur peut gerer personnage, quetes et inventaire sans debug tools.
3. Les surfaces UI sont assez specifiees pour eviter la derive pendant l'implementation.
