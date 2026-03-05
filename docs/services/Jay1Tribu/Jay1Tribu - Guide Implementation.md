# Jay1Tribu â€” Guide d'implÃ©mentation

## Contexte

Ce document constitue le **guide d'implÃ©mentation** du Service Jay1Tribu. Il dÃ©crit la structure attendue du crate, les modules, les dÃ©pendances (KindMother, MWS), les phases de dÃ©veloppement et les points de conformitÃ© aux contraintes et invariants.

## PortÃ©e / Scope

- **Applicable Ã  :** DÃ©veloppement du crate Jay1Tribu, intÃ©gration Central, contrat Miou/MWS.
- **Audience :** DÃ©veloppeurs, architectes.
- **Statut :** Guide normatif â€” rÃ©fÃ©rence d'implÃ©mentation.

**Note :** Le crate `jay1tribu` n'existe pas encore dans le dÃ©pÃ´t ; ce guide est anticipatoire et sera mis Ã  jour au fur et Ã  mesure de l'implÃ©mentation.

---

## 1. Vue d'ensemble de l'implÃ©mentation

### 1.1 Composants Ã  crÃ©er

| Composant | Type | Emplacement | RÃ´le |
|-----------|------|-------------|------|
| **jay1tribu** | Crate Service (Strate 7) | `crates/jay1tribu/` | Logique mÃ©tier : tribus, salons, amis, messages, persistance locale, transport MWS |
| **OpÃ©rateurs Jay1Tribu** | OpÃ©rateurs | Dans `jay1tribu` | Messagerie, tribus, amis (voir Architecture et Positionnement) |
| **Vue Jay1Tribu** | Service Central | `apps/central/src/services/jay1tribu/` (ou Ã©quivalent) | Interface utilisateur : liste tribus/salons/amis, conversations |
| **Connexion DB / MWS** | Data & transport | Central + jay1tribu | ServiceDb (KindMother), client MWS pour transport et prÃ©sence |

### 1.2 Ordre de dÃ©pendances

```
KindMother (existant)
MWS (prÃ©sence, transport)
       â”‚
       â–¼
jay1tribu (crate) â€” persistance, messagerie, tribus, amis
       â”‚
       â”œâ”€â”€ Miyukini Central (service view, ouverture service)
       â”‚
       â””â”€â”€ Miou (get_online_friends, get_friends_list)
```

### 1.3 Principes d'implÃ©mentation impÃ©ratifs

| # | Principe | VÃ©rification |
|---|----------|---------------|
| P1 | **Pas d'archives centrales** | Aucun envoi du contenu des messages vers un serveur central ; archivage local uniquement. |
| P2 | **Transit cryptÃ©** | Toute donnÃ©e Ã©changÃ©e entre COGs est cryptÃ©e (TLS et/ou E2E selon spec sÃ©curitÃ©). |
| P3 | **Persistance via KindMother** | Toutes les Ã©critures locales passent par KindMother (WriteIntent). |
| P4 | **PrÃ©sence via MWS** | La prÃ©sence (en ligne / hors ligne) est lue depuis le MWS ; pas de duplication de logique. |
| P5 | **Gouvernance par les Cores** | Toute action soumise Ã  StrongFather, KindMother, Master Butler, WorrySentinel, Border Guard via BondingBrother. |

---

## 2. Structure standard du crate (pattern Services)

ConformÃ©ment au pattern [miyukini-services](_index.md) :

```
crates/jay1tribu/
â”œâ”€â”€ Cargo.toml
â””â”€â”€ src/
    â”œâ”€â”€ lib.rs
    â”œâ”€â”€ errors.rs
    â”œâ”€â”€ data/
    â”‚   â”œâ”€â”€ mod.rs
    â”‚   â”œâ”€â”€ types.rs          # Tribe, Salon, Message, Friend, Role, etc.
    â”‚   â”œâ”€â”€ kindmother_db.rs  # (legacy-sqlite)
    â”‚   â””â”€â”€ kindmother_client_db.rs  # (kindmother-only)
    â”œâ”€â”€ auth/                 # Optionnel â€” permissions, RLS
    â”‚   â”œâ”€â”€ mod.rs
    â”‚   â””â”€â”€ permissions.rs
    â”œâ”€â”€ services/             # Adaptateurs inter-services (lecture rÃ©flÃ©chie si besoin)
    â”‚   â””â”€â”€ mod.rs
    â”œâ”€â”€ domain/               # Logique mÃ©tier : crÃ©ation tribu, envoi, livraison diffÃ©rÃ©e
    â”‚   â””â”€â”€ mod.rs
    â””â”€â”€ transport/            # Ou mws/ â€” abstraction transport MWS, chiffrement
        â”œâ”€â”€ mod.rs
        â””â”€â”€ ...
```

### 2.1 Feature flags recommandÃ©s

```toml
[features]
default = ["kindmother-only"]
legacy-sqlite = []
kindmother-only = ["kindmother-client", "kindmother-db-key"]
db-encryption = ["kindmother-db-key"]
```

### 2.2 DÃ©pendances Cargo.toml (vision)

```toml
[dependencies]
# Persistance
kindmother = { path = "../kindmother" }
kindmother-db-key = { path = "../kindmother-db-key", optional = true }
# kindmother-client pour kindmother-only

# MWS / transport (Ã  adapter selon API MWS rÃ©elle)
# miyu-webway-participant ou Ã©quivalent pour prÃ©sence et transport

# Utilitaires
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
thiserror = "2"

# Chiffrement (WorrySentinel, spec technique Ã  valider)
# ring ou aes-gcm, etc.
```

---

## 3. Module data/

### 3.1 Types de domaine (types.rs)

Au minimum (conceptuel) :

- **Tribe** : id, name, creator_cog_id, created_at, settings, etc.
- **Salon** : id, tribe_id (optionnel), type (direct | collective), participants, created_at.
- **Message** : id, salon_id, sender_cog_id, content_encrypted ou ref, created_at.
- **Friend** : id, profile_id, friend_cog_id, pseudo (rÃ©solu), added_at.
- **Role** : id, tribe_id, name, permissions (bitmask ou liste).

Identifiants : UUID v4 ; timestamps ISO 8601. Convention CRUD : `tribe_list`, `tribe_by_id`, `tribe_create`, `tribe_update`, `tribe_delete`, etc.

### 3.2 Persistance (kindmother_db / kindmother_client_db)

- **InstanceType** : `Daughter` pour les DB locales du service.
- Toute Ã©criture passe par KindMother (WriteIntent) ; pas de connexion SQL directe en contournement.
- SchÃ©mas : tables `tribes`, `salons`, `salon_members`, `messages`, `friends`, `roles`, `tribe_member_roles`, etc. (dÃ©tail Ã  dÃ©finir en phase de conception schÃ©ma).

---

## 4. Transport et MWS

- **PrÃ©sence** : consommation des APIs ou Ã©vÃ©nements MWS pour Â« en ligne / hors ligne Â». Pas de rÃ©implÃ©mentation.
- **Transport des messages** : envoi/rÃ©ception via le MWS (canal sÃ©curisÃ©). Chiffrement du payload applicatif selon [Jay1Tribu - Securite et Conformite](./Jay1Tribu%20-%20Securite%20et%20Conformite.md).
- **Livraison diffÃ©rÃ©e (tribu)** : lorsque un membre se reconnecte, le systÃ¨me vÃ©rifie si l'Ã©metteur est connectÃ© et envoie les messages/fichiers non encore livrÃ©s ; implÃ©mentation Ã  dÃ©tailler (file dâ€™attente locale, reprise Ã  la connexion).

---

## 5. IntÃ©gration Miyukini Central

- Enregistrement du service dans la liste des services (Salon / BibliothÃ¨que).
- Ouverture de lâ€™Ã©cran/onglet Jay1Tribu depuis Central (navigation, pas de lecture du contenu par Central).
- Connexion Ã  la base Jay1Tribu (ServiceDb) depuis `apps/central` si nÃ©cessaire (ex. via `ServiceConnections` ou Ã©quivalent).
- Exposer les capacitÃ©s pour Miou : `get_online_friends`, `get_friends_list` (voir [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md)).

---

## 6. IntÃ©gration Miou

- ImplÃ©menter le contrat dÃ©crit dans [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md) : retour de listes dâ€™amis et amis en ligne sans contenu de messages.
- DÃ©gradation gracieuse : si Jay1Tribu est indisponible, retourner liste vide ou erreur gÃ©rÃ©e ; pas de crash Central/Miou.

---

## 7. Phases suggÃ©rÃ©es

| Phase | Objectif | Livrables |
|-------|----------|-----------|
| **1** | Infrastructure | Crate `jay1tribu`, structure data/, types, KindMother (Ã©criture/lecture locale de base). |
| **2** | Salons et messages | CrÃ©ation salon direct/collectif, envoi/rÃ©ception messages (transport MWS + chiffrement), archivage local. |
| **3** | Tribus et rÃ´les | CrÃ©ation tribu, invitations, rÃ´les (Chef de tribu, admin, membre), salons de tribu. |
| **4** | Amis et prÃ©sence | Liste dâ€™amis, consommation prÃ©sence MWS, get_online_friends / get_friends_list pour Miou. |
| **5** | Livraison diffÃ©rÃ©e | Synchronisation Ã  la reconnexion pour les tribus (si Ã©metteur connectÃ©). |
| **6** | Fichiers et images | Envoi/rÃ©ception fichiers et images (chiffrÃ©s), stockage local. |
| **7** | UI Central et Miou | Ã‰crans Central, connexion Miou, dÃ©gradation gracieuse. |
| **8** | SÃ©curitÃ© et audit | Revue chiffrement, conformitÃ© C-1 Ã  C-8, invariants, audit. |

---

## 8. Matrice de vÃ©rification (rÃ©sumÃ©)

Avant toute livraison, vÃ©rifier :

| Contrainte | VÃ©rification |
|------------|--------------|
| C-1 | Aucune archive centralisÃ©e ; tout local. |
| C-2 | Transit cryptÃ© (tests, revue code). |
| C-3 | HÃ©bergement utilisateur uniquement. |
| C-4 | Toutes les Ã©critures via KindMother. |
| C-5 | Type 3 dÃ©clarÃ© ; espaces Central + Inter-COG. |
| C-6 | Livraison diffÃ©rÃ©e conditionnÃ©e (Ã©metteur connectÃ©, paramÃ©trage). |
| C-7 | RÃ´les attribuÃ©s par Chef de tribu, gouvernÃ©s par Cores. |
| C-8 | PrÃ©sence lue depuis MWS uniquement. |

---

## 9. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [Jay1Tribu - Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md) | Concepts, modÃ¨le mÃ©tier. |
| [Jay1Tribu - Architecture et Positionnement](./Jay1Tribu%20-%20Architecture%20et%20Positionnement.md) | Pyramide, OpÃ©rateurs, MWS, Cores. |
| [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) | Contraintes et invariants. |
| [Jay1Tribu - Securite et Conformite](./Jay1Tribu%20-%20Securite%20et%20Conformite.md) | Chiffrement, contrÃ´les d'accÃ¨s. |
| [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md) | Contrat Central / Miou. |
| Skill miyukini-services | Pattern data/, auth/, services/, feature flags. |
| Skill miyukini-kindmother-db | KindMother, InstanceType, kindmother-db-key. |

---

**Document** : Jay1Tribu â€” Guide d'implÃ©mentation  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Guide normatif

