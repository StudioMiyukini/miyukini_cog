# Etape 02 - Runtime coeur, donnees et persistence

## Objectif

Stabiliser les fondations runtime et data-driven qui supporteront tous les systemes D2 et une externalisation future du serveur.

## Taches

1. Definir la boucle runtime globale: boot, load, simulate, render handoff, save.
2. Definir les phases de tick et l'ordre minimal des systems.
3. Poser le modele `World` / entites / composants / ressources partagees.
4. Poser le chargement de scene et la frontiere moteur/jeu pour les scenes.
5. Definir les schemas de contenus pour classes, skills, items, zones, quetes et monstres.
6. Ajouter une validation stricte des schemas et une strategie d'evolution de version.
7. Concevoir le profil de personnage, la structure de sauvegarde et les identifiants stables.
8. Implementer le gestionnaire save/load et le roundtrip minimal d'un personnage.
9. Poser le mecanisme de migration de sauvegardes.
10. Definir les frontieres commandes, events et snapshots pour local host et dedicated futur.
11. Poser un mode `single-player local sim` et un mode `dedicated sim stub`.
12. Creer une scene de boot ville + zone de test pour valider chargement, spawn et transitions.
13. Ajouter les tests de smoke runtime, tests schemas et tests save/load.

## Documentation de soutien

1. Documenter les schemas de donnees et leur versionnage.
2. Documenter le contrat save/load et la strategie de migration.
3. Documenter les invariants runtime et les frontieres moteur/jeu.
4. Documenter les frontieres simulation locale / serveur autoritaire.

## Criteres de sortie

1. Runtime bootable avec contenu de base charge.
2. Persistence fonctionnelle sur un personnage test.
3. Contrats de donnees suffisants pour authoring et tests.
4. Les bases du mode dedicated/MMO sont posees dans le runtime.
