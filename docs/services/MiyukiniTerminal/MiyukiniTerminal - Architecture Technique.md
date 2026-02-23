# MiyukiniTerminal — Architecture Technique

## Contexte

Ce document décrit l'**architecture technique** de l'application MiyukiniTerminal : couches, flux de données, interactions entre composants et décisions techniques. Stack confirmée : **Dioxus** (UI) + **Rust** (logique, MWS, persistance).

**Références :**

- [Document Fondateur](./MiyukiniTerminal%20-%20Document%20Fondateur.md)
- [Spec Canaux Connexion MWS Parent-Enfant](./MiyukiniTerminal%20-%20Spec%20Canaux%20Connexion%20MWS%20Parent%20Enfant.md)
- [Stack Dioxus Mobile Spec](./MiyukiniTerminal%20-%20Stack%20Dioxus%20Mobile%20Spec.md)
- [MWS - Protocole Relay](../../miyukini-webway-system/protocole/MWS%20-%20Protocole%20Relay.md)

---

## Portée / Scope

- Schémas d'architecture (Mermaid)
- Couches applicatives
- Flux de données et interactions
- Décisions techniques (Rust/Dioxus confirmé)
- Réutilisation des crates existants

---

## 1. Vue d'ensemble des couches

```mermaid
flowchart TB
    subgraph UI["Couche UI (Dioxus Mobile)"]
        screens[Écrans]
        components[Composants]
        theme[Thème / Design System]
    end

    subgraph Services["Couche Services"]
        liaison[Liaison Parent]
        sync[Synchronisation]
        mws_client[MWS Client]
        cache[Cache Local]
    end

    subgraph MWS["MWS / Réseau"]
        relay_client[Relay Client]
        participant[MiyuWebwayParticipant]
    end

    subgraph Storage["Couche Stockage"]
        identity[Identity]
        queue[Queue Actions]
        preferences[Préférences]
    end

    screens --> Services
    components --> theme
    liaison --> Storage
    sync --> mws_client
    sync --> cache
    mws_client --> relay_client
    relay_client --> participant
    cache --> Storage
    queue --> Storage
```

---

## 2. Schéma détaillé par couche

### 2.1 Couche UI (Dioxus Mobile)

| Composant | Rôle |
|-----------|------|
| **Écrans** | Liaison, Salon, Service (détail), Paramètres, Profil |
| **Composants** | Boutons, cartes, listes, champs, modals (réutilisés ou adaptés Central) |
| **Thème** | Palette Gaming héritée ; adaptations mobile (taille touch, grille) |

**Entrées :** Gestes utilisateur, événements  
**Sorties :** Appels à la couche Services (liaison, sync, actions)

### 2.2 Couche Services

| Composant | Rôle |
|-----------|------|
| **Liaison Parent** | Validation token, création identité, stockage `parent_cog_id`, `cog_id` |
| **Synchronisation** | Sync avec parent (services, préférences, cache) ; détection online/offline |
| **MWS Client** | Connexion Relay, Passeport, Permis, heartbeats |
| **Cache Local** | Données en cache (JayKonta, JayKoa) ; lecture en offline |

### 2.3 Couche MWS

| Composant | Rôle |
|-----------|------|
| **Relay Client** | TCP/TLS vers Relay (port 7000) ; REGISTER avec `parent_cog_id` ; connexion **directe** par défaut (Mode A) |
| **MiyuWebwayParticipant** | Crate existant ; transport, déclaration, discovery (adapté client seul) |

**Pas de :** Tracker serveur (port 21000 en écoute) — Terminal = client uniquement.

**Canaux :** Voir [Spec Canaux Connexion MWS Parent-Enfant](./MiyukiniTerminal%20-%20Spec%20Canaux%20Connexion%20MWS%20Parent%20Enfant.md) pour les modes direct Relay vs via parent.

### 2.4 Couche Stockage

| Composant | Rôle |
|-----------|------|
| **Identity** | `cog_id`, `parent_cog_id`, identité utilisateur |
| **Queue Actions** | Actions différées (dépenses, événements) ; rejeu à la reconnexion |
| **Préférences** | Thème, verrouillage, notifications |
| **Cache** | Données services (soldes, mouvements, agenda) |

---

## 3. Flux de données principaux

### 3.1 Flux liaison (premier lancement)

```mermaid
sequenceDiagram
    participant U as Utilisateur
    participant UI as Écran Liaison
    participant Svc as Liaison Service
    participant St as Stockage
    participant MWS as MWS Client
    participant R as Relay

    U->>UI: Scan QR / Saisie token
    UI->>Svc: validate_token(token)
    Svc->>Svc: Décoder token, vérifier signature
    Svc->>St: Stocker cog_id, parent_cog_id
    Svc->>MWS: connect_relay()
    MWS->>R: REGISTER (Passeport + parent_cog_id)
    R->>MWS: REGISTER_OK (Permis)
    MWS->>Svc: Permis reçu
    Svc->>UI: Liaison OK
```

### 3.2 Flux synchronisation

```mermaid
sequenceDiagram
    participant UI as Salon
    participant Sync as Sync Service
    participant Cache as Cache
    participant MWS as MWS Client
    participant Parent as COG Parent (STABLE)

    UI->>Sync: refresh()
    Sync->>MWS: connecté ?
    alt Connecté
        MWS->>Parent: Requête données (services, cache)
        Parent->>MWS: Réponse JSON
        MWS->>Sync: Données
        Sync->>Cache: Mise à jour
        Sync->>UI: Rafraîchir
    else Offline
        Sync->>Cache: Lecture cache
        Sync->>UI: Afficher cache (indicateur)
    end
```

### 3.3 Flux action différée (offline → online)

```mermaid
sequenceDiagram
    participant U as Utilisateur
    participant UI as Écran Service
    participant Queue as Queue Actions
    participant Sync as Sync Service
    participant Parent as COG Parent

    U->>UI: Saisir dépense
    UI->>Queue: Enregistrer action (type, payload)
    Queue->>UI: OK (en attente)
    Note over Sync: Reconnexion détectée
    Sync->>Queue: rejouer()
    Queue->>Parent: Envoyer actions pending
    Parent->>Queue: Confirmations
    Queue->>Queue: Marquer sent
    Queue->>Sync: Sync cache
```

---

## 4. Décisions techniques

### 4.1 Stack confirmée

| Décision | Choix | Justification |
|----------|-------|---------------|
| **UI** | Dioxus 0.6+ (mobile) | Compatibilité maximale avec Central ; Rust partagé |
| **Langage** | Rust | Un seul langage ; réutilisation crates ; sécurité mémoire |
| **MWS** | miyuwebway_participant (adapté) | Éviter duplication protocole ; cohérence MWS |
| **Stockage** | SQLite / rusqlite ou KindMother | Persistance locale ; option chiffrement |

### 4.2 Pas de Kotlin / JNI

- **Pas de JNI** : logique MWS en Rust pur ; Dioxus compile en binaire Android (NDK)
- **Pas de Kotlin** : éviter deux codebases ; maintenance simplifiée

### 4.3 Réutilisation crates

| Crate | Usage |
|-------|-------|
| `miyuwebway_participant` | relay_client, declaration, transport ; adapter pour client seul (pas tracker) |
| `apps/origin` (protocol) | Types `CogType`, `OsType`, messages (extraire en crate partagé si besoin) |
| `kindmother` / `kindmother-client` | Optionnel ; rusqlite suffit pour MVP |
| `miyukini-central` | Thème, patterns (référence) ; pas de dépendance directe (app séparée) |

---

## 5. Structure du projet (cible)

```
apps/terminal/
├── Cargo.toml
├── src/
│   ├── main.rs              # Point d'entrée Dioxus mobile
│   ├── app.rs               # App racine, providers
│   ├── state.rs             # AppState, AppContext
│   ├── theme.rs             # ThemePalette (hérité Central)
│   ├── screens/
│   │   ├── liaison.rs
│   │   ├── salon.rs
│   │   ├── service_detail.rs
│   │   ├── parametres.rs
│   │   └── profil.rs
│   ├── components/
│   │   └── ...
│   ├── services/
│   │   ├── liaison.rs
│   │   ├── sync.rs
│   │   └── mws.rs
│   ├── storage/
│   │   ├── identity.rs
│   │   ├── queue.rs
│   │   └── cache.rs
│   └── mws/
│       └── relay_client.rs  # ou wrapper miyuwebway_participant
└── ...                      # Fichiers Dioxus mobile (Android)
```

---

## 6. Points d'intégration externe

| Point | Protocole / Format |
|-------|-------------------|
| **Relay** | TCP/TLS, port 7000 ; trames binaires MWS |
| **Central (parent)** | Token de liaison ; API sync (à définir) |
| **Stockage Android** | `getFilesDir()`, `databases/` ; SQLite |

---

## 7. Logique de décision par couche

### 7.1 Couche UI : règles de routage

| Événement | Condition | Action |
|-----------|-----------|--------|
| Lancement app | `identity` vide | Afficher écran Liaison |
| Lancement app | `identity` présente | Afficher Salon |
| Clic service | Service JayKonta | Ouvrir écran détail JayKonta |
| Clic service | Service JayKoa | Ouvrir écran détail JayKoa |
| Pull-to-refresh | Toujours | Déclencher sync |
| Perte réseau | Détection ConnectivityManager | Mettre connection_state = Offline |
| Retour réseau | Détection | Mettre Online, lancer sync, rejouer queue |

### 7.2 Couche Services : protocole sync

```mermaid
flowchart TD
    S[Sync demandée] --> C{connection_state ?}
    C -->|Offline| R[Retour : lecture cache uniquement]
    C -->|Online| T[Construire requête]
    T --> U{since disponible ?}
    U -->|Oui| V[sync_type = delta]
    U -->|Non| W[sync_type = full]
    V --> X[POST /api/terminal/sync]
    W --> X
    X --> Y{Réponse ?}
    Y -->|200| Z[Mettre à jour cache, last_sync_at]
    Y -->|Erreur| AA[connection_state = Degrading]
    AA --> AB[Retry ou fallback cache]
```

### 7.3 Couche MWS : état machine connexion

| État | Transition entrante | Transition sortante |
|------|---------------------|----------------------|
| Disconnected | Init | Connexion TCP réussie → Connecting |
| Connecting | TCP connect | TLS OK → Registering |
| Registering | Envoi REGISTER | REGISTER_OK → Connected ; REGISTER_ERR → Disconnected |
| Connected | Régulier | Heartbeat ; CLOSE → Disconnected |
| Reconnecting | Perte connexion | REGISTER → Registering |

### 7.4 Conformité MSCM/MIP

L'architecture doit faciliter le **balisage MSCM** : chaque module (liaison, mws, sync, storage) doit exposer des blocs identifiables pour la Phase B. Voir [Spec MSCM MIP Conformite](./MiyukiniTerminal%20-%20Spec%20MSCM%20MIP%20Conformite.md).

---

## 8. Références

- [Stack Dioxus Mobile Spec](./MiyukiniTerminal%20-%20Stack%20Dioxus%20Mobile%20Spec.md)
- [Spec MSCM MIP Conformite](./MiyukiniTerminal%20-%20Spec%20MSCM%20MIP%20Conformite.md)
- [Spec MiyuWebwayParticipant Adapt](./MiyukiniTerminal%20-%20Spec%20MiyuWebwayParticipant%20Adapt.md)
- [Spec Stockage Local](./MiyukiniTerminal%20-%20Spec%20Stockage%20Local.md)
