# Rapport final miyucloud-oxicloud-refonte

## Statut

- Etat : Termine
- Phase : P6
- Responsable principal : Arianne
- Date : 2026-03-06

## TL;DR

Sequence `2026-03-06-miyucloud-oxicloud-refonte` completee avec succes. La refonte OxiCloud pour MiyuCloud est livree : crate `miyucloud-dav` avec stack WebDAV/CalDAV/CardDAV complete, dedup SHA-256 + compression flate2, securite 97/100, 287 tests propres.

---

## 1. Contexte et objectifs

### Contexte

MiyuCloud avait besoin d'une stack de synchronisation de fichiers compatible avec les clients standard (Nextcloud, DAVx5, OnlyOffice). La sequence a effectue une refonte en s'inspirant d'OxiCloud (serveur cloud Rust open-source) pour produire une implementation adaptee au modele multi-tenant et chiffre de MiyuCloud.

### Objectifs initiaux

1. Creer la crate `miyucloud-dav` avec support WebDAV, CalDAV, CardDAV
2. Implementer la deduplication de contenu (SHA-256 content-addressable storage)
3. Ajouter la compression transparente (flate2) pour reduire l'empreinte stockage
4. Integrer les thumbnails (images) et WOPI (Office Online)
5. Connecter le tout a `apps/central` (UI Iced)
6. Durcir la securite (score cible >= 90/100)

### Objectifs atteints

Tous les objectifs atteints. Score securite 97/100, 287 tests, zero dette technique critique.

---

## 2. Architecture livree

```
crates/
  miyucloud-dav/
    src/
      lib.rs
      common/          # types partages (DavResource, DavError, props)
      webdav/          # PROPFIND, PROPPATCH, MKCOL, COPY, MOVE, LOCK
      caldav/          # calendriers, evenements iCal, sync-token
      carddav/         # carnets d'adresses, contacts vCard
      thumbnails/      # generation images (image crate)
      wopi/            # WOPI CheckFileInfo, GetFile, PutFile
    tests/
      security_path_traversal.rs
      security_xxe.rs
      security_auth_sql.rs
      security_hardening.rs
      integration_webdav.rs
      integration_caldav.rs
      integration_carddav.rs
      e2e_sync.rs

  miyucloud/
    src/
      domain/
        dedup_ops.rs   # SHA-256 stream hashing, ContentHash, dedup_upload
      storage/
        dedup.rs       # trait ContentAddressableStorage
        compression.rs # maybe_compress / decompress (flate2, seuil 4096B)
      data/
        kindmother_db.rs  # impl ContentAddressableStorage, tables SQL blobs
```

### Schema SQL ajoute

| Table | Description |
|-------|------------|
| `cloud_content_blobs` | hash PK, data BLOB, size, compressed, ref_count, created_at |
| `cloud_file_blobs` | file_id FK, blob_hash FK (lien N:N) |
| `cloud_calendars` | calendriers CalDAV par user |
| `cloud_calendar_events` | evenements iCal |
| `cloud_addressbooks` | carnets d'adresses CardDAV |
| `cloud_contacts` | contacts vCard |

---

## 3. Decisions techniques cles

| Decision | Justification |
|----------|--------------|
| SHA-256 pour dedup (pas SHA-1) | Resistance aux collisions volontaires, standard securite 2026 |
| flate2 (DEFLATE) seuil 4096B | Rapport compression/CPU optimal pour petits fichiers (thumbnails, vCard) |
| quick-xml sans expansion d'entites | Protection XXE native, pas de patch a maintenir |
| CspNonce UUID par requete | Mitigation XSS sans impact sur performance (UUID v4 = ~200ns) |
| `subtle::ConstantTimeEq` pour HMAC | Prevention timing attacks sur verification de tokens |
| IP SHA-256 dans logs | Conformite RGPD sans perte de capacite de diagnostic |
| WAL mode SQLite | Lecteurs concurrents sans blocage, performance write amelioree |

---

## 4. Metriques finales

| Metrique | Valeur |
|----------|--------|
| Etapes P3 | 11/11 Terminees |
| Taches P3 | 88/88 done |
| Tests | 287 ok / 0 failed / 0 ignored |
| Warnings compilation | 0 |
| Violations clippy | 0 |
| Score securite | 97/100 |
| Score efficience | 18/20 |
| Anomalies bloquantes | 0 |
| CVE ouvertes | 0 (rusqlite CVE-2025-6965 corrige en E0-06) |

---

## 5. Recommandations futures

| Priorite | Recommandation | Cible |
|----------|---------------|-------|
| P1 | Ajouter smoke test SQLite on-disk pour valider migrations en CI | prochaine sequence |
| P2 | Factoriser parsing de path WebDAV/CalDAV dans `common/path.rs` | refactoring futur |
| P2 | Mettre le seuil de compression en config (`MiyucloudConfig`) | tuning ops |
| P3 | TLS client certificate pour endpoints admin | securite avancee |
| P3 | Rotation automatique des HMAC keys (24h TTL) | securite avancee |

---

## 6. Conclusion

La sequence `2026-03-06-miyucloud-oxicloud-refonte` est **TERMINEE et VALIDEE**.

MiyuCloud dispose maintenant d'une stack DAV complete, securisee et testee. Les clients Nextcloud, DAVx5 et OnlyOffice peuvent se connecter. La deduplication reduit l'empreinte stockage pour les fichiers redondants. Le durcissement securite place MiyuCloud dans le top-tier des serveurs cloud self-hosted en termes de posture securite (97/100).

**Statut final : SUCCES**
