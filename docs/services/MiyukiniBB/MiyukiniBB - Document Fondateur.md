# MiyukiniBB — Document Fondateur

## Vision

**MiyukiniBB** est le service forum officiel Miyukini COG. Il fournit un **forum unifié** (phpBB) dont les **comptes sont les profils Central** : un seul identifiant (email / pseudo + mot de passe) pour l’application Central et pour le forum, quel que soit le point d’accès.

## Accès et auth unifiée

- **Forum unifié** : le forum est disponible depuis **le site internet d’Origin** (ex. `forum.miyukini.com` ou section forum du site Origin) et depuis **le service Central** (lien ou intégration dans l’app Central).
- **Auth unifiée** : en cas de connexion depuis le site Origin ou depuis Central, le **login** (email ou pseudonyme) et le **mot de passe** de l’utilisateur sont **les mêmes que pour Central**. Une seule identité, une seule authentification pour le forum et pour Central.

## Base de données hébergée sur Origin

La **base de données de MiyukiniBB** est hébergée sur **Origin** :
- Table **forum_profiles** (SQLite, configurée via `forum_profiles_db_path`) : copie des profils Central pour la validation des connexions forum (auth unifiée).
- Base **phpBB** (MySQL/SQLite selon déploiement) : contenus du forum (catégories, sujets, messages, sessions). Elle est hébergée sur le même serveur qu’Origin (VPS Origin).

## Périmètre

- **In scope**
  - Forum de discussion (phpBB 3.3 adapté).
  - Auth unifiée avec Miyukini Central (même login/pseudo/email et mot de passe sur le site Origin et dans Central).
  - Synchronisation des profils Central vers Origin pour permettre la connexion au forum depuis le site ou depuis Central.
  - Hébergement des données et du forum sur Origin (site + base(s)).

- **Hors scope**
  - Gestion des contenus forum (catégories, sujets, messages) : assurée par phpBB.
  - Logique métier forum (modération, permissions phpBB) : côté phpBB.
  - Le toolkit **MiyuForum** (strate 6) reste le kit d’outils forum (boards, topics, posts) ; MiyukiniBB est le **service** (strate 7) = le produit forum hébergé.

## Architecture

1. **Central** (app + `miyukini-central`)  
   Gère les profils (email, password_hash, pseudonyme). À la création ou mise à jour d’un profil (ou à la connexion), un client peut appeler le **sync** vers Origin pour que l’utilisateur puisse se connecter au forum (depuis le site Origin ou depuis Central) avec les mêmes identifiants.

2. **Origin**  
   Héberge les données et le forum :
   - **API** : `POST /api/auth/forum/sync` (mise à jour de la table `forum_profiles`), `POST /api/auth/forum/validate` (validation email + mot de passe).
   - **Base forum_profiles** (SQLite) : copie des profils Central pour l’auth unifiée.
   - **Forum phpBB** : base phpBB + application, servie depuis le site Origin (ex. `forum.miyukini.com` ou chemin dédié).

3. **Forum (phpBB)**  
   - Extension **StudioMiyukini Central Auth** : au login (depuis le site ou depuis Central), envoie email/pseudo + mot de passe à Origin `/api/auth/forum/validate` ; si OK, crée ou réutilise l’utilisateur phpBB (même identité que le profil Central).
   - **Mêmes identifiants** : login (email ou pseudo) et mot de passe identiques à Central.

## Compte « MiyukiniBB »

Les comptes du forum sont nommés **comptes MiyukiniBB** : ce sont les **profils Central** dont les données sont synchronisées sur Origin. Un seul compte (Central) donne accès au forum **depuis le site Origin ou depuis le service Central**, avec les **mêmes identifiants** (email ou pseudo + mot de passe).

## Crate `miyukinibb`

Le crate Rust `crates/miyukinibb` fournit :

- **`MiyukiniBbClient`** : client pour l’API Origin (configuré avec l’URL de base).
- **`sync_profile`** : envoi d’un profil (central_id, email, password_hash, pseudonyme) vers `POST /api/auth/forum/sync`.
- **`ForumProfileSync`** : type de données pour le sync.
- **`MiyukiniBbError`** : erreurs (URL invalide, HTTP, API, profil invalide).

Utilisation typique depuis Central (ou un outil d’import) : après création ou mise à jour d’un profil, appeler `MiyukiniBbClient::new(origin_url)?.sync_profile(&ForumProfileSync::new(...))` pour que l’utilisateur puisse se connecter au forum.

## Déploiement

- **Origin** : configurer `forum_profiles_db_path` dans `origin.toml` (ex. `data/forum_profiles.db`). La base MiyukiniBB (forum_profiles + base phpBB) est hébergée sur le serveur Origin.
- **Forum** : déployer phpBB (dépôt StudioMiyukini/phpbb-COG) + extension `studiomiyukini/centralauth` sur Origin ; configurer l’URL de l’API Origin dans l’ACP phpBB.
- **Thème** : le style **Miyukini Origin** (`styles/miyukini_origin/`) aligne l’UI du forum sur le site Origin (thème sombre, violet/cyan, polices Inter et JetBrains Mono). À activer dans **Personnalisation** → **Styles**.
- **Accès** : le forum est disponible depuis le site Origin (ex. `forum.miyukini.com`) et depuis le service Central (lien ou intégration). Connexion avec les mêmes login (email/pseudo) et mot de passe que Central.

## Références

- Extension phpBB : `forum/phpBB/ext/studiomiyukini/centralauth/`
- Style Miyukini Origin : `forum/phpBB/styles/miyukini_origin/`
- API forum auth : `apps/origin/src/web/forum_auth.rs`
- Client : `crates/miyukinibb/`
