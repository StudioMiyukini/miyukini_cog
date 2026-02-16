# MiyukiniBB — Service forum (phpBB adapté Miyukini COG)

## Identité

**MiyukiniBB** est le service forum de l’écosystème Miyukini COG. Il repose sur **phpBB** adapté pour Miyukini :

- **Forum unifié** : disponible depuis **le site internet d’Origin** (ex. `forum.miyukini.com`) et depuis **le service Central** (lien ou intégration dans l’app).
- **Base de données sur Origin** : la base de données de MiyukiniBB (auth + contenus phpBB) est hébergée sur Origin.
- **Auth unifiée** : en cas de connexion depuis le site ou depuis Central, le **login** (email ou pseudo) et le **mot de passe** sont **les mêmes que pour Central** — une seule identité pour le forum et pour Central.

## Composants

| Composant | Rôle |
|-----------|------|
| **Crate `miyukinibb`** | Client Rust pour synchroniser les profils Central vers Origin (`sync_profile`). |
| **Origin** | Hébergement des données (base `forum_profiles` + base phpBB), API `/api/auth/forum/validate` et `/api/auth/forum/sync`, site et forum (ex. `forum.miyukini.com`). |
| **phpBB + extension Central Auth** | Forum (StudioMiyukini/phpbb-COG, extension `studiomiyukini/centralauth`), même login/password que Central. |
| **Style Miyukini Origin** | Thème phpBB aligné sur le site Origin (sombre, violet/cyan, Inter + JetBrains Mono). |

## Documentation

| Document | Rôle |
|----------|------|
| [MiyukiniBB - Document Fondateur](./MiyukiniBB%20-%20Document%20Fondateur.md) | Vision, périmètre, auth unifiée, déploiement. |
| [MiyukiniBB - Intégration Central](./MiyukiniBB%20-%20Integration%20Central.md) | Sync profil depuis Central, usage du crate `miyukinibb`. |

## Liaisons

- [Miyukini Central](../MiyukiniCentral/) — Hub et profils (source des comptes forum).
- [Origin](../../miyukini-webway-system/) — MWS Origin, API auth forum, hébergement.
- Toolkit **MiyuForum** (`crates/miyuforum`) — Outils forum (catégories, sujets, posts) ; MiyukiniBB est le **service** produit forum.
