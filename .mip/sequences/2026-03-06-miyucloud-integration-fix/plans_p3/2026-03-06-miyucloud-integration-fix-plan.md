# Plan P3 -- miyucloud-integration-fix

<!-- @id: mip.plan.miyucloud-integration-fix -->

## Vagues et taches

### V0 -- Debloquer compilation (CRITIQUE)

| ID | Tache | Fichiers | Agent |
|----|-------|----------|-------|
| T01 | Resoudre dep `lord_of_the_castle` bloquant le build Central | `apps/central/Cargo.toml` | Francois |
| T02 | Verifier compilation des 14 fichiers UI miyucloud dans le workspace | `apps/central/src/services/miyucloud/*` | Francois |
| T03 | Aligner types `state.rs` <-> `crates/miyucloud/data/types.rs` | `apps/central/src/services/miyucloud/state.rs` | Francois |

**Gate V0** : `cargo check -p miyukini-central-native` OK

### V1 -- Integration Central <-> Server (CRITIQUE)

| ID | Tache | Fichiers | Agent |
|----|-------|----------|-------|
| T04 | ServiceManager passe env vars MiyuCloud au lancement | `apps/central/src/service_manager/launcher.rs` | Francois |
| T05 | Stocker/lire le COG token en DB Central | `crates/miyukini-central/src/auth/db.rs`, `client.rs` | Francois |
| T06 | Configurer `central_db_path` automatiquement | `launcher.rs` | Francois |
| T07 | Health check GET /health au demarrage du service | `apps/central/src/services/miyucloud/mod.rs` | Francois |
| T08 | Valider flow complet : Central -> MiyuCloud -> API -> UI | integration test | Francois |

**Gate V1** : MiyuCloud demarre depuis Central, `GET /health` repond OK, UI affiche le contenu

### V2 -- Test E2E cycle complet (HAUTE)

| ID | Tache | Fichiers | Agent |
|----|-------|----------|-------|
| T09 | Upload fichier via UI -> verifier en DB + stockage chiffre | test manuel | Lise |
| T10 | Creer partage -> acceder via surface web HTTPS | test manuel | Lise |
| T11 | Tester TOTP setup via UI -> verifier en DB | test manuel | Lise |
| T12 | Tester onboarding wizard -> verifier completion | test manuel | Lise |

**Gate V2** : Tous les scenarios E2E fonctionnels

### V3 -- Monitoring + polish (MOYENNE)

| ID | Tache | Fichiers | Agent |
|----|-------|----------|-------|
| T13 | Implementer `check_disk_space()` Windows + Unix | `crates/miyucloud/src/monitoring/mod.rs` | Francois |
| T14 | Connecter health dashboard UI aux donnees reelles | `auth_security.rs`, `client.rs` | Lise |
| T15 | Verifier purge rate_limiter en conditions de charge | `rate_limiter.rs` | Victor |

**Gate V3** : Health dashboard affiche des valeurs reelles

### V4 -- Documentation + cloture (STANDARD)

| ID | Tache | Fichiers | Agent |
|----|-------|----------|-------|
| T16 | Mettre a jour MSCM nouveaux fichiers | annotations in-code | Denis |
| T17 | Mettre a jour memoire projet `project-miyucloud.md` | `.mip/memory/project-miyucloud.md` | Denis |
| T18 | Gate P4 (audit) + P5 (test humain) + P6 (cloture) | traces phases | Maria + George |

**Gate V4** : Sequence close, memoire a jour

## DAG

```
V0 -> V1 -> V2 -> V3 -> V4
```
