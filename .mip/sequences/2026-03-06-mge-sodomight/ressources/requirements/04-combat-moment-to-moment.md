# 04 - Combat moment-to-moment

## Principes

- combat temps reel
- haute mortalite si mauvaise lecture
- controle par souris + clavier
- priorite a la lisibilite sur la sophistication visuelle

## Sous-systemes obligatoires

### Deplacement

- click-to-move
- follow target
- collision sur grille logique ou volumes equivalentes
- stamina equivalent pour tension debut de partie si souhaite
- teleport / dash / leap selon skills

### Attaque et cast

- animation state machines
- cancel windows volontaires ou non selon skill
- wind-up, active frames, recovery
- speed modifiers:
  - attack speed
  - cast speed
  - hit recovery
  - block rate

### Defense

- armor / defense equivalent
- chance to hit equivalent si cible D2-like stricte
- block rate
- dodge / evade specifique a certains kits
- resistances elementaires et physiques

### Consommables

- potions vie
- potions mana
- potions utilitaires
- rejuv equivalent
- ceinture a slots accessibles en combat

### Feedback

- hit flash
- damage numbers optionnels
- son d'impact distinct par material et degat
- telegraphes elites et boss lisibles
- corpse state pour loot et necromancy-like skills

## Combat loops a couvrir

- melee mono-cible
- ranged projectile
- AOE point-target
- cone / nova
- summon swarm
- trap placement
- aura maintenance
- corpse interaction

## Besoins Sodomight

- preservation du "feel" D2:
  - inertie courte
  - reponse nette
  - importance des breakpoints
  - hit recovery visible
- modernisations acceptees:
  - 60+ fps rendu
  - interpolation fluide
  - options de visibilite

## Exigences moteur

- tick logique fixe
- couche animation independante du rendu
- projectile system haute cadence
- broadphase collision efficace
- debug overlay:
  - hitboxes
  - aggro radius
  - state buffs
  - DPS timeline
