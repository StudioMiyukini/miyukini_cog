# Etape 08 - Monstres, boss, mercenaire et modes meta D2

## Objectif

Couvrir les systemes D2 transverses qui doivent exister des le MVP meme quand leur expression complete depasse l'Acte 1.

## Taches

1. Definir le roster monstres Acte 1 par famille, biome, niveau et role tactique.
2. Poser les variantes elites, champions, immunites et modificateurs de packs.
3. Implementer les IA melee, ranged, caster, summoner et fuite si necessaire.
4. Implementer les scripts de boss, mini-boss et rencontres speciales.
5. Implementer le mercenaire: recrutement, equipement, mort, resurrection, progression.
6. Implementer le mode hardcore: creation, drapeaux, mort irreversible et protections save.
7. Implementer le mode party/co-op: creation de partie, join, sync progression et partage XP.
8. Implementer le PvP volontaire, ses regles minimales et les garde-fous de grief.
9. Implementer ladder runtime: data model, classement, reset policy, affichage minimum.
10. Poser `mge-proto`, la taxonomie commandes/events/snapshots et le versioning reseau.
11. Poser `mge-replication`, les snapshots, deltas et l'interest management de base.
12. Poser les services boundaries MMO-ready: gateway, realm, zone, social, persistence.
13. Prevoir les harness/debug scenes pour valider les features non naturellement debloquees en Acte 1.
14. Ajouter les tests local host vs dedicated sim sur scenarios critiques.

## Documentation de soutien

1. Documenter le roster monstres, elites, boss et scripts d'IA.
2. Documenter les contrats mercenaire, party, PvP, hardcore et ladder.
3. Documenter les scenes de validation et strategies de test des features meta.
4. Documenter la cible de backend MMO et ses frontieres avec le MVP solo/coop.

## Criteres de sortie

1. Les monstres et boss Acte 1 couvrent la difficulte voulue.
2. Les modes meta D2 cibles sont testables et traces.
3. Le backend est preparable vers MMO sans casser le coeur gameplay.
4. Aucun systeme majeur D2 n'est laisse implicite ou non documente.
