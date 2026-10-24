# MiyuCloud — index

<!-- @id services.miyucloud.index -->
<!-- @role service-index -->
<!-- @layer S -->
<!-- @human Cloud privé auto-hébergé : fichiers, calendriers, contacts ; distinct de JayCloud (sauvegarde) -->
<!-- @do navigate_miyucloud -->

- **Rôle** : cloud privé auto-hébergé — fichiers, calendriers (CalDAV), contacts (CardDAV), édition (WOPI)
- **≠ JayCloud** : JayCloud fait la **sauvegarde** chiffrée (snapshots, restauration) ; les deux services sont conservés (décision du 2026-10-24) — [JayCloud](../JayCloud/_index.md)
- **Code**
  - `apps/miyucloud` (binaire `miyucloud-server`, ≈ 69 000 lignes, licence MIT, README et CI propres)
  - `crates/miyucloud` (bibliothèque) · `crates/miyucloud-dav` (WebDAV, CalDAV, CardDAV, WOPI)
- **Exposition** : surface HTTP la plus large du COG (WebDAV, compatibilité Nextcloud) — priorité P0 de la [revue](../../implementation/REVUE-CODE-PREPARATION.md)
- **Cible** : service hybride H2 (moteur en zone COG, façade en Franche) — [services hybrides](../../architecture/SERVICES-HYBRIDES-ET-PORTAIL.md)
- **Doc détaillée** : `apps/miyucloud/README.md`, `apps/miyucloud/doc/`
