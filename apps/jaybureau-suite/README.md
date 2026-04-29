# Jay Bureau — Apps

Suite bureautique collaborative type Google Workspace.

## Apps disponibles

| Dossier | Binaire | Role |
|---------|---------|------|
| [`hub/`](hub/) | `jaybureau` | Hub lanceur de la suite — liste les apps + documents recents |
| [`jay-docs/`](jay-docs/) | `jay-docs` | 📄 Editeur de documents collaboratif (CRDT Yrs + textarea + presence) |
| [`jay-sheets/`](jay-sheets/) | `jay-sheets` | 📊 Tableur (50×20 cellules, valeurs typees, barre de formule) |
| [`jay-slides/`](jay-slides/) | `jay-slides` | 🎞 Presentations avec slides + thumbnails |
| [`jay-formulaire/`](jay-formulaire/) | `jay-formulaire` | 📝 Builder de formulaires (8 types de champs) |
| [`jay-reunion/`](jay-reunion/) | `jay-reunion` | 🎥 Visioconference (lobby + stage + controles) |
| [`jay-club/`](jay-club/) | `jay-club` | 🌐 JayClub — reseau social type Instagram/Meta |
| [`jay-mail/`](jay-mail/) | `jay-mail` | ✉️ Client email (SMTP + IMAP) |
| [`jay-message/`](jay-message/) | `jay-message` | 🔒 Messagerie chiffree de bout en bout (Signal-like) |

## Architecture

Chaque app est un binaire Dioxus 0.7 Desktop independant. Le hub (`jaybureau`) lance les autres apps en tant que processus fils via `std::process::Command`.

Les apps partagent les types et la logique via les crates fondations dans [`../../crates/jaybureau-suite/`](../../crates/jaybureau-suite/).

## Build

```bash
# Build une app specifique
cargo build -p jay-docs --release

# Build tout
cargo build -p jaybureau -p jay-docs -p jay-sheets -p jay-slides \
            -p jay-formulaire -p jay-reunion -p jay-club -p jay-mail -p jay-message
```

## Lancement

```bash
# Lancer le hub (qui peut spawn les autres)
cargo run -p jaybureau

# Lancer une app directement
cargo run -p jay-docs
```
