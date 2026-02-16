# MiyukiniBB — Intégration Central

## Principe

Le forum MiyukiniBB est **unifié** : il est accessible depuis le **site Origin** et depuis le **service Central**. Dans les deux cas, la connexion utilise les **mêmes identifiants que Central** (login : email ou pseudo, mot de passe). Pour que cela fonctionne, les **profils Central** doivent être synchronisés vers Origin (base hébergée sur Origin). Le crate **`miyukinibb`** fournit le client pour appeler l’API Origin.

## Dépendance

Dans le crate ou l’app qui gère les profils Central (ex. `miyukini-central`, `apps/central`) :

```toml
miyukinibb = { path = "../miyukinibb" }
```

## Utilisation

### 1. Créer le client

L’URL de base d’Origin doit être configurable (config, variable d’environnement). Exemple :

```rust
use miyukinibb::{MiyukiniBbClient, ForumProfileSync};

let origin_url = std::env::var("ORIGIN_API_URL").unwrap_or_else(|_| "https://origin.miyukini.com".into());
let client = MiyukiniBbClient::new(origin_url)?;
```

### 2. Synchroniser un profil après création ou mise à jour

Le hash du mot de passe doit être le **SHA256 hex** (identique à Central). Exemple après création d’un profil dans Central :

```rust
let sync = ForumProfileSync::new(
    profile.id.clone(),
    profile.email.clone(),
    profile_password_hash,  // déjà calculé (SHA256 hex)
    profile.pseudonyme.clone(),
);
if let Err(e) = client.sync_profile(&sync) {
    tracing::warn!("MiyukiniBB sync failed: {}", e);
}
```

### 3. Quand appeler le sync

- **À la création du profil** : après `create_profile` (ou équivalent), appeler `sync_profile` pour que le nouvel utilisateur puisse se connecter au forum.
- **À la mise à jour du mot de passe** : appeler `sync_profile` avec le nouveau `password_hash`.
- **À la mise à jour du pseudonyme** : appeler `sync_profile` pour mettre à jour l’affichage côté forum.
- **Optionnel à la connexion** : pour garder le pseudonyme à jour à chaque login (évite des appels systématiques si pas de changement).

## Erreurs

`MiyukiniBbError` couvre :

- `InvalidOriginUrl` : URL vide ou invalide.
- `Http` : échec réseau ou timeout (ureq).
- `Api { status, body }` : Origin a répondu avec une erreur (4xx/5xx ou `ok: false`).
- `Json` : réponse non JSON valide.
- `InvalidProfile` : central_id ou email manquant.

En cas d’échec du sync, le forum ne pourra pas authentifier l’utilisateur avec ce profil tant que le sync n’aura pas réussi (retry manuel ou au prochain login).

## Configuration Central

Pour activer le sync depuis Central, il faut :

1. Une **URL Origin** configurable (ex. `ORIGIN_API_URL` ou champ dans les préférences).
2. Un point d’appel après création/mise à jour de profil (et éventuellement à la connexion) qui instancie `MiyukiniBbClient` et appelle `sync_profile`.

Si l’URL n’est pas configurée, le sync peut être désactivé (pas d’appel, ou client optionnel).
