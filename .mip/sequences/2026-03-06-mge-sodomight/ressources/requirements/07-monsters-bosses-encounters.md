# 07 - Monstres, bosses, encounters

## Taxonomie ennemie

- trash melee
- ranged kiting
- tank / shield
- support aura
- summon master
- kamikaze / corpse explosion
- assassin mobile
- elite pack leader
- boss scenario

## Variantes requises

- normal
- champion
- unique
- super unique
- boss d'acte

## Proprietes obligatoires

- famille
- AI archetype
- damage profile
- resistances / immunites
- loot table
- move speed
- attack speed
- aggro radius
- leash logic
- on-death effect
- summon list eventuel

## Rencontres typiques D2 a reproduire

- couloir de pression avec ranged et blockers
- salle de burst melee
- elites avec affixes dangereux
- packs mixes cassant les mauvaises resistances
- boss de quete avec arena, adds et pattern simple mais lethal

## Boss design

- boss lisible a la D2:
  - peu de phases
  - pattern courts mais durs
  - arena modifiee par adds ou hazards
- modernisations autorisees:
  - telegraphes plus propres
  - meilleure camera readability

## Affixes et modificateurs

- extra fast
- cursed
- magic resistant
- mana burn equivalent
- teleport / blink
- extra strong
- multishot equivalent
- aura enchanted equivalent

## Exigences moteur

- AI data-driven a base de FSM ou utility simple
- support elites + affixes composees
- outils de simulation de packs et heatmaps de lethalite
- debug des immunites et resist breaks
