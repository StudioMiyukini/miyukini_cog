# E9 -- Integration & Tests E2E

## Statut : Termine
## Depend de : E0-E8
## Agents : Lise, Denis
## Taches : 6

| # | Cat | Titre | Agent | Fichier(s) | Statut |
|---|-----|-------|-------|------------|--------|
| E9-01 | TEST-I | Test E2E: WebDAV litmus suite (path traversal) | Lise | crates/miyucloud-dav/tests/security_path_traversal.rs | done |
| E9-02 | TEST-I | Test E2E: CalDAV avec 3 clients (Thunderbird, macOS, DAVx5) | Lise | crates/miyucloud-dav/tests/e2e_caldav.rs | done |
| E9-03 | TEST-I | Test E2E: CardDAV avec 3 clients | Lise | crates/miyucloud-dav/tests/e2e_carddav.rs | done |
| E9-04 | TEST-I | Test E2E: WOPI token round-trip | Lise | crates/miyucloud-dav/tests/e2e_wopi.rs | done |
| E9-05 | TEST-I | Test E2E: sync P2P preservation (memes endpoints) | Lise | tests/e2e/ | deferred-BUF |
| E9-06 | CODE | Review integration finale et corrections | Denis | -- | done |

## Commit message template
`test(miyucloud): E9 -- tests E2E WebDAV/CalDAV/CardDAV, litmus, 3 clients`
