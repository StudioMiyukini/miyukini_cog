# Assets UI - Miyukini Central

Ce dossier contient les assets UI pour Miyukini Central.

## Structure

```
assets/
├── pixel_ui/          # Sprites pixel art (PNG)
│   ├── tabs/          # Sprites d'onglets
│   ├── buttons/       # Sprites de boutons
│   └── icons/         # Icônes pixel art
├── css_ui/            # Références CSS (chrome-tabs, etc.)
│   └── chrome-tabs/   # CSS chrome-tabs pour référence
└── README.md          # Ce fichier
```

## Sources

### Pixel Art
- **Kenney's Pixel UI Pack** (CC0) - https://kenney.nl/assets/pixel-ui-pack
- À télécharger et placer dans `pixel_ui/`

### CSS
- **chrome-tabs** (MIT) - https://github.com/adamschwartz/chrome-tabs
- À cloner dans `css_ui/chrome-tabs/`

## Utilisation

Les assets pixel art sont chargés dans egui via `egui::TextureHandle`.
Les CSS sont utilisés comme référence pour adapter le style dans egui.
