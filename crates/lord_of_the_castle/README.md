# Lord of the Castle — Miyukini Survivor

**Titre** du service **Miyukini Survivor** : jeu hybride Survivor + Tower Defense.

Le joueur protège le **Château** au centre, construit des **tours** en phase **Préparation**, et affronte des **vagues d'ennemis** en phase **Bataille**.

## Lancer le jeu (standalone)

```bash
cargo run -p lord_of_the_castle
```

## Contrôles (MVP)

- **Déplacement** : touches **W A S D** ou **flèches** (8 directions).
- **Phase Préparation** : bouton **« Lancer la vague »** pour passer en Bataille.
- **Phase Bataille** : le joueur attaque automatiquement à portée ; les ennemis se dirigent vers le Château.

## Documentation

- [Miyukini Survivor - Document Fondateur](../../docs/services/MiyukiniSurvivor/Miyukini%20Survivor%20-%20Document%20Fondateur.md)
- [Miyukini Survivor - Gameplay et Mecaniques](../../docs/services/MiyukiniSurvivor/Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md)
- [Miyukini Survivor - Ecrans et UI](../../docs/services/MiyukiniSurvivor/Miyukini%20Survivor%20-%20Ecrans%20et%20UI.md)

## Implémentation actuelle (MVP)

- Phases **Préparation** / **Bataille** avec passage par bouton.
- **Château** (40×40 px, 50 PV) au centre ; **game over** si 0 PV.
- **Joueur** (10×10 px, 10 PV) : déplacement 8 dir., attaque auto (1 s, 6 px, 1–2 dégâts).
- **Ennemis** (normal / mini-boss / boss) : spawn aux bords, déplacement vers Château (priorité Joueur > Tour > Château), dégâts au contact.
- **Tours** (20×20 px, 100 PV) : portée 80 px, 1 projectile/s, 1 dégât (construction prévue, pas encore en UI).
- Barre haute : vague, ennemis restants, or.

Point d'accès utilisateur canonique : **Miyukini Central**. Ce binaire permet un lancement standalone pour développement et tests.
