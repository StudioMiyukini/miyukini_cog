# 17 - Content authoring et generation d'assets

## Regle produit

Les assets et textures doivent etre produits en interne pour `Sodomight`. Le moteur doit donc integrer un pipeline de fabrication et de validation d'assets, pas seulement un loader final.

## Regle creative

- inspiration D2 autorisee sur les principes de lisibilite, de categorisation et de rythme visuel
- aucune extraction, aucune retouche, aucun tracing d'asset D2
- chaque famille d'assets doit avoir sa bible de style propre `Sodomight`

## Families d'assets

- tiles terrain
- props
- personnages
- monstres
- projectiles
- VFX
- UI frames
- icons items / skills
- portraits / splash
- audio

## Pipeline cible

1. concept
2. production source
3. validation naming
4. bake atlas / metadata
5. previsualisation
6. publication dans data pack

## Metadonnees par asset

- id stable
- categorie
- dimensions source
- pivot
- collision hint
- tags palette
- licence interne
- version

## Animation

- directions requises
- cadence par clip
- events embarques
- dependances SFX/VFX

## Outils necessaires

- baker atlas
- previewer clips
- linter naming
- thumbnail generator
- validateur palette/tailles
- validateur originalite/style guide
- pipeline de blockout -> rendu source -> paintover -> sheet finale

## Exigences moteur

- aucun asset ne doit etre charge sans metadata valide
- build assets reproductible
- hashes pour detecter mismatch client/package
- variations couleur/equipement pilotables par data pour reduire la dette de production
