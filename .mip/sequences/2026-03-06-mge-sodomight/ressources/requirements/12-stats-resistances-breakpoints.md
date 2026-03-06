# 12 - Stats, resistances et breakpoints

## Systeme de stats coeur

Le modele cible doit couvrir les leviers D2 qui changent reellement la sensation de build:

- niveau personnage
- points de vie
- mana
- stamina equivalent si retenu
- force
- dexterite
- vitalite
- energie
- attack rating equivalent
- defense equivalent
- block chance
- resistances:
  - feu
  - froid
  - foudre
  - poison
  - physique si le design Sodomight l'explicite

## Effets fonctionnels attendus

- force ouvre equipement et augmente certains dommages
- dexterite ouvre equipement, precision et block
- vitalite augmente la robustesse
- energie nourrit les builds casters
- resistances sont obligatoires pour survivre au mid/late game

## Breakpoints a reproduire ou assumer

- attack speed
- cast rate
- hit recovery
- block rate
- run/walk feel

Deux options sont possibles:

1. reproduction stricte type D2
2. interpolation moderne avec tables de paliers simulees

La sequence recommande 2 pour le runtime, tout en preservant des tables de paliers gameplay afin de garder le "feel" de build hunts.

## Etats et caps

- cap de resistances de base
- penalites par difficulte
- caps et overcaps eventuels
- slow / chill / freeze
- curse lowers resist
- crushing blow / open wounds / deadly strike equivalents si l'on copie D2 a fond

## Besoins de debug

- panneau de stats "finales"
- panneau de decomposition:
  - base
  - equipement
  - buffs
  - debuffs
  - difficulty penalty
- verification du palier atteint pour chaque vitesse cle

## Exigences moteur

- formules pur data + code versionne
- snapshot lisible des stats finales
- tests unitaires par formule
- possibilite de rejouer une formule a partir d'un dump de build
