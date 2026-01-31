# Miyukini Central — Hub de gestion des Services (MVP)

Opérateur d'Interface (Strate 7) : vitrine du Registre d'Opérateurs, catalogue des Services, Mes Services, lancement de Services.

## Lancer le Hub (MVP)

```bash
cargo run -p miyukini-central
```

## Contenu du MVP

- **Catalogue** : Services factices (Calculatrice, Jeu, Traitement de texte, Notes) répartis en catégories (Utilitaires, Loisirs, Productivité).
- **Mes Services** : Liste des Services activés ; « Ouvrir » affiche l’UI du Service dans le panneau central.
- **Services démo** :
  - **Calculatrice** : opérations basiques (+, −, ×, /, C, =).
  - **Jeu** : clics rapides (score).
  - **Traitement de texte** : zone de texte multiligne.
  - **Notes** : liste de notes avec ajout / suppression.

## Documentation conceptuelle

- [Miyukini Conceptual References - Miyukini Central Hub Services](../../docs/reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md)
- [Stack UI egui/eframe](../../docs/ux_ui/Miyukini%20-%20Stack%20UI%20egui%20eframe.md)

## Structure du crate

- `src/app.rs` : Application principale (MiyukiniCentralApp), vues (Accueil, Catalogue, Mes Services, Paramètres).
- `src/catalog.rs` : Catalogue mock (catégories, métadonnées des Services factices).
- `src/services/` : UIs des Services démo (calculator, game, text_editor, notes).

En production, le catalogue viendrait de Master Butler via BondingBrother ; le lancement serait gouverné par StrongFather (Mandat de Permission).
