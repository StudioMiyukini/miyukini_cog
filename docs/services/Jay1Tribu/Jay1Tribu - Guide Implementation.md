# Jay1Tribu — Guide d'implémentation

## Contexte

Ce document constitue le **guide d'implémentation** du Service Jay1Tribu. Il décrit la structure attendue du crate, les modules, les dépendances (KindMother, MWS), les phases de développement et les points de conformité aux contraintes et invariants.

## Portée / Scope

- **Applicable à :** Développement du crate Jay1Tribu, intégration Central, contrat Miou/MWS.
- **Audience :** Développeurs, architectes.
- **Statut :** Guide normatif — référence d'implémentation.

**Note :** Le crate `jay1tribu` n'existe pas encore dans le dépôt ; ce guide est anticipatoire et sera mis à jour au fur et à mesure de l'implémentation.

---

## 1. Vue d'ensemble de l'implémentation

### 1.1 Composants à créer

| Composant | Type | Emplacement | Rôle |
|-----------|------|-------------|------|
| **jay1tribu** | Crate Service (Strate 7) | `crates/jay1tribu/` | Logique métier : tribus, salons, amis, messages, persistance locale, transport MWS |
| **Opérateurs Jay1Tribu** | Opérateurs | Dans `jay1tribu` | Messagerie, tribus, amis (voir Architecture et Positionnement) |
| **Vue Jay1Tribu** | Service Central | `apps/central/src/services/jay1tribu/` (ou équivalent) | Interface utilisateur : liste tribus/salons/amis, conversations |
| **Connexion DB / MWS** | Data & transport | Central + jay1tribu | ServiceDb (KindMother), client MWS pour transport et présence |

### 1.2 Ordre de dépendances

```
KindMother (existant)
MWS (présence, transport)
       │
       ▼
jay1tribu (crate) — persistance, messagerie, tribus, amis
       │
       ├── Miyukini Central (service view, ouverture service)
       │
       └── Miou (get_online_friends, get_friends_list)
```

### 1.3 Principes d'implémentation impératifs

| # | Principe | Vérification |
|---|----------|---------------|
| P1 | **Pas d'archives centrales** | Aucun envoi du contenu des messages vers un serveur central ; archivage local uniquement. |
| P2 | **Transit crypté** | Toute donnée échangée entre COGs est cryptée (TLS et/ou E2E selon spec sécurité). |
| P3 | **Persistance via KindMother** | Toutes les écritures locales passent par KindMother (WriteIntent). |
| P4 | **Présence via MWS** | La présence (en ligne / hors ligne) est lue depuis le MWS ; pas de duplication de logique. |
| P5 | **Gouvernance par les Cores** | Toute action soumise à StrongFather, KindMother, Master Butler, WorrySentinel, Border Guard via BondingBrother. |

---

## 2. Structure standard du crate (pattern Services)

Conformément au pattern [miyukini-services](.cursor/skills/miyukini-services/SKILL.md) :

```
crates/jay1tribu/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── errors.rs
    ├── data/
    │   ├── mod.rs
    │   ├── types.rs          # Tribe, Salon, Message, Friend, Role, etc.
    │   ├── kindmother_db.rs  # (legacy-sqlite)
    │   └── kindmother_client_db.rs  # (kindmother-only)
    ├── auth/                 # Optionnel — permissions, RLS
    │   ├── mod.rs
    │   └── permissions.rs
    ├── services/             # Adaptateurs inter-services (lecture réfléchie si besoin)
    │   └── mod.rs
    ├── domain/               # Logique métier : création tribu, envoi, livraison différée
    │   └── mod.rs
    └── transport/            # Ou mws/ — abstraction transport MWS, chiffrement
        ├── mod.rs
        └── ...
```

### 2.1 Feature flags recommandés

```toml
[features]
default = ["kindmother-only"]
legacy-sqlite = []
kindmother-only = ["kindmother-client", "kindmother-db-key"]
db-encryption = ["kindmother-db-key"]
```

### 2.2 Dépendances Cargo.toml (vision)

```toml
[dependencies]
# Persistance
kindmother = { path = "../kindmother" }
kindmother-db-key = { path = "../kindmother-db-key", optional = true }
# kindmother-client pour kindmother-only

# MWS / transport (à adapter selon API MWS réelle)
# miyu-webway-participant ou équivalent pour présence et transport

# Utilitaires
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
thiserror = "2"

# Chiffrement (WorrySentinel, spec technique à valider)
# ring ou aes-gcm, etc.
```

---

## 3. Module data/

### 3.1 Types de domaine (types.rs)

Au minimum (conceptuel) :

- **Tribe** : id, name, creator_cog_id, created_at, settings, etc.
- **Salon** : id, tribe_id (optionnel), type (direct | collective), participants, created_at.
- **Message** : id, salon_id, sender_cog_id, content_encrypted ou ref, created_at.
- **Friend** : id, profile_id, friend_cog_id, pseudo (résolu), added_at.
- **Role** : id, tribe_id, name, permissions (bitmask ou liste).

Identifiants : UUID v4 ; timestamps ISO 8601. Convention CRUD : `tribe_list`, `tribe_by_id`, `tribe_create`, `tribe_update`, `tribe_delete`, etc.

### 3.2 Persistance (kindmother_db / kindmother_client_db)

- **InstanceType** : `Daughter` pour les DB locales du service.
- Toute écriture passe par KindMother (WriteIntent) ; pas de connexion SQL directe en contournement.
- Schémas : tables `tribes`, `salons`, `salon_members`, `messages`, `friends`, `roles`, `tribe_member_roles`, etc. (détail à définir en phase de conception schéma).

---

## 4. Transport et MWS

- **Présence** : consommation des APIs ou événements MWS pour « en ligne / hors ligne ». Pas de réimplémentation.
- **Transport des messages** : envoi/réception via le MWS (canal sécurisé). Chiffrement du payload applicatif selon [Jay1Tribu - Securite et Conformite](./Jay1Tribu%20-%20Securite%20et%20Conformite.md).
- **Livraison différée (tribu)** : lorsque un membre se reconnecte, le système vérifie si l'émetteur est connecté et envoie les messages/fichiers non encore livrés ; implémentation à détailler (file d’attente locale, reprise à la connexion).

---

## 5. Intégration Miyukini Central

- Enregistrement du service dans la liste des services (Salon / Bibliothèque).
- Ouverture de l’écran/onglet Jay1Tribu depuis Central (navigation, pas de lecture du contenu par Central).
- Connexion à la base Jay1Tribu (ServiceDb) depuis `apps/central` si nécessaire (ex. via `ServiceConnections` ou équivalent).
- Exposer les capacités pour Miou : `get_online_friends`, `get_friends_list` (voir [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md)).

---

## 6. Intégration Miou

- Implémenter le contrat décrit dans [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md) : retour de listes d’amis et amis en ligne sans contenu de messages.
- Dégradation gracieuse : si Jay1Tribu est indisponible, retourner liste vide ou erreur gérée ; pas de crash Central/Miou.

---

## 7. Phases suggérées

| Phase | Objectif | Livrables |
|-------|----------|-----------|
| **1** | Infrastructure | Crate `jay1tribu`, structure data/, types, KindMother (écriture/lecture locale de base). |
| **2** | Salons et messages | Création salon direct/collectif, envoi/réception messages (transport MWS + chiffrement), archivage local. |
| **3** | Tribus et rôles | Création tribu, invitations, rôles (Chef de tribu, admin, membre), salons de tribu. |
| **4** | Amis et présence | Liste d’amis, consommation présence MWS, get_online_friends / get_friends_list pour Miou. |
| **5** | Livraison différée | Synchronisation à la reconnexion pour les tribus (si émetteur connecté). |
| **6** | Fichiers et images | Envoi/réception fichiers et images (chiffrés), stockage local. |
| **7** | UI Central et Miou | Écrans Central, connexion Miou, dégradation gracieuse. |
| **8** | Sécurité et audit | Revue chiffrement, conformité C-1 à C-8, invariants, audit. |

---

## 8. Matrice de vérification (résumé)

Avant toute livraison, vérifier :

| Contrainte | Vérification |
|------------|--------------|
| C-1 | Aucune archive centralisée ; tout local. |
| C-2 | Transit crypté (tests, revue code). |
| C-3 | Hébergement utilisateur uniquement. |
| C-4 | Toutes les écritures via KindMother. |
| C-5 | Type 3 déclaré ; espaces Central + Inter-COG. |
| C-6 | Livraison différée conditionnée (émetteur connecté, paramétrage). |
| C-7 | Rôles attribués par Chef de tribu, gouvernés par Cores. |
| C-8 | Présence lue depuis MWS uniquement. |

---

## 9. Références

| Document | Rôle |
|----------|------|
| [Jay1Tribu - Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md) | Concepts, modèle métier. |
| [Jay1Tribu - Architecture et Positionnement](./Jay1Tribu%20-%20Architecture%20et%20Positionnement.md) | Pyramide, Opérateurs, MWS, Cores. |
| [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) | Contraintes et invariants. |
| [Jay1Tribu - Securite et Conformite](./Jay1Tribu%20-%20Securite%20et%20Conformite.md) | Chiffrement, contrôles d'accès. |
| [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md) | Contrat Central / Miou. |
| Skill miyukini-services | Pattern data/, auth/, services/, feature flags. |
| Skill miyukini-kindmother-db | KindMother, InstanceType, kindmother-db-key. |

---

**Document** : Jay1Tribu — Guide d'implémentation  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Guide normatif
