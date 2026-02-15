# MiyuAuth

## Toolkit d'Authentification et Sessions

**MiyuAuth** est le toolkit responsable de toutes les opérations d'authentification et de gestion des sessions dans Miyukini.

## Fonction

> MiyuAuth **vérifie** les identités et **gère** les sessions.

MiyuAuth est l'outil de confiance utilisé par TAMR pour valider les identités et maintenir les sessions utilisateur.

## Capacités

### Authentification

| Capacité | Description |
|----------|-------------|
| Vérification mot de passe | Hash Argon2 |
| Token validation | JWT local |
| Multi-facteur | TOTP supporté |
| Biométrie | Délégation au système |

### Gestion des Sessions

| Capacité | Description |
|----------|-------------|
| Création | Nouvelle session |
| Validation | Session active ? |
| Renouvellement | Extension durée |
| Révocation | Fin de session |

## Architecture

```
┌─────────────────────────────────────────────────┐
│                   MIYUAUTH                       │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │           Authentication Engine           │   │
│  └──────────────────────────────────────────┘   │
│       │           │           │           │     │
│       ▼           ▼           ▼           ▼     │
│  ┌────────┐ ┌──────────┐ ┌────────┐ ┌────────┐ │
│  │Password│ │  Token   │ │  MFA   │ │Session │ │
│  │Verifier│ │ Manager  │ │Handler │ │Manager │ │
│  └────────┘ └──────────┘ └────────┘ └────────┘ │
└─────────────────────────────────────────────────┘
```

## Flux d'Authentification

```
Demande d'authentification
        │
        ▼
┌─────────────────┐
│ Identification  │──► Qui prétend être l'utilisateur ?
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Vérification    │──► Mot de passe / Token / Biométrie
└────────┬────────┘
         │ OK
         ▼
┌─────────────────┐
│ MFA (si activé) │──► Code TOTP
└────────┬────────┘
         │ OK
         ▼
┌─────────────────┐
│ Création session│──► Génération token
└────────┬────────┘
         │
         ▼
    Authentifié
```

## Intégration avec TAMR

```
TAMR ──► "Authentifier Alice"
    │
    ▼
BondingBrother ──► MiyuAuth
                      │
                      ▼
                 Vérification
                      │
                      ▼
                 Résultat
                      │
    ┌─────────────────┘
    ▼
TAMR ◄── {authenticated: true, user_id: 42}
```

## Stockage des Credentials

MiyuAuth **ne stocke jamais** les mots de passe en clair :

| Donnée | Stockage |
|--------|----------|
| Mot de passe | Hash Argon2id |
| Token | JWT signé localement |
| Clé TOTP | Chiffrée (SQLCipher) |
| Session | En mémoire + persistance |

## Algorithmes

### Hachage de Mot de Passe

```
Argon2id avec paramètres :
- Memory : 64 Mo
- Iterations : 3
- Parallelism : 4
- Output : 32 bytes
```

### Tokens

```
JWT (JSON Web Token) :
- Algorithme : HS256 (local)
- Durée : configurable
- Claims : user_id, session_id, exp
```

## API (via BondingBrother)

### Intentions Supportées

| Intention | Paramètres | Résultat |
|-----------|------------|----------|
| `AUTHENTICATE` | user, credential | {authenticated, user_id} |
| `SESSION_CREATE` | user_id | {session_id, token} |
| `SESSION_VALIDATE` | token | {valid, user_id} |
| `SESSION_REVOKE` | session_id | {revoked} |
| `MFA_SETUP` | user_id | {secret, qr_code} |
| `MFA_VERIFY` | user_id, code | {valid} |

## Sessions

### Cycle de Vie

```
Création ──► Active ──► Expiration
                │
                ├──► Renouvellement ──► Active
                │
                └──► Révocation ──► Terminée
```

### Propriétés

| Propriété | Valeur |
|-----------|--------|
| Durée par défaut | 24 heures |
| Renouvellement | Automatique si actif |
| Max sessions | Configurable |
| Révocation | Immédiate |

## Sécurité

### Protection des Credentials

- Hash irréversible (Argon2id)
- Sel unique par utilisateur
- Pas de stockage de secrets en clair
- Clés dérivées du contexte

### Protection des Sessions

- Tokens signés
- Durée de vie limitée
- Révocation immédiate possible
- Isolation par utilisateur

### Rate Limiting

| Action | Limite |
|--------|--------|
| Tentatives login | 5 / minute |
| Création session | 10 / heure |
| Vérification MFA | 3 / minute |

## Contrats

### Contrat de Frontière

MiyuAuth **peut** :
- Vérifier des credentials
- Créer/gérer des sessions
- Générer des tokens

MiyuAuth **ne peut pas** :
- Décider qui a accès à quoi (c'est TAMR)
- Stocker des données métier
- Accéder à d'autres Toolkits

### Contrat de Gouvernance

MiyuAuth respecte :
- LOI-1 : Fonctionne offline
- LOI-3 : Données locales souveraines
- Pas de dépendance externe

## Cas d'Usage

### Login Standard

```
Utilisateur entre : email + mot de passe
        │
        ▼
TAMR demande authentification
        │
        ▼
MiyuAuth :
  1. Récupère le hash (via KindMother)
  2. Vérifie le mot de passe
  3. Crée une session
  4. Retourne le token
        │
        ▼
Utilisateur connecté
```

### Authentification MFA

```
Après login réussi :
        │
        ▼
MiyuAuth : MFA requis ?
        │ Oui
        ▼
Demande code TOTP
        │
        ▼
MiyuAuth vérifie le code
        │
        ▼
Session activée
```

## Conformité

- Algorithmes standards (Argon2, TOTP RFC 6238)
- Pas de crypto maison
- Audit possible de toutes les opérations
- Compatible avec les gestionnaires de mots de passe
