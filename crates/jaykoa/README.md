# JayKoa — Calendrier universel du COG

Récepteur temporel transversal. Reflète, agrège et orchestre le temps issu des
autres Services (JayRDV, JayFestival).

## Architecture

```text
crates/jaykoa/
├── src/
│   ├── data/        # Modèle domaine (types, repository, migrations)
│   ├── screens/     # Écrans principaux (calendrier, détail, settings)
│   ├── services/    # Services métier (sync, conflits)
│   ├── ui/          # Composants UI egui (organisms, atoms)
│   ├── export/      # Export iCal / partage
│   ├── app.rs       # Application eframe (point d'entrée UI)
│   ├── app_state.rs # État applicatif partagé
│   ├── lib.rs       # Façade publique du crate
│   └── main.rs      # Point d'entrée standalone
└── Cargo.toml
```

## Modes d'exécution

| Mode | Entrée | Description |
|------|--------|-------------|
| **Standalone** | `main.rs` | Lancement autonome via `cargo run -p jaykoa` |
| **Embarqué** | `new_embedded()` | Intégré dans Central comme service enfant |

## Dépendances principales

- **egui / eframe** — Interface graphique immédiate
- **kindmother** — Couche de persistance (WriteIntent, ReadQuery)
- **rusqlite** — Base SQLite locale (bundled)
- **chrono** — Manipulation temporelle (dates, heures, fuseaux)

## Invariants (Document Fondateur §4)

1. JayKoa **ne crée jamais** d'événement externe.
2. JayKoa **ne modifie jamais** un booking.
3. JayKoa **ne calcule aucune** disponibilité.
4. JayKoa **ne décide jamais** du temps.
5. Toute écriture passe par KindMother (`WriteIntent`).
6. Les reflets externes sont strictement en lecture seule.

## Référence

Documentation complète : [`docs/services/JayKoa/`](../../docs/services/JayKoa/)
