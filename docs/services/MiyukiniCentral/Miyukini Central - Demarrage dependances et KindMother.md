# Miyukini Central — Démarrage, dépendances et KindMother

Documentation du processus de démarrage de Miyukini Central : vérification des dépendances, installation si nécessaire, puis vérification de la présence du service KindMother (KM).

## Ordre des phases au démarrage

1. **Phase 0 — Dépendances** : Central vérifie que les dépendances sont présentes : l’exécutable `kindmother-server` et la bibliothèque `miyuwebway_participant` (client MWS). Si `kindmother-server` est absent, Central construit les deux crates (`miyuwebway_participant` et `kindmother-service`).
2. **Phase 1 — Présence de KM** : Central vérifie que le service KindMother est accessible (déjà en cours d’exécution). Sinon, il le lance puis attend qu’il soit prêt.
3. **Phase 2** : Lancement de l’interface Dioxus.

Si la Phase 0 ou la Phase 1 échoue, Central affiche une erreur et quitte (code 1).

## Phase 0 — Vérification des dépendances

### Dépendances concernées

| Dépendance | Type | Rôle |
|------------|------|------|
| **kindmother-server** | Exécutable (crate `kindmother-service`) | Serveur de persistance lancé par Central si absent. |
| **miyuwebway_participant** | Bibliothèque | Client MWS (Miyukini Webway System) utilisé par Central pour la vue MWS ; construite en même temps que `kindmother-service` lorsque l’exécutable est manquant. |

### Comportement

- Central cherche l’exécutable `kindmother-server` (ou `kindmother-server.exe` sous Windows) dans :
  1. Le même répertoire que l’exécutable Central ;
  2. `target/debug` et `target/release` du répertoire courant (mode développement) ;
  3. Le `PATH` système.
- **Si trouvé** : on passe directement à la Phase 1.
- **Si absent** : Central tente d’installer les dépendances en construisant les crates `miyuwebway_participant` (bibliothèque) et `kindmother-service` (exécutable) :
  1. Détection de la racine du workspace (répertoire contenant un `Cargo.toml` avec `[workspace]`), en remontant depuis le répertoire courant ou depuis le chemin de l’exécutable.
  2. Exécution de `cargo build -p miyuwebway_participant -p kindmother-service` (avec `--release` si Central a été lancé depuis `target/release`).
  3. Nouvelle recherche de l’exécutable `kindmother-server` ; si présent, Phase 0 réussie.

### Conditions pour l’auto-installation

- Central doit être lancé depuis (ou avec un répertoire courant sous) la racine du dépôt Miyukini COG, pour que la racine du workspace soit trouvée.
- `cargo` doit être disponible dans le `PATH`.

### En cas d’échec

- Message d’erreur affiché (console et, sous Windows, encadré dans la console).
- Central quitte avec le code 1.
- Causes typiques : racine du workspace introuvable, `cargo` absent, échec de compilation de `miyuwebway_participant` ou `kindmother-service`.

## Phase 1 — Vérification de la présence de KindMother

### Comportement

- Vérification de l’accessibilité du service KindMother sur l’adresse configurée (par défaut `127.0.0.1:50051`, variable d’environnement `KINDMOTHER_LISTEN_ADDR`).
- **Si le service répond** : considéré comme déjà en cours d’exécution → Phase 1 réussie.
- **Si le service ne répond pas** :
  1. Lancement du processus `kindmother-server` (chemin trouvé en Phase 0).
  2. Attente que le service accepte les connexions (plusieurs tentatives avec délai).
  3. Si le service devient prêt : Phase 1 réussie ; Central garde le handle du processus et l’arrêtera à la fermeture.

### En cas d’échec

- Message d’erreur (exécutable introuvable, échec du spawn, ou service qui ne devient pas prêt dans le délai imparti).
- Central quitte avec le code 1.

## Implémentation (référence)

- **Module** : `apps/central/src/kindmother_launcher.rs`
- **Fonctions exposées** :
  - `ensure_dependencies()` : vérification/installation des dépendances (`kindmother-server` et, si construction, `miyuwebway_participant`).
  - `ensure_kindmother_running()` : vérification/lancement du service KindMother.
- **Point d’entrée** : `apps/central/src/main.rs` (Phase 0 puis Phase 1 avant le lancement de l’UI).

## Résumé

| Phase | Rôle | Si échec |
|-------|------|----------|
| 0 | Présence de `kindmother-server` et cohérence du workspace (`miyuwebway_participant`, `kindmother-service`) ; construction si absent | Exit 1, message d’erreur |
| 1 | Service KindMother accessible ; lancement si besoin | Exit 1, message d’erreur |
| 2 | Interface Central | — |

Central assure ainsi que les dépendances sont présentes (et les installe si possible), puis que KindMother est disponible avant d’afficher l’interface.
