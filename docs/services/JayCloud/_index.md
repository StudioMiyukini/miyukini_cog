# JayCloud — index

<!-- @id services.jaycloud.index -->
<!-- @role service-index -->
<!-- @layer S -->
<!-- @human Service de sauvegarde souveraine du COG ; distinct de MiyuCloud (cloud privé) -->
<!-- @do navigate_jaycloud -->

- **Rôle** : sauvegarde cloud souveraine — backup chiffré, snapshots complets et incrémentaux, restauration sélective, liens publics signés, export WebDAV (rclone, restic, Duplicati)
- **≠ MiyuCloud** : MiyuCloud est le **cloud privé** (fichiers, CalDAV, CardDAV) ; JayCloud ne le remplace pas (décision du 2026-10-24) — [MiyuCloud](../MiyuCloud/_index.md)
- **Code** : `crates/jaycloud` (128 tests), `crates/jaycloud-client`, `crates/jaycloud-migrate` (import MiyuCloud optionnel)
- **Restes du plan de remplacement abandonné** (code, à traiter au lot 0) : `miyucloud_transition_days` dans `crates/jaycloud/src/config.rs`, copie de code MiyuCloud dans `crates/jaycloud/src/files/`
- **Documents**
  - [Document fondateur](JayCloud%20-%20Document%20Fondateur.md)
  - [Spec MSCM / MIP conformité](JayCloud%20-%20Spec%20MSCM%20MIP%20Conformite.md)
