# Etape 06 - Items, loot, economie ville et craft

## Objectif

Livrer la profondeur d'itemisation D2 et les services de ville qui la soutiennent.

## Taches

1. Definir les familles d'items, slots, contraintes d'equipement et types d'usage.
2. Implementer les raretes: normal, magic, rare, set, unique et variantes requises.
3. Poser le systeme d'affixes prefix/suffix et leurs regles de generation.
4. Poser les definitions d'uniques, sets et objets signatures du MVP.
5. Implementer sockets et compatibilites item <-> socketable.
6. Implementer gemmes, runes, joyaux et leurs modificateurs.
7. Implementer les runewords equivalents et leurs conditions de validite.
8. Implementer charms et objets passifs d'inventaire si retenus.
9. Implementer l'or, les prix d'achat/vente et la courbe economique ville/terrain.
10. Implementer vendors, refresh de catalogue, identify, repair et gamble.
11. Implementer le cube equivalent, les recettes MVP et les erreurs de recette.
12. Construire les tables de loot par monstres, elites, coffres, boss et vendors.
13. Ajouter un simulateur de drops/economie pour equilibrage hors runtime manuel.
14. Poser les tests de generation d'objets, roundtrip inventaire et recettes.

## Documentation de soutien

1. Documenter la taxonomie d'items et les regles d'affixes.
2. Documenter les tables de loot, vendors et recettes/crafts.
3. Documenter les flux ville -> terrain -> retour ville.

## Criteres de sortie

1. L'itemisation couvre les boucles majeures D2.
2. Les services de ville critiques sont jouables et comprehensibles.
3. Les tables de contenu economie/loot sont pretes pour equilibrage.
