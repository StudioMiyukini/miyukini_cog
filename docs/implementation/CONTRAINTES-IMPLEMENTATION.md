# Contraintes d'implémentation COG

<!-- @id impl.contraintes -->
<!-- @role implementation-constraints -->
<!-- @layer 0-9+zones -->
<!-- @human Règles obligatoires pour tout code COG : build, erreurs, sécurité, architecture, données, IA, tests, traçabilité -->
<!-- @do constrain_implementation -->

- **Statut** : v1.0, 2026-10-24 — s'applique à tout code nouveau ou modifié ; C-24 révisée le 2026-10-24 (A-003) et le 2026-10-25 (A-004)
- **Base** : [doc conceptuelle gelée](../architecture/GEL-DOC-CONCEPTUELLE-v1.md) · [menaces EX-01..63](../securite/MODELE-MENACES-COG.md)
- **Règle** : une contrainte non tenue bloque la fusion ; une dérogation est écrite, motivée et datée dans la PR
- **Niveaux** : **[B]** bloquant · **[R]** requis avant la sortie du lot · **[V]** visé

## 1. Build et outillage

<!-- @id impl.contraintes.build -->
<!-- @role build-rules -->
<!-- @do constrain_build -->

- **C-01 [B]** Toute crate hérite des lints du workspace (`[lints] workspace = true`) — aujourd'hui 37 / 149
- **C-02 [B]** `unsafe_code = "forbid"` sans exception (0 bloc `unsafe` mesuré au 2026-10-24 : à conserver)
- **C-03 [B]** `cargo clippy -D warnings` propre sur les crates touchées
- **C-04 [B]** `Cargo.lock` versionné ; `cargo audit` et `cargo deny` (licences, doublons, sources) en CI (EX-15)
- **C-05 [R]** Chaîne Rust épinglée (`rust-toolchain.toml`)
- **C-06 [V]** Compilations reproductibles pour les Cores et services officiels (EX-04)

## 2. Erreurs et robustesse

<!-- @id impl.contraintes.erreurs -->
<!-- @role error-rules -->
<!-- @do constrain_errors -->

- **C-10 [B]** Aucun `unwrap`, `expect`, `panic!`, `todo!` hors tests dans : Kernel, Cores, toolkits sécurité, code exposé au réseau
- **C-11 [B]** Erreurs typées (`thiserror`) ; une variante par cause de refus
- **C-12 [B]** **Fail-closed** : toute erreur de contrôle = refus, jamais d'autorisation par défaut
- **C-13 [B]** Tailles bornées sur toute entrée (octets, éléments, profondeur)
- **C-14 [R]** Pas de `todo!` / `unimplemented!` en chemin d'exécution ; un stub refuse explicitement

## 3. Sécurité

<!-- @id impl.contraintes.securite -->
<!-- @role security-rules -->
<!-- @do constrain_security -->

- **C-20 [B]** Identité dérivée d'un justificatif vérifié, **jamais** d'un champ de la requête (persona, user_id, rôle)
- **C-21 [B]** Exposition désactivée par défaut ; écoute sur `127.0.0.1` sauf contrat signé (EX-48)
- **C-22 [B]** Authentification sur tout point d'accès non local ; comparaisons de secrets en temps constant
- **C-23 [B]** Signature vérifiée **avant** tout décodage de contenu (EX-09)
- **C-24 [B]** Cryptographie limitée à l'inventaire ci-dessous (révisée le 2026-10-24, amendement A-003, NC-ANSSI-09 ; complétée le 2026-10-25, A-004) ; toute primitive ou crate nouvelle = révision de C-24
  - **Règle** : **aucune primitive maison** ; le code de schéma (Shamir GF(256), RS1024 et Feistel de SLIP-39, SLIP-10, Merkle RFC 9162) n'est admis qu'avec les **vecteurs de test officiels** de sa spécification
  - **Inventaire** (algorithme → crate → usage → statut), relevé sur les `Cargo.toml` du workspace :
    - **Ed25519** → `ed25519-dalek` 2, `verify_strict` → `miyusceau` (Sceau, seul format d'identité signée), `miyauth`, `miyuracine` (via `miyusceau`), `kindmother-client`, `kindmother-service`, `miyukini-cog-bridge`, `jaymessage`, `jay-collab`, `miyuwebway_participant`, `miyuportail` (trames de voie, A-004) → autorisé ; hors listes ANSSI, justification à produire (NC-ANSSI-10)
    - **X25519** → `x25519-dalek` 2 → `miyukini-cog-bridge` (pont E2E), `jaymessage`, `crates/miyucloud` (`crypto/e2e.rs`, `sync/transport.rs`) → autorisé, à justifier (NC-ANSSI-10), non post-quantique
    - **XChaCha20-Poly1305** → `chacha20poly1305` 0.10 → `miyukini-cog-bridge` (`e2e.rs`), `jaymessage` (`session.rs`) → autorisé, à justifier
    - **ChaCha20-Poly1305** → `chacha20poly1305` 0.10 → `jaycloud` (`kits/crypto_kit.rs`), `crates/miyucloud` (`crypto/at_rest.rs`, `crypto/e2e.rs`) → autorisé
    - **AES-256-GCM** → `aes-gcm` 0.10 → `apps/miyucloud` (`blob_encryption_service.rs`), `apps/miou-llm-bridge` (`security.rs`) ; déclarée sans usage dans `miyukini-admin` → autorisé en 256 bits seulement (NC-ANSSI-15)
    - **HKDF-SHA256** → `hkdf` 0.12 → `miyauth` (identifiant par paire), `miyukini-cog-bridge`, `jaymessage`, `jaycloud`, `crates/miyucloud` → autorisé
    - **HMAC-SHA256** → `hmac` 0.12 → `miyuracine` (SLIP-10, PBKDF2), `cog-web-portal`, `central-remote`, `origin`, `miou-llm-bridge` → autorisé
    - **PBKDF2-HMAC-SHA256** → `pbkdf2` 0.12 → `miyuracine` (Feistel SLIP-39) → autorisé, réservé à SLIP-39
    - **Argon2id** → `argon2` 0.5 → `kindmother-db-key` (64 Mio, 3 passes, 4 voies), `origin`, `miyukini-central`, `miyukini-connect`, `miyukini-admin`, `miyualicia-api`, `jaycloud`, `apps/miyucloud`, `crates/miyucloud` → autorisé pour les secrets de faible entropie ; paramètres à documenter (NC-ANSSI-17)
    - **SHA-256** → `sha2` 0.10 → `miyusceau` (`kid`, identité des titres, Merkle), `miyauth`, `miyuracine`, `miyumarket` (`package_id`), `miyuportail` (empreintes des passages, A-004), `borderguard` (arrêtés), autres crates → autorisé
    - **BLAKE3** → `blake3` 1 → `apps/miyucloud` (empreintes de contenu), `miyuvoicecapture` (identifiant de périphérique) → **usages non sécuritaires uniquement** (NC-ANSSI-16)
    - **MD5** → `md-5` 0.10 → `apps/miyucloud` (`chunked_upload_service.rs`, ETag) → usages non sécuritaires uniquement
    - **TLS (rustls)** → `rustls` 0.22 (`ring` : `origin`, `rumqttc`) et 0.23 (`ring` **et** `aws-lc-rs` compilés par unification des features : `axum-server`, `hyper-rustls`, `tokio-tungstenite`) ; `rcgen` (`apps/miyucloud`, via `ring`) → autorisé ; `ring` n'offre pas X25519MLKEM768 ; fournisseur unique, version unique et profil TLS à fixer (NC-ANSSI-20)
    - **TLS (OpenSSL)** → `native-tls` / `openssl-sys` via `reqwest` aux features par défaut (`alicia-mvp`, `miyualicia`, `miyualicia-http`) et `imap` (`jaymail`) → **écart**, à ramener sur rustls (NC-ANSSI-20)
    - **SQLCipher** → `rusqlite` feature `bundled-sqlcipher` (libcrypto OpenSSL du système) → bases chiffrées (`kindmother-service`, `kindmother-heritage`, `kindmother-import`, `crates/miyucloud`, `jay1tribu`, `miyukiniwatch`) ; clé dérivée par `kindmother-db-key` → autorisé
    - **Aléa** → `rand_core::OsRng` / `getrandom` (noyau) → partout ; `rand` 0.8 présent : seul `OsRng` admis pour un secret → autorisé
    - **Jetons hérités** → `jsonwebtoken` 9 / 10, HS256 (`miyualicia-api`, `apps/miyucloud`) ; `totp-rs` (`miyukini-connect`, `crates/miyucloud`) → tolérés jusqu'à leur remplacement par passeport et visa ([inventaire](INVENTAIRE-SECRETS-PARTAGES.md))
    - **Utilitaires** → `subtle` (temps constant, C-22), `zeroize` (C-25)
    - **Confinement du sas** (non cryptographique, A-004) → `seccompiler` 0.5 (filtre seccomp-bpf, API sûre) et `rustix` 1 (`process` : rlimits, `no_new_privs`) → `miyuportail` (`bac_a_sable.rs`) → autorisé
  - **Écarts connus** à la règle : HMAC réimplémenté (`jaycloud` `kits/token_kit.rs`, O-21) ; MAC `SHA256(secret ‖ charge)` (`miyucloud-dav`, M-20) → à corriger
- **C-25 [B]** Secrets : `zeroize`, jamais journalisés, jamais dans le dépôt
- **C-26 [R]** Tout décodeur d'entrée externe a une cible de fuzzing (EX-10)
- **C-27 [B]** Moteur de service sans parseur de protocole externe (EX-41) ; décodage dans le Portail
- **C-28 [B]** Contenu non fiable jamais rendu tel quel (EX-23)

## 4. Architecture

<!-- @id impl.contraintes.architecture -->
<!-- @role architecture-rules -->
<!-- @do constrain_architecture -->

- **C-30 [B]** Sens des dépendances : S → C → K ; un Core ne dépend d'aucun service
- **C-31 [B]** Les Cores gouvernent, n'exécutent pas (D1) ; exécution par Douane, Vigies, Kernel
- **C-32 [B]** Services : intentions vers BondingBrother ; décision StrongFather ; persistance KindMother seule
- **C-33 [B]** Zone COG sans route sortante ; flux uniquement par le Portail (voies à sens unique, EX-44)
- **C-34 [R]** Chaque service déclare son contrat (ressources, flux, classe H0–H3) dans un manifeste signé (EX-16, EX-40)
- **C-35 [B]** MiyukiniAdmin en zone COG uniquement (D7)

## 5. Données

<!-- @id impl.contraintes.donnees -->
<!-- @role data-rules -->
<!-- @do constrain_data -->

- **C-40 [B]** Une clé de base par service (EX-21) ; étiquettes de sensibilité KindMother (EX-47)
- **C-41 [R]** Migrations réversibles, base principale versionnée (EX-08)
- **C-42 [B]** Aucun langage de requête exposé par une base secondaire (EX-07)

## 6. IA

<!-- @id impl.contraintes.ia -->
<!-- @role ai-rules -->
<!-- @do constrain_ai -->

- **C-50 [B]** Outils de MAIA appelés par intentions, jamais par commande libre
- **C-51 [B]** Marquage `etranger` propagé ; aucune action sur contenu étranger sans validation (EX-60)
- **C-52 [B]** Aucun apprentissage automatique sur données étrangères (EX-63)
- **C-53 [R]** Modèles reçus par le canal officiel, signés (EX-26)

## 7. Tests

<!-- @id impl.contraintes.tests -->
<!-- @role test-rules -->
<!-- @do constrain_tests -->

- **C-60 [B]** Tout changement est testé ; tout refus de sécurité a son test négatif
- **C-61 [B]** Formats binaires : vecteurs de référence figés (modèle : `miyusceau/tests/vecteurs.rs`)
- **C-62 [R]** Couverture mesurée des crates sécurité ; objectif ≥ 80 % des lignes
- **C-63 [V]** Tests de bout en bout entre deux COG (fédération, Portail)

## 8. Dépendances et licences

<!-- @id impl.contraintes.dependances -->
<!-- @role dependency-rules -->
<!-- @do constrain_dependencies -->

- **C-70 [B]** Nouvelle dépendance justifiée dans la PR ; fonctionnalités minimales
- **C-71 [B]** Licences Apache-2.0, MIT, BSD, ISC, Zlib ; aucun code GPL (IDS/IPS maison en salle blanche)
- **C-72 [R]** Modèles IA : Apache-2.0 ou MIT uniquement

## 9. Traçabilité

<!-- @id impl.contraintes.tracabilite -->
<!-- @role traceability-rules -->
<!-- @do constrain_traceability -->

- **C-80 [B]** Chaque PR cite ses exigences (EX-xx), décisions (Dx) et contraintes (C-xx)
- **C-81 [B]** Bloc MSCM `@id` sur tout nouveau module
- **C-82 [B]** Écart avec la doc gelée = amendement écrit avant le code ([gel](../architecture/GEL-DOC-CONCEPTUELLE-v1.md))
