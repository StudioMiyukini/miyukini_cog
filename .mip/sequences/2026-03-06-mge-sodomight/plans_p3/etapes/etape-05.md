# Etape 05 - Stats, classes, skills et combat

## Objectif

Implementer les regles de personnage et de combat qui donnent la profondeur D2 a `Sodomight`.

## Taches

1. Implementer la creation personnage et la selection de classe.
2. Poser les statistiques de base, derivees et modificateurs temporaires.
3. Implementer l'XP, les niveaux et l'allocation de points de stats.
4. Implementer l'allocation de points de competences et les verifications de prerequis.
5. Definir et brancher les arbres de competences classe par classe.
6. Implementer les actions de base: marcher, courir, interagir, attaquer, utiliser une skill.
7. Implementer la pipeline melee: windup, hit frame, verification de cible, resolution degats.
8. Implementer la pipeline distance: projectile, collision, portee, penetration si requise.
9. Implementer la pipeline sorts: cout ressource, cast time, release, effets.
10. Implementer buffs, debuffs, auras et etats temporaires.
11. Implementer invocations, pieges ou equivalents si retenus pour les classes cibles.
12. Integrer armor, block, resistances, hit chance, hit recovery, breakpoints et critiques si retenus.
13. Implementer mort, corpse, retour ville et relance de boucle apres defaite.
14. Ajouter les harness de test sur formules, timings et cas limites combat.

## Documentation de soutien

1. Rediger le registre des formules gameplay.
2. Documenter les classes, arbres et regles de progression.
3. Documenter les timings, breakpoints et signaux de combat.

## Criteres de sortie

1. Le joueur peut creer et faire progresser plusieurs classes.
2. Le combat couvre les archetypes majeurs D2.
3. Les formules critiques sont documentees et testables.
