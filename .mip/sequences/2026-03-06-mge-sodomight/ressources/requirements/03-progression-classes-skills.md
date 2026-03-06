# 03 - Progression, classes et skills

## Cible de reproduction

`Sodomight` doit viser un roster initial de 7 archetypes jouables, comme Diablo II avec LoD:

- melee lourd
- melee agile
- caster elementaire
- hybride invocations
- hybride aura/support
- distance physique
- hybride pieges/ombre

Les noms et l'IP peuvent diverger, mais la couverture systemique doit rester equivalente.

## Structure de progression de personnage

- niveau personnage
- stats de base:
  - force equivalent
  - dexterite equivalent
  - vitalite equivalent
  - energie equivalent
- points de stat par niveau
- points de skill par niveau / quete
- breakpoints lies a la vitesse d'action et a l'equipement

## Arbre de competences

- 3 arbres par classe
- 10 skills minimum par arbre pour une cible D2-like
- repartition:
  - attacks de base / generators
  - AOE
  - mobilite
  - buff / debuff
  - summon / utility
  - one-point wonders

## Regles de design a reproduire

- prerequis entre skills
- synergies numeriques entre skills
- couts en mana / resource calculables
- skill lvl + skill tree bonus + all skills bonus
- distinction forte entre build principal et support kit

## Besoins Sodomight

- data model pour classes, trees, skills, synergies, prerequisites, animations, tags de damage
- support des skills canalises, projectiles, cones, novas, auras, totems/pieges, invocations
- support des states:
  - stun
  - chill / freeze
  - poison
  - curse
  - aura
  - shield / absorb

## Progression secondaire a conserver

- mercenaire avec level scaling, equipement et role de build
- quetes donnant stats ou skills permanents
- respec encadre, pas gratuit a l'infini dans la cible D2-like
- progression de stash et de craft via economy, pas via battle pass ou meta soft

## Exigences moteur

- runtime de skill totalement data-driven
- calcul de degats separant:
  - base skill
  - weapon contribution
  - stat scaling
  - resistance
  - critical-like modifiers
  - on-hit / on-kill procs
- systeme de tags pour immunites et resist reductions
- systeme de formule versionne pour pouvoir corriger sans casser les saves
