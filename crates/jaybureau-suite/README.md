# Jay Bureau — Crates fondations

Crates partages entre toutes les apps de la suite Jay Bureau.

## Crates

| Crate | Role | Fonctionnalites cles |
|-------|------|---------------------|
| [`jaybureau-core/`](jaybureau-core/) | Types partages | `DocKind`, `DocumentBase`, `DocumentAcl`, `CollabEvent`, `Presence`, types par app (Doc/Sheet/Slide/Form/Meeting) |
| [`jay-collab/`](jay-collab/) | CRDT temps reel | Yrs (Yjs port Rust), client WebSocket, `CollabRoom` serveur, `PresenceManager` cursors/couleurs |
| [`jayclub/`](jayclub/) | Reseau social | Posts (visibility, mentions, hashtags), reactions 7 types, stories 24h, follows, notifications, persistence SQLite |
| [`jaymail/`](jaymail/) | Client email | SMTP via `lettre`, IMAP, types `Email`/`Address`/`Mailbox`, defaults Miyukini |
| [`jaymessage/`](jaymessage/) | Messagerie E2E | Identite ed25519, sessions X25519+ChaCha20-Poly1305, Sender Keys pour groupes |

## Dependances externes

- **Yrs** — CRDT port Rust de Yjs (Google Docs-like collab)
- **lettre** — SMTP client
- **rusqlite** — SQLite bundled (persistence JayClub)
- **x25519-dalek + chacha20poly1305 + ed25519-dalek** — crypto E2E
- **tokio + tokio-tungstenite** — async + WebSocket

## Reutilisation

Les crates de `jaybureau-suite/` peuvent etre embarques dans n'importe quel app Miyukini :
- `jay-collab` peut servir d'engine collab pour MGE (Miyukini Game Engine)
- `jayclub::SqliteStore` peut etre integre dans Central Desktop
- `jaymessage::IdentityKey` reutilise `miyukini-cog-bridge::e2e` (le meme protocole que le bridge Android)

## Tests

```bash
cargo test -p jaybureau-core -p jay-collab -p jayclub -p jaymail -p jaymessage
```

Total : ~45 tests, 0 echecs.
