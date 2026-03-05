<!-- @id mem.project.miyucloud
     @do provide_miyucloud_project_decisions
     @role project
     @layer memory
     @human Décisions projet MiyuCloud — architecture, crypto, défauts -->

# MiyuCloud — Cloud privé P2P (confirmé mars 2026)

> Cloud privé auto-hébergé, chiffré, P2P. Chantier T5 livré. Remplace Jay1Tribu pour le partage fichiers (D3).

**3 composants** :
- `crates/miyucloud/` (Strate 7) : Bibliotheque. 36 fichiers, 9104 lignes. Persistance KindMother (12 tables), stockage chiffre chunke, crypto, auth, sync P2P, export ZIP.
- `apps/miyucloud/` (Strate 7) : Serveur standalone dual. API HTTP `127.0.0.1:11440` + surface web HTTPS `0.0.0.0:11442`. Peer discovery UDP `11443`.
- `apps/central/src/services/miyucloud/` : UI Dioxus 0.6. 12 composants (sidebar, explorer, upload, partage, corbeille, sync, settings).

**Crypto** : ChaCha20-Poly1305 at-rest, Argon2id + HKDF derivation, X25519 E2E, canary passphrase. Master key en RAM uniquement.
**Decisions verrouillees** : D1 (P2P pur), D2 (surface web sandboxee), D3 (remplace Jay1Tribu), D4 (chiffrement obligatoire).
**Metriques** : 66 fichiers, 18435 lignes, 257 tests (257 OK), 0 clippy warnings, MSCM 100%, audit securite 87/100.
**Defauts majeurs en attente** : F-01 (download sans session), F-02 (timing attack hash), F-03 (passphrase par defaut). Plans de correction dans `.mip/audits/2026-03-01-miyucloud-audit-securite.md`.
