# Spec technique -- miyucloud-integration-fix

<!-- @id: mip.spec.miyucloud-integration-fix -->
<!-- @do: define_technical_spec_for_integration -->
<!-- @role: spec -->
<!-- @layer: S7-Operator -->

## 1. Perimetre technique

### V0 -- Compilation
- **Cible** : `cargo check -p miyukini-central-native` compile sans erreur
- **Action** : feature-gate ou retrait de `lord_of_the_castle` dans les deps Central
- **Alignement types** : importer directement depuis `miyucloud::data::types` dans Central UI `state.rs`

### V1 -- Integration runtime
- **ServiceManager** : passer les env vars au process MiyuCloud
  - `MIYUCLOUD_COG_TOKEN` : genere a l'install, stocke en DB Central (`auth::db`)
  - `MIYUCLOUD_PASSPHRASE` : derivee depuis la master key Central ou saisie utilisateur
  - `MIYUCLOUD_CENTRAL_DB_PATH` : pointe vers la DB Central automatiquement
- **Client HTTP** : lire le token depuis la DB Central pour construire `MiyuCloudClient`
- **Health probe** : poll `GET /health` apres le spawn, timeout 10s, retry 3x

### V2 -- E2E
- Test fonctionnel via l'UI (non automatise pour cette sequence)
- Checklist manuelle : upload, download, share, TOTP, onboarding

### V3 -- Monitoring
- **`check_disk_space()`** : implementation platform-specific
  - Unix : `libc::statvfs`
  - Windows : `winapi::um::fileapi::GetDiskFreeSpaceExW`
- **Health dashboard** : connecter `HealthStatus` depuis `GET /api/admin/health`

## 2. Fichiers impactes (estimation)

| Fichier | Action |
|---------|--------|
| `apps/central/Cargo.toml` | Retrait/feature-gate `lord_of_the_castle` |
| `apps/central/src/services/miyucloud/state.rs` | Importer types depuis crate |
| `apps/central/src/service_manager/launcher.rs` | Passer env vars MiyuCloud |
| `apps/central/src/services/miyucloud/client.rs` | Lire token depuis DB |
| `apps/central/src/services/miyucloud/mod.rs` | Health probe au demarrage |
| `crates/miyucloud/src/monitoring/mod.rs` | Implementer `check_disk_space()` |

## 3. Contraintes

- Zero regression sur les 37 + 14 tests existants
- Zero clippy warning
- Pas de passphrase en clair dans la DB ou les logs
- Pas d'URL externe en dur (LOI-1)
