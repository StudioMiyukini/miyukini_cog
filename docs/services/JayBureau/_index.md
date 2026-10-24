# Suite Jay Bureau — index

<!-- @id services.jaybureau.index -->
<!-- @role service-reference -->
<!-- @layer S -->
<!-- @human Suite bureautique collaborative (docs, tableur, slides, formulaires, réunion, social, mail, messagerie E2E) : crates fondations + 9 apps Dioxus Desktop -->
<!-- @do reference_jaybureau -->

- **Strate** : S (services) · apps Dioxus 0.7 Desktop + crates fondations partagées
- **README** : [crates](../../../crates/jaybureau-suite/README.md) · [apps](../../../apps/jaybureau-suite/README.md)
- **Statut** : MVP démo — UI présentes, données de démonstration, réseau non câblé (§6)

## 1. Identité

<!-- @id services.jaybureau.identite -->
<!-- @role component-inventory -->
<!-- @do list_jaybureau_crates -->

| Crate (lib) | Chemin | Rôle |
|---|---|---|
| `jaybureau-core` | `crates/jaybureau-suite/jaybureau-core` | types partagés : `DocKind`, `DocumentBase`, `DocumentAcl`, `CollabEvent`, `Presence`, types Doc/Sheet/Slide/Form/Meeting |
| `jay-collab` | `crates/jaybureau-suite/jay-collab` | CRDT Yrs, client WebSocket, `CollabRoom` / `RoomRegistry`, `PresenceManager` |
| `jayclub` | `crates/jaybureau-suite/jayclub` | réseau social : posts, réactions, stories 24 h, follows, notifications ; store mémoire + `SqliteStore` |
| `jaymail` | `crates/jaybureau-suite/jaymail` | SMTP (`lettre`), IMAP (démo), types `Email` / `Mailbox` |
| `jaymessage` | `crates/jaybureau-suite/jaymessage` | messagerie E2E : identité ed25519, sessions X25519 + ChaCha20-Poly1305, Sender Keys groupes |

| Binaire | Chemin | Rôle |
|---|---|---|
| `jaybureau` | `apps/jaybureau-suite/hub` | lanceur ; lance les autres apps en processus fils |
| `jay-docs` | `apps/jaybureau-suite/jay-docs` | éditeur de documents (Yrs + barre de présence) |
| `jay-sheets` | `apps/jaybureau-suite/jay-sheets` | tableur |
| `jay-slides` | `apps/jaybureau-suite/jay-slides` | présentations |
| `jay-formulaire` | `apps/jaybureau-suite/jay-formulaire` | builder de formulaires |
| `jay-reunion` | `apps/jaybureau-suite/jay-reunion` | visioconférence (UI lobby / scène) |
| `jay-club` | `apps/jaybureau-suite/jay-club` | réseau social |
| `jay-mail` | `apps/jaybureau-suite/jay-mail` | client mail |
| `jay-message` | `apps/jaybureau-suite/jay-message` | messagerie chiffrée |

- **Côté serveur** : `JayBureauState` dans `apps/origin/src/web/jaybureau.rs` (stockage des documents + `RoomRegistry`)
- **Adaptateurs Alicia** (`ServiceClient`) : `crates/jaydocs-client`, `jaysheets-client`, `jayslides-client`, `jaymail-client`, `jaymessage-client`
- **Manifestes** : `service.manifest.json` dans `jaymail`, `jaymessage`, `jay-docs`, `jay-sheets`, `jay-slides`

## 2. Ce que fait la suite

<!-- @id services.jaybureau.fonctions -->
<!-- @role feature-summary -->
<!-- @do summarize_jaybureau_features -->

- Édition collaborative : documents Yrs, fusion de modifications concurrentes (`jay-collab/src/lib.rs`, tests de convergence)
- ACL par document : rôles, accès par lien, `grant` / `revoke` (`jaybureau-core/src/permission.rs`)
- Réseau social JayClub : posts (visibilité, mentions, hashtags), commentaires, stories expirables, notifications
- Mail : envoi SMTP réel via `lettre` (`jaymail/src/smtp_client.rs`) ; réception IMAP **simulée**
- Messagerie E2E : chiffrement 1:1 via `miyukini_cog_bridge::e2e`, signature ed25519, clés de groupe avec rotation

## 3. API et points d'entrée

<!-- @id services.jaybureau.api -->
<!-- @role public-api -->
<!-- @do list_jaybureau_api -->

- `jay_collab` : `new_doc`, `encode_state`, `encode_update_since`, `apply_update` ; `CollabRoom::apply_and_broadcast`, `update_presence`, `leave` ; client `connect(ws_url)` → `{ws_url}/collab/{doc_id}`
- `jaybureau_core::DocumentAcl` : `owner_only`, `role_for`, `grant`, `revoke` ; `Role::can_edit / can_comment / can_share / can_delete`
- `jayclub::SqliteStore` : `open(path)`, `in_memory()`, CRUD profils / posts / réactions / commentaires / stories / follows / notifications, `purge_expired_stories`
- `jaymail` : `send_email(&SmtpConfig, &EmailDraft)` (async) ; `fetch_inbox`, `fetch_message` (démo)
- `jaymessage` : `IdentityKey::{generate, from_hex, to_hex, sign, verify}` ; `SessionStore::{register, encrypt_for, decrypt_from}` ; `perform_handshake` ; `GroupKey::{encrypt, decrypt}` ; rotation via store de groupes
- Routes Origin **documentées en commentaire** (`apps/origin/src/web/jaybureau.rs` l. 9-18) : `/api/jaybureau/docs[...]`, `/ws/jaybureau/collab/:doc_id` — non montées (§6)

## 4. Données et stockage

<!-- @id services.jaybureau.donnees -->
<!-- @role data-storage -->
<!-- @do describe_jaybureau_storage -->

| Donnée | Support | Source |
|---|---|---|
| Documents (méta + snapshot Yrs) | fichiers `{id}.json` + `{id}.bin` dans `storage_dir` | `apps/origin/src/web/jaybureau.rs` (`save_to_disk`) |
| JayClub | SQLite (`rusqlite` bundled) ou mémoire | `jayclub/src/sqlite_store.rs`, `store.rs` |
| Sessions E2E | mémoire (`Mutex<HashMap>`), non persistées | `jaymessage/src/session.rs` |
| Clé d'identité | export hex `to_hex()` — « à stocker chiffré » (commentaire), aucun stockage fourni | `jaymessage/src/identity.rs` |
| Identifiants mail | `SmtpConfig` / `ImapConfig` en clair dans la struct | `jaymail/src/config.rs` |

- Aucun passage par KindMother (`jayclub/src/store.rs` : « MVP, à remplacer par KindMother »)

## 5. Posture sécurité

<!-- @id services.jaybureau.securite -->
<!-- @role security-posture -->
<!-- @do assess_jaybureau_security -->

- **Exposition réseau** : apps Desktop locales ; client collab WebSocket sortant ; SMTP sortant
- **Auth collab** : aucun jeton ni identité dans `connect()` (`jay-collab/src/client.rs` l. 54) — **gap**
- **ACL** : vérifiée côté Origin pour `list_for_user` et `delete_doc` (tests) ; **non vérifiée** dans `CollabRoom::apply_and_broadcast` (tout participant écrit) — **gap**
- **Mail** :
  - `TlsMode::None` → `builder_dangerous` (SMTP sans TLS) possible — **gap**
  - `SmtpConfig` / `ImapConfig` dérivent `Debug` avec `password` → fuite possible en log — **gap**
- **E2E** :
  - handshake = X25519 brut, clé publique éphémère **non signée** par l'identité ed25519 → pas d'authentification du pair (MITM possible) — **gap**
  - groupes : nonce = compteur fourni par l'appelant (`groups.rs` l. 46-49) ; réutilisation de compteur = réutilisation de nonce — **à vérifier côté appelant**
  - sessions et clés uniquement en mémoire ; app `jay-message` génère des pairs démo à chaque lancement
- **JayClub** : `delete_post` vérifie l'auteur ; `mark_notification_read(notif_id)` sans contrôle du propriétaire — **gap**
- **Hub** : repli sur `PATH` si le binaire est absent à côté de l'exe (`hub/src/main.rs` l. 152-155) → risque de détournement de binaire ; suffixe `.exe` codé en dur
- **Fail-closed** : non — pas de gouvernance COG (StrongFather, BondingBrother, MasterButler) dans ces crates

## 6. Statut

<!-- @id services.jaybureau.statut -->
<!-- @role status -->
<!-- @do report_jaybureau_status -->

| Crate | Tests | État |
|---|---|---|
| `jaybureau-core` | 17 | fonctionnel (types, ACL) |
| `jay-collab` | 4 | CRDT fonctionnel ; serveur non exposé |
| `jayclub` | 21 | fonctionnel (mémoire + SQLite) |
| `jaymail` | 3 | SMTP réel ; IMAP stub démo |
| `jaymessage` | 7 | primitives crypto OK ; pas de transport |
| apps (9) | 0 | UI démo, données en dur |

- Total : 52 tests (comptage `#[test]` dans `src/`) ; `JayBureauState` (Origin) : 3 tests
- `jay-mail` : comptes démo (`"demo"` en mot de passe, `jay-mail/src/main.rs` l. 29), envoi journalisé « [Demo] »

## 7. Lacunes connues

<!-- @id services.jaybureau.lacunes -->
<!-- @role known-gaps -->
<!-- @do list_jaybureau_gaps -->

- Routes REST / WebSocket Origin non montées (`JayBureauState` seulement réexporté dans `apps/origin/src/web/mod.rs`)
- IMAP non implémenté ; `jay-reunion` sans média réel (à vérifier)
- Aucune authentification du pair E2E ni persistance chiffrée des clés
- Pas d'intégration KindMother / StrongFather ; persistance fichiers directe côté Origin
- Binaires et manifestes ciblés Windows (`.exe`, `windows_x64`)
- Pas de doc fonctionnelle par app (à créer si la suite sort du stade démo)
