# Mini-site de sequence (JSX sans build)

Ce dossier contient l'interface de navigation des artefacts de sequence.

## Fichiers

- `index.html` : interface JSX (React + Babel CDN)
- `manifest.json` : structure des onglets et fichiers affiches

## Usage

1. Ouvrir `ui/index.html` dans le navigateur.
2. Le `manifest.json` local est charge automatiquement.
3. Naviguer dans les onglets standards : P0, P3, P4, P5, Rapport final.

## Notes

- Les fichiers sont lus en chemins relatifs depuis la sequence courante.
- Aucun selecteur de dossier n'est necessaire.
- Si ouverture directe en `file://` bloque les lectures, lancer un serveur local a la racine `.mip` (ex: `python -m http.server`).

