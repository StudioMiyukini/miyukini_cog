# ☁️ Miyukini Cloud

**Self-hosted cloud storage, calendars & contacts — ultra-fast, secure & lightweight.**

Built in Rust for minimal resource usage. Your files, photos, calendars & contacts, all in one place.

## Features

- **File Management** — Upload, download, organize with drag-and-drop. Deduplication via SHA-256.
- **Photo Sync** — Mobile photo backup with smart sync (only new files uploaded).
- **WebDAV** — RFC 4918 compatible. Works with Windows Explorer, macOS Finder, Nautilus.
- **CalDAV** — RFC 4791 calendars. Compatible with Thunderbird, iOS, DAVx⁵, GNOME Calendar.
- **CardDAV** — RFC 6352 contacts. Compatible with Thunderbird, iOS, GNOME Contacts.
- **PWA** — Installable on iPhone, Android & Desktop as a native-like app.
- **WOPI** — Collabora Online / OnlyOffice integration for document editing.
- **NextCloud Compatible** — Legacy NextCloud clients can connect via compatibility layer.
- **Security** — JWT auth, Argon2id passwords, CSRF protection, rate limiting, audit logging, RLS.
- **RGPD / GDPR** — Data export, erasure, consent management.
- **Encryption at Rest** — AES-256-GCM for file blobs.
- **MWS Tunnel** — Connect via Miyukini Webway System for zero-config internet access.
- **DDNS** — No-IP / DuckDNS support for dynamic DNS.

## Quick Start

### Docker (recommended)

```bash
docker compose up -d
```

Open `http://localhost:8086` — create your admin account on first run.

### From Source

```bash
# Prerequisites: Rust 1.93+, PostgreSQL 13+
docker compose up -d postgres   # Start PostgreSQL
cargo run                       # Start server on port 8086
```

### Environment

Copy `example.env` to `.env` and configure:

```bash
MIYUCLOUD_STORAGE_PATH=./storage
MIYUCLOUD_SERVER_PORT=8086
MIYUCLOUD_DB_CONNECTION_STRING=postgres://postgres:postgres@localhost:5432/miyucloud
MIYUCLOUD_ENABLE_AUTH=true
MIYUCLOUD_BASE_URL=https://cloud.example.com
```

See `example.env` for all options.

## Architecture

Hexagonal / Clean Architecture in Rust (Axum + PostgreSQL + sqlx).

```
src/
├── domain/          # Core entities (File, Folder, User, Calendar, Contact)
├── application/     # Use cases, ports (traits), services, DTOs
├── infrastructure/  # PostgreSQL repos, JWT, Argon2, encryption, caching
├── interfaces/      # HTTP handlers, middleware, WebDAV/CalDAV/CardDAV
└── common/          # Config, DI container, error types
```

- **512 MB RAM** minimum — runs on a Raspberry Pi
- **~40 MB** Docker image (Alpine + static binary)
- **< 1s** cold start
- **Content-addressable** blob storage with deduplication

## Mobile App

Miyukini Cloud works as a PWA (Progressive Web App):

- **iPhone**: Safari → Share → "Add to Home Screen"
- **Android**: Chrome → Menu → "Install app"
- **Photo Sync**: Select photos from gallery → only new ones are uploaded

Access `/mobile` for guided installation.

## Security

| Control | Implementation |
|---|---|
| Authentication | JWT HS256 + BLAKE3 cache, Argon2id, OIDC/SSO |
| Authorization | RBAC + PostgreSQL Row-Level Security |
| Encryption at rest | AES-256-GCM (opt-in) |
| Audit logging | Persistent `audit.events` table |
| Rate limiting | IP-based sliding window |
| CSRF | Double-submit cookie |
| GDPR | Export, erasure, consent APIs |

## License

MIT License — see [LICENSE](LICENSE).

## Credits

Based on [OxiCloud](https://github.com/DioCrafts/OxiCloud) by DioCrafts.
Extended with security, mobile, MWS & GDPR by [Studio Miyukini](https://github.com/StudioMiyukini).
