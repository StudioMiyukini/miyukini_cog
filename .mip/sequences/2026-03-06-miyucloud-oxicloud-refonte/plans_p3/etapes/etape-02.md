# E2 -- Dedup & Compression

## Statut : Termine
## Depend de : E1
## Agents : Francois, Lise
## Taches : 8

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E2-01 | CODE | Impl ContentAddressableStorage pour SQLite (store_blob, read_blob) | Francois | crates/miyucloud/src/data/kindmother_db.rs | done |
| E2-02 | CODE | Impl increment_refcount, decrement_refcount, blob_exists | Francois | crates/miyucloud/src/data/kindmother_db.rs | done |
| E2-03 | CODE | SHA-256 hashing pipeline (stream + finalize) | Francois | crates/miyucloud/src/domain/dedup_ops.rs | done |
| E2-04 | CODE | Compression flate2 pour blobs > seuil configurable | Francois | crates/miyucloud/src/storage/compression.rs | done |
| E2-05 | CODE | Integration dedup dans upload flow existant | Francois | crates/miyucloud/src/domain/file_ops.rs | done |
| E2-06 | TEST-U | Tests store/read blob, refcount, hash collision | Lise | crates/miyucloud/src/data/kindmother_db.rs | done |
| E2-07 | TEST-U | Tests compression/decompression round-trip | Lise | crates/miyucloud/src/storage/compression.rs | done |
| E2-08 | TEST-I | Test integration: upload fichier duplique, verifier dedup | Lise | crates/miyucloud/src/data/kindmother_db.rs | done |

## Commit message template
`feat(miyucloud): E2 -- dedup SHA-256 content-addressable + compression flate2`
