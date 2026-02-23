# MiyukiniTerminal — Spécification Adaptation MiyuWebwayParticipant

## Contexte

Ce document décrit les **parties réutilisables** du crate `miyuwebway_participant`, les **adaptations** nécessaires pour un client Terminal (pas de Tracker serveur, pas de port en écoute), et l'**API minimale** exposée au Terminal.

**Références :**

- [Architecture Technique](./MiyukiniTerminal%20-%20Architecture%20Technique.md)
- Crate : `crates/miyuwebway_participant/`
- [Spec Protocole Relay Terminal](./MiyukiniTerminal%20-%20Spec%20Protocole%20Relay%20Terminal.md)

---

## Portée / Scope

- Parties réutilisables (declaration, transport, relay_client)
- Parties à exclure ou adapter (tracker serveur, port 21000)
- API minimale Terminal
- Dépendances `no_std` (si applicable)

---

## 1. Structure actuelle miyuwebway_participant

### 1.1 Modules

| Module | Rôle |
|--------|------|
| `relay_client` | Connexion TLS Relay, REGISTER, heartbeat |
| `tracker_client` | Annonce, découverte (client → Tracker) |
| `declaration` | Construction/signature déclarations MWS |
| `transport` | Envoi/réception messages bas niveau |
| `cog_list` | Liste COGs locale |
| `discovery` | Requêtes discovery |
| `address` | Résolution adresse Tracker |
| `port` | Vérification ports MWS |
| `context` | GovernedContext |
| `mws_service` | Service haut niveau (coordonne relay + tracker) |
| `protocol` | Types, messages |

### 1.2 Usage Central (STABLE)

Central utilise MwsService pour :
- Se connecter au Relay (enregistrement)
- S'annoncer au Tracker (découverte)
- Maintenir session (heartbeat)

---

## 2. Parties réutilisables pour Terminal

### 2.1 relay_client

| Composant | Réutilisable | Adaptation |
|-----------|--------------|-------------|
| Connexion TLS | ✅ | Identique |
| REGISTER | ✅ | Ajouter parent_cog_id (déjà dans protocol) |
| REGISTER_OK parsing | ✅ | Identique |
| HEARTBEAT | ✅ | Identique |
| Session management | ✅ | Identique |

**Action :** Vérifier que `RegisterPayload` supporte `parent_cog_id` (oui dans `apps/origin`). Adapter la construction du payload côté Terminal (cog_type=TERMINAL, os_type=ANDROID, parent_cog_id fourni).

### 2.2 protocol / types

| Type | Usage Terminal |
|------|----------------|
| CogType | Utiliser TERMINAL (0x05) |
| OsType | Utiliser Android (0x03) |
| RegisterPayload | Construire avec parent_cog_id |
| RegisterOkPayload | Parser réponse |

### 2.3 transport

| Fonction | Usage |
|----------|-------|
| Envoi trames | ✅ |
| Réception trames | ✅ |
| Gestion connexion | ✅ |

### 2.4 declaration

| Fonction | Usage Terminal |
|----------|---------------|
| Construire Passeport/svc_manifest | ✅ Adapté (liste services réduite) |
| Signer | ✅ (si gouverné par Cores) |
| Valider | ✅ |

---

## 3. Parties à exclure ou adapter

### 3.1 tracker_client (annonce en écoute)

| Rôle | Terminal |
|------|----------|
| S'annoncer sur port 21000 | ❌ Terminal n'écoute pas |
| Tracker serveur | ❌ Pas de port en écoute |

**Adaptation :** Utiliser `tracker_client` uniquement en **mode client** (requêtes discovery vers Tracker) si besoin de découvrir d'autres COGs. Pas d'annonce locale.

### 3.2 Port 21000 en écoute

Terminal ne doit **jamais** ouvrir le port 21000. Aucun serveur TCP local pour MWS.

### 3.3 mws_service

Le `MwsService` actuel coordonne relay + tracker (annonce). Pour Terminal :

- Option A : Créer `MwsTerminalService` allégé (relay uniquement)
- Option B : Paramétrer MwsService pour désactiver l'annonce Tracker

---

## 4. API minimale exposée au Terminal

### 4.1 Fonctions requises

```rust
// Connexion Relay
pub async fn connect_relay(
    relay_addr: &str,
    identity: &TerminalIdentity,
) -> Result<RelaySession, MwsError>;

// Envoi REGISTER avec parent_cog_id
pub async fn register(
    session: &mut RelaySession,
    payload: RegisterPayload,
) -> Result<RegisterOkPayload, RegisterError>;

// Heartbeat
pub async fn heartbeat(session: &mut RelaySession) -> Result<(), MwsError>;

// Fermeture
pub fn close(session: &mut RelaySession);
```

### 4.2 Types Terminal

```rust
pub struct TerminalIdentity {
    pub cog_id: String,
    pub parent_cog_id: String,
    pub core_version: String,
    pub service_list: Vec<ServiceEntry>,
    pub environment_health: EnvironmentHealth,
}

pub struct RelaySession {
    // session_id, permis_id, etc.
}
```

### 4.3 Dépendances

| Crate | Usage |
|-------|-------|
| `tokio` | Async runtime |
| `native-tls` ou `rustls` | TLS |
| `bytes` | Manipulation buffers |
| `serde`, `serde_json` | JSON |
| Pas de `std` spécial : crates standard | — |

---

## 5. no_std

Le crate `miyuwebway_participant` actuel utilise `std`. Pour le Terminal :

- **Pas de no_std** requis pour le MVP ; Android supporte std
- Si évolution vers environnement contraint : évaluer extraction des parties purement algorithmiques (declaration, validation) en no_std

---

## 6. Intégration dans apps/terminal

### 6.1 Dépendance Cargo.toml

```toml
[dependencies]
miyuwebway_participant = { path = "../../crates/miyuwebway_participant" }
```

### 6.2 Wrapper Terminal

Créer un module `apps/terminal/src/mws/` qui :

1. Construit `TerminalIdentity` depuis le stockage local
2. Appelle `relay_client::connect` avec les bons paramètres
3. Construit `RegisterPayload` avec cog_type=TERMINAL, os_type=ANDROID, parent_cog_id
4. Gère heartbeat en tâche de fond
5. Expose un état simplifié : `Connected` / `Disconnected` / `Reconnecting`

---

## 7. Options d'implémentation

### Option A : Réutiliser miyuwebway_participant tel quel

- Extraire/utiliser `relay_client`, `protocol`, `transport`
- Construire les payloads avec parent_cog_id
- Ne pas démarrer la partie Tracker (annonce)

### Option B : Créer miyuwebway_terminal_client

- Crate dérivé ou fork allégé
- Seulement Relay client
- Moins de dépendances ; plus simple à maintenir pour mobile

**Recommandation :** Option A pour le MVP ; Option B si le poids ou la complexité devient un problème.

---

## 8. Intégration MSCM/MIP

### 8.1 Blocs à baliser dans le wrapper Terminal

Tout code qui encapsule ou étend `miyuwebway_participant` doit être balisé MSCM pour la Phase B :

| Fonction | @id suggéré | @do |
|----------|-------------|-----|
| connect_relay | terminal.mws.v1.fn.relay_connect | Établit connexion TCP/TLS au Relay |
| build_register_payload | terminal.mws.v1.fn.build_register | Construit payload REGISTER avec parent_cog_id |
| parse_register_ok | terminal.mws.v1.fn.parse_register_ok | Parse session_id, permis_id |
| send_heartbeat | terminal.mws.v1.fn.send_heartbeat | Envoie HEARTBEAT |

### 8.2 Dépendances MIP

Le crate `miyuwebway_participant` existant est déjà (ou doit être) balisé MSCM. Le Terminal ajoute une **couche d'adaptation** — les blocs `terminal.mws.*` — qui seront indexés dans le MIP du projet. Le générateur MIP doit scanner `apps/terminal/src/` en plus des crates.

---

## 9. Références

- Crate : `crates/miyuwebway_participant/src/`
- [Spec MSCM MIP Conformite](./MiyukiniTerminal%20-%20Spec%20MSCM%20MIP%20Conformite.md)
- [Spec Protocole Relay Terminal](./MiyukiniTerminal%20-%20Spec%20Protocole%20Relay%20Terminal.md)
