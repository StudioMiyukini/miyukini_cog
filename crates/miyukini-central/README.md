# Miyukini Central â€” Hub de gestion des Services (MVP)

OpÃ©rateur d'Interface (Strate 7) : vitrine du Registre d'OpÃ©rateurs, catalogue des Services, Mes Services, lancement de Services.

## Lancer le Hub (MVP)

```bash
cargo run -p miyukini-central
```

## Contenu du MVP

- **Catalogue** : Services factices (Calculatrice, Jeu, Traitement de texte, Notes) rÃ©partis en catÃ©gories (Utilitaires, Loisirs, ProductivitÃ©).
- **Mes Services** : Liste des Services activÃ©s ; Â« Ouvrir Â» affiche lâ€™UI du Service dans le panneau central.
- **Services dÃ©mo** :
  - **Calculatrice** : opÃ©rations basiques (+, âˆ’, Ã—, /, C, =).
  - **Jeu** : clics rapides (score).
  - **Traitement de texte** : zone de texte multiligne.
  - **Notes** : liste de notes avec ajout / suppression.

## Documentation conceptuelle

- [Miyukini Conceptual References - Miyukini Central Hub Services](..//..//docs//_index.md)
- [Stack UI egui/eframe](..//..//docs//_index.md)

## Structure du crate

- `src/app.rs` : Application principale (MiyukiniCentralApp), vues (Accueil, Catalogue, Mes Services, ParamÃ¨tres).
- `src/catalog.rs` : Catalogue mock (catÃ©gories, mÃ©tadonnÃ©es des Services factices).
- `src/services/` : UIs des Services dÃ©mo (calculator, game, text_editor, notes).

En production, le catalogue viendrait de Master Butler via BondingBrother ; le lancement serait gouvernÃ© par StrongFather (Mandat de Permission).

