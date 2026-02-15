---
name: miyukini-mws-origin
description: Architecture et developpement de Miyukini Origin (apps/origin), point central de verite du MWS. Couvre le Relay (TLS, sessions, tunnels, verification 3 phases), le Tracker (TCP, pools, catalogues, lobbys), le serveur web (site, API, downloads), la configuration et le deploiement. Utiliser quand on travaille sur apps/origin, quand on modifie le relay ou le tracker, quand on ajoute des routes web, ou quand on deploie Origin.
---

# MWS Origin — Point central de verite

## Architecture

```
apps/origin/src/
├── main.rs           # Point d'entree, demarrage des serveurs
├── config.rs         # OriginConfig (ports, chemins, TLS)
├── protocol.rs       # Types communs (CogType, OsType, SESSION_ID_SIZE)
├── admin.rs          # Serveur admin (monitoring)
├── relay/
│   ├── mod.rs          # Module relay
│   ├── server.rs       # RelayServer TCP+TLS (port 7000)
│   ├── session.rs      # SessionManager, Session, etats
│   ├── tunnel.rs       # TunnelManager, Tunnel, TunnelMessage
│   ├── verification.rs # Verifier (3 phases : cle Cores, blocs MIP, sante)
│   ├── metrics.rs      # RelayMetrics
│   └── permis.rs       # PermisRegistry (Permis de circulation)
├── tracker/
│   ├── mod.rs          # Module tracker
│   ├── server.rs       # TrackerServer TCP (port 21000)
│   ├── pool.rs         # PoolManager (pools par version Cores)
│   ├── catalog.rs      # Catalog (services, lobbys, COGs)
│   ├── protocol.rs     # TrackerMessageType, payloads
│   ├── metrics.rs      # TrackerMetrics
│   └── visit_tracker.rs # Suivi visites catalogue
└── web/
    ├── mod.rs          # Module web
    ├── server.rs       # WebServer HTTP (port 80/443)
    ├── pages.rs        # Pages HTML dynamiques
    ├── content.rs      # ContentManager (blog, annonces, downloads)
    └── api.rs          # API REST
```

## Serveurs

| Serveur | Port | Protocole | Role |
|---------|------|-----------|------|
| Relay | 7000 | TCP+TLS | Connexion COGs, verification, tunnels |
| Tracker | 21000 | TCP binaire | Decouverte, pools, lobbys |
| Web | 80/443 | HTTP(S) | Site web, API, telechargements |
| Admin | 9000 | HTTP | Monitoring interne |

## Relay — Verification 3 phases

```
COG → Relay
  Phase A : Cle de conformite des Cores (hash version → verification)
  Phase B : Blocs de code MIP des Services (bloc aleatoire → chiffre → verification)
  Phase C : Sante de l'environnement (trust_state, security_level, services actifs)
  → Si OK : Permis de circulation delivre
```

## Tracker — Protocole binaire

```
[version:u8][type:u8][length:u32][payload:bytes]
```

Types de messages : `Announce`, `Heartbeat`, `SearchCogs`, `CreateLobby`, `JoinLobby`, etc.

## Demarrage (main.rs)

```rust
#[tokio::main]
async fn main() {
    let config = OriginConfig::load(path);
    let relay = RelayServer::new(&config).await;
    let tracker = TrackerServer::new(&config).await;
    let web = WebServer::new(&config).await;
    let admin = AdminServer::new(&config);

    tokio::select! {
        _ = relay.run() => {},
        _ = tracker.run() => {},
        _ = web.run() => {},
        _ = admin.run() => {},
    }
}
```

## Etat du code

Le code relay/tracker est **prepare pour fonctionnalites futures** : beaucoup de structs et methodes sont implementees mais pas encore connectees. Les modules ont `#![allow(dead_code)]` car c'est du code pre-integre.

## Regles

1. **Async partout** : `tokio` runtime, `Arc<RwLock<T>>` pour etat partage
2. **Securite** : TLS obligatoire pour le relay, verification 3 phases
3. **Isolation** : chaque serveur dans son module, communication par types partages
4. **Metriques** : `AtomicU64` pour compteurs thread-safe
5. **Logs** : `tracing` avec niveaux (info, warn, debug, error)

## Deploiement

Voir le skill `hostinger-vps` pour le deploiement sur le VPS Hostinger.

## References

- **Code** : `apps/origin/`
- **Documentation MWS** : `docs/miyukini-webway-system/`
- **Relay** : `docs/reference/Miyukini Conceptual References - Miyukini Webway Relay.md`
- **Deploiement** : `hostinger_VPS.md`
