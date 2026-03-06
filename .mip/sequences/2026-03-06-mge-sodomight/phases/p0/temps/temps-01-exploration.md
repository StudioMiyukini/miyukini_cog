# P0 Temps 01 - Exploration et cadrage

## Reformulation

Construire `Miyukini Game Engine` dans `mge/` comme workspace Rust independant afin de produire `Sodomight`, un ARPG dark fantasy tres proche de Diablo II sur sa boucle, sa structure de progression, son itemisation, sa lisibilite isometrique et son rythme de farm, tout en restant installable et lancable depuis `Central`.

## Classification

- Classe retenue : T5
- Justification :
  - nouveau moteur + nouveau jeu
  - backend Rust + rendu proprietaire
  - integration packaging Central/Market
  - documentation exhaustive multi-domaines
  - exigences assets, outils, runtime, UX, securite, ops

## Contraintes explicites utilisateur

- Documentation modulaire et granulaire
- Maximum 400 lignes par fichier
- Monolithe interdit
- `mge/` independant du reste du workspace racine
- Jeux installables et executables dans `Central`
- Backend Rust
- Rendu graphique entierement construit
- Assets et textures produits en interne
- Sources internet autorisees et souhaitees
- Qualite : code propre, clair, maintenable, scalable

## Hypotheses de travail

- `Sodomight` vise d'abord une copie systemique de Diablo II avant tout ecart creatif majeur
- La premiere phase d'execution cherchera la parite systemique D2 sur un contenu borne au camp de depart et a l'Acte 1 complet
- L'integration `Central` cible le mode `Standalone` via manifeste Market, avec vue d'information dans Central et lancement de processus externe
