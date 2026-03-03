# Audit de securite P4 -- Sodomight MVP (Sprint 3)

**Date** : 2026-03-03
**Auditeur** : Victor (Expert Cybersecurite, Miyukini AI Studio)
**Perimetre** : mge-net, mge-script, mge-save, mge-arpg-trade, sodomight/boss.rs
**Build** : 1194 tests, 0 clippy warning, build clean
**Branche** : feat/sodomight-mvp

## TL;DR

Score global : **88/100**. Le code audite est solide pour un MVP de jeu solo/listen-server.
Les protections SEC implementees (SEC-03 a SEC-22) sont fonctionnelles et bien testees.
4 defauts non-bloquants identifies : FNV-1a faible pour la verification de scripts,
CRC32 non-cryptographique pour l'integrite reseau, 1 `unwrap_or("unknown")` residuel
dans engine.rs production, et absence de `ChatMessage.text` sanitization cote serveur.
Aucun defaut bloquant. Aucune certification obligatoire applicable (jeu video).

---

## 1. Threat Model

| Surface | Scenario | Impact | Mitigation | Statut |
|---------|----------|--------|------------|--------|
| TCP/JSON protocol | Message oversized (DoS) | Eleve | SEC-16: 64 KiB max, FrameCodec reject | OK |
| TCP/JSON protocol | Message corruption | Moyen | SEC-20: CRC32 integrity check | OK (voir S-02) |
| TCP/JSON protocol | Direction spoofing | Eleve | `is_invalid_client_message()` guard | OK |
| Player movement | Speed hack | Eleve | SEC-03: MoveValidator rate limit + speed check | OK |
| Login/Auth | Brute force | Eleve | SEC-03: MoveValidator rate limit (move scope) | PARTIEL (voir S-05) |
| Rhai scripts | Infinite loop / DoS | Critique | SEC-08: max_operations 50K, max_call_levels 32 | OK |
| Rhai scripts | Eval injection | Critique | SEC-08: `eval` symbol disabled | OK |
| Rhai scripts | Resource exhaustion | Eleve | SEC-08: max_string 4096, max_array 1024, max_map 256 | OK |
| Rhai scripts | Tampered script files | Eleve | SEC-09: FNV-1a hash verification | OK (voir S-01) |
| Rhai scripts | XP/item inflation | Eleve | SEC-18: reward_xp cap 100K, reward_item cap 100 | OK |
| Character names | XSS / injection | Moyen | SEC-17: validate_name whitelist + HTML filter | OK |
| Character data | Stat overflow | Moyen | validate_character: level [1,99], stats [0,1023] | OK |
| Gold wallet | Negative gold exploit | Critique | SEC-19: add_gold/remove_gold reject negative | OK |
| Gold wallet | Overflow exploit | Eleve | GOLD_MAX cap 2.5B, overflow check | OK |
| NPC vendor | Client-defined price | Critique | SEC-21: price from VendorItem.price_sell, not client | OK |
| P2P trade | Item duplication | Critique | Atomic BEGIN/COMMIT/ROLLBACK with snapshot | OK |
| P2P trade | Insufficient gold cheat | Eleve | Validation before transfer, rollback on fail | OK |
| Save files | Corruption on crash | Moyen | SEC-22: backup_before_session file copy | OK |
| DB access | SQL injection | Critique | Parametrized queries (rusqlite params![]) | OK |
| Password storage | Hash cracking | Critique | Argon2id with OsRng salt | OK |
| Chat messages | HTML/script injection | Moyen | Non valide cote serveur | DEFAUT (voir S-04) |
| Boss combat | Negative damage | Faible | `saturating_sub` on HP, u32 damage type | OK |

## 2. Audit des dependances

| Dependance | Utilisation | Risque connu | Statut |
|------------|-------------|-------------|--------|
| rhai | Script engine | Aucune CVE connue, activement maintenu | OK |
| rusqlite | SQLite wrapper | Aucune CVE connue, activement maintenu | OK |
| argon2 | Password hashing | Aucune CVE connue, algorithme approuve (Argon2id) | OK |
| password-hash | PHC format parser | Aucune CVE connue | OK |
| serde / serde_json | Serialization | Aucune CVE critique connue | OK |
| uuid | UUID v4 generation | Aucune CVE connue | OK |
| chrono | Timestamps | Aucune CVE critique recente | OK |
| thiserror | Error derive | Aucune CVE connue | OK |
| tokio | Async runtime (mge-net) | Activement maintenu | OK |
| tracing | Logging | Aucune CVE connue | OK |
| rand / rand_chacha | RNG (sodomight) | Aucune CVE connue | OK |

**Recommandation** : Executer `cargo audit` regulierement. Aucune alerte au moment de l'audit.

## 3. Scan du code

### 3.1. unsafe_code

- [x] `unsafe_code = "forbid"` dans `[workspace.lints.rust]` (mge/Cargo.toml ligne 60)
- [x] Tous les crates utilisent `[lints] workspace = true` -- herite du forbid
- [x] `mge-arpg-trade/src/lib.rs` ajoute `#![deny(unsafe_code)]` en plus (doublon inoffensif)
- **Verdict** : CONFORME. Aucun code unsafe possible.

### 3.2. unwrap() en production

- [x] mge-net/src/codec.rs : 0 unwrap
- [x] mge-net/src/config.rs : 0 unwrap
- [x] mge-net/src/message.rs : 0 unwrap
- [x] mge-net/src/client_id.rs : 0 unwrap
- [x] mge-net/src/tests.rs : unwrap dans #[cfg(test)] uniquement -- CONFORME
- [ ] mge-script/src/engine.rs ligne 134 : `.unwrap_or("unknown")` -- non-critique (fallback safe)
- [x] mge-save/src/characters.rs : unwrap dans #[cfg(test)] uniquement
- [x] mge-save/src/db.rs : unwrap dans #[cfg(test)] uniquement
- [x] mge-arpg-trade/src/vendor.rs : `unwrap_or(i64::MAX)` safe fallback, unwrap dans tests
- [x] mge-arpg-trade/src/wallet.rs : unwrap dans #[cfg(test)] uniquement
- [x] sodomight/src/boss.rs : `unwrap_or(0)` safe fallback, 0 unwrap
- **Verdict** : CONFORME. Aucun `unwrap()` susceptible de panique en production.

### 3.3. Secrets hardcodes

- [x] Aucune URL en dur (http:// ou https://) dans le perimetre audite
- [x] Aucune passphrase, token ou cle en dur
- [x] `password_hash` en tests utilise des placeholders ("h", "$2b$...")
- [x] Argon2id utilise `OsRng` pour le sel (CSPRNG)
- **Verdict** : CONFORME.

### 3.4. Validation des entrees

- [x] SEC-17 : `validate_name()` whitelist alphanumerique + espace/tiret/underscore, HTML interdit
- [x] SEC-17 : longueur [2, 24], pas d'espaces consecutifs, trim automatique
- [x] `validate_character()` : level [1, 99], stats [0, 1023], gold [0, 2.5B]
- [x] SEC-16 : `FrameCodec.decode()` rejette payload > max_message_size
- [x] SEC-19 : `add_gold(-X)` et `remove_gold(-X)` rejetes
- [x] SEC-21 : prix de vente lu depuis `VendorItem.price_sell`, pas depuis le client
- [x] `is_invalid_client_message()` detecte les messages ServerMessage envoyes par un client
- [ ] `ClientMessage::Chat { text }` : pas de validation/sanitization du contenu -- voir S-04
- **Verdict** : BON. 1 defaut mineur (chat text non sanitize).

### 3.5. Chiffrement et integrite

- [x] SEC-20 : CRC32 IEEE avec LUT compile-time (codec.rs lignes 89-118)
- [x] Argon2id pour le hashage de mots de passe (algorithme approuve)
- [x] OsRng pour la generation de sel (CSPRNG)
- [ ] SEC-09 : FNV-1a 64-bit pour la verification de scripts -- voir S-01
- [ ] CRC32 n'est pas un MAC cryptographique -- voir S-02
- **Verdict** : ACCEPTABLE pour MVP. FNV-1a et CRC32 sont suffisants pour le contexte listen-server.

### 3.6. Logging securite

- [x] `tracing::info!` / `tracing::warn!` / `tracing::error!` utilises dans le script engine
- [x] Rhai API expose `log_info` et `log_warn` aux scripts
- [x] Erreurs de compilation de scripts tracees via `tracing::error!`
- [ ] Pas de logging des tentatives de speed hack ou rate limit -- amelioration future
- **Verdict** : BASIQUE. Suffisant pour MVP, a renforcer pour la production.

### 3.7. Rate limiting

- [x] SEC-03 : `MoveValidator` dans sodomight-server/validation.rs
- [x] Speed check : `dx.abs() > max_speed * dt * tolerance` -> SpeedHack
- [x] Rate limit : 25 moves/second/player avec fenetre glissante 1s
- [x] Fenetre reset apres 1000ms via `saturating_sub`
- [x] Tests couvrent : normal, speed hack, rate limit, tolerance, window reset
- **Verdict** : CONFORME pour le scope move. Auth rate limiting hors scope MVP.

### 3.8. Script sandboxing

- [x] SEC-08 : `max_operations(50_000)` -- anti-boucle infinie
- [x] SEC-08 : `max_call_levels(32)` -- anti-recursion profonde
- [x] SEC-08 : `max_string_size(4096)` -- anti-allocation memoire
- [x] SEC-08 : `max_array_size(1024)` -- anti-allocation memoire
- [x] SEC-08 : `max_map_size(256)` -- anti-allocation memoire
- [x] SEC-08 : `max_expr_depths(32, 32)` -- anti-expression profonde
- [x] SEC-08 : `disable_symbol("eval")` -- anti-injection eval
- [x] SEC-18 : `reward_xp` plafonne a 100_000
- [x] SEC-18 : `reward_item` plafonne a 100 unites
- **Verdict** : CONFORME. Sandboxing robuste pour un game engine Rhai.

### 3.9. Comparaison de secrets

- [x] Argon2id `verify_password` utilise une comparaison interne resistante au timing
- [x] CRC32 compare avec `!=` (non-secret, donnees publiques -- acceptable)
- [x] FNV-1a compare avec `!=` (non-secret, hash de fichier -- acceptable dans ce contexte)
- **Verdict** : CONFORME. Pas de comparaison de secret via `==` en dehors de crypto.

## 4. Tests de securite executes

| Test | Resultat | Details |
|------|----------|---------|
| SEC-16 : message > 64 KiB | PASSE | `net_max_message_64k` : codec rejette frame 65537 bytes |
| SEC-17 : nom HTML injection | PASSE | `name_html_inject` : `<`, `>`, `&`, `"`, `'` rejetes |
| SEC-17 : nom trop court/long | PASSE | `name_too_short`, `name_too_long` : bornes [2, 24] |
| SEC-17 : espaces consecutifs | PASSE | `name_consecutive_spaces` : rejete |
| SEC-18 : XP cap | PASSE | reward_xp plafonne a 100_000 (code engine.rs:266) |
| SEC-18 : item qty cap | PASSE | reward_item plafonne a 100 (code engine.rs:257) |
| SEC-19 : gold negatif add | PASSE | `gold_i64_negative_add_rejected` |
| SEC-19 : gold negatif remove | PASSE | `gold_i64_negative_remove_rejected` |
| SEC-20 : CRC32 roundtrip | PASSE | `codec_crc32_roundtrip` |
| SEC-20 : CRC32 tampered | PASSE | `codec_crc32_tampered` : detection corruption |
| SEC-20 : CRC32 buffer court | PASSE | `codec_crc32_too_short_buffer` |
| SEC-21 : prix vendor def | PASSE | `vendor_sell_price_from_def` : prix = VendorItem.price_sell |
| SEC-21 : fallback /4 | PASSE | `vendor_sell_fallback_div4` : 200/4 = 50 |
| SEC-22 : DB backup | PASSE | `db_backup_creates_file`, `db_backup_nonexistent_noop` |
| SEC-03 : speed hack detect | PASSE | `validate_move_speed_hack` |
| SEC-03 : rate limit moves | PASSE | `validate_move_rate_limit` : 26eme move rejete |
| SEC-03 : window reset | PASSE | `validate_move_rate_window_resets` |
| SQL injection | PASSE | Toutes les requetes utilisent `params![]` (rusqlite) |
| Direction spoofing | PASSE | `is_invalid_detects_server_as_client` |
| Client msg parse | PASSE | `client_cannot_send_server_msg` |
| Trade atomicity | PASSE | `trade_atomic_success`, `trade_atomic_rollback` |
| Trade state machine | PASSE | `trade_state_check`, `trade_confirm_from_confirmed_is_error` |
| Gold overflow | PASSE | `wallet_overflow`, `wallet_overflow_from_zero` |
| Gold underflow | PASSE | `wallet_underflow`, `wallet_underflow_from_empty` |
| Boss saturating damage | PASSE | `andariel_death` : `saturating_sub` evite underflow |
| Password hash Argon2 | PASSE | `account_correct_password`, `account_wrong_password` |

## 5. Score de securite

| Critere | Score /20 | Commentaire |
|---------|----------|-------------|
| Authentification & autorisation | 17/20 | Argon2id OK, pas de rate limit login scope MVP, sessions basiques |
| Chiffrement & secrets | 16/20 | Argon2id + OsRng OK, FNV-1a faible pour hash scripts, CRC32 non-crypto |
| Validation des entrees | 18/20 | Excellente couverture (names, stats, gold, messages, moves), chat non sanitize |
| Dependances & supply chain | 19/20 | Toutes les deps maintenues, 0 CVE, unsafe_code forbid workspace |
| Logging & monitoring | 18/20 | Tracing en place, Rhai logging, pas de log des rejets securite |
| **Score global** | **88/100** | |

## 6. Conformite certifications

| Certification | Applicable ? | Justification |
|--------------|-------------|---------------|
| ISO 27001 | Non | Jeu video indie, pas de donnees sensibles d'entreprise |
| VP2 / RGPD | Non | Listen-server local, pas de cloud, pas de donnees personnelles transmises |
| HDS | Non | Pas de donnees de sante |
| ISO 20000-1 | Non | Pas de service IT manage |
| NF461 | Non | Pas d'archivage electronique |
| NF203 | Non | Pas de logiciel de gestion/comptabilite |
| NF525 | Non | Pas de logiciel de caisse |
| CMMI | Non | Pas de contrat defense |

**Aucune certification obligatoire applicable.** Le projet est un jeu video indie en architecture listen-server.

## 7. Defauts et recommandations

| # | Defaut | Gravite | Recommandation | Statut |
|---|--------|---------|----------------|--------|
| S-01 | SEC-09 utilise FNV-1a 64-bit pour la verification de scripts. FNV-1a n'est pas un hash cryptographique et offre une resistance aux collisions faible (64 bits). Un attaquant pourrait creer un script modifie avec le meme hash en ~2^32 tentatives. | Faible | Acceptable pour listen-server ou les scripts sont locaux. Pour un serveur dedie futur, migrer vers BLAKE3 ou SHA-256 (sans depasser les besoins -- pas de secret). | Accepte (MVP) |
| S-02 | SEC-20 utilise CRC32 pour l'integrite des messages reseau. CRC32 n'est pas un MAC et ne protege pas contre la modification intentionnelle (pas de cle secrete). Un MITM pourrait modifier un message et recalculer le CRC32. | Faible | Acceptable pour listen-server LAN. Pour le multijoueur public, ajouter HMAC-SHA256 ou un canal TLS. | Accepte (MVP) |
| S-03 | `mge-script/src/engine.rs` ligne 134 : `.unwrap_or("unknown")` pour le file_stem d'un script. Non-critique car c'est un fallback safe, mais ne respecte pas strictement le pattern "pas de unwrap en production". | Info | Remplacer par `.unwrap_or_default()` ou une gestion explicite pour coherence stylistique. | Accepte |
| S-04 | `ClientMessage::Chat { text }` n'a pas de validation/sanitization du contenu cote serveur. Un joueur malveillant pourrait envoyer du texte arbitraire (HTML, unicode abuse, longueur excessive). Le `ChatBroadcast` le relaie tel quel. | Moyen | Ajouter : (1) longueur max (ex: 256 chars), (2) strip HTML `<>&"'`, (3) rate limit chat messages, (4) filtrage unicode abuse (control chars, RTL override). | A corriger |
| S-05 | Pas de rate limiting specifique sur les tentatives de login/auth (scope MVP). Le `MoveValidator` ne couvre que les messages `PlayerMove`. | Faible | Hors scope MVP listen-server. Pour le serveur dedie, ajouter un rate limiter par IP sur le endpoint d'authentification (max 5 tentatives / minute). | Differe |

## 8. Verdict

**DEFAUTS NON-BLOQUANTS (acceptes pour MVP)**

Le code audite est de bonne qualite pour un MVP de jeu video en architecture listen-server.
Les items de securite SEC-03 a SEC-22 sont correctement implementes et couverts par des tests.
Les 5 defauts identifies sont tous non-bloquants :
- S-01 et S-02 sont des limitations connues et acceptables pour le contexte listen-server LAN
- S-03 est un point de style mineur
- S-04 (chat sanitization) devrait etre corrige avant un mode multijoueur public
- S-05 (auth rate limit) est differe au serveur dedie

Le score de **88/100** reflete un bon niveau de securite pour un projet de jeu au stade MVP,
avec des bases solides pour durcir le code lors de la transition vers un serveur dedie.

---

*Audit realise par Victor, Expert Cybersecurite, Miyukini AI Studio.*
*Methodologie : Revue de code manuelle + scan patterns + verification tests + OWASP Top 10.*
