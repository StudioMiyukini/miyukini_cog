# HyperUI - Reference Composants Locale

## Contexte

Ce dossier contient une copie locale des composants [HyperUI](https://github.com/markmead/hyperui) (Tailwind CSS v4) pour les utiliser sans dépendre du site hyperui.dev. Source : [markmead/hyperui](https://github.com/markmead/hyperui), licence MIT.

## Portee / Scope

- **Composants** : 172+ snippets HTML (Application, Marketing, Neobrutalism) dans `components/`.
- **Assets** : `assets/component.css` (build Tailwind v4), `assets/component.js` (scripts optionnels).
- **Metadata** : descriptions et titres dans `metadata/` (application, marketing, neobrutalism).

## Structure

```
hyperui/
├── assets/           # component.css, component.js (Tailwind v4)
├── components/       # Snippets HTML par categorie
│   ├── application/  # Accordions, Badges, Breadcrumbs, Inputs, Modals, Tables, etc.
│   ├── marketing/    # Buttons, Cards, Headers, Footers, CTAs, etc.
│   └── neobrutalism/ # Style neobrutalism (Accordions, Alerts, Buttons, etc.)
├── metadata/         # Fichiers MDX (titre, description) par composant
└── HyperUI - Reference Composants Locale.md  # Ce fichier
```

## Utilisation locale

### Option 1 : Copier-coller dans votre projet Tailwind

1. Ouvrir le fichier HTML du composant (ex. `components/application/accordions/1.html`).
2. Copier le contenu du `<body>` (sans les balises `<body>` si vous l’integre dans une page existante).
3. Coller dans votre projet (HTML, JSX, Vue, etc.) en conservant les classes Tailwind.
4. Utiliser Tailwind CSS v4 (ou v3) dans votre projet ; les classes sont standard.

### Option 2 : Previsualiser un fichier HTML complet

Les fichiers `.html` reference `/component.css` et `/component.js`. Pour previsualiser :

- Soit servir ce dossier a la racine d’un serveur local et adapter les chemins (ex. base href).
- Soit ouvrir le fichier et remplacer temporairement `/component.css` par le chemin vers `assets/component.css` (ex. `../assets/component.css` selon l’emplacement du fichier).

### Option 3 : Inclure le CSS HyperUI dans votre build

- `assets/component.css` est un build Tailwind v4 complet. Vous pouvez le referencer en lien pour des maquettes ou prototypes, ou vous en inspirer pour votre propre theme Tailwind.

## Index des composants

Voir le fichier **HyperUI - Index Composants.md** dans ce dossier pour la liste complete par categorie.

## Licence

Composants HyperUI : MIT. Consulter le depot source pour le texte exact.
